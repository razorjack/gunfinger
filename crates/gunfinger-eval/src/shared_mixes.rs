//! Rendered mixes around a shared-material panel's played files (session
//! 21): each mix plays one played file whole among tracks drawn as
//! `mixes` draws them (`mixes::draw_around`), and is searched with the
//! played recording indexed, left out (the main case) and, where another
//! rip of it is indexed, with the played file alone left out (the
//! positive control). Other audio, such as the development mixes, can be
//! searched as it is with everything indexed.
//!
//! Every detection carries the measures of the session's candidates: the
//! peak verifier's share, the file's own material on each side
//! (`Verifier::beyond`) and, for a confident detection, the strongest
//! other indexed file that holds its passage in the library (`holder`),
//! with every file the lookup finds (`holders`), so the holder can be
//! taken again under another clusters file (session 23).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search_peaks};
use gunfinger_core::speed::{Rung, ladder};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::mixes::{MixFound, PlannedMix, Whole, draw_around, render_once};
use crate::padding::{Padding, SecondLibrary};
use crate::rng::Rng;
use crate::shared::{Roots, indexes, recording};
use crate::verifier::{Beyond, Verifier};

/// The file's own material looked for on each side of a detection: one
/// window of the confidence rule and of the verifier.
const BEYOND_SECONDS: f64 = 10.0;
/// Drawn tracks are at least this long, as in `mixes::Pools`.
const DRAWN_MIN_SECONDS: f64 = 150.0;
/// The files a holder lookup finds with at least this many hits are
/// listed. The 60 hits that mark a play are not far above chance at NAS
/// scale: unrelated tracks reached 40 and 48 hits in a mix and 66-78 per
/// excerpt (experiments 0040-0042), so a holder near the mark may be
/// unrelated.
pub const LISTED_HOLDER_HITS: u32 = 30;

/// The mixes to render and the audio to search, as
/// `scripts/analysis/shared_mixes.py plan` writes them. Unknown fields are
/// refused, so a binary older than the plan fails instead of ignoring them.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub seeds: Vec<u64>,
    /// Drawn plays before each played file, and as many after.
    pub around: usize,
    /// Files whose duplicate clusters are never drawn.
    pub exclude: Vec<String>,
    /// One mix per seed and played file.
    pub played: Vec<Played>,
    /// Files that are one recording beyond the duplicate clusters, such
    /// as the panel's joined rips: a file's own recording never holds its
    /// passage for `holder`.
    #[serde(default)]
    pub recordings: Vec<Vec<String>>,
    /// Other audio, searched as it is with everything indexed.
    #[serde(default)]
    pub queries: Vec<Query>,
    /// Every index of a mix also leaves out the files its drawn plays
    /// were rendered from, so a drawn track is found only through another
    /// rip of it, as a real play would be.
    #[serde(default)]
    pub leave_out_drawn: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Played {
    pub name: String,
    pub file: String,
    /// The played recording's files beyond its duplicate cluster.
    #[serde(default)]
    pub recording: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub name: String,
    /// Relative to the corpus's sets directory.
    pub audio: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct Report {
    pub mixes: Vec<Rendered>,
    pub runs: Vec<Run>,
}

#[derive(Serialize, Deserialize)]
pub struct Rendered {
    pub seed: u64,
    pub played: Played,
    pub mix: PlannedMix,
}

/// One mix or query searched against one index.
#[derive(Serialize, Deserialize)]
pub struct Run {
    /// The mix's or the query's name.
    pub name: String,
    /// `everything`, `played left out` or `source left out`.
    pub index: String,
    pub left_out: Vec<String>,
    /// Every detection with two windows or more, strongest first.
    pub detections: Vec<Measured>,
}

#[derive(Serialize, Deserialize)]
pub struct Measured {
    #[serde(flatten)]
    pub found: MixFound,
    pub track_end_seconds: f64,
    /// The file's own material on each side with room for it.
    pub beyond: Vec<Beyond>,
    /// For a confident detection, the strongest other indexed file that
    /// holds its passage; absent when none was found or the detection is
    /// not confident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder: Option<Holder>,
    /// For a confident detection, every file its lookup finds with at
    /// least `LISTED_HOLDER_HITS` hits, its own recording's included.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub holders: Vec<Holder>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Holder {
    pub asset: String,
    pub hits: u32,
    pub windows: u32,
}

pub struct Options<'a> {
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub padding: &'a Padding,
    /// The corpus's sets directory, for the plan's queries.
    pub sets: &'a Path,
    pub jobs: usize,
}

/// What one index searches: a rendered mix or a query.
enum Target<'a> {
    Mix(&'a Rendered),
    Query(&'a Query),
}

pub fn run(
    plan: &Plan,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    roots: &Roots,
    dir: &Path,
    options: &Options,
) -> Result<Report, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let second = options.padding.second.as_ref();
    let verifier = Verifier::new(&records, second);
    let rendered = draw(plan, &records, second, clusters)?;
    fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    for result in map_in_order(&rendered, options.jobs, |rendered| {
        render_once(&rendered.mix, roots, &mix_path(dir, &rendered.mix))
    }) {
        result?;
    }
    eprintln!("{} mixes rendered in {}", rendered.len(), dir.display());

    let indexed = |file: &str| {
        records
            .iter()
            .any(|record| record.header.source.path == file)
            || second.is_some_and(|second| second.holds(file))
    };
    let own = |file: &str| own_recording(file, clusters, &plan.recordings);

    let mut runs = Vec::new();
    for (left_out, searched) in &targets(&rendered, plan, clusters, &indexed) {
        let index = options
            .matching
            .index(options.padding.index(&records, &profile, left_out)?);
        for (index_name, target) in searched {
            let (name, path) = match target {
                Target::Mix(rendered) => (rendered.mix.name.clone(), mix_path(dir, &rendered.mix)),
                Target::Query(query) => (query.name.clone(), options.sets.join(&query.audio)),
            };
            let audio = decode(&path, profile.sample_rate, Excerpt::default())
                .map_err(|error| format!("{}: {error}", path.display()))?;
            let detections = options.matching.search(
                &index,
                &audio.samples,
                &profile,
                options.ladder,
                options.jobs,
            );
            let detections = map_in_order(&detections, options.jobs, |detection| {
                measure(&index, detection, &audio.samples, &profile, &verifier, &own)
            });
            eprintln!(
                "{name} [{index_name}]: {} detections, {} confident",
                detections.len(),
                detections
                    .iter()
                    .filter(|measured| measured.found.confidence() == Confidence::Confident)
                    .count()
            );
            runs.push(Run {
                name,
                index: (*index_name).to_owned(),
                left_out: left_out.iter().cloned().collect(),
                detections,
            });
        }
    }
    Ok(Report {
        mixes: rendered,
        runs,
    })
}

/// What each index searches, by the files it leaves out: every mix under
/// `shared::indexes`'s indexes, each also without the mix's drawn files
/// when the plan says so, and the plan's queries with everything indexed.
fn targets<'a>(
    rendered: &'a [Rendered],
    plan: &'a Plan,
    clusters: &Clusters,
    indexed: &dyn Fn(&str) -> bool,
) -> BTreeMap<BTreeSet<String>, Vec<(&'static str, Target<'a>)>> {
    let mut groups: BTreeMap<BTreeSet<String>, Vec<(&str, Target)>> = BTreeMap::new();
    for rendered in rendered {
        let files = recording(clusters, &rendered.played.file, &rendered.played.recording);
        let drawn = drawn_files(rendered, plan.leave_out_drawn);
        for (index_name, mut left_out) in indexes(&rendered.played.file, &files, indexed) {
            left_out.extend(drawn.iter().cloned());
            groups
                .entry(left_out)
                .or_default()
                .push((index_name, Target::Mix(rendered)));
        }
    }
    for query in &plan.queries {
        groups
            .entry(BTreeSet::new())
            .or_default()
            .push(("everything", Target::Query(query)));
    }
    groups
}

/// One mix per seed and played file, the seed's draws in the plan's order.
fn draw(
    plan: &Plan,
    records: &[PeakRecord],
    second: Option<&SecondLibrary>,
    clusters: &Clusters,
) -> Result<Vec<Rendered>, String> {
    let pool = drawn_pool(records, &plan.exclude, clusters);
    let mut rendered = Vec::new();
    for &seed in &plan.seeds {
        let mut rng = Rng::new(seed);
        for played in &plan.played {
            let whole = Whole {
                asset: &played.file,
                duration_seconds: duration(records, second, &played.file)?,
            };
            let name = format!("seed-{seed}-{}", played.name);
            rendered.push(Rendered {
                seed,
                played: played.clone(),
                mix: draw_around(&name, &whole, plan.around, &pool, clusters, &mut rng)?,
            });
        }
    }
    Ok(rendered)
}

/// The library's records long enough to draw, outside the duplicate
/// clusters of the files `exclude` names.
fn drawn_pool<'a>(
    records: &'a [PeakRecord],
    exclude: &[String],
    clusters: &Clusters,
) -> Vec<&'a PeakRecord> {
    let excluded: BTreeSet<String> = exclude
        .iter()
        .flat_map(|file| clusters.cluster_of(file))
        .collect();
    records
        .iter()
        .filter(|record| {
            record.header.duration_seconds >= DRAWN_MIN_SECONDS
                && !excluded.contains(&record.header.source.path)
        })
        .collect()
}

fn duration(
    records: &[PeakRecord],
    second: Option<&SecondLibrary>,
    path: &str,
) -> Result<f64, String> {
    if let Some(record) = records
        .iter()
        .find(|record| record.header.source.path == path)
    {
        return Ok(record.header.duration_seconds);
    }
    match second.and_then(|second| second.record(path)) {
        Some(record) => record.map(|record| record.header.duration_seconds),
        None => Err(format!("no current peak record of {path}")),
    }
}

/// The files `rendered`'s drawn plays come from, when the plan leaves
/// them out.
fn drawn_files(rendered: &Rendered, leave_out_drawn: bool) -> BTreeSet<String> {
    if !leave_out_drawn {
        return BTreeSet::new();
    }
    rendered
        .mix
        .plays
        .iter()
        .filter(|play| play.asset != rendered.played.file)
        .map(|play| play.asset.clone())
        .collect()
}

fn mix_path(dir: &Path, mix: &PlannedMix) -> PathBuf {
    dir.join(format!("{}.mp3", mix.name))
}

/// `file`'s duplicate cluster and every listed recording that holds it.
fn own_recording(file: &str, clusters: &Clusters, recordings: &[Vec<String>]) -> BTreeSet<String> {
    let mut files = clusters.cluster_of(file);
    for recording in recordings {
        if recording.iter().any(|member| member == file) {
            files.extend(recording.iter().cloned());
        }
    }
    files
}

/// A detection with the candidates' measures; only detections that are at
/// least possible, which alone take part in plays, are measured.
fn measure(
    index: &Index,
    detection: &Detection,
    samples: &[f32],
    profile: &Profile,
    verifier: &Verifier,
    own: &dyn Fn(&str) -> BTreeSet<String>,
) -> Measured {
    let level = MixFound::new(index, detection, None).confidence();
    let shown = level >= Confidence::Possible;
    let found = MixFound::new(
        index,
        detection,
        shown
            .then(|| verifier.verify(index, detection, samples, profile))
            .flatten(),
    );
    let found_holders = if level == Confidence::Confident {
        verifier.peaks(&found.asset).map(|peaks| {
            lookup(
                index,
                &peaks,
                detection.track_start_seconds,
                detection.track_end_seconds,
                profile,
            )
        })
    } else {
        None
    };
    let (holder, holders) = found_holders.map_or((None, Vec::new()), |found_holders| {
        (
            strongest_other(found_holders.iter().cloned(), &own(&found.asset)),
            listed(found_holders),
        )
    });
    Measured {
        beyond: if shown {
            verifier.beyond(index, detection, samples, profile, BEYOND_SECONDS)
        } else {
            Vec::new()
        },
        track_end_seconds: detection.track_end_seconds,
        holder,
        holders,
        found,
    }
}

/// A lookup in a map of passages that indexed files share, made from the
/// files the index holds: the file's stored peaks within a span of the
/// file (`track_start_seconds` to `track_end_seconds`), searched as the
/// duplicate search does (rescaled to each turntable rung). Every
/// detection found, in the search's order.
pub fn lookup(
    index: &Index,
    peaks: &[Peak],
    track_start_seconds: f64,
    track_end_seconds: f64,
    profile: &Profile,
) -> Vec<Holder> {
    search_peaks(
        index,
        &passage(peaks, track_start_seconds, track_end_seconds, profile),
        profile,
        &ladder(),
        1,
    )
    .iter()
    .map(|found| Holder {
        asset: index.asset(found.asset).path.clone(),
        hits: found.evidence.hits,
        windows: found.evidence.windows,
    })
    .collect()
}

/// The file's peaks within a span of the file.
fn passage(
    peaks: &[Peak],
    track_start_seconds: f64,
    track_end_seconds: f64,
    profile: &Profile,
) -> Vec<Peak> {
    let span = profile.frames(track_start_seconds)..=profile.frames(track_end_seconds);
    peaks
        .iter()
        .filter(|peak| span.contains(&peak.frame))
        .copied()
        .collect()
}

/// The strongest file found outside `own`, the file's own recording.
fn strongest_other(found: impl Iterator<Item = Holder>, own: &BTreeSet<String>) -> Option<Holder> {
    found
        .filter(|holder| !own.contains(&holder.asset))
        .max_by_key(|holder| holder.hits)
}

/// Each file found with at least `LISTED_HOLDER_HITS` hits, once, at its
/// strongest detection; strongest first, then by path.
pub fn listed(found: Vec<Holder>) -> Vec<Holder> {
    let mut strongest: BTreeMap<String, Holder> = BTreeMap::new();
    for holder in found {
        if holder.hits < LISTED_HOLDER_HITS {
            continue;
        }
        match strongest.get(&holder.asset) {
            Some(kept) if kept.hits >= holder.hits => {}
            _ => {
                strongest.insert(holder.asset.clone(), holder);
            }
        }
    }
    let mut listed: Vec<Holder> = strongest.into_values().collect();
    listed.sort_by(|a, b| b.hits.cmp(&a.hits).then_with(|| a.asset.cmp(&b.asset)));
    listed
}

pub fn print_summary(report: &Report) {
    for run in &report.runs {
        let confident: Vec<&Measured> = run
            .detections
            .iter()
            .filter(|measured| measured.found.confidence() == Confidence::Confident)
            .collect();
        println!(
            "{:<40} {:<16} {:>3} confident: {}",
            run.name,
            run.index,
            confident.len(),
            confident
                .iter()
                .map(|measured| format!(
                    "{} ({} hits)",
                    measured.found.asset.rsplit('/').next().unwrap_or_default(),
                    measured.found.hits
                ))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::store::RecordHeader;

    use super::*;

    fn record(path: &str, duration_seconds: f64) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds,
            },
            peaks: Vec::new(),
        }
    }

    fn clusters(duplicates: Vec<Vec<&str>>) -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates: duplicates
                .into_iter()
                .map(|members| members.into_iter().map(str::to_owned).collect())
                .collect(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        }
    }

    fn holder(asset: &str, hits: u32) -> Holder {
        Holder {
            asset: asset.to_owned(),
            hits,
            windows: 3,
        }
    }

    fn played(recording: &[&str]) -> Played {
        Played {
            name: String::from("p"),
            file: String::from("played.mp3"),
            recording: recording.iter().map(|&file| file.to_owned()).collect(),
        }
    }

    #[test]
    fn the_pool_leaves_out_short_tracks_and_excluded_clusters() {
        let records = [
            record("a.mp3", 300.0),
            record("a-rip.mp3", 300.0),
            record("b.mp3", 100.0),
            record("c.mp3", 300.0),
        ];
        let clusters = clusters(vec![vec!["a.mp3", "a-rip.mp3"]]);

        let pool = drawn_pool(&records, &[String::from("a.mp3")], &clusters);

        let paths: Vec<&str> = pool
            .iter()
            .map(|record| record.header.source.path.as_str())
            .collect();
        assert_eq!(paths, ["c.mp3"]);
    }

    #[test]
    fn a_files_own_recording_holds_its_cluster_and_listed_recordings() {
        let clusters = clusters(vec![vec!["a.mp3", "a-rip.mp3"]]);
        let recordings = vec![vec![String::from("a.mp3"), String::from("a-upload.m4a")]];

        assert_eq!(
            own_recording("a.mp3", &clusters, &recordings),
            BTreeSet::from([
                String::from("a.mp3"),
                String::from("a-rip.mp3"),
                String::from("a-upload.m4a")
            ])
        );
        assert_eq!(
            own_recording("b.mp3", &clusters, &recordings),
            BTreeSet::from([String::from("b.mp3")])
        );
    }

    #[test]
    fn a_plan_with_an_unknown_field_is_refused() {
        let plan =
            r#"{"seeds": [1], "around": 1, "exclude": [], "played": [], "leave_out_drawn": true}"#;
        let misspelt = plan.replace("leave_out_drawn", "leave_out_drawm");
        let in_played = r#"{"seeds": [1], "around": 1, "exclude": [],
            "played": [{"name": "p", "file": "a.mp3", "recordings": []}]}"#;
        let in_query = r#"{"seeds": [1], "around": 1, "exclude": [], "played": [],
            "queries": [{"name": "q", "audio": "q.mp3", "audios": []}]}"#;

        assert!(serde_json::from_str::<Plan>(plan).unwrap().leave_out_drawn);
        assert!(serde_json::from_str::<Plan>(&misspelt).is_err());
        assert!(serde_json::from_str::<Plan>(in_played).is_err());
        assert!(serde_json::from_str::<Plan>(in_query).is_err());
        for session_plan in [
            include_str!("../../../docs/panels/shared-mixes-s21.json"),
            include_str!("../../../docs/panels/shared-mixes-s21-rips.json"),
        ] {
            assert!(serde_json::from_str::<Plan>(session_plan).is_ok());
        }
    }

    #[test]
    fn the_passage_is_the_files_peaks_within_the_span() {
        let profile = Profile::CURRENT;
        let peaks: Vec<Peak> = [5.0, 10.0, 15.0, 20.0, 25.0, 105.0]
            .into_iter()
            .map(|seconds| Peak {
                frame: profile.frames(seconds),
                bin: 100.0,
                magnitude: 0.0,
            })
            .collect();

        let frames: Vec<f64> = passage(&peaks, 10.0, 20.0, &profile)
            .iter()
            .map(|peak| peak.frame)
            .collect();

        assert_eq!(
            frames,
            [10.0, 15.0, 20.0].map(|seconds| profile.frames(seconds))
        );
    }

    #[test]
    fn the_holder_is_the_strongest_file_outside_the_own_recording() {
        let own = BTreeSet::from([String::from("a.mp3"), String::from("a-rip.mp3")]);
        let found = [
            holder("a-rip.mp3", 900),
            holder("remix.mp3", 120),
            holder("vip.mp3", 300),
        ];

        assert_eq!(
            strongest_other(found.into_iter(), &own),
            Some(holder("vip.mp3", 300))
        );
        assert_eq!(
            strongest_other([holder("a.mp3", 50)].into_iter(), &own),
            None
        );
    }

    #[test]
    fn the_listed_holders_are_each_file_once_at_its_strongest_from_30_hits() {
        let found = vec![
            holder("remix.mp3", 120),
            holder("a-rip.mp3", 900),
            holder("chance.mp3", 29),
            holder("remix.mp3", 140),
            holder("vip.mp3", 120),
            holder("edit.mp3", 30),
        ];

        assert_eq!(
            listed(found),
            vec![
                holder("a-rip.mp3", 900),
                holder("remix.mp3", 140),
                holder("vip.mp3", 120),
                holder("edit.mp3", 30),
            ]
        );
    }

    #[test]
    fn each_seed_draws_one_mix_per_played_file_around_it() {
        let records: Vec<PeakRecord> = (0..10)
            .map(|n| record(&format!("t{n}.mp3"), 300.0))
            .chain([record("played.mp3", 200.0)])
            .collect();
        let plan = Plan {
            seeds: vec![1, 2],
            around: 2,
            exclude: vec![String::from("played.mp3")],
            played: vec![played(&[])],
            recordings: Vec::new(),
            queries: Vec::new(),
            leave_out_drawn: false,
        };

        let rendered = draw(&plan, &records, None, &clusters(Vec::new())).unwrap();

        assert_eq!(rendered.len(), 2);
        assert_eq!(rendered[0].mix.name, "seed-1-p");
        for rendered in &rendered {
            let plays = &rendered.mix.plays;
            assert_eq!(plays.len(), 5);
            assert_eq!(plays[2].asset, "played.mp3");
            assert!(
                plays
                    .iter()
                    .filter(|play| play.asset == "played.mp3")
                    .count()
                    == 1
            );
        }
        assert_ne!(rendered[0].mix.plays[0], rendered[1].mix.plays[0]);
    }

    #[test]
    fn drawn_files_are_left_out_only_when_the_plan_says_so() {
        let records: Vec<PeakRecord> = (0..4)
            .map(|n| record(&format!("t{n}.mp3"), 300.0))
            .chain([record("played.mp3", 200.0)])
            .collect();
        let plan = Plan {
            seeds: vec![1],
            around: 1,
            exclude: vec![String::from("played.mp3")],
            played: vec![played(&[])],
            recordings: Vec::new(),
            queries: Vec::new(),
            leave_out_drawn: true,
        };
        let rendered = &draw(&plan, &records, None, &clusters(Vec::new())).unwrap()[0];

        let drawn = drawn_files(rendered, true);

        assert_eq!(drawn.len(), 2);
        assert!(!drawn.contains("played.mp3"));
        assert!(drawn.iter().all(|file| file.starts_with('t')));
        assert!(drawn_files(rendered, false).is_empty());
    }

    #[test]
    fn every_index_of_a_mix_leaves_out_its_drawn_files_when_the_plan_says_so() {
        let records: Vec<PeakRecord> = (0..4)
            .map(|n| record(&format!("t{n}.mp3"), 300.0))
            .chain([record("played.mp3", 200.0), record("played-rip.mp3", 200.0)])
            .collect();
        let clusters = clusters(Vec::new());
        let mut plan = Plan {
            seeds: vec![1],
            around: 1,
            exclude: vec![String::from("played.mp3"), String::from("played-rip.mp3")],
            played: vec![played(&["played.mp3", "played-rip.mp3"])],
            recordings: Vec::new(),
            queries: vec![Query {
                name: String::from("q"),
                audio: PathBuf::from("q.mp3"),
            }],
            leave_out_drawn: true,
        };
        let rendered = draw(&plan, &records, None, &clusters).unwrap();
        let drawn = drawn_files(&rendered[0], true);
        let indexed = |_: &str| true;
        let with = |extra: &[&str]| -> BTreeSet<String> {
            drawn
                .iter()
                .cloned()
                .chain(extra.iter().map(|&file| file.to_owned()))
                .collect()
        };
        let names = |groups: &BTreeMap<BTreeSet<String>, Vec<(&'static str, Target)>>| {
            groups
                .iter()
                .flat_map(|(left_out, targets)| {
                    targets.iter().map(move |(index, target)| {
                        let name = match target {
                            Target::Mix(rendered) => rendered.mix.name.clone(),
                            Target::Query(query) => query.name.clone(),
                        };
                        (left_out.clone(), *index, name)
                    })
                })
                .collect::<BTreeSet<_>>()
        };

        let left_out = names(&targets(&rendered, &plan, &clusters, &indexed));
        plan.leave_out_drawn = false;
        let kept = names(&targets(&rendered, &plan, &clusters, &indexed));

        let mix = || String::from("seed-1-p");
        assert_eq!(drawn.len(), 2);
        assert_eq!(
            left_out,
            BTreeSet::from([
                (with(&[]), "everything", mix()),
                (
                    with(&["played.mp3", "played-rip.mp3"]),
                    "played left out",
                    mix()
                ),
                (with(&["played.mp3"]), "source left out", mix()),
                (BTreeSet::new(), "everything", String::from("q")),
            ])
        );
        assert_eq!(
            kept,
            BTreeSet::from([
                (BTreeSet::new(), "everything", mix()),
                (
                    BTreeSet::from([String::from("played.mp3"), String::from("played-rip.mp3")]),
                    "played left out",
                    mix()
                ),
                (
                    BTreeSet::from([String::from("played.mp3")]),
                    "source left out",
                    mix()
                ),
                (BTreeSet::new(), "everything", String::from("q")),
            ])
        );
    }
}

//! The self-generated speed sweep.
//!
//! About a fifth of the duplicate clusters are held out of the index. Thirty
//! second excerpts of indexed and held-out assets are played at turntable
//! speeds and encoded as MP3. Indexed excerpts must return their own cluster;
//! held-out excerpts must return nothing confident. Everything is drawn from
//! a seed, so a run is repeatable.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Pass;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, MIN_HITS_TO_REFINE, SKIPPED_SHARE};
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::fullest::{self, FullestReport, Ranking};
use crate::library_map::LibraryMap;
use crate::matching::Matching;
use crate::padding::{Padding, SECOND_LIBRARY_PREFIX, original_path};
use crate::render::{Encoding, render_excerpt};
use crate::rng::Rng;
use crate::titles::SameTitles;
use crate::verifier::{Verification, Verifier};

pub const SPEEDS_PERCENT: [f64; 9] = [-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0, 8.0];
/// Speeds at and past the ladder's ends, a rung (0.4%) apart up to 11.2%:
/// the ends of the ladders with 1, 2, 3 and 5 extra rungs among them
/// (session 51).
pub const EDGE_SPEEDS_PERCENT: [f64; 18] = [
    -12.0, -11.2, -10.8, -10.4, -10.0, -9.6, -9.2, -8.8, -8.4, 8.4, 8.8, 9.2, 9.6, 10.0, 10.4,
    10.8, 11.2, 12.0,
];
const HELD_OUT_SHARE: f64 = 0.2;
const INDEXED_EXCERPTS: usize = 60;
const HELD_OUT_EXCERPTS: usize = 20;
pub const EXCERPT_SECONDS: f64 = 30.0;

#[derive(Serialize, Deserialize)]
pub struct SweepReport {
    pub seed: u64,
    pub held_out_clusters: usize,
    pub indexed_assets: usize,
    pub per_speed: Vec<SpeedRow>,
    pub queries: Vec<Query>,
    /// With the candidate trace: the lists the searched index set aside.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_aside: Option<SetAside>,
}

/// The lists an index leaves out of the search for candidates.
#[derive(Serialize, Deserialize)]
pub struct SetAside {
    pub census: FullestReport,
    /// Their hashes, in order.
    pub hashes: Vec<u32>,
}

/// A first-pass detection the second pass measures again (at least
/// `MIN_HITS_TO_REFINE` hits), with its first-pass evidence.
#[derive(Serialize, Deserialize)]
pub struct Candidate {
    pub asset: String,
    /// Whether the asset is in the excerpt's own cluster.
    pub correct: bool,
    pub hits: u32,
    pub windows: u32,
}

#[derive(Serialize, Deserialize)]
pub struct SpeedRow {
    pub speed_percent: f64,
    pub indexed_excerpts: usize,
    pub recalled: usize,
    pub recall: f64,
    pub wrong_answers: usize,
    /// Confident detections of a file judged to carry the excerpt's title
    /// (ADR 0022): right, and counted apart.
    #[serde(default, skip_serializing_if = "crate::titles::no_credits")]
    pub same_title: usize,
    pub mean_speed_error_percent: f64,
    pub max_speed_error_percent: f64,
}

/// One excerpt at one speed and what the search made of it.
#[derive(Serialize, Deserialize)]
pub struct Query {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
    pub speed_percent: f64,
    /// Every detection with at least two windows, strongest first.
    pub detections: Vec<Outcome>,
    /// With the candidate trace: the first pass's candidates, strongest
    /// first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub candidates: Option<Vec<Candidate>>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub asset: String,
    /// Whether the asset is in the excerpt's own cluster.
    pub correct: bool,
    /// Whether the asset is outside that cluster but judged to carry its
    /// title (`docs/same-titles.txt`, ADR 0022): right, counted apart.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub same_title: bool,
    pub confident: bool,
    pub windows: u32,
    pub hits: u32,
    pub speed_percent: f64,
    /// Counted by the second pass (`Pass::Fitted`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fitted: bool,
    /// Whether the asset is in the cluster of a track played beneath the
    /// excerpt (`recall`'s damaged queries), which is neither right nor
    /// wrong.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub partner: bool,
    /// The peak verifier's measures, with `--verify`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<Verification>,
}

impl Outcome {
    /// Whether the detection names the excerpt's title: its own cluster,
    /// or a file judged to carry the same title.
    pub fn is_right(&self) -> bool {
        self.correct || self.same_title
    }

    /// A confident detection that names another title, other than a
    /// damaged query's partner.
    pub fn is_wrong(&self) -> bool {
        self.confident && !self.is_right() && !self.partner
    }
}

/// An excerpt to render: which asset, from where.
#[derive(Serialize, Deserialize)]
pub struct Draw {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
}

/// The sweep's seeded choice: the held-out assets and the excerpts, indexed
/// ones first. Other harness commands reuse it to work on the same audio.
pub struct Plan {
    pub held_out: BTreeSet<String>,
    pub draws: Vec<Draw>,
}

/// A plan as first drawn for a seed, kept in `panels/` and reused, so that
/// the same excerpts and held-out recordings are measured as the library
/// grows. Assets added later are indexed, unless they are duplicates of a
/// held-out one.
#[derive(Serialize, Deserialize)]
struct Panel {
    seed: u64,
    /// Library assets when the panel was drawn.
    drawn_from_assets: usize,
    held_out: BTreeSet<String>,
    draws: Vec<Draw>,
}

impl Plan {
    /// The panel saved for `seed` in `panels`; drawn and saved there the
    /// first time. It is always read back from the file, so that every run
    /// sees the same rounding of its start times.
    pub fn for_seed(
        records: &[PeakRecord],
        clusters: &Clusters,
        seed: u64,
        panels: &Path,
    ) -> Result<Plan, String> {
        let path = panels.join(format!("sweep-seed-{seed}.json"));
        if !path.exists() {
            let plan = Plan::draw(records, clusters, seed);
            let panel = Panel {
                seed,
                drawn_from_assets: records.len(),
                held_out: plan.held_out,
                draws: plan.draws,
            };
            fs::create_dir_all(panels)
                .map_err(|error| format!("cannot create {}: {error}", panels.display()))?;
            let text = serde_json::to_string_pretty(&panel).map_err(|error| error.to_string())?;
            fs::write(&path, text)
                .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        }
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let panel: Panel =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
        let present: BTreeSet<&str> = records
            .iter()
            .map(|record| record.header.source.path.as_str())
            .collect();
        let missing: Vec<&str> = panel
            .draws
            .iter()
            .map(|draw| draw.asset.as_str())
            .filter(|asset| !present.contains(asset))
            .collect();
        if let Some(first) = missing.first() {
            return Err(format!(
                "{} draws an excerpt from {} assets no longer in the library, such as {first}",
                path.display(),
                missing.len()
            ));
        }
        Ok(Plan {
            held_out: panel
                .held_out
                .iter()
                .flat_map(|asset| clusters.cluster_of(asset))
                .collect(),
            draws: panel.draws,
        })
    }

    fn draw(records: &[PeakRecord], clusters: &Clusters, seed: u64) -> Plan {
        let mut rng = Rng::new(seed);
        let held_out = held_out_assets(records, clusters, &mut rng);
        let draws = draw_excerpts(records, &held_out, &mut rng);
        Plan { held_out, draws }
    }
}

pub struct Options<'a> {
    pub seed: u64,
    /// The files credited to another recording's title (ADR 0022).
    pub same_titles: &'a SameTitles,
    /// Where the seed's panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    /// What is added to the index: another library's records.
    pub padding: &'a Padding,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// Measure every detection with the peak verifier.
    pub verify: bool,
    /// With the other library's map: search only the indexed excerpts
    /// whose recording has a rip that is not an identical copy of the
    /// excerpt's source, with each such source and its identical copies
    /// left out of the index.
    pub other_rips: Option<&'a LibraryMap>,
    /// Keep each query's first-pass candidates and the lists set aside.
    pub trace: Trace,
    /// Set the fullest lists aside ranked by distinct holders instead of
    /// postings; `None` for the matcher's own index.
    pub ranking: Option<Ranking>,
    pub speeds: Speeds,
    pub jobs: usize,
}

/// The speeds the excerpts are played at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Speeds {
    /// The sweep's own, within the ladder (`SPEEDS_PERCENT`).
    Sweep,
    /// At and past the ladder's ends (`EDGE_SPEEDS_PERCENT`).
    Edge,
}

impl Speeds {
    pub fn percents(self) -> &'static [f64] {
        match self {
            Speeds::Sweep => &SPEEDS_PERCENT,
            Speeds::Edge => &EDGE_SPEEDS_PERCENT,
        }
    }
}

/// Whether the sweep keeps the first pass's candidates (session 50).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Trace {
    Off,
    Candidates,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<SweepReport, String> {
    let Options {
        seed,
        same_titles,
        panels,
        padding,
        ladder,
        matching,
        verify,
        other_rips,
        trace,
        ranking,
        speeds,
        jobs,
    } = *options;
    let speeds = speeds.percents();
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let verifier = verify.then(|| Verifier::new(&records, padding.second.as_ref()));
    let Plan { held_out, draws } = Plan::for_seed(&records, clusters, seed, panels)?;
    let (searched_draws, excluded) = match other_rips {
        Some(map) => other_rips_only(&draws, clusters, map, &held_out),
        None => ((0..draws.len()).collect(), held_out.clone()),
    };
    let built = padding.index(&records, &profile, &excluded)?;
    let index = match ranking {
        Some(ranking) => ranking.index(built, clusters),
        None => matching.index(built),
    };
    same_titles.warn_outside(&index, &excluded);
    let dir = work.join("sweep").join(format!("seed-{seed}"));
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let source = |draw: &Draw| library.root.join(&draw.asset);
    let renders: Vec<(&Draw, f64, PathBuf)> =
        render_all(source, draws.iter().enumerate(), speeds, &dir, jobs)?
            .into_iter()
            .enumerate()
            .filter(|(render, _)| searched_draws.contains(&(render / speeds.len())))
            .map(|(_, render)| render)
            .collect();

    // One excerpt per thread, each searched on a single thread.
    let searched: Vec<Result<Query, String>> =
        map_in_order(&renders, jobs, |(draw, speed_percent, path)| {
            let audio = decode(path, profile.sample_rate, Excerpt::default())
                .map_err(|error| error.to_string())?;
            let own_cluster = clusters.cluster_of(&draw.asset);
            let own_title = same_titles.credited(clusters, &own_cluster);
            let detections = matching.search(&index, &audio.samples, &profile, ladder, 1);
            let candidates = (trace == Trace::Candidates).then(|| {
                matching
                    .first_pass(&index, &audio.samples, &profile, ladder, 1)
                    .iter()
                    .filter(|detection| detection.evidence.hits >= MIN_HITS_TO_REFINE)
                    .map(|detection| candidate(&index, detection, &own_cluster))
                    .collect()
            });
            Ok(Query {
                asset: draw.asset.clone(),
                held_out: draw.held_out,
                start_seconds: draw.start_seconds,
                speed_percent: *speed_percent,
                detections: detections
                    .iter()
                    .map(|detection| {
                        let verified = verifier.as_ref().and_then(|verifier| {
                            verifier.verify(&index, detection, &audio.samples, &profile)
                        });
                        outcome(&index, detection, &own_cluster, &own_title, verified)
                    })
                    .collect(),
                candidates,
            })
        });
    let queries = searched
        .into_iter()
        .collect::<Result<Vec<Query>, String>>()?;

    Ok(SweepReport {
        seed,
        held_out_clusters: clusters_held_out(clusters, &held_out),
        indexed_assets: index.assets().len(),
        per_speed: speeds
            .iter()
            .map(|&speed| speed_row(speed, &queries))
            .collect(),
        queries,
        set_aside: (trace == Trace::Candidates).then(|| SetAside {
            census: fullest::census(&index, SKIPPED_SHARE),
            hashes: fullest::set_aside_hashes(&index),
        }),
    })
}

/// The positions in `draws` of the indexed excerpts whose recording keeps
/// a rip that is not an identical copy of the excerpt's source once every
/// such source and its copies are left out, and what the index leaves out:
/// the held-out recordings, and those sources with their copies.
fn other_rips_only(
    draws: &[Draw],
    clusters: &Clusters,
    map: &LibraryMap,
    held_out: &BTreeSet<String>,
) -> (BTreeSet<usize>, BTreeSet<String>) {
    let with_other_rips: Vec<usize> = (0..draws.len())
        .filter(|&number| !draws[number].held_out)
        .filter(|&number| {
            let identical = identical_copies(&draws[number].asset, map);
            clusters
                .cluster_of(&draws[number].asset)
                .iter()
                .any(|member| !identical.contains(member))
        })
        .collect();
    let mut excluded = held_out.clone();
    for &number in &with_other_rips {
        excluded.extend(identical_copies(&draws[number].asset, map));
    }
    let searched = with_other_rips
        .into_iter()
        .filter(|&number| {
            clusters
                .cluster_of(&draws[number].asset)
                .iter()
                .any(|member| !excluded.contains(member))
        })
        .collect();
    (searched, excluded)
}

/// `asset` and the files whose peak records are identical to its: its
/// copies in the other library, named as the index names them, and the
/// corpus files that stand for the same copy.
fn identical_copies(asset: &str, map: &LibraryMap) -> BTreeSet<String> {
    let mut identical = BTreeSet::from([asset.to_owned()]);
    if let Some(own) = map.copies.iter().find(|copies| copies.corpus == asset) {
        for copies in map
            .copies
            .iter()
            .filter(|copies| copies.stands_for == own.stands_for)
        {
            identical.insert(copies.corpus.clone());
            identical.extend(
                std::iter::once(&copies.stands_for)
                    .chain(&copies.also)
                    .map(|path| format!("{SECOND_LIBRARY_PREFIX}{path}")),
            );
        }
    }
    identical
}

/// Shuffles the clusters and holds out the first fifth of them, with every
/// member.
fn held_out_assets(records: &[PeakRecord], clusters: &Clusters, rng: &mut Rng) -> BTreeSet<String> {
    let mut keys: Vec<BTreeSet<String>> = records
        .iter()
        .map(|record| clusters.cluster_of(&record.header.source.path))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    rng.shuffle(&mut keys);
    let count = (keys.len() as f64 * HELD_OUT_SHARE).round() as usize;
    keys.into_iter().take(count).flatten().collect()
}

fn clusters_held_out(clusters: &Clusters, held_out: &BTreeSet<String>) -> usize {
    held_out
        .iter()
        .map(|asset| clusters.cluster_of(asset))
        .collect::<BTreeSet<_>>()
        .len()
}

fn draw_excerpts(records: &[PeakRecord], held_out: &BTreeSet<String>, rng: &mut Rng) -> Vec<Draw> {
    let long_enough: Vec<&PeakRecord> = records
        .iter()
        .filter(|record| record.header.duration_seconds >= 2.0 * EXCERPT_SECONDS)
        .collect();
    let mut draws = Vec::new();
    for (wanted, want_held_out) in [(INDEXED_EXCERPTS, false), (HELD_OUT_EXCERPTS, true)] {
        let mut pool: Vec<&&PeakRecord> = long_enough
            .iter()
            .filter(|record| held_out.contains(&record.header.source.path) == want_held_out)
            .collect();
        rng.shuffle(&mut pool);
        for record in pool.into_iter().take(wanted) {
            // Stay clear of intros and outros, which are often sparse.
            let duration = record.header.duration_seconds;
            let earliest = 0.15 * duration;
            let latest = 0.85 * duration - EXCERPT_SECONDS;
            draws.push(Draw {
                asset: record.header.source.path.clone(),
                held_out: want_held_out,
                start_seconds: earliest + rng.unit() * (latest - earliest).max(0.0),
            });
        }
    }
    draws
}

/// Renders every excerpt at every speed of `speeds` on `jobs` threads,
/// from the file `source` gives each draw, reusing files from an earlier
/// run with the same seed.
pub fn render_all<'d>(
    source: impl Fn(&Draw) -> PathBuf + Sync,
    draws: impl Iterator<Item = (usize, &'d Draw)>,
    speeds: &[f64],
    dir: &Path,
    jobs: usize,
) -> Result<Vec<(&'d Draw, f64, PathBuf)>, String> {
    let work: Vec<(&Draw, f64, PathBuf)> = draws
        .flat_map(|(number, draw)| {
            speeds.iter().map(move |&speed| {
                let name = format!(
                    "{number:03}-{}.{}",
                    speed_name(speed),
                    Encoding::Mp3(128).extension()
                );
                (draw, speed, dir.join(name))
            })
        })
        .collect();
    let failures: Vec<String> = map_in_order(&work, jobs, |(draw, speed, path)| {
        if crate::render::is_complete(path) {
            return None;
        }
        let speed_ratio = 1.0 + speed / 100.0;
        render_excerpt(
            &source(draw),
            draw.start_seconds,
            EXCERPT_SECONDS,
            speed_ratio,
            Encoding::Mp3(128),
            path,
        )
        .err()
    })
    .into_iter()
    .flatten()
    .collect();
    match failures.first() {
        Some(failure) => Err(failure.clone()),
        None => Ok(work),
    }
}

/// A speed in a render's name: whole percents as the sweep has always
/// named them (`+3`), others to a tenth (`+8.4`).
fn speed_name(speed_percent: f64) -> String {
    if speed_percent.fract() == 0.0 {
        format!("{speed_percent:+.0}")
    } else {
        format!("{speed_percent:+.1}")
    }
}

/// `own_title` holds the files outside `own_cluster` judged to carry its
/// title.
pub fn outcome(
    index: &Index,
    detection: &Detection,
    own_cluster: &BTreeSet<String>,
    own_title: &BTreeSet<String>,
    verified: Option<Verification>,
) -> Outcome {
    let asset = &index.asset(detection.asset).path;
    Outcome {
        asset: asset.clone(),
        correct: own_cluster.contains(original_path(asset)),
        same_title: own_title.contains(original_path(asset)),
        confident: detection.evidence.is_confident(),
        windows: detection.evidence.windows,
        hits: detection.evidence.hits,
        speed_percent: (detection.speed.0 - 1.0) * 100.0,
        fitted: detection.evidence.pass == Pass::Fitted,
        partner: false,
        verified,
    }
}

fn candidate(index: &Index, detection: &Detection, own_cluster: &BTreeSet<String>) -> Candidate {
    let asset = &index.asset(detection.asset).path;
    Candidate {
        asset: asset.clone(),
        correct: own_cluster.contains(original_path(asset)),
        hits: detection.evidence.hits,
        windows: detection.evidence.windows,
    }
}

pub fn speed_row<'a>(speed_percent: f64, queries: impl IntoIterator<Item = &'a Query>) -> SpeedRow {
    let at_speed: Vec<&Query> = queries
        .into_iter()
        .filter(|query| query.speed_percent == speed_percent)
        .collect();
    let indexed: Vec<&&Query> = at_speed.iter().filter(|query| !query.held_out).collect();
    let mut errors = Vec::new();
    for query in &indexed {
        let best_right = query
            .detections
            .iter()
            .find(|outcome| outcome.is_right() && outcome.confident);
        if let Some(outcome) = best_right {
            errors.push((outcome.speed_percent - speed_percent).abs());
        }
    }
    let wrong_answers = at_speed
        .iter()
        .flat_map(|query| &query.detections)
        .filter(|outcome| outcome.is_wrong())
        .count();
    let same_title = at_speed
        .iter()
        .flat_map(|query| &query.detections)
        .filter(|outcome| outcome.confident && outcome.same_title)
        .count();
    SpeedRow {
        speed_percent,
        indexed_excerpts: indexed.len(),
        recalled: errors.len(),
        recall: errors.len() as f64 / indexed.len().max(1) as f64,
        wrong_answers,
        same_title,
        mean_speed_error_percent: errors.iter().sum::<f64>() / errors.len().max(1) as f64,
        max_speed_error_percent: errors.iter().copied().fold(0.0, f64::max),
    }
}

pub fn print_summary(report: &SweepReport) {
    println!(
        "sweep seed {}: {} indexed assets, {} clusters held out",
        report.seed, report.indexed_assets, report.held_out_clusters
    );
    println!(
        "{:>7} {:>8} {:>8} {:>6} {:>6} {:>12} {:>11}",
        "speed", "recalled", "recall", "wrong", "title", "mean error", "max error"
    );
    for row in &report.per_speed {
        println!(
            "{:>6}% {:>4}/{:<3} {:>7.1}% {:>6} {:>6} {:>11.3}% {:>10.3}%",
            speed_name(row.speed_percent),
            row.recalled,
            row.indexed_excerpts,
            row.recall * 100.0,
            row.wrong_answers,
            row.same_title,
            row.mean_speed_error_percent,
            row.max_speed_error_percent
        );
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::store::RecordHeader;

    use super::*;

    fn record(path: &str) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: String::new(),
                source: Asset {
                    path: path.to_owned(),
                    size: 0,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 300.0,
            },
            peaks: Vec::new(),
        }
    }

    fn draw(asset: &str, held_out: bool) -> Draw {
        Draw {
            asset: asset.to_owned(),
            held_out,
            start_seconds: 60.0,
        }
    }

    fn copies(corpus: &str, stands_for: &str, also: &[&str]) -> crate::library_map::Copies {
        crate::library_map::Copies {
            corpus: corpus.to_owned(),
            stands_for: stands_for.to_owned(),
            same_size: true,
            also: also.iter().map(|path| (*path).to_owned()).collect(),
        }
    }

    #[test]
    fn renders_keep_whole_speeds_names_and_name_others_to_a_tenth() {
        assert_eq!(speed_name(-8.0), "-8");
        assert_eq!(speed_name(0.0), "+0");
        assert_eq!(speed_name(3.0), "+3");
        assert_eq!(speed_name(-10.0), "-10");
        assert_eq!(speed_name(8.4), "+8.4");
        assert_eq!(speed_name(-11.2), "-11.2");
    }

    #[test]
    fn edge_speeds_are_symmetric_and_hold_the_ends_of_wider_ladders() {
        let edge = Speeds::Edge.percents();
        let names: BTreeSet<String> = edge.iter().map(|&speed| speed_name(speed)).collect();

        assert_eq!(names.len(), edge.len());
        for &speed in edge {
            assert!(edge.contains(&-speed));
            assert!(speed.abs() > 8.0);
        }
        // With 1, 2, 3 and 5 extra rungs the turntable ladder ends here.
        for end in [8.4, 8.8, 9.2, 10.0] {
            assert!(edge.contains(&end));
        }
        assert_eq!(Speeds::Sweep.percents(), &SPEEDS_PERCENT);
    }

    #[test]
    fn other_rips_leave_out_the_source_and_its_identical_copies() {
        let names = |paths: &[&str]| paths.iter().map(|path| (*path).to_owned()).collect();
        let clusters = clusters(vec![
            names(&[
                "a.mp3",
                "a-rip.mp3",
                "second-library/A.mp3",
                "second-library/A copy.mp3",
            ]),
            names(&["b.mp3", "b-copy.mp3", "second-library/B.mp3"]),
        ]);
        let map = LibraryMap {
            other_library: None,
            corpus_records: 3,
            other_records: 3,
            copies: vec![
                copies("a.mp3", "A.mp3", &["A copy.mp3"]),
                copies("b.mp3", "B.mp3", &[]),
                copies("b-copy.mp3", "B.mp3", &[]),
            ],
            without_copy: Vec::new(),
        };
        let draws = [
            draw("a.mp3", false),
            draw("b.mp3", false),
            draw("h.mp3", true),
        ];
        let held_out = BTreeSet::from([String::from("h.mp3")]);

        let (searched, excluded) = other_rips_only(&draws, &clusters, &map, &held_out);

        // b's cluster holds only identical copies of it: nothing to find.
        assert_eq!(searched, BTreeSet::from([0]));
        assert_eq!(
            excluded.into_iter().collect::<Vec<_>>(),
            names(&[
                "a.mp3",
                "h.mp3",
                "second-library/A copy.mp3",
                "second-library/A.mp3"
            ])
        );
    }

    fn clusters(duplicates: Vec<Vec<String>>) -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates,
            pairs: Vec::new(),
            cut_links: Vec::new(),
        }
    }

    #[test]
    fn a_panel_keeps_its_draws_as_the_library_grows() {
        let panels = std::env::temp_dir().join(format!("gunfinger-panels-{}", std::process::id()));
        let library: Vec<PeakRecord> = (0..50).map(|n| record(&format!("{n:02}.mp3"))).collect();
        let first = Plan::for_seed(&library, &clusters(Vec::new()), 7, &panels).unwrap();
        let held_out = first.held_out.first().unwrap().clone();

        // A new rip of a held-out recording, and unrelated new tracks.
        let rip = String::from("new rip.mp3");
        let mut grown = library;
        grown.extend((0..30).map(|n| record(&format!("new {n:02}.mp3"))));
        grown.push(record(&rip));
        let grown_clusters = clusters(vec![vec![held_out, rip.clone()]]);
        let again = Plan::for_seed(&grown, &grown_clusters, 7, &panels).unwrap();
        let removed = Plan::for_seed(&grown[1..], &grown_clusters, 7, &panels);
        fs::remove_dir_all(&panels).unwrap();

        let excerpts = |plan: &Plan| -> Vec<(String, u64)> {
            plan.draws
                .iter()
                .map(|draw| (draw.asset.clone(), draw.start_seconds.to_bits()))
                .collect()
        };
        assert_eq!(excerpts(&again), excerpts(&first));
        assert!(again.held_out.is_superset(&first.held_out));
        assert_eq!(again.held_out.len(), first.held_out.len() + 1);
        assert!(again.held_out.contains(&rip));
        assert!(
            removed.is_err() == first.draws.iter().any(|draw| draw.asset == "00.mp3"),
            "a panel whose excerpt's asset is gone is an error"
        );
    }

    fn query(held_out: bool, detections: Vec<Outcome>) -> Query {
        Query {
            asset: "own.mp3".to_owned(),
            held_out,
            start_seconds: 60.0,
            speed_percent: 3.0,
            detections,
            candidates: None,
        }
    }

    fn confident(asset: &str, correct: bool, same_title: bool) -> Outcome {
        Outcome {
            asset: asset.to_owned(),
            correct,
            same_title,
            confident: true,
            windows: 10,
            hits: 600,
            speed_percent: 3.0,
            fitted: true,
            partner: false,
            verified: None,
        }
    }

    #[test]
    fn a_same_title_detection_is_right_and_counted_apart() {
        let queries = [
            // Found only as a version judged to carry its title.
            query(false, vec![confident("version.opus", false, true)]),
            // Held out, found as that version and as an unjudged file.
            query(
                true,
                vec![
                    confident("version.opus", false, true),
                    confident("other.opus", false, false),
                ],
            ),
        ];

        let row = speed_row(3.0, &queries);

        assert_eq!((row.indexed_excerpts, row.recalled), (1, 1));
        assert_eq!((row.wrong_answers, row.same_title), (1, 2));
    }

    #[test]
    fn an_outcome_is_same_title_for_a_judged_file_and_its_padding_copies() {
        use gunfinger_core::confidence::Evidence;
        use gunfinger_core::index::AssetId;
        use gunfinger_core::speed::{Playback, SpeedRatio};

        let records = [
            record("own.mp3"),
            record("second-library/version.opus"),
            record("padding-copy/1/second-library/version.opus"),
            record("second-library/other.opus"),
        ];
        let index = Index::build(&records).unwrap();
        let own_cluster = BTreeSet::from(["own.mp3".to_owned()]);
        let own_title = BTreeSet::from(["second-library/version.opus".to_owned()]);
        let judged = |asset: u32| {
            let detection = Detection {
                asset: AssetId(asset),
                start_seconds: 0.0,
                end_seconds: 30.0,
                track_start_seconds: 60.0,
                track_end_seconds: 90.0,
                speed: SpeedRatio(1.0),
                playback: Playback::Turntable,
                evidence: Evidence::new(10, 600),
            };
            let outcome = outcome(&index, &detection, &own_cluster, &own_title, None);
            (outcome.correct, outcome.same_title, outcome.is_wrong())
        };

        assert_eq!(judged(0), (true, false, false));
        assert_eq!(judged(1), (false, true, false));
        assert_eq!(judged(2), (false, true, false));
        assert_eq!(judged(3), (false, false, true));
    }
}

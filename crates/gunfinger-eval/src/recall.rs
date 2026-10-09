//! Recall beyond the corpus recordings: a panel of sources drawn from the
//! larger library the corpus was drawn from (`--other-peaks-dir`), whose
//! excerpts are rendered and searched as the sweep's are.
//!
//! The sources are that library's records outside the clusters of the
//! corpus recordings and not copies of corpus files, tagged with an artist
//! and a title, whose length lies in the track length range the library
//! was indexed with. Their recording families (the same normalised artist
//! and title) are split by seed into a development and a validation half,
//! so that no recording is measured in both. Each half draws 60 indexed and
//! 20 held-out sources, one per family, a third from each kind of source.
//! The panel is drawn once per seed, before any result is seen, and kept in
//! `panels/`; only the development half is searched.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Rule;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::padding::{Padding, SECOND_LIBRARY_PREFIX};
use crate::rng::Rng;
use crate::sweep::{
    Draw, EXCERPT_SECONDS, Query, SPEEDS_PERCENT, SpeedRow, outcome, render_all, speed_row,
};

const INDEXED_SOURCES: usize = 60;
const HELD_OUT_SOURCES: usize = 20;
/// The track length range the owner's configuration indexes the NAS
/// library with (1:30 to 15:00).
const MIN_SOURCE_SECONDS: f64 = 90.0;
const MAX_SOURCE_SECONDS: f64 = 900.0;

/// Where a source comes from, by the library's top folders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// Scene releases, under `__full_scene/`.
    SceneRelease,
    /// YouTube channels, under `__youtube_archivists/`.
    YoutubeChannel,
    /// Label packs, vinyl rips and the rest.
    Elsewhere,
}

const KINDS: [Kind; 3] = [Kind::SceneRelease, Kind::YoutubeChannel, Kind::Elsewhere];

impl Kind {
    fn of(path: &str) -> Kind {
        if path.starts_with("__full_scene/") {
            Kind::SceneRelease
        } else if path.starts_with("__youtube_archivists/") {
            Kind::YoutubeChannel
        } else {
            Kind::Elsewhere
        }
    }
}

/// A panel as first drawn for a seed.
#[derive(Serialize, Deserialize)]
pub struct Panel {
    pub seed: u64,
    /// The other library's current records when it was drawn, those that
    /// could be sources, and their families.
    pub drawn_from_records: usize,
    pub pool: usize,
    pub families: usize,
    pub development: Half,
    pub validation: Half,
}

#[derive(Serialize, Deserialize)]
pub struct Half {
    pub families: usize,
    /// Indexed sources first, then held-out ones.
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// As the index names it: `second-library/<path>`.
    pub asset: String,
    pub kind: Kind,
    pub family: String,
    pub held_out: bool,
    pub start_seconds: f64,
}

/// A record that may be drawn.
struct Candidate {
    asset: String,
    kind: Kind,
    family: String,
    duration_seconds: f64,
}

impl Panel {
    /// The panel saved for `seed` in `panels`, drawn from the other
    /// library's records and saved there the first time.
    pub fn for_seed(
        other: &PeakStore,
        assets: &[Asset],
        corpus_recordings: &BTreeSet<String>,
        seed: u64,
        panels: &Path,
    ) -> Result<Panel, String> {
        let path = panels.join(format!("recall-seed-{seed}.json"));
        if !path.exists() {
            let candidates = candidates(other, assets, corpus_recordings);
            let panel = draw(&candidates, assets.len(), seed);
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
        let present: BTreeSet<String> = assets
            .iter()
            .map(|asset| format!("{SECOND_LIBRARY_PREFIX}{}", asset.path))
            .collect();
        let missing = panel
            .development
            .sources
            .iter()
            .filter(|source| !present.contains(&source.asset))
            .count();
        if missing > 0 {
            return Err(format!(
                "{} draws {missing} development sources that no longer have a record",
                path.display()
            ));
        }
        Ok(panel)
    }
}

/// The other library's records that may be sources: not among
/// `corpus_recordings` (their clusters and copies of corpus files), tagged
/// with an artist and a title, and in the track length range.
fn candidates(
    other: &PeakStore,
    assets: &[Asset],
    corpus_recordings: &BTreeSet<String>,
) -> Vec<Candidate> {
    let profile = Profile::CURRENT;
    assets
        .iter()
        .filter_map(|asset| {
            let name = format!("{SECOND_LIBRARY_PREFIX}{}", asset.path);
            if corpus_recordings.contains(&name) {
                return None;
            }
            let duration_seconds = other.current_header(asset, &profile)?.duration_seconds;
            if !(MIN_SOURCE_SECONDS..=MAX_SOURCE_SECONDS).contains(&duration_seconds)
                || duration_seconds < 2.0 * EXCERPT_SECONDS
            {
                return None;
            }
            let tags = other.tags(asset)?;
            Some(Candidate {
                kind: Kind::of(&asset.path),
                family: family(tags.artist.as_deref()?, tags.title.as_deref()?),
                asset: name,
                duration_seconds,
            })
        })
        .collect()
}

/// Artist and title in lower case, with every run of other characters than
/// letters and digits as one space.
fn family(artist: &str, title: &str) -> String {
    let text = format!("{artist} - {title}").to_lowercase();
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Splits the families in two by seed, then draws each half.
fn draw(candidates: &[Candidate], records: usize, seed: u64) -> Panel {
    let mut rng = Rng::new(seed);
    let mut families: Vec<&str> = candidates
        .iter()
        .map(|candidate| candidate.family.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    rng.shuffle(&mut families);
    let (development, validation) = families.split_at(families.len() / 2);
    let mut half = |families: &[&str]| {
        let families: BTreeSet<&str> = families.iter().copied().collect();
        let members: Vec<&Candidate> = candidates
            .iter()
            .filter(|candidate| families.contains(candidate.family.as_str()))
            .collect();
        Half {
            families: families.len(),
            sources: draw_half(&members, &mut rng),
        }
    };
    Panel {
        seed,
        drawn_from_records: records,
        pool: candidates.len(),
        families: families.len(),
        development: half(development),
        validation: half(validation),
    }
}

/// 60 indexed and 20 held-out sources, one per family, an equal share from
/// each kind, each with an excerpt clear of the intro and outro as the
/// sweep draws them.
fn draw_half(members: &[&Candidate], rng: &mut Rng) -> Vec<Source> {
    let mut taken_families = BTreeSet::new();
    let mut sources = Vec::new();
    for (wanted, held_out) in [(INDEXED_SOURCES, false), (HELD_OUT_SOURCES, true)] {
        for (number, &kind) in KINDS.iter().enumerate() {
            let share = wanted / KINDS.len() + usize::from(number < wanted % KINDS.len());
            let mut pool: Vec<&&Candidate> = members
                .iter()
                .filter(|candidate| candidate.kind == kind)
                .collect();
            rng.shuffle(&mut pool);
            let mut taken = 0;
            for candidate in pool {
                if taken == share {
                    break;
                }
                if !taken_families.insert(candidate.family.as_str()) {
                    continue;
                }
                let duration = candidate.duration_seconds;
                let earliest = 0.15 * duration;
                let latest = 0.85 * duration - EXCERPT_SECONDS;
                sources.push(Source {
                    asset: candidate.asset.clone(),
                    kind,
                    family: candidate.family.clone(),
                    held_out,
                    start_seconds: earliest + rng.unit() * (latest - earliest).max(0.0),
                });
                taken += 1;
            }
        }
    }
    sources
}

/// The development half's excerpts, rendered at every speed from the other
/// library's audio under `root` into `dir`, reusing earlier renders.
pub fn render(
    panel: &Panel,
    root: &Path,
    dir: &Path,
    jobs: usize,
) -> Result<Vec<(Source, f64, PathBuf)>, String> {
    let draws: Vec<Draw> = panel
        .development
        .sources
        .iter()
        .map(|source| Draw {
            asset: source.asset.clone(),
            held_out: source.held_out,
            start_seconds: source.start_seconds,
        })
        .collect();
    fs::create_dir_all(dir).map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    let source_file = |draw: &Draw| {
        root.join(
            draw.asset
                .strip_prefix(SECOND_LIBRARY_PREFIX)
                .unwrap_or(&draw.asset),
        )
    };
    let rendered = render_all(source_file, &draws, dir, jobs)?;
    Ok(rendered
        .into_iter()
        .enumerate()
        .map(|(number, (_, speed, path))| {
            let source = &panel.development.sources[number / SPEEDS_PERCENT.len()];
            (source.clone(), speed, path)
        })
        .collect())
}

/// Why an indexed excerpt was not recalled: its own cluster never became a
/// candidate, or the strongest detection of it had too few hits for the
/// rule, or enough hits in too few windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Miss {
    NeverACandidate,
    TooFewHits,
    TooFewWindows,
}

impl Miss {
    /// `None` when the query was recalled or held out.
    pub fn of(query: &Query, rule: Rule) -> Option<Miss> {
        if query.held_out
            || query
                .detections
                .iter()
                .any(|outcome| outcome.correct && outcome.confident)
        {
            return None;
        }
        let strongest = query
            .detections
            .iter()
            .filter(|outcome| outcome.correct)
            .max_by_key(|outcome| outcome.hits);
        Some(match strongest {
            None => Miss::NeverACandidate,
            Some(outcome) if outcome.hits < rule.min_hits => Miss::TooFewHits,
            Some(_) => Miss::TooFewWindows,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecallReport {
    pub seed: u64,
    pub indexed_assets: usize,
    pub held_out_clusters: usize,
    pub per_speed: Vec<SpeedRow>,
    /// Recall per kind of source, over every speed.
    pub per_kind: Vec<KindRow>,
    pub queries: Vec<RecallQuery>,
}

#[derive(Serialize, Deserialize)]
pub struct KindRow {
    pub kind: Kind,
    pub indexed_queries: usize,
    pub recalled: usize,
    pub wrong_answers: usize,
    pub held_out_queries: usize,
    pub held_out_confident: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RecallQuery {
    pub kind: Kind,
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miss: Option<Miss>,
    #[serde(flatten)]
    pub query: Query,
}

pub struct Options<'a> {
    pub seed: u64,
    pub padding: &'a Padding,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub jobs: usize,
}

/// Searches the development half's rendered excerpts against the corpus
/// and the other library, with the held-out sources' clusters left out.
/// Indexed excerpts must return their own cluster, held-out ones nothing
/// confident.
pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    rendered: &[(Source, f64, PathBuf)],
    options: &Options,
) -> Result<RecallReport, String> {
    let Options {
        seed,
        padding,
        ladder,
        matching,
        jobs,
    } = *options;
    let profile = Profile::CURRENT;
    let held_out: BTreeSet<String> = rendered
        .iter()
        .filter(|(source, _, _)| source.held_out)
        .flat_map(|(source, _, _)| clusters.cluster_of(&source.asset))
        .collect();
    let (records, _) = load_records(library, store, &profile, &held_out);
    let index = matching.index(padding.index(&records, &profile, &held_out)?);
    drop(records);

    let searched: Vec<Result<RecallQuery, String>> =
        map_in_order(rendered, jobs, |(source, speed_percent, path)| {
            let audio = decode(path, profile.sample_rate, Excerpt::default())
                .map_err(|error| error.to_string())?;
            let own_cluster = clusters.cluster_of(&source.asset);
            let detections = matching.search(&index, &audio.samples, &profile, ladder, 1);
            let query = Query {
                asset: source.asset.clone(),
                held_out: source.held_out,
                start_seconds: source.start_seconds,
                speed_percent: *speed_percent,
                detections: detections
                    .iter()
                    .map(|detection| outcome(&index, detection, &own_cluster, None))
                    .collect(),
            };
            Ok(RecallQuery {
                kind: source.kind,
                family: source.family.clone(),
                miss: Miss::of(&query, matching.rule()),
                query,
            })
        });
    let queries = searched
        .into_iter()
        .collect::<Result<Vec<RecallQuery>, String>>()?;
    Ok(RecallReport {
        seed,
        indexed_assets: index.assets().len(),
        held_out_clusters: held_out
            .iter()
            .map(|asset| clusters.cluster_of(asset))
            .collect::<BTreeSet<_>>()
            .len(),
        per_speed: SPEEDS_PERCENT
            .iter()
            .map(|&speed| speed_row(speed, queries.iter().map(|query| &query.query)))
            .collect(),
        per_kind: KINDS.iter().map(|&kind| kind_row(kind, &queries)).collect(),
        queries,
    })
}

fn kind_row(kind: Kind, queries: &[RecallQuery]) -> KindRow {
    let of_kind: Vec<&Query> = queries
        .iter()
        .filter(|query| query.kind == kind)
        .map(|query| &query.query)
        .collect();
    let (held_out, indexed): (Vec<&Query>, Vec<&Query>) =
        of_kind.into_iter().partition(|query| query.held_out);
    KindRow {
        kind,
        indexed_queries: indexed.len(),
        recalled: indexed
            .iter()
            .filter(|query| {
                query
                    .detections
                    .iter()
                    .any(|outcome| outcome.correct && outcome.confident)
            })
            .count(),
        wrong_answers: indexed.iter().map(|query| confident_wrong(query)).sum(),
        held_out_queries: held_out.len(),
        held_out_confident: held_out
            .iter()
            .filter(|query| confident_wrong(query) > 0)
            .count(),
    }
}

fn confident_wrong(query: &Query) -> usize {
    query
        .detections
        .iter()
        .filter(|outcome| outcome.confident && !outcome.correct)
        .count()
}

pub fn print_summary(report: &RecallReport) {
    println!(
        "recall panel seed {}, development half: {} indexed assets, {} clusters held out",
        report.seed, report.indexed_assets, report.held_out_clusters
    );
    println!(
        "{:>7} {:>8} {:>8} {:>6}",
        "speed", "recalled", "recall", "wrong"
    );
    for row in &report.per_speed {
        println!(
            "{:>+6.0}% {:>4}/{:<3} {:>7.1}% {:>6}",
            row.speed_percent,
            row.recalled,
            row.indexed_excerpts,
            row.recall * 100.0,
            row.wrong_answers
        );
    }
    for row in &report.per_kind {
        println!(
            "{:?}: {}/{} recalled, {} wrong answers; held out: {} of {} with a confident answer",
            row.kind,
            row.recalled,
            row.indexed_queries,
            row.wrong_answers,
            row.held_out_confident,
            row.held_out_queries
        );
    }
    let mut misses: BTreeMap<String, usize> = BTreeMap::new();
    for query in &report.queries {
        if let Some(miss) = query.miss {
            *misses.entry(format!("{miss:?}")).or_default() += 1;
        }
    }
    println!("misses: {misses:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sweep::Outcome;

    fn candidate(asset: &str, family: &str) -> Candidate {
        Candidate {
            asset: format!("{SECOND_LIBRARY_PREFIX}{asset}"),
            kind: Kind::of(asset),
            family: family.to_owned(),
            duration_seconds: 300.0,
        }
    }

    #[test]
    fn a_family_is_its_normalised_artist_and_title() {
        assert_eq!(
            family("Ed Rush & Optical", "Dozer (VIP)"),
            "ed rush optical dozer vip"
        );
        assert_eq!(
            family("ED RUSH  & OPTICAL", "Dozer [VIP]"),
            family("Ed Rush & Optical", "Dozer (VIP)")
        );
    }

    #[test]
    fn kinds_come_from_the_top_folders() {
        assert_eq!(Kind::of("__full_scene/1999/a.mp3"), Kind::SceneRelease);
        assert_eq!(
            Kind::of("__youtube_archivists/channel/a.opus"),
            Kind::YoutubeChannel
        );
        assert_eq!(Kind::of("Label - Pack/a.mp3"), Kind::Elsewhere);
    }

    #[test]
    fn halves_share_no_family_and_draw_one_source_per_family() {
        let mut candidates = Vec::new();
        for number in 0..400 {
            let family = format!("artist {} - title", number / 2);
            for folder in ["__full_scene/s", "__youtube_archivists/c", "label"] {
                candidates.push(candidate(&format!("{folder}/{number}.mp3"), &family));
            }
        }

        let panel = draw(&candidates, candidates.len(), 2026);

        let families = |half: &Half| -> BTreeSet<String> {
            half.sources
                .iter()
                .map(|source| source.family.clone())
                .collect()
        };
        for half in [&panel.development, &panel.validation] {
            assert_eq!(half.sources.len(), 80);
            assert_eq!(families(half).len(), 80, "one source per family");
            let held_out: Vec<Kind> = half
                .sources
                .iter()
                .filter(|source| source.held_out)
                .map(|source| source.kind)
                .collect();
            assert_eq!(held_out.len(), 20);
            assert_eq!(
                held_out
                    .iter()
                    .filter(|&&kind| kind == Kind::Elsewhere)
                    .count(),
                6
            );
            assert!(half.sources[..60].iter().all(|source| !source.held_out));
        }
        assert!(families(&panel.development).is_disjoint(&families(&panel.validation)));
        assert_eq!(panel.families, 200);
    }

    fn query(held_out: bool, outcomes: &[(bool, bool, u32, u32)]) -> Query {
        Query {
            asset: String::from("second-library/a.mp3"),
            held_out,
            start_seconds: 60.0,
            speed_percent: 0.0,
            detections: outcomes
                .iter()
                .map(|&(correct, confident, hits, windows)| Outcome {
                    asset: String::new(),
                    correct,
                    confident,
                    windows,
                    hits,
                    speed_percent: 0.0,
                    fitted: true,
                    verified: None,
                })
                .collect(),
        }
    }

    #[test]
    fn a_miss_says_why_the_own_cluster_was_not_confident() {
        let rule = gunfinger_core::confidence::FITTED_RULE;
        let miss = |held_out, outcomes: &[(bool, bool, u32, u32)]| {
            Miss::of(&query(held_out, outcomes), rule)
        };

        assert_eq!(miss(false, &[(true, true, 900, 3)]), None);
        assert_eq!(miss(true, &[]), None);
        assert_eq!(
            miss(false, &[(false, true, 900, 3)]),
            Some(Miss::NeverACandidate)
        );
        assert_eq!(
            miss(false, &[(true, false, 120, 3), (true, false, 80, 4)]),
            Some(Miss::TooFewHits)
        );
        assert_eq!(
            miss(false, &[(true, false, 400, 2)]),
            Some(Miss::TooFewWindows)
        );
    }
}

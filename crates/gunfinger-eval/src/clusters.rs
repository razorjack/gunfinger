//! Duplicate clusters: library files that are the same recording.
//!
//! Each file is searched against the whole library on a narrow ladder (rips
//! of one recording differ in speed by a percent at most). Two files are the
//! same recording when a single alignment covers nearly all of the shorter
//! one. The criterion is deliberately strict: a remix or VIP that shares some
//! sections with the original must stay separate. Clusters depend on library
//! audio alone, never on what a set scan returned.
//!
//! With `Source::Peaks` each file's stored peaks are searched instead of its
//! decoded audio: no decoding, so it scales to a large library, at the cost
//! of the hashes that rescaled peaks lose (`search::search_peaks`).
//!
//! In a larger library the corpus was drawn from, `find_around` finds the
//! clusters of the corpus recordings only: clustering every file of tens of
//! thousands against every other would take days.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::{TrackLength, build_index, load_records};
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search, search_peaks};
use gunfinger_core::speed::{Rung, SpeedRatio};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::padding::SECOND_LIBRARY_PREFIX;

/// The alignment must cover this share of the shorter file.
const MIN_COVERAGE: f64 = 0.8;
/// Pairs above this coverage but below `MIN_COVERAGE` are reported for review.
const REPORTED_COVERAGE: f64 = 0.2;
/// In another library, pairs with this many hits are kept whatever their
/// coverage: the census of related recordings (experiment 0015).
const RELATED_HITS: u32 = 30;
/// Pairs from this coverage up to `MIN_COVERAGE` go to the owner to judge.
const BORDERLINE_COVERAGE: f64 = 0.4;
/// The other library was indexed with its own track length range.
const ANY_LENGTH: TrackLength = TrackLength {
    min: Duration::ZERO,
    max: Duration::from_secs(24 * 3600),
};
/// Rips of one recording play within this speed of each other.
const SPEEDS: [f64; 11] = [
    0.98, 0.984, 0.988, 0.992, 0.996, 1.0, 1.004, 1.008, 1.012, 1.016, 1.02,
];

/// Every library asset's cluster. Assets without a duplicate form a cluster
/// of their own.
#[derive(Debug, Serialize, Deserialize)]
pub struct Clusters {
    pub criterion: String,
    /// Clusters with more than one member, each sorted, ordered by first member.
    pub duplicates: Vec<Vec<String>>,
    /// The evidence for every pair considered, strongest first.
    pub pairs: Vec<Pair>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pair {
    pub query: String,
    pub found: String,
    /// Share of the shorter file covered by the alignment.
    pub coverage: f64,
    pub speed: f64,
    pub hits: u32,
    pub same_recording: bool,
    /// Where the alignment lies in each file, in seconds.
    #[serde(default)]
    pub query_start_seconds: f64,
    #[serde(default)]
    pub query_end_seconds: f64,
    #[serde(default)]
    pub found_start_seconds: f64,
    #[serde(default)]
    pub found_end_seconds: f64,
}

impl Clusters {
    pub fn load(path: &Path) -> Result<Clusters, String> {
        let text = fs::read_to_string(path).map_err(|error| {
            format!(
                "cannot read {} ({error}); run `gunfinger-eval clusters` first",
                path.display()
            )
        })?;
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// The members of the cluster holding `asset`, including itself.
    pub fn cluster_of(&self, asset: &str) -> BTreeSet<String> {
        self.duplicates
            .iter()
            .find(|members| members.iter().any(|member| member == asset))
            .map_or_else(
                || BTreeSet::from([asset.to_owned()]),
                |members| members.iter().cloned().collect(),
            )
    }
}

/// What each library file is searched with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// Its decoded audio.
    Audio,
    /// Its stored peaks, rescaled to each speed.
    Peaks,
}

pub fn find(
    library: &Library,
    store: &PeakStore,
    source: Source,
    jobs: usize,
) -> Result<Clusters, String> {
    let mut pairs = self_match(library, store, source, jobs, |pair| {
        pair.coverage >= REPORTED_COVERAGE
    })?;
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));

    Ok(Clusters {
        criterion: format!(
            "one alignment at speed {:.2}..{:.2} covers at least {:.0}% of the shorter file{}",
            SPEEDS[0],
            SPEEDS[SPEEDS.len() - 1],
            MIN_COVERAGE * 100.0,
            match source {
                Source::Audio => "",
                Source::Peaks => " (stored peaks searched)",
            }
        ),
        duplicates: merge(same_recording(&pairs)),
        pairs,
    })
}

/// The clusters of the corpus recordings in another library. Each query
/// (a corpus record) is searched by its stored peaks against an index of
/// `other_assets` alone; files of the other library found to be the same
/// recording are searched in turn, until none is new, so that chains of
/// rips are followed. `copies` (corpus files' identical copies) are not
/// searched again. Files of the other library are named
/// `second-library/<path>`, as in an index of both. Pairs are kept from 20%
/// coverage or 30 hits.
pub fn find_around(
    queries: Vec<PeakRecord>,
    other: &PeakStore,
    other_assets: &[Asset],
    copies: &BTreeSet<String>,
    jobs: usize,
) -> Result<Vec<Pair>, String> {
    let profile = Profile::CURRENT;
    let built = build_index(other_assets, other, &profile, &BTreeSet::new(), ANY_LENGTH)
        .map_err(|error| error.to_string())?;
    for problem in &built.problems {
        eprintln!("left out: {problem}");
    }
    let index = built.index;
    let by_path: BTreeMap<&str, &Asset> = other_assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let ladder = clustering_ladder();
    let mut searched: BTreeSet<String> = queries
        .iter()
        .map(|record| record.header.source.path.clone())
        .collect();
    let mut round = queries;
    let mut pairs = Vec::new();
    let mut number = 0;
    while !round.is_empty() {
        number += 1;
        eprintln!("round {number}: {} queries", round.len());
        let finished = AtomicUsize::new(0);
        let found: Vec<Vec<Pair>> = map_in_order(&round, jobs, |record| {
            let query = &record.header.source.path;
            let detections = search_peaks(&index, &record.peaks, &profile, &ladder, 1);
            let kept = detections
                .iter()
                .filter_map(|detection| {
                    let asset = index.asset(detection.asset);
                    let found = format!("{SECOND_LIBRARY_PREFIX}{}", asset.path);
                    let shorter = record.header.duration_seconds.min(asset.duration_seconds);
                    (found != *query).then(|| pair(query, found, shorter, detection))
                })
                .filter(|pair| pair.coverage >= REPORTED_COVERAGE || pair.hits >= RELATED_HITS)
                .collect();
            let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
            eprintln!("[{count}/{}] {query}", round.len());
            kept
        });
        let new: BTreeSet<String> = found
            .iter()
            .flatten()
            .filter(|pair| pair.same_recording && !searched.contains(&pair.found))
            .map(|pair| pair.found.clone())
            .filter(|found| {
                found
                    .strip_prefix(SECOND_LIBRARY_PREFIX)
                    .is_some_and(|path| !copies.contains(path))
            })
            .collect();
        round = Vec::new();
        for found in new {
            let path = &found[SECOND_LIBRARY_PREFIX.len()..];
            let Some(&asset) = by_path.get(path) else {
                continue;
            };
            let mut record = other
                .load(asset, &profile)
                .map_err(|error| error.to_string())?;
            record.header.source.path = found.clone();
            round.push(record);
            searched.insert(found);
        }
        pairs.extend(found.into_iter().flatten());
    }
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));
    Ok(pairs)
}

/// The corpus clusters joined by the same-recording pairs found in another
/// library (`find_around`), which are kept as the evidence.
pub fn merged(corpus: &Clusters, pairs: Vec<Pair>, other_library: &str) -> Clusters {
    let corpus_links = corpus.duplicates.iter().flat_map(|members| {
        members
            .iter()
            .map(|member| (members[0].as_str(), member.as_str()))
    });
    Clusters {
        criterion: format!(
            "{}; with the rips of the corpus recordings in {other_library} (stored peaks searched, named {SECOND_LIBRARY_PREFIX}<path>)",
            corpus.criterion
        ),
        duplicates: merge(corpus_links.chain(same_recording(&pairs))),
        pairs,
    }
}

/// What `find_around` found: whether the corpus clusters reappear through
/// the other library, the other library's further rips of the corpus
/// recordings, and borderline pairs for the owner to judge by ear.
pub fn print_around(corpus: &Clusters, pairs: &[Pair], copies: &BTreeSet<String>) {
    let is_corpus = |path: &str| !path.starts_with(SECOND_LIBRARY_PREFIX);
    let found_again: Vec<Vec<String>> = merge(same_recording(pairs))
        .into_iter()
        .map(|members| {
            members
                .into_iter()
                .filter(|member| is_corpus(member))
                .collect::<Vec<_>>()
        })
        .filter(|members| members.len() > 1)
        .collect();
    let projected = Clusters {
        criterion: String::new(),
        duplicates: found_again,
        pairs: Vec::new(),
    };
    print_differences("the other library", &projected, "the corpus", corpus);
    let further: Vec<&Pair> = pairs
        .iter()
        .filter(|pair| {
            pair.same_recording
                && pair
                    .found
                    .strip_prefix(SECOND_LIBRARY_PREFIX)
                    .is_some_and(|path| !copies.contains(path))
        })
        .collect();
    let further_files: BTreeSet<&str> = further.iter().map(|pair| pair.found.as_str()).collect();
    println!(
        "{} further rips of corpus recordings (not copies of corpus files):",
        further_files.len()
    );
    for pair in further {
        println!(
            "  {} = {} ({:.0}%, {} hits)",
            pair.query,
            pair.found,
            100.0 * pair.coverage,
            pair.hits
        );
    }
    let borderline: Vec<&Pair> = pairs
        .iter()
        .filter(|pair| pair.coverage >= BORDERLINE_COVERAGE && !pair.same_recording)
        .collect();
    println!(
        "{} borderline pairs ({:.0}% to {:.0}% coverage), for the owner:",
        borderline.len(),
        100.0 * BORDERLINE_COVERAGE,
        100.0 * MIN_COVERAGE
    );
    for pair in borderline {
        println!(
            "  {} ~ {} ({:.0}%, {} hits)",
            pair.query,
            pair.found,
            100.0 * pair.coverage,
            pair.hits
        );
    }
    let related = pairs
        .iter()
        .filter(|pair| pair.coverage < REPORTED_COVERAGE)
        .count();
    println!(
        "{related} pairs below 20% coverage with {RELATED_HITS} hits or more (related recordings)"
    );
}

/// Rips of one recording play within a percent or two of each other.
fn clustering_ladder() -> Vec<Rung> {
    SPEEDS
        .into_iter()
        .map(|speed| Rung::Turntable(SpeedRatio(speed)))
        .collect()
}

fn pair(query: &str, found: String, shorter: f64, detection: &Detection) -> Pair {
    let coverage = (detection.end_seconds - detection.start_seconds) / shorter;
    Pair {
        query: query.to_owned(),
        found,
        coverage,
        speed: detection.speed.0,
        hits: detection.evidence.hits,
        same_recording: coverage >= MIN_COVERAGE,
        query_start_seconds: detection.start_seconds,
        query_end_seconds: detection.end_seconds,
        found_start_seconds: detection.track_start_seconds,
        found_end_seconds: detection.track_end_seconds,
    }
}

fn same_recording(pairs: &[Pair]) -> impl Iterator<Item = (&str, &str)> {
    pairs
        .iter()
        .filter(|pair| pair.same_recording)
        .map(|pair| (pair.query.as_str(), pair.found.as_str()))
}

/// Searches every indexed file against the whole library, one file per
/// thread, and returns its detections of other files that `keep` accepts.
pub fn self_match(
    library: &Library,
    store: &PeakStore,
    source: Source,
    jobs: usize,
    keep: impl Fn(&Pair) -> bool + Sync,
) -> Result<Vec<Pair>, String> {
    let profile = Profile::CURRENT;
    let (records, problems) = load_records(library, store, &profile, &BTreeSet::new());
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let index = Index::build(&records).map_err(|error| error.to_string())?;
    let ladder = clustering_ladder();
    let durations: BTreeMap<&str, f64> = records
        .iter()
        .map(|record| {
            (
                record.header.source.path.as_str(),
                record.header.duration_seconds,
            )
        })
        .collect();

    let finished = AtomicUsize::new(0);
    let found: Vec<Result<Vec<Pair>, String>> = map_in_order(&records, jobs, |record| {
        let query = &record.header.source.path;
        let detections = match source {
            Source::Audio => {
                let audio = decode(
                    &library.root.join(query),
                    profile.sample_rate,
                    Excerpt::default(),
                )
                .map_err(|error| error.to_string())?;
                search(&index, &audio.samples, &profile, &ladder, 1)
            }
            Source::Peaks => search_peaks(&index, &record.peaks, &profile, &ladder, 1),
        };
        let mut pairs = Vec::new();
        for detection in &detections {
            let found = &index.asset(detection.asset).path;
            if found == query {
                continue;
            }
            let shorter = durations[query.as_str()].min(durations[found.as_str()]);
            let pair = pair(query, found.clone(), shorter, detection);
            if keep(&pair) {
                pairs.push(pair);
            }
        }
        let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("[{count}/{}] {query}", records.len());
        Ok(pairs)
    });
    let mut pairs = Vec::new();
    for result in found {
        pairs.extend(result?);
    }
    Ok(pairs)
}

pub fn print_summary(clusters: &Clusters) {
    println!("{}", clusters.criterion);
    println!("{} clusters with duplicates:", clusters.duplicates.len());
    for members in &clusters.duplicates {
        println!("  {}", members.join("  |  "));
    }
}

/// Prints the clusters with duplicates that one set has and the other does
/// not.
pub fn print_differences(name: &str, clusters: &Clusters, other_name: &str, other: &Clusters) {
    let only = |a: &Clusters, b: &Clusters| -> Vec<String> {
        a.duplicates
            .iter()
            .filter(|members| !b.duplicates.contains(members))
            .map(|members| members.join("  |  "))
            .collect()
    };
    let (ours, theirs) = (only(clusters, other), only(other, clusters));
    println!(
        "against {other_name}: {} clusters the same, {} only in {name}, {} only in {other_name}",
        clusters.duplicates.len() - ours.len(),
        ours.len(),
        theirs.len()
    );
    for members in ours {
        println!("  only in {name}: {members}");
    }
    for members in theirs {
        println!("  only in {other_name}: {members}");
    }
}

/// Follows parent links up to the representative of `node`'s set.
fn root<'a>(parent: &BTreeMap<&'a str, &'a str>, mut node: &'a str) -> &'a str {
    while let Some(&up) = parent.get(node).filter(|&&up| up != node) {
        node = up;
    }
    node
}

/// Transitive closure of links between paths, such as same-recording pairs
/// (union-find over paths).
fn merge<'a>(links: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<Vec<String>> {
    let mut parent: BTreeMap<&str, &str> = BTreeMap::new();
    for (query, found) in links {
        for node in [query, found] {
            parent.entry(node).or_insert(node);
        }
        let (a, b) = (root(&parent, query), root(&parent, found));
        if a != b {
            parent.insert(a.max(b), a.min(b));
        }
    }
    let mut clusters: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for &node in parent.keys() {
        clusters
            .entry(root(&parent, node))
            .or_default()
            .push(node.to_owned());
    }
    clusters.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(query: &str, found: &str, same_recording: bool) -> Pair {
        Pair {
            query: query.to_owned(),
            found: found.to_owned(),
            coverage: 0.0,
            speed: 1.0,
            hits: 0,
            same_recording,
            query_start_seconds: 0.0,
            query_end_seconds: 0.0,
            found_start_seconds: 0.0,
            found_end_seconds: 0.0,
        }
    }

    #[test]
    fn same_recording_pairs_merge_transitively() {
        let pairs = [
            pair("c.mp3", "a.mp3", true),
            pair("d.mp3", "e.mp3", false),
            pair("b.mp3", "c.mp3", true),
            pair("x.mp3", "y.mp3", true),
        ];

        let clusters = merge(same_recording(&pairs));

        assert_eq!(
            clusters,
            [vec!["a.mp3", "b.mp3", "c.mp3"], vec!["x.mp3", "y.mp3"]]
        );
    }

    #[test]
    fn rips_in_another_library_join_the_corpus_clusters() {
        let corpus = Clusters {
            criterion: String::from("corpus"),
            duplicates: vec![vec!["a.mp3".to_owned(), "b.mp3".to_owned()]],
            pairs: Vec::new(),
        };
        let pairs = vec![
            pair("a.mp3", "second-library/x/a.mp3", true),
            pair("second-library/x/a.mp3", "second-library/y/a-rip.mp3", true),
            pair("c.mp3", "second-library/x/c.mp3", true),
            pair("c.mp3", "second-library/x/c-remix.mp3", false),
        ];

        let merged = merged(&corpus, pairs, "/music");

        assert_eq!(
            merged.duplicates,
            [
                vec![
                    "a.mp3",
                    "b.mp3",
                    "second-library/x/a.mp3",
                    "second-library/y/a-rip.mp3"
                ],
                vec!["c.mp3", "second-library/x/c.mp3"],
            ]
        );
        assert!(merged.criterion.starts_with("corpus; "));
        assert_eq!(merged.pairs.len(), 4);
    }

    #[test]
    fn an_asset_without_duplicates_is_its_own_cluster() {
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec!["a.mp3".to_owned(), "b.mp3".to_owned()]],
            pairs: Vec::new(),
        };

        assert_eq!(clusters.cluster_of("b.mp3").len(), 2);
        assert_eq!(
            clusters.cluster_of("z.mp3"),
            BTreeSet::from(["z.mp3".to_owned()])
        );
    }
}

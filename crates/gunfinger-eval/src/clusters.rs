//! Duplicate clusters: library files that are the same recording.
//!
//! Each file is searched against the whole library on a narrow ladder (rips
//! of one recording differ in speed by a percent at most). Two files are the
//! same recording when a single alignment covers nearly all of the shorter
//! one. The criterion is deliberately strict: a remix or VIP that shares some
//! sections with the original must stay separate. Clusters depend on library
//! audio alone, never on what a set scan returned.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::search;
use gunfinger_core::speed::{Rung, SpeedRatio};
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

/// The alignment must cover this share of the shorter file.
const MIN_COVERAGE: f64 = 0.8;
/// Pairs above this coverage but below `MIN_COVERAGE` are reported for review.
const REPORTED_COVERAGE: f64 = 0.2;
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

pub fn find(library: &Library, store: &PeakStore, jobs: usize) -> Result<Clusters, String> {
    let mut pairs = self_match(library, store, jobs, |pair| {
        pair.coverage >= REPORTED_COVERAGE
    })?;
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));

    Ok(Clusters {
        criterion: format!(
            "one alignment at speed {:.2}..{:.2} covers at least {:.0}% of the shorter file",
            SPEEDS[0],
            SPEEDS[SPEEDS.len() - 1],
            MIN_COVERAGE * 100.0
        ),
        duplicates: merge(&pairs),
        pairs,
    })
}

/// Searches every indexed file against the whole library, one file per
/// thread, and returns its detections of other files that `keep` accepts.
pub fn self_match(
    library: &Library,
    store: &PeakStore,
    jobs: usize,
    keep: impl Fn(&Pair) -> bool + Sync,
) -> Result<Vec<Pair>, String> {
    let profile = Profile::CURRENT;
    let (records, problems) = load_records(library, store, &profile, &BTreeSet::new());
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let index = Index::build(&records).map_err(|error| error.to_string())?;
    let ladder: Vec<Rung> = SPEEDS
        .into_iter()
        .map(|speed| Rung::Turntable(SpeedRatio(speed)))
        .collect();
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
        let audio = decode(
            &library.root.join(query),
            profile.sample_rate,
            Excerpt::default(),
        )
        .map_err(|error| error.to_string())?;
        let mut pairs = Vec::new();
        for detection in search(&index, &audio.samples, &profile, &ladder, 1) {
            let found = &index.asset(detection.asset).path;
            if found == query {
                continue;
            }
            let shorter = durations[query.as_str()].min(durations[found.as_str()]);
            let coverage = (detection.end_seconds - detection.start_seconds) / shorter;
            let pair = Pair {
                query: query.clone(),
                found: found.clone(),
                coverage,
                speed: detection.speed.0,
                hits: detection.evidence.hits,
                same_recording: coverage >= MIN_COVERAGE,
                query_start_seconds: detection.start_seconds,
                query_end_seconds: detection.end_seconds,
                found_start_seconds: detection.track_start_seconds,
                found_end_seconds: detection.track_end_seconds,
            };
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

/// Follows parent links up to the representative of `node`'s set.
fn root<'a>(parent: &BTreeMap<&'a str, &'a str>, mut node: &'a str) -> &'a str {
    while let Some(&up) = parent.get(node).filter(|&&up| up != node) {
        node = up;
    }
    node
}

/// Transitive closure of the same-recording pairs (union-find over paths).
fn merge(pairs: &[Pair]) -> Vec<Vec<String>> {
    let mut parent: BTreeMap<&str, &str> = BTreeMap::new();
    for pair in pairs.iter().filter(|pair| pair.same_recording) {
        for node in [pair.query.as_str(), pair.found.as_str()] {
            parent.entry(node).or_insert(node);
        }
        let (a, b) = (root(&parent, &pair.query), root(&parent, &pair.found));
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

        let clusters = merge(&pairs);

        assert_eq!(
            clusters,
            [vec!["a.mp3", "b.mp3", "c.mp3"], vec!["x.mp3", "y.mp3"]]
        );
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

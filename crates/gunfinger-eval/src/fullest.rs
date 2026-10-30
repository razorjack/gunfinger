//! Which records hold the postings of the fullest posting lists: the lists
//! `--skip-fullest` sets aside when looking for candidates. Records built
//! on common material (famous breaks, shared drums) should hold more of
//! them than their share of the index.
//!
//! The index is the one the scans search: the library's records and the
//! padding (with `--other-peaks-dir`, the other library less the copies
//! corpus files stand for).
//!
//! `Ranking` sets the same share aside ranked by distinct holders instead
//! of postings, a variant for measuring the filter (session 50).
//! `setting_aside_like` searches an index with the lists another index,
//! saved in a file, sets aside (session 59).

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use gunfinger_core::hash::{HASH_BITS, PairHash};
use gunfinger_core::index::{AssetId, Index, saved};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::SKIPPED_SHARE;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::files::read_lines;
use crate::padding::{Padding, original_path};

/// What the lists set aside are ranked by, instead of their postings.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Ranking {
    /// The number of distinct files holding a hash: a file that repeats
    /// it counts once, each exact copy as a file of its own.
    Files,
    /// The number of distinct recordings: a file's duplicate cluster, a
    /// file in no cluster itself, an exact copy its original's.
    Clusters,
}

impl Ranking {
    /// The index as built, setting aside the default share of lists
    /// (`SKIPPED_SHARE`) held by the most files or recordings.
    pub fn index(self, index: Index, clusters: &Clusters) -> Index {
        let holders: Vec<u32> = match self {
            Ranking::Files => (0..index.assets().len() as u32).collect(),
            Ranking::Clusters => recordings(&index, clusters),
        };
        index.skipping_most_held(SKIPPED_SHARE, |asset| holders[asset.0 as usize])
    }

    pub fn name(self) -> &'static str {
        match self {
            Ranking::Files => "files",
            Ranking::Clusters => "clusters",
        }
    }
}

/// A recording: a duplicate cluster, or a file in none.
#[derive(PartialEq, Eq, Hash)]
enum Recording<'a> {
    Cluster(usize),
    File(&'a str),
}

/// A number for each asset's recording: equal for the members of a
/// cluster and for exact copies of one file.
fn recordings(index: &Index, clusters: &Clusters) -> Vec<u32> {
    let mut cluster_of: HashMap<&str, usize> = HashMap::new();
    for (cluster, members) in clusters.duplicates.iter().enumerate() {
        for member in members {
            cluster_of.insert(member, cluster);
        }
    }
    let mut numbers: HashMap<Recording, u32> = HashMap::new();
    index
        .assets()
        .iter()
        .map(|asset| {
            let original = original_path(&asset.path);
            let recording = cluster_of
                .get(original)
                .map_or(Recording::File(original), |&cluster| {
                    Recording::Cluster(cluster)
                });
            let next = numbers.len() as u32;
            *numbers.entry(recording).or_insert(next)
        })
        .collect()
}

/// How the lists an index sets aside differ from those of another index,
/// saved in a file, with the same files left out: "here" is the index
/// searched, "there" the saved one. Session 52's prune moved detections
/// of files it did not touch (session 57); this traces whether the lists
/// set aside moved them.
#[derive(Clone, Serialize, Deserialize)]
pub struct SetAsideLike {
    /// The saved index, only read.
    pub file: String,
    /// The files it holds that the index searched does not.
    pub only_there: Vec<String>,
    /// Whether, with those files left out as well, its postings are the
    /// searched index's, posting for posting (`Index::without_assets`).
    pub same_postings_without_them: bool,
    pub lists_here: usize,
    pub lists_there: usize,
    /// The shortest list each sets aside.
    pub shortest_here: usize,
    pub shortest_there: usize,
    pub set_aside_here_only: Vec<MovedList>,
    pub set_aside_there_only: Vec<MovedList>,
}

/// A list set aside by one of the two indexes only, with its postings in
/// each, and those there of the files only there.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MovedList {
    pub hash: u32,
    pub here: usize,
    pub there: usize,
    pub there_of_files_only_there: usize,
}

/// `index` (as built, less `left_out`), searched with the lists set aside
/// that the index saved at `saved` sets aside with `left_out` left out of
/// it too, by the same `share`: its own postings, the other index's cut.
/// The file is only read; both indexes are in memory at once.
pub fn setting_aside_like(
    index: Index,
    saved: &Path,
    left_out: &BTreeSet<String>,
    share: f64,
) -> Result<(Index, SetAsideLike), String> {
    let unusable = |error: saved::Unusable| {
        format!(
            "cannot load {} for --set-aside-like: {error}",
            saved.display()
        )
    };
    let here = index.skipping_fullest(share);
    let here_hashes = set_aside_hashes(&here);
    let there = saved::load(saved, &saved::provenance(saved).map_err(unusable)?)
        .map_err(unusable)?
        .without_assets(left_out);
    let here_paths: BTreeSet<&str> = here
        .assets()
        .iter()
        .map(|asset| asset.path.as_str())
        .collect();
    let only_there: BTreeSet<String> = there
        .assets()
        .iter()
        .filter(|asset| !here_paths.contains(asset.path.as_str()))
        .map(|asset| asset.path.clone())
        .collect();
    let only_there_ids: BTreeSet<AssetId> = (0..there.assets().len() as u32)
        .map(AssetId)
        .filter(|&asset| only_there.contains(&there.asset(asset).path))
        .collect();
    let there = there.skipping_fullest(share);
    let there_hashes = set_aside_hashes(&there);
    let moved = |hashes: &[u32], elsewhere: &[u32]| -> Vec<MovedList> {
        hashes
            .iter()
            .filter(|hash| elsewhere.binary_search(hash).is_err())
            .map(|&hash| {
                let there_list = there.postings(PairHash(hash));
                MovedList {
                    hash,
                    here: here.postings(PairHash(hash)).len(),
                    there: there_list.len(),
                    there_of_files_only_there: there_list
                        .iter()
                        .filter(|posting| only_there_ids.contains(&posting.asset()))
                        .count(),
                }
            })
            .collect()
    };
    let shortest = |index: &Index, hashes: &[u32]| {
        hashes
            .iter()
            .map(|&hash| index.postings(PairHash(hash)).len())
            .min()
            .unwrap_or(0)
    };
    let mut like = SetAsideLike {
        file: saved.display().to_string(),
        only_there: only_there.iter().cloned().collect(),
        same_postings_without_them: false,
        lists_here: here_hashes.len(),
        lists_there: there_hashes.len(),
        shortest_here: shortest(&here, &here_hashes),
        shortest_there: shortest(&there, &there_hashes),
        set_aside_here_only: moved(&here_hashes, &there_hashes),
        set_aside_there_only: moved(&there_hashes, &here_hashes),
    };
    let there = there.without_assets(&only_there);
    like.same_postings_without_them =
        there.assets() == here.assets() && there.posting_lists().eq(here.posting_lists());
    drop(there);
    let hashes: Vec<PairHash> = there_hashes.into_iter().map(PairHash).collect();
    Ok((here.skipping_hashes(&hashes), like))
}

/// One line on stderr: how the lists set aside differ from the saved
/// index's.
pub fn print_like(like: &SetAsideLike) {
    eprintln!(
        "set aside like {}: {} lists there, {} here; {} only there, {} only here; {} files only there; same postings without them: {}",
        like.file,
        like.lists_there,
        like.lists_here,
        like.set_aside_there_only.len(),
        like.set_aside_here_only.len(),
        like.only_there.len(),
        like.same_postings_without_them
    );
}

/// The hashes whose lists `index` leaves out of the search for
/// candidates, in hash order.
pub fn set_aside_hashes(index: &Index) -> Vec<u32> {
    (0..1u32 << HASH_BITS)
        .filter(|&hash| {
            !index.postings(PairHash(hash)).is_empty()
                && index.scanned_postings(PairHash(hash)).is_empty()
        })
        .collect()
}

#[derive(Serialize, Deserialize)]
pub struct FullestReport {
    /// The share of non-empty lists set aside, as for `--skip-fullest`.
    pub share: f64,
    pub lists: usize,
    pub skipped_lists: usize,
    /// The shortest list set aside.
    pub shortest_skipped: usize,
    pub postings: usize,
    pub skipped_postings: usize,
    /// Every asset, the most skipped share first.
    pub assets: Vec<AssetPostings>,
    /// With `--shared-by`: the hashes each pair of files holds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pairs: Vec<SharedHashes>,
}

/// The distinct hashes two files both hold, and how many of them are in
/// lists set aside.
#[derive(Serialize, Deserialize)]
pub struct SharedHashes {
    pub a: String,
    pub b: String,
    pub shared: usize,
    pub set_aside: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AssetPostings {
    pub path: String,
    pub duration_seconds: f64,
    pub postings: usize,
    /// Postings in the lists set aside.
    pub skipped: usize,
}

impl AssetPostings {
    fn skipped_share(&self) -> f64 {
        self.skipped as f64 / self.postings.max(1) as f64
    }
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    padding: &Padding,
    share: f64,
    pairs: &[(String, String)],
) -> Result<FullestReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let index = padding.index(&records, &profile, &BTreeSet::new())?;
    drop(records);
    let index = index.skipping_fullest(share);
    let pairs = pairs
        .iter()
        .map(|(a, b)| shared_hashes(&index, a, b))
        .collect::<Result<Vec<SharedHashes>, String>>()?;
    Ok(FullestReport {
        pairs,
        ..census(&index, share)
    })
}

/// Pairs of index paths, two per line separated by a tab.
pub fn read_pairs(file: &Path) -> Result<Vec<(String, String)>, String> {
    read_lines(file)?
        .iter()
        .map(|line| {
            line.split_once('\t')
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .ok_or_else(|| {
                    format!(
                        "{}: not two paths separated by a tab: {line}",
                        file.display()
                    )
                })
        })
        .collect()
}

fn shared_hashes(index: &Index, a: &str, b: &str) -> Result<SharedHashes, String> {
    let asset = |path: &str| {
        index
            .assets()
            .iter()
            .position(|asset| asset.path == path)
            .map(|position| AssetId(position as u32))
            .ok_or_else(|| format!("{path} is not in the index"))
    };
    let (first, second) = (asset(a)?, asset(b)?);
    let mut pair = SharedHashes {
        a: a.to_owned(),
        b: b.to_owned(),
        shared: 0,
        set_aside: 0,
    };
    for hash in (0..1u32 << HASH_BITS).map(PairHash) {
        if index.postings_for_asset(hash, first).is_empty()
            || index.postings_for_asset(hash, second).is_empty()
        {
            continue;
        }
        pair.shared += 1;
        pair.set_aside += usize::from(index.scanned_postings(hash).is_empty());
    }
    Ok(pair)
}

/// Counts each asset's postings, and those in lists the index leaves out
/// of the search for candidates.
pub fn census(index: &Index, share: f64) -> FullestReport {
    let mut assets: Vec<AssetPostings> = index
        .assets()
        .iter()
        .map(|asset| AssetPostings {
            path: asset.path.clone(),
            duration_seconds: asset.duration_seconds,
            postings: 0,
            skipped: 0,
        })
        .collect();
    let mut report = FullestReport {
        share,
        lists: 0,
        skipped_lists: 0,
        shortest_skipped: usize::MAX,
        postings: index.posting_count(),
        skipped_postings: 0,
        assets: Vec::new(),
        pairs: Vec::new(),
    };
    for hash in 0..1u32 << HASH_BITS {
        let list = index.postings(PairHash(hash));
        if list.is_empty() {
            continue;
        }
        report.lists += 1;
        let skipped = index.scanned_postings(PairHash(hash)).is_empty();
        if skipped {
            report.skipped_lists += 1;
            report.skipped_postings += list.len();
            report.shortest_skipped = report.shortest_skipped.min(list.len());
        }
        for posting in list {
            let asset = &mut assets[posting.asset().0 as usize];
            asset.postings += 1;
            asset.skipped += usize::from(skipped);
        }
    }
    assets.sort_by(|a, b| b.skipped_share().total_cmp(&a.skipped_share()));
    report.assets = assets;
    report
}

pub fn print_summary(report: &FullestReport) {
    println!(
        "{} of {} lists set aside (the fullest {}%, {} postings or more): {} of {} postings ({:.1}%)",
        report.skipped_lists,
        report.lists,
        report.share * 100.0,
        report.shortest_skipped,
        report.skipped_postings,
        report.postings,
        100.0 * report.skipped_postings as f64 / report.postings.max(1) as f64
    );
    let mut by_count: Vec<usize> = report.assets.iter().map(|asset| asset.skipped).collect();
    by_count.sort_unstable_by(|a, b| b.cmp(a));
    for percent in [1, 10] {
        let top = (by_count.len() * percent).div_ceil(100);
        let held: usize = by_count[..top].iter().sum();
        println!(
            "the {top} assets with most such postings ({percent}%) hold {:.1}% of them",
            100.0 * held as f64 / report.skipped_postings.max(1) as f64
        );
    }
    println!("the highest shares of postings set aside:");
    for asset in report.assets.iter().take(10) {
        println!(
            "  {:5.1}%  {:>7} of {:>7}  {}",
            100.0 * asset.skipped_share(),
            asset.skipped,
            asset.postings,
            asset.path
        );
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::time::Duration;

    use gunfinger_core::indexing::TrackLength;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::peaks::Peak;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    fn record(path: &str, bins: &[f32]) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_string(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 10.0,
            },
            peaks: bins
                .iter()
                .enumerate()
                .map(|(frame, &bin)| Peak {
                    frame: 10.0 * frame as f64,
                    bin,
                    magnitude: 10.0,
                })
                .collect(),
        }
    }

    #[test]
    fn a_cluster_and_the_exact_copies_of_a_file_are_one_recording() {
        let records = [
            record("a.mp3", &[100.0]),
            record("b.mp3", &[100.0]),
            record("c.mp3", &[100.0]),
            record("padding-copy/1/c.mp3", &[100.0]),
            record("padding-copy/1/a.mp3", &[100.0]),
        ];
        let index = Index::build(&records).unwrap();
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("b.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        assert_eq!(recordings(&index, &clusters), [0, 0, 1, 1, 0]);
    }

    #[test]
    fn ranking_by_recordings_sets_aside_the_list_most_recordings_hold() {
        // a.mp3 and its two copies share one hash; b.mp3 and c.mp3, two
        // recordings, share another. With half the lists set aside, by
        // files it is the first, by recordings the second.
        let common = [100.0, 120.0];
        let other = [300.0, 340.0];
        let records = [
            record("a.mp3", &common),
            record("b.mp3", &other),
            record("c.mp3", &other),
            record("padding-copy/1/a.mp3", &common),
            record("padding-copy/2/a.mp3", &common),
        ];
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: Vec::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let index = Index::build(&records).unwrap();
        let list_of = |length: usize| {
            (0..1u32 << HASH_BITS)
                .find(|&hash| index.postings(PairHash(hash)).len() == length)
                .unwrap()
        };
        let (common_hash, other_hash) = (list_of(3), list_of(2));
        let numbers = recordings(&index, &clusters);

        let by_files = Index::build(&records)
            .unwrap()
            .skipping_most_held(0.5, |asset| asset.0);
        let by_recordings = Index::build(&records)
            .unwrap()
            .skipping_most_held(0.5, |asset| numbers[asset.0 as usize]);

        assert_eq!(set_aside_hashes(&by_files), [common_hash]);
        assert_eq!(set_aside_hashes(&by_recordings), [other_hash]);
    }

    #[test]
    fn a_saved_index_with_one_file_more_lends_its_lists_set_aside() {
        // Before "gone.mp3" was removed, the list c.mp3, d.mp3 and it share
        // was the fullest of two and set aside; without it both lists hold
        // two postings, a tie at the cut, and neither is set aside.
        let ab = [100.0, 120.0];
        let cd = [300.0, 340.0];
        let before = [
            record("a.mp3", &ab),
            record("b.mp3", &ab),
            record("c.mp3", &cd),
            record("gone.mp3", &cd),
            record("d.mp3", &cd),
        ];
        let dir = std::env::temp_dir().join(format!("gunfinger-like-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("before.index");
        let provenance = saved::Provenance {
            profile: Profile::CURRENT.id(),
            hash_design: String::from("design"),
            track_length: TrackLength {
                min: Duration::ZERO,
                max: Duration::from_secs(600),
            },
            sources: before
                .iter()
                .map(|record| record.header.source.clone())
                .collect(),
        };
        saved::save(&file, &provenance, &Index::build(&before).unwrap()).unwrap();
        let after: Vec<PeakRecord> = before
            .iter()
            .filter(|record| record.header.source.path != "gone.mp3")
            .cloned()
            .collect();
        let index = Index::build(&after).unwrap();
        let cd_hash = (0..1u32 << HASH_BITS)
            .find(|&hash| {
                index.postings(PairHash(hash)).len() == 2
                    && index.postings(PairHash(hash))[0].asset() == AssetId(2)
            })
            .unwrap();
        assert!(set_aside_hashes(&Index::build(&after).unwrap().skipping_fullest(0.5)).is_empty());

        let (searched, like) = setting_aside_like(index, &file, &BTreeSet::new(), 0.5).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(like.only_there, ["gone.mp3"]);
        assert!(like.same_postings_without_them);
        assert_eq!((like.lists_here, like.lists_there), (0, 1));
        assert!(like.set_aside_here_only.is_empty());
        assert_eq!(
            like.set_aside_there_only,
            [MovedList {
                hash: cd_hash,
                here: 2,
                there: 3,
                there_of_files_only_there: 1,
            }]
        );
        assert_eq!(set_aside_hashes(&searched), [cd_hash]);
        assert_eq!(searched.postings(PairHash(cd_hash)).len(), 2);
    }

    #[test]
    fn a_pair_counts_the_hashes_both_files_hold_and_those_set_aside() {
        // a.mp3 and b.mp3 share two hashes, one of which four more files
        // hold; c.mp3 shares none with a.mp3.
        let common = [100.0, 120.0];
        let mut records = vec![
            record("a.mp3", &[100.0, 120.0, 0.0, 300.0, 340.0]),
            record("b.mp3", &[300.0, 340.0, 0.0, 100.0, 120.0]),
            record("c.mp3", &[200.0, 230.0]),
        ];
        records.extend((0..4).map(|copy| record(&format!("common-{copy}.mp3"), &common)));
        let index = Index::build(&records).unwrap();
        let lists = index
            .posting_lists()
            .filter(|list| !list.is_empty())
            .count();
        let index = index.skipping_fullest(1.0 / lists as f64);
        assert_eq!(set_aside_hashes(&index).len(), 1);

        let pair = shared_hashes(&index, "a.mp3", "b.mp3").unwrap();
        let none = shared_hashes(&index, "a.mp3", "c.mp3").unwrap();

        assert_eq!((pair.shared, pair.set_aside), (2, 1));
        assert_eq!((none.shared, none.set_aside), (0, 0));
        assert!(shared_hashes(&index, "a.mp3", "missing.mp3").is_err());
    }

    #[test]
    fn postings_of_the_lists_set_aside_are_counted_per_asset() {
        // Three peaks give three pair hashes. Four records share theirs, a
        // fifth repeats one of them, so that list (5 postings) is the
        // fullest; the rare record's lists hold one posting each.
        let common = [100.0, 120.0, 90.0];
        let mut records: Vec<PeakRecord> = (0..4)
            .map(|copy| record(&format!("common-{copy}.mp3"), &common))
            .collect();
        records.push(record("half.mp3", &[100.0, 120.0]));
        records.push(record("rare.mp3", &[300.0, 340.0, 290.0]));
        let index = Index::build(&records).unwrap();
        let share = 1.0 / 6.0;

        let report = census(&index.skipping_fullest(share), share);

        assert_eq!(report.lists, 6);
        assert_eq!(report.skipped_lists, 1);
        assert_eq!(report.shortest_skipped, 5);
        assert_eq!(report.skipped_postings, 5);
        assert_eq!(report.assets[0].path, "half.mp3");
        assert_eq!(
            (report.assets[0].skipped, report.assets[0].postings),
            (1, 1)
        );
        let skipped = |path: &str| {
            let asset = report
                .assets
                .iter()
                .find(|asset| asset.path == path)
                .unwrap();
            (asset.skipped, asset.postings)
        };
        assert_eq!(skipped("common-0.mp3"), (1, 3));
        assert_eq!(skipped("rare.mp3"), (0, 3));
    }
}

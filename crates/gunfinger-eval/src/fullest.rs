//! Which records hold the postings of the fullest posting lists: the lists
//! `--skip-fullest` sets aside when looking for candidates. Records built
//! on common material (famous breaks, shared drums) should hold more of
//! them than their share of the index.
//!
//! The index is the one the scans search: the library's records and the
//! padding (with `--other-peaks-dir`, the other library less the copies
//! corpus files stand for).

use std::collections::BTreeSet;

use gunfinger_core::hash::{HASH_BITS, PairHash};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::padding::Padding;

#[derive(Serialize)]
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
}

#[derive(Serialize)]
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
) -> Result<FullestReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let index = padding.index(&records, &profile, &BTreeSet::new())?;
    drop(records);
    Ok(census(&index.skipping_fullest(share), share))
}

/// Counts each asset's postings, and those in lists the index leaves out
/// of the search for candidates.
fn census(index: &Index, share: f64) -> FullestReport {
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

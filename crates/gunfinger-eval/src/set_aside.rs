//! A set-aside threshold by posting count (brief 88): the lists the first
//! pass leaves out are those longer than one number, kept beside the index,
//! instead of the fullest share of each index a run searches (ADR 0014).
//! Today's cut is a share, so that every change of the library moves lists
//! across it (experiment 0242). A number moves a list only when that list
//! itself crosses it. A variant under measurement: nothing here is the
//! default.
//!
//! `derive` reads the number off `Index::skipping_fullest`, so that it
//! keeps exactly the lists the share keeps at that index; `skipping_above`
//! applies it; `FixedCut::after_growth` is the rule for keeping or
//! re-deriving the number as the index grows.

use std::fs;
use std::path::Path;

use gunfinger_core::hash::{HASH_BITS, PairHash};
use gunfinger_core::index::Index;
use serde::{Deserialize, Serialize};

use crate::digest::Sha256;

/// The growth of the whole index's postings past which the number is
/// derived again: stated in advance in brief 88.
pub const GROWTH_LIMIT: f64 = 0.10;

/// The number and what it was derived from, the file kept beside a saved
/// index.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedCut {
    /// Lists with more postings than this are set aside.
    pub postings_per_list: u32,
    /// The share of the fullest lists it replaces.
    pub share: f64,
    /// Past this growth of the postings the number is derived again.
    pub growth_limit: f64,
    /// The whole index's postings and assets when derived.
    pub postings: u64,
    pub assets: u32,
}

impl FixedCut {
    /// The cut the rule gives at `index` (as built), having been `self`:
    /// `self` until the postings have grown by more than the limit, then
    /// derived again at `index`. The bool says it was derived again.
    pub fn after_growth(self, index: Index) -> (FixedCut, bool) {
        if index.posting_count() as f64 <= self.postings as f64 * (1.0 + self.growth_limit) {
            (self, false)
        } else {
            (derive(index, self.share, self.growth_limit), true)
        }
    }

    pub fn read(path: &Path) -> Result<FixedCut, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        serde_json::from_str(&text)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))
    }
}

/// The cut that keeps exactly the lists the `share` keeps at `index`,
/// read off `Index::skipping_fullest`.
pub fn derive(index: Index, share: f64, growth_limit: f64) -> FixedCut {
    let postings = index.posting_count() as u64;
    let assets = index.assets().len() as u32;
    let index = index.skipping_fullest(share);
    let longest_kept = hashes()
        .map(|hash| index.scanned_postings(hash).len() as u32)
        .max()
        .unwrap_or(0);
    FixedCut {
        postings_per_list: longest_kept,
        share,
        growth_limit,
        postings,
        assets,
    }
}

/// `index` searched with the lists longer than `postings_per_list` set
/// aside, whatever else it holds.
pub fn skipping_above(index: Index, postings_per_list: u32) -> Index {
    let over: Vec<PairHash> = hashes()
        .filter(|&hash| index.postings(hash).len() as u32 > postings_per_list)
        .collect();
    index.skipping_hashes(&over)
}

fn hashes() -> impl Iterator<Item = PairHash> {
    (0..1u32 << HASH_BITS).map(PairHash)
}

/// A digest of the files an index names and of every list's length: two
/// indexes built from the same records agree, and a change of a record
/// changes it unless that record's postings were all replaced by others at
/// the same hashes. For checking that a replayed index is the one it
/// stands for (brief 88).
pub fn lists_digest(index: &Index) -> String {
    let mut digest = Sha256::default();
    for asset in index.assets() {
        digest.update(asset.path.as_bytes());
        digest.update(&[0]);
    }
    for list in index.posting_lists() {
        digest.update(&(list.len() as u32).to_le_bytes());
    }
    digest.finish()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::peaks::Peak;
    use gunfinger_core::profile::Profile;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;
    use crate::fullest::set_aside_hashes;

    fn record(path: &str, bins: &[f32]) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 1,
                        nanos: 0,
                    },
                },
                duration_seconds: 10.0,
            },
            peaks: bins
                .iter()
                .enumerate()
                .map(|(frame, &bin)| Peak {
                    frame: 8.0 * frame as f64,
                    bin,
                    magnitude: 10.0,
                })
                .collect(),
        }
    }

    /// Three records with lists of unequal length: the first file's bins
    /// twice, so its pairs repeat, and two files holding some of its pairs.
    fn records() -> Vec<PeakRecord> {
        let many: Vec<f32> = (0..12).map(|i| 100.0 + (i % 5) as f32 * 11.0).collect();
        vec![
            record("a.mp3", &many),
            record("b.mp3", &many),
            record("c.mp3", &many[..4]),
        ]
    }

    fn index() -> Index {
        Index::build(&records()).unwrap()
    }

    fn the_share(index: Index, share: f64) -> Vec<u32> {
        set_aside_hashes(&index.skipping_fullest(share))
    }

    #[test]
    fn the_derived_number_keeps_exactly_the_lists_the_share_keeps() {
        // A share that sets aside some lists and keeps others at this index.
        let lists = index()
            .posting_lists()
            .filter(|list| !list.is_empty())
            .count();
        let share = 3.0 / lists as f64;
        let cut = derive(index(), share, GROWTH_LIMIT);
        let by_number = set_aside_hashes(&skipping_above(index(), cut.postings_per_list));
        let by_share = the_share(index(), share);
        assert_eq!(by_number, by_share);
        assert!(!by_share.is_empty());
    }

    #[test]
    fn the_number_is_written_and_read_back_whole() {
        let dir = std::env::temp_dir().join(format!("gunfinger-cut-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("a.set-aside.json");
        let cut = derive(index(), 0.5, GROWTH_LIMIT);
        fs::write(&path, serde_json::to_string(&cut).unwrap()).unwrap();
        let read = FixedCut::read(&path).unwrap();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(read, cut);
    }

    #[test]
    fn leaving_files_out_can_only_take_lists_out_of_what_a_number_sets_aside() {
        // Leaving a file out only shortens lists, so a list can leave the
        // set aside by its own length and none can enter it, unlike a share,
        // whose cut can fall with the lists around it.
        let cut = derive(index(), 0.5, GROWTH_LIMIT);
        let before = set_aside_hashes(&skipping_above(index(), cut.postings_per_list));
        let left_out = BTreeSet::from([String::from("c.mp3")]);
        let smaller = || index().without_assets(&left_out);
        let after = set_aside_hashes(&skipping_above(smaller(), cut.postings_per_list));
        assert!(!before.is_empty());
        assert!(after.iter().all(|hash| before.contains(hash)));
        let shorter = smaller();
        for hash in before.iter().filter(|hash| !after.contains(hash)) {
            assert!(shorter.postings(PairHash(*hash)).len() as u32 <= cut.postings_per_list);
        }
    }

    #[test]
    fn the_number_is_kept_up_to_the_growth_limit_and_derived_again_past_it() {
        let cut = derive(index(), 0.5, GROWTH_LIMIT);
        let derived_at = |postings: f64| FixedCut {
            postings: (index().posting_count() as f64 / postings).ceil() as u64,
            ..cut
        };
        // Grown by 5% and 9%: within the limit. By 20%: past it.
        assert!(!derived_at(1.05).after_growth(index()).1);
        assert!(!derived_at(1.09).after_growth(index()).1);
        let (grown, derived) = derived_at(1.20).after_growth(index());
        assert!(derived);
        assert_eq!(grown.postings_per_list, cut.postings_per_list);
        assert_eq!(grown.postings, index().posting_count() as u64);
    }

    #[test]
    fn lists_digests_tell_indexes_apart() {
        let same = lists_digest(&index());
        assert_eq!(same, lists_digest(&index()));
        let without_c = index().without_assets(&BTreeSet::from([String::from("c.mp3")]));
        assert_ne!(same, lists_digest(&without_c));
    }
}

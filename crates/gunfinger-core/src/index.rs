//! The inverted index: for every hash, where in the library it occurs.
//!
//! Postings are grouped by hash and addressed through an offsets table: the
//! postings of hash `h` are `postings[offsets[h]..offsets[h + 1]]`. This is the
//! layout an on-disk index would use, so its size measurements are real.

use crate::hash::{HASH_BITS, PairHash, Point, for_each_pair};
use crate::store::PeakRecord;

/// Position of an asset in the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(pub u32);

/// One occurrence of a hash: the asset and the frame of the anchor peak,
/// packed into 32 bits.
///
/// 17 frame bits hold 35 minutes at 16 ms per frame, beyond the 20-minute
/// track limit. The remaining 15 bits address 32,768 assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Posting(u32);

const FRAME_BITS: u32 = 17;
pub const MAX_FRAMES: u32 = 1 << FRAME_BITS;
pub const MAX_ASSETS: usize = 1 << (32 - FRAME_BITS);

impl Posting {
    fn new(asset: AssetId, frame: u32) -> Posting {
        Posting((asset.0 << FRAME_BITS) | frame)
    }

    pub fn asset(self) -> AssetId {
        AssetId(self.0 >> FRAME_BITS)
    }

    pub fn frame(self) -> u32 {
        self.0 & (MAX_FRAMES - 1)
    }
}

/// What the index knows about an asset besides its postings.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexedAsset {
    /// Path relative to the library root.
    pub path: String,
    pub duration_seconds: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("the index holds at most {MAX_ASSETS} assets, but {count} were given")]
    TooManyAssets { count: usize },
    #[error(
        "{path} is longer than the index can address ({MAX_FRAMES} frames); lower --max-track-minutes"
    )]
    TooLong { path: String },
}

pub struct Index {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
    postings: Vec<Posting>,
}

impl Index {
    /// Builds the index from peak records; asset `i` is `records[i]`.
    ///
    /// Two passes over the hashes: the first counts postings per hash to lay
    /// out the offsets table, the second fills each hash's slots. Postings of
    /// a hash end up ordered by asset and then by frame.
    pub fn build(records: &[PeakRecord]) -> Result<Index, IndexError> {
        if records.len() > MAX_ASSETS {
            return Err(IndexError::TooManyAssets {
                count: records.len(),
            });
        }
        let points: Vec<Vec<Point>> = records
            .iter()
            .map(|record| record.peaks.iter().map(Point::from).collect())
            .collect();
        for (record, points) in records.iter().zip(&points) {
            if points
                .last()
                .is_some_and(|last| last.frame.round() >= f64::from(MAX_FRAMES))
            {
                return Err(IndexError::TooLong {
                    path: record.header.source.path.clone(),
                });
            }
        }

        let mut offsets = vec![0_u32; (1 << HASH_BITS) + 1];
        for points in &points {
            for_each_pair(points, |hash, _| offsets[hash.0 as usize + 1] += 1);
        }
        for hash in 1..offsets.len() {
            offsets[hash] += offsets[hash - 1];
        }

        let mut next_slot = offsets.clone();
        let mut postings = vec![Posting(0); offsets[offsets.len() - 1] as usize];
        for (asset, points) in (0..).map(AssetId).zip(&points) {
            for_each_pair(points, |hash, anchor| {
                let slot = &mut next_slot[hash.0 as usize];
                postings[*slot as usize] = Posting::new(asset, points[anchor].frame.round() as u32);
                *slot += 1;
            });
        }

        let assets = records
            .iter()
            .map(|record| IndexedAsset {
                path: record.header.source.path.clone(),
                duration_seconds: record.header.duration_seconds,
            })
            .collect();
        Ok(Index {
            assets,
            offsets,
            postings,
        })
    }

    pub fn postings(&self, hash: PairHash) -> &[Posting] {
        let start = self.offsets[hash.0 as usize] as usize;
        let end = self.offsets[hash.0 as usize + 1] as usize;
        &self.postings[start..end]
    }

    pub fn asset(&self, id: AssetId) -> &IndexedAsset {
        &self.assets[id.0 as usize]
    }

    pub fn assets(&self) -> &[IndexedAsset] {
        &self.assets
    }

    pub fn posting_count(&self) -> usize {
        self.postings.len()
    }

    /// The posting list of every hash, in hash order.
    pub fn posting_lists(&self) -> impl Iterator<Item = &[Posting]> {
        self.offsets
            .windows(2)
            .map(|pair| &self.postings[pair[0] as usize..pair[1] as usize])
    }

    /// Bytes the index occupies: the offsets table and the postings.
    pub fn size_bytes(&self) -> usize {
        size_of_val(self.offsets.as_slice()) + size_of_val(self.postings.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::{Asset, Timestamp};
    use crate::peaks::Peak;
    use crate::store::RecordHeader;

    fn record(path: &str, peaks: &[(f64, f32)]) -> PeakRecord {
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
                duration_seconds: 1.0,
            },
            peaks: peaks
                .iter()
                .map(|&(frame, bin)| Peak {
                    frame,
                    bin,
                    magnitude: 0.0,
                })
                .collect(),
        }
    }

    fn hashes_of(peaks: &[(f64, f32)]) -> Vec<PairHash> {
        let points: Vec<Point> = peaks
            .iter()
            .map(|&(frame, bin)| Point { frame, bin })
            .collect();
        let mut hashes = Vec::new();
        for_each_pair(&points, |hash, _| hashes.push(hash));
        hashes
    }

    #[test]
    fn postings_pack_asset_and_frame() {
        let posting = Posting::new(AssetId(MAX_ASSETS as u32 - 1), MAX_FRAMES - 1);

        assert_eq!(posting.asset(), AssetId(MAX_ASSETS as u32 - 1));
        assert_eq!(posting.frame(), MAX_FRAMES - 1);
    }

    #[test]
    fn every_pair_is_found_under_its_hash() {
        let first = [(10.0, 100.0), (20.0, 110.0)];
        let second = [(500.2, 100.0), (509.8, 110.0), (900.0, 300.0)];
        let index = Index::build(&[record("a.mp3", &first), record("b.mp3", &second)]).unwrap();

        let shared = hashes_of(&first)[0];
        let found: Vec<(AssetId, u32)> = index
            .postings(shared)
            .iter()
            .map(|posting| (posting.asset(), posting.frame()))
            .collect();

        assert_eq!(found, [(AssetId(0), 10), (AssetId(1), 500)]);
        assert_eq!(index.posting_count(), 1 + hashes_of(&second).len());
        assert_eq!(index.asset(AssetId(1)).path, "b.mp3");
    }

    #[test]
    fn a_hash_without_postings_has_an_empty_list() {
        let index = Index::build(&[record("a.mp3", &[(0.0, 100.0), (5.0, 101.0)])]).unwrap();

        assert!(index.postings(PairHash(0)).is_empty());
        assert_eq!(
            index.posting_lists().map(<[Posting]>::len).sum::<usize>(),
            1
        );
    }
}

//! The inverted index: for every hash, where in the library it occurs.
//!
//! Postings are grouped by hash and addressed through an offsets table: the
//! postings of hash `h` are `postings[offsets[h]..offsets[h + 1]]`. This is the
//! layout an on-disk index would use, so its size measurements are real.

use std::time::Duration;

use crate::hash::{HASH_BITS, PairHash, Point, for_each_pair};
use crate::profile::Profile;
use crate::store::PeakRecord;
use crate::timecode::format_timecode;

/// Position of an asset in the index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AssetId(pub u32);

/// One occurrence of a hash: the asset and the frame of the anchor peak,
/// packed into 32 bits.
///
/// 16 frame bits hold 17:28 at 16 ms per frame, above the 17-minute
/// default track limit. The remaining 16 bits address 65,536 assets
/// (ADR 0010).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Posting(u32);

const FRAME_BITS: u32 = 16;
pub const MAX_FRAMES: u32 = 1 << FRAME_BITS;
pub const MAX_ASSETS: usize = 1 << (32 - FRAME_BITS);

/// The longest record a posting's frame can address under the current
/// profile.
pub fn addressable_length() -> Duration {
    Duration::from_secs_f64(Profile::CURRENT.seconds(f64::from(MAX_FRAMES)))
}

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
    #[error(
        "the index holds at most {MAX_ASSETS} assets; leave folders out with .gunfingerignore or narrow the track length range (--min-track, --max-track)"
    )]
    TooManyAssets,
    #[error(
        "{path} is longer than the {limit} the index can address; set --max-track (`max_track`) to {limit} or less",
        limit = format_timecode(addressable_length())
    )]
    TooLong { path: String },
    #[error("a peak record changed while the index was being built; run the command again")]
    Changed,
}

pub struct Index {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
    postings: Vec<Posting>,
    /// Lists longer than this are left out of `scanned_postings`.
    longest_scanned: u32,
}

impl Index {
    /// Builds the index from peak records in memory; asset `i` is
    /// `records[i]`.
    pub fn build(records: &[PeakRecord]) -> Result<Index, IndexError> {
        let mut counting = Index::counting();
        for record in records {
            counting.count(record)?;
        }
        let mut filling = counting.into_filling();
        for record in records {
            filling.fill(record)?;
        }
        filling.finish()
    }

    /// Starts building an index in two passes over the peak records, each
    /// reading one record at a time: the first counts the postings of every
    /// hash to lay out the offsets table, the second fills each hash's
    /// slots. Only the index and one record are in memory at once.
    pub fn counting() -> Counting {
        Counting {
            assets: Vec::new(),
            offsets: vec![0; (1 << HASH_BITS) + 1],
        }
    }

    pub fn postings(&self, hash: PairHash) -> &[Posting] {
        let start = self.offsets[hash.0 as usize] as usize;
        let end = self.offsets[hash.0 as usize + 1] as usize;
        &self.postings[start..end]
    }

    /// The postings a search scans for candidates: all of them, unless
    /// `skipping_fullest` set the fullest lists aside.
    pub fn scanned_postings(&self, hash: PairHash) -> &[Posting] {
        let list = self.postings(hash);
        if list.len() as u32 > self.longest_scanned {
            &[]
        } else {
            list
        }
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

    /// Empties the posting lists of the most common hashes: the `share` of
    /// non-empty lists with the most postings (ties at the cut are kept).
    /// Such hashes occur all over the library, so they cost the most
    /// lookups and tell assets apart the least.
    pub fn without_fullest(self, share: f64) -> Index {
        let Some(longest_kept) = self.longest_kept(share) else {
            return self;
        };
        let mut offsets = Vec::with_capacity(self.offsets.len());
        let mut postings = Vec::new();
        offsets.push(0);
        for list in self.posting_lists() {
            if list.len() as u32 <= longest_kept {
                postings.extend_from_slice(list);
            }
            offsets.push(postings.len() as u32);
        }
        Index {
            assets: self.assets,
            offsets,
            postings,
            longest_scanned: u32::MAX,
        }
    }

    /// Like `without_fullest`, but keeps the lists: `scanned_postings`
    /// leaves them out, `postings` still returns them.
    pub fn skipping_fullest(self, share: f64) -> Index {
        match self.longest_kept(share) {
            Some(longest_scanned) => Index {
                longest_scanned,
                ..self
            },
            None => self,
        }
    }

    /// The longest list kept when the `share` of fullest non-empty lists
    /// is set aside; `None` when that share rounds to no list.
    fn longest_kept(&self, share: f64) -> Option<u32> {
        let mut lengths: Vec<u32> = self
            .offsets
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .filter(|&length| length > 0)
            .collect();
        let dropped = (share * lengths.len() as f64).round() as usize;
        if dropped == 0 {
            return None;
        }
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        Some(lengths.get(dropped).copied().unwrap_or(0))
    }

    /// Bytes the index occupies: the offsets table and the postings.
    pub fn size_bytes(&self) -> usize {
        size_of_val(self.offsets.as_slice()) + size_of_val(self.postings.as_slice())
    }
}

/// The first pass of a build: the assets in order, and the number of
/// postings of hash `h` at `offsets[h + 1]`.
pub struct Counting {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
}

impl Counting {
    /// Counts the postings of the next asset's record.
    pub fn count(&mut self, record: &PeakRecord) -> Result<(), IndexError> {
        if self.assets.len() == MAX_ASSETS {
            return Err(IndexError::TooManyAssets);
        }
        let points = points_of(record)?;
        for_each_pair(&points, |hash, _| self.offsets[hash.0 as usize + 1] += 1);
        self.assets.push(IndexedAsset {
            path: record.header.source.path.clone(),
            duration_seconds: record.header.duration_seconds,
        });
        Ok(())
    }

    /// Lays out the offsets table. The same records must then be filled in
    /// the same order.
    pub fn into_filling(self) -> Filling {
        let Counting {
            assets,
            mut offsets,
        } = self;
        for hash in 1..offsets.len() {
            offsets[hash] += offsets[hash - 1];
        }
        let next_slot = offsets[..offsets.len() - 1].to_vec();
        let postings = vec![Posting(0); offsets[offsets.len() - 1] as usize];
        Filling {
            assets,
            offsets,
            next_slot,
            postings,
            filled: 0,
        }
    }
}

/// The second pass of a build: each record's postings placed in its
/// hashes' slots, so that a hash's postings are ordered by asset and then
/// by frame.
pub struct Filling {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
    /// Where the next posting of each hash goes.
    next_slot: Vec<u32>,
    postings: Vec<Posting>,
    /// Assets filled so far.
    filled: usize,
}

impl Filling {
    /// Places the postings of the next asset's record. A record that is not
    /// the one counted in its place, or no longer has the same hashes, is an
    /// error rather than a corrupt index.
    pub fn fill(&mut self, record: &PeakRecord) -> Result<(), IndexError> {
        let path = &record.header.source.path;
        if self
            .assets
            .get(self.filled)
            .is_none_or(|asset| asset.path != *path)
        {
            return Err(IndexError::Changed);
        }
        let points = points_of(record)?;
        let asset = AssetId(self.filled as u32);
        let mut overflowed = false;
        for_each_pair(&points, |hash, anchor| {
            let hash = hash.0 as usize;
            let slot = self.next_slot[hash];
            if slot == self.offsets[hash + 1] {
                overflowed = true;
                return;
            }
            self.postings[slot as usize] = Posting::new(asset, points[anchor].frame.round() as u32);
            self.next_slot[hash] += 1;
        });
        if overflowed {
            return Err(IndexError::Changed);
        }
        self.filled += 1;
        Ok(())
    }

    /// The index, once every counted record has been filled.
    pub fn finish(self) -> Result<Index, IndexError> {
        let complete = self.filled == self.assets.len()
            && self
                .next_slot
                .iter()
                .zip(&self.offsets[1..])
                .all(|(next, end)| next == end);
        if !complete {
            return Err(IndexError::Changed);
        }
        Ok(Index {
            assets: self.assets,
            offsets: self.offsets,
            postings: self.postings,
            longest_scanned: u32::MAX,
        })
    }
}

/// A record's peaks as hashing points, checked against the frames a
/// posting can address. Peaks are in frame order, so the last one has the
/// latest anchor frame; a later frame would overflow into the asset bits.
fn points_of(record: &PeakRecord) -> Result<Vec<Point>, IndexError> {
    let points: Vec<Point> = record.peaks.iter().map(Point::from).collect();
    if points
        .last()
        .is_some_and(|last| last.frame.round() >= f64::from(MAX_FRAMES))
    {
        return Err(IndexError::TooLong {
            path: record.header.source.path.clone(),
        });
    }
    Ok(points)
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
    fn postings_pack_the_largest_asset_and_frame() {
        let posting = Posting::new(AssetId(65_535), 65_535);

        assert_eq!(posting.asset(), AssetId(65_535));
        assert_eq!(posting.frame(), 65_535);
        assert_eq!(MAX_ASSETS, 65_536);
        assert_eq!(MAX_FRAMES, 65_536);
    }

    #[test]
    fn the_index_takes_65536_assets_and_refuses_the_next() {
        let silent = record("silent.mp3", &[]);
        let mut counting = Index::counting();
        for _ in 0..65_536 {
            counting.count(&silent).unwrap();
        }

        let next = counting.count(&silent);

        assert!(matches!(next, Err(IndexError::TooManyAssets)));
    }

    #[test]
    fn a_record_ending_at_the_last_frame_keeps_its_frames() {
        let peaks = [(65_534.0, 100.0), (65_535.0, 110.0)];
        let index = Index::build(&[record("long.mp3", &peaks)]).unwrap();

        let postings = index.postings(hashes_of(&peaks)[0]);

        assert_eq!(postings.len(), 1);
        assert_eq!(postings[0].asset(), AssetId(0));
        assert_eq!(postings[0].frame(), 65_534);
    }

    #[test]
    fn a_record_past_17_28_is_refused() {
        let past = record("mix.mp3", &[(65_534.0, 100.0), (65_536.0, 110.0)]);

        let error = Index::build(&[past]).err().unwrap();

        assert!(matches!(error, IndexError::TooLong { .. }));
        assert_eq!(format_timecode(addressable_length()), "17:28");
        assert!(
            error
                .to_string()
                .contains("mix.mp3 is longer than the 17:28"),
            "{error}"
        );
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
    fn dropping_the_fullest_lists_empties_the_common_hash_only() {
        let first = [(10.0, 100.0), (20.0, 110.0)];
        let second = [
            (500.2, 100.0),
            (509.8, 110.0),
            (900.0, 300.0),
            (920.0, 300.0),
        ];
        let index = Index::build(&[record("a.mp3", &first), record("b.mp3", &second)]).unwrap();
        let lists = index
            .posting_lists()
            .filter(|list| !list.is_empty())
            .count();
        let before = index.posting_count();

        let thinned = index.without_fullest(1.0 / lists as f64);

        assert!(thinned.postings(hashes_of(&first)[0]).is_empty());
        assert_eq!(thinned.posting_count(), before - 2);
    }

    #[test]
    fn skipping_the_fullest_lists_keeps_them_for_counting() {
        let first = [(10.0, 100.0), (20.0, 110.0)];
        let second = [
            (500.2, 100.0),
            (509.8, 110.0),
            (900.0, 300.0),
            (920.0, 300.0),
        ];
        let index = Index::build(&[record("a.mp3", &first), record("b.mp3", &second)]).unwrap();
        let lists = index
            .posting_lists()
            .filter(|list| !list.is_empty())
            .count();
        let common = hashes_of(&first)[0];
        let rare = *hashes_of(&second).last().unwrap();

        let skipping = index.skipping_fullest(1.0 / lists as f64);

        assert!(skipping.scanned_postings(common).is_empty());
        assert_eq!(skipping.postings(common).len(), 2);
        assert_eq!(skipping.scanned_postings(rare).len(), 1);
    }

    #[test]
    fn filling_needs_the_records_counted_in_the_same_order() {
        let first = record("a.mp3", &[(10.0, 100.0), (20.0, 110.0)]);
        let second = record("b.mp3", &[(500.0, 100.0), (510.0, 110.0)]);
        let mut counting = Index::counting();
        counting.count(&first).unwrap();
        counting.count(&second).unwrap();

        let mut filling = counting.into_filling();
        let swapped = filling.fill(&second);

        assert!(matches!(swapped, Err(IndexError::Changed)));
    }

    #[test]
    fn a_record_that_changed_between_the_passes_is_an_error() {
        let counted = record("a.mp3", &[(10.0, 100.0), (20.0, 110.0)]);
        let changed = record("a.mp3", &[(10.0, 100.0), (20.0, 110.0), (30.0, 120.0)]);
        let fewer = record("a.mp3", &[(10.0, 100.0)]);

        let mut counting = Index::counting();
        counting.count(&counted).unwrap();
        let mut more_hashes = counting.into_filling();
        let filled_more = more_hashes.fill(&changed);

        let mut counting = Index::counting();
        counting.count(&counted).unwrap();
        let mut fewer_hashes = counting.into_filling();
        fewer_hashes.fill(&fewer).unwrap();

        assert!(matches!(filled_more, Err(IndexError::Changed)));
        assert!(matches!(fewer_hashes.finish(), Err(IndexError::Changed)));
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

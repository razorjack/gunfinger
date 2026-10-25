//! The inverted index: for every hash, where in the library it occurs.
//!
//! Postings are grouped by hash and addressed through an offsets table: the
//! postings of hash `h` are `postings[offsets[h]..offsets[h + 1]]`. This is the
//! layout an on-disk index would use, so its size measurements are real.

pub mod saved;

use std::collections::BTreeSet;
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
    pub(crate) fn new(asset: AssetId, frame: u32) -> Posting {
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
    #[error(
        "the index holds at most {} postings; leave folders out with .gunfingerignore or narrow the track length range (--min-track, --max-track)",
        u32::MAX
    )]
    TooManyPostings,
    #[error("a peak record changed while the index was being built; run the command again")]
    Changed,
}

pub struct Index {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
    postings: Vec<Posting>,
    /// The lists `scanned_postings` leaves out.
    set_aside: SetAside,
}

/// Which posting lists the search for candidates leaves out.
#[derive(Debug, PartialEq)]
enum SetAside {
    /// Lists longer than this (`u32::MAX`: none).
    Longer(u32),
    /// The lists of these hashes, one bit per hash (`skipping_most_held`).
    Hashes(Vec<u64>),
}

impl SetAside {
    const NONE: SetAside = SetAside::Longer(u32::MAX);

    fn holds(&self, hash: PairHash, list: &[Posting]) -> bool {
        match self {
            SetAside::Longer(longest) => list.len() as u32 > *longest,
            SetAside::Hashes(bits) => (bits[(hash.0 >> 6) as usize] >> (hash.0 & 63)) & 1 == 1,
        }
    }
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
    /// slots. Besides the index, only the records being counted or filled
    /// are in memory.
    pub fn counting() -> Counting {
        Counting {
            assets: Vec::new(),
            offsets: vec![0; (1 << HASH_BITS) + 1],
            postings: 0,
        }
    }

    /// A hash's postings, ordered by asset (and by frame within an asset
    /// when built from records).
    pub fn postings(&self, hash: PairHash) -> &[Posting] {
        let start = self.offsets[hash.0 as usize] as usize;
        let end = self.offsets[hash.0 as usize + 1] as usize;
        &self.postings[start..end]
    }

    /// The contiguous postings of one asset, in their original order.
    pub fn postings_for_asset(&self, hash: PairHash, asset: AssetId) -> &[Posting] {
        let postings = self.postings(hash);
        let first = postings.partition_point(|posting| posting.asset() < asset);
        let end = first + postings[first..].partition_point(|posting| posting.asset() == asset);
        &postings[first..end]
    }

    /// The postings a search scans for candidates: all of them, unless
    /// `skipping_fullest` set the fullest lists aside. Ordered by asset, as
    /// `postings`.
    pub fn scanned_postings(&self, hash: PairHash) -> &[Posting] {
        let list = self.postings(hash);
        if self.set_aside.holds(hash, list) {
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
            set_aside: SetAside::NONE,
        }
    }

    /// Like `without_fullest`, but keeps the lists: `scanned_postings`
    /// leaves them out, `postings` still returns them.
    pub fn skipping_fullest(self, share: f64) -> Index {
        match self.longest_kept(share) {
            Some(longest) => Index {
                set_aside: SetAside::Longer(longest),
                ..self
            },
            None => self,
        }
    }

    /// Like `skipping_fullest`, but ranks the lists by how many distinct
    /// holders their postings name instead of by their length: `holder`
    /// gives each asset's (the file itself, or the recording it belongs
    /// to). A file that repeats a hash, or copies of one recording, then
    /// count once. For measuring the filter's ranking (session 50); the
    /// matchers rank by length.
    pub fn skipping_most_held(self, share: f64, holder: impl Fn(AssetId) -> u32) -> Index {
        let holders: Vec<u32> = (0..self.assets.len() as u32)
            .map(|asset| holder(AssetId(asset)))
            .collect();
        let mut last_list =
            vec![u32::MAX; holders.iter().max().map_or(0, |&most| most as usize + 1)];
        let held_by: Vec<u32> = self
            .posting_lists()
            .enumerate()
            .map(|(list, postings)| {
                let mut distinct = 0;
                for posting in postings {
                    let holder = holders[posting.asset().0 as usize] as usize;
                    if last_list[holder] != list as u32 {
                        last_list[holder] = list as u32;
                        distinct += 1;
                    }
                }
                distinct
            })
            .collect();
        let Some(most_kept) = kept_at_cut(held_by.iter().copied(), share) else {
            return self;
        };
        let mut bits = vec![0_u64; held_by.len().div_ceil(64)];
        for (hash, &held) in held_by.iter().enumerate() {
            if held > most_kept {
                bits[hash >> 6] |= 1 << (hash & 63);
            }
        }
        Index {
            set_aside: SetAside::Hashes(bits),
            ..self
        }
    }

    /// The longest list kept when the `share` of fullest non-empty lists
    /// is set aside; `None` when that share rounds to no list.
    fn longest_kept(&self, share: f64) -> Option<u32> {
        kept_at_cut(self.offsets.windows(2).map(|pair| pair[1] - pair[0]), share)
    }

    /// The index as built without the assets whose paths `left_out` names:
    /// their postings removed and the assets after them renumbered in
    /// order. A build places each hash's postings asset by asset, each
    /// asset's in the order its record gives them, so this is the index
    /// built from the other records in the same order, posting for posting
    /// (ADR 0014). Call it on an index as built: lists `skipping_fullest`
    /// or `skipping_most_held` set aside would stay set aside by their old
    /// lengths or holders.
    pub fn without_assets(mut self, left_out: &BTreeSet<String>) -> Index {
        let mut kept = 0;
        let renumbered: Vec<Option<AssetId>> = self
            .assets
            .iter()
            .map(|asset| {
                (!left_out.contains(&asset.path)).then(|| {
                    kept += 1;
                    AssetId(kept - 1)
                })
            })
            .collect();
        if kept as usize == self.assets.len() {
            return self;
        }
        // Postings move only towards the start, so each is read before a
        // kept one is written over it.
        let mut written = 0;
        let mut start = 0;
        for hash in 1..self.offsets.len() {
            let end = self.offsets[hash] as usize;
            for read in start..end {
                let posting = self.postings[read];
                if let Some(asset) = renumbered[posting.asset().0 as usize] {
                    self.postings[written] = Posting::new(asset, posting.frame());
                    written += 1;
                }
            }
            start = end;
            self.offsets[hash] = written as u32;
        }
        self.postings.truncate(written);
        self.assets.retain(|asset| !left_out.contains(&asset.path));
        self
    }

    /// Bytes the index occupies: the offsets table and the postings.
    pub fn size_bytes(&self) -> usize {
        size_of_val(self.offsets.as_slice()) + size_of_val(self.postings.as_slice())
    }
}

/// The largest value kept when the `share` of the non-zero `values` with
/// the largest values is set aside (ties at the cut are kept); `None`
/// when that share rounds to none.
fn kept_at_cut(values: impl Iterator<Item = u32>, share: f64) -> Option<u32> {
    let mut values: Vec<u32> = values.filter(|&value| value > 0).collect();
    let dropped = (share * values.len() as f64).round() as usize;
    if dropped == 0 {
        return None;
    }
    values.sort_unstable_by(|a, b| b.cmp(a));
    Some(values.get(dropped).copied().unwrap_or(0))
}

/// The first pass of a build: the assets in order, and the number of
/// postings of hash `h` at `offsets[h + 1]`.
pub struct Counting {
    assets: Vec<IndexedAsset>,
    offsets: Vec<u32>,
    /// All postings counted. The offsets table's `u32` positions address
    /// 4.29 billion; past that its sums would wrap.
    postings: u64,
}

impl Counting {
    /// Counts the postings of the next asset's record.
    pub fn count(&mut self, record: &PeakRecord) -> Result<(), IndexError> {
        self.room()?;
        self.count_hashed(&HashedRecord::new(record)?)
    }

    /// Counts the postings of the next asset's record, hashed.
    pub fn count_hashed(&mut self, record: &HashedRecord) -> Result<(), IndexError> {
        self.room()?;
        let pairs = record.pairs.len() as u64;
        if self.postings + pairs > u64::from(u32::MAX) {
            return Err(IndexError::TooManyPostings);
        }
        self.postings += pairs;
        for &(hash, _) in &record.pairs {
            self.offsets[hash.0 as usize + 1] += 1;
        }
        self.assets.push(IndexedAsset {
            path: record.path.clone(),
            duration_seconds: record.duration_seconds,
        });
        Ok(())
    }

    /// `TooManyAssets` once the index holds as many as it can address.
    pub fn room(&self) -> Result<(), IndexError> {
        if self.assets.len() == MAX_ASSETS {
            return Err(IndexError::TooManyAssets);
        }
        Ok(())
    }

    /// Lays out the offsets table. The same records must then be filled in
    /// the same order.
    pub fn into_filling(self) -> Filling {
        let Counting {
            assets,
            mut offsets,
            ..
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
        self.next_is(&record.header.source.path)?;
        self.fill_hashed(&HashedRecord::new(record)?)
    }

    /// Places the postings of the next asset's record, hashed.
    pub fn fill_hashed(&mut self, record: &HashedRecord) -> Result<(), IndexError> {
        self.next_is(&record.path)?;
        let asset = AssetId(self.filled as u32);
        for &(hash, frame) in &record.pairs {
            let hash = hash.0 as usize;
            let slot = self.next_slot[hash];
            if slot == self.offsets[hash + 1] {
                return Err(IndexError::Changed);
            }
            self.postings[slot as usize] = Posting::new(asset, frame);
            self.next_slot[hash] += 1;
        }
        self.filled += 1;
        Ok(())
    }

    fn next_is(&self, path: &str) -> Result<(), IndexError> {
        if self
            .assets
            .get(self.filled)
            .is_none_or(|asset| asset.path != path)
        {
            return Err(IndexError::Changed);
        }
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
            set_aside: SetAside::NONE,
        })
    }
}

/// A record's postings before it has an asset id: each pair's hash and
/// anchor frame, in `for_each_pair`'s order. Records can be hashed on
/// several threads, then counted and filled one at a time in asset order.
pub struct HashedRecord {
    path: String,
    duration_seconds: f64,
    pairs: Vec<(PairHash, u32)>,
}

impl HashedRecord {
    pub fn new(record: &PeakRecord) -> Result<HashedRecord, IndexError> {
        let points = points_of(record)?;
        let mut pairs = Vec::new();
        for_each_pair(&points, |hash, anchor| {
            pairs.push((hash, points[anchor].frame.round() as u32));
        });
        Ok(HashedRecord {
            path: record.header.source.path.clone(),
            duration_seconds: record.header.duration_seconds,
            pairs,
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
    fn asset_postings_match_a_full_scan_including_missing_assets() {
        let peaks = [(10.0, 100.0), (20.0, 110.0), (30.0, 120.0)];
        let index = Index::build(&[record("a.mp3", &peaks), record("b.mp3", &peaks)]).unwrap();
        for hash in hashes_of(&peaks) {
            for asset in [AssetId(0), AssetId(1), AssetId(2)] {
                let expected: Vec<_> = index
                    .postings(hash)
                    .iter()
                    .copied()
                    .filter(|p| p.asset() == asset)
                    .collect();
                assert_eq!(index.postings_for_asset(hash, asset), expected);
            }
        }
    }

    #[test]
    fn leaving_assets_out_gives_the_index_built_without_them() {
        let records: Vec<PeakRecord> = (0..6)
            .map(|asset| {
                // Shared peaks put every asset in the same lists; the
                // asset's own peak adds lists of its own.
                let own = 150.0 + 10.0 * asset as f32;
                record(
                    &format!("{asset}.mp3"),
                    &[(10.0, 100.0), (20.0, own), (30.0, 120.0), (40.0, 110.0)],
                )
            })
            .collect();
        let left_out: BTreeSet<String> = ["0.mp3", "3.mp3", "4.mp3", "missing.mp3"]
            .into_iter()
            .map(String::from)
            .collect();
        let kept: Vec<PeakRecord> = records
            .iter()
            .filter(|record| !left_out.contains(&record.header.source.path))
            .cloned()
            .collect();

        let filtered = Index::build(&records).unwrap().without_assets(&left_out);
        let built = Index::build(&kept).unwrap();

        assert_eq!(filtered.assets(), built.assets());
        assert_eq!(filtered.offsets, built.offsets);
        assert_eq!(filtered.postings, built.postings);
        assert_eq!(filtered.set_aside, SetAside::NONE);
        assert_eq!(filtered.postings.iter().map(|p| p.asset().0).max(), Some(2));
    }

    #[test]
    fn leaving_out_no_indexed_asset_keeps_the_index() {
        let peaks = [(10.0, 100.0), (20.0, 110.0), (30.0, 120.0)];
        let index = Index::build(&[record("a.mp3", &peaks), record("b.mp3", &peaks)]).unwrap();
        let postings = index.postings.clone();

        let kept = index.without_assets(&BTreeSet::from([String::from("c.mp3")]));

        assert_eq!(kept.assets().len(), 2);
        assert_eq!(kept.postings, postings);
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
    fn the_index_refuses_more_postings_than_its_offsets_address() {
        let two_pairs = record("a.mp3", &[(10.0, 100.0), (20.0, 110.0), (30.0, 120.0)]);
        let mut counting = Index::counting();
        counting.postings = u64::from(u32::MAX) - 1;

        let refused = counting.count(&two_pairs);

        assert!(matches!(refused, Err(IndexError::TooManyPostings)));
        assert!(counting.assets.is_empty());
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
    fn ranking_by_holders_sets_aside_the_list_most_files_hold() {
        // One file repeats a hash four times (a loop); three files hold
        // another once each. By length the loop's list is the fullest, by
        // files the other.
        let looped = [
            (10.0, 100.0),
            (20.0, 110.0),
            (110.0, 100.0),
            (120.0, 110.0),
            (210.0, 100.0),
            (220.0, 110.0),
            (310.0, 100.0),
            (320.0, 110.0),
        ];
        let shared = [(10.0, 300.0), (20.0, 320.0)];
        let records = [
            record("loop.mp3", &looped),
            record("a.mp3", &shared),
            record("b.mp3", &shared),
            record("c.mp3", &shared),
        ];
        let loop_hash = hashes_of(&[(10.0, 100.0), (20.0, 110.0)])[0];
        let shared_hash = hashes_of(&shared)[0];
        let lists = |index: &Index| {
            index
                .posting_lists()
                .filter(|list| !list.is_empty())
                .count()
        };
        let index = Index::build(&records).unwrap();
        let share = 1.0 / lists(&index) as f64;
        assert_eq!(index.postings(loop_hash).len(), 4);
        assert_eq!(index.postings(shared_hash).len(), 3);

        let by_length = Index::build(&records).unwrap().skipping_fullest(share);
        let by_files = index.skipping_most_held(share, |asset| asset.0);

        assert!(by_length.scanned_postings(loop_hash).is_empty());
        assert_eq!(by_length.scanned_postings(shared_hash).len(), 3);
        assert_eq!(by_files.scanned_postings(loop_hash).len(), 4);
        assert!(by_files.scanned_postings(shared_hash).is_empty());
        assert_eq!(by_files.postings(shared_hash).len(), 3);
    }

    #[test]
    fn copies_given_one_holder_count_once() {
        // a.mp3 and two copies hold one hash, b.mp3 and c.mp3 another: by
        // files the first list is held most, by recording the second.
        let first = [(10.0, 100.0), (20.0, 110.0)];
        let second = [(10.0, 300.0), (20.0, 320.0)];
        let records = [
            record("a.mp3", &first),
            record("copy-1/a.mp3", &first),
            record("copy-2/a.mp3", &first),
            record("b.mp3", &second),
            record("c.mp3", &second),
        ];
        let share = 1.0 / 2.0;
        let recording = |asset: AssetId| if asset.0 < 3 { 0 } else { asset.0 };

        let by_files = Index::build(&records)
            .unwrap()
            .skipping_most_held(share, |asset| asset.0);
        let by_recording = Index::build(&records)
            .unwrap()
            .skipping_most_held(share, recording);

        assert!(by_files.scanned_postings(hashes_of(&first)[0]).is_empty());
        assert_eq!(by_files.scanned_postings(hashes_of(&second)[0]).len(), 2);
        assert_eq!(by_recording.scanned_postings(hashes_of(&first)[0]).len(), 3);
        assert!(
            by_recording
                .scanned_postings(hashes_of(&second)[0])
                .is_empty()
        );
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

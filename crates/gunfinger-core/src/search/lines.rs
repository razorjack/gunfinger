//! Lines: hits of one asset, within one window of query time, whose offsets
//! agree. This is Wang 2003's offset histogram (§2.3), found by sorting each
//! asset's hits by offset, with the rung's speed as the line's slope. Assets
//! whose hits cannot form a line are not sorted.

use std::mem;
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use super::{KeyLockPeaks, Query};
use crate::hash::{MAX_DELTA_FRAMES, Point, for_each_pair};
use crate::index::{AssetId, Index, MAX_FRAMES, Posting};
use crate::parallel::map_in_order_with;
use crate::profile::Profile;
use crate::speed::{Playback, Rung};

/// Query time is cut into windows this long. Within one window a residual
/// speed error of 0.2% drifts the offset by about one frame.
pub(super) const WINDOW_SECONDS: f64 = 10.0;
/// Hits whose offsets lie within this many frames belong to one line.
pub(super) const LINE_SPAN_FRAMES: f64 = 2.0;
/// Fewer hits than this in a window are not worth keeping as a line.
pub(super) const MIN_LINE_HITS: usize = 3;
/// A looped section matches its reference at several offsets; keep a few.
const MAX_LINES_PER_ASSET: usize = 3;
/// Lines from neighbouring rungs predicting reference times this close are
/// the same alignment.
const SAME_LINE_FRAMES: f64 = 4.0;
/// A window's hits are grouped by asset this many assets at a time. Smaller
/// parts were faster with two workers per core, by inference because each
/// worker's hits then stay in its share of the cache (experiments 0167-0169).
const ASSETS_PER_PART: usize = 1024;
/// Offsets are counted in buckets this wide (`OffsetSlots`), so that a
/// line's offsets lie in one bucket or two neighbouring ones.
const OFFSET_BUCKET_FRAMES: f64 = 2.0 * LINE_SPAN_FRAMES;
/// The buckets are folded into this many slots.
const OFFSET_SLOTS: usize = 4096;
// A pair's count stops at `MIN_LINE_HITS`, below the high bit of its byte
// (`OffsetSlots::may_form_line`).
const _: () = assert!(MIN_LINE_HITS < 0x80);
// A slot is kept as a `u16`.
const _: () = assert!(OFFSET_SLOTS <= 1 << 16);
// Integer offsets are a posting's 16-bit frame minus a phase, modulo 2^16,
// which folds whole buckets of whole frames into the slots (`Hit::slot`).
const _: () = assert!(MAX_FRAMES == 1 << 16);
const _: () = assert!(OFFSET_BUCKET_FRAMES as usize as f64 == OFFSET_BUCKET_FRAMES);
const _: () =
    assert!((MAX_FRAMES as usize).is_multiple_of(OFFSET_BUCKET_FRAMES as usize * OFFSET_SLOTS));

pub(super) fn design() -> String {
    format!(
        "window={WINDOW_SECONDS}s line={LINE_SPAN_FRAMES}fr min-hits={MIN_LINE_HITS} per-asset={MAX_LINES_PER_ASSET} same={SAME_LINE_FRAMES}fr"
    )
}

/// Hits of one asset along one line within one window.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Line {
    pub asset: AssetId,
    pub window: u32,
    /// The rung the line was found on.
    pub speed: f64,
    pub playback: Playback,
    /// Mean of the hits' reference frame minus `speed` times query frame.
    pub offset: f64,
    pub hits: u32,
    /// Query frames of the first and last hit.
    pub first: f64,
    pub last: f64,
}

impl Line {
    pub fn reference_frame_at(&self, query_frame: f64) -> f64 {
        self.speed * query_frame + self.offset
    }

    pub fn centre(&self) -> f64 {
        (self.first + self.last) / 2.0
    }
}

/// A posting a query hash hit, as taken from its list: 8 bytes. A window
/// hits up to 2 million postings at 33,596 assets (experiment 0086), held
/// one part at a time.
#[derive(Clone, Copy)]
struct Hit {
    /// The posting, its frame replaced by the hit's integer offset: the
    /// frame minus the anchor's phase (`Phases`), modulo 2^16. The
    /// pre-filter reads nothing else (experiment 0190).
    shifted: Posting,
    /// The anchor's position among the rung's points.
    anchor: u32,
}

impl Hit {
    fn taken(posting: Posting, anchor: u32, phase: u16) -> Hit {
        let offset = (posting.frame() as u16).wrapping_sub(phase);
        Hit {
            shifted: Posting::new(posting.asset(), u32::from(offset)),
            anchor,
        }
    }

    fn asset(self) -> AssetId {
        self.shifted.asset()
    }

    /// The posting's frame, given back exactly from the integer offset.
    fn frame(self, phases: &[u16]) -> u32 {
        let offset = self.shifted.frame() as u16;
        u32::from(offset.wrapping_add(phases[self.anchor as usize]))
    }

    /// The slot of the integer offset: buckets of `OFFSET_BUCKET_FRAMES`
    /// folded into `OFFSET_SLOTS` slots. The offset is kept modulo 2^16, a
    /// multiple of the 4 x 4,096 frames the slots cover, so it folds like
    /// the buckets.
    fn slot(self) -> usize {
        self.shifted.frame() as usize / OFFSET_BUCKET_FRAMES as usize % OFFSET_SLOTS
    }
}

/// A hit of an asset whose hits may form a line, with its alignment.
#[derive(Clone, Copy)]
struct AlignedHit {
    /// Reference frame minus the anchor's frame at the assumed speed.
    offset: f64,
    /// The anchor's frame at the query's own speed.
    query_frame: f64,
}

/// One worker's hits, kept for all its jobs: the posting lists the window
/// being searched hit, one part of the assets' hits taken from them, the
/// same hits grouped by asset, and the aligned hits of the asset being
/// turned into lines.
///
/// Each thread keeps its buffers for all its jobs: buffers freed after
/// every rung were kept by the allocator, 15.7 GB instead of 5.0 GB for 10
/// minutes of query on 10 threads, when a window's hits were held whole
/// (200 MB at 26,000 assets, experiment 0029).
struct WindowHits<'i> {
    /// The postings each query hash of the window hit, with the hash's
    /// anchor, in the order they were looked up. Each list is ordered by
    /// asset, as the index holds it; the hits of the parts already grouped
    /// are taken off its front.
    lists: Vec<(&'i [Posting], u32)>,
    assets: usize,
    part: Vec<Hit>,
    grouped: Vec<Hit>,
    aligned: Vec<AlignedHit>,
    /// A counter for each asset of a part and one more, all zero between
    /// parts (see `group_part`).
    ends: Vec<u32>,
    /// The phases of the points of the block and rung being searched.
    phases: Phases,
    slots: OffsetSlots,
}

impl<'i> WindowHits<'i> {
    fn new(assets: usize) -> WindowHits<'i> {
        WindowHits {
            lists: Vec::new(),
            assets,
            part: Vec::new(),
            grouped: Vec::new(),
            aligned: Vec::new(),
            ends: vec![0; ASSETS_PER_PART + 1],
            phases: Phases::default(),
            slots: OffsetSlots::new(),
        }
    }

    /// Keeps the postings a query hash of the window being searched hit.
    fn find(&mut self, postings: &'i [Posting], anchor: u32) {
        if !postings.is_empty() {
            self.lists.push((postings, anchor));
        }
    }

    /// Groups the window's hits by asset, turns each asset's densest offset
    /// clusters into lines, and empties the buffers for the next window.
    ///
    /// Lines depend only on each asset's offsets in sorted order: a chosen
    /// cluster never splits a run of equal offsets (the candidate starting
    /// at the run's first hit is longer and is considered first), so any
    /// order of ties gives the same lines.
    ///
    /// At 33,596 assets nearly every asset has 3 hits in a window, and
    /// sorting them all took 40% of a search's CPU; `may_form_line` leaves
    /// 2% of them to sort (experiment 0093). Only those are aligned.
    fn lines(&mut self, window: u32, rung: Rung, points: &[Point]) -> Vec<Line> {
        let speed = rung.speed();
        let mut lines = Vec::new();
        for first in (0..self.assets).step_by(ASSETS_PER_PART) {
            let end = (first + ASSETS_PER_PART).min(self.assets);
            self.take_part(end);
            self.group_part(first);
            let mut start = 0;
            for asset in first..end {
                let same_asset = &self.grouped[start..self.ends[asset - first] as usize];
                start = self.ends[asset - first] as usize;
                if same_asset.len() < MIN_LINE_HITS
                    || (self.phases.exact && !self.slots.may_form_line(same_asset))
                {
                    continue;
                }
                self.aligned.clear();
                self.aligned.extend(same_asset.iter().map(|&hit| {
                    let anchor = points[hit.anchor as usize].frame;
                    AlignedHit {
                        offset: f64::from(hit.frame(&self.phases.of_points)) - anchor,
                        query_frame: anchor / speed.0,
                    }
                }));
                self.aligned
                    .sort_unstable_by(|a, b| a.offset.total_cmp(&b.offset));
                for cluster in densest_clusters(&self.aligned) {
                    lines.push(line_of(
                        AssetId(asset as u32),
                        &self.aligned[cluster],
                        window,
                        rung,
                    ));
                }
            }
            self.ends.fill(0);
        }
        self.lists.clear();
        lines
    }

    /// Takes off the front of every list the hits of assets before `end`,
    /// list by list, so that each asset's hits keep the order they were
    /// found in, each with its integer offset (`Hit::taken`).
    ///
    /// Grouping all of a window's hits by asset at once wrote each to one of
    /// as many places as there are assets, nearly every write a cache miss
    /// at 33,596 assets; a part's hits fit in the cache (experiment 0096).
    fn take_part(&mut self, end: usize) {
        self.part.clear();
        for (postings, anchor) in &mut self.lists {
            let taken = front_partition(postings, end);
            let (part, rest) = postings.split_at(taken);
            let phase = self.phases.of_points[*anchor as usize];
            self.part.extend(
                part.iter()
                    .map(|&posting| Hit::taken(posting, *anchor, phase)),
            );
            *postings = rest;
        }
    }

    /// A counting sort of the part's hits by asset: asset numbers are dense,
    /// so two passes over the hits group them (experiments 0075, 0085,
    /// 0086). Leaves the hits of the part's `i`th asset in
    /// `grouped[ends[i - 1]..ends[i]]`, in the order they were taken.
    fn group_part(&mut self, first: usize) {
        let ends = &mut self.ends;
        for hit in &self.part {
            ends[hit.asset().0 as usize - first + 1] += 1;
        }
        for asset in 1..ends.len() {
            ends[asset] += ends[asset - 1];
        }
        if self.grouped.len() < self.part.len() {
            self.grouped.resize(self.part.len(), self.part[0]);
        }
        for &hit in &self.part {
            let slot = &mut ends[hit.asset().0 as usize - first];
            self.grouped[*slot as usize] = hit;
            *slot += 1;
        }
    }
}

/// How many postings at the front of `postings`, ordered by asset, belong
/// to assets before `end`: `partition_point`, searched from the front.
///
/// A part takes a short run off the front of each list. A binary search
/// over the whole remainder reads about 7 cache lines far from that run,
/// one after another, for every list and part; doubling from the front
/// reads only near it, which lets parts be small (experiment 0167).
fn front_partition(postings: &[Posting], end: usize) -> usize {
    let before = |posting: &Posting| (posting.asset().0 as usize) < end;
    if postings.first().is_none_or(|first| !before(first)) {
        return 0;
    }
    // When the loop ends, `postings[known]` is before `end` and the boundary
    // lies after it, at or before `beyond`.
    let mut known = 0;
    let mut beyond = 1;
    while beyond < postings.len() && before(&postings[beyond]) {
        known = beyond;
        beyond *= 2;
    }
    let beyond = beyond.min(postings.len());
    known + 1 + postings[known + 1..beyond].partition_point(before)
}

/// Query windows searched on every rung before their lines are merged.
/// Each block repeats the analysis of about 100 STFT frames at its edges
/// (1% of a block of this length).
const BLOCK_WINDOWS: u32 = 12;

/// The distinct lines of every rung (`distinct`), sorted by asset and
/// window. `progress` gets the parts of the search finished and their
/// number after each.
///
/// The query is searched a block of windows at a time on every rung, each
/// block and rung a job for the next free worker thread. Once every rung
/// has searched a block, its lines are merged, so only the blocks being
/// searched are held unmerged. The lines and their order are those of
/// searching the whole query on each rung and merging at the end:
/// `distinct` compares lines of one asset and window only, and a window's
/// hits come from the same anchors in the same order.
pub(super) fn on_ladder(
    index: &Index,
    query: Query,
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize, usize) + Sync,
) -> Vec<Line> {
    in_blocks(index, query, profile, ladder, jobs, BLOCK_WINDOWS, progress)
}

fn in_blocks(
    index: &Index,
    query: Query,
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    block_windows: u32,
    progress: impl Fn(usize, usize) + Sync,
) -> Vec<Line> {
    let windows = (query.frames(profile) / profile.frames(WINDOW_SECONDS)) as u32 + 1;
    let blocks = windows.div_ceil(block_windows);
    let groups = rung_groups(ladder, profile);
    let work: Vec<_> = (0..blocks)
        .flat_map(|block| groups.iter().cloned().map(move |rungs| (block, rungs)))
        .collect();
    let unmerged: Vec<Mutex<Unmerged>> = (0..blocks)
        .map(|_| {
            Mutex::new(Unmerged {
                rungs: vec![None; ladder.len()],
                missing: ladder.len(),
            })
        })
        .collect();
    let merged = Mutex::new(Vec::new());
    let finished = AtomicUsize::new(0);
    let assets = index.assets().len();
    map_in_order_with(
        &work,
        jobs,
        || WindowHits::new(assets),
        |hits, (block, rungs)| {
            let first = block * block_windows;
            let mut cached = KeyLockPeaks::default();
            // The last block takes every window left, however long the query.
            let end = if block + 1 == blocks {
                u32::MAX
            } else {
                first + block_windows
            };
            for rung in rungs.clone() {
                let lines = in_windows(
                    index,
                    query,
                    profile,
                    ladder[rung],
                    first..end,
                    hits,
                    &mut cached,
                );
                let complete = {
                    let mut block = lock(&unmerged[*block as usize]);
                    block.rungs[rung] = Some(lines);
                    block.missing -= 1;
                    (block.missing == 0).then(|| mem::take(&mut block.rungs))
                };
                if let Some(rungs) = complete {
                    let mut lines = Vec::with_capacity(rungs.iter().flatten().map(Vec::len).sum());
                    for rung in rungs.into_iter().flatten() {
                        lines.extend(rung);
                    }
                    let kept = distinct(lines);
                    lock(&merged).extend(kept);
                }
                progress(
                    finished.fetch_add(1, Ordering::Relaxed) + 1,
                    blocks as usize * ladder.len(),
                );
            }
        },
    );
    let mut lines = merged.into_inner().unwrap_or_else(PoisonError::into_inner);
    // Blocks finish in any order. Their windows differ, so a stable sort
    // gives the order merging all lines at once gives.
    lines.sort_by_key(|line| (line.asset, line.window));
    lines
}

fn rung_groups(ladder: &[Rung], profile: &Profile) -> Vec<Range<usize>> {
    let mut groups = Vec::new();
    let mut next = 0;
    while next < ladder.len() {
        let first = next;
        next += 1;
        if matches!(ladder[first], Rung::KeyLocked(_)) {
            while next < ladder.len()
                && matches!(ladder[next], Rung::KeyLocked(_))
                && ladder[next].hop(profile) == ladder[first].hop(profile)
            {
                next += 1;
            }
        }
        groups.push(first..next);
    }
    groups
}

/// One block's lines, by rung in ladder order, until every rung has
/// searched it.
struct Unmerged {
    rungs: Vec<Option<Vec<Line>>>,
    missing: usize,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    // A panicking worker is re-raised by `map_in_order_with`.
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Looks up the query hashes of the anchors in `windows` at one assumed
/// speed and collects the lines each window's hits form. `hits` holds no
/// hit before and after.
fn in_windows<'i>(
    index: &'i Index,
    query: Query,
    profile: &Profile,
    rung: Rung,
    windows: Range<u32>,
    hits: &mut WindowHits<'i>,
    cached: &mut KeyLockPeaks,
) -> Vec<Line> {
    let speed = rung.speed();
    let window_frames = profile.frames(WINDOW_SECONDS);
    // An anchor's targets lie up to `MAX_DELTA_FRAMES` reference frames
    // after it.
    let reach = (MAX_DELTA_FRAMES + 1.0) / speed.0;
    let points = query.points_near(
        rung,
        profile,
        f64::from(windows.start) * window_frames..f64::from(windows.end) * window_frames + reach,
        cached,
    );
    hits.phases.find(&points);
    let mut lines = Vec::new();
    let mut window = 0;
    for_each_pair(&points, |hash, anchor| {
        let anchor_window = (points[anchor].frame / speed.0 / window_frames) as u32;
        // Anchors are ordered by frame to within a frame, so a window only
        // ever moves forward. Anchors of earlier STFT frames lie earlier, so
        // the points before `points` cannot move it.
        if anchor_window > window {
            if windows.contains(&window) {
                lines.extend(hits.lines(window, rung, &points));
            }
            window = anchor_window;
        }
        if windows.contains(&window) {
            hits.find(index.scanned_postings(hash), anchor as u32);
        }
    });
    if windows.contains(&window) {
        lines.extend(hits.lines(window, rung, &points));
    }
    lines
}

/// The line of one asset's hits in a cluster.
fn line_of(asset: AssetId, members: &[AlignedHit], window: u32, rung: Rung) -> Line {
    let query_frames = members.iter().map(|hit| hit.query_frame);
    Line {
        asset,
        window,
        speed: rung.speed().0,
        playback: rung.playback(),
        offset: members.iter().map(|hit| hit.offset).sum::<f64>() / members.len() as f64,
        hits: members.len() as u32,
        first: query_frames.clone().fold(f64::INFINITY, f64::min),
        last: query_frames.fold(f64::NEG_INFINITY, f64::max),
    }
}

/// Points this far from frame 0 or farther leave too little of a frame
/// for the integer offsets' rounding (see `OffsetSlots::may_form_line`).
const MAX_PHASE_FRAMES: f64 = (1_u64 << 31) as f64;

/// Each point's frame rounded down, modulo 2^16: its phase. A hit's
/// reference frame minus its anchor's phase is its integer offset, from
/// which the pre-filter finds its slot without floating point. Slots of
/// `f64` offsets, with a call to `floor` per hit on x86-64, were 37-46% of
/// the filter's time on the Linux machine (experiment 0189).
#[derive(Default)]
struct Phases {
    of_points: Vec<u16>,
    /// Whether every point lies within `MAX_PHASE_FRAMES` of frame 0. When
    /// one does not (no query of a few hours comes near), every asset is
    /// sorted.
    exact: bool,
}

impl Phases {
    fn find(&mut self, points: &[Point]) {
        self.exact = points
            .iter()
            .all(|point| point.frame.abs() < MAX_PHASE_FRAMES);
        self.of_points.clear();
        // Within `MAX_PHASE_FRAMES` the cast to `i64` is exact, and the
        // cast to `u16` keeps the low 16 bits: the floor modulo 2^16.
        self.of_points
            .extend(points.iter().map(|point| point.frame.floor() as i64 as u16));
    }
}

/// Counts of one asset's offsets in pairs of neighbouring slots, the
/// slots being buckets of `OFFSET_BUCKET_FRAMES` folded into
/// `OFFSET_SLOTS`: byte `k` counts the hits in slots `k - 1` and `k`, so
/// that a hit in slot `s` adds to bytes `s` and `s + 1`, one 16-bit word.
/// Byte 0 and byte `OFFSET_SLOTS` both count the pair of the last slot and
/// slot 0. All zero between assets.
struct OffsetSlots {
    pairs: Box<[u8; OFFSET_SLOTS + 1]>,
}

/// Adds one to both bytes of a word.
const BOTH: u16 = 0x0101;
/// Added to a word of two counts, sets a byte's high bit when its count
/// has reached `MIN_LINE_HITS`. A count is at most `MIN_LINE_HITS` (the
/// filter stops there), so a byte's sum is at most 0x80 and no carry
/// crosses into the other byte.
const REACHED: u16 = (0x80 - MIN_LINE_HITS as u16) * BOTH;
const HIGH_BITS: u16 = 0x8080;

impl OffsetSlots {
    fn new() -> OffsetSlots {
        OffsetSlots {
            pairs: Box::new([0; OFFSET_SLOTS + 1]),
        }
    }

    /// Whether one asset's hits, in any order, may hold a cluster for
    /// `densest_clusters`: false only when no 3 of their integer offsets
    /// lie in one slot or two neighbouring slots.
    ///
    /// A hit's integer offset, reference frame minus its anchor's frame
    /// rounded down, exceeds its offset by less than a frame. A cluster's
    /// offsets differ by at most `LINE_SPAN_FRAMES` as computed; with
    /// points within `MAX_PHASE_FRAMES` of frame 0 they err by less than
    /// 2^-20 frames, so the true offsets differ by less than 2.001 and the
    /// integer offsets by less than 3.001, at most 3. Buckets of
    /// `OFFSET_BUCKET_FRAMES` then differ by at most one. Folding buckets
    /// into slots keeps neighbours neighbours (slot `OFFSET_SLOTS - 1`
    /// neighbours slot 0) and only adds counts.
    ///
    /// A hit adds to the two pairs that hold its slot with one 16-bit
    /// word, and the test of both is one addition and a mask: counting
    /// each slot alone and reading its neighbours apart cost two more loads
    /// per hit (experiment 0203). The counted pairs are found again from the
    /// hits to clear them, which costs less than keeping a list of them
    /// (experiment 0190).
    fn may_form_line(&mut self, hits: &[Hit]) -> bool {
        let pairs = &mut self.pairs;
        let mut may = false;
        let mut counted = hits.len();
        for (i, &hit) in hits.iter().enumerate() {
            let slot = hit.slot();
            let word = u16::from_le_bytes([pairs[slot], pairs[slot + 1]]) + BOTH;
            pairs[slot..slot + 2].copy_from_slice(&word.to_le_bytes());
            if slot == 0 {
                pairs[OFFSET_SLOTS] += 1;
            } else if slot == OFFSET_SLOTS - 1 {
                pairs[0] += 1;
            }
            if (word + REACHED) & HIGH_BITS != 0 {
                may = true;
                counted = i + 1;
                break;
            }
        }
        for &hit in &hits[..counted] {
            let slot = hit.slot();
            pairs[slot] = 0;
            pairs[slot + 1] = 0;
        }
        pairs[0] = 0;
        pairs[OFFSET_SLOTS] = 0;
        may
    }
}

/// Index ranges of the densest non-overlapping offset clusters in hits
/// sorted by offset: for each hit, the hits within `LINE_SPAN_FRAMES` after
/// it form a candidate; the largest candidates win.
fn densest_clusters(hits: &[AlignedHit]) -> Vec<Range<usize>> {
    let mut candidates = Vec::new();
    let mut end = 0;
    for start in 0..hits.len() {
        end = end.max(start);
        while end < hits.len() && hits[end].offset - hits[start].offset <= LINE_SPAN_FRAMES {
            end += 1;
        }
        if end - start >= MIN_LINE_HITS {
            candidates.push(start..end);
        }
    }
    candidates.sort_by(|a, b| b.len().cmp(&a.len()).then(a.start.cmp(&b.start)));
    let mut chosen: Vec<Range<usize>> = Vec::new();
    for candidate in candidates {
        if chosen.len() == MAX_LINES_PER_ASSET {
            break;
        }
        if chosen
            .iter()
            .all(|taken| candidate.end <= taken.start || taken.end <= candidate.start)
        {
            chosen.push(candidate);
        }
    }
    chosen
}

/// Neighbouring rungs see the same alignment; keeps the strongest view of
/// each. Returns the lines sorted by asset and window.
fn distinct(mut lines: Vec<Line>) -> Vec<Line> {
    lines.sort_by(|a, b| {
        (a.asset, a.window)
            .cmp(&(b.asset, b.window))
            .then(b.hits.cmp(&a.hits))
    });
    let mut kept: Vec<Line> = Vec::new();
    let mut group_start = 0;
    for line in lines {
        if kept
            .last()
            .is_none_or(|last| (last.asset, last.window) != (line.asset, line.window))
        {
            group_start = kept.len();
        }
        let centre = line.centre();
        let seen = kept[group_start..].iter().any(|other| {
            (other.reference_frame_at(centre) - line.reference_frame_at(centre)).abs()
                <= SAME_LINE_FRAMES
        });
        if !seen {
            kept.push(line);
        }
    }
    kept
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::search::test_audio;
    use crate::spectrogram::Signal;
    use crate::speed::SpeedRatio;

    /// A posting a query hash hit, as found in its list.
    #[derive(Clone, Copy)]
    struct Found {
        posting: Posting,
        anchor: u32,
    }

    fn hit(offset: f64) -> AlignedHit {
        AlignedHit {
            offset,
            query_frame: 0.0,
        }
    }

    /// A line in `window` (625 frames each) whose hits span 400 frames.
    pub fn line(window: u32, speed: f64, offset: f64, hits: u32) -> Line {
        let first = f64::from(window) * 625.0 + 100.0;
        Line {
            asset: AssetId(0),
            window,
            speed,
            playback: Playback::Turntable,
            offset,
            hits,
            first,
            last: first + 400.0,
        }
    }

    /// The lines of a window's hits as the search found them up to
    /// session 12: one comparison sort by asset and offset.
    fn lines_by_comparison_sort(
        hits: &[Found],
        points: &[Point],
        window: u32,
        rung: Rung,
    ) -> Vec<Line> {
        let mut found: Vec<(AssetId, f64, f64)> = hits
            .iter()
            .map(|hit| {
                let anchor = points[hit.anchor as usize];
                (
                    hit.posting.asset(),
                    f64::from(hit.posting.frame()) - anchor.frame,
                    anchor.frame / rung.speed().0,
                )
            })
            .collect();
        found.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.total_cmp(&b.1)));
        let mut lines = Vec::new();
        for same_asset in found.chunk_by(|a, b| a.0 == b.0) {
            let aligned: Vec<AlignedHit> = same_asset
                .iter()
                .map(|&(_, offset, query_frame)| AlignedHit {
                    offset,
                    query_frame,
                })
                .collect();
            for cluster in densest_clusters(&aligned) {
                lines.push(line_of(same_asset[0].0, &aligned[cluster], window, rung));
            }
        }
        lines
    }

    /// `count` hits of `assets` assets in a seeded random order, on 2,000
    /// anchors a quarter frame apart, so that equal offsets with different
    /// query frames meet at cluster edges. Every seventh asset also plays
    /// along a line.
    fn mixed_hits(assets: u32, count: usize, seed: u64) -> (Vec<Found>, Vec<Point>) {
        let mut state = seed;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let points: Vec<Point> = (0..2000)
            .map(|anchor| Point {
                frame: f64::from(anchor) * 0.25,
                bin: 100.0,
            })
            .collect();
        let hits = (0..count)
            .map(|_| {
                let asset = (next() % u64::from(assets)) as u32;
                let anchor = (next() % 2000) as u32;
                let frame = if asset.is_multiple_of(7) && next().is_multiple_of(2) {
                    points[anchor as usize].frame as u32 + 1000 + asset % 50 + (next() % 3) as u32
                } else {
                    (next() % 4000) as u32
                };
                Found {
                    posting: Posting::new(AssetId(asset), frame),
                    anchor,
                }
            })
            .collect();
        (hits, points)
    }

    /// The hits as posting lists, as the search finds them: anchor by
    /// anchor, each anchor's hits cut into lists of 1 to 60 postings ordered
    /// by asset (as the index holds them).
    fn as_lists(mut hits: Vec<Found>, seed: u64) -> Vec<(Vec<Posting>, u32)> {
        let mut state = seed;
        hits.sort_by_key(|hit| hit.anchor);
        let mut lists = Vec::new();
        for same_anchor in hits.chunk_by(|a, b| a.anchor == b.anchor) {
            let mut rest = same_anchor;
            while !rest.is_empty() {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let (list, after) = rest.split_at((1 + state % 60).min(rest.len() as u64) as usize);
                let mut postings: Vec<Posting> = list.iter().map(|hit| hit.posting).collect();
                postings.sort_by_key(|posting| posting.asset());
                lists.push((postings, list[0].anchor));
                rest = after;
            }
        }
        lists
    }

    #[test]
    fn grouping_by_asset_finds_the_lines_of_a_comparison_sort() {
        let rung = Rung::KeyLocked(SpeedRatio(1.012));
        let mut window_hits = WindowHits::new(9000);
        // A large window over three parts, the last one shorter, then a
        // smaller one in the same buffers, one with no hit in the middle
        // part, and one with every hit on the index's last asset.
        let (last_asset, points) = mixed_hits(1, 5_000, 2028);
        let (gap, gap_points) = mixed_hits(9000, 50_000, 2029);
        let middle = ASSETS_PER_PART as u32..2 * ASSETS_PER_PART as u32;
        let windows = [
            mixed_hits(9000, 400_000, 2026),
            mixed_hits(9000, 20_000, 2027),
            (
                gap.into_iter()
                    .filter(|hit| !middle.contains(&hit.posting.asset().0))
                    .collect(),
                gap_points,
            ),
            (
                last_asset
                    .into_iter()
                    .map(|hit| Found {
                        posting: Posting::new(AssetId(8999), hit.posting.frame()),
                        ..hit
                    })
                    .collect(),
                points,
            ),
        ];
        let mut windows: Vec<_> = windows
            .into_iter()
            .map(|(hits, points)| (as_lists(hits, 2030), points))
            .collect();
        // A hash whose posting list is empty.
        windows[1].0.insert(3, (Vec::new(), 17));
        for (window, (lists, points)) in (7..).zip(&windows) {
            let found: Vec<Found> = lists
                .iter()
                .flat_map(|(postings, anchor)| {
                    postings.iter().map(|&posting| Found {
                        posting,
                        anchor: *anchor,
                    })
                })
                .collect();
            let expected = lines_by_comparison_sort(&found, points, window, rung);
            assert!(!expected.is_empty());
            for (postings, anchor) in lists {
                window_hits.find(postings, *anchor);
            }
            window_hits.phases.find(points);

            assert!(
                window_hits.lines(window, rung, points) == expected,
                "window {window}"
            );
            assert!(window_hits.lists.is_empty());
            assert!(window_hits.ends.iter().all(|&end| end == 0));
        }
    }

    fn postings_of(assets: &[u32]) -> Vec<Posting> {
        assets
            .iter()
            .map(|&asset| Posting::new(AssetId(asset), asset % 7))
            .collect()
    }

    #[test]
    fn a_part_ends_where_a_binary_search_ends_it() {
        let mut state = 2031_u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        for _ in 0..2_000 {
            let len = (next() % 300) as usize;
            let spread = 1 + next() % 2_000;
            let mut assets: Vec<u32> = (0..len).map(|_| (next() % spread) as u32).collect();
            assets.sort_unstable();
            let postings = postings_of(&assets);
            for end in (0..=spread as usize + 1).step_by(1 + spread as usize / 97) {
                let expected =
                    postings.partition_point(|posting| (posting.asset().0 as usize) < end);

                assert_eq!(
                    front_partition(&postings, end),
                    expected,
                    "{assets:?} {end}"
                );
            }
        }
    }

    #[test]
    fn a_part_ends_at_the_start_at_the_end_and_past_it() {
        let postings = postings_of(&[3, 3, 5, 8, 8, 8, 13]);

        assert_eq!(front_partition(&[], 0), 0);
        assert_eq!(front_partition(&[], 10), 0);
        assert_eq!(front_partition(&postings, 0), 0);
        assert_eq!(front_partition(&postings, 3), 0);
        assert_eq!(front_partition(&postings, 4), 2);
        assert_eq!(front_partition(&postings, 8), 3);
        assert_eq!(front_partition(&postings, 13), 6);
        assert_eq!(front_partition(&postings, 14), 7);
        assert_eq!(front_partition(&postings, 1 << 16), 7);
        for len in 1..=17 {
            let same = postings_of(&vec![4; len]);

            assert_eq!(front_partition(&same, 4), 0, "{len}");
            assert_eq!(front_partition(&same, 5), len, "{len}");
        }
    }

    #[test]
    fn the_densest_offset_cluster_becomes_a_line() {
        let hits = [
            hit(-50.0),
            hit(10.0),
            hit(10.5),
            hit(11.0),
            hit(11.9),
            hit(30.0),
        ];

        assert_eq!(densest_clusters(&hits), vec![1..5]);
    }

    #[test]
    fn scattered_hits_form_no_line() {
        let hits = [hit(0.0), hit(5.0), hit(10.0), hit(15.0)];

        assert!(densest_clusters(&hits).is_empty());
    }

    /// Hits at the given reference frames and anchor frames, one anchor
    /// each, with the points and their phases.
    fn hits_at(frames: &[(u32, f64)]) -> (Vec<Found>, Vec<Point>, Phases) {
        let points: Vec<Point> = frames
            .iter()
            .map(|&(_, frame)| Point { frame, bin: 100.0 })
            .collect();
        let hits = frames
            .iter()
            .zip(0..)
            .map(|(&(reference, _), anchor)| Found {
                posting: Posting::new(AssetId(0), reference),
                anchor,
            })
            .collect();
        let mut phases = Phases::default();
        phases.find(&points);
        (hits, points, phases)
    }

    /// Hits whose offsets as computed are `offsets`, at reference frames
    /// near the middle of a posting's range.
    fn hits_with_offsets(offsets: &[f64]) -> (Vec<Found>, Vec<Point>, Phases) {
        let frames: Vec<(u32, f64)> = offsets
            .iter()
            .zip(0..)
            .map(|(&offset, i)| (30_000 + 7 * i, f64::from(30_000 + 7 * i) - offset))
            .collect();
        hits_at(&frames)
    }

    fn may_form_line(slots: &mut OffsetSlots, hits: &[Found], phases: &Phases) -> bool {
        assert!(phases.exact);
        let taken: Vec<Hit> = hits
            .iter()
            .map(|found| {
                Hit::taken(
                    found.posting,
                    found.anchor,
                    phases.of_points[found.anchor as usize],
                )
            })
            .collect();
        slots.may_form_line(&taken)
    }

    /// The hits as `lines` aligns them, sorted by offset.
    fn aligned(hits: &[Found], points: &[Point]) -> Vec<AlignedHit> {
        let mut aligned: Vec<AlignedHit> = hits
            .iter()
            .map(
                |found| hit(f64::from(found.posting.frame()) - points[found.anchor as usize].frame),
            )
            .collect();
        aligned.sort_unstable_by(|a, b| a.offset.total_cmp(&b.offset));
        aligned
    }

    /// The pre-filter up to session 40: slots of offsets in `f64`.
    fn may_form_line_in_floats(offsets: impl Iterator<Item = f64>) -> bool {
        let mut counts = vec![0_u8; OFFSET_SLOTS];
        for offset in offsets {
            let slot = (offset / OFFSET_BUCKET_FRAMES).floor() as i64 as usize % OFFSET_SLOTS;
            counts[slot] += 1;
            let here = counts[slot];
            let next = counts[(slot + 1) % OFFSET_SLOTS];
            let previous = counts[(slot + OFFSET_SLOTS - 1) % OFFSET_SLOTS];
            if usize::from(here + next.max(previous)) >= MIN_LINE_HITS {
                return true;
            }
        }
        false
    }

    /// The pre-filter up to session 42: each slot counted alone, its
    /// neighbours read apart.
    fn may_form_line_by_neighbours(hits: &[Hit]) -> bool {
        let mut counts = vec![0_u8; OFFSET_SLOTS];
        for &hit in hits {
            let slot = hit.slot();
            counts[slot] += 1;
            let here = counts[slot];
            let next = counts[(slot + 1) % OFFSET_SLOTS];
            let previous = counts[(slot + OFFSET_SLOTS - 1) % OFFSET_SLOTS];
            if usize::from(here + next.max(previous)) >= MIN_LINE_HITS {
                return true;
            }
        }
        false
    }

    #[test]
    fn pairs_of_slots_pass_the_assets_neighbouring_slots_passed() {
        let mut state = 2037_u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut slots = OffsetSlots::new();
        let (mut passed, mut failed) = (0, 0);
        for _ in 0..300_000 {
            // Offsets anywhere, or within a few slots of a point that is
            // often the slots' wrap (offsets 0 and 2^16 - 1 are slots 0
            // and 4095 at phase 0).
            let around = [
                0,
                1,
                4,
                16_380,
                16_383,
                65_532,
                65_535,
                (next() % 65_536) as u32,
            ][(next() % 8) as usize];
            let spread = [12, 40, 65_536][(next() % 3) as usize];
            let hits: Vec<Hit> = (0..=next() % 12)
                .map(|anchor| {
                    let offset = (around + (next() % spread) as u32) % 65_536;
                    Hit::taken(Posting::new(AssetId(3), offset), anchor as u32, 0)
                })
                .collect();

            let may = slots.may_form_line(&hits);

            assert_eq!(may, may_form_line_by_neighbours(&hits), "{around} {spread}");
            assert!(slots.pairs.iter().all(|&count| count == 0));
            if may {
                passed += 1;
            } else {
                failed += 1;
            }
        }
        assert!(passed > 50_000 && failed > 50_000, "{passed} {failed}");
    }

    #[test]
    fn a_line_across_the_slots_wrap_is_sorted() {
        let mut slots = OffsetSlots::new();
        // Slots 4095 and 0, in either order and with a third hit in each.
        for offsets in [
            [65_535, 0, 1],
            [0, 65_535, 65_534],
            [65_533, 65_532, 3],
            [2, 1, 65_534],
        ] {
            let hits: Vec<Hit> = (0..)
                .zip(offsets)
                .map(|(anchor, offset)| Hit::taken(Posting::new(AssetId(3), offset), anchor, 0))
                .collect();

            assert!(slots.may_form_line(&hits), "{offsets:?}");
            assert!(slots.pairs.iter().all(|&count| count == 0));
        }
        let apart: Vec<Hit> = (0..)
            .zip([65_535, 0, 8])
            .map(|(anchor, offset)| Hit::taken(Posting::new(AssetId(3), offset), anchor, 0))
            .collect();
        assert!(!slots.may_form_line(&apart));
        assert!(slots.pairs.iter().all(|&count| count == 0));
    }

    #[test]
    fn a_phase_is_the_frame_rounded_down_modulo_2_16() {
        let points: Vec<Point> = [
            0.0, 0.25, 0.999, 1.0, -0.25, -1.0, 65_535.75, 65_536.5, 200_001.9,
        ]
        .into_iter()
        .map(|frame| Point { frame, bin: 0.0 })
        .collect();
        let mut phases = Phases::default();

        phases.find(&points);

        assert!(phases.exact);
        assert_eq!(
            phases.of_points,
            [0, 0, 0, 1, 65_535, 65_535, 65_535, 0, 3_393]
        );
        phases.find(&[Point {
            frame: MAX_PHASE_FRAMES,
            bin: 0.0,
        }]);
        assert!(!phases.exact);
        phases.find(&[Point {
            frame: f64::NAN,
            bin: 0.0,
        }]);
        assert!(!phases.exact);
    }

    #[test]
    fn a_taken_hit_gives_back_its_posting() {
        let phases = [0, 1, 4, 30_000, u16::MAX];
        for frame in [0, 1, 3, 4, 29_999, 30_000, 65_534, MAX_FRAMES - 1] {
            for (anchor, &phase) in (0..).zip(&phases) {
                let posting = Posting::new(AssetId(4_321), frame);
                let hit = Hit::taken(posting, anchor, phase);

                assert_eq!(hit.asset(), AssetId(4_321));
                assert_eq!(hit.anchor, anchor);
                assert_eq!(hit.frame(&phases), frame, "{frame} {phase}");
                assert_eq!(
                    hit.slot(),
                    (i64::from(frame) - i64::from(phase))
                        .div_euclid(4)
                        .rem_euclid(4_096) as usize,
                    "{frame} {phase}"
                );
            }
        }
    }

    #[test]
    fn hits_with_no_three_in_neighbouring_slots_cannot_form_a_line() {
        let mut slots = OffsetSlots::new();
        let (hits, points, phases) = hits_with_offsets(&[15.0, 0.0, 10.0, 5.0, -7.0]);

        assert!(densest_clusters(&aligned(&hits, &points)).is_empty());
        assert!(!may_form_line(&mut slots, &hits, &phases));
        assert!(slots.pairs.iter().all(|&count| count == 0));
    }

    #[test]
    fn a_line_is_sorted_wherever_its_offsets_fall() {
        let mut slots = OffsetSlots::new();
        // Bucket edges, negative offsets, the slots' wrap (bucket -1 is slot
        // 4095, bucket 4096 slot 0), equal offsets and a span of exactly 2
        // frames, from anchors with and without a fraction.
        for first in [
            0.0, 2.0, 3.75, -0.5, -4.0, -6.25, 16_382.5, -16_385.0, 0.999,
        ] {
            let (hits, points, phases) = hits_with_offsets(&[
                first + 300.0,
                first + 2.0,
                first - 50.0,
                first + 0.75,
                first,
                first + 2.0,
            ]);

            assert_eq!(
                densest_clusters(&aligned(&hits, &points)).len(),
                1,
                "{first}"
            );
            assert!(may_form_line(&mut slots, &hits, &phases), "{first}");
            assert!(slots.pairs.iter().all(|&count| count == 0));
        }
    }

    #[test]
    fn a_line_spanning_more_than_two_frames_once_rounded_is_sorted() {
        // Offsets 4, 3 and 2 - 2^-52: 4 - (2 - 2^-52) rounds to 2, so these
        // hits are a cluster; buckets of 2 frames would put them in three.
        let (hits, points, phases) = hits_at(&[(4, 0.0), (3, 0.0), (2, f64::EPSILON)]);
        let aligned = aligned(&hits, &points);

        assert_eq!(aligned[0].offset, 2.0 - f64::EPSILON);
        assert_eq!(densest_clusters(&aligned), vec![0..3]);
        assert!(may_form_line(&mut OffsetSlots::new(), &hits, &phases));
    }

    #[test]
    fn every_set_of_hits_with_a_line_is_sorted() {
        let mut state = 2029_u64;
        let mut next = move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut slots = OffsetSlots::new();
        let (mut with_lines, mut skipped, mut as_in_floats) = (0, 0, 0);
        let sets = 200_000;
        for _ in 0..sets {
            // Offsets over spans from a few frames to a whole track, from
            // reference frames anywhere in a posting's range and anchor
            // frames at a rung's speed: whole frames at speed 1, frames
            // with a fraction at the other two.
            let spread = [6, 12, 40, 70_000][(next() % 4) as usize];
            let base = (next() % 200_000) as f64 - 100_000.0;
            let speed = [1.0, 1.0123, 0.9375][(next() % 3) as usize];
            let frames: Vec<(u32, f64)> = (0..3 + next() % 10)
                .map(|_| {
                    let reference = (next() % u64::from(MAX_FRAMES)) as u32;
                    let offset = base + (next() % spread) as f64 - (next() % 64) as f64 / 64.0;
                    let anchor = ((f64::from(reference) - offset) / speed).round() * speed;
                    (reference, anchor)
                })
                .collect();
            let (hits, points, phases) = hits_at(&frames);
            let may = may_form_line(&mut slots, &hits, &phases);
            assert!(slots.pairs.iter().all(|&count| count == 0));
            let aligned = aligned(&hits, &points);
            if may == may_form_line_in_floats(aligned.iter().map(|hit| hit.offset)) {
                as_in_floats += 1;
            }
            if !densest_clusters(&aligned).is_empty() {
                assert!(may, "{frames:?}");
                with_lines += 1;
            } else if !may {
                skipped += 1;
            }
        }
        assert!(
            with_lines > 20_000 && skipped > 20_000 && as_in_floats > sets * 9 / 10,
            "{with_lines} {skipped} {as_in_floats}"
        );
    }

    #[test]
    fn points_too_far_out_for_phases_leave_every_asset_to_be_sorted() {
        let rung = Rung::Turntable(SpeedRatio(1.0));
        let (hits, mut points) = mixed_hits(300, 20_000, 2032);
        points[1999].frame = MAX_PHASE_FRAMES + 0.5;
        let expected = lines_by_comparison_sort(&hits, &points, 3, rung);
        let lists = as_lists(hits, 2033);
        let mut window_hits = WindowHits::new(300);
        for (postings, anchor) in &lists {
            window_hits.find(postings, *anchor);
        }
        window_hits.phases.find(&points);

        assert!(!window_hits.phases.exact);
        assert!(!expected.is_empty());
        assert!(window_hits.lines(3, rung, &points) == expected);
    }

    #[test]
    fn neighbouring_rungs_seeing_one_alignment_are_merged() {
        let strong = line(4, 1.0, 1000.0, 40);
        let neighbour = line(4, 1.004, 1000.0 - 0.004 * 2700.0, 12);
        let elsewhere = line(4, 1.0, 3000.0, 10);

        let kept = distinct(vec![neighbour, strong.clone(), elsewhere.clone()]);

        assert_eq!(kept, [strong, elsewhere]);
    }

    #[test]
    fn searching_in_blocks_finds_the_lines_of_searching_the_whole_query() {
        let profile = Profile::CURRENT;
        let rate = profile.sample_rate;
        let original = test_audio::track(80.0, rate);
        let index = Index::build(&[test_audio::record("original.wav", &original)]).unwrap();
        // 45 s from 20.3 s in, 0.6% fast: windows hold partial alignments.
        let played = test_audio::played(&original, 1.006, 20.3, 45.0, rate);
        let peaks = test_audio::record("played.wav", &played).peaks;
        let mut ladder: Vec<Rung> = [1.0, 1.004, 1.008]
            .into_iter()
            .flat_map(|speed| {
                [
                    Rung::Turntable(SpeedRatio(speed)),
                    Rung::KeyLocked(SpeedRatio(speed)),
                ]
            })
            .collect();

        ladder.extend([
            Rung::KeyLocked(SpeedRatio(1.004)),
            Rung::KeyLocked(SpeedRatio(1.008)),
        ]);
        for query in [Query::Samples(Signal::Mono(&played)), Query::Peaks(&peaks)] {
            let whole = distinct(
                ladder
                    .iter()
                    .flat_map(|&rung| {
                        in_windows(
                            &index,
                            query,
                            &profile,
                            rung,
                            0..u32::MAX,
                            &mut WindowHits::new(index.assets().len()),
                            &mut KeyLockPeaks::default(),
                        )
                    })
                    .collect(),
            );
            assert!(whole.len() >= 4, "{}", whole.len());
            for block_windows in [1, 2, 4] {
                let blocks = in_blocks(
                    &index,
                    query,
                    &profile,
                    &ladder,
                    3,
                    block_windows,
                    |_, _| {},
                );
                assert!(blocks == whole, "blocks of {block_windows} windows");
            }
        }
    }
}

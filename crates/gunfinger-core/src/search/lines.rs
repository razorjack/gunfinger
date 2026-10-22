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
use crate::index::{AssetId, Index, Posting};
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
/// A window's hits are grouped by asset this many assets at a time, so that
/// a worker's hits stay in its share of the cache with two workers per core
/// (experiments 0096, 0169).
const ASSETS_PER_PART: usize = 1024;
/// Offsets are counted in buckets this wide (`OffsetSlots`), so that a
/// line's offsets lie in one bucket or two neighbouring ones.
const OFFSET_BUCKET_FRAMES: f64 = 2.0 * LINE_SPAN_FRAMES;
/// The buckets are folded into this many slots.
const OFFSET_SLOTS: usize = 4096;
// A slot's count stops at `MIN_LINE_HITS`, and two slots' counts are added.
const _: () = assert!(2 * MIN_LINE_HITS <= u8::MAX as usize);
// A slot is kept as a `u16`.
const _: () = assert!(OFFSET_SLOTS <= 1 << 16);

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

/// A posting a query hash hit, as found: 8 bytes. A window hits up to 2
/// million postings at 33,596 assets (experiment 0086), held one part at a
/// time.
#[derive(Clone, Copy)]
struct Hit {
    posting: Posting,
    /// The anchor's position among the rung's points.
    anchor: u32,
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
                let offsets = same_asset.iter().map(|&hit| offset_of(hit, points));
                if same_asset.len() < MIN_LINE_HITS || !self.slots.may_form_line(offsets) {
                    continue;
                }
                self.aligned.clear();
                self.aligned
                    .extend(same_asset.iter().map(|&hit| AlignedHit {
                        offset: offset_of(hit, points),
                        query_frame: points[hit.anchor as usize].frame / speed.0,
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
    /// found in.
    ///
    /// Grouping all of a window's hits by asset at once wrote each to one of
    /// as many places as there are assets, nearly every write a cache miss
    /// at 33,596 assets; a part's hits fit in the cache (experiment 0096).
    fn take_part(&mut self, end: usize) {
        self.part.clear();
        for (postings, anchor) in &mut self.lists {
            let taken = front_partition(postings, end);
            let (part, rest) = postings.split_at(taken);
            self.part.extend(part.iter().map(|&posting| Hit {
                posting,
                anchor: *anchor,
            }));
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
            ends[hit.posting.asset().0 as usize - first + 1] += 1;
        }
        for asset in 1..ends.len() {
            ends[asset] += ends[asset - 1];
        }
        if self.grouped.len() < self.part.len() {
            self.grouped.resize(self.part.len(), self.part[0]);
        }
        for &hit in &self.part {
            let slot = &mut ends[hit.posting.asset().0 as usize - first];
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
    // `postings[known]` is before `end`; the boundary lies after it and at
    // or before `beyond`.
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

/// Reference frame minus the anchor's frame at the rung's speed.
fn offset_of(hit: Hit, points: &[Point]) -> f64 {
    f64::from(hit.posting.frame()) - points[hit.anchor as usize].frame
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

/// Counts of one asset's offsets in buckets of `OFFSET_BUCKET_FRAMES`
/// folded into `OFFSET_SLOTS` slots, all zero between assets.
struct OffsetSlots {
    counts: Box<[u8; OFFSET_SLOTS]>,
    /// The slots counted for the asset, to clear afterwards.
    touched: Vec<u16>,
}

impl OffsetSlots {
    fn new() -> OffsetSlots {
        OffsetSlots {
            counts: Box::new([0; OFFSET_SLOTS]),
            touched: Vec::new(),
        }
    }

    /// Whether one asset's hits, given by their offsets in any order, may
    /// hold a cluster for `densest_clusters`: false only when no 3 of them
    /// lie in one slot or two neighbouring slots.
    ///
    /// A cluster's offsets differ by at most `LINE_SPAN_FRAMES` as computed,
    /// so by less than `OFFSET_BUCKET_FRAMES`, and their buckets by at most
    /// one. Folding buckets into slots keeps neighbours neighbours (slot
    /// `OFFSET_SLOTS - 1` neighbours slot 0) and only adds counts.
    fn may_form_line(&mut self, offsets: impl Iterator<Item = f64>) -> bool {
        let counts = &mut self.counts;
        let mut may = false;
        for offset in offsets {
            let slot = (offset / OFFSET_BUCKET_FRAMES).floor() as i64 as usize % OFFSET_SLOTS;
            self.touched.push(slot as u16);
            counts[slot] += 1;
            let here = counts[slot];
            let next = counts[(slot + 1) % OFFSET_SLOTS];
            let previous = counts[(slot + OFFSET_SLOTS - 1) % OFFSET_SLOTS];
            if usize::from(here + next.max(previous)) >= MIN_LINE_HITS {
                may = true;
                break;
            }
        }
        for &slot in &self.touched {
            counts[usize::from(slot)] = 0;
        }
        self.touched.clear();
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
    use crate::speed::SpeedRatio;

    fn hit(offset: f64) -> AlignedHit {
        AlignedHit {
            offset,
            query_frame: 0.0,
        }
    }

    fn offsets(hits: &[AlignedHit]) -> impl Iterator<Item = f64> + '_ {
        hits.iter().map(|hit| hit.offset)
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
        hits: &[Hit],
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
    fn mixed_hits(assets: u32, count: usize, seed: u64) -> (Vec<Hit>, Vec<Point>) {
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
                Hit {
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
    fn as_lists(mut hits: Vec<Hit>, seed: u64) -> Vec<(Vec<Posting>, u32)> {
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
                    .map(|hit| Hit {
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
            let found: Vec<Hit> = lists
                .iter()
                .flat_map(|(postings, anchor)| {
                    postings.iter().map(|&posting| Hit {
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

    #[test]
    fn hits_with_no_three_in_neighbouring_slots_cannot_form_a_line() {
        let mut slots = OffsetSlots::new();
        let scattered = [hit(15.0), hit(0.0), hit(10.0), hit(5.0), hit(-7.0)];

        assert!(!slots.may_form_line(offsets(&scattered)));
        assert!(slots.counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn a_line_is_sorted_wherever_its_offsets_fall() {
        let mut slots = OffsetSlots::new();
        // Bucket edges, negative offsets, the slots' wrap (bucket -1 is slot
        // 4095, bucket 4096 slot 0) and a span of exactly 2 frames.
        for first in [0.0, 2.0, 3.75, -0.5, -4.0, -6.25, 16_382.5, -16_385.0] {
            let hits = [
                hit(first + 300.0),
                hit(first + 2.0),
                hit(first - 50.0),
                hit(first + 0.75),
                hit(first),
            ];
            let mut sorted = hits.to_vec();
            sorted.sort_unstable_by(|a, b| a.offset.total_cmp(&b.offset));

            assert_eq!(densest_clusters(&sorted).len(), 1, "{first}");
            assert!(slots.may_form_line(offsets(&hits)), "{first}");
            assert!(slots.counts.iter().all(|&count| count == 0));
        }
    }

    #[test]
    fn a_line_spanning_more_than_two_frames_once_rounded_is_sorted() {
        // 4 - (2 - 2^-52) rounds to 2, so these hits are a cluster; buckets
        // of 2 frames would put them in three.
        let hits = [hit(4.0), hit(3.0), hit(2.0 - f64::EPSILON)];
        let mut sorted = hits.to_vec();
        sorted.reverse();

        assert_eq!(densest_clusters(&sorted), vec![0..3]);
        assert!(OffsetSlots::new().may_form_line(offsets(&hits)));
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
        let (mut with_lines, mut skipped) = (0, 0);
        for _ in 0..100_000 {
            // Whole reference frames minus anchor frames with a fraction,
            // over spans from a few frames to a whole track.
            let spread = [6, 12, 40, 70_000][(next() % 4) as usize];
            let base = (next() % 200_000) as f64 - 100_000.0;
            let mut hits: Vec<AlignedHit> = (0..3 + next() % 10)
                .map(|_| hit(base + (next() % spread) as f64 - (next() % 64) as f64 / 64.0))
                .collect();
            let may = slots.may_form_line(offsets(&hits));
            assert!(slots.counts.iter().all(|&count| count == 0));
            hits.sort_unstable_by(|a, b| a.offset.total_cmp(&b.offset));
            if !densest_clusters(&hits).is_empty() {
                assert!(
                    may,
                    "{:?}",
                    hits.iter().map(|hit| hit.offset).collect::<Vec<_>>()
                );
                with_lines += 1;
            } else if !may {
                skipped += 1;
            }
        }
        assert!(
            with_lines > 10_000 && skipped > 10_000,
            "{with_lines} {skipped}"
        );
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
        for query in [Query::Samples(&played), Query::Peaks(&peaks)] {
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

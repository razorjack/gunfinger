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
use crate::speed::{Playback, Rung, SpeedRatio};

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
/// Offsets are counted in buckets of 4 frames (`offset / 4`, floored), more
/// than `LINE_SPAN_FRAMES`, and the buckets folded into this many slots.
const OFFSET_SLOTS: usize = 4096;

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

/// A posting a query hash hit, as found: 8 bytes, as a window holds up
/// to 2 million at 33,596 assets (experiment 0086).
#[derive(Clone, Copy)]
struct Hit {
    posting: Posting,
    /// The anchor's position among the rung's points.
    anchor: u32,
}

/// A hit once grouped with the other hits of its asset.
#[derive(Clone, Copy, Default)]
struct AlignedHit {
    /// Reference frame minus the anchor's frame at the assumed speed.
    offset: f64,
    /// The anchor's frame at the query's own speed.
    query_frame: f64,
}

/// One worker's hits, kept for all its jobs: those of the window being
/// searched as they are found, and the same hits grouped by asset.
///
/// Each thread keeps its buffers for all its jobs. A window's hits reach
/// 200 MB at 26,000 assets, and buffers freed after every rung were kept by
/// the allocator: 15.7 GB instead of 5.0 GB for 10 minutes of query on 10
/// threads (experiment 0029).
struct WindowHits {
    found: Vec<Hit>,
    /// The first `found.len()` hold the window's hits, asset by asset.
    grouped: Vec<AlignedHit>,
    /// A counter for each asset of the index and one more, all zero
    /// between windows (see `group_by_asset`).
    ends: Vec<u32>,
    /// All zero between assets (see `may_form_line`).
    slot_counts: Box<[u8; OFFSET_SLOTS]>,
}

impl WindowHits {
    fn new(assets: usize) -> WindowHits {
        WindowHits {
            found: Vec::new(),
            grouped: Vec::new(),
            ends: vec![0; assets + 1],
            slot_counts: Box::new([0; OFFSET_SLOTS]),
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
    /// 2% of them to sort (experiment 0093).
    fn lines(&mut self, window: u32, rung: Rung, points: &[Point]) -> Vec<Line> {
        self.group_by_asset(points, rung.speed());
        let mut lines = Vec::new();
        let mut start = 0;
        for (asset, &end) in self.ends[..self.ends.len() - 1].iter().enumerate() {
            let same_asset = &mut self.grouped[start..end as usize];
            start = end as usize;
            if same_asset.len() < MIN_LINE_HITS || !may_form_line(same_asset, &mut self.slot_counts)
            {
                continue;
            }
            same_asset.sort_unstable_by(|a, b| a.offset.total_cmp(&b.offset));
            for cluster in densest_clusters(same_asset) {
                lines.push(line_of(
                    AssetId(asset as u32),
                    &same_asset[cluster],
                    window,
                    rung,
                ));
            }
        }
        self.ends.fill(0);
        self.found.clear();
        lines
    }

    /// A counting sort by asset: asset numbers are dense and below the
    /// index's count, so two passes over the hits group them, where a
    /// comparison sort took most of a search's time at 33,596 assets
    /// (experiments 0075, 0085, 0086). Leaves asset `a`'s hits in
    /// `grouped[ends[a - 1]..ends[a]]`.
    fn group_by_asset(&mut self, points: &[Point], speed: SpeedRatio) {
        let ends = &mut self.ends;
        for hit in &self.found {
            ends[hit.posting.asset().0 as usize + 1] += 1;
        }
        // Each asset's first slot.
        for asset in 1..ends.len() {
            ends[asset] += ends[asset - 1];
        }
        if self.grouped.len() < self.found.len() {
            self.grouped.resize(self.found.len(), AlignedHit::default());
        }
        for hit in &self.found {
            let anchor = points[hit.anchor as usize];
            let slot = &mut ends[hit.posting.asset().0 as usize];
            self.grouped[*slot as usize] = AlignedHit {
                offset: f64::from(hit.posting.frame()) - anchor.frame,
                query_frame: anchor.frame / speed.0,
            };
            *slot += 1;
        }
    }
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
fn in_windows(
    index: &Index,
    query: Query,
    profile: &Profile,
    rung: Rung,
    windows: Range<u32>,
    hits: &mut WindowHits,
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
            for &posting in index.scanned_postings(hash) {
                hits.found.push(Hit {
                    posting,
                    anchor: anchor as u32,
                });
            }
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

/// Whether one asset's hits, in any order, may hold a cluster for
/// `densest_clusters`: false only when no 3 of them lie in one slot or two
/// neighbouring slots. Leaves `counts` all zero.
///
/// A cluster's offsets differ by at most `LINE_SPAN_FRAMES` (2) as
/// computed, so by less than 4, and their buckets by at most one. Folding
/// buckets into slots keeps neighbours neighbours (slot `OFFSET_SLOTS - 1`
/// neighbours slot 0) and only adds counts.
fn may_form_line(hits: &[AlignedHit], counts: &mut [u8; OFFSET_SLOTS]) -> bool {
    let slot_of = |hit: &AlignedHit| (hit.offset * 0.25).floor() as i64 as usize % OFFSET_SLOTS;
    let mut may = false;
    let mut counted = 0;
    for hit in hits {
        let slot = slot_of(hit);
        counts[slot] += 1;
        counted += 1;
        let here = counts[slot];
        let next = counts[(slot + 1) % OFFSET_SLOTS];
        let previous = counts[(slot + OFFSET_SLOTS - 1) % OFFSET_SLOTS];
        if usize::from(here + next.max(previous)) >= MIN_LINE_HITS {
            may = true;
            break;
        }
    }
    for hit in &hits[..counted] {
        counts[slot_of(hit)] = 0;
    }
    may
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
                let frame = if asset % 7 == 0 && next() % 2 == 0 {
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

    #[test]
    fn grouping_by_asset_finds_the_lines_of_a_comparison_sort() {
        let rung = Rung::KeyLocked(SpeedRatio(1.012));
        let mut window_hits = WindowHits::new(3000);
        // A large window, then a smaller one in the same buffers, then one
        // with every hit on the index's last asset.
        let (last_asset, points) = mixed_hits(1, 5_000, 2028);
        let windows = [
            mixed_hits(3000, 400_000, 2026),
            mixed_hits(3000, 20_000, 2027),
            (
                last_asset
                    .into_iter()
                    .map(|hit| Hit {
                        posting: Posting::new(AssetId(2999), hit.posting.frame()),
                        ..hit
                    })
                    .collect(),
                points,
            ),
        ];
        for (window, (hits, points)) in (7..).zip(windows) {
            let expected = lines_by_comparison_sort(&hits, &points, window, rung);
            assert!(!expected.is_empty());
            window_hits.found.extend(hits);

            assert!(
                window_hits.lines(window, rung, &points) == expected,
                "window {window}"
            );
            assert!(window_hits.found.is_empty());
            assert!(window_hits.ends.iter().all(|&end| end == 0));
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
    fn hits_that_cannot_form_a_line_are_not_sorted() {
        let mut counts = Box::new([0; OFFSET_SLOTS]);
        let scattered = [hit(15.0), hit(0.0), hit(10.0), hit(5.0), hit(-7.0)];

        assert!(!may_form_line(&scattered, &mut counts));
        assert!(counts.iter().all(|&count| count == 0));
    }

    #[test]
    fn a_line_is_sorted_wherever_its_offsets_fall() {
        let mut counts = Box::new([0; OFFSET_SLOTS]);
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
            assert!(may_form_line(&hits, &mut counts), "{first}");
            assert!(counts.iter().all(|&count| count == 0));
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
        assert!(may_form_line(&hits, &mut Box::new([0; OFFSET_SLOTS])));
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
        let mut counts = Box::new([0; OFFSET_SLOTS]);
        let (mut with_lines, mut skipped) = (0, 0);
        for _ in 0..100_000 {
            // Whole reference frames minus anchor frames with a fraction,
            // over spans from a few frames to a whole track.
            let spread = [6, 12, 40, 70_000][(next() % 4) as usize];
            let base = (next() % 200_000) as f64 - 100_000.0;
            let mut hits: Vec<AlignedHit> = (0..3 + next() % 10)
                .map(|_| hit(base + (next() % spread) as f64 - (next() % 64) as f64 / 64.0))
                .collect();
            let may = may_form_line(&hits, &mut counts);
            assert!(counts.iter().all(|&count| count == 0));
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

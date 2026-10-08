//! Lines: hits of one asset, within one window of query time, whose offsets
//! agree. This is Wang 2003's offset histogram (§2.3), found by sorting each
//! asset's hits by offset, with the rung's speed as the line's slope.

use std::mem;
use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use super::Query;
use crate::hash::{MAX_DELTA_FRAMES, for_each_pair};
use crate::index::{AssetId, Index};
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

struct Hit {
    asset: AssetId,
    /// Reference frame minus the anchor's frame at the assumed speed.
    offset: f64,
    query_frame: f64,
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
///
/// Each thread keeps one buffer of hits for all its jobs. A window's hits
/// reach 200 MB at 26,000 assets, and buffers freed after every rung were
/// kept by the allocator: 15.7 GB instead of 5.0 GB for 10 minutes of
/// query on 10 threads (experiment 0029).
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
    let work: Vec<(u32, usize)> = (0..blocks)
        .flat_map(|block| (0..ladder.len()).map(move |rung| (block, rung)))
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
    map_in_order_with(&work, jobs, Vec::new, |hits, &(block, rung)| {
        let first = block * block_windows;
        // The last block takes every window left, however long the query.
        let end = if block + 1 == blocks {
            u32::MAX
        } else {
            first + block_windows
        };
        let lines = in_windows(index, query, profile, ladder[rung], first..end, hits);
        let complete = {
            let mut block = lock(&unmerged[block as usize]);
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
        progress(finished.fetch_add(1, Ordering::Relaxed) + 1, work.len());
    });
    let mut lines = merged.into_inner().unwrap_or_else(PoisonError::into_inner);
    // Blocks finish in any order. Their windows differ, so a stable sort
    // gives the order merging all lines at once gives.
    lines.sort_by_key(|line| (line.asset, line.window));
    lines
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
/// speed and collects the lines each window's hits form. `hits` is an empty
/// buffer, left empty.
fn in_windows(
    index: &Index,
    query: Query,
    profile: &Profile,
    rung: Rung,
    windows: Range<u32>,
    hits: &mut Vec<Hit>,
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
    );
    let mut lines = Vec::new();
    let mut window = 0;
    for_each_pair(&points, |hash, anchor| {
        let anchor = points[anchor];
        let query_frame = anchor.frame / speed.0;
        let anchor_window = (query_frame / window_frames) as u32;
        // Anchors are ordered by frame to within a frame, so a window only
        // ever moves forward. Anchors of earlier STFT frames lie earlier, so
        // the points before `points` cannot move it.
        if anchor_window > window {
            if windows.contains(&window) {
                lines.extend(lines_in_window(hits, window, rung));
            }
            window = anchor_window;
        }
        if windows.contains(&window) {
            for posting in index.scanned_postings(hash) {
                hits.push(Hit {
                    asset: posting.asset(),
                    offset: f64::from(posting.frame()) - anchor.frame,
                    query_frame,
                });
            }
        }
    });
    if windows.contains(&window) {
        lines.extend(lines_in_window(hits, window, rung));
    }
    lines
}

/// Groups one window's hits by asset and offset, turns the densest offset
/// clusters into lines, and empties `hits` for the next window.
fn lines_in_window(hits: &mut Vec<Hit>, window: u32, rung: Rung) -> Vec<Line> {
    hits.sort_unstable_by(|a, b| a.asset.cmp(&b.asset).then(a.offset.total_cmp(&b.offset)));
    let mut lines = Vec::new();
    for same_asset in hits.chunk_by(|a, b| a.asset == b.asset) {
        for cluster in densest_clusters(same_asset) {
            let members = &same_asset[cluster];
            let query_frames = members.iter().map(|hit| hit.query_frame);
            lines.push(Line {
                asset: members[0].asset,
                window,
                speed: rung.speed().0,
                playback: rung.playback(),
                offset: members.iter().map(|hit| hit.offset).sum::<f64>() / members.len() as f64,
                hits: members.len() as u32,
                first: query_frames.clone().fold(f64::INFINITY, f64::min),
                last: query_frames.fold(f64::NEG_INFINITY, f64::max),
            });
        }
    }
    hits.clear();
    lines
}

/// Index ranges of the densest non-overlapping offset clusters in hits
/// sorted by offset: for each hit, the hits within `LINE_SPAN_FRAMES` after
/// it form a candidate; the largest candidates win.
fn densest_clusters(hits: &[Hit]) -> Vec<Range<usize>> {
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

    fn hit(offset: f64) -> Hit {
        Hit {
            asset: AssetId(0),
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
        let ladder: Vec<Rung> = [1.0, 1.004, 1.008]
            .into_iter()
            .flat_map(|speed| {
                [
                    Rung::Turntable(SpeedRatio(speed)),
                    Rung::KeyLocked(SpeedRatio(speed)),
                ]
            })
            .collect();

        for query in [Query::Samples(&played), Query::Peaks(&peaks)] {
            let whole = distinct(
                ladder
                    .iter()
                    .flat_map(|&rung| {
                        in_windows(&index, query, &profile, rung, 0..u32::MAX, &mut Vec::new())
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

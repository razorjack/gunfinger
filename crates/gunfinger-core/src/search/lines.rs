//! Lines: hits of one asset, within one window of query time, whose offsets
//! agree.

use std::ops::Range;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::hash::for_each_pair;
use crate::index::{AssetId, Index};
use crate::parallel::map_in_order;
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

/// The lines of every rung, one rung per worker thread. `progress` gets the
/// number of rungs finished after each.
pub(super) fn on_ladder(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize) + Sync,
) -> Vec<Line> {
    let finished = AtomicUsize::new(0);
    map_in_order(ladder, jobs, |&rung| {
        let lines = at_rung(index, samples, profile, rung);
        progress(finished.fetch_add(1, Ordering::Relaxed) + 1);
        lines
    })
    .into_iter()
    .flatten()
    .collect()
}

/// Looks up every query hash at one assumed speed and collects the lines
/// each window's hits form.
fn at_rung(index: &Index, samples: &[f32], profile: &Profile, rung: Rung) -> Vec<Line> {
    let points = rung.points(samples, profile);
    let speed = rung.speed();
    let window_frames = profile.frames(WINDOW_SECONDS);
    let mut lines = Vec::new();
    let mut hits = Vec::new();
    let mut window = 0;
    for_each_pair(&points, |hash, anchor| {
        let anchor = points[anchor];
        let query_frame = anchor.frame / speed.0;
        let anchor_window = (query_frame / window_frames) as u32;
        // Anchors are ordered by frame to within a frame, so a window only
        // ever moves forward.
        if anchor_window > window {
            lines.extend(lines_in_window(&mut hits, window, rung));
            window = anchor_window;
        }
        for posting in index.postings(hash) {
            hits.push(Hit {
                asset: posting.asset(),
                offset: f64::from(posting.frame()) - anchor.frame,
                query_frame,
            });
        }
    });
    lines.extend(lines_in_window(&mut hits, window, rung));
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
pub(super) fn distinct(mut lines: Vec<Line>) -> Vec<Line> {
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
}

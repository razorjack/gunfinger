//! Finding library assets in a query.
//!
//! Evidence for a track is a set of hash hits lying on one line,
//! `reference frame = speed * query frame + offset`, that persists over time:
//!
//! 1. On every rung of the speed ladder, each query hash hits the postings
//!    that share it. Within a window of query time, hits of one asset whose
//!    offsets agree to within a couple of frames form a line.
//! 2. Lines of one asset in successive windows that predict the same
//!    reference time are chained. A played record keeps its line for minutes;
//!    chance coincidences and briefly shared samples do not.

use crate::confidence::Evidence;
use crate::hash::for_each_pair;
use crate::index::{AssetId, Index};
use crate::parallel::map_in_order;
use crate::profile::Profile;
use crate::speed::{SpeedRatio, points_at_speed};

/// Query time is cut into windows this long. Within one window a residual
/// speed error of 0.2% drifts the offset by about one frame.
const WINDOW_SECONDS: f64 = 10.0;
/// Hits whose offsets lie within this many frames belong to one line.
const LINE_SPAN_FRAMES: f64 = 2.0;
/// Fewer hits than this in a window are not worth keeping as a line.
const MIN_LINE_HITS: usize = 3;
/// A looped section matches its reference at several offsets; keep a few.
const MAX_LINES_PER_ASSET: usize = 3;
/// Lines from neighbouring rungs predicting reference times this close are
/// the same alignment.
const SAME_LINE_FRAMES: f64 = 4.0;
/// Windows without hits allowed inside a chain (a breakdown, a cut).
const MAX_GAP_WINDOWS: u32 = 2;
/// Successive lines are chained when they predict the same reference time to
/// within this many frames, plus this fraction of the time between them
/// (the speed of a line is only known to the nearest rung).
const LINK_TOLERANCE_FRAMES: f64 = 4.0;
const LINK_TOLERANCE_SLOPE: f64 = 0.004;

/// A library asset found playing in the query.
#[derive(Debug, Clone, PartialEq)]
pub struct Detection {
    pub asset: AssetId,
    /// Query time of the first and last aligned hit, in seconds.
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speed: SpeedRatio,
    pub evidence: Evidence,
}

/// Searches `samples` (mono, at the profile's rate) for the assets of
/// `index` at each assumed speed of `ladder` (normally `speed::ladder()`),
/// running rungs on `jobs` threads. Returns every chain of lines, strongest
/// first; the caller decides which are confident.
pub fn search(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[SpeedRatio],
    jobs: usize,
) -> Vec<Detection> {
    let lines = distinct_lines(lines_on_ladder(index, samples, profile, ladder, jobs));
    let mut detections = chain_lines(&lines, profile);
    detections.sort_by_key(|detection| std::cmp::Reverse(detection.evidence.hits));
    strongest_per_moment(detections)
}

/// A record plays once at any moment. Repeated sections make weaker,
/// parallel alignments of the same asset; drop every detection that
/// overlaps a stronger one of the same asset. `detections` are sorted
/// strongest first.
fn strongest_per_moment(detections: Vec<Detection>) -> Vec<Detection> {
    let mut kept: Vec<Detection> = Vec::new();
    for detection in detections {
        let overlaps_stronger = kept.iter().any(|stronger| {
            stronger.asset == detection.asset
                && stronger.start_seconds < detection.end_seconds
                && detection.start_seconds < stronger.end_seconds
        });
        if !overlaps_stronger {
            kept.push(detection);
        }
    }
    kept
}

/// Hits of one asset along one line within one window.
#[derive(Debug, Clone, PartialEq)]
struct Line {
    asset: AssetId,
    window: u32,
    speed: f64,
    offset: f64,
    hits: u32,
    /// Query frames of the first and last hit.
    first: f64,
    last: f64,
}

impl Line {
    fn reference_frame_at(&self, query_frame: f64) -> f64 {
        self.speed * query_frame + self.offset
    }

    fn centre(&self) -> f64 {
        (self.first + self.last) / 2.0
    }
}

struct Hit {
    asset: AssetId,
    /// Reference frame minus the anchor's frame at the assumed speed.
    offset: f64,
    query_frame: f64,
}

fn lines_on_ladder(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    rungs: &[SpeedRatio],
    jobs: usize,
) -> Vec<Line> {
    map_in_order(rungs, jobs, |&speed| {
        lines_at_speed(index, samples, profile, speed)
    })
    .into_iter()
    .flatten()
    .collect()
}

/// Looks up every query hash at one assumed speed and collects the lines
/// each window's hits form.
fn lines_at_speed(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    speed: SpeedRatio,
) -> Vec<Line> {
    let points = points_at_speed(samples, profile, speed);
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
            find_lines(&mut hits, window, speed, &mut lines);
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
    find_lines(&mut hits, window, speed, &mut lines);
    lines
}

/// Groups one window's hits by asset and offset, keeps the densest offset
/// clusters as lines, and empties `hits`.
fn find_lines(hits: &mut Vec<Hit>, window: u32, speed: SpeedRatio, lines: &mut Vec<Line>) {
    hits.sort_unstable_by(|a, b| a.asset.cmp(&b.asset).then(a.offset.total_cmp(&b.offset)));
    for same_asset in hits.chunk_by(|a, b| a.asset == b.asset) {
        for cluster in densest_clusters(same_asset) {
            let members = &same_asset[cluster];
            let count = members.len() as f64;
            lines.push(Line {
                asset: members[0].asset,
                window,
                speed: speed.0,
                offset: members.iter().map(|hit| hit.offset).sum::<f64>() / count,
                hits: members.len() as u32,
                first: members
                    .iter()
                    .map(|hit| hit.query_frame)
                    .fold(f64::INFINITY, f64::min),
                last: members
                    .iter()
                    .map(|hit| hit.query_frame)
                    .fold(f64::NEG_INFINITY, f64::max),
            });
        }
    }
    hits.clear();
}

/// Index ranges of the densest non-overlapping offset clusters in hits
/// sorted by offset: for each hit, the hits within `LINE_SPAN_FRAMES` after
/// it form a candidate; the largest candidates win.
fn densest_clusters(hits: &[Hit]) -> Vec<std::ops::Range<usize>> {
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
    let mut chosen: Vec<std::ops::Range<usize>> = Vec::new();
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

/// Neighbouring rungs see the same alignment; keep the strongest view of
/// each.
fn distinct_lines(mut lines: Vec<Line>) -> Vec<Line> {
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

/// Chains each asset's lines across windows into detections.
///
/// One pass of dynamic programming over the lines in window order finds,
/// for every line, the chain with the most hits that ends there. Chains are
/// then taken from the highest score down, each stopping where it would
/// reuse a line already taken. A single window of hits is never evidence of
/// a played record, so one-line chains are dropped.
fn chain_lines(lines: &[Line], profile: &Profile) -> Vec<Detection> {
    let mut detections = Vec::new();
    for same_asset in lines.chunk_by(|a, b| a.asset == b.asset) {
        let (score, previous) = best_chains(same_asset);
        let mut ends: Vec<usize> = (0..same_asset.len()).collect();
        ends.sort_by(|&a, &b| score[b].cmp(&score[a]));
        let mut taken = vec![false; same_asset.len()];
        for end in ends {
            let mut chain = Vec::new();
            let mut next = Some(end);
            while let Some(line) = next.filter(|&line| !taken[line]) {
                taken[line] = true;
                chain.push(&same_asset[line]);
                next = previous[line];
            }
            if chain.len() >= 2 {
                chain.reverse();
                detections.push(detection(&chain, profile));
            }
        }
    }
    detections
}

/// For each line (sorted by window), the hits of the best chain ending at it
/// and the line before it on that chain.
fn best_chains(lines: &[Line]) -> (Vec<u64>, Vec<Option<usize>>) {
    let mut score = vec![0_u64; lines.len()];
    let mut previous = vec![None; lines.len()];
    for current in 0..lines.len() {
        let hits = u64::from(lines[current].hits);
        score[current] = hits;
        for earlier in (0..current).rev() {
            if lines[current].window - lines[earlier].window > MAX_GAP_WINDOWS + 1 {
                break;
            }
            if links(&lines[earlier], &lines[current]) && score[earlier] + hits > score[current] {
                score[current] = score[earlier] + hits;
                previous[current] = Some(earlier);
            }
        }
    }
    (score, previous)
}

fn links(earlier: &Line, later: &Line) -> bool {
    if later.window <= earlier.window || later.window - earlier.window > MAX_GAP_WINDOWS + 1 {
        return false;
    }
    let at = later.centre();
    let gap = at - earlier.centre();
    let disagreement = (earlier.reference_frame_at(at) - later.reference_frame_at(at)).abs();
    disagreement <= LINK_TOLERANCE_FRAMES + LINK_TOLERANCE_SLOPE * gap.abs()
}

fn detection(chain: &[&Line], profile: &Profile) -> Detection {
    let first = chain
        .iter()
        .map(|line| line.first)
        .fold(f64::INFINITY, f64::min);
    let last = chain
        .iter()
        .map(|line| line.last)
        .fold(f64::NEG_INFINITY, f64::max);
    Detection {
        asset: chain[0].asset,
        start_seconds: profile.seconds(first),
        end_seconds: profile.seconds(last),
        speed: SpeedRatio(chain_speed(chain)),
        evidence: Evidence {
            windows: chain.len() as u32,
            hits: chain.iter().map(|line| line.hits).sum(),
        },
    }
}

/// The slope of the chain: a hit-weighted least-squares fit of reference
/// time against query time through the lines' centres, which resolves the
/// speed far more finely than the ladder step. A short chain falls back to
/// the hit-weighted mean of its rungs.
fn chain_speed(chain: &[&Line]) -> f64 {
    const MIN_FIT_FRAMES: f64 = 1000.0;
    let weight = |line: &Line| f64::from(line.hits);
    let total: f64 = chain.iter().map(|line| weight(line)).sum();
    let mean_rung = chain
        .iter()
        .map(|line| weight(line) * line.speed)
        .sum::<f64>()
        / total;
    let span = chain.last().map_or(0.0, |line| line.centre()) - chain[0].centre();
    if chain.len() < 3 || span < MIN_FIT_FRAMES {
        return mean_rung;
    }
    let mean_query = chain
        .iter()
        .map(|line| weight(line) * line.centre())
        .sum::<f64>()
        / total;
    let mean_reference = chain
        .iter()
        .map(|line| weight(line) * line.reference_frame_at(line.centre()))
        .sum::<f64>()
        / total;
    let mut covariance = 0.0;
    let mut variance = 0.0;
    for line in chain {
        let dq = line.centre() - mean_query;
        covariance += weight(line) * dq * (line.reference_frame_at(line.centre()) - mean_reference);
        variance += weight(line) * dq * dq;
    }
    covariance / variance
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(asset: u32, offset: f64) -> Hit {
        Hit {
            asset: AssetId(asset),
            offset,
            query_frame: 0.0,
        }
    }

    fn line(window: u32, speed: f64, offset: f64, hits: u32) -> Line {
        let first = f64::from(window) * 625.0 + 100.0;
        Line {
            asset: AssetId(0),
            window,
            speed,
            offset,
            hits,
            first,
            last: first + 400.0,
        }
    }

    #[test]
    fn the_densest_offset_cluster_becomes_a_line() {
        let hits = [
            hit(0, -50.0),
            hit(0, 10.0),
            hit(0, 10.5),
            hit(0, 11.0),
            hit(0, 11.9),
            hit(0, 30.0),
        ];

        let clusters = densest_clusters(&hits);

        assert_eq!(clusters, vec![1..5]);
    }

    #[test]
    fn scattered_hits_form_no_line() {
        let hits = [hit(0, 0.0), hit(0, 5.0), hit(0, 10.0), hit(0, 15.0)];

        assert!(densest_clusters(&hits).is_empty());
    }

    #[test]
    fn lines_on_one_alignment_chain_and_others_do_not() {
        // Reference frame = 1.02 * query frame + 5000 throughout.
        let lines = [
            line(0, 1.02, 5000.0, 20),
            line(1, 1.02, 5000.0, 25),
            line(1, 1.02, 9000.0, 4), // a loop elsewhere in the record
            line(3, 1.024, 5000.0 - 0.004 * 2200.0, 30), // next rung, after a gap
            line(9, 1.02, 5000.0, 30), // too long a gap
        ];

        let (score, previous) = best_chains(&lines);

        assert_eq!(score, [20, 45, 4, 75, 30]);
        assert_eq!(previous, [None, Some(0), None, Some(1), None]);
    }

    #[test]
    fn neighbouring_rungs_seeing_one_alignment_are_merged() {
        let strong = line(4, 1.0, 1000.0, 40);
        let mut neighbour = line(4, 1.004, 1000.0 - 0.004 * 2700.0, 12);
        neighbour.first = strong.first;
        neighbour.last = strong.last;
        let elsewhere = line(4, 1.0, 3000.0, 10);

        let kept = distinct_lines(vec![neighbour, strong.clone(), elsewhere.clone()]);

        assert_eq!(kept, [strong, elsewhere]);
    }

    #[test]
    fn a_long_chain_measures_speed_between_rungs() {
        // True speed 1.031, seen on rungs 1.028 and 1.032.
        let true_line = |window: u32, rung: f64, hits: u32| {
            let mut line = line(window, rung, 0.0, hits);
            let centre = line.centre();
            line.offset = 1.031 * centre + 777.0 - rung * centre;
            line
        };
        let lines = [
            true_line(0, 1.032, 10),
            true_line(2, 1.028, 10),
            true_line(5, 1.032, 10),
        ];
        let chain: Vec<&Line> = lines.iter().collect();

        assert!((chain_speed(&chain) - 1.031).abs() < 1e-9);
    }
}

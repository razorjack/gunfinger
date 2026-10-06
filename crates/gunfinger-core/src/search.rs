//! Finding library assets in a query.
//!
//! Evidence for a track is a set of hash hits lying on one line,
//! `reference frame = speed * query frame + offset`, that persists over time:
//!
//! 1. On every rung of the speed ladder, each query hash hits the postings
//!    that share it. Within a window of query time, hits of one asset whose
//!    offsets agree to within a couple of frames form a line (`lines`).
//! 2. Lines of one asset in successive windows that predict the same
//!    reference time are chained (`chains`). A played record keeps its line
//!    for minutes; chance coincidences and briefly shared samples do not.

mod chains;
mod lines;

use std::cmp::Reverse;

use crate::confidence::Evidence;
use crate::index::{AssetId, Index};
use crate::profile::Profile;
use crate::speed::{self, Playback, Rung, SpeedRatio};

/// A library asset found playing in the query.
#[derive(Debug, Clone, PartialEq)]
pub struct Detection {
    pub asset: AssetId,
    /// Query time of the first and last aligned hit, in seconds.
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Where those hits lie in the asset (reference time), in seconds.
    pub track_start_seconds: f64,
    pub track_end_seconds: f64,
    pub speed: SpeedRatio,
    /// For a key-locked detection, `speed` is the tempo; pitch is
    /// unchanged.
    pub playback: Playback,
    pub evidence: Evidence,
}

/// The matching settings as reports record them: lines, chains and the
/// ladder's speeds. The version changes when matching changes in a way
/// these numbers do not show.
pub fn design() -> String {
    format!(
        "lines-chains-v1 {} {} {}",
        lines::design(),
        chains::design(),
        speed::design()
    )
}

/// One window's hits of one asset along one line: the evidence chains join
/// into detections.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowLine {
    pub asset: AssetId,
    /// The window of query time, counted from the start of the query.
    pub window: u32,
    /// Query time of the first and last hit, in seconds.
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Where the first hit lies in the asset, in seconds.
    pub track_seconds: f64,
    /// The rung the line was found on.
    pub speed: SpeedRatio,
    pub playback: Playback,
    pub hits: u32,
}

/// A search with its evidence: every line found and, for each detection,
/// the lines its chain took.
pub struct Trace {
    /// As `search` returns them, strongest first.
    pub detections: Vec<Detection>,
    /// Sorted by asset and window.
    pub lines: Vec<WindowLine>,
    /// `chains[i]` holds the indexes in `lines` of `detections[i]`'s lines,
    /// in window order.
    pub chains: Vec<Vec<usize>>,
}

/// Window length in seconds, for showing where windows fall.
pub const WINDOW_SECONDS: f64 = lines::WINDOW_SECONDS;

/// Searches `samples` (mono, at the profile's rate) for the assets of
/// `index` on each rung of `ladder` (normally `speed::ladder()`),
/// running rungs on `jobs` threads. Returns the detections strongest first;
/// the caller decides which are confident.
pub fn search(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
) -> Vec<Detection> {
    search_with_progress(index, samples, profile, ladder, jobs, |_| {})
}

/// Like `search`, calling `progress` from the workers with the number of
/// rungs finished after each.
pub fn search_with_progress(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize) + Sync,
) -> Vec<Detection> {
    let (_, chained) = lines_and_detections(index, samples, profile, ladder, jobs, progress);
    chained
        .into_iter()
        .map(|(detection, _)| detection)
        .collect()
}

/// Like `search_with_progress`, keeping the evidence (`explain`).
pub fn trace_with_progress(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize) + Sync,
) -> Trace {
    let (lines, chained) = lines_and_detections(index, samples, profile, ladder, jobs, progress);
    let (detections, chains) = chained.into_iter().unzip();
    Trace {
        detections,
        lines: lines
            .iter()
            .map(|line| WindowLine {
                asset: line.asset,
                window: line.window,
                start_seconds: profile.seconds(line.first),
                end_seconds: profile.seconds(line.last),
                track_seconds: profile.seconds(line.reference_frame_at(line.first).max(0.0)),
                speed: SpeedRatio(line.speed),
                playback: line.playback,
                hits: line.hits,
            })
            .collect(),
        chains,
    }
}

/// The distinct lines of every rung, and the detections strongest first,
/// each with the indexes of its chain's lines.
fn lines_and_detections(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize) + Sync,
) -> (Vec<lines::Line>, Vec<(Detection, Vec<usize>)>) {
    let lines = lines::distinct(lines::on_ladder(
        index, samples, profile, ladder, jobs, progress,
    ));
    let mut detections = chains::detections(&lines, profile);
    detections.sort_by_key(|(detection, _)| Reverse(detection.evidence.hits));
    let detections = strongest_per_moment(detections);
    (lines, detections)
}

/// A record plays once at any moment. Repeated sections make weaker,
/// parallel alignments of the same asset; drop every detection that
/// overlaps a stronger one of the same asset. `detections` are sorted
/// strongest first.
fn strongest_per_moment(detections: Vec<(Detection, Vec<usize>)>) -> Vec<(Detection, Vec<usize>)> {
    let mut kept: Vec<(Detection, Vec<usize>)> = Vec::new();
    for (detection, chain) in detections {
        let overlaps_stronger = kept.iter().any(|(stronger, _)| {
            stronger.asset == detection.asset
                && stronger.start_seconds < detection.end_seconds
                && detection.start_seconds < stronger.end_seconds
        });
        if !overlaps_stronger {
            kept.push((detection, chain));
        }
    }
    kept
}

#[cfg(test)]
mod tests {
    use super::*;

    fn detection(asset: u32, start_seconds: f64, end_seconds: f64, hits: u32) -> Detection {
        Detection {
            asset: AssetId(asset),
            start_seconds,
            end_seconds,
            track_start_seconds: 0.0,
            track_end_seconds: end_seconds - start_seconds,
            speed: SpeedRatio(1.0),
            playback: Playback::Turntable,
            evidence: Evidence { windows: 5, hits },
        }
    }

    #[test]
    fn a_weaker_overlapping_detection_of_the_same_asset_is_dropped() {
        let strongest_first = vec![
            detection(1, 100.0, 400.0, 900),
            detection(1, 350.0, 500.0, 300), // a repeat of a section
            detection(2, 350.0, 500.0, 300), // another asset: kept
            detection(1, 600.0, 700.0, 200), // played again later: kept
        ];

        let kept = strongest_per_moment(
            strongest_first
                .into_iter()
                .map(|found| (found, Vec::new()))
                .collect(),
        );

        let spans: Vec<(u32, f64)> = kept
            .iter()
            .map(|(found, _)| (found.asset.0, found.start_seconds))
            .collect();
        assert_eq!(spans, [(1, 100.0), (2, 350.0), (1, 600.0)]);
    }
}

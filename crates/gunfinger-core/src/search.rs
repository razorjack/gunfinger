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
use crate::speed::SpeedRatio;

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
    pub evidence: Evidence,
}

/// Searches `samples` (mono, at the profile's rate) for the assets of
/// `index` at each assumed speed of `ladder` (normally `speed::ladder()`),
/// running rungs on `jobs` threads. Returns the detections strongest first;
/// the caller decides which are confident.
pub fn search(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[SpeedRatio],
    jobs: usize,
) -> Vec<Detection> {
    let lines = lines::distinct(lines::on_ladder(index, samples, profile, ladder, jobs));
    let mut detections = chains::detections(&lines, profile);
    detections.sort_by_key(|detection| Reverse(detection.evidence.hits));
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

        let kept = strongest_per_moment(strongest_first);

        let spans: Vec<(u32, f64)> = kept
            .iter()
            .map(|found| (found.asset.0, found.start_seconds))
            .collect();
        assert_eq!(spans, [(1, 100.0), (2, 350.0), (1, 600.0)]);
    }
}

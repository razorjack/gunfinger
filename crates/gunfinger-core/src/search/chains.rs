//! Chains: lines of one asset in successive windows that predict the same
//! reference time, joined into detections.

use super::Detection;
use super::lines::Line;
use crate::confidence::Evidence;
use crate::profile::Profile;
use crate::speed::SpeedRatio;

/// Windows without hits allowed inside a chain (a breakdown, a cut).
const MAX_GAP_WINDOWS: u32 = 2;
/// Successive lines are chained when they predict the same reference time to
/// within this many frames, plus this fraction of the time between them
/// (the speed of a line is only known to the nearest rung).
const LINK_TOLERANCE_FRAMES: f64 = 4.0;
const LINK_TOLERANCE_SLOPE: f64 = 0.004;
/// A chain shorter than this (16 s) gives its speed as the mean of its rungs
/// rather than by a fit.
const MIN_FIT_FRAMES: f64 = 1000.0;

/// Chains each asset's lines (sorted by asset and window) into detections.
///
/// One pass of dynamic programming over the lines in window order finds,
/// for every line, the chain with the most hits that ends there. Chains are
/// then taken from the highest score down, each stopping where it would
/// reuse a line already taken. A single window of hits is never evidence of
/// a played record, so one-line chains are dropped.
pub(super) fn detections(lines: &[Line], profile: &Profile) -> Vec<Detection> {
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
            // Lines further back are out of reach, which keeps this linear.
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
    let disagreement = (earlier.reference_frame_at(at) - later.reference_frame_at(at)).abs();
    disagreement <= LINK_TOLERANCE_FRAMES + LINK_TOLERANCE_SLOPE * (at - earlier.centre()).abs()
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
/// time against query time through the lines' centres,
/// `cov(query, reference) / var(query)`. It resolves the speed far more
/// finely than the ladder step. A short chain gives the mean of its rungs.
fn chain_speed(chain: &[&Line]) -> f64 {
    let span = chain[chain.len() - 1].centre() - chain[0].centre();
    if chain.len() < 3 || span < MIN_FIT_FRAMES {
        return weighted_mean(chain, |line| line.speed);
    }
    let reference = |line: &Line| line.reference_frame_at(line.centre());
    let mean_query = weighted_mean(chain, Line::centre);
    let mean_reference = weighted_mean(chain, reference);
    let covariance = weighted_mean(chain, |line| {
        (line.centre() - mean_query) * (reference(line) - mean_reference)
    });
    let variance = weighted_mean(chain, |line| (line.centre() - mean_query).powi(2));
    covariance / variance
}

/// The mean of `value` over the chain, each line weighted by its hits.
fn weighted_mean(chain: &[&Line], value: impl Fn(&Line) -> f64) -> f64 {
    let total: f64 = chain.iter().map(|line| f64::from(line.hits)).sum();
    chain
        .iter()
        .map(|line| f64::from(line.hits) * value(line))
        .sum::<f64>()
        / total
}

#[cfg(test)]
mod tests {
    use super::super::lines::tests::line;
    use super::*;

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
    fn single_windows_are_not_detections() {
        let lines = [line(0, 1.0, 0.0, 50), line(5, 1.0, 0.0, 50)];

        assert!(detections(&lines, &Profile::CURRENT).is_empty());
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

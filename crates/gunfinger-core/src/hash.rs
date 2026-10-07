//! Hashes from pairs of peaks (Wang 2003, "combinatorial hashing").
//!
//! Each peak is an anchor, paired with the next few peaks in a target zone
//! ahead of it. A pair is hashed from the anchor frequency, the frequency
//! difference and the time difference. Absolute frequency stays in the hash:
//! it makes hashes distinctive and posting lists short. The price is that a
//! speed change moves every hash, so a query is searched under a ladder of
//! assumed speeds (see `speed`).

use crate::peaks::Peak;

/// Pairs per anchor. Two keep every result of five at 41% of the postings
/// (experiment 0004).
pub const FAN_OUT: usize = 2;
/// Target zone: frames after the anchor, and bins above or below it.
pub const MAX_DELTA_FRAMES: f64 = 63.0;
const MAX_DELTA_BINS: f32 = 63.0;

const ANCHOR_BITS: u32 = 8;
const DELTA_BIN_BITS: u32 = 7;
const DELTA_FRAME_BITS: u32 = 6;
pub const HASH_BITS: u32 = ANCHOR_BITS + DELTA_BIN_BITS + DELTA_FRAME_BITS;

/// Anchor frequencies are quantised in whole bins up to the knee and in
/// relative steps above it. A residual speed error `e` moves a frequency `f`
/// by `e * f`: in linear bins high anchors break first, while relative steps
/// tolerate the same error at every frequency (experiment 0001). Below the
/// knee a relative step would be finer than a bin.
const ANCHOR_KNEE_BINS: f32 = 50.0;
const ANCHOR_RELATIVE_STEP: f32 = 0.02;

/// The hash design as reports record it: results made under another design
/// are not comparable. The version changes when hashing changes in a way
/// these numbers do not show.
pub fn design() -> String {
    format!(
        "pairs-v1 fan-out={FAN_OUT} zone={MAX_DELTA_FRAMES}x{MAX_DELTA_BINS} anchor={ANCHOR_BITS}bit/knee={ANCHOR_KNEE_BINS}/step={ANCHOR_RELATIVE_STEP} dbin={DELTA_BIN_BITS}bit dframe={DELTA_FRAME_BITS}bit"
    )
}

/// A packed pair hash: `anchor level | bin delta | frame delta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PairHash(pub u32);

/// A peak position in reference coordinates: frames and bins as they are at
/// the native speed of the recording.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub frame: f64,
    pub bin: f32,
}

impl From<&Peak> for Point {
    fn from(peak: &Peak) -> Point {
        Point {
            frame: peak.frame,
            bin: peak.bin,
        }
    }
}

/// Calls `emit(hash, anchor)` for every pair, where `anchor` indexes
/// `points`. Points must be ordered by frame (to within a frame).
pub fn for_each_pair(points: &[Point], mut emit: impl FnMut(PairHash, usize)) {
    for anchor in 0..points.len() {
        for target in targets(points, anchor).take(FAN_OUT) {
            emit(pair_hash(points[anchor], points[target]), anchor);
        }
    }
}

/// Indexes of the points in the target zone of `points[anchor]`, nearest
/// in time first.
pub fn targets(points: &[Point], anchor: usize) -> impl Iterator<Item = usize> + '_ {
    let a = points[anchor];
    (anchor + 1..points.len())
        .take_while(move |&target| (points[target].frame - a.frame).round() <= MAX_DELTA_FRAMES)
        .filter(move |&target| {
            let b = points[target];
            (b.frame - a.frame).round() >= 1.0 && (b.bin - a.bin).round().abs() <= MAX_DELTA_BINS
        })
}

/// The hash of an anchor and a point in its target zone.
pub fn pair_hash(anchor: Point, target: Point) -> PairHash {
    pack(
        anchor.bin,
        (target.bin - anchor.bin).round(),
        (target.frame - anchor.frame).round(),
    )
}

fn pack(anchor_bin: f32, delta_bins: f32, delta_frames: f64) -> PairHash {
    let anchor = (anchor_level(anchor_bin).round() as u32).min((1 << ANCHOR_BITS) - 1);
    let delta = (delta_bins + MAX_DELTA_BINS) as u32;
    PairHash(
        (anchor << (DELTA_BIN_BITS + DELTA_FRAME_BITS))
            | (delta << DELTA_FRAME_BITS)
            | delta_frames as u32,
    )
}

/// Whole bins below the knee; above it, the knee plus the number of
/// relative steps from the knee to `bin`. The two pieces meet at the knee,
/// where one relative step is one bin.
fn anchor_level(bin: f32) -> f32 {
    if bin < ANCHOR_KNEE_BINS {
        bin
    } else {
        ANCHOR_KNEE_BINS + (bin / ANCHOR_KNEE_BINS).ln() / ANCHOR_RELATIVE_STEP.ln_1p()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn point(frame: f64, bin: f32) -> Point {
        Point { frame, bin }
    }

    fn pairs(points: &[Point]) -> Vec<(PairHash, usize)> {
        let mut found = Vec::new();
        for_each_pair(points, |hash, anchor| found.push((hash, anchor)));
        found
    }

    #[test]
    fn fields_are_packed_without_overlap() {
        assert_eq!(pack(0.0, -MAX_DELTA_BINS, 0.0), PairHash(0));
        assert_eq!(pack(1.0, -MAX_DELTA_BINS, 0.0), PairHash(1 << 13));
        assert_eq!(pack(0.0, 1.0 - MAX_DELTA_BINS, 0.0), PairHash(1 << 6));
        assert_eq!(pack(0.0, -MAX_DELTA_BINS, 63.0), PairHash(63));
        assert!(pack(512.0, MAX_DELTA_BINS, 63.0).0 < 1 << HASH_BITS);
    }

    #[test]
    fn anchor_levels_are_continuous_at_the_knee_and_fit_their_bits() {
        let below = anchor_level(ANCHOR_KNEE_BINS - 0.001);
        assert!((below - anchor_level(ANCHOR_KNEE_BINS)).abs() < 0.01);
        assert!(anchor_level(512.0) < (1 << ANCHOR_BITS) as f32);
        // One level above the knee is a 2% step in frequency.
        assert!((anchor_level(400.0 * 1.02) - anchor_level(400.0) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn pairs_stay_inside_the_target_zone() {
        let points = [
            point(0.0, 100.0),
            point(0.0, 140.0),  // same frame: not a target
            point(10.0, 300.0), // too far in frequency
            point(20.0, 120.0),
            point(70.0, 100.0), // too far in time from the first two
        ];

        let found = pairs(&points);

        let anchors: Vec<usize> = found.iter().map(|&(_, anchor)| anchor).collect();
        assert_eq!(anchors, [0, 1, 3]);
        assert_eq!(found[0].0, pack(100.0, 20.0, 20.0));
    }

    #[test]
    fn fan_out_limits_pairs_per_anchor() {
        let points: Vec<Point> = (0..20)
            .map(|frame| point(f64::from(frame), 200.0))
            .collect();

        let from_first = pairs(&points)
            .into_iter()
            .filter(|&(_, anchor)| anchor == 0)
            .count();

        assert_eq!(from_first, FAN_OUT);
    }

    proptest! {
        #[test]
        fn anchor_levels_never_decrease_with_frequency(a in 0.0_f32..512.0, b in 0.0_f32..512.0) {
            let (low, high) = if a <= b { (a, b) } else { (b, a) };

            prop_assert!(anchor_level(low) <= anchor_level(high));
        }

        #[test]
        fn every_pair_fits_its_fields_and_the_fan_out(
            raw in prop::collection::vec((0.0_f64..2.0, 5.0_f32..500.0), 0..200)
        ) {
            let mut frame = 0.0;
            let points: Vec<Point> = raw
                .iter()
                .map(|&(step, bin)| {
                    frame += step;
                    point(frame, bin)
                })
                .collect();

            let mut per_anchor = vec![0; points.len()];
            for (hash, anchor) in pairs(&points) {
                per_anchor[anchor] += 1;
                let delta_frames = hash.0 & ((1 << DELTA_FRAME_BITS) - 1);
                let delta_bins = (hash.0 >> DELTA_FRAME_BITS) & ((1 << DELTA_BIN_BITS) - 1);
                prop_assert!(hash.0 < 1 << HASH_BITS);
                prop_assert!((1..=63).contains(&delta_frames));
                prop_assert!(delta_bins <= 2 * MAX_DELTA_BINS as u32);
            }
            prop_assert!(per_anchor.iter().all(|&count| count <= FAN_OUT));
        }
    }
}

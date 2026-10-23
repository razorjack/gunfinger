//! Pair hashes on the verifier's alignment: what exact quantisation loses
//! after fitting (the roadmap's "Evidence beyond exact pair hashes", step
//! 2; experiment 0149). The query's pairs are matched to the asset's own
//! pairs in the aligned reference passage, never looked up in the index:
//! first with the same hash, as the second pass counts its hits, then,
//! among the pairs left, with each quantised component of the hash at most
//! one step away, as Panako 2.0's near-exact hashing allows (Six 2021, §2).
//! A pair whose peaks fell into neighbouring bins or frames is recovered
//! that way; one whose peak is gone, or whose anchor now pairs with
//! another target, is not.
//!
//! Session 44 (experiment 0206) follows each reference pair to its peaks
//! (`split`): an endpoint gone, the target displaced beyond the query
//! anchor's first two targets, or the pair formed with a changed value. It
//! also pairs the query with more targets than the index does
//! (`QUERY_FAN_OUTS`), the record's pairs unchanged.

use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};

use gunfinger_core::hash::{FAN_OUT, HASH_BITS, PairHash, Point, pair_hash, targets};
use gunfinger_core::peaks::Peak;
use serde::{Deserialize, Serialize};

/// `hash::pack`'s layout, low bits first: the frame delta, the bin delta,
/// then the anchor level in the bits left.
const DELTA_FRAME_BITS: u32 = 6;
const DELTA_BIN_BITS: u32 = 7;
const ANCHOR_BITS: u32 = HASH_BITS - DELTA_BIN_BITS - DELTA_FRAME_BITS;

/// The query fan-outs counted beside the index's `FAN_OUT` (experiment
/// 0206).
pub const QUERY_FAN_OUTS: [usize; 3] = [3, 5, 8];

/// The query's pairs in the detection's span and the record's in the
/// passage, and how many of them correspond.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Pairs {
    pub query: u32,
    pub reference: u32,
    /// With the same hash.
    pub exact: u32,
    /// Among the pairs `exact` left, with a hash one step away in at least
    /// one component and at most one step in each.
    pub near: u32,
    /// Why each reference pair was matched or lost (at the alignment only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<Split>,
    /// The exact count with the query paired with more targets, one entry
    /// per `QUERY_FAN_OUTS`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fan_outs: Vec<FanOut>,
}

/// The query paired with each anchor's first `fan_out` targets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FanOut {
    pub fan_out: u32,
    /// The query's pairs at this fan-out, those at `FAN_OUT` included.
    pub query: u32,
    /// Reference pairs `exact` left that the extra pairs match with the
    /// same hash.
    pub extra: u32,
}

/// A pair hash, the reference frame of its anchor and the points it was
/// formed from: for a record's pair, where it lies (rounded, as the index
/// stores it) and its peaks; for a query pair, where the alignment expects
/// the reference anchor and the query's points.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pair {
    pub hash: PairHash,
    pub frame: f64,
    pub anchor: usize,
    pub target: usize,
}

/// The record's pairs, formed as the index forms them.
pub fn of_record(peaks: &[Peak]) -> Vec<Pair> {
    let points: Vec<Point> = peaks.iter().map(Point::from).collect();
    let mut pairs = Vec::new();
    for anchor in 0..points.len() {
        for target in targets(&points, anchor).take(FAN_OUT) {
            pairs.push(Pair {
                hash: pair_hash(points[anchor], points[target]),
                frame: points[anchor].frame.round(),
                anchor,
                target,
            });
        }
    }
    pairs
}

/// The pairs of `points` with each anchor's targets of the given ranks
/// (0 is the first target), in anchor order: `0..FAN_OUT` are the index's
/// pairs. `place(anchor)` gives the pair's frame, or `None` to leave the
/// anchor out.
pub fn with_targets(
    points: &[Point],
    ranks: Range<usize>,
    place: impl Fn(usize) -> Option<f64>,
) -> Vec<Pair> {
    let mut pairs = Vec::new();
    for anchor in 0..points.len() {
        let Some(frame) = place(anchor) else {
            continue;
        };
        for target in targets(points, anchor).skip(ranks.start).take(ranks.len()) {
            pairs.push(Pair {
                hash: pair_hash(points[anchor], points[target]),
                frame,
                anchor,
                target,
            });
        }
    }
    pairs
}

/// How a reference pair in the passage was matched.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Exact,
    Near,
    Unmatched,
}

/// The reference pairs in the passage, each with its status, and the
/// counts.
pub struct Matched {
    pub pairs: Pairs,
    pub reference: Vec<Pair>,
    pub status: Vec<Status>,
}

/// The query pairs, moved `shift` reference frames, against the record's
/// pairs whose anchor lies in `passage`. A correspondence's anchors lie
/// within `frames` of each other. `extra` holds, for each fan-out, the
/// query's pairs beyond `FAN_OUT`.
pub fn count(
    query: &[Pair],
    extra: &[(usize, Vec<Pair>)],
    record: &[Pair],
    passage: RangeInclusive<f64>,
    shift: f64,
    frames: f64,
) -> Matched {
    let reference: Vec<Pair> = record
        .iter()
        .filter(|pair| passage.contains(&pair.frame))
        .copied()
        .collect();
    let shifted = |pairs: &[Pair]| -> Vec<Pair> {
        pairs
            .iter()
            .map(|pair| Pair {
                frame: pair.frame + shift,
                ..*pair
            })
            .collect()
    };
    let query = shifted(query);
    let by_hash = by_hash(&reference);
    let mut status = vec![Status::Unmatched; reference.len()];
    let unmatched = take(&query, &by_hash, &reference, &mut status, frames);
    let after_exact = status.clone();
    take_near(&unmatched, &by_hash, &reference, &mut status, frames);
    let fan_outs = extra
        .iter()
        .map(|(fan_out, pairs)| {
            let mut status = after_exact.clone();
            let extra = shifted(pairs);
            let left = take(&extra, &by_hash, &reference, &mut status, frames);
            FanOut {
                fan_out: *fan_out as u32,
                query: (query.len() + extra.len()) as u32,
                extra: (extra.len() - left.len()) as u32,
            }
        })
        .collect();
    let counted = |wanted| status.iter().filter(|&&s| s == wanted).count() as u32;
    Matched {
        pairs: Pairs {
            query: query.len() as u32,
            reference: reference.len() as u32,
            exact: counted(Status::Exact),
            near: counted(Status::Near),
            split: None,
            fan_outs,
        },
        reference,
        status,
    }
}

fn by_hash(reference: &[Pair]) -> HashMap<PairHash, Vec<usize>> {
    let mut by_hash: HashMap<PairHash, Vec<usize>> = HashMap::new();
    for (at, pair) in reference.iter().enumerate() {
        by_hash.entry(pair.hash).or_default().push(at);
    }
    by_hash
}

/// Query pairs take, in order, the nearest reference pair with the same
/// hash that is still unmatched, and mark it exact. Returns the query
/// pairs that took none.
fn take<'q>(
    query: &'q [Pair],
    by_hash: &HashMap<PairHash, Vec<usize>>,
    reference: &[Pair],
    status: &mut [Status],
    frames: f64,
) -> Vec<&'q Pair> {
    let mut unmatched = Vec::new();
    for pair in query {
        let candidates = by_hash.get(&pair.hash).into_iter().flatten();
        match nearest(candidates, reference, status, pair.frame, frames) {
            Some(at) => status[at] = Status::Exact,
            None => unmatched.push(pair),
        }
    }
    unmatched
}

/// Then near correspondences among the pairs left, the same way.
fn take_near(
    unmatched: &[&Pair],
    by_hash: &HashMap<PairHash, Vec<usize>>,
    reference: &[Pair],
    status: &mut [Status],
    frames: f64,
) {
    for pair in unmatched {
        let candidates = neighbours(pair.hash).filter_map(|hash| by_hash.get(&hash));
        if let Some(at) = nearest(candidates.flatten(), reference, status, pair.frame, frames) {
            status[at] = Status::Near;
        }
    }
}

/// The reference pair among `candidates`, not yet matched, whose anchor
/// lies nearest `frame` and within `frames` of it; the earliest on a tie.
fn nearest<'a>(
    candidates: impl Iterator<Item = &'a usize>,
    reference: &[Pair],
    status: &[Status],
    frame: f64,
    frames: f64,
) -> Option<usize> {
    candidates
        .copied()
        .filter(|&at| status[at] == Status::Unmatched)
        .map(|at| (at, (reference[at].frame - frame).abs()))
        .filter(|&(_, distance)| distance <= frames)
        .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
        .map(|(at, _)| at)
}

/// The reference pairs of each status by kind (experiment 0206): the
/// pairs lost from the exact count are the `near` and `unmatched` ones.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Split {
    pub exact: Kinds,
    pub near: Kinds,
    pub unmatched: Kinds,
}

/// Reference pairs by what became of their two peaks in the query.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Kinds {
    pub anchor_gone: u32,
    pub target_gone: u32,
    pub both_gone: u32,
    /// Both found and the query pairs them (the target is among the query
    /// anchor's first `FAN_OUT`): with the same hash, one step away in each
    /// component at most, or further.
    pub paired_same: u32,
    pub paired_near: u32,
    pub paired_far: u32,
    /// Both found, the target in the anchor's zone but later in it: third,
    /// fourth or fifth, sixth to eighth, ninth or later.
    pub displaced_3: u32,
    pub displaced_4_5: u32,
    pub displaced_6_8: u32,
    pub displaced_later: u32,
    /// Both found, the target outside the anchor's zone.
    pub outside_zone: u32,
}

/// What became of a reference pair's peaks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    // In order of preference among candidate peaks: the kind nearest to a
    // pair the query forms with the same hash comes first.
    PairedSame,
    PairedNear,
    PairedFar,
    /// The target's rank among the anchor's targets, 1 for the first.
    Displaced(usize),
    OutsideZone,
    AnchorGone,
    TargetGone,
    BothGone,
}

impl Kinds {
    fn add(&mut self, kind: Kind) {
        let counter = match kind {
            Kind::AnchorGone => &mut self.anchor_gone,
            Kind::TargetGone => &mut self.target_gone,
            Kind::BothGone => &mut self.both_gone,
            Kind::PairedSame => &mut self.paired_same,
            Kind::PairedNear => &mut self.paired_near,
            Kind::PairedFar => &mut self.paired_far,
            Kind::Displaced(..=3) => &mut self.displaced_3,
            Kind::Displaced(4..=5) => &mut self.displaced_4_5,
            Kind::Displaced(6..=8) => &mut self.displaced_6_8,
            Kind::Displaced(_) => &mut self.displaced_later,
            Kind::OutsideZone => &mut self.outside_zone,
        };
        *counter += 1;
    }
}

/// Each reference pair's kind, by its status. `found(peak)` lists the
/// query points (indexes into `query`) that find the record's peak
/// `peak`, as the verifier finds peaks.
pub fn split(matched: &Matched, query: &[Point], found: impl Fn(usize) -> Vec<usize>) -> Split {
    let mut split = Split::default();
    for (pair, status) in matched.reference.iter().zip(&matched.status) {
        let kind = kind(pair, query, &found(pair.anchor), &found(pair.target));
        match status {
            Status::Exact => split.exact.add(kind),
            Status::Near => split.near.add(kind),
            Status::Unmatched => split.unmatched.add(kind),
        }
    }
    split
}

/// The best kind over the candidate query points for the reference pair's
/// anchor and target.
fn kind(pair: &Pair, query: &[Point], anchors: &[usize], targets_found: &[usize]) -> Kind {
    match (anchors.is_empty(), targets_found.is_empty()) {
        (true, true) => return Kind::BothGone,
        (true, false) => return Kind::AnchorGone,
        (false, true) => return Kind::TargetGone,
        (false, false) => {}
    }
    let mut best = Kind::OutsideZone;
    for &anchor in anchors {
        for &target in targets_found {
            let rank = targets(query, anchor).position(|at| at == target);
            let kind = match rank {
                None => Kind::OutsideZone,
                Some(rank) if rank < FAN_OUT => {
                    match value(pair_hash(query[anchor], query[target]), pair.hash) {
                        Value::Same => Kind::PairedSame,
                        Value::Near => Kind::PairedNear,
                        Value::Far => Kind::PairedFar,
                    }
                }
                Some(rank) => Kind::Displaced(rank + 1),
            };
            best = best.min(kind);
        }
    }
    best
}

enum Value {
    Same,
    Near,
    Far,
}

fn value(query: PairHash, reference: PairHash) -> Value {
    if query == reference {
        return Value::Same;
    }
    let steps = components(query)
        .into_iter()
        .zip(components(reference))
        .map(|(a, b)| a.abs_diff(b));
    if steps.max().unwrap_or(0) <= 1 {
        Value::Near
    } else {
        Value::Far
    }
}

/// A pair hash's quantised components: the anchor level, the bin delta
/// and the frame delta, each in steps of its quantisation.
fn components(hash: PairHash) -> [u32; 3] {
    let frame_mask = (1 << DELTA_FRAME_BITS) - 1;
    let bin_mask = (1 << DELTA_BIN_BITS) - 1;
    [
        hash.0 >> (DELTA_BIN_BITS + DELTA_FRAME_BITS),
        (hash.0 >> DELTA_FRAME_BITS) & bin_mask,
        hash.0 & frame_mask,
    ]
}

fn packed([anchor, delta_bins, delta_frames]: [u32; 3]) -> PairHash {
    PairHash(
        (anchor << (DELTA_BIN_BITS + DELTA_FRAME_BITS))
            | (delta_bins << DELTA_FRAME_BITS)
            | delta_frames,
    )
}

/// The 26 hashes whose components each differ from `hash`'s by at most one
/// step, not all by none; those outside a component's bits are left out.
fn neighbours(hash: PairHash) -> impl Iterator<Item = PairHash> {
    let centre = components(hash);
    let limits = [1 << ANCHOR_BITS, 1 << DELTA_BIN_BITS, 1 << DELTA_FRAME_BITS];
    let steps = [-1_i64, 0, 1];
    steps
        .into_iter()
        .flat_map(move |a| {
            steps
                .into_iter()
                .flat_map(move |b| steps.into_iter().map(move |c| [a, b, c]))
        })
        .filter(|step| *step != [0, 0, 0])
        .filter_map(move |step| {
            let mut moved = [0; 3];
            for component in 0..3 {
                let value = i64::from(centre[component]) + step[component];
                if value < 0 || value >= limits[component] {
                    return None;
                }
                moved[component] = value as u32;
            }
            Some(packed(moved))
        })
}

#[cfg(test)]
mod tests {
    use gunfinger_core::hash::for_each_pair;
    use gunfinger_core::index::Index;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::profile::Profile;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    fn point(frame: f64, bin: f32) -> Point {
        Point { frame, bin }
    }

    fn pair(hash: PairHash, frame: f64) -> Pair {
        Pair {
            hash,
            frame,
            anchor: 0,
            target: 0,
        }
    }

    /// Session 30's counts: no extra fan-outs.
    fn counts(
        query: &[Pair],
        record: &[Pair],
        passage: RangeInclusive<f64>,
        shift: f64,
        frames: f64,
    ) -> Pairs {
        count(query, &[], record, passage, shift, frames).pairs
    }

    #[test]
    fn components_follow_the_hashs_layout() {
        // Below the knee the anchor level is the anchor's bin.
        let hash = pair_hash(point(100.0, 30.0), point(112.0, 25.0));

        assert_eq!(components(hash), [30, 63 - 5, 12]);
        assert_eq!(packed(components(hash)), hash);
    }

    #[test]
    fn a_pair_one_step_away_in_each_component_counts_as_near_and_not_exact() {
        let anchor = point(1000.0, 30.0);
        let query = pair_hash(anchor, point(1012.0, 40.0));
        let moved = [
            pair_hash(point(1000.0, 31.0), point(1012.0, 41.0)), // anchor level
            pair_hash(anchor, point(1012.0, 41.0)),              // bin delta
            pair_hash(anchor, point(1013.0, 40.0)),              // frame delta
            pair_hash(point(1000.0, 29.0), point(1013.0, 40.0)), // all three
        ];

        for reference in moved {
            assert_ne!(reference, query);
            let counted = counts(
                &[pair(query, 1000.0)],
                &[pair(reference, 1000.0)],
                0.0..=2000.0,
                0.0,
                1.0,
            );
            assert_eq!((counted.exact, counted.near), (0, 1), "{reference:?}");
        }
        let two_steps = pair_hash(anchor, point(1014.0, 40.0));
        let counted = counts(
            &[pair(query, 1000.0)],
            &[pair(two_steps, 1000.0)],
            0.0..=2000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (0, 0));
    }

    #[test]
    fn neighbours_are_the_26_hashes_one_step_away() {
        let hash = packed([100, 63, 20]);

        let found: Vec<PairHash> = neighbours(hash).collect();

        assert_eq!(found.len(), 26);
        assert!(!found.contains(&hash));
        assert!(found.contains(&packed([101, 62, 21])));
        assert!(!found.contains(&packed([102, 63, 20])));
        // At the edge of a component's bits there is no step outwards.
        assert_eq!(neighbours(packed([0, 63, 20])).count(), 17);
    }

    #[test]
    fn no_correspondence_counts_twice() {
        let hash = packed([40, 63, 10]);
        let near = packed([41, 63, 10]);
        // Two query pairs, one reference pair with the same hash: one exact,
        // and the second query pair cannot take it again as near.
        let counted = counts(
            &[pair(hash, 500.0), pair(hash, 500.5)],
            &[pair(hash, 500.0)],
            0.0..=1000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (1, 0));
        // One query pair, an exact and a near reference pair: it counts once,
        // as exact.
        let counted = counts(
            &[pair(hash, 500.0)],
            &[pair(near, 500.0), pair(hash, 500.0)],
            0.0..=1000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (1, 0));
        // A reference pair taken as exact is not taken again as near.
        let counted = counts(
            &[pair(hash, 500.0), pair(near, 500.0)],
            &[pair(hash, 500.0)],
            0.0..=1000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (1, 0));
    }

    #[test]
    fn a_reference_pair_outside_the_passage_never_counts() {
        let hash = packed([40, 63, 10]);
        let near = packed([40, 64, 10]);
        let query = [pair(hash, 1000.0), pair(near, 1300.0)];
        let reference = [pair(hash, 1000.0), pair(near, 1300.0)];

        let inside = counts(&query, &reference, 900.0..=1400.0, 0.0, 1.0);
        let without_second = counts(&query, &reference, 900.0..=1200.0, 0.0, 1.0);
        let without_first = counts(&query, &reference, 1100.0..=1400.0, 0.0, 1.0);

        assert_eq!((inside.exact, inside.reference), (2, 2));
        assert_eq!(
            (
                without_second.exact,
                without_second.near,
                without_second.reference
            ),
            (1, 0, 1)
        );
        assert_eq!(
            (
                without_first.exact,
                without_first.near,
                without_first.reference
            ),
            (1, 0, 1)
        );
    }

    #[test]
    fn a_correspondence_needs_its_anchors_within_the_frames_and_takes_the_nearest() {
        let hash = packed([40, 63, 10]);
        let query = [pair(hash, 1000.0)];

        let far = counts(&query, &[pair(hash, 1001.5)], 0.0..=2000.0, 0.0, 1.0);
        let near_enough = counts(&query, &[pair(hash, 1001.0)], 0.0..=2000.0, 0.0, 1.0);
        let shifted = counts(&query, &[pair(hash, 1300.0)], 0.0..=2000.0, 300.0, 1.0);

        assert_eq!(far.exact, 0);
        assert_eq!(near_enough.exact, 1);
        assert_eq!(shifted.exact, 1);
        // The first query pair takes its nearest reference pair, 1000.1, not
        // the first listed, 1000.9, which leaves 1000.9 for the second.
        let two = counts(
            &[pair(hash, 1000.0), pair(hash, 1001.2)],
            &[pair(hash, 1000.9), pair(hash, 1000.1)],
            0.0..=2000.0,
            0.0,
            1.0,
        );
        assert_eq!(two.exact, 2);
    }

    #[test]
    fn extra_fan_outs_match_only_the_reference_pairs_the_exact_count_left() {
        let [first, second, third, fourth] = [10, 11, 12, 13].map(|level| packed([level, 63, 10]));
        let fourth_near = packed([14, 63, 10]);
        let reference = [
            pair(first, 1000.0),
            pair(second, 1000.0),
            pair(third, 1100.0),
            pair(fourth_near, 1200.0),
        ];
        // At fan-out 2 the first pair is exact and the fourth near.
        let query = [pair(first, 1000.0), pair(fourth, 1200.0)];
        let to_three = vec![
            pair(first, 1000.5),       // its reference pair is exact already
            pair(second, 1000.0),      // new
            pair(second, 1000.2),      // the same reference pair: once
            pair(third, 1050.0),       // too far from its reference pair
            pair(fourth_near, 1200.0), // near at fan-out 2, exact here
        ];
        let to_five = vec![pair(third, 1100.0)];

        let matched = count(
            &query,
            &[(3, to_three), (5, to_five)],
            &reference,
            0.0..=2000.0,
            0.0,
            1.0,
        );

        assert_eq!((matched.pairs.exact, matched.pairs.near), (1, 1));
        assert_eq!(
            matched.pairs.fan_outs,
            [
                FanOut {
                    fan_out: 3,
                    query: 7,
                    extra: 2,
                },
                // Each fan-out starts again from the exact count's pairs.
                FanOut {
                    fan_out: 5,
                    query: 3,
                    extra: 1,
                },
            ]
        );
        assert_eq!(
            matched.status,
            [
                Status::Exact,
                Status::Unmatched,
                Status::Unmatched,
                Status::Near
            ]
        );
    }

    #[test]
    fn pairs_with_later_targets_follow_the_index_pairs() {
        let points: Vec<Point> = (0..8)
            .map(|at| point(f64::from(at) * 2.0, 30.0 + f64::from(at) as f32))
            .collect();
        let everywhere = |anchor: usize| Some(anchor as f64);

        let first_two = with_targets(&points, 0..FAN_OUT, everywhere);
        let third_to_fifth = with_targets(&points, FAN_OUT..5, everywhere);
        let none_from_the_first = with_targets(&points, 0..FAN_OUT, |anchor| {
            (anchor > 0).then_some(anchor as f64)
        });

        let mut from_hashing = Vec::new();
        for_each_pair(&points, |hash, anchor| from_hashing.push((hash, anchor)));
        let ours: Vec<(PairHash, usize)> = first_two
            .iter()
            .map(|pair| (pair.hash, pair.anchor))
            .collect();
        assert_eq!(ours, from_hashing);
        let from_first: Vec<usize> = third_to_fifth
            .iter()
            .filter(|pair| pair.anchor == 0)
            .map(|pair| pair.target)
            .collect();
        assert_eq!(from_first, [3, 4, 5]);
        assert_eq!(third_to_fifth[0].hash, pair_hash(points[0], points[3]));
        assert!(none_from_the_first.iter().all(|pair| pair.anchor > 0));
        assert_eq!(none_from_the_first.len(), first_two.len() - FAN_OUT);
    }

    /// An anchor at frame 100 and twelve later points 4 frames apart, then
    /// one outside its zone.
    fn zone() -> Vec<Point> {
        let mut points = vec![point(100.0, 50.0)];
        points.extend((1..=12).map(|at| point(100.0 + 4.0 * f64::from(at), 55.0)));
        points.push(point(200.0, 50.0));
        points
    }

    fn reference_pair(hash: PairHash) -> Pair {
        Pair {
            hash,
            frame: 100.0,
            anchor: 0,
            target: 1,
        }
    }

    #[test]
    fn a_reference_pair_is_gone_paired_displaced_or_outside_the_zone() {
        let query = zone();
        let formed = reference_pair(pair_hash(query[0], query[1]));
        let a_bin_up = reference_pair(pair_hash(query[0], point(104.0, 56.0)));
        let two_frames_on = reference_pair(pair_hash(query[0], point(106.0, 55.0)));

        assert_eq!(kind(&formed, &query, &[], &[1]), Kind::AnchorGone);
        assert_eq!(kind(&formed, &query, &[0], &[]), Kind::TargetGone);
        assert_eq!(kind(&formed, &query, &[], &[]), Kind::BothGone);
        assert_eq!(kind(&formed, &query, &[0], &[1]), Kind::PairedSame);
        assert_eq!(kind(&formed, &query, &[0], &[2]), Kind::PairedFar);
        assert_eq!(kind(&a_bin_up, &query, &[0], &[1]), Kind::PairedNear);
        assert_eq!(kind(&two_frames_on, &query, &[0], &[1]), Kind::PairedFar);
        assert_eq!(kind(&formed, &query, &[0], &[3]), Kind::Displaced(3));
        assert_eq!(kind(&formed, &query, &[0], &[8]), Kind::Displaced(8));
        assert_eq!(kind(&formed, &query, &[0], &[13]), Kind::OutsideZone);
        // A target before its anchor is outside the zone too.
        assert_eq!(kind(&formed, &query, &[3], &[1]), Kind::OutsideZone);
        // Of several candidates, the kind nearest a formed pair counts.
        assert_eq!(kind(&formed, &query, &[0], &[13, 5, 1]), Kind::PairedSame);
        assert_eq!(kind(&formed, &query, &[0], &[9, 4]), Kind::Displaced(4));
    }

    #[test]
    fn the_split_counts_each_reference_pair_once_by_its_status() {
        let query = zone();
        let hash = pair_hash(query[0], query[1]);
        // Record peaks 0 and 1 are found at the query's points 0 and 1;
        // the record's peak n + 1 at the query's point n for the rest.
        let found = |peak: usize| match peak {
            0 | 1 => vec![peak],
            99 => Vec::new(),
            _ => vec![peak - 1],
        };
        let with_target = |target: usize| Pair {
            hash,
            frame: 100.0,
            anchor: 0,
            target,
        };
        let matched = Matched {
            pairs: counts(&[], &[], 0.0..=1.0, 0.0, 1.0),
            reference: vec![
                with_target(1),  // formed
                with_target(99), // its target gone
                with_target(4),  // third
                with_target(5),  // fourth
                with_target(7),  // sixth
                with_target(11), // tenth
                with_target(14), // outside the zone
            ],
            status: vec![
                Status::Exact,
                Status::Unmatched,
                Status::Unmatched,
                Status::Near,
                Status::Unmatched,
                Status::Unmatched,
                Status::Unmatched,
            ],
        };

        let split = split(&matched, &query, found);

        assert_eq!(
            split.exact,
            Kinds {
                paired_same: 1,
                ..Kinds::default()
            }
        );
        assert_eq!(
            split.near,
            Kinds {
                displaced_4_5: 1,
                ..Kinds::default()
            }
        );
        assert_eq!(
            split.unmatched,
            Kinds {
                target_gone: 1,
                displaced_3: 1,
                displaced_6_8: 1,
                displaced_later: 1,
                outside_zone: 1,
                ..Kinds::default()
            }
        );
    }

    #[test]
    fn a_records_pairs_are_the_indexs_postings() {
        let peaks: Vec<Peak> = [
            (0.0, 30.0),
            (5.2, 40.0),
            (9.0, 35.0),
            (40.6, 60.0),
            (80.0, 30.0),
        ]
        .iter()
        .map(|&(frame, bin)| Peak {
            frame,
            bin,
            magnitude: 10.0,
        })
        .collect();
        let record = PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: String::from("a.mp3"),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 10.0,
            },
            peaks: peaks.clone(),
        };
        let index = Index::build(&[record]).unwrap();

        let pairs = of_record(&peaks);

        let mut from_index: Vec<(PairHash, u32)> = (0..1 << HASH_BITS)
            .map(PairHash)
            .flat_map(|hash| {
                index
                    .postings(hash)
                    .iter()
                    .map(move |posting| (hash, posting.frame()))
            })
            .collect();
        let mut ours: Vec<(PairHash, u32)> = pairs
            .iter()
            .map(|pair| (pair.hash, pair.frame as u32))
            .collect();
        from_index.sort_unstable();
        ours.sort_unstable();
        // Two from the first anchor (the fan-out), two, one and one after it.
        assert_eq!(ours.len(), 6);
        assert_eq!(ours, from_index);
    }
}

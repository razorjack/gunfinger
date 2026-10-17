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

use std::collections::HashMap;
use std::ops::RangeInclusive;

use gunfinger_core::hash::{HASH_BITS, PairHash, Point, for_each_pair};
use gunfinger_core::peaks::Peak;
use serde::{Deserialize, Serialize};

/// `hash::pack`'s layout, low bits first: the frame delta, the bin delta,
/// then the anchor level in the bits left.
const DELTA_FRAME_BITS: u32 = 6;
const DELTA_BIN_BITS: u32 = 7;
const ANCHOR_BITS: u32 = HASH_BITS - DELTA_BIN_BITS - DELTA_FRAME_BITS;

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
}

/// A pair hash and the reference frame of its anchor: for a record's pair,
/// where it lies (rounded, as the index stores it); for a query pair, where
/// the alignment expects the reference anchor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pair {
    pub hash: PairHash,
    pub frame: f64,
}

/// The record's pairs, formed as the index forms them.
pub fn of_record(peaks: &[Peak]) -> Vec<Pair> {
    let points: Vec<Point> = peaks.iter().map(Point::from).collect();
    let mut pairs = Vec::new();
    for_each_pair(&points, |hash, anchor| {
        pairs.push(Pair {
            hash,
            frame: points[anchor].frame.round(),
        });
    });
    pairs
}

/// The query pairs, moved `shift` reference frames, against the record's
/// pairs whose anchor lies in `passage`. A correspondence's anchors lie
/// within `frames` of each other.
pub fn count(
    query: &[Pair],
    record: &[Pair],
    passage: RangeInclusive<f64>,
    shift: f64,
    frames: f64,
) -> Pairs {
    let reference: Vec<Pair> = record
        .iter()
        .filter(|pair| passage.contains(&pair.frame))
        .copied()
        .collect();
    let query: Vec<Pair> = query
        .iter()
        .map(|pair| Pair {
            hash: pair.hash,
            frame: pair.frame + shift,
        })
        .collect();
    let (exact, near) = correspondences(&query, &reference, frames);
    Pairs {
        query: query.len() as u32,
        reference: reference.len() as u32,
        exact,
        near,
    }
}

/// Exact correspondences, then near ones among the pairs left. Query pairs
/// take, in order, the nearest reference pair not yet taken; each pair
/// counts once.
fn correspondences(query: &[Pair], reference: &[Pair], frames: f64) -> (u32, u32) {
    let mut by_hash: HashMap<PairHash, Vec<usize>> = HashMap::new();
    for (at, pair) in reference.iter().enumerate() {
        by_hash.entry(pair.hash).or_default().push(at);
    }
    let mut taken = vec![false; reference.len()];
    let mut unmatched = Vec::new();
    let mut exact = 0;
    for pair in query {
        let candidates = by_hash.get(&pair.hash).into_iter().flatten();
        match nearest(candidates, reference, &taken, pair.frame, frames) {
            Some(at) => {
                taken[at] = true;
                exact += 1;
            }
            None => unmatched.push(pair),
        }
    }
    let mut near = 0;
    for pair in unmatched {
        let candidates = neighbours(pair.hash).filter_map(|hash| by_hash.get(&hash));
        if let Some(at) = nearest(candidates.flatten(), reference, &taken, pair.frame, frames) {
            taken[at] = true;
            near += 1;
        }
    }
    (exact, near)
}

/// The reference pair among `candidates`, not taken, whose anchor lies
/// nearest `frame` and within `frames` of it; the earliest on a tie.
fn nearest<'a>(
    candidates: impl Iterator<Item = &'a usize>,
    reference: &[Pair],
    taken: &[bool],
    frame: f64,
    frames: f64,
) -> Option<usize> {
    candidates
        .copied()
        .filter(|&at| !taken[at])
        .map(|at| (at, (reference[at].frame - frame).abs()))
        .filter(|&(_, distance)| distance <= frames)
        .min_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)))
        .map(|(at, _)| at)
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
    use gunfinger_core::hash::pair_hash;
    use gunfinger_core::index::Index;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::profile::Profile;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    fn point(frame: f64, bin: f32) -> Point {
        Point { frame, bin }
    }

    fn pair(hash: PairHash, frame: f64) -> Pair {
        Pair { hash, frame }
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
            let counted = count(
                &[pair(query, 1000.0)],
                &[pair(reference, 1000.0)],
                0.0..=2000.0,
                0.0,
                1.0,
            );
            assert_eq!((counted.exact, counted.near), (0, 1), "{reference:?}");
        }
        let two_steps = pair_hash(anchor, point(1014.0, 40.0));
        let counted = count(
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
        let counted = count(
            &[pair(hash, 500.0), pair(hash, 500.5)],
            &[pair(hash, 500.0)],
            0.0..=1000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (1, 0));
        // One query pair, an exact and a near reference pair: it counts once,
        // as exact.
        let counted = count(
            &[pair(hash, 500.0)],
            &[pair(near, 500.0), pair(hash, 500.0)],
            0.0..=1000.0,
            0.0,
            1.0,
        );
        assert_eq!((counted.exact, counted.near), (1, 0));
        // A reference pair taken as exact is not taken again as near.
        let counted = count(
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

        let inside = count(&query, &reference, 900.0..=1400.0, 0.0, 1.0);
        let without_second = count(&query, &reference, 900.0..=1200.0, 0.0, 1.0);
        let without_first = count(&query, &reference, 1100.0..=1400.0, 0.0, 1.0);

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

        let far = count(&query, &[pair(hash, 1001.5)], 0.0..=2000.0, 0.0, 1.0);
        let near_enough = count(&query, &[pair(hash, 1001.0)], 0.0..=2000.0, 0.0, 1.0);
        let shifted = count(&query, &[pair(hash, 1300.0)], 0.0..=2000.0, 300.0, 1.0);

        assert_eq!(far.exact, 0);
        assert_eq!(near_enough.exact, 1);
        assert_eq!(shifted.exact, 1);
        // The first query pair takes its nearest reference pair, 1000.1, not
        // the first listed, 1000.9, which leaves 1000.9 for the second.
        let two = count(
            &[pair(hash, 1000.0), pair(hash, 1001.2)],
            &[pair(hash, 1000.9), pair(hash, 1000.1)],
            0.0..=2000.0,
            0.0,
            1.0,
        );
        assert_eq!(two.exact, 2);
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

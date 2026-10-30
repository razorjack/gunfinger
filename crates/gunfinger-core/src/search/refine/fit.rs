//! Other alignments for the second pass's measurement than the chain's
//! speed (`SecondPassFit`), for measuring what that estimate costs
//! (experiments 0251-0253). Each starts from the measurement at the chain's
//! speed, on the same span and pivot.

use super::{Alignment, Candidate, Hit, Measured, SEARCH_FRAMES, drift, measure};
use crate::search::SecondPassFit;

/// Candidates with fewer hits at the chain's speed keep that measurement:
/// a third of the rule's 240, so that only a move of speed that tripled the
/// hits could make one confident.
pub const MIN_HITS_TO_FIT: u32 = 80;
/// The grid: speeds `GRID_STEP` apart, `GRID_STEPS` either side of the
/// chain's (±0.4%).
const GRID_STEPS: i32 = 20;
const GRID_STEP: f64 = 0.0002;
/// Hits are collected this far from a grid speed's alignment, so that its
/// offset can be fitted again before they are counted within
/// `SEARCH_FRAMES` of it.
const COLLECT_FRAMES: f64 = 2.0 * SEARCH_FRAMES;
/// The iterated fit: at most this many rounds, each within `FIT_REACH` of
/// the chain's speed, ending after a round that moved the speed less than
/// `MIN_ROUND_CHANGE`.
const MAX_ROUNDS: u32 = 2;
const FIT_REACH: f64 = 0.004;
const MIN_ROUND_CHANGE: f64 = 0.00005;

/// The alignment and measurement `fit` keeps for `candidate`, given its
/// measurement at the chain's alignment.
pub(super) fn refit(
    fit: SecondPassFit,
    candidate: &Candidate,
    chain: Alignment,
    at_chain_speed: Vec<Measured>,
) -> (Alignment, Vec<Measured>) {
    if total(&at_chain_speed) < MIN_HITS_TO_FIT {
        return (chain, at_chain_speed);
    }
    match fit {
        SecondPassFit::ChainSpeed => (chain, at_chain_speed),
        SecondPassFit::BestOfGrid => best_of_grid(candidate, chain, at_chain_speed),
        SecondPassFit::CrossFitted => cross_fitted(candidate, chain, &at_chain_speed),
        SecondPassFit::Iterated => iterated(candidate, chain, at_chain_speed),
    }
}

/// Which windows take part: all, or those of one parity.
#[derive(Clone, Copy)]
enum Windows {
    All,
    Parity(u32),
}

impl Windows {
    fn contains(self, window: u32) -> bool {
        match self {
            Windows::All => true,
            Windows::Parity(parity) => window % 2 == parity,
        }
    }
}

/// One speed of the grid: its alignment on the chain's pivot and the hits
/// collected within `COLLECT_FRAMES` of it.
struct GridSpeed {
    alignment: Alignment,
    hits: Vec<Hit>,
}

/// The grid's speeds, the chain's first, then outwards, the slower before
/// the faster: the order in which ties are settled.
fn grid(candidate: &Candidate, chain: Alignment) -> impl Iterator<Item = GridSpeed> {
    let steps = std::iter::once(0).chain((1..=GRID_STEPS).flat_map(|step| [-step, step]));
    steps.map(move |step| {
        let alignment = Alignment {
            speed: chain.speed * (1.0 + GRID_STEP * f64::from(step)),
            ..chain
        };
        GridSpeed {
            alignment,
            hits: candidate.hits_near(alignment, candidate.span.clone(), .., COLLECT_FRAMES),
        }
    })
}

impl GridSpeed {
    /// The alignment moved by the hit-weighted mean deviation of the
    /// densest runs of `windows`, and its hits on `windows`.
    fn fitted(&self, windows: Windows) -> (Alignment, Vec<Measured>) {
        let mut hits: Vec<Hit> = self
            .hits
            .iter()
            .filter(|hit| windows.contains(hit.window))
            .copied()
            .collect();
        let runs = measure(&mut hits, self.alignment);
        if runs.is_empty() {
            return (self.alignment, runs);
        }
        let shift = runs
            .iter()
            .map(|run| f64::from(run.hits) * run.deviation)
            .sum::<f64>()
            / f64::from(total(&runs));
        let shifted = Alignment {
            reference: self.alignment.reference + shift,
            ..self.alignment
        };
        (shifted, self.counted(shifted, windows))
    }

    /// The hits on `windows` within `SEARCH_FRAMES` of `shifted`, this
    /// speed's alignment moved along the reference.
    fn counted(&self, shifted: Alignment, windows: Windows) -> Vec<Measured> {
        let shift = shifted.reference - self.alignment.reference;
        let mut hits: Vec<Hit> = self
            .hits
            .iter()
            .filter(|hit| windows.contains(hit.window))
            .filter(|hit| (hit.deviation - shift).abs() <= SEARCH_FRAMES)
            .map(|hit| Hit {
                deviation: hit.deviation - shift,
                ..*hit
            })
            .collect();
        measure(&mut hits, shifted)
    }
}

/// The measurement with the most hits of the chain's and the grid's.
fn best_of_grid(
    candidate: &Candidate,
    chain: Alignment,
    at_chain_speed: Vec<Measured>,
) -> (Alignment, Vec<Measured>) {
    let mut best = (chain, at_chain_speed);
    for speed in grid(candidate, chain) {
        let fitted = speed.fitted(Windows::All);
        if total(&fitted.1) > total(&best.1) {
            best = fitted;
        }
    }
    best
}

/// For each parity of windows, the alignment with the most hits on it of
/// the chain's and the grid's, its hits counted on the other parity; the
/// two counts together, at the hit-weighted mean of their speeds.
fn cross_fitted(
    candidate: &Candidate,
    chain: Alignment,
    at_chain_speed: &[Measured],
) -> (Alignment, Vec<Measured>) {
    let speeds: Vec<GridSpeed> = grid(candidate, chain).collect();
    let mut measured = Vec::new();
    let mut weighted_speed = 0.0;
    for parity in [0, 1] {
        let (fitting, counting) = (Windows::Parity(parity), Windows::Parity(1 - parity));
        let on = |runs: &[Measured], windows: Windows| -> Vec<Measured> {
            runs.iter()
                .filter(|run| windows.contains(run.window))
                .cloned()
                .collect()
        };
        let mut chosen = (total(&on(at_chain_speed, fitting)), chain, None);
        for (index, speed) in speeds.iter().enumerate() {
            let (shifted, runs) = speed.fitted(fitting);
            if total(&runs) > chosen.0 {
                chosen = (total(&runs), shifted, Some(index));
            }
        }
        let counted = match chosen.2 {
            None => on(at_chain_speed, counting),
            Some(index) => speeds[index].counted(chosen.1, counting),
        };
        weighted_speed += f64::from(total(&counted)) * chosen.1.speed;
        measured.extend(counted);
    }
    measured.sort_by_key(|run| run.window);
    let speed = match total(&measured) {
        0 => chain.speed,
        hits => weighted_speed / f64::from(hits),
    };
    (Alignment { speed, ..chain }, measured)
}

/// From the chain's alignment, at most `MAX_ROUNDS` rounds of the drift of
/// the measured windows' deviations and a measurement there, while the
/// speed stays within `FIT_REACH` of the chain's; the last round's
/// measurement, more hits or fewer.
fn iterated(
    candidate: &Candidate,
    chain: Alignment,
    at_chain_speed: Vec<Measured>,
) -> (Alignment, Vec<Measured>) {
    let mut current = (chain, at_chain_speed);
    for _ in 0..MAX_ROUNDS {
        let runs: Vec<&Measured> = current.1.iter().collect();
        let Some(next) = drift(&runs, current.0) else {
            break;
        };
        if (next.speed / chain.speed - 1.0).abs() > FIT_REACH {
            break;
        }
        let change = (next.speed / current.0.speed - 1.0).abs();
        let mut hits = candidate.hits_near(next, candidate.span.clone(), .., SEARCH_FRAMES);
        current = (next, measure(&mut hits, next));
        if change < MIN_ROUND_CHANGE {
            break;
        }
    }
    current
}

fn total(runs: &[Measured]) -> u32 {
    runs.iter().map(|run| run.hits).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confidence::Evidence;
    use crate::index::{AssetId, Index};
    use crate::profile::Profile;
    use crate::search::Detection;
    use crate::search::test_audio::{played, record, track};
    use crate::spectrogram::Signal;
    use crate::speed::{Playback, SpeedRatio};

    /// A 40 s query playing a record at its own speed from 20 s, measured
    /// on an alignment `chain_speed` fast through its middle, as a chain
    /// whose speed is off would be.
    struct Play {
        index: Index,
        query: Vec<f32>,
        detection: Detection,
        chain: Alignment,
    }

    fn play(chain_speed: f64) -> Play {
        let profile = Profile::CURRENT;
        let rate = profile.sample_rate;
        let reference = track(90.0, rate);
        let middle = profile.frames(20.0);
        Play {
            index: Index::build(&[record("a.wav", &reference)]).unwrap(),
            query: played(&reference, 1.0, 20.0, 40.0, rate),
            detection: Detection {
                asset: AssetId(0),
                start_seconds: 0.0,
                end_seconds: 40.0,
                track_start_seconds: 20.0,
                track_end_seconds: 60.0,
                speed: SpeedRatio(chain_speed),
                playback: Playback::Turntable,
                evidence: Evidence::new(4, 400),
            },
            chain: Alignment {
                speed: chain_speed,
                query: middle,
                reference: middle + profile.frames(20.0),
                playback: Playback::Turntable,
            },
        }
    }

    impl Play {
        fn refit(&self, fit: SecondPassFit) -> (Alignment, Vec<Measured>) {
            let candidate = Candidate {
                index: &self.index,
                samples: Signal::Mono(&self.query),
                profile: &Profile::CURRENT,
                detection: &self.detection,
                span: 0..self.query.len(),
            };
            let mut hits =
                candidate.hits_near(self.chain, candidate.span.clone(), .., SEARCH_FRAMES);
            let at_chain_speed = measure(&mut hits, self.chain);
            refit(fit, &candidate, self.chain, at_chain_speed)
        }
    }

    #[test]
    fn each_fit_moves_a_speed_0_3_percent_off_towards_the_play_and_gains_hits() {
        let off = play(1.003);
        let (_, at_chain_speed) = off.refit(SecondPassFit::ChainSpeed);

        for fit in [
            SecondPassFit::BestOfGrid,
            SecondPassFit::CrossFitted,
            SecondPassFit::Iterated,
        ] {
            let (alignment, measured) = off.refit(fit);
            assert!(
                (alignment.speed - 1.0).abs() < 0.0006,
                "{fit:?}: {}",
                alignment.speed
            );
            assert!(
                total(&measured) > total(&at_chain_speed) * 5 / 4,
                "{fit:?}: {} after {}",
                total(&measured),
                total(&at_chain_speed)
            );
        }
    }

    #[test]
    fn at_the_plays_speed_the_grid_keeps_at_least_the_chains_hits_and_the_split_about_as_many() {
        let on = play(1.0);
        let (_, at_chain_speed) = on.refit(SecondPassFit::ChainSpeed);
        let (grid, best) = on.refit(SecondPassFit::BestOfGrid);
        let (_, split) = on.refit(SecondPassFit::CrossFitted);

        assert!(total(&best) >= total(&at_chain_speed));
        assert!((grid.speed - 1.0).abs() <= 0.0004, "{}", grid.speed);
        let ratio = f64::from(total(&split)) / f64::from(total(&at_chain_speed));
        assert!((0.9..1.1).contains(&ratio), "{ratio}");
        assert_eq!(split.len(), at_chain_speed.len());
    }

    #[test]
    fn the_split_counts_each_window_once_on_the_half_it_did_not_choose_on() {
        let (_, split) = play(1.003).refit(SecondPassFit::CrossFitted);

        let windows: Vec<u32> = split.iter().map(|run| run.window).collect();
        let mut sorted = windows.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(windows, sorted);
        assert!(windows.len() >= 3, "{windows:?}");
    }

    #[test]
    fn a_candidate_below_the_floor_keeps_its_measurement_at_the_chains_speed() {
        let off = play(1.003);
        let candidate = Candidate {
            index: &off.index,
            samples: Signal::Mono(&off.query),
            profile: &Profile::CURRENT,
            detection: &off.detection,
            span: 0..off.query.len(),
        };
        let mut hits = candidate.hits_near(off.chain, candidate.span.clone(), .., SEARCH_FRAMES);
        let mut weak = measure(&mut hits, off.chain);
        weak.truncate(1);
        weak[0].hits = MIN_HITS_TO_FIT - 1;

        for fit in [
            SecondPassFit::BestOfGrid,
            SecondPassFit::CrossFitted,
            SecondPassFit::Iterated,
        ] {
            let (alignment, measured) = refit(fit, &candidate, off.chain, weak.clone());
            assert_eq!(alignment.speed, 1.003);
            assert_eq!(total(&measured), MIN_HITS_TO_FIT - 1);
        }
    }
}

//! The second pass (opt-in): each candidate's span is analysed again with
//! one STFT at its fitted speed, and its hits are counted against that asset
//! alone, in every posting list (also those `Index::skipping_fullest` keeps
//! from the first pass).
//!
//! The ladder sees a play on the rung nearest its speed, up to half a step
//! away. A clean render 0.1-0.25% from its rung keeps two thirds of the
//! hashes it keeps at its own speed (experiment 0021). Chance alignments
//! have no speed to return to.
//!
//! The speed is fitted to the chain's lines on rungs next to its strongest
//! line's, so that a chance line linked from far away cannot pull it
//! (experiment 0020). Hits are counted per 10 s window as in the first pass:
//! the densest cluster of offsets near the alignment, when it has enough
//! hits.

use std::ops::Range;

use super::Detection;
use super::chains::chain_speed;
use super::lines::{LINE_SPAN_FRAMES, Line, MIN_LINE_HITS, WINDOW_SECONDS};
use crate::confidence::{Evidence, Pass};
use crate::hash::for_each_pair;
use crate::index::Index;
use crate::parallel::map_in_order;
use crate::profile::Profile;
use crate::speed::{self, Playback, Rung, SpeedRatio};

/// Candidates with fewer first-pass hits are left as they are: weak either
/// way, and most of a query's candidates.
const MIN_HITS_TO_REFINE: u32 = 10;
/// The span analysed reaches this far beyond the candidate's first and last
/// hit, where the ladder may have seen too few hits for a line.
const MARGIN_SECONDS: f64 = WINDOW_SECONDS;
/// Hits are looked for this far either side of the alignment, to follow a
/// play whose speed wanders.
const SEARCH_FRAMES: f64 = 24.0;
/// Lines on rungs this far from the strongest line's take part in the fit.
const RUNG_REACH: f64 = 1.5 * speed::STEP;

/// Measures each detection with at least `MIN_HITS_TO_REFINE` hits again at
/// its fitted speed; `chains[i]` holds the indexes in `lines` of
/// `detections[i]`'s lines. Refined evidence is `Pass::Fitted`; a candidate
/// with no window of hits there is dropped. Returns the detections in no
/// particular order.
pub(super) fn refine(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    lines: &[Line],
    chained: &[(Detection, Vec<usize>)],
    jobs: usize,
) -> Vec<Detection> {
    map_in_order(chained, jobs, |(detection, chain)| {
        if detection.evidence.hits < MIN_HITS_TO_REFINE {
            return Some(detection.clone());
        }
        let chain: Vec<&Line> = chain.iter().map(|&line| &lines[line]).collect();
        at_fitted_speed(index, samples, profile, detection, &chain)
    })
    .into_iter()
    .flatten()
    .collect()
}

/// One hit near the alignment: its window, its distance from the alignment
/// in reference frames, and its query frame.
struct Hit {
    window: u32,
    deviation: f64,
    query_frame: f64,
}

fn at_fitted_speed(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    detection: &Detection,
    chain: &[&Line],
) -> Option<Detection> {
    let playback = detection.playback;
    let strongest = chain
        .iter()
        .filter(|line| line.playback == playback)
        .max_by_key(|line| line.hits)?;
    let near: Vec<&Line> = chain
        .iter()
        .copied()
        .filter(|line| line.playback == playback)
        .filter(|line| (line.speed - strongest.speed).abs() <= RUNG_REACH)
        .collect();
    let speed = chain_speed(&near);
    let (anchor, anchor_reference) = (
        strongest.centre(),
        strongest.reference_frame_at(strongest.centre()),
    );
    let alignment = |query_frame: f64| anchor_reference + speed * (query_frame - anchor);

    let rate = f64::from(profile.sample_rate);
    let first_sample = ((detection.start_seconds - MARGIN_SECONDS).max(0.0) * rate) as usize;
    let last_sample =
        (((detection.end_seconds + MARGIN_SECONDS) * rate) as usize).min(samples.len());
    if first_sample >= last_sample {
        return None;
    }
    let rung = match playback {
        Playback::Turntable => Rung::Turntable(SpeedRatio(speed)),
        Playback::KeyLocked => Rung::KeyLocked(SpeedRatio(speed)),
    };
    let points = rung.points(&samples[first_sample..last_sample], profile);
    let start_frame = profile.frames(first_sample as f64 / rate);
    let window_frames = profile.frames(WINDOW_SECONDS);
    let mut hits = Vec::new();
    for_each_pair(&points, |hash, anchor_point| {
        let query_frame = start_frame + points[anchor_point].frame / speed;
        let expected = alignment(query_frame);
        for posting in index.postings(hash) {
            let deviation = f64::from(posting.frame()) - expected;
            if posting.asset() == detection.asset && deviation.abs() <= SEARCH_FRAMES {
                hits.push(Hit {
                    window: (query_frame / window_frames) as u32,
                    deviation,
                    query_frame,
                });
            }
        }
    });
    hits.sort_unstable_by(|a, b| {
        a.window
            .cmp(&b.window)
            .then(a.deviation.total_cmp(&b.deviation))
    });

    let lines: Vec<&[Hit]> = hits
        .chunk_by(|a, b| a.window == b.window)
        .map(|window| &window[densest(window)])
        .filter(|line| line.len() >= MIN_LINE_HITS)
        .collect();
    let (opening, closing) = (lines.first()?, lines.last()?);
    let first = opening
        .iter()
        .map(|hit| hit.query_frame)
        .fold(f64::INFINITY, f64::min);
    let last = closing
        .iter()
        .map(|hit| hit.query_frame)
        .fold(f64::NEG_INFINITY, f64::max);
    let reference = |line: &[Hit], query_frame: f64| {
        let deviation = line.iter().map(|hit| hit.deviation).sum::<f64>() / line.len() as f64;
        profile.seconds((alignment(query_frame) + deviation).max(0.0))
    };
    Some(Detection {
        asset: detection.asset,
        start_seconds: profile.seconds(first),
        end_seconds: profile.seconds(last),
        track_start_seconds: reference(opening, first),
        track_end_seconds: reference(closing, last),
        speed: SpeedRatio(speed),
        playback,
        evidence: Evidence {
            windows: lines.len() as u32,
            hits: lines.iter().map(|line| line.len() as u32).sum(),
            pass: Pass::Fitted,
        },
    })
}

/// The index range of the largest run of hits (sorted by deviation) whose
/// deviations lie within `LINE_SPAN_FRAMES`.
fn densest(hits: &[Hit]) -> Range<usize> {
    let mut best = 0..0;
    let mut end = 0;
    for start in 0..hits.len() {
        end = end.max(start);
        while end < hits.len() && hits[end].deviation - hits[start].deviation <= LINE_SPAN_FRAMES {
            end += 1;
        }
        if end - start > best.len() {
            best = start..end;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use crate::index::Index;
    use crate::profile::Profile;
    use crate::search::test_audio::{played, record, track};
    use crate::search::{search, search_twice};
    use crate::speed::{Rung, SpeedRatio};

    #[test]
    fn the_second_pass_finds_the_speed_between_rungs_and_more_hits() {
        let profile = Profile::CURRENT;
        let reference = track(90.0, profile.sample_rate);
        let index = Index::build(&[record("a.wav", &reference)]).unwrap();
        let query = played(&reference, 1.0018, 20.0, 40.0, profile.sample_rate);
        let ladder: Vec<Rung> = [0.996, 1.0, 1.004]
            .map(|speed| Rung::Turntable(SpeedRatio(speed)))
            .to_vec();

        let first = &search(&index, &query, &profile, &ladder, 1)[0];
        let second = &search_twice(&index, &query, &profile, &ladder, 1)[0];

        assert!(
            (second.speed.0 - 1.0018).abs() < 0.0003,
            "{:?}",
            second.speed
        );
        assert!(
            second.evidence.hits > first.evidence.hits,
            "{} after {}",
            second.evidence.hits,
            first.evidence.hits
        );
        assert!((second.track_start_seconds - 20.0).abs() < 0.5);
        assert_eq!(second.evidence.windows, 4);
    }
}

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

use std::ops::{Range, RangeBounds};

use super::chains::{chain_speed, without_weak_ends};
use super::lines::{LINE_SPAN_FRAMES, Line, MIN_LINE_HITS, WINDOW_SECONDS};
use super::{Detection, Options};
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

/// With `Options::speed_per_stretch`: windows measured again together at
/// their own speed.
const STRETCH_WINDOWS: u32 = 3;
/// A stretch whose speed is closer than this to the fitted speed keeps its
/// first measurement; at 0.05% about 90% of hashes survive (experiment
/// 0001).
const MIN_SPEED_CORRECTION: f64 = 0.0005;
/// Analysis around a stretch reaches this far beyond it, for the peaks'
/// neighbourhoods and the anchors' target zones.
const STRETCH_MARGIN_FRAMES: f64 = 100.0;

/// Measures each detection with at least `MIN_HITS_TO_REFINE` hits again at
/// its fitted speed; `chains[i]` holds the indexes in `lines` of
/// `detections[i]`'s lines. Refined evidence is `Pass::Fitted`; a candidate
/// with no window of hits there is dropped. With `speed_per_stretch`, each
/// stretch of `STRETCH_WINDOWS` windows whose hits drift from the fitted
/// speed is measured again at its own. Returns the detections in no
/// particular order. With `trim_weak_ends`, boundaries leave out weak
/// windows at either end.
pub(super) fn refine(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    lines: &[Line],
    chained: &[(Detection, Vec<usize>)],
    jobs: usize,
    options: Options,
) -> Vec<Detection> {
    map_in_order(chained, jobs, |(detection, chain)| {
        if detection.evidence.hits < MIN_HITS_TO_REFINE {
            return Some(detection.clone());
        }
        let chain: Vec<&Line> = chain.iter().map(|&line| &lines[line]).collect();
        at_fitted_speed(index, samples, profile, detection, &chain, options)
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

/// A window's densest run of hits near an alignment.
#[derive(Clone)]
struct Measured {
    window: u32,
    hits: u32,
    /// Query frames of the first and last hit and their hit-weighted mean.
    first: f64,
    last: f64,
    centre: f64,
    /// The mean deviation from the alignment.
    deviation: f64,
    /// Reference frames of the first and last hit, on the alignment moved
    /// by the deviation.
    reference_first: f64,
    reference_last: f64,
}

/// Where a candidate is measured: a straight alignment of reference frames
/// against query frames, and the playback.
#[derive(Clone, Copy)]
struct Alignment {
    speed: f64,
    /// A query frame and the reference frame it aligns with.
    query: f64,
    reference: f64,
    playback: Playback,
}

impl Alignment {
    fn at(&self, query_frame: f64) -> f64 {
        self.reference + self.speed * (query_frame - self.query)
    }

    fn rung(&self) -> Rung {
        match self.playback {
            Playback::Turntable => Rung::Turntable(SpeedRatio(self.speed)),
            Playback::KeyLocked => Rung::KeyLocked(SpeedRatio(self.speed)),
        }
    }
}

fn at_fitted_speed(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    detection: &Detection,
    chain: &[&Line],
    options: Options,
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
    let fitted = Alignment {
        speed: chain_speed(&near),
        query: strongest.centre(),
        reference: strongest.reference_frame_at(strongest.centre()),
        playback,
    };

    // The chain's first and last hits: the detection's boundaries unless
    // they were trimmed.
    let start_seconds = profile.seconds(chain[0].first);
    let end_seconds = profile.seconds(chain[chain.len() - 1].last);
    let rate = f64::from(profile.sample_rate);
    let first_sample = ((start_seconds - MARGIN_SECONDS).max(0.0) * rate) as usize;
    let last_sample = (((end_seconds + MARGIN_SECONDS) * rate) as usize).min(samples.len());
    if first_sample >= last_sample {
        return None;
    }
    let span = first_sample..last_sample;
    let mut hits = hits_near(index, samples, profile, detection, fitted, span.clone(), ..);
    let mut measured = measure(&mut hits, fitted);
    if options.speed_per_stretch {
        measured = per_stretch(index, samples, profile, detection, fitted, span, measured);
    }
    if measured.is_empty() {
        return None;
    }
    let ends = if options.trim_weak_ends {
        without_weak_ends(&measured, |line| line.hits)
    } else {
        0..measured.len()
    };
    let (opening, closing) = (&measured[ends.start], &measured[ends.end - 1]);
    Some(Detection {
        asset: detection.asset,
        start_seconds: profile.seconds(opening.first),
        end_seconds: profile.seconds(closing.last),
        track_start_seconds: profile.seconds(opening.reference_first.max(0.0)),
        track_end_seconds: profile.seconds(closing.reference_last.max(0.0)),
        speed: SpeedRatio(fitted.speed),
        playback,
        evidence: Evidence {
            windows: measured.len() as u32,
            hits: measured.iter().map(|line| line.hits).sum(),
            pass: Pass::Fitted,
        },
    })
}

/// The hits of `detection`'s asset within `SEARCH_FRAMES` of `alignment`,
/// analysing `samples[span]` at its speed; only anchors in `windows` count.
fn hits_near(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    detection: &Detection,
    alignment: Alignment,
    span: Range<usize>,
    windows: impl RangeBounds<u32>,
) -> Vec<Hit> {
    let rate = f64::from(profile.sample_rate);
    let points = alignment.rung().points(&samples[span.clone()], profile);
    let start_frame = profile.frames(span.start as f64 / rate);
    let window_frames = profile.frames(WINDOW_SECONDS);
    let mut hits = Vec::new();
    for_each_pair(&points, |hash, anchor_point| {
        let query_frame = start_frame + points[anchor_point].frame / alignment.speed;
        let window = (query_frame / window_frames) as u32;
        if !windows.contains(&window) {
            return;
        }
        let expected = alignment.at(query_frame);
        for posting in index.postings(hash) {
            let deviation = f64::from(posting.frame()) - expected;
            if posting.asset() == detection.asset && deviation.abs() <= SEARCH_FRAMES {
                hits.push(Hit {
                    window,
                    deviation,
                    query_frame,
                });
            }
        }
    });
    hits
}

/// Each window's densest run of hits, when it has enough hits, in window
/// order.
fn measure(hits: &mut [Hit], alignment: Alignment) -> Vec<Measured> {
    hits.sort_unstable_by(|a, b| {
        a.window
            .cmp(&b.window)
            .then(a.deviation.total_cmp(&b.deviation))
    });
    hits.chunk_by(|a, b| a.window == b.window)
        .map(|window| &window[densest(window)])
        .filter(|line| line.len() >= MIN_LINE_HITS)
        .map(|line| {
            let count = line.len() as f64;
            let deviation = line.iter().map(|hit| hit.deviation).sum::<f64>() / count;
            let first = line
                .iter()
                .map(|hit| hit.query_frame)
                .fold(f64::INFINITY, f64::min);
            let last = line
                .iter()
                .map(|hit| hit.query_frame)
                .fold(f64::NEG_INFINITY, f64::max);
            Measured {
                window: line[0].window,
                hits: line.len() as u32,
                first,
                last,
                centre: line.iter().map(|hit| hit.query_frame).sum::<f64>() / count,
                deviation,
                reference_first: alignment.at(first) + deviation,
                reference_last: alignment.at(last) + deviation,
            }
        })
        .collect()
}

/// Measures each stretch of `STRETCH_WINDOWS` windows again at its own
/// speed and alignment, fitted to the drift of the deviations in it and
/// the window either side, when that speed is `MIN_SPEED_CORRECTION` or
/// more from the fitted one. The new measurement replaces the first, more
/// hits or fewer. `measured` is in window order.
fn per_stretch(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    detection: &Detection,
    fitted: Alignment,
    span: Range<usize>,
    measured: Vec<Measured>,
) -> Vec<Measured> {
    let Some(first_window) = measured.first().map(|line| line.window) else {
        return measured;
    };
    let last_window = measured[measured.len() - 1].window;
    let window_frames = profile.frames(WINDOW_SECONDS);
    let hop = profile.hop as f64;
    let mut result = Vec::with_capacity(measured.len());
    let mut stretch_start = first_window;
    while stretch_start <= last_window {
        let stretch = stretch_start..stretch_start + STRETCH_WINDOWS;
        stretch_start = stretch.end;
        let fit: Vec<&Measured> = measured
            .iter()
            .filter(|line| line.window + 1 >= stretch.start && line.window <= stretch.end)
            .collect();
        let kept = measured
            .iter()
            .filter(|line| stretch.contains(&line.window));
        let Some(local) = drift(&fit, fitted)
            .filter(|local| (local.speed - fitted.speed).abs() >= MIN_SPEED_CORRECTION)
        else {
            result.extend(kept.map(Measured::clone));
            continue;
        };
        let around = |window: u32, margin: f64| {
            let frame = f64::from(window) * window_frames + margin;
            ((frame.max(0.0) * hop) as usize).clamp(span.start, span.end)
        };
        let samples_span = around(stretch.start, -STRETCH_MARGIN_FRAMES)
            ..around(stretch.end, STRETCH_MARGIN_FRAMES);
        if samples_span.is_empty() {
            result.extend(kept.map(Measured::clone));
            continue;
        }
        let mut hits = hits_near(
            index,
            samples,
            profile,
            detection,
            local,
            samples_span,
            stretch,
        );
        result.extend(measure(&mut hits, local));
    }
    result
}

/// The alignment through the hit-weighted deviations of `lines` from
/// `fitted`: its slope added to the fitted speed. `None` without two
/// windows to fit.
fn drift(lines: &[&Measured], fitted: Alignment) -> Option<Alignment> {
    if lines.len() < 2 {
        return None;
    }
    let total: f64 = lines.iter().map(|line| f64::from(line.hits)).sum();
    let mean = |value: &dyn Fn(&Measured) -> f64| {
        lines
            .iter()
            .map(|line| f64::from(line.hits) * value(line))
            .sum::<f64>()
            / total
    };
    let centre = mean(&|line| line.centre);
    let deviation = mean(&|line| line.deviation);
    let variance = mean(&|line| (line.centre - centre).powi(2));
    if variance <= 0.0 {
        return None;
    }
    let slope = mean(&|line| (line.centre - centre) * (line.deviation - deviation)) / variance;
    Some(Alignment {
        speed: fitted.speed + slope,
        query: centre,
        reference: fitted.at(centre) + deviation,
        playback: fitted.playback,
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
    use crate::search::{Options, search, search_twice, search_with};
    use crate::speed::{Rung, SpeedRatio};

    #[test]
    fn a_speed_that_changes_within_a_play_is_followed_per_stretch() {
        let profile = Profile::CURRENT;
        let rate = profile.sample_rate;
        let reference = track(120.0, rate);
        let index = Index::build(&[record("a.wav", &reference)]).unwrap();
        // 40 s at 0.999 then 40 s at 1.003, without a jump in the record.
        let mut query = played(&reference, 0.999, 10.0, 40.0, rate);
        query.extend(played(&reference, 1.003, 10.0 + 0.999 * 40.0, 40.0, rate));
        let ladder: Vec<Rung> = [0.996, 1.0, 1.004]
            .map(|speed| Rung::Turntable(SpeedRatio(speed)))
            .to_vec();
        let options = |speed_per_stretch| Options {
            second_pass: true,
            speed_per_stretch,
            ..Options::default()
        };

        let once = &search_with(&index, &query, &profile, &ladder, 1, options(false))[0];
        let stretched = &search_with(&index, &query, &profile, &ladder, 1, options(true))[0];

        assert!(
            stretched.evidence.hits > once.evidence.hits * 11 / 10,
            "{} after {}",
            stretched.evidence.hits,
            once.evidence.hits
        );
        assert!((stretched.track_start_seconds - 10.0).abs() < 0.5);
    }

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

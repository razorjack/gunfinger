//! Version 2's line (`docs/brief-26.md`, step 3): where the reference
//! plays within a reported segment, measured from the reference's own
//! hash hits instead of taken from the report. Experiment 0127 found
//! reported lines 98-327 ms from the audio: the matcher takes a play's
//! track position from its opening window, which can hold another
//! alignment while the record is mixed in or a loop point, and some
//! uploads drift in speed within a play.
//!
//! The segment's mix is analysed at the reported speed against an index
//! of the reference alone. Each 10 s window's densest run of hits, at any
//! offset, is a *knot*. Knots whose offsets follow on within a pitch ride
//! form a *stretch*; a larger step is a jump (a loop, a needle drop). A
//! stretch's line is the fewest chords through its knots that keep every
//! knot within a frame, so a drifting speed is followed piece by piece.
//! A line is tested only between the reference's own hits.

use gunfinger_core::hash::for_each_pair;
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{STRONG_LINE_HITS, WINDOW_SECONDS};
use gunfinger_core::speed::{Rung, SpeedRatio};
use serde::Serialize;

use super::Segment;

/// A run's hits lie within this many file frames of each other (the
/// matcher's line span).
const RUN_FRAMES: f64 = 2.0;
/// A knot follows the previous one in its stretch when their offsets from
/// the reported line differ by at most this share of the time between
/// them (a 2% pitch ride)...
const RIDE: f64 = 0.02;
/// ...plus this many frames, the measurement's spread.
const SPREAD_FRAMES: f64 = 2.0;
/// Every knot lies within this many frames of its piece's chord.
const PIECE_TOLERANCE_FRAMES: f64 = 1.0;

/// One hash of the mix found in the reference: its mix time, and how far
/// its file time lies from the reported line there (seconds).
#[derive(Clone, Copy)]
pub struct Hit {
    pub t: f64,
    pub deviation: f64,
}

/// A window's densest run of hits.
#[derive(Clone, Serialize)]
pub struct Knot {
    /// The mean mix time of its hits, and the file time there.
    pub t: f64,
    pub file: f64,
    /// `file` minus the reported line's file time at `t` (seconds).
    pub deviation: f64,
    pub hits: usize,
    /// The mix times of its first and last hit.
    pub first: f64,
    pub last: f64,
    /// The stretch it belongs to, counted within the segment; `None` for
    /// a stray.
    pub stretch: Option<usize>,
}

/// A reported segment's measured line.
#[derive(Serialize)]
pub struct Line {
    pub reported: Segment,
    pub knots: Vec<Knot>,
    pub stretches: usize,
    pub strays: usize,
    /// The straight pieces the verifier tests, in mix order.
    pub pieces: Vec<Segment>,
}

/// The line of `segment` measured on `samples`, the mix from
/// `clock_start` seconds, against `reference`, an index of the
/// reference's record alone.
pub fn measure(samples: &[f32], clock_start: f64, segment: &Segment, reference: &Index) -> Line {
    let hits = hits(samples, clock_start, segment, reference);
    let mut knots = knots(&hits, segment);
    let stretches = stretches(&mut knots);
    let pieces = stretches
        .iter()
        .flat_map(|stretch| pieces(stretch, segment.speed))
        .collect();
    Line {
        reported: segment.clone(),
        strays: knots.iter().filter(|knot| knot.stretch.is_none()).count(),
        stretches: stretches.len(),
        knots,
        pieces,
    }
}

/// The hits of the mix analysed as turntable playback at the reported
/// speed, inside the segment's span.
fn hits(samples: &[f32], clock_start: f64, segment: &Segment, reference: &Index) -> Vec<Hit> {
    let profile = Profile::CURRENT;
    let points = Rung::Turntable(SpeedRatio(segment.speed)).points(samples, &profile);
    let mut hits = Vec::new();
    for_each_pair(&points, |hash, anchor| {
        // Points are in reference frames; the mix runs `speed` times slower.
        let t = clock_start + profile.seconds(points[anchor].frame / segment.speed);
        if !(segment.start_seconds..=segment.end_seconds).contains(&t) {
            return;
        }
        let reported = segment.file_time(t);
        for posting in reference.postings(hash) {
            hits.push(Hit {
                t,
                deviation: profile.seconds(f64::from(posting.frame())) - reported,
            });
        }
    });
    hits
}

/// Each 10 s window's knot, counted from the segment's start, where the
/// window has one.
fn knots(hits: &[Hit], segment: &Segment) -> Vec<Knot> {
    let windows = ((segment.end_seconds - segment.start_seconds) / WINDOW_SECONDS) as usize + 1;
    let mut by_window = vec![Vec::new(); windows];
    for hit in hits {
        let window = ((hit.t - segment.start_seconds) / WINDOW_SECONDS) as usize;
        by_window[window.min(windows - 1)].push(*hit);
    }
    by_window
        .iter_mut()
        .filter_map(|window| knot(window, segment))
        .collect()
}

/// The densest run of `window`'s hits within `RUN_FRAMES` of each other,
/// at any offset (ties to the run nearer the reported line), when it has
/// at least `STRONG_LINE_HITS` hits.
fn knot(window: &mut [Hit], segment: &Segment) -> Option<Knot> {
    window.sort_by(|a, b| a.deviation.total_cmp(&b.deviation));
    let span = Profile::CURRENT.seconds(RUN_FRAMES);
    let mut best: &[Hit] = &[];
    let mut end = 0;
    for start in 0..window.len() {
        end = end.max(start);
        while end < window.len() && window[end].deviation - window[start].deviation <= span {
            end += 1;
        }
        let run = &window[start..end];
        let denser = run.len() > best.len()
            || (run.len() == best.len() && mean_deviation(run).abs() < mean_deviation(best).abs());
        if denser {
            best = run;
        }
    }
    if best.len() < STRONG_LINE_HITS as usize {
        return None;
    }
    let count = best.len() as f64;
    let t = best.iter().map(|hit| hit.t).sum::<f64>() / count;
    let deviation = mean_deviation(best);
    Some(Knot {
        t,
        file: segment.file_time(t) + deviation,
        deviation,
        hits: best.len(),
        first: best.iter().map(|hit| hit.t).fold(f64::INFINITY, f64::min),
        last: best
            .iter()
            .map(|hit| hit.t)
            .fold(f64::NEG_INFINITY, f64::max),
        stretch: None,
    })
}

fn mean_deviation(run: &[Hit]) -> f64 {
    run.iter().map(|hit| hit.deviation).sum::<f64>() / run.len().max(1) as f64
}

/// The knots' stretches, in mix order: a knot follows the previous one
/// unless its offset jumped. A stretch of one knot, among others, is a
/// stray and dropped. Marks each kept knot with its stretch.
fn stretches(knots: &mut [Knot]) -> Vec<Vec<Knot>> {
    let spread = Profile::CURRENT.seconds(SPREAD_FRAMES);
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (i, knot) in knots.iter().enumerate() {
        let follows = groups
            .last()
            .and_then(|group| group.last())
            .is_some_and(|&previous| {
                let previous = &knots[previous];
                (knot.deviation - previous.deviation).abs() <= RIDE * (knot.t - previous.t) + spread
            });
        match groups.last_mut() {
            Some(group) if follows => group.push(i),
            _ => groups.push(vec![i]),
        }
    }
    if knots.len() > 1 {
        groups.retain(|group| group.len() > 1);
    }
    for (number, group) in groups.iter().enumerate() {
        for &i in group {
            knots[i].stretch = Some(number);
        }
    }
    groups
        .iter()
        .map(|group| group.iter().map(|&i| knots[i].clone()).collect())
        .collect()
}

/// A stretch's pieces: chords between the knots `corners` picks, the first
/// from the stretch's first hit and the last to its last hit. A lone knot
/// is its own piece at the reported speed.
fn pieces(stretch: &[Knot], reported_speed: f64) -> Vec<Segment> {
    let (first, last) = (&stretch[0], &stretch[stretch.len() - 1]);
    if stretch.len() == 1 {
        return vec![through(first, reported_speed, first.first, first.last)];
    }
    let mut ends = vec![0];
    corners(stretch, 0, stretch.len() - 1, &mut ends);
    ends.push(stretch.len() - 1);
    ends.windows(2)
        .map(|pair| {
            let (a, b) = (&stretch[pair[0]], &stretch[pair[1]]);
            let speed = (b.file - a.file) / (b.t - a.t);
            let start = if pair[0] == 0 { first.first } else { a.t };
            let end = if pair[1] == stretch.len() - 1 {
                last.last
            } else {
                b.t
            };
            through(a, speed, start, end)
        })
        .collect()
}

/// The knots strictly between `from` and `to` where the chord is split
/// (Ramer-Douglas-Peucker), in order: the farthest from the chord while
/// any lies beyond `PIECE_TOLERANCE_FRAMES`.
fn corners(knots: &[Knot], from: usize, to: usize, found: &mut Vec<usize>) {
    let (a, b) = (&knots[from], &knots[to]);
    let speed = (b.file - a.file) / (b.t - a.t);
    let distance = |knot: &Knot| (knot.file - (a.file + speed * (knot.t - a.t))).abs();
    let farthest =
        (from + 1..to).max_by(|&i, &j| distance(&knots[i]).total_cmp(&distance(&knots[j])));
    let tolerance = Profile::CURRENT.seconds(PIECE_TOLERANCE_FRAMES);
    if let Some(corner) = farthest.filter(|&i| distance(&knots[i]) > tolerance) {
        corners(knots, from, corner, found);
        found.push(corner);
        corners(knots, corner, to, found);
    }
}

/// The piece from `start` to `end` (mix seconds) through `knot` at `speed`.
fn through(knot: &Knot, speed: f64, start: f64, end: f64) -> Segment {
    Segment {
        start_seconds: start,
        end_seconds: end,
        track_start_seconds: knot.file + speed * (start - knot.t),
        speed,
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::peaks::extract_peaks;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    const RATE: f64 = 8000.0;

    /// Two tones at pseudo-random frequencies every 50 ms, so that every
    /// stretch of the file is distinct.
    fn reference(seconds: f64) -> Vec<f32> {
        let mut state: u64 = 26;
        let mut draw = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 11) as f64 / (1u64 << 53) as f64
        };
        let burst = (0.05 * RATE) as usize;
        let mut samples = vec![0.0_f32; (seconds * RATE) as usize];
        for chunk in samples.chunks_mut(burst) {
            let tones = [200.0 + 3300.0 * draw(), 200.0 + 3300.0 * draw()];
            for (i, sample) in chunk.iter_mut().enumerate() {
                let t = i as f64 / RATE;
                let envelope = (std::f64::consts::PI * i as f64 / burst as f64).sin();
                let value: f64 = tones
                    .iter()
                    .map(|hz| (2.0 * std::f64::consts::PI * hz * t).sin())
                    .sum();
                *sample = (0.3 * envelope * value) as f32;
            }
        }
        samples
    }

    fn index_of(samples: &[f32]) -> Index {
        let profile = Profile::CURRENT;
        let record = PeakRecord {
            header: RecordHeader {
                profile: profile.id(),
                source: Asset {
                    path: "reference.wav".to_owned(),
                    size: 0,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: samples.len() as f64 / RATE,
            },
            peaks: extract_peaks(samples, &profile),
        };
        Index::build(&[record]).unwrap()
    }

    /// `seconds` of mix in which mix time `t` plays file time `file(t)`.
    fn played(samples: &[f32], seconds: f64, file: impl Fn(f64) -> f64) -> Vec<f32> {
        (0..(seconds * RATE) as usize)
            .map(|i| {
                let at = file(i as f64 / RATE) * RATE;
                let (whole, fraction) = (at.floor() as usize, at.fract() as f32);
                samples[whole] * (1.0 - fraction) + samples[whole + 1] * fraction
            })
            .collect()
    }

    fn segment(start: f64, end: f64, track_start: f64, speed: f64) -> Segment {
        Segment {
            start_seconds: start,
            end_seconds: end,
            track_start_seconds: track_start,
            speed,
        }
    }

    /// The largest distance (seconds) between the pieces' lines and the
    /// truth, every 0.1 s over what the pieces cover.
    fn largest_error(pieces: &[Segment], truth: impl Fn(f64) -> f64) -> f64 {
        let mut largest: f64 = 0.0;
        for piece in pieces {
            let mut t = piece.start_seconds;
            while t <= piece.end_seconds {
                largest = largest.max((piece.file_time(t) - truth(t)).abs());
                t += 0.1;
            }
        }
        largest
    }

    fn covered(pieces: &[Segment]) -> f64 {
        pieces
            .iter()
            .map(|piece| piece.end_seconds - piece.start_seconds)
            .sum()
    }

    #[test]
    fn a_line_150_ms_off_is_measured_where_the_audio_is() {
        let file = reference(90.0);
        let truth = |t: f64| 20.0 + 1.02 * t;
        let mix = played(&file, 50.0, truth);
        // The report puts file time 20.15 s where the mix plays 20 s.
        let reported = segment(1.0, 49.0, truth(1.0) + 0.15, 1.02);

        let line = measure(&mix, 0.0, &reported, &index_of(&file));

        assert_eq!((line.stretches, line.strays), (1, 0));
        assert_eq!(line.pieces.len(), 1);
        assert!(line.knots.len() >= 4, "{}", line.knots.len());
        assert!(
            line.knots
                .iter()
                .all(|knot| (knot.deviation + 0.15).abs() < 0.016)
        );
        assert!(largest_error(&line.pieces, truth) < 0.016);
        assert!(covered(&line.pieces) > 40.0, "{}", covered(&line.pieces));
    }

    #[test]
    fn a_drifting_copy_is_followed_by_pieces() {
        let file = reference(90.0);
        // The speed drifts from 0.995 to 1.005 over the minute.
        let truth = |t: f64| 10.0 + 0.995 * t + 0.5 * (0.01 / 60.0) * t * t;
        let mix = played(&file, 60.0, truth);
        // The report's one line, at the mean speed, is up to 75 ms off.
        let reported = segment(1.0, 59.0, truth(1.0), 1.0);
        assert!(largest_error(std::slice::from_ref(&reported), truth) > 0.06);

        let line = measure(&mix, 0.0, &reported, &index_of(&file));

        assert_eq!((line.stretches, line.strays), (1, 0));
        assert!(line.pieces.len() >= 2, "{}", line.pieces.len());
        assert!(
            largest_error(&line.pieces, truth) < 0.032,
            "{}",
            largest_error(&line.pieces, truth)
        );
        assert!(covered(&line.pieces) > 45.0, "{}", covered(&line.pieces));
    }

    #[test]
    fn a_jump_back_starts_a_new_stretch() {
        let file = reference(90.0);
        // File 10-45 s, then a jump back to 30 s at mix time 35 s.
        let truth = |t: f64| if t < 35.0 { 10.0 + t } else { t - 5.0 };
        let mix = played(&file, 70.0, truth);
        let reported = segment(1.0, 69.0, truth(1.0), 1.0);

        let line = measure(&mix, 0.0, &reported, &index_of(&file));

        assert_eq!((line.stretches, line.strays), (2, 0));
        assert!(largest_error(&line.pieces, truth) < 0.016);
        // Nothing is tested across the jump.
        assert!(
            line.pieces
                .iter()
                .all(|piece| piece.end_seconds <= 35.0 || piece.start_seconds >= 35.0)
        );
        assert!(covered(&line.pieces) > 55.0, "{}", covered(&line.pieces));
    }

    #[test]
    fn a_lone_window_at_another_offset_is_a_stray() {
        let file = reference(120.0);
        // Ten seconds of another part of the file in the middle.
        let truth = |t: f64| {
            if (30.0..40.0).contains(&t) {
                t + 60.0
            } else {
                10.0 + t
            }
        };
        let mix = played(&file, 60.0, truth);
        let reported = segment(0.5, 59.5, truth(0.5), 1.0);

        let line = measure(&mix, 0.0, &reported, &index_of(&file));

        assert_eq!((line.stretches, line.strays), (2, 1));
        assert!(largest_error(&line.pieces, truth) < 0.016);
        assert!(
            line.pieces
                .iter()
                .all(|piece| piece.end_seconds <= 30.0 || piece.start_seconds >= 40.0)
        );
    }

    #[test]
    fn a_segment_without_the_reference_has_no_line() {
        let file = reference(90.0);
        let silence = vec![0.0_f32; (30.0 * RATE) as usize];
        let reported = segment(1.0, 29.0, 5.0, 1.0);

        let line = measure(&silence, 0.0, &reported, &index_of(&file));

        assert!(line.knots.is_empty());
        assert!(line.pieces.is_empty());
    }
}

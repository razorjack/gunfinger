//! A peak verifier after Qfp's (Sonnleitner & Widmer 2016, §VI-C), as a
//! harness diagnostic. For a detection, the asset's reference peaks within
//! the aligned span are mapped to their expected places in the query at
//! the detection's speed, playback and alignment, and looked for among the
//! query's peaks within a small tolerance in time and frequency. Each 10 s
//! window of the span is aligned on its own, by the densest run of the
//! asset's hash hits, as the second pass follows a wandering speed; the
//! verifier itself searches no offset.
//!
//! It counts reference peaks found in the query, never query peaks
//! matched: a blend adds query peaks, which must not count for either
//! record. Peaks can survive where their pair hashes do not (a neighbour
//! that changed, a peak half a bin away; Six 2021, §2), so the count is
//! evidence the second pass does not see. The same count with the
//! reference shifted a few seconds away from the alignment, either way,
//! measures the chance level, including what the track's own repetition
//! gives.

use std::collections::HashMap;

use gunfinger_core::hash::{Point, for_each_pair};
use gunfinger_core::index::Index;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::speed::{Playback, Rung};
use gunfinger_core::store::PeakRecord;
use serde::{Deserialize, Serialize};

use crate::padding::SecondLibrary;

/// How far from its expected place a query peak may lie and still find a
/// reference peak: half-widths in bins and in frames.
#[derive(Clone, Copy, Debug)]
pub struct Tolerance {
    pub bins: f32,
    pub frames: f64,
}

/// The settings measured to choose one. Qfp's box of 12 bins by 18 frames
/// of 4 ms is about ±6 bins and ±2 of Gunfinger's 16 ms frames.
pub const TOLERANCES: [Tolerance; 3] = [
    Tolerance {
        bins: 1.0,
        frames: 1.0,
    },
    Tolerance {
        bins: 3.0,
        frames: 2.0,
    },
    Tolerance {
        bins: 6.0,
        frames: 2.0,
    },
];

/// Shifts of the reference away from the alignment that measure chance, in
/// reference seconds. None is a whole number of bars between 160 and 180
/// beats per minute, so a drum loop repeating every bar does not line up
/// again.
pub const CHANCE_SHIFTS_SECONDS: [f64; 4] = [-6.2, -5.0, 5.0, 6.2];

/// Query time analysed beyond the detection's span, so that peaks at its
/// edges keep their neighbourhoods.
const MARGIN_SECONDS: f64 = 1.0;
/// Each window of the span this long is aligned on its own...
const WINDOW_SECONDS: f64 = 10.0;
/// ...by the asset's hash hits within this many frames of the line through
/// the detection's ends (the second pass keeps hits within 24 frames of its
/// fitted line, which can lie 24 frames from that line at either end)...
const SEARCH_FRAMES: f64 = 48.0;
/// ...at the mean of their densest run this wide (`LINE_SPAN_FRAMES` in the
/// search), when it has this many hits.
const RUN_FRAMES: f64 = 2.0;
const MIN_RUN_HITS: usize = 3;
/// Support over time counts slices of the span this long.
const SLICE_SECONDS: f64 = 1.0;

/// One detection measured under one tolerance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Verification {
    /// `<bins>x<frames>`, the tolerance's half-widths.
    pub tolerance: String,
    pub aligned: Count,
    /// At each of `CHANCE_SHIFTS_SECONDS`, in order.
    pub shifted: Vec<Count>,
}

/// Reference peaks in a span and those found in the query.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Count {
    pub shift_seconds: f64,
    pub reference_peaks: u32,
    pub found: u32,
    /// `found` over `reference_peaks`.
    pub share: f64,
    /// The share of 1 s slices of the detection's span (query time) with a
    /// found peak.
    pub support: f64,
}

/// The reference peaks of indexed assets: the library's records, kept in
/// memory, and the second library's, read from its store when needed.
pub struct Verifier<'a> {
    library: HashMap<String, Vec<Peak>>,
    second: Option<&'a SecondLibrary>,
}

impl<'a> Verifier<'a> {
    pub fn new(records: &[PeakRecord], second: Option<&'a SecondLibrary>) -> Verifier<'a> {
        Verifier {
            library: records
                .iter()
                .map(|record| (record.header.source.path.clone(), record.peaks.clone()))
                .collect(),
            second,
        }
    }

    /// `detection` measured under every tolerance; empty when its asset's
    /// peaks are not at hand (a synthetic copy) or its span is empty.
    pub fn verify(
        &self,
        index: &Index,
        detection: &Detection,
        samples: &[f32],
        profile: &Profile,
    ) -> Vec<Verification> {
        let path = &index.asset(detection.asset).path;
        let loaded;
        let reference: &[Peak] = match self.library.get(path) {
            Some(peaks) => peaks,
            None => match self.second.and_then(|second| second.record(path)) {
                Some(Ok(record)) => {
                    loaded = record.peaks;
                    &loaded
                }
                Some(Err(error)) => {
                    eprintln!("verifier: {error}");
                    return Vec::new();
                }
                None => return Vec::new(),
            },
        };
        let Some(line) = Line::of(detection, profile) else {
            return Vec::new();
        };
        let analysed = Analysed::of(detection, samples, profile, &line);
        let offsets = line.window_offsets(index, detection, &analysed, profile);
        let query = analysed.placed(&line);
        let aligned = Aligned {
            line,
            offsets,
            window_frames: profile.frames(WINDOW_SECONDS),
        };
        TOLERANCES
            .iter()
            .map(|&tolerance| Verification {
                tolerance: format!("{}x{}", tolerance.bins, tolerance.frames),
                aligned: count(reference, &query, &aligned, tolerance, 0.0, profile),
                shifted: CHANCE_SHIFTS_SECONDS
                    .iter()
                    .map(|&shift| count(reference, &query, &aligned, tolerance, shift, profile))
                    .collect(),
            })
            .collect()
    }
}

/// The straight line through the detection's first and last aligned hits:
/// reference frame = `reference` + `slope` × (query frame − `first`), in
/// frames of the profile's hop. The second pass places both on its fitted
/// alignment, each moved by its window's deviation.
struct Line {
    reference: f64,
    slope: f64,
    /// The span in query frames.
    first: f64,
    last: f64,
}

impl Line {
    fn of(detection: &Detection, profile: &Profile) -> Option<Line> {
        let first = profile.frames(detection.start_seconds);
        let last = profile.frames(detection.end_seconds);
        let reference = profile.frames(detection.track_start_seconds);
        let slope = (profile.frames(detection.track_end_seconds) - reference) / (last - first);
        (last > first && slope.is_finite() && slope > 0.0).then_some(Line {
            reference,
            slope,
            first,
            last,
        })
    }

    fn reference_at(&self, query_frame: f64) -> f64 {
        self.reference + self.slope * (query_frame - self.first)
    }

    fn query_at(&self, reference_frame: f64) -> f64 {
        self.first + (reference_frame - self.reference) / self.slope
    }

    fn windows(&self, window_frames: f64) -> usize {
        ((self.last - self.first) / window_frames).ceil().max(1.0) as usize
    }

    /// Each window's offset from the line, as the second pass measures it:
    /// the mean of the densest run of the asset's hash hits within
    /// `RUN_FRAMES`, when it has `MIN_RUN_HITS`. A window without one takes
    /// the nearest window's offset; with none at all, the line itself.
    fn window_offsets(
        &self,
        index: &Index,
        detection: &Detection,
        analysed: &Analysed,
        profile: &Profile,
    ) -> Vec<f64> {
        let window_frames = profile.frames(WINDOW_SECONDS);
        let windows = self.windows(window_frames);
        let mut deviations: Vec<Vec<f64>> = vec![Vec::new(); windows];
        for_each_pair(&analysed.points, |hash, anchor| {
            let query_frame = analysed.query_frame(&analysed.points[anchor]);
            let from_start = query_frame - self.first;
            if from_start < 0.0 || query_frame > self.last {
                return;
            }
            let window = ((from_start / window_frames) as usize).min(windows - 1);
            let expected = self.reference_at(query_frame);
            for posting in index.postings(hash) {
                let deviation = f64::from(posting.frame()) - expected;
                if posting.asset() == detection.asset && deviation.abs() <= SEARCH_FRAMES {
                    deviations[window].push(deviation);
                }
            }
        });
        let measured: Vec<Option<f64>> = deviations.into_iter().map(densest_run).collect();
        (0..windows)
            .map(|window| {
                (0..windows)
                    .filter_map(|other| {
                        measured[other].map(|offset| (window.abs_diff(other), offset))
                    })
                    .min_by_key(|(distance, _)| *distance)
                    .map_or(0.0, |(_, offset)| offset)
            })
            .collect()
    }
}

/// The mean of the densest run of `deviations` within `RUN_FRAMES`, when
/// it holds `MIN_RUN_HITS`.
fn densest_run(mut deviations: Vec<f64>) -> Option<f64> {
    deviations.sort_by(f64::total_cmp);
    let mut best = 0..0;
    let mut end = 0;
    for start in 0..deviations.len() {
        end = end.max(start);
        while end < deviations.len() && deviations[end] - deviations[start] <= RUN_FRAMES {
            end += 1;
        }
        if end - start > best.len() {
            best = start..end;
        }
    }
    let run = &deviations[best];
    (run.len() >= MIN_RUN_HITS).then(|| run.iter().sum::<f64>() / run.len() as f64)
}

/// The query around the span analysed at the detection's speed (pitch and
/// tempo for a turntable, tempo alone with key lock): points in reference
/// bins whose frames count from `from_frame` in query frames times the
/// speed, as the second pass hashes them.
struct Analysed {
    points: Vec<Point>,
    from_frame: f64,
    speed: f64,
}

impl Analysed {
    fn of(detection: &Detection, samples: &[f32], profile: &Profile, line: &Line) -> Analysed {
        let rate = f64::from(profile.sample_rate);
        let from = ((profile.seconds(line.first) - MARGIN_SECONDS).max(0.0) * rate) as usize;
        let to =
            (((profile.seconds(line.last) + MARGIN_SECONDS) * rate) as usize).min(samples.len());
        let rung = match detection.playback {
            Playback::Turntable => Rung::Turntable(detection.speed),
            Playback::KeyLocked => Rung::KeyLocked(detection.speed),
        };
        Analysed {
            points: if from < to {
                rung.points(&samples[from..to], profile)
            } else {
                Vec::new()
            },
            from_frame: from as f64 / profile.hop as f64,
            speed: detection.speed.0,
        }
    }

    fn query_frame(&self, point: &Point) -> f64 {
        self.from_frame + point.frame / self.speed
    }

    /// The points placed on the line, sorted by frame: where reference
    /// peaks are expected, before each window's offset.
    fn placed(&self, line: &Line) -> Vec<Placed> {
        let mut placed: Vec<Placed> = self
            .points
            .iter()
            .map(|point| Placed {
                reference_frame: line.reference_at(self.query_frame(point)),
                bin: point.bin,
            })
            .collect();
        placed.sort_by(|a, b| a.reference_frame.total_cmp(&b.reference_frame));
        placed
    }
}

/// A query peak on the line: the reference frame it aligns with, and its
/// bin.
struct Placed {
    reference_frame: f64,
    bin: f32,
}

/// The line and each window's offset from it.
struct Aligned {
    line: Line,
    offsets: Vec<f64>,
    window_frames: f64,
}

/// The reference peaks of the span, `shift_seconds` away from the
/// alignment, and those with a query peak within `tolerance` of their
/// expected place: on the line, moved by their window's offset.
fn count(
    reference: &[Peak],
    query: &[Placed],
    aligned: &Aligned,
    tolerance: Tolerance,
    shift_seconds: f64,
    profile: &Profile,
) -> Count {
    let line = &aligned.line;
    let shift = profile.frames(shift_seconds);
    let span = line.reference_at(line.first) + shift..=line.reference_at(line.last) + shift;
    let slice_frames = profile.frames(SLICE_SECONDS);
    let slices = ((line.last - line.first) / slice_frames).ceil().max(1.0) as usize;
    let windows = aligned.offsets.len();
    let mut supported = vec![false; slices];
    let mut reference_peaks = 0;
    let mut found = 0;
    for peak in reference.iter().filter(|peak| span.contains(&peak.frame)) {
        reference_peaks += 1;
        let on_line = peak.frame - shift;
        let from_start = line.query_at(on_line) - line.first;
        let window = ((from_start / aligned.window_frames) as usize).min(windows - 1);
        // A window's hits lie `offset` reference frames from the line
        // (reference minus line), so its query peaks lie as far before the
        // reference peaks they find.
        let expected = on_line - aligned.offsets[window];
        let first =
            query.partition_point(|point| point.reference_frame < expected - tolerance.frames);
        let is_found = query[first..]
            .iter()
            .take_while(|point| point.reference_frame <= expected + tolerance.frames)
            .any(|point| (point.bin - peak.bin).abs() <= tolerance.bins);
        if is_found {
            found += 1;
            supported[((from_start / slice_frames) as usize).min(slices - 1)] = true;
        }
    }
    Count {
        shift_seconds,
        reference_peaks,
        found,
        share: if reference_peaks == 0 {
            0.0
        } else {
            f64::from(found) / f64::from(reference_peaks)
        },
        support: supported.iter().filter(|&&slice| slice).count() as f64 / slices as f64,
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::confidence::Evidence;
    use gunfinger_core::index::AssetId;
    use gunfinger_core::speed::SpeedRatio;

    use super::*;

    fn peak(frame: f64, bin: f32) -> Peak {
        Peak {
            frame,
            bin,
            magnitude: 0.0,
        }
    }

    fn placed(reference_frame: f64, bin: f32) -> Placed {
        Placed {
            reference_frame,
            bin,
        }
    }

    /// Reference frames from `reference` align with query frames from
    /// `first` to `last`, each window moved by its offset.
    fn aligned(first: f64, last: f64, reference: f64, offsets: Vec<f64>) -> Aligned {
        Aligned {
            line: Line {
                reference,
                slope: 1.0,
                first,
                last,
            },
            offsets,
            window_frames: Profile::CURRENT.frames(WINDOW_SECONDS),
        }
    }

    const NARROW: Tolerance = Tolerance {
        bins: 1.0,
        frames: 1.0,
    };

    #[test]
    fn reference_peaks_found_near_their_place_count_once_each() {
        let profile = Profile::CURRENT;
        let line = aligned(0.0, 100.0, 1000.0, vec![0.0]);
        let reference = [
            peak(1010.0, 100.0),
            peak(1050.0, 200.0),
            peak(1090.0, 300.0),
            peak(2000.0, 100.0), // outside the span
        ];
        let query = [
            placed(1010.5, 100.5),
            // Two query peaks near one reference peak: it is found once.
            placed(1049.5, 199.5),
            placed(1050.5, 200.5),
            // A blended record's peak, far from any reference peak.
            placed(1070.0, 50.0),
            // Too far in frequency.
            placed(1090.0, 305.0),
        ];

        let counted = count(&reference, &query, &line, NARROW, 0.0, &profile);

        assert_eq!(counted.reference_peaks, 3);
        assert_eq!(counted.found, 2);
        assert!((counted.share - 2.0 / 3.0).abs() < 1e-9);
        // 100 frames is 1.6 s, two slices; both found peaks lie in the
        // first.
        assert!((counted.support - 0.5).abs() < 1e-9);
    }

    #[test]
    fn a_shift_looks_for_other_reference_peaks_at_the_same_query_places() {
        let profile = Profile::CURRENT;
        let line = aligned(0.0, 100.0, 1000.0, vec![0.0]);
        let shift = profile.frames(5.0);
        let reference = [peak(1010.0, 100.0), peak(1010.0 + shift, 100.0)];
        let query = [placed(1010.0, 100.0)];

        let shifted = count(&reference, &query, &line, NARROW, 5.0, &profile);

        assert_eq!(shifted.reference_peaks, 1);
        assert_eq!(shifted.found, 1);
    }

    #[test]
    fn each_window_is_looked_for_at_its_own_offset() {
        let profile = Profile::CURRENT;
        let second = profile.frames(1.0);
        let reference: Vec<Peak> = (0..25)
            .map(|at| peak(f64::from(at) * second + 5.0, 100.0))
            .collect();
        // The second window's query peaks lie 10 frames late on the line,
        // as when the speed wanders from it: its hits lie 10 frames before
        // the line.
        let late = profile.frames(10.0)..profile.frames(20.0);
        let query: Vec<Placed> = reference
            .iter()
            .map(|peak| {
                let offset = if late.contains(&peak.frame) {
                    10.0
                } else {
                    0.0
                };
                placed(peak.frame + offset, peak.bin)
            })
            .collect();
        let straight = aligned(0.0, profile.frames(25.0), 0.0, vec![0.0, 0.0, 0.0]);
        let followed = aligned(0.0, profile.frames(25.0), 0.0, vec![0.0, -10.0, 0.0]);

        let on_the_line = count(&reference, &query, &straight, NARROW, 0.0, &profile);
        let per_window = count(&reference, &query, &followed, NARROW, 0.0, &profile);

        assert_eq!(on_the_line.found, 15);
        assert_eq!(per_window.found, 25);
    }

    #[test]
    fn a_windows_offset_is_its_densest_run_of_hits() {
        assert_eq!(densest_run(vec![9.0, -20.0, 10.0, 11.0, 40.0]), Some(10.0));
        assert_eq!(densest_run(vec![9.0, 10.0, 30.0]), None);
    }

    #[test]
    fn the_line_runs_through_the_detections_ends() {
        let profile = Profile::CURRENT;
        let detection = Detection {
            asset: AssetId(0),
            start_seconds: 10.0,
            end_seconds: 20.0,
            track_start_seconds: 40.0,
            track_end_seconds: 50.5,
            speed: SpeedRatio(1.05),
            playback: Playback::Turntable,
            evidence: Evidence::new(3, 300),
        };

        let line = Line::of(&detection, &profile).unwrap();

        assert!((line.reference_at(profile.frames(10.0)) - profile.frames(40.0)).abs() < 1e-9);
        assert!((line.reference_at(profile.frames(20.0)) - profile.frames(50.5)).abs() < 1e-9);
        assert!((line.query_at(profile.frames(50.5)) - profile.frames(20.0)).abs() < 1e-9);
    }
}

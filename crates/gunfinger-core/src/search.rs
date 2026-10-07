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
mod refine;

use std::cmp::Reverse;
use std::ops::Range;

use crate::confidence::Evidence;
use crate::hash::Point;
use crate::index::{AssetId, Index};
use crate::peaks::Peak;
use crate::profile::Profile;
use crate::speed::{self, Playback, Rung, SpeedRatio};

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
    /// For a key-locked detection, `speed` is the tempo; pitch is
    /// unchanged.
    pub playback: Playback,
    pub evidence: Evidence,
}

/// The matching settings as reports record them: lines, chains and the
/// ladder's speeds. The version changes when matching changes in a way
/// these numbers do not show.
pub fn design() -> String {
    format!(
        "lines-chains-v1 {} {} {}",
        lines::design(),
        chains::design(),
        speed::design()
    )
}

/// One window's hits of one asset along one line: the evidence chains join
/// into detections.
#[derive(Debug, Clone, PartialEq)]
pub struct WindowLine {
    pub asset: AssetId,
    /// The window of query time, counted from the start of the query.
    pub window: u32,
    /// Query time of the first and last hit, in seconds.
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Where the first hit lies in the asset, in seconds.
    pub track_seconds: f64,
    /// The rung the line was found on.
    pub speed: SpeedRatio,
    pub playback: Playback,
    pub hits: u32,
}

/// A search with its evidence: every line found and, for each detection,
/// the lines its chain took.
pub struct Trace {
    /// As `search` returns them, strongest first.
    pub detections: Vec<Detection>,
    /// Sorted by asset and window.
    pub lines: Vec<WindowLine>,
    /// `chains[i]` holds the indexes in `lines` of `detections[i]`'s lines,
    /// in window order.
    pub chains: Vec<Vec<usize>>,
}

/// Window length in seconds, for showing where windows fall.
pub const WINDOW_SECONDS: f64 = lines::WINDOW_SECONDS;

pub use chains::{Links, STRONG_LINE_HITS};

/// Opt-in changes to matching, under evaluation. The default is the
/// matcher `search` runs.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Options {
    /// Measure each candidate again at its fitted speed (`search_twice`).
    pub second_pass: bool,
    /// Which lines a chain may join.
    pub links: Links,
    /// In the second pass, measure each stretch of a few windows again at
    /// its own speed when it drifts from the fitted one.
    pub speed_per_stretch: bool,
    /// Detections' boundaries leave out weak windows at either end; the
    /// evidence still counts them.
    pub trim_weak_ends: bool,
}

/// Searches `samples` (mono, at the profile's rate) for the assets of
/// `index` on each rung of `ladder` (normally `speed::ladder()`),
/// running rungs on `jobs` threads. Returns the detections strongest first;
/// the caller decides which are confident.
pub fn search(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
) -> Vec<Detection> {
    search_with_progress(index, samples, profile, ladder, jobs, |_, _| {})
}

/// Like `search`, calling `progress` from the workers with the parts of the
/// search finished and their number after each part.
pub fn search_with_progress(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize, usize) + Sync,
) -> Vec<Detection> {
    let (_, chained) = lines_and_detections(
        index,
        Query::Samples(samples),
        profile,
        ladder,
        jobs,
        Options::default(),
        progress,
    );
    chained
        .into_iter()
        .map(|(detection, _)| detection)
        .collect()
}

/// Like `search`, then the second pass (opt-in, `refine`): each candidate
/// with a few hits or more is measured again at its fitted speed, its
/// evidence on that pass's scale (`Pass::Fitted`).
pub fn search_twice(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
) -> Vec<Detection> {
    let options = Options {
        second_pass: true,
        ..Options::default()
    };
    search_with(index, samples, profile, ladder, jobs, options)
}

/// Like `search`, with opt-in changes.
pub fn search_with(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    options: Options,
) -> Vec<Detection> {
    let (lines, chained) = lines_and_detections(
        index,
        Query::Samples(samples),
        profile,
        ladder,
        jobs,
        options,
        |_, _| {},
    );
    if !options.second_pass {
        return chained
            .into_iter()
            .map(|(detection, _)| detection)
            .collect();
    }
    let mut refined = refine::refine(index, samples, profile, &lines, &chained, jobs, options);
    refined.sort_by_key(|detection| Reverse(detection.evidence.hits));
    strongest_per_moment(
        refined
            .into_iter()
            .map(|detection| (detection, Vec::new()))
            .collect(),
    )
    .into_iter()
    .map(|(detection, _)| detection)
    .collect()
}

/// Searches the stored peaks of a library file instead of audio, without
/// decoding it: for finding duplicates. The peaks are rescaled to each rung
/// rather than analysed again, which loses about half the hashes that
/// survive (experiment 0001); whole recordings keep plenty.
pub fn search_peaks(
    index: &Index,
    peaks: &[Peak],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
) -> Vec<Detection> {
    let (_, chained) = lines_and_detections(
        index,
        Query::Peaks(peaks),
        profile,
        ladder,
        jobs,
        Options::default(),
        |_, _| {},
    );
    chained
        .into_iter()
        .map(|(detection, _)| detection)
        .collect()
}

/// What is searched: audio, or the stored peaks of a library file.
#[derive(Clone, Copy)]
enum Query<'a> {
    Samples(&'a [f32]),
    Peaks(&'a [Peak]),
}

impl Query<'_> {
    /// The query's peaks in reference coordinates under `rung` whose query
    /// frames (reference frame over speed) lie in `query_frames`, with a few
    /// more on either side. Each STFT frame's peaks are all in or all out.
    fn points_near(self, rung: Rung, profile: &Profile, query_frames: Range<f64>) -> Vec<Point> {
        match self {
            Query::Samples(samples) => {
                let step = rung.hop(profile) as f64 / profile.hop as f64;
                rung.points_in(samples, profile, stft_frames(query_frames, step))
            }
            Query::Peaks(peaks) => {
                let frames = stft_frames(query_frames, 1.0);
                // A refined peak lies within half a frame of its STFT frame.
                let stft_frame = |peak: &Peak| peak.frame.round() as usize;
                let first = peaks.partition_point(|peak| stft_frame(peak) < frames.start);
                let end = peaks.partition_point(|peak| stft_frame(peak) < frames.end);
                let tempo = rung.speed().0;
                let pitch = match rung {
                    Rung::Turntable(speed) => speed.0,
                    Rung::KeyLocked(_) => 1.0,
                };
                peaks[first..end.max(first)]
                    .iter()
                    .map(|peak| Point {
                        frame: peak.frame * tempo,
                        bin: (f64::from(peak.bin) / pitch) as f32,
                    })
                    .collect()
            }
        }
    }

    /// The query's length in query frames, at least.
    fn frames(self, profile: &Profile) -> f64 {
        match self {
            Query::Samples(samples) => samples.len() as f64 / profile.hop as f64,
            Query::Peaks(peaks) => peaks.last().map_or(0.0, |peak| peak.frame),
        }
    }
}

/// The STFT frames, `step` query frames apart, whose peaks may lie in
/// `query_frames`, and two more either side.
fn stft_frames(query_frames: Range<f64>, step: f64) -> Range<usize> {
    let start = (query_frames.start / step).floor() - 2.0;
    let end = (query_frames.end / step).ceil() + 2.0;
    // Conversion saturates: an infinite end is `usize::MAX`.
    (start.max(0.0) as usize)..(end as usize)
}

/// Like `search_with_progress`, keeping the evidence (`explain`).
pub fn trace_with_progress(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    progress: impl Fn(usize, usize) + Sync,
) -> Trace {
    trace_with(
        index,
        samples,
        profile,
        ladder,
        jobs,
        Options::default(),
        progress,
    )
}

/// Like `trace_with_progress`, with opt-in changes to the first pass; the
/// second pass is not traced.
pub fn trace_with(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    options: Options,
    progress: impl Fn(usize, usize) + Sync,
) -> Trace {
    let (lines, chained) = lines_and_detections(
        index,
        Query::Samples(samples),
        profile,
        ladder,
        jobs,
        options,
        progress,
    );
    let (detections, chains) = chained.into_iter().unzip();
    Trace {
        detections,
        lines: lines
            .iter()
            .map(|line| WindowLine {
                asset: line.asset,
                window: line.window,
                start_seconds: profile.seconds(line.first),
                end_seconds: profile.seconds(line.last),
                track_seconds: profile.seconds(line.reference_frame_at(line.first).max(0.0)),
                speed: SpeedRatio(line.speed),
                playback: line.playback,
                hits: line.hits,
            })
            .collect(),
        chains,
    }
}

/// The distinct lines of every rung, and the detections strongest first,
/// each with the indexes of its chain's lines.
fn lines_and_detections(
    index: &Index,
    query: Query,
    profile: &Profile,
    ladder: &[Rung],
    jobs: usize,
    options: Options,
    progress: impl Fn(usize, usize) + Sync,
) -> (Vec<lines::Line>, Vec<(Detection, Vec<usize>)>) {
    let lines = lines::on_ladder(index, query, profile, ladder, jobs, progress);
    let mut detections = chains::detections(&lines, profile, options.links, options.trim_weak_ends);
    detections.sort_by_key(|(detection, _)| Reverse(detection.evidence.hits));
    let detections = strongest_per_moment(detections);
    (lines, detections)
}

/// A record plays once at any moment. Repeated sections make weaker,
/// parallel alignments of the same asset; drop every detection that
/// overlaps a stronger one of the same asset. `detections` are sorted
/// strongest first.
fn strongest_per_moment(detections: Vec<(Detection, Vec<usize>)>) -> Vec<(Detection, Vec<usize>)> {
    let mut kept: Vec<(Detection, Vec<usize>)> = Vec::new();
    for (detection, chain) in detections {
        let overlaps_stronger = kept.iter().any(|(stronger, _)| {
            stronger.asset == detection.asset
                && stronger.start_seconds < detection.end_seconds
                && detection.start_seconds < stronger.end_seconds
        });
        if !overlaps_stronger {
            kept.push((detection, chain));
        }
    }
    kept
}

/// Synthetic audio for tests of the search.
#[cfg(test)]
pub(crate) mod test_audio {
    use crate::library::{Asset, Timestamp};
    use crate::peaks::extract_peaks;
    use crate::profile::Profile;
    use crate::store::{PeakRecord, RecordHeader};

    /// Tone bursts at pseudo-random frequencies, a new pair every 50 ms.
    pub fn track(seconds: f64, rate: u32) -> Vec<f32> {
        let mut state: u64 = 2026;
        let mut next = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (state >> 33) as f64 / f64::from(1_u32 << 31)
        };
        let burst = (0.05 * f64::from(rate)) as usize;
        let mut samples = vec![0.0_f32; (seconds * f64::from(rate)) as usize];
        for chunk in samples.chunks_mut(burst) {
            let tones = [200.0 + 3300.0 * next(), 200.0 + 3300.0 * next()];
            for (i, sample) in chunk.iter_mut().enumerate() {
                let t = i as f64 / f64::from(rate);
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

    /// `samples` played at `speed` from `from` seconds, for `seconds`.
    pub fn played(samples: &[f32], speed: f64, from: f64, seconds: f64, rate: u32) -> Vec<f32> {
        let rate = f64::from(rate);
        (0..(seconds * rate) as usize)
            .map(|i| {
                let at = from * rate + i as f64 * speed;
                let (whole, fraction) = (at.floor() as usize, (at.fract()) as f32);
                samples[whole] * (1.0 - fraction) + samples[whole + 1] * fraction
            })
            .collect()
    }

    /// The peak record of `samples` as the library file `path`.
    pub fn record(path: &str, samples: &[f32]) -> PeakRecord {
        let profile = Profile::CURRENT;
        PeakRecord {
            header: RecordHeader {
                profile: profile.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 0,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: samples.len() as f64 / f64::from(profile.sample_rate),
            },
            peaks: extract_peaks(samples, &profile),
        }
    }
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
            playback: Playback::Turntable,
            evidence: Evidence::new(5, hits),
        }
    }

    #[test]
    fn stored_peaks_find_another_rip_of_the_recording() {
        let profile = Profile::CURRENT;
        let original = test_audio::track(90.0, profile.sample_rate);
        // Another rip: 0.8% fast, its first 10 s missing.
        let rip = test_audio::played(&original, 1.008, 10.0, 75.0, profile.sample_rate);
        let index = Index::build(&[test_audio::record("original.wav", &original)]).unwrap();
        let ladder: Vec<Rung> = [0.996, 1.0, 1.004, 1.008, 1.012]
            .map(|speed| Rung::Turntable(SpeedRatio(speed)))
            .to_vec();

        let found = search_peaks(
            &index,
            &test_audio::record("rip.wav", &rip).peaks,
            &profile,
            &ladder,
            1,
        );

        let best = &found[0];
        assert!((best.speed.0 - 1.008).abs() < 0.001, "{:?}", best.speed);
        assert!((best.track_start_seconds - 10.0).abs() < 1.0);
        assert!(best.end_seconds - best.start_seconds > 70.0);
    }

    #[test]
    fn a_weaker_overlapping_detection_of_the_same_asset_is_dropped() {
        let strongest_first = vec![
            detection(1, 100.0, 400.0, 900),
            detection(1, 350.0, 500.0, 300), // a repeat of a section
            detection(2, 350.0, 500.0, 300), // another asset: kept
            detection(1, 600.0, 700.0, 200), // played again later: kept
        ];

        let kept = strongest_per_moment(
            strongest_first
                .into_iter()
                .map(|found| (found, Vec::new()))
                .collect(),
        );

        let spans: Vec<(u32, f64)> = kept
            .iter()
            .map(|(found, _)| (found.asset.0, found.start_seconds))
            .collect();
        assert_eq!(spans, [(1, 100.0), (2, 350.0), (1, 600.0)]);
    }
}

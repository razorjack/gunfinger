//! The diagnostic version verifier (`docs/brief-24.md`, step 3, and
//! version 2, `docs/brief-26.md`, step 3;
//! `docs/research/2026-10-16-version-verification.md`): for each play of
//! a list, does the mix still play the reference's content along the
//! play's line where the hashes are silent? Each reported segment's line
//! is measured from the reference's own hits (`line`), as straight
//! pieces; along each piece the reference is rendered at the piece's
//! speed onto the mix's clock and compared patch by patch (`compare`):
//! supported, contradicted, or unobservable. Apart from that, each patch
//! is *distinctive* or not: a patch whose file time lies in a span of the
//! reference that a sibling (an indexed file outside the reference's
//! recording, named by the play's run) also holds supports the shared
//! passage, not the exact title.
//!
//! It decides nothing and changes no detection: the summaries are read
//! by `scripts/analysis/version_plays.py`, which applies the threshold.

mod compare;
mod line;

use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::search_peaks;
use gunfinger_core::speed::ladder;
use gunfinger_core::store::PeakRecord;
use serde::{Deserialize, Serialize};

use crate::pair::{Records, Sources};
use crate::render::{Playback, render_samples_as_indexed};
use crate::shared::Roots;
use crate::shared_mixes::LISTED_HOLDER_HITS;
use compare::{Judged, PatchState};
use line::Line;

/// The mix is decoded, and the reference rendered, this far beyond each
/// end of a segment or piece, so that a patch at the edge has its lag and
/// its dynamics.
const MARGIN_SECONDS: f64 = 1.0;
/// The siblings are searched over the play's file span widened this much.
const HELD_MARGIN_SECONDS: f64 = 10.0;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayList {
    pub plays: Vec<Play>,
}

/// One play to verify, as `scripts/analysis/version_plays.py` writes it.
/// Unknown fields are refused, so a misspelt field cannot be ignored.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Play {
    pub id: String,
    /// The mix's audio file.
    pub mix: PathBuf,
    /// The reference, by its harness name.
    pub reference: String,
    /// Indexed files outside the reference's recording that the play's
    /// run names as holders of its passage.
    #[serde(default)]
    pub siblings: Vec<String>,
    pub segments: Vec<Segment>,
    /// Passed through untouched (the play's set, outcome, role).
    #[serde(default)]
    pub meta: serde_json::Value,
}

/// One straight line: file time = `track_start + speed x (t - start)`.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Segment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub track_start_seconds: f64,
    pub speed: f64,
}

impl Segment {
    fn file_time(&self, t: f64) -> f64 {
        self.track_start_seconds + self.speed * (t - self.start_seconds)
    }
}

#[derive(Serialize)]
pub struct Verified {
    pub id: String,
    pub reference: String,
    pub meta: serde_json::Value,
    pub summaries: Summaries,
    /// Each reported segment's measured line, in order.
    pub lines: Vec<Line>,
    /// The reference's spans (file seconds) that the siblings hold.
    pub held: Vec<[f64; 2]>,
    /// One timeline per piece of the measured lines.
    pub segments: Vec<Timeline>,
    /// Siblings whose peaks were not at hand, and the like.
    pub notes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Default, Serialize)]
pub struct Summaries {
    /// S1: contradicted / (supported + contradicted) patches.
    pub s1: Option<f64>,
    /// S1 with version 1's band rules on the same patches; deciding
    /// nothing.
    pub s1_version_1_rules: Option<f64>,
    /// S2: the same over distinctive patches.
    pub s2: Option<f64>,
    /// S3: supported distinctive patches x the patch step.
    pub s3_seconds: f64,
    pub supported: usize,
    pub contradicted: usize,
    pub unobservable: usize,
    pub distinctive_supported: usize,
    pub distinctive_contradicted: usize,
    /// Patches left out because the mix or the reference is digital
    /// silence there.
    pub silent: usize,
}

#[derive(Serialize)]
pub struct Timeline {
    pub segment: Segment,
    pub patches: Vec<Patch>,
}

#[derive(Serialize)]
pub struct Patch {
    /// The mix time and the reference's file time at the patch's centre.
    pub t: f64,
    pub file: f64,
    pub lag_frames: isize,
    pub state: PatchState,
    pub version_1_state: PatchState,
    pub distinctive: bool,
    /// The low, mid and high groups' gains (dB, mix minus reference).
    pub gains: [Option<f64>; compare::GROUPS],
    /// One letter per band, lowest first: `S` supported, `C`
    /// contradicted, `K` killed, `M` masked, `?` undecided, `.` not
    /// informative.
    pub bands: String,
    pub correlations: Vec<Option<f64>>,
    pub residuals: Vec<Option<f64>>,
}

pub fn run(
    plays: &[Play],
    sources: &Sources,
    roots: &Roots,
    jobs: usize,
) -> Result<Vec<Verified>, String> {
    let records = Records::load(sources)?;
    Ok(map_in_order(plays, jobs, |play| {
        let verified = verify(play, &records, sources, roots);
        eprintln!("{}", play.id);
        verified
    }))
}

fn verify(play: &Play, records: &Records, sources: &Sources, roots: &Roots) -> Verified {
    let mut verified = Verified {
        id: play.id.clone(),
        reference: play.reference.clone(),
        meta: play.meta.clone(),
        summaries: Summaries::default(),
        lines: Vec::new(),
        held: Vec::new(),
        segments: Vec::new(),
        notes: Vec::new(),
        error: None,
    };
    let reference = match records.record(sources, &play.reference) {
        Ok(record) => record,
        Err(error) => {
            verified.error = Some(format!("no line can be measured: {error}"));
            return verified;
        }
    };
    match measured(play, &reference, roots) {
        Ok(Measurement {
            lines,
            pieces,
            silent,
        }) => {
            let tested: Vec<Segment> = pieces.iter().map(|piece| piece.line.clone()).collect();
            let held = held_spans(
                play,
                &reference,
                &tested,
                records,
                sources,
                &mut verified.notes,
            );
            let typical = compare::typical(pieces.iter().flat_map(|piece| &piece.patches));
            let timelines: Vec<Timeline> = pieces
                .iter()
                .map(|piece| Timeline {
                    segment: piece.line.clone(),
                    patches: piece
                        .patches
                        .iter()
                        .map(|patch| row(&compare::judge(patch, &typical), piece, &held))
                        .collect(),
                })
                .collect();
            verified.summaries = summarise(
                timelines.iter().flat_map(|timeline| &timeline.patches),
                silent,
            );
            verified.lines = lines;
            verified.held = held;
            verified.segments = timelines;
        }
        Err(error) => verified.error = Some(error),
    }
    verified
}

/// The play's measured lines, and each piece with its patches.
struct Measurement {
    lines: Vec<Line>,
    pieces: Vec<Piece>,
    /// Patches left out as silent.
    silent: usize,
}

/// One straight piece of a measured line, the excerpt of the mix it is
/// compared on, and its patches before judging.
struct Piece {
    line: Segment,
    clock: Clock,
    patches: Vec<compare::Measured>,
}

/// Each reported segment's line measured on its excerpt of the mix
/// against the reference's record alone, then each piece compared.
fn measured(play: &Play, reference: &PeakRecord, roots: &Roots) -> Result<Measurement, String> {
    let profile = Profile::CURRENT;
    let path = roots.path(&play.reference)?;
    let index = Index::build(std::slice::from_ref(reference))
        .map_err(|error| format!("{}: {error}", play.reference))?;
    let mut measurement = Measurement {
        lines: Vec::new(),
        pieces: Vec::new(),
        silent: 0,
    };
    for segment in &play.segments {
        let clock = Clock::around(segment);
        let mix = clock.decode(&play.mix)?;
        let line = line::measure(&mix, clock.start, segment, &index);
        for piece in &line.pieces {
            let around = Clock::around(piece);
            let mix_levels = compare::levels(&clock.cut(&mix, &around), &profile);
            let rendered = on_the_mix_clock(&path, piece, &around)?;
            let reference_levels = compare::levels(&rendered, &profile);
            let (first, end) = around.frames(piece);
            let patches = compare::measure(&mix_levels, &reference_levels, first, end);
            measurement.silent += patch_count(first, end.min(mix_levels.len())) - patches.len();
            measurement.pieces.push(Piece {
                line: piece.clone(),
                clock: around,
                patches,
            });
        }
        measurement.lines.push(line);
    }
    Ok(measurement)
}

/// How many patches fit in frames `first..end`, as `compare::measure`
/// steps through them.
fn patch_count(first: usize, end: usize) -> usize {
    if end < first + compare::PATCH_FRAMES {
        0
    } else {
        (end - first - compare::PATCH_FRAMES) / compare::STEP_FRAMES + 1
    }
}

/// The excerpt of the mix a segment or piece is compared on: from
/// `MARGIN_SECONDS` before its start (or the mix's start) to as long
/// after its end.
struct Clock {
    start: f64,
    seconds: f64,
}

impl Clock {
    fn around(segment: &Segment) -> Clock {
        let start = (segment.start_seconds - MARGIN_SECONDS).max(0.0);
        Clock {
            start,
            seconds: segment.end_seconds + MARGIN_SECONDS - start,
        }
    }

    fn samples(&self) -> usize {
        (self.seconds * f64::from(Profile::CURRENT.sample_rate)).round() as usize
    }

    /// The mix's samples on this clock, silent past the mix's end.
    fn decode(&self, mix: &Path) -> Result<Vec<f32>, String> {
        let decoded = decode(
            mix,
            Profile::CURRENT.sample_rate,
            Excerpt {
                start: Some(Duration::from_secs_f64(self.start)),
                duration: Some(Duration::from_secs_f64(self.seconds)),
            },
        )
        .map_err(|error| format!("{}: {error}", mix.display()))?;
        let mut samples = decoded.samples;
        samples.resize(self.samples(), 0.0);
        Ok(samples)
    }

    /// The part of `samples`, on this clock, that `inner` covers; `inner`
    /// starts no earlier than this clock. Silent past the end.
    fn cut(&self, samples: &[f32], inner: &Clock) -> Vec<f32> {
        let rate = f64::from(Profile::CURRENT.sample_rate);
        let from = (((inner.start - self.start) * rate).round() as usize).min(samples.len());
        let mut part: Vec<f32> = samples[from..]
            .iter()
            .copied()
            .take(inner.samples())
            .collect();
        part.resize(inner.samples(), 0.0);
        part
    }

    /// The excerpt's frames whose start lies within the segment.
    fn frames(&self, segment: &Segment) -> (usize, usize) {
        let profile = Profile::CURRENT;
        let first = profile.frames(segment.start_seconds - self.start).ceil() as usize;
        let end = profile.frames(segment.end_seconds - self.start).floor() as usize + 1;
        (first, end)
    }

    /// The mix time of frame `frame` of the excerpt.
    fn time(&self, frame: f64) -> f64 {
        self.start + Profile::CURRENT.seconds(frame)
    }
}

/// The reference along the segment's line for the excerpt's samples,
/// rendered as turntable playback at the line's speed; silent where the
/// file has no audio.
fn on_the_mix_clock(path: &Path, segment: &Segment, clock: &Clock) -> Result<Vec<f32>, String> {
    let rate = f64::from(Profile::CURRENT.sample_rate);
    let at_start = segment.file_time(clock.start);
    let lead = if at_start < 0.0 {
        (-at_start / segment.speed * rate).round() as usize
    } else {
        0
    };
    let length = clock.samples();
    let resample = format!("aresample={}", Profile::CURRENT.sample_rate);
    let seconds = clock.seconds - lead as f64 / rate;
    let mut samples = vec![0.0; lead.min(length)];
    if seconds > 0.0 {
        samples.extend(render_samples_as_indexed(
            path,
            at_start.max(0.0),
            seconds,
            Playback::Turntable(segment.speed),
            Some(&resample),
        )?);
    }
    samples.resize(length, 0.0);
    Ok(samples)
}

fn row(judged: &Judged, piece: &Piece, held: &[[f64; 2]]) -> Patch {
    let centre = piece
        .clock
        .time(judged.first as f64 + compare::PATCH_FRAMES as f64 / 2.0);
    let file = piece.line.file_time(centre);
    Patch {
        t: rounded(centre),
        file: rounded(file),
        lag_frames: judged.lag,
        state: judged.state,
        version_1_state: judged.version_1_state,
        distinctive: !held.iter().any(|[from, to]| (*from..=*to).contains(&file)),
        gains: judged.gains.map(|gain| gain.map(rounded)),
        bands: judged.bands.iter().map(|state| state.letter()).collect(),
        correlations: judged.correlations.iter().map(|r| r.map(rounded)).collect(),
        residuals: judged.residuals.iter().map(|e| e.map(rounded)).collect(),
    }
}

fn rounded(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

/// The play's summaries over its judged patches.
fn summarise<'a>(patches: impl Iterator<Item = &'a Patch>, silent: usize) -> Summaries {
    let mut summaries = Summaries {
        silent,
        ..Summaries::default()
    };
    let (mut version_1_supported, mut version_1_contradicted) = (0, 0);
    for patch in patches {
        match patch.state {
            PatchState::Supported => {
                summaries.supported += 1;
                summaries.distinctive_supported += usize::from(patch.distinctive);
            }
            PatchState::Contradicted => {
                summaries.contradicted += 1;
                summaries.distinctive_contradicted += usize::from(patch.distinctive);
            }
            PatchState::Unobservable => summaries.unobservable += 1,
        }
        match patch.version_1_state {
            PatchState::Supported => version_1_supported += 1,
            PatchState::Contradicted => version_1_contradicted += 1,
            PatchState::Unobservable => {}
        }
    }
    summaries.s1 = share(summaries.contradicted, summaries.supported);
    summaries.s1_version_1_rules = share(version_1_contradicted, version_1_supported);
    summaries.s2 = share(
        summaries.distinctive_contradicted,
        summaries.distinctive_supported,
    );
    summaries.s3_seconds = rounded(
        summaries.distinctive_supported as f64
            * Profile::CURRENT.seconds(compare::STEP_FRAMES as f64),
    );
    summaries
}

fn share(contradicted: usize, supported: usize) -> Option<f64> {
    let decided = contradicted + supported;
    (decided > 0).then(|| contradicted as f64 / decided as f64)
}

/// The reference's spans that its siblings hold: its stored peaks over
/// the tested pieces' file span, widened, searched on the turntable
/// ladder against the siblings alone; each detection of at least
/// `LISTED_HOLDER_HITS` hits holds the span it covers.
fn held_spans(
    play: &Play,
    reference: &PeakRecord,
    tested: &[Segment],
    records: &Records,
    sources: &Sources,
    notes: &mut Vec<String>,
) -> Vec<[f64; 2]> {
    if play.siblings.is_empty() || tested.is_empty() {
        return Vec::new();
    }
    let siblings: Vec<PeakRecord> = play
        .siblings
        .iter()
        .filter_map(|sibling| match records.record(sources, sibling) {
            Ok(record) => Some(record),
            Err(error) => {
                notes.push(error);
                None
            }
        })
        .collect();
    let index = match Index::build(&siblings) {
        Ok(index) if !siblings.is_empty() => index,
        Ok(_) => return Vec::new(),
        Err(error) => {
            notes.push(format!("held spans unknown: {error}"));
            return Vec::new();
        }
    };
    let (from, to) = file_span(tested);
    let profile = Profile::CURRENT;
    let span =
        profile.frames(from - HELD_MARGIN_SECONDS)..=profile.frames(to + HELD_MARGIN_SECONDS);
    let passage: Vec<_> = reference
        .peaks
        .iter()
        .filter(|peak| span.contains(&peak.frame))
        .copied()
        .collect();
    let mut held: Vec<[f64; 2]> = search_peaks(&index, &passage, &profile, &ladder(), 1)
        .iter()
        .filter(|found| found.evidence.hits >= LISTED_HOLDER_HITS)
        .map(|found| [rounded(found.start_seconds), rounded(found.end_seconds)])
        .collect();
    held.sort_by(|a, b| a[0].total_cmp(&b[0]));
    held
}

/// The earliest and latest file times the segments reach.
fn file_span(segments: &[Segment]) -> (f64, f64) {
    segments
        .iter()
        .flat_map(|segment| {
            [
                segment.track_start_seconds,
                segment.file_time(segment.end_seconds),
            ]
        })
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), time| {
            (low.min(time), high.max(time))
        })
}

pub fn print_summary(verified: &[Verified]) {
    for play in verified {
        let summaries = &play.summaries;
        let share =
            |value: Option<f64>| value.map_or_else(|| String::from("  -  "), |v| format!("{v:.3}"));
        let pieces: usize = play.lines.iter().map(|line| line.pieces.len()).sum();
        println!(
            "{:<60} S1 {}  S2 {}  S3 {:>6.1} s  ({} supported, {} contradicted, {} unobservable; {} pieces){}",
            play.id,
            share(summaries.s1),
            share(summaries.s2),
            summaries.s3_seconds,
            summaries.supported,
            summaries.contradicted,
            summaries.unobservable,
            pieces,
            play.error
                .as_ref()
                .map_or_else(String::new, |error| format!("  error: {error}"))
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::audio::tests::tone;

    fn segment(start: f64, end: f64, track_start: f64, speed: f64) -> Segment {
        Segment {
            start_seconds: start,
            end_seconds: end,
            track_start_seconds: track_start,
            speed,
        }
    }

    fn patch(state: PatchState, distinctive: bool) -> Patch {
        Patch {
            t: 0.0,
            file: 0.0,
            lag_frames: 0,
            state,
            version_1_state: state,
            distinctive,
            gains: [None; compare::GROUPS],
            bands: String::new(),
            correlations: Vec::new(),
            residuals: Vec::new(),
        }
    }

    #[test]
    fn the_line_maps_mix_time_to_file_time() {
        let line = segment(100.0, 160.0, 20.0, 1.05);

        assert!((line.file_time(100.0) - 20.0).abs() < 1e-12);
        assert!((line.file_time(110.0) - 30.5).abs() < 1e-12);
        assert_eq!(
            file_span(&[line, segment(170.0, 180.0, 90.0, 1.0)]),
            (20.0, 100.0)
        );
    }

    #[test]
    fn the_clock_reaches_a_second_beyond_the_segment_but_not_before_the_mix() {
        let clock = Clock::around(&segment(30.0, 90.0, 0.0, 1.0));
        assert_eq!((clock.start, clock.seconds), (29.0, 62.0));
        assert_eq!(clock.samples(), 496_000);
        // The segment's frames start 1 s (62.5 frames) into the excerpt.
        assert_eq!(clock.frames(&segment(30.0, 90.0, 0.0, 1.0)), (63, 3813));

        let early = Clock::around(&segment(0.4, 20.0, 0.0, 1.0));
        assert_eq!(early.start, 0.0);
        assert!((early.seconds - 21.0).abs() < 1e-12);
    }

    #[test]
    fn a_pieces_excerpt_is_cut_from_its_segments_on_the_same_clock() {
        let outer = Clock::around(&segment(30.0, 40.0, 0.0, 1.0));
        // Sample n of the segment's excerpt holds n.
        let samples: Vec<f32> = (0..outer.samples()).map(|n| n as f32).collect();
        let inner = Clock::around(&segment(35.0, 41.0, 0.0, 1.0));

        let part = outer.cut(&samples, &inner);

        assert_eq!(part.len(), inner.samples());
        // The piece's excerpt starts 34 s in, 5 s after the segment's.
        assert_eq!(part[0], 40_000.0);
        assert_eq!(part[1], 40_001.0);
        // It reaches past the segment's excerpt, which ends at 41 s, 7 s
        // in: silent from there.
        assert_eq!(part[55_999], 95_999.0);
        assert_eq!(part[56_000], 0.0);
        assert_eq!(part[inner.samples() - 1], 0.0);
    }

    #[test]
    fn patches_are_counted_as_measure_steps_through_them() {
        assert_eq!(patch_count(0, 124), 0);
        assert_eq!(patch_count(0, 125), 1);
        assert_eq!(patch_count(0, 140), 1);
        assert_eq!(patch_count(0, 141), 2);
    }

    #[test]
    fn the_summaries_count_decided_patches_and_distinctive_ones_apart() {
        let patches = [
            patch(PatchState::Supported, true),
            patch(PatchState::Supported, false),
            patch(PatchState::Supported, false),
            patch(PatchState::Contradicted, true),
            patch(PatchState::Unobservable, true),
        ];

        let summaries = summarise(patches.iter(), 4);

        assert_eq!(summaries.s1, Some(0.25));
        assert_eq!(summaries.s2, Some(0.5));
        assert!((summaries.s3_seconds - 0.26).abs() < 1e-9);
        assert_eq!(
            (
                summaries.supported,
                summaries.contradicted,
                summaries.unobservable,
                summaries.silent
            ),
            (3, 1, 1, 4)
        );
    }

    #[test]
    fn a_play_without_a_decided_patch_has_no_share() {
        let summaries = summarise([patch(PatchState::Unobservable, true)].iter(), 0);

        assert_eq!(summaries.s1, None);
        assert_eq!(summaries.s2, None);
    }

    /// Zero crossings per second of a tone, twice its frequency.
    fn crossings_per_second(samples: &[f32]) -> f64 {
        let crossings = samples
            .windows(2)
            .filter(|pair| (pair[0] < 0.0) != (pair[1] < 0.0))
            .count();
        crossings as f64 / (samples.len() as f64 / 8000.0)
    }

    #[test]
    fn the_reference_is_rendered_at_the_fitted_speed_on_the_mix_clock() {
        let dir = std::env::temp_dir().join(format!("gunfinger-versions-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("tone.mp3");
        if tone(&file, 8.0, 500).is_none() {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        // The file's 0.5 s is heard at mix time 10 s, 5% fast: the
        // excerpt starts at 9 s, 1.05 s of file before the file's start.
        let line = segment(10.0, 13.0, 0.5, 1.05);
        let clock = Clock::around(&line);

        let rendered = on_the_mix_clock(&file, &line, &clock).unwrap();

        assert_eq!(rendered.len(), clock.samples());
        let lead = (0.55 / 1.05 * 8000.0_f64).round() as usize;
        assert!(rendered[..lead - 400].iter().all(|&x| x == 0.0));
        let heard = &rendered[lead + 2000..lead + 18_000];
        let frequency = crossings_per_second(heard) / 2.0;
        assert!((frequency - 525.0).abs() < 5.0, "{frequency}");
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_line_into_the_file_puts_its_onset_where_the_line_says() {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-versions-onset-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("onset.wav");
        let generated = std::process::Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("aevalsrc=if(gte(t\\,3)\\,sin(2*PI*500*t)\\,0):s=8000:d=8")
            .arg(&file)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        // File time 2 s is heard at mix time 20 s, 5% fast; the excerpt
        // starts at 19 s, file time 0.95 s, and the tone's onset at file
        // time 3 s falls 1 s + 1 / 1.05 s into it.
        let line = segment(20.0, 24.0, 2.0, 1.05);
        let clock = Clock::around(&line);

        let rendered = on_the_mix_clock(&file, &line, &clock).unwrap();

        let onset = ((1.0 + 1.0 / 1.05) * 8000.0_f64).round() as usize;
        let loud = |samples: &[f32]| samples.iter().any(|x| x.abs() > 0.1);
        assert!(!loud(&rendered[onset - 2000..onset - 40]));
        assert!(loud(&rendered[onset + 40..onset + 400]));
        std::fs::remove_dir_all(&dir).ok();
    }
}

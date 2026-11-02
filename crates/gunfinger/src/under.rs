//! `identify --under` (ADR 0025): a quieter track under a confident one.
//! For each confident entry of the tracklist, its strongest play's file is
//! rendered where that play's segments place it, taken out of the
//! recording with the core's mask (`residual`, session 70's *Align*), and
//! what is left is searched again under today's confidence rule. A
//! detection there is a play under the entry unless its file is one of the
//! entry's, shares material with the file taken out, or is already
//! confident nearby in the recording (`Dropped`).

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{DecodeError, Decoder, Played};
use gunfinger_core::index::{AssetId, Index};
use gunfinger_core::peaks::Peak;
use gunfinger_core::plays::{self, MAX_GAP_SECONDS};
use gunfinger_core::profile::Profile;
use gunfinger_core::residual::{Mask, residual};
use gunfinger_core::search::{Detection, search_peaks, search_with};
use gunfinger_core::speed::{self, Rung};

use crate::catalog::Catalog;
use crate::console::Console;
use crate::export::{Entry, entries};
use crate::report::{FoundPlay, Level, Playback, Report, SharedWith};

/// The step as a report's search settings record it.
pub fn design() -> String {
    format!(
        "{}; reference: the strongest play of each confident entry",
        Mask::UNDER.design()
    )
}

/// The plays found under the report's confident entries in `samples`, the
/// audio the report was made from, in order of start time.
pub fn find(
    report: &Report,
    catalog: &Catalog,
    samples: &[f32],
    ladder: &[Rung],
    jobs: usize,
    console: &Console,
) -> Vec<FoundPlay> {
    let started = Instant::now();
    let confident: Vec<Entry> = entries(report)
        .into_iter()
        .filter(|entry| entry.confidence() == Level::Confident)
        .collect();
    let mut found = Vec::new();
    for entry in &confident {
        let Some(taken_out) = TakenOut::of(report, entry) else {
            continue;
        };
        match under_entry(report, catalog, samples, &taken_out, ladder, jobs) {
            Ok(plays) => found.extend(plays),
            Err(error) => console.warning(format_args!(
                "--under: {} is not taken out of the recording, so nothing is searched under it: {error}",
                taken_out.play.asset
            )),
        }
    }
    console.detail(format_args!(
        "searched under {} confident tracks in {:.1} s",
        confident.len(),
        started.elapsed().as_secs_f64()
    ));
    once_each(found)
}

/// The play of a confident entry whose file is taken out: the entry's
/// strongest (entries put confident plays first, most hits first), the
/// rip of the track that the recording's audio supports best.
struct TakenOut<'a> {
    play: &'a FoundPlay,
    /// Its number in the report, from 1.
    number: usize,
    /// Every file of the entry: its plays' and their same-audio copies.
    files: Vec<&'a str>,
}

impl<'a> TakenOut<'a> {
    fn of(report: &'a Report, entry: &Entry<'a>) -> Option<TakenOut<'a>> {
        let play = *entry.plays.first()?;
        let number = report
            .plays
            .iter()
            .position(|other| std::ptr::eq(other, play))?
            + 1;
        let files = entry
            .plays
            .iter()
            .flat_map(|play| {
                std::iter::once(play.asset.as_str())
                    .chain(play.same_audio.iter().map(String::as_str))
            })
            .collect();
        Some(TakenOut {
            play,
            number,
            files,
        })
    }
}

fn under_entry(
    report: &Report,
    catalog: &Catalog,
    samples: &[f32],
    taken_out: &TakenOut,
    ladder: &[Rung],
    jobs: usize,
) -> Result<Vec<FoundPlay>, String> {
    let profile = Profile::CURRENT;
    let rate = f64::from(profile.sample_rate);
    let at = |seconds: f64| {
        (((seconds - report.query.start_seconds) * rate)
            .round()
            .max(0.0) as usize)
            .min(samples.len())
    };
    let (first, last) = (
        at(taken_out.play.start_seconds),
        at(taken_out.play.end_seconds),
    );
    if last <= first {
        return Ok(Vec::new());
    }
    let excerpt = &samples[first..last];
    let offset = report.query.start_seconds + first as f64 / rate;
    let file = catalog.root.join(&taken_out.play.asset);
    let reference = placed(taken_out.play, &file, offset, excerpt.len(), &profile)
        .map_err(|error| error.to_string())?;
    let left = residual(excerpt, &reference, &profile, Mask::UNDER);
    let detections = search_with(
        &catalog.index,
        &left.samples,
        &profile,
        ladder,
        jobs,
        catalog.matcher.options(),
    );
    let shared = SharedMaterial::new(catalog, &taken_out.play.asset, ladder, jobs)?;
    let mut verdicts: BTreeMap<AssetId, bool> = BTreeMap::new();
    let kept: Vec<Detection> = detections
        .into_iter()
        .filter(|detection| detection.evidence.confidence() >= Confidence::Possible)
        .filter(|detection| {
            let path = &catalog.index.asset(detection.asset).path;
            let start = offset + detection.start_seconds;
            let end = offset + detection.end_seconds;
            let dropped = Dropped::of(path, start, end, taken_out, &report.plays).is_some()
                || *verdicts
                    .entry(detection.asset)
                    .or_insert_with(|| shared.with(catalog, path));
            !dropped
        })
        .collect();
    Ok(plays::merge_same_audio(plays::group(&kept))
        .iter()
        .map(|same| FoundPlay {
            under: Some(SharedWith {
                play: taken_out.number,
                asset: taken_out.play.asset.clone(),
            }),
            ..FoundPlay::new(catalog, same, offset)
        })
        .collect())
}

/// `play`'s file as the recording plays it over `length` samples from
/// `offset` seconds: each segment rendered by its own line, strongest
/// first where segments overlap; silence outside the segments.
fn placed(
    play: &FoundPlay,
    file: &Path,
    offset: f64,
    length: usize,
    profile: &Profile,
) -> Result<Vec<f32>, DecodeError> {
    let rate = f64::from(profile.sample_rate);
    let mut audio = vec![0.0_f32; length];
    let mut written = vec![false; length];
    let mut segments: Vec<_> = play.segments.iter().collect();
    segments.sort_by_key(|segment| std::cmp::Reverse(segment.hits));
    for segment in segments {
        let rendered = Decoder::default().render(
            file,
            profile.sample_rate,
            Played {
                start_seconds: segment.track_start_seconds,
                seconds: segment.end_seconds - segment.start_seconds,
                speed: segment.speed,
                playback: match segment.playback {
                    Playback::Turntable => speed::Playback::Turntable,
                    Playback::KeyLocked => speed::Playback::KeyLocked,
                },
            },
        )?;
        let start = ((segment.start_seconds - offset) * rate).round() as isize;
        for (index, &sample) in rendered.samples.iter().enumerate() {
            let Ok(at) = usize::try_from(start + index as isize) else {
                continue;
            };
            if at < length && !written[at] {
                audio[at] = sample;
                written[at] = true;
            }
        }
    }
    Ok(audio)
}

/// Why a detection in what is left is not a play under the entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dropped {
    /// Its file is one of the entry's: what the mask left of the track.
    TakenOut,
    /// A confident play of its file overlaps it or lies within
    /// `MAX_GAP_SECONDS` of it: the tracklist names the track there
    /// already, as when the quieter track plays on alone.
    NamedNearby,
}

impl Dropped {
    /// The rules that need only the report; `SharedMaterial` is the third.
    fn of(
        path: &str,
        start: f64,
        end: f64,
        taken_out: &TakenOut,
        plays: &[FoundPlay],
    ) -> Option<Dropped> {
        if taken_out.files.contains(&path) {
            return Some(Dropped::TakenOut);
        }
        plays
            .iter()
            .any(|play| {
                play.confidence == Level::Confident
                    && (play.asset == path || play.same_audio.iter().any(|copy| copy == path))
                    && play.start_seconds - MAX_GAP_SECONDS <= end
                    && start <= play.end_seconds + MAX_GAP_SECONDS
            })
            .then_some(Dropped::NamedNearby)
    }
}

/// Whether a file shares material with the file taken out, by their stored
/// peaks: the file's peaks searched against an index of the taken-out
/// file alone give a detection at the possible tier or above, which ADR
/// 0006 reads as "this recording, or one sharing material with it". Other
/// rips of the track and related versions with a passage in common are
/// held back this way (the user's "related versions not printed at
/// first"), and so may tracks built on the same sampled break.
struct SharedMaterial<'a> {
    taken_out: Index,
    ladder: &'a [Rung],
    jobs: usize,
}

impl<'a> SharedMaterial<'a> {
    fn new(
        catalog: &Catalog,
        path: &str,
        ladder: &'a [Rung],
        jobs: usize,
    ) -> Result<SharedMaterial<'a>, String> {
        let record = catalog
            .sources
            .get(path)
            .ok_or_else(|| String::from("its peak record is not indexed"))
            .and_then(|asset| {
                catalog
                    .store
                    .load(asset, &Profile::CURRENT)
                    .map_err(|error| error.to_string())
            })?;
        let taken_out =
            Index::build(std::slice::from_ref(&record)).map_err(|error| error.to_string())?;
        Ok(SharedMaterial {
            taken_out,
            ladder,
            jobs,
        })
    }

    /// Whether `path` shares material with the file taken out; a file
    /// whose peaks cannot be read is held back too.
    fn with(&self, catalog: &Catalog, path: &str) -> bool {
        let Some(asset) = catalog.sources.get(path) else {
            return true;
        };
        match catalog.store.load(asset, &Profile::CURRENT) {
            Ok(record) => self.shares(&record.peaks),
            Err(_) => true,
        }
    }

    fn shares(&self, peaks: &[Peak]) -> bool {
        search_peaks(
            &self.taken_out,
            peaks,
            &Profile::CURRENT,
            self.ladder,
            self.jobs,
        )
        .iter()
        .any(|detection| detection.evidence.confidence() >= Confidence::Possible)
    }
}

/// The plays in order of start time, a play overlapping a stronger one of
/// the same file found under another entry left out.
fn once_each(mut found: Vec<FoundPlay>) -> Vec<FoundPlay> {
    found.sort_by_key(|play| std::cmp::Reverse(play.hits));
    let mut kept: Vec<FoundPlay> = Vec::new();
    for play in found {
        let repeated = kept.iter().any(|other| {
            other.asset == play.asset
                && other.start_seconds < play.end_seconds
                && play.start_seconds < other.end_seconds
        });
        if !repeated {
            kept.push(play);
        }
    }
    kept.sort_by(|a, b| a.start_seconds.total_cmp(&b.start_seconds));
    kept
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::{ReportFormat, render};
    use crate::style::{ColorChoice, Style};
    use crate::table::tests::report;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::peaks::extract_peaks;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    #[test]
    fn the_file_taken_out_is_the_strongest_confident_play_of_its_entry() {
        let mut report = report();
        // A weaker rip of a.wav over the same passage, lining up with it:
        // one entry with a.wav.
        let rip = FoundPlay {
            asset: String::from("rip-of-a.wav"),
            same_audio: Vec::new(),
            hits: 400,
            ..report.plays[0].clone()
        };
        report.plays.insert(1, rip);
        let found = entries(&report);
        let taken_out = TakenOut::of(&report, &found[0]).unwrap();
        assert_eq!(taken_out.play.asset, "a.wav");
        assert_eq!(taken_out.number, 1);
        assert_eq!(taken_out.files, ["a.wav", "copy-of-a.wav", "rip-of-a.wav"]);

        // Stronger now: the rip is taken out, under its own number.
        report.plays[1].hits = 900;
        let found = entries(&report);
        let taken_out = TakenOut::of(&report, &found[0]).unwrap();
        assert_eq!(taken_out.play.asset, "rip-of-a.wav");
        assert_eq!(taken_out.number, 2);
    }

    #[test]
    fn what_is_left_of_the_track_and_tracks_named_nearby_are_dropped() {
        let report = report();
        let found = entries(&report);
        // a.wav plays 0-41 s; b.wav is confident from 38 to 81 s.
        let taken_out = TakenOut::of(&report, &found[0]).unwrap();
        let dropped = |path: &str, start: f64, end: f64| {
            Dropped::of(path, start, end, &taken_out, &report.plays)
        };
        assert_eq!(dropped("a.wav", 5.0, 30.0), Some(Dropped::TakenOut));
        assert_eq!(dropped("copy-of-a.wav", 5.0, 30.0), Some(Dropped::TakenOut));
        // b.wav from 10 s: 28 s before its confident play.
        assert_eq!(dropped("b.wav", 10.0, 30.0), Some(Dropped::NamedNearby));
        // c.wav is confident from 96 s: 91 s after this detection's end.
        assert_eq!(dropped("c.wav", 0.0, 5.0), None);
        assert_eq!(dropped("c.wav", 0.0, 6.0), Some(Dropped::NamedNearby));
        // insert.wav is only possible in the recording.
        assert_eq!(dropped("insert.wav", 10.0, 30.0), None);
        assert_eq!(dropped("d.wav", 10.0, 30.0), None);
    }

    /// Tone bursts at pseudo-random frequencies from `seed`, a new pair
    /// every 50 ms, at the profile's rate.
    fn track(seed: u64, seconds: f64) -> Vec<f32> {
        let rate = f64::from(Profile::CURRENT.sample_rate);
        let mut state = seed;
        let mut next = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (state >> 33) as f64 / f64::from(1_u32 << 31)
        };
        let burst = (0.05 * rate) as usize;
        let mut samples = vec![0.0_f32; (seconds * rate) as usize];
        for chunk in samples.chunks_mut(burst) {
            let tones = [200.0 + 3300.0 * next(), 200.0 + 3300.0 * next()];
            for (i, sample) in chunk.iter_mut().enumerate() {
                let t = i as f64 / rate;
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

    fn record(path: &str, samples: &[f32]) -> PeakRecord {
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

    #[test]
    fn a_file_with_a_passage_of_the_track_taken_out_shares_material_and_another_does_not() {
        let a = track(1, 60.0);
        let ladder = speed::ladder();
        let shared = SharedMaterial {
            taken_out: Index::build(&[record("a.wav", &a)]).unwrap(),
            ladder: &ladder,
            jobs: 2,
        };
        // A remix: 30 s of other material, then 30 s of A.
        let mut remix = track(2, 30.0);
        remix.extend_from_slice(&a[..a.len() / 2]);
        assert!(shared.shares(&record("remix.wav", &remix).peaks));
        assert!(!shared.shares(&record("other.wav", &track(3, 60.0)).peaks));
    }

    fn partner(asset: &str, start: f64, end: f64, hits: u32) -> FoundPlay {
        FoundPlay {
            asset: String::from(asset),
            start_seconds: start,
            end_seconds: end,
            hits,
            ..report().plays[0].clone()
        }
    }

    #[test]
    fn a_play_found_under_two_entries_is_kept_once_the_stronger() {
        let kept = once_each(vec![
            partner("x.wav", 50.0, 70.0, 300),
            partner("x.wav", 30.0, 55.0, 250),
            partner("y.wav", 10.0, 20.0, 90),
            partner("x.wav", 200.0, 220.0, 100),
        ]);
        let summary: Vec<(&str, f64, u32)> = kept
            .iter()
            .map(|play| (play.asset.as_str(), play.start_seconds, play.hits))
            .collect();
        assert_eq!(
            summary,
            [
                ("y.wav", 10.0, 90),
                ("x.wav", 50.0, 300),
                ("x.wav", 200.0, 100)
            ]
        );
    }

    /// The fixture's report (a.wav, b.wav, insert.wav possible, c.wav) with
    /// x.wav found confidently under a.wav (play 1) and y.wav found as
    /// possible under c.wav (play 4).
    fn with_plays_under() -> Report {
        let mut report = report();
        let under = |asset: &str, start, end, confidence, play: usize| FoundPlay {
            asset: String::from(asset),
            same_audio: Vec::new(),
            start_seconds: start,
            end_seconds: end,
            confidence,
            hits: 300,
            segments: vec![crate::report::Segment {
                start_seconds: start,
                end_seconds: end,
                confidence,
                hits: 300,
                ..report.plays[0].segments[0].clone()
            }],
            under: Some(SharedWith {
                play,
                asset: report.plays[play - 1].asset.clone(),
            }),
            ..report.plays[0].clone()
        };
        report.under = Some(vec![
            under("x.wav", 10.0, 35.0, Level::Confident, 1),
            under("y.wav", 100.0, 120.0, Level::Possible, 4),
        ]);
        report
    }

    fn rendered(report: &Report, format: ReportFormat) -> String {
        render(report, format, Style::for_stdout(ColorChoice::Never)).unwrap()
    }

    #[test]
    fn a_confident_play_found_under_another_is_a_tracklist_entry_marked_under_it() {
        assert_eq!(
            rendered(&with_plays_under(), ReportFormat::Tracklist),
            " 1.    0:00  a\n 2.    0:10  x (under 1.)\n 3.    0:38  b\n 4.    1:24  insert (possible)\n 5.    1:36  c\n"
        );
    }

    #[test]
    fn the_table_lists_every_play_found_under_another_in_time_order() {
        let table = rendered(&with_plays_under(), ReportFormat::Human);
        let lines: Vec<&str> = table.lines().collect();
        let row = |asset: &str| lines.iter().position(|line| line.contains(asset)).unwrap();
        assert!(
            lines[row("x.wav")].ends_with("x.wav  under a.wav (play 1)"),
            "{table}"
        );
        assert!(
            lines[row("y.wav")].ends_with("y.wav  under c.wav (play 4)"),
            "{table}"
        );
        assert!(
            row("a.wav") < row("x.wav") && row("x.wav") < row("b.wav"),
            "{table}"
        );
        assert!(row("c.wav") < row("y.wav"), "{table}");
    }

    #[test]
    fn csv_gets_a_column_for_the_play_under_which_a_play_was_found_only_when_searched() {
        let csv = rendered(&with_plays_under(), ReportFormat::Csv);
        let lines: Vec<&str> = csv.lines().collect();
        assert!(lines[0].ends_with(",shares_material_with_play,under_play"));
        assert_eq!(lines.len(), 7);
        assert!(
            lines[1].contains(",a.wav,") && lines[1].ends_with(","),
            "{csv}"
        );
        assert!(
            lines[2].contains(",x.wav,") && lines[2].ends_with(",1"),
            "{csv}"
        );
        assert!(
            lines[6].contains(",y.wav,") && lines[6].ends_with(",4"),
            "{csv}"
        );

        let mut searched = report();
        searched.under = Some(Vec::new());
        let plain = rendered(&report(), ReportFormat::Csv);
        assert!(
            plain
                .lines()
                .next()
                .unwrap()
                .ends_with(",shares_material_with_play")
        );
        let empty = rendered(&searched, ReportFormat::Csv);
        assert_eq!(empty.lines().count(), plain.lines().count());
        assert!(
            empty
                .lines()
                .all(|line| line.ends_with(',') || line.ends_with("under_play"))
        );
    }

    #[test]
    fn cue_sheets_and_timelines_leave_plays_found_under_others_out() {
        for format in [ReportFormat::Cue, ReportFormat::Timeline] {
            assert_eq!(
                rendered(&with_plays_under(), format),
                rendered(&report(), format)
            );
        }
    }

    #[test]
    fn a_saved_report_keeps_the_plays_found_under_others_and_shows_them_again() {
        let found = with_plays_under();
        let json = rendered(&found, ReportFormat::Json);
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(
            value["under"][0]["under"],
            serde_json::json!({ "play": 1, "asset": "a.wav" })
        );
        assert!(value["plays"][0].get("under").is_none());
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../work/tests/under-saved");
        std::fs::create_dir_all(&dir).unwrap();
        let saved = dir.join("mix.json");
        std::fs::write(&saved, &json).unwrap();
        let read = Report::read(&saved).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        for format in [
            ReportFormat::Tracklist,
            ReportFormat::Human,
            ReportFormat::Csv,
        ] {
            assert_eq!(rendered(&read, format), rendered(&found, format));
        }
        // Without the step, the report has no `under` at all.
        let plain = rendered(&report(), ReportFormat::Json);
        assert!(!plain.contains("\"under\""));
    }

    #[test]
    fn a_play_found_under_a_play_the_report_does_not_hold_is_rejected() {
        let mut report = with_plays_under();
        if let Some(under) = report.under.as_mut() {
            under[0].under = Some(SharedWith {
                play: 9,
                asset: String::from("a.wav"),
            });
        }
        let json = rendered(&report, ReportFormat::Json);
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../work/tests/under-invalid");
        std::fs::create_dir_all(&dir).unwrap();
        let saved = dir.join("mix.json");
        std::fs::write(&saved, &json).unwrap();
        let read = Report::read(&saved);
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(read.is_err());
    }
}

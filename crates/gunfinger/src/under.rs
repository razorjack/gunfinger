//! `identify --under` (ADR 0025): a quieter track under a confident one.
//! For each confident entry of the tracklist, its strongest play's file is
//! rendered where that play's segments place it, taken out of the
//! recording with the core's mask (`residual`, session 70's *Align*), and
//! what is left is searched again under today's confidence rule. A
//! detection there is a play under the entry unless its file is one of the
//! entry's, shares material with the file taken out, or is already
//! confident nearby in the recording (`Dropped`); a play found there is
//! dropped when it is one track with a confident play of the recording
//! (`named_as_one_track`). The entries' decodes and masks run on the
//! workers, one entry each; their residuals are then searched in entry
//! order, each search on the workers, so the output does not depend on
//! the number of workers.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{DecodeError, Decoder, Played};
use gunfinger_core::index::{AssetId, Index};
use gunfinger_core::parallel;
use gunfinger_core::peaks::Peak;
use gunfinger_core::plays::{self, MAX_GAP_SECONDS};
use gunfinger_core::profile::Profile;
use gunfinger_core::residual::{Mask, residual};
use gunfinger_core::search::{Detection, search_peaks, search_with};
use gunfinger_core::speed::{self, Rung};

use crate::catalog::Catalog;
use crate::console::Console;
use crate::export::{Entry, entries};
use crate::report::{FoundPlay, Level, PASSAGE_GAP_SECONDS, Playback, Report, SharedWith};

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
    let taken_out: Vec<TakenOut> = confident
        .iter()
        .filter_map(|entry| TakenOut::of(report, entry))
        .collect();
    let residuals = parallel::map_in_order(&taken_out, jobs, |taken_out| {
        Left::of(report, catalog, samples, taken_out)
    });
    let mut found = Vec::new();
    for (taken_out, left) in taken_out.iter().zip(residuals) {
        let plays = left.and_then(|left| match left {
            Some(left) => under_entry(report, catalog, &left, taken_out, ladder, jobs),
            None => Ok(Vec::new()),
        });
        match plays {
            Ok(plays) => found.extend(plays),
            Err(error) => console.warning(format_args!(
                "--under: nothing is reported under {}: {error}",
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

/// What is left of the recording over an entry's strongest play once its
/// file is taken out: the residual of the play's span, and the recording's
/// time at its first sample.
struct Left {
    samples: Vec<f32>,
    offset: f64,
}

impl Left {
    /// The residual over `taken_out`'s play, none when the play spans no
    /// sample of `samples`. One thread: the decode of the play's file and
    /// the mask.
    fn of(
        report: &Report,
        catalog: &Catalog,
        samples: &[f32],
        taken_out: &TakenOut,
    ) -> Result<Option<Left>, String> {
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
            return Ok(None);
        }
        let excerpt = &samples[first..last];
        let offset = report.query.start_seconds + first as f64 / rate;
        let file = catalog.root.join(&taken_out.play.asset);
        let reference = placed(taken_out.play, &file, offset, excerpt.len(), &profile)
            .map_err(|error| error.to_string())?;
        Ok(Some(Left {
            samples: residual(excerpt, &reference, &profile, Mask::UNDER).samples,
            offset,
        }))
    }
}

fn under_entry(
    report: &Report,
    catalog: &Catalog,
    left: &Left,
    taken_out: &TakenOut,
    ladder: &[Rung],
    jobs: usize,
) -> Result<Vec<FoundPlay>, String> {
    let profile = Profile::CURRENT;
    let offset = left.offset;
    let detections = search_with(
        &catalog.index,
        &left.samples,
        &profile,
        ladder,
        jobs,
        catalog.matcher.options(),
    );
    let candidates: Vec<Detection> = detections
        .into_iter()
        .filter(|detection| detection.evidence.confidence() >= Confidence::Possible)
        .filter(|detection| {
            let path = &catalog.index.asset(detection.asset).path;
            let start = offset + detection.start_seconds;
            let end = offset + detection.end_seconds;
            Dropped::of(path, start, end, taken_out, &report.plays).is_none()
        })
        .collect();
    if candidates.is_empty() {
        return Ok(Vec::new());
    }
    let shared = SharedMaterial::new(catalog, &taken_out.play.asset, ladder, jobs)?;
    let mut verdicts: BTreeMap<AssetId, bool> = BTreeMap::new();
    let kept: Vec<Detection> = candidates
        .into_iter()
        .filter(|detection| {
            let path = &catalog.index.asset(detection.asset).path;
            !*verdicts
                .entry(detection.asset)
                .or_insert_with(|| shared.with(catalog, path))
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
        .filter(|found| !named_as_one_track(found, &report.plays))
        .collect())
}

/// Whether a play found under another is one track with a confident play
/// of the recording, by the tests of the tracklist's entry rule (ADR 0017):
/// the two line up and are one passage of the mix (they overlap, or at
/// most `PASSAGE_GAP_SECONDS` lie between them), or their files are
/// tagged alike and the confident play overlaps it or lies within
/// `MAX_GAP_SECONDS` of it. Another upload of a track the tracklist names
/// there already, over the same passage or going on alone after it. Two
/// tracks whose lines put their files' starts at the same mix time are
/// common in a mix, so lining up across a longer stretch is no evidence
/// of one track (session 73 lost two right partners to it, experiment
/// 0274).
fn named_as_one_track(found: &FoundPlay, plays: &[FoundPlay]) -> bool {
    plays.iter().any(|play| {
        play.confidence == Level::Confident
            && ((within(found, play, PASSAGE_GAP_SECONDS) && found.lines_up_with(play))
                || (within(found, play, MAX_GAP_SECONDS) && found.tagged_alike(play)))
    })
}

/// Whether `play` overlaps `found` or lies within `seconds` of it.
fn within(found: &FoundPlay, play: &FoundPlay, seconds: f64) -> bool {
    play.start_seconds - seconds <= found.end_seconds
        && found.start_seconds <= play.end_seconds + seconds
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
/// held back this way, as the user decided that related versions are not
/// printed at first, and so may tracks built on the same sampled break.
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

    #[test]
    fn a_play_found_under_another_that_is_one_track_with_a_confident_play_is_dropped() {
        let report = report();
        // b.wav is confident from 38 to 81 s at -5%, from 5 s in the track.
        let upload = |track_start: f64, artist: &str| FoundPlay {
            asset: String::from("upload-of-b.wav"),
            start_seconds: 40.0,
            end_seconds: 75.0,
            track_start_seconds: track_start,
            speed: 0.95,
            tags: Some(crate::report::PlayTags {
                artist: Some(artist.to_owned()),
                title: Some(String::from("Tune")),
                album: None,
            }),
            ..report.plays[1].clone()
        };
        // Lining up with b.wav's play: the track would have started at the
        // same mix time.
        assert!(named_as_one_track(
            &upload(5.0 + 2.0 * 0.95, "Other"),
            &report.plays
        ));
        // 20 s away and tagged otherwise: another track.
        assert!(!named_as_one_track(&upload(25.0, "Other"), &report.plays));
        let mut tagged = report.plays.clone();
        tagged[1].tags = Some(crate::report::PlayTags {
            artist: Some(String::from("Artist")),
            title: Some(String::from("Tune")),
            album: None,
        });
        // Tagged alike over the same passage, however it lines up.
        assert!(named_as_one_track(&upload(25.0, "Artist"), &tagged));
        // Under a.wav before b.wav comes in, lining up with b.wav's play:
        // the track goes on alone, and the tracklist names it there.
        let earlier = FoundPlay {
            start_seconds: 10.0,
            end_seconds: 30.0,
            track_start_seconds: 5.0 - 28.0 * 0.95,
            ..upload(0.0, "Other")
        };
        assert!(named_as_one_track(&earlier, &report.plays));
        // The same 100 s before b.wav's play: too far to be that play.
        let far = FoundPlay {
            start_seconds: earlier.start_seconds - 100.0,
            end_seconds: earlier.end_seconds - 100.0,
            track_start_seconds: earlier.track_start_seconds - 100.0 * 0.95,
            ..earlier.clone()
        };
        assert!(far.lines_up_with(&report.plays[1]));
        assert!(!named_as_one_track(&far, &report.plays));
    }

    fn tagged(artist: &str, title: &str) -> Option<crate::report::PlayTags> {
        Some(crate::report::PlayTags {
            artist: Some(artist.to_owned()),
            title: Some(title.to_owned()),
            album: None,
        })
    }

    /// A play of `asset` from `start` to `end` at `speed` whose line puts
    /// its file's start at mix time `started`.
    fn lined_up(
        asset: &str,
        start: f64,
        end: f64,
        speed: f64,
        started: f64,
        confidence: Level,
        tags: Option<crate::report::PlayTags>,
    ) -> FoundPlay {
        FoundPlay {
            asset: asset.to_owned(),
            same_audio: Vec::new(),
            start_seconds: start,
            end_seconds: end,
            track_start_seconds: (start - started) * speed,
            speed,
            confidence,
            tags,
            ..report().plays[1].clone()
        }
    }

    #[test]
    fn a_find_lining_up_with_another_tracks_play_across_a_long_stretch_is_kept() {
        // Experiment 0274, drops seed 2050, mix-14: Tee Bee - Two Faced is
        // confident from 421.2 to 553.3 s at +2.02%, its file starting at
        // 325.8 s by its line. Chris SU & SKC - Blackout is found
        // confidently under the next entry from 589.8 s, 36 s after Two
        // Faced ends, its line putting its file's start 0.5 s later.
        let two_faced = FoundPlay {
            track_start_seconds: 97.330,
            ..lined_up(
                "two-faced.mp3",
                421.218,
                553.336,
                1.020_215,
                0.0,
                Level::Confident,
                tagged("Tee Bee", "Two Faced"),
            )
        };
        let started = 421.218 - 97.330 / 1.020_215;
        let blackout = |start: f64, end: f64| {
            lined_up(
                "blackout.mp3",
                start,
                end,
                1.0,
                started + 0.5,
                Level::Confident,
                tagged("Chris SU & SKC", "Blackout"),
            )
        };
        let plays = [two_faced];
        assert!(blackout(589.766, 633.272).lines_up_with(&plays[0]));
        assert!(!named_as_one_track(&blackout(589.766, 633.272), &plays));
        // The same line 8 s after Two Faced ends: one passage, a track
        // going on alone, so another upload of Two Faced.
        assert!(named_as_one_track(&blackout(561.3, 600.0), &plays));
    }

    #[test]
    fn a_possible_find_lining_up_with_another_tracks_play_35_s_before_is_kept() {
        // Experiment 0274, drops seed 2053, mix-00: Carlito & Addiction -
        // Into Music is confident from 461.0 to 611.1 s at -1.64%; Ed Rush &
        // Optical - Splinter is found as possible from 646.2 s, its line
        // putting its file's start 3.5 s from Into Music's.
        let into_music = FoundPlay {
            track_start_seconds: 133.367,
            ..lined_up(
                "into-music.mp3",
                461.035,
                611.113,
                0.983_608,
                0.0,
                Level::Confident,
                tagged("Carlito & Addiction", "Into Music"),
            )
        };
        let started = 461.035 - 133.367 / 0.983_608;
        let splinter = lined_up(
            "splinter.opus",
            646.201,
            686.229,
            1.0,
            started - 3.5,
            Level::Possible,
            tagged("Ed Rush & Optical", "Splinter"),
        );
        let plays = [into_music];
        assert!(splinter.lines_up_with(&plays[0]));
        assert!(!named_as_one_track(&splinter, &plays));
    }

    #[test]
    fn uploads_over_the_same_passage_and_tracks_tagged_alike_nearby_are_still_dropped() {
        let louder = lined_up(
            "tune.mp3",
            100.0,
            200.0,
            1.0,
            40.0,
            Level::Confident,
            tagged("Artist", "Tune"),
        );
        let plays = [louder];
        // Another upload of the track over the same passage, tagged
        // otherwise, lining up: a repeated upload.
        let upload = lined_up(
            "upload.opus",
            120.0,
            180.0,
            1.0,
            41.5,
            Level::Confident,
            tagged("artist", "Tune (Original Mix)"),
        );
        assert!(named_as_one_track(&upload, &plays));
        // An unrelated drop inside the same passage whose line happens to
        // put its file's start 4 s from the track's: the line test cannot
        // tell it from an upload, and it is dropped as ADR 0017's entry
        // rule joins such plays.
        let unrelated = lined_up(
            "other.mp3",
            150.0,
            190.0,
            1.0,
            44.0,
            Level::Confident,
            tagged("Someone", "Else"),
        );
        assert!(named_as_one_track(&unrelated, &plays));
        // An upload tagged alike 60 s after the track ends, however it
        // lines up: still within the 90 s of the tag test.
        let tagged_later = lined_up(
            "tagged.m4a",
            260.0,
            300.0,
            1.0,
            150.0,
            Level::Confident,
            tagged("Artist", "Tune"),
        );
        assert!(named_as_one_track(&tagged_later, &plays));
        // Neither lining up nor tagged alike: another track.
        let other = lined_up(
            "other.mp3",
            150.0,
            190.0,
            1.0,
            60.0,
            Level::Confident,
            tagged("Someone", "Else"),
        );
        assert!(!named_as_one_track(&other, &plays));
    }

    #[test]
    fn plays_found_under_others_that_are_one_track_are_one_tracklist_entry() {
        let mut found = with_plays_under();
        if let Some(under) = found.under.as_mut() {
            let mut rip = under[0].clone();
            rip.asset = String::from("rip-of-x.wav");
            rip.hits = 200;
            under.insert(1, rip);
        }
        assert_eq!(
            rendered(&found, ReportFormat::Tracklist),
            " 1.    0:00  a\n 2.    0:10  x  (also: rip-of-x) (under 1.)\n 3.    0:38  b\n 4.    1:24  insert (possible)\n 5.    1:36  c\n"
        );
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
    fn csv_gets_columns_for_play_numbers_only_when_searched_under() {
        let csv = rendered(&with_plays_under(), ReportFormat::Csv);
        let lines: Vec<&str> = csv.lines().collect();
        assert!(lines[0].ends_with(",shares_material_with_play,play,under_play"));
        assert_eq!(lines.len(), 7);
        let row = |line: usize, asset: &str, end: &str| {
            assert!(
                lines[line].contains(&format!(",{asset},")) && lines[line].ends_with(end),
                "{csv}"
            );
        };
        row(1, "a.wav", ",1,");
        row(2, "x.wav", ",,1");
        row(3, "b.wav", ",2,");
        row(5, "c.wav", ",4,");
        row(6, "y.wav", ",,4");

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
        for (number, line) in empty.lines().skip(1).enumerate() {
            assert!(line.ends_with(&format!(",{},", number + 1)), "{empty}");
        }
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
    fn a_report_whose_plays_found_under_others_do_not_fit_its_plays_is_rejected() {
        let mut beyond = with_plays_under();
        if let Some(under) = beyond.under.as_mut() {
            under[0].under = Some(SharedWith {
                play: 9,
                asset: String::from("a.wav"),
            });
        }
        let mut unmarked = with_plays_under();
        if let Some(under) = unmarked.under.as_mut() {
            under[0].under = None;
        }
        let mut marked_play = report();
        marked_play.plays[1].under = Some(SharedWith {
            play: 1,
            asset: String::from("a.wav"),
        });
        assert!(with_plays_under().validate().is_ok());
        for report in [beyond, unmarked, marked_play] {
            assert!(report.validate().is_err());
        }
    }
}

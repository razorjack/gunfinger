//! `questions detections`: is a detection right? One question per play of
//! a saved `identify` report (`-f json`): the mix is the timeline's
//! reference lane, from a little before the play to a little after, and the
//! play's file is placed by its segments (track time = track start +
//! speed × (mix time - start)), as `gunfinger listen` places it. Answers
//! go to the answer log only; report edits are for later
//! (roadmap, "Report edits").

use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::audition::names::Libraries;
use crate::audition::placement::{self, Alignment};
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Span, Timeline,
};

/// Seconds of the mix shown before and after a play.
const MARGIN_SECONDS: f64 = 30.0;

/// What the questions need of a report (`gunfinger identify -f json`).
#[derive(Debug, Deserialize)]
pub struct Report {
    pub query: Query,
    /// The library root the plays' assets are relative to.
    pub library: Option<PathBuf>,
    pub plays: Vec<Play>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub path: PathBuf,
    pub duration_seconds: f64,
}

#[derive(Debug, Deserialize)]
pub struct Play {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub confidence: String,
    pub hits: u32,
    pub windows: u32,
    pub playback: String,
    pub speed: f64,
    pub segments: Vec<Segment>,
    #[serde(default)]
    pub tags: Tags,
}

#[derive(Debug, Deserialize)]
pub struct Segment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub track_start_seconds: f64,
    pub speed: f64,
    pub confidence: String,
    pub hits: u32,
    pub windows: u32,
}

#[derive(Debug, Default, Deserialize)]
pub struct Tags {
    pub artist: Option<String>,
    pub title: Option<String>,
}

impl Report {
    pub fn load(path: &Path) -> Result<Report, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }
}

/// One question per play; `length` gives an audio file's decoded length in
/// seconds, to keep the track's lane inside its file.
pub fn questions(
    report: &Report,
    libraries: &Libraries,
    length: impl Fn(&Path) -> Result<f64, String>,
) -> Result<QuestionFile, String> {
    let root = report.library.as_deref().ok_or(
        "the report names no library (a --store-only search); its plays cannot be found as files",
    )?;
    let questions = report
        .plays
        .iter()
        .enumerate()
        .map(|(index, play)| {
            question(report, root, index + 1, play, libraries, &length)
                .map_err(|error| format!("play {} ({}): {error}", index + 1, play.asset))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: format!("Detections: {}", file_name(&report.query.path)),
        made_by: String::from("gunfinger-eval questions detections"),
        questions,
    })
}

fn question(
    report: &Report,
    root: &Path,
    number: usize,
    play: &Play,
    libraries: &Libraries,
    length: &impl Fn(&Path) -> Result<f64, String>,
) -> Result<Question, String> {
    let mix = &report.query.path;
    let track = root.join(&play.asset);
    let timeline = (
        (play.start_seconds - MARGIN_SECONDS).max(0.0),
        (play.end_seconds + MARGIN_SECONDS).min(report.query.duration_seconds),
    );
    let mut segments: Vec<(usize, &Segment)> = play.segments.iter().enumerate().collect();
    segments.sort_by_key(|(_, segment)| std::cmp::Reverse(segment.hits));
    let alignments: Vec<Alignment> = segments
        .iter()
        .map(|(index, segment)| Alignment {
            start: segment.start_seconds,
            end: segment.end_seconds,
            offset: segment.track_start_seconds - segment.speed * segment.start_seconds,
            speed: segment.speed,
            label: format!("segment {}", index + 1),
        })
        .collect();
    let track_lane = placement::tile(&track, length(&track)?, timeline, &alignments);
    if track_lane.is_empty() {
        return Err(String::from("the track has no audio beside the play"));
    }

    let mut markers = vec![
        Marker {
            at: play.start_seconds,
            label: String::from("the play starts"),
            kind: MarkerKind::Listen,
        },
        Marker {
            at: play.end_seconds,
            label: String::from("the play ends"),
            kind: MarkerKind::Edge,
        },
    ];
    markers.extend(
        play.segments
            .iter()
            .enumerate()
            .skip(1)
            .map(|(index, segment)| Marker {
                at: segment.start_seconds,
                label: format!("segment {} starts", index + 1),
                kind: MarkerKind::Edge,
            }),
    );
    markers.sort_by(|a, b| a.at.total_cmp(&b.at));
    let spans = play
        .segments
        .iter()
        .enumerate()
        .map(|(index, segment)| Span {
            start: segment.start_seconds,
            end: segment.end_seconds,
            lanes: vec![0, 1],
            label: format!(
                "segment {}: {}, {} hits in {} windows, speed {:+.2}%",
                index + 1,
                segment.confidence,
                segment.hits,
                segment.windows,
                (segment.speed - 1.0) * 100.0
            ),
        })
        .collect();

    let name = match (&play.tags.artist, &play.tags.title) {
        (Some(artist), Some(title)) => format!("{artist} - {title}"),
        (None, Some(title)) => title.clone(),
        _ => file_name(&track),
    };
    let mut details = vec![
        format!(
            "{}, {} hits in {} windows, {} at {:+.2}%",
            play.confidence,
            play.hits,
            play.windows,
            play.playback,
            (play.speed - 1.0) * 100.0
        ),
        format!(
            "Mix {}–{}",
            clock(play.start_seconds),
            clock(play.end_seconds)
        ),
    ];
    if play.playback != "turntable" {
        details.push(String::from(
            "Played key-locked: speed matching here changes the track's pitch with its tempo, which the mix did not",
        ));
    }
    let files = vec![libraries.name(mix), libraries.name(&track)];
    let start_second = format!("{:.0}", play.start_seconds);
    Ok(Question {
        id: QuestionId::derive("detection", &[&files[0], &files[1], &start_second]),
        kind: String::from("detection"),
        title: format!(
            "Play {number} at {}: {name} ({})",
            clock(play.start_seconds),
            play.confidence
        ),
        text: String::from("Is this the track playing in the mix here?"),
        details,
        files,
        timeline: Timeline {
            start: timeline.0,
            end: timeline.1,
            reference: Some(0),
        },
        lanes: vec![
            Lane {
                name: String::from("mix"),
                pieces: vec![Piece {
                    file: mix.clone(),
                    start: timeline.0,
                    end: timeline.1,
                    offset: 0.0,
                    speed: 1.0,
                    label: None,
                }],
            },
            Lane {
                name: String::from("track"),
                pieces: track_lane,
            },
        ],
        markers,
        spans,
        choices: vec![
            choice('y', "right", "confirmed"),
            choice('n', "wrong", "wrong"),
            choice('u', "unsure", "unsure"),
        ],
        effect: Effect::Log,
    })
}

fn choice(key: char, label: &str, value: &str) -> Choice {
    Choice {
        key,
        label: label.to_owned(),
        value: value.to_owned(),
    }
}

fn file_name(path: &Path) -> String {
    path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    )
}

/// `M:SS` or `H:MM:SS`.
fn clock(seconds: f64) -> String {
    let whole = seconds.max(0.0) as u64;
    let (hours, minutes, rest) = (whole / 3600, whole / 60 % 60, whole % 60);
    if hours > 0 {
        format!("{hours}:{minutes:02}:{rest:02}")
    } else {
        format!("{minutes}:{rest:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::schema::check;

    const REPORT: &str = r#"{
      "schema_version": 3,
      "query": {"path": "/mixes/mix.mp3", "start_seconds": 0.0, "duration_seconds": 2770.0},
      "library": "/nas/music",
      "plays": [
        {"asset": "label/Artist - Tune.opus", "start_seconds": 130.0, "end_seconds": 398.0,
         "track_start_seconds": 45.8, "track_end_seconds": 328.2, "speed": 1.048, "playback": "turntable",
         "confidence": "confident", "windows": 27, "hits": 6872,
         "segments": [
           {"start_seconds": 130.0, "end_seconds": 340.0, "track_start_seconds": 45.8, "track_end_seconds": 265.9,
            "speed": 1.048, "playback": "turntable", "confidence": "confident", "windows": 21, "hits": 6677},
           {"start_seconds": 341.3, "end_seconds": 398.0, "track_start_seconds": 268.8, "track_end_seconds": 328.2,
            "speed": 1.048, "playback": "turntable", "confidence": "possible", "windows": 6, "hits": 195}],
         "tags": {"artist": "Artist", "title": "Tune"}},
        {"asset": "Other.mp3", "start_seconds": 2750.0, "end_seconds": 2770.0, "track_start_seconds": 0.0,
         "track_end_seconds": 20.0, "speed": 1.0, "playback": "key-locked", "confidence": "possible",
         "windows": 3, "hits": 70, "segments": [
           {"start_seconds": 2750.0, "end_seconds": 2770.0, "track_start_seconds": 0.0, "track_end_seconds": 20.0,
            "speed": 1.0, "playback": "key-locked", "confidence": "possible", "windows": 3, "hits": 70}]}
      ]
    }"#;

    fn libraries() -> Libraries {
        Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            second: PathBuf::from("/nas/music"),
        }
    }

    #[test]
    fn each_play_becomes_a_question_with_the_mix_as_reference() {
        let report: Report = serde_json::from_str(REPORT).unwrap();

        let file = questions(&report, &libraries(), |_| Ok(400.0)).unwrap();
        check(&file).unwrap();

        let [play, last] = &file.questions[..] else {
            panic!("two questions expected");
        };
        assert_eq!(play.title, "Play 1 at 2:10: Artist - Tune (confident)");
        assert_eq!(
            play.files,
            ["/mixes/mix.mp3", "second-library/label/Artist - Tune.opus"]
        );
        assert_eq!(
            play.id,
            QuestionId::derive(
                "detection",
                &[
                    "/mixes/mix.mp3",
                    "second-library/label/Artist - Tune.opus",
                    "130"
                ]
            )
        );
        assert_eq!((play.timeline.start, play.timeline.end), (100.0, 428.0));
        assert_eq!(
            play.lanes[0].pieces[0].file,
            PathBuf::from("/mixes/mix.mp3")
        );
        let track = &play.lanes[1].pieces;
        assert_eq!(
            track[0].file,
            PathBuf::from("/nas/music/label/Artist - Tune.opus")
        );
        // Segment 1 from the timeline's start (where the track has audio)
        // until segment 2 starts; at the play's start the track is at 45.8 s.
        assert!((track[0].file_time(130.0) - 45.8).abs() < 1e-9);
        assert_eq!(track[1].start, 341.3);
        assert!((track[1].file_time(341.3) - 268.8).abs() < 1e-9);
        assert_eq!(
            play.markers
                .iter()
                .map(|marker| marker.at)
                .collect::<Vec<_>>(),
            [130.0, 341.3, 398.0]
        );
        assert_eq!(play.effect, Effect::Log);
        assert_eq!((last.timeline.start, last.timeline.end), (2720.0, 2770.0));
        assert!(
            last.title.ends_with("Other.mp3 (possible)"),
            "{}",
            last.title
        );
        assert!(last.details.iter().any(|line| line.contains("key-locked")));
    }

    #[test]
    fn a_report_without_a_library_is_refused() {
        let report: Report =
            serde_json::from_str(&REPORT.replace(r#""library": "/nas/music","#, "")).unwrap();

        let error = questions(&report, &libraries(), |_| Ok(400.0)).unwrap_err();

        assert!(error.contains("names no library"), "{error}");
    }
}

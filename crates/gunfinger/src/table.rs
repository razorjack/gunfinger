//! The default human output: one row per play.

use std::time::Duration;

use gunfinger_core::timecode::format_timecode;

use crate::report::{FoundPlay, Level, Playback, Report};
use crate::style::Style;

/// Width of the columns before the asset path.
const ASSET_COLUMN: usize = 62;

/// One row per play; assets with the same audio and a play's segments are
/// listed underneath. `in track` is the part of the track that was heard.
pub fn table(report: &Report, style: Style) -> String {
    let mut lines = vec![style.bold(&format!(
        "{:<19} {:<13} {:>7}  {:<10} {:>6}  asset",
        "time", "in track", "speed", "confidence", "hits"
    ))];
    for play in &report.plays {
        lines.push(row(
            &span(play.start_seconds, play.end_seconds),
            &span(play.track_start_seconds, play.track_end_seconds),
            play.speed,
            &paint(play.confidence, style),
            play.hits,
            &format!(
                "{}{}",
                named(&play.asset, play.playback),
                shared_material(play, str::to_owned)
            ),
        ));
        for asset in &play.same_audio {
            lines.push(style.dim(&format!("{:ASSET_COLUMN$}also {asset}", "")));
        }
        if let Some(titles) = other_titles(play) {
            lines.push(style.dim(&format!("{:ASSET_COLUMN$}{titles}", "")));
        }
        if play.segments.len() > 1 {
            for segment in &play.segments {
                lines.push(style.dim(&row(
                    &format!("  {}", span(segment.start_seconds, segment.end_seconds)),
                    &span(segment.track_start_seconds, segment.track_end_seconds),
                    segment.speed,
                    &format!("{:<10}", segment.confidence.label()),
                    segment.hits,
                    "",
                )));
            }
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

/// A key-locked play's speed is its tempo; the name says so.
pub fn named(name: &str, playback: Playback) -> String {
    match playback {
        Playback::Turntable => name.to_owned(),
        Playback::KeyLocked => format!("{name}  (key lock)"),
    }
}

/// "  shares material with <name> (play N)" for a possible play inside a
/// confident play of another recording, else nothing.
pub fn shared_material(play: &FoundPlay, name: impl Fn(&str) -> String) -> String {
    play.shares_material_with
        .as_ref()
        .map_or_else(String::new, |shared| {
            format!(
                "  shares material with {} (play {})",
                name(&shared.asset),
                shared.play
            )
        })
}

/// "titled X; other files here: Y (play 3), Z (plays 4, 5)" for a play
/// whose recording other files give other titles, one entry per spelling.
fn other_titles(play: &FoundPlay) -> Option<String> {
    let own = play.tagged_name()?;
    let mut spellings: Vec<(&str, Vec<usize>)> = Vec::new();
    for other in &play.other_titles {
        match spellings
            .iter_mut()
            .find(|(title, _)| title.eq_ignore_ascii_case(&other.title))
        {
            Some((_, plays)) => plays.push(other.play),
            None => spellings.push((&other.title, vec![other.play])),
        }
    }
    if spellings.is_empty() {
        return None;
    }
    let others: Vec<String> = spellings
        .iter()
        .map(|(title, plays)| {
            let numbers: Vec<String> = plays.iter().map(ToString::to_string).collect();
            let label = if plays.len() == 1 { "play" } else { "plays" };
            format!("{title} ({label} {})", numbers.join(", "))
        })
        .collect();
    Some(format!(
        "titled {own}; other files here: {}",
        others.join(", ")
    ))
}

/// The confidence padded to its column, then coloured.
fn paint(level: Level, style: Style) -> String {
    let cell = format!("{:<10}", level.label());
    match level {
        Level::Confident => style.green(&cell),
        Level::Possible => style.yellow(&cell),
        Level::Weak => cell,
    }
}

fn row(time: &str, track: &str, speed: f64, confidence: &str, hits: u32, asset: &str) -> String {
    let row = format!(
        "{time:<19} {track:<13} {:>+6.2}%  {confidence} {hits:>6}  {asset}",
        (speed - 1.0) * 100.0
    );
    row.trim_end().to_owned()
}

pub fn span(start_seconds: f64, end_seconds: f64) -> String {
    format!("{}-{}", timecode(start_seconds), timecode(end_seconds))
}

pub fn timecode(seconds: f64) -> String {
    format_timecode(Duration::from_secs_f64(seconds.max(0.0)))
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::playback::PlaybackChoice;
    use crate::report::{OtherTitle, PlayTags, Query, SCHEMA_VERSION, Segment, SharedWith};
    use crate::style::ColorChoice;

    fn segment(start_seconds: f64, end_seconds: f64, track_start_seconds: f64) -> Segment {
        Segment {
            start_seconds,
            end_seconds,
            track_start_seconds,
            track_end_seconds: track_start_seconds + (end_seconds - start_seconds) * 1.06,
            speed: 1.06,
            playback: Playback::Turntable,
            confidence: Level::Confident,
            windows: 4,
            hits: 400,
        }
    }

    /// The mix of the end-to-end test, as `identify` reports it.
    pub fn report() -> Report {
        let play = |asset: &str, start, end, track_start, speed, confidence, hits| FoundPlay {
            asset: String::from(asset),
            same_audio: Vec::new(),
            start_seconds: start,
            end_seconds: end,
            track_start_seconds: track_start,
            track_end_seconds: track_start + (end - start) * speed,
            speed,
            playback: Playback::Turntable,
            confidence,
            windows: 4,
            hits,
            segments: vec![Segment {
                start_seconds: start,
                end_seconds: end,
                track_start_seconds: track_start,
                track_end_seconds: track_start + (end - start) * speed,
                speed,
                playback: Playback::Turntable,
                confidence,
                windows: 4,
                hits,
            }],
            shares_material_with: None,
            other_titles: Vec::new(),
            tags: None,
        };
        Report {
            schema_version: SCHEMA_VERSION,
            query: Query {
                path: PathBuf::from("/mixes/mix.wav"),
                start_seconds: 0.0,
                duration_seconds: 150.0,
                playback: Some(PlaybackChoice::Both),
                requested_duration_seconds: None,
            },
            library: PathBuf::from("/library"),
            search: None,
            plays: vec![
                FoundPlay {
                    same_audio: vec![String::from("copy-of-a.wav")],
                    ..play("a.wav", 0.0, 41.0, 10.0, 1.03, Level::Confident, 853)
                },
                FoundPlay {
                    playback: Playback::KeyLocked,
                    ..play("b.wav", 38.0, 81.0, 5.0, 0.95, Level::Confident, 818)
                },
                play("insert.wav", 84.0, 94.0, 0.0, 1.0, Level::Possible, 75),
                FoundPlay {
                    segments: vec![segment(96.0, 111.0, 20.0), segment(112.0, 140.0, 41.0)],
                    track_end_seconds: 41.0 + 28.0 * 1.06,
                    ..play("c.wav", 96.0, 140.0, 20.0, 1.06, Level::Confident, 800)
                },
            ],
        }
    }

    #[test]
    fn the_table_lists_same_audio_and_segments_under_the_play() {
        let table = table(&report(), Style::for_stdout(ColorChoice::Never));

        assert_eq!(
            table,
            "\
time                in track        speed  confidence   hits  asset
0:00-0:41           0:10-0:52      +3.00%  confident     853  a.wav
                                                              also copy-of-a.wav
0:38-1:21           0:05-0:45      -5.00%  confident     818  b.wav  (key lock)
1:24-1:34           0:00-0:10      +0.00%  possible       75  insert.wav
1:36-2:20           0:20-1:10      +6.00%  confident     800  c.wav
  1:36-1:51         0:20-0:35      +6.00%  confident     400
  1:52-2:20         0:41-1:10      +6.00%  confident     400
"
        );
    }

    #[test]
    fn a_play_inside_a_confident_play_of_another_recording_says_so() {
        let mut report = report();
        report.plays[2].shares_material_with = Some(SharedWith {
            play: 2,
            asset: String::from("b.wav"),
        });

        let table = table(&report, Style::for_stdout(ColorChoice::Never));

        assert!(
            table.contains("possible       75  insert.wav  shares material with b.wav (play 2)\n"),
            "{table}"
        );
    }

    #[test]
    fn a_play_whose_passage_other_files_title_otherwise_says_so() {
        let mut report = report();
        report.plays[1].tags = Some(PlayTags {
            artist: Some(String::from("Future Cut")),
            title: Some(String::from("Sex Drive")),
            album: None,
        });
        report.plays[1].other_titles = vec![
            OtherTitle {
                play: 5,
                asset: String::from("upload.m4a"),
                title: String::from("FUTURE CUT - the specialist"),
            },
            OtherTitle {
                play: 6,
                asset: String::from("rip.mp3"),
                title: String::from("Future Cut - The Specialist"),
            },
        ];

        let table = table(&report, Style::for_stdout(ColorChoice::Never));

        assert!(
            table.contains(
                "b.wav  (key lock)\n                                                              titled Future Cut - Sex Drive; other files here: FUTURE CUT - the specialist (plays 5, 6)\n"
            ),
            "{table}"
        );
    }

    #[test]
    fn colour_marks_the_confidence_without_moving_columns() {
        let table = table(&report(), Style::for_stdout(ColorChoice::Always));

        let play_row = table.lines().nth(1).unwrap_or_default();
        assert!(play_row.contains("+3.00%  \x1b[32mconfident \x1b[0m    853  a.wav"));
    }
}

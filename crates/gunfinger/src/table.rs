//! The default human output: one row per play.

use std::time::Duration;

use gunfinger_core::timecode::format_timecode;

use crate::report::{Level, Report};
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
            &play.asset,
        ));
        for asset in &play.same_audio {
            lines.push(style.dim(&format!("{:ASSET_COLUMN$}also {asset}", "")));
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
    use crate::report::{FoundPlay, Query, SCHEMA_VERSION, Segment};
    use crate::style::ColorChoice;

    fn segment(start_seconds: f64, end_seconds: f64, track_start_seconds: f64) -> Segment {
        Segment {
            start_seconds,
            end_seconds,
            track_start_seconds,
            track_end_seconds: track_start_seconds + (end_seconds - start_seconds) * 1.06,
            speed: 1.06,
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
            confidence,
            windows: 4,
            hits,
            segments: vec![Segment {
                start_seconds: start,
                end_seconds: end,
                track_start_seconds: track_start,
                track_end_seconds: track_start + (end - start) * speed,
                speed,
                confidence,
                windows: 4,
                hits,
            }],
        };
        Report {
            schema_version: SCHEMA_VERSION,
            query: Query {
                path: PathBuf::from("/mixes/mix.wav"),
                start_seconds: 0.0,
                duration_seconds: 150.0,
            },
            library: PathBuf::from("/library"),
            plays: vec![
                FoundPlay {
                    same_audio: vec![String::from("copy-of-a.wav")],
                    ..play("a.wav", 0.0, 41.0, 10.0, 1.03, Level::Confident, 853)
                },
                play("b.wav", 38.0, 81.0, 5.0, 0.95, Level::Confident, 818),
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
0:38-1:21           0:05-0:45      -5.00%  confident     818  b.wav
1:24-1:34           0:00-0:10      +0.00%  possible       75  insert.wav
1:36-2:20           0:20-1:10      +6.00%  confident     800  c.wav
  1:36-1:51         0:20-0:35      +6.00%  confident     400
  1:52-2:20         0:41-1:10      +6.00%  confident     400
"
        );
    }

    #[test]
    fn colour_marks_the_confidence_without_moving_columns() {
        let table = table(&report(), Style::for_stdout(ColorChoice::Always));

        let play_row = table.lines().nth(1).unwrap_or_default();
        assert!(play_row.contains("+3.00%  \x1b[32mconfident \x1b[0m    853  a.wav"));
    }
}

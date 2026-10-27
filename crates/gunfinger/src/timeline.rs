//! `--format timeline`: each play as a bar across the searched recording.

use gunfinger_core::timecode::format_seconds;
use std::path::Path;

use crate::report::{FoundPlay, Level, Report};
use crate::style::Style;
use crate::table::{named, shared_material};

/// Columns of the bar; at 56 minutes, one column is about 53 seconds.
const BAR_COLUMNS: usize = 64;
/// Width of the start time before each bar.
const TIME_COLUMN: usize = 8;

/// One line per play: its start, a bar marking its segments (`█` confident,
/// `▒` possible) joined across gaps (`─`), and the file name.
pub fn timeline(report: &Report, style: Style) -> String {
    let scale = Scale {
        from: report.query.start_seconds,
        seconds: report.query.duration_seconds.max(1.0),
    };
    let mut lines = vec![axis(report)];
    for play in &report.plays {
        let bar: String = bar(play, &scale).into_iter().collect();
        let bar = match play.confidence {
            Level::Confident => style.green(&bar),
            Level::Possible => style.yellow(&bar),
            Level::Weak => bar,
        };
        lines.push(format!(
            "{:>TIME_COLUMN$} │{bar}│ {}{}",
            format_seconds(play.start_seconds),
            named(file_name(&play.asset), play.playback),
            shared_material(play, |asset| file_name(asset).to_owned())
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

struct Scale {
    from: f64,
    seconds: f64,
}

impl Scale {
    fn column(&self, seconds: f64) -> usize {
        let share = ((seconds - self.from) / self.seconds).clamp(0.0, 1.0);
        ((share * BAR_COLUMNS as f64) as usize).min(BAR_COLUMNS - 1)
    }
}

fn bar(play: &FoundPlay, scale: &Scale) -> Vec<char> {
    let mut bar = vec![' '; BAR_COLUMNS];
    for cell in &mut bar[scale.column(play.start_seconds)..=scale.column(play.end_seconds)] {
        *cell = '─';
    }
    // Weakest first: a weaker segment at another speed can lie inside a
    // stronger one (ADR 0021) and must not cover its mark.
    let mut segments: Vec<_> = play.segments.iter().collect();
    segments.sort_by_key(|segment| segment.confidence);
    for segment in segments {
        let mark = match segment.confidence {
            Level::Confident => '█',
            Level::Possible => '▒',
            Level::Weak => '░',
        };
        for cell in
            &mut bar[scale.column(segment.start_seconds)..=scale.column(segment.end_seconds)]
        {
            *cell = mark;
        }
    }
    bar
}

/// Times at the start, the quarters and the end of the bar.
fn axis(report: &Report) -> String {
    let mut axis = vec![' '; BAR_COLUMNS + 2];
    for quarter in 0..=4_u32 {
        let seconds =
            report.query.start_seconds + report.query.duration_seconds * f64::from(quarter) / 4.0;
        let label: Vec<char> = format_seconds(seconds).chars().collect();
        let at = (quarter as usize * BAR_COLUMNS / 4 + 1).min(axis.len() - label.len());
        axis[at..at + label.len()].copy_from_slice(&label);
    }
    let axis: String = axis.into_iter().collect();
    format!("{:TIME_COLUMN$} {}", "", axis.trim_end())
}

fn file_name(asset: &str) -> &str {
    Path::new(asset)
        .file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or(asset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::Segment;
    use crate::style::ColorChoice;
    use crate::table::tests::report;

    #[test]
    fn each_play_is_a_bar_at_its_place_in_the_recording() {
        let timeline = timeline(&report(), Style::for_stdout(ColorChoice::Never));

        let expected = [
            "          0:00            0:37            1:15            1:52         2:30",
            "    0:00 │██████████████████                                              │ a",
            "    0:38 │                ███████████████████                             │ b  (key lock)",
            "    1:24 │                                   ▒▒▒▒▒▒                       │ insert",
            "    1:36 │                                        ████████████████████    │ c",
            "",
        ];
        assert_eq!(timeline, expected.join("\n"));
    }

    #[test]
    fn a_possible_segment_inside_a_confident_one_does_not_cover_it() {
        let mut report = report();
        let play = &mut report.plays[0];
        let inside = Segment {
            start_seconds: 10.0,
            end_seconds: 20.0,
            confidence: Level::Possible,
            ..play.segments[0].clone()
        };
        let after = Segment {
            start_seconds: 45.0,
            end_seconds: 55.0,
            ..inside.clone()
        };
        play.segments.extend([inside, after]);
        play.end_seconds = 55.0;
        let scale = Scale {
            from: 0.0,
            seconds: 150.0,
        };

        let bar: String = bar(play, &scale).into_iter().collect();

        assert_eq!(
            bar,
            format!("{}─{}{}", "█".repeat(18), "▒".repeat(5), " ".repeat(40))
        );
    }
}

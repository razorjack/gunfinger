//! `--format timeline`: each play as a bar across the searched recording.

use std::path::Path;

use crate::report::{FoundPlay, Level, Report};
use crate::style::Style;
use crate::table::{named, shared_material, timecode};

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
            timecode(play.start_seconds),
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
    for segment in &play.segments {
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
        let label: Vec<char> = timecode(seconds).chars().collect();
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
}

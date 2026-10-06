//! `gunfinger review`: step through a saved report by ear. Lists the plays,
//! then reads commands from stdin: a play's number plays the recording and
//! the tracks found there (as `listen` does) in the middle of the play;
//! `<n> start` and `<n> end` play near its edges; `at <time>` plays any
//! moment.

use std::io::{BufRead, Write};

use gunfinger_core::timecode::parse_timecode;
use miette::IntoDiagnostic;

use crate::console::Console;
use crate::listen::{clips, play};
use crate::report::{FoundPlay, Report};
use crate::style::Style;
use crate::table::{named, span};

/// Seconds inside a play's edges that `start` and `end` begin at.
const EDGE_SECONDS: f64 = 5.0;

const HELP: &str = "<n> plays the middle of play n, <n> start and <n> end its edges, at <time> any moment; l lists, q quits";

pub fn run(report: &Report, seconds: f64, style: Style, console: &Console) -> miette::Result<()> {
    print!("{}", list(report, style));
    println!("{HELP}");
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        std::io::stdout().flush().into_diagnostic()?;
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).into_diagnostic()? == 0 {
            return Ok(());
        }
        match command(report, line.trim()) {
            Command::Quit => return Ok(()),
            Command::List => print!("{}", list(report, style)),
            Command::Help => println!("{HELP}"),
            Command::Listen(at) => match clips(report, at) {
                Ok(clips) => play(&clips, seconds, console)?,
                Err(error) => console.warning(error),
            },
            Command::Unknown(text) => console.warning(format_args!("{text} (h for help)")),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    Listen(f64),
    List,
    Help,
    Quit,
    Unknown(String),
}

fn command(report: &Report, line: &str) -> Command {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words.as_slice() {
        [] | ["l" | "list"] => Command::List,
        ["q" | "quit"] => Command::Quit,
        ["h" | "help" | "?"] => Command::Help,
        ["at", time] => match parse_timecode(time) {
            Ok(time) => Command::Listen(time.as_secs_f64()),
            Err(error) => Command::Unknown(error.to_string()),
        },
        [number, place @ ..] => {
            let Some(play) = number
                .parse::<usize>()
                .ok()
                .and_then(|number| number.checked_sub(1))
                .and_then(|index| report.plays.get(index))
            else {
                return Command::Unknown(format!("no play {number}"));
            };
            match place {
                [] => Command::Listen(middle(play)),
                ["start"] => {
                    Command::Listen((play.start_seconds + EDGE_SECONDS).min(play.end_seconds))
                }
                ["end"] => {
                    Command::Listen((play.end_seconds - 2.0 * EDGE_SECONDS).max(play.start_seconds))
                }
                _ => Command::Unknown(format!("unknown place {}", place.join(" "))),
            }
        }
    }
}

/// The middle of the play's longest segment: well inside the record,
/// away from crossfades.
fn middle(play: &FoundPlay) -> f64 {
    play.segments
        .iter()
        .max_by(|a, b| {
            (a.end_seconds - a.start_seconds).total_cmp(&(b.end_seconds - b.start_seconds))
        })
        .map_or(play.start_seconds, |segment| {
            (segment.start_seconds + segment.end_seconds) / 2.0
        })
}

fn list(report: &Report, style: Style) -> String {
    let mut lines = Vec::new();
    for (number, play) in report.plays.iter().enumerate() {
        lines.push(format!(
            "{:>3}  {:<19} {:<10} {}",
            style.bold(&(number + 1).to_string()),
            span(play.start_seconds, play.end_seconds),
            play.confidence.label(),
            named(&play.asset, play.playback)
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::ColorChoice;
    use crate::table::tests::report;

    #[test]
    fn commands_pick_a_moment_in_a_play() {
        let report = report();

        assert_eq!(command(&report, "1"), Command::Listen(20.5));
        assert_eq!(command(&report, "4"), Command::Listen(126.0));
        assert_eq!(command(&report, "2 start"), Command::Listen(43.0));
        assert_eq!(command(&report, "2 end"), Command::Listen(71.0));
        assert_eq!(command(&report, "at 1:30"), Command::Listen(90.0));
        assert_eq!(command(&report, "q"), Command::Quit);
        assert!(matches!(command(&report, "9"), Command::Unknown(_)));
        assert!(matches!(command(&report, "at noon"), Command::Unknown(_)));
    }

    #[test]
    fn the_list_numbers_the_plays() {
        let list = list(&report(), Style::for_stdout(ColorChoice::Never));

        assert!(list.starts_with("  1  0:00-0:41           confident  a.wav\n"));
        assert!(list.contains("  2  0:38-1:21           confident  b.wav  (key lock)\n"));
    }
}

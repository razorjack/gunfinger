//! Colour in human output. JSON, pipes and files stay plain.

use std::ffi::OsString;
use std::io::IsTerminal;

use clap::ValueEnum;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum ColorChoice {
    /// Colour when writing to a terminal, unless `NO_COLOR` is set.
    Auto,
    Always,
    Never,
}

/// Paints text with ANSI codes, or leaves it alone.
#[derive(Clone, Copy, Debug)]
pub struct Style {
    color: bool,
}

impl Style {
    pub fn for_stdout(choice: ColorChoice) -> Style {
        Style::new(choice, std::io::stdout().is_terminal())
    }

    pub fn for_stderr(choice: ColorChoice) -> Style {
        Style::new(choice, std::io::stderr().is_terminal())
    }

    fn new(choice: ColorChoice, terminal: bool) -> Style {
        Style {
            color: wants_color(
                choice,
                terminal,
                std::env::var_os("NO_COLOR"),
                std::env::var_os("TERM"),
            ),
        }
    }

    pub fn bold(self, text: &str) -> String {
        self.paint("1", text)
    }

    pub fn dim(self, text: &str) -> String {
        self.paint("2", text)
    }

    pub fn red(self, text: &str) -> String {
        self.paint("31", text)
    }

    pub fn green(self, text: &str) -> String {
        self.paint("32", text)
    }

    pub fn yellow(self, text: &str) -> String {
        self.paint("33", text)
    }

    fn paint(self, code: &str, text: &str) -> String {
        if self.color && !text.is_empty() {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_owned()
        }
    }
}

/// `NO_COLOR` (no-color.org) counts when it is set and not empty; an explicit
/// `--color always` overrides it.
fn wants_color(
    choice: ColorChoice,
    terminal: bool,
    no_color: Option<OsString>,
    term: Option<OsString>,
) -> bool {
    match choice {
        ColorChoice::Always => true,
        ColorChoice::Never => false,
        ColorChoice::Auto => {
            terminal
                && no_color.is_none_or(|value| value.is_empty())
                && term.is_none_or(|term| term != "dumb")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auto_colours_a_terminal_only() {
        assert!(wants_color(ColorChoice::Auto, true, None, None));
        assert!(!wants_color(ColorChoice::Auto, false, None, None));
    }

    #[test]
    fn no_color_and_a_dumb_terminal_turn_auto_off() {
        let set = |value: &str| Some(OsString::from(value));
        assert!(!wants_color(ColorChoice::Auto, true, set("1"), None));
        assert!(wants_color(ColorChoice::Auto, true, set(""), None));
        assert!(!wants_color(ColorChoice::Auto, true, None, set("dumb")));
    }

    #[test]
    fn an_explicit_choice_wins() {
        assert!(wants_color(
            ColorChoice::Always,
            false,
            Some(OsString::from("1")),
            None
        ));
        assert!(!wants_color(ColorChoice::Never, true, None, None));
    }

    #[test]
    fn plain_style_leaves_text_alone() {
        let plain = Style { color: false };
        let colored = Style { color: true };
        assert_eq!(plain.green("confident"), "confident");
        assert_eq!(colored.green("confident"), "\x1b[32mconfident\x1b[0m");
    }
}

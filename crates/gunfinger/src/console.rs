//! Messages on stderr. Results go to stdout; everything else comes here:
//! progress and messages, which `--quiet` turns off; details such as
//! timings and files left out of the index, which only `--verbose` shows;
//! and warnings, which are always shown.

use std::fmt::Display;
use std::io::IsTerminal;

use crate::style::{ColorChoice, Style};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

pub struct Console {
    style: Style,
    verbosity: Verbosity,
    /// Progress lines rewrite themselves, which only a terminal shows well.
    terminal: bool,
}

impl Console {
    pub fn new(color: ColorChoice, verbosity: Verbosity) -> Console {
        Console {
            style: Style::for_stderr(color),
            verbosity,
            terminal: std::io::stderr().is_terminal(),
        }
    }

    pub fn is_verbose(&self) -> bool {
        self.verbosity == Verbosity::Verbose
    }

    /// Rewrites the current line, on a terminal only.
    pub fn progress(&self, message: impl Display) {
        if self.terminal && self.verbosity >= Verbosity::Normal {
            eprint!("\r{message}\x1b[K");
        }
    }

    /// Clears the progress line.
    pub fn progress_done(&self) {
        if self.terminal && self.verbosity >= Verbosity::Normal {
            eprint!("\r\x1b[K");
        }
    }

    pub fn info(&self, message: impl Display) {
        if self.verbosity >= Verbosity::Normal {
            eprintln!("{message}");
        }
    }

    /// Shown with `--verbose` only.
    pub fn detail(&self, message: impl Display) {
        if self.is_verbose() {
            eprintln!("{message}");
        }
    }

    /// The first line gets the `warning:` label; further lines are details.
    pub fn warning(&self, message: impl Display) {
        eprintln!("{} {message}", self.style.yellow("warning:"));
    }

    pub fn error(&self, message: impl Display) {
        eprintln!("{} {message}", self.style.red("error:"));
    }
}

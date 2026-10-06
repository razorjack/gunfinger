//! Messages on stderr. Results go to stdout; everything else comes here:
//! progress and timing, which `--quiet` turns off, and warnings, which it
//! does not.

use std::fmt::Display;
use std::io::IsTerminal;

use crate::style::{ColorChoice, Style};

pub struct Console {
    style: Style,
    quiet: bool,
    /// Progress lines rewrite themselves, which only a terminal shows well.
    terminal: bool,
}

impl Console {
    pub fn new(color: ColorChoice, quiet: bool) -> Console {
        Console {
            style: Style::for_stderr(color),
            quiet,
            terminal: std::io::stderr().is_terminal(),
        }
    }

    /// Rewrites the current line, on a terminal only.
    pub fn progress(&self, message: impl Display) {
        if self.terminal && !self.quiet {
            eprint!("\r{message}\x1b[K");
        }
    }

    /// Clears the progress line.
    pub fn progress_done(&self) {
        if self.terminal && !self.quiet {
            eprint!("\r\x1b[K");
        }
    }

    pub fn info(&self, message: impl Display) {
        if !self.quiet {
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

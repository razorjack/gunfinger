//! Command-line interface of Gunfinger.

mod catalog;
mod console;
mod identify;
mod index;
mod stats;
mod style;

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use console::Console;
use gunfinger_core::decode::Excerpt;
use gunfinger_core::timecode::parse_timecode;
use style::{ColorChoice, Style};

/// Identify tracks from your own collection inside DJ mixes.
#[derive(Parser)]
#[command(name = "gunfinger", version)]
struct Cli {
    /// Directory of the peak store.
    #[arg(
        long,
        global = true,
        env = "GUNFINGER_PEAKS_DIR",
        default_value = "work/peaks"
    )]
    peaks_dir: PathBuf,

    /// Worker threads [default: available parallelism].
    #[arg(long, global = true, env = "GUNFINGER_JOBS")]
    jobs: Option<NonZeroUsize>,

    /// When to colour human output.
    #[arg(long, global = true, value_enum, default_value_t = ColorChoice::Auto)]
    color: ColorChoice,

    /// Print only results, warnings and errors.
    #[arg(long, short, global = true)]
    quiet: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Extract the peaks of every audio file in a library into the peak store.
    Index {
        /// Root directory of the library; asset identities are relative to it.
        library: PathBuf,
        /// Files longer than this are mixes or album rips and are skipped.
        #[arg(long, default_value_t = 20)]
        max_track_minutes: u64,
    },
    /// Find library tracks inside a recording.
    Identify {
        /// The recording to search, typically a DJ mix.
        audio: PathBuf,
        /// Root directory of the indexed library.
        #[arg(long)]
        library: PathBuf,
        /// Start of the part to search (seconds, M:SS or H:MM:SS).
        #[arg(long, value_parser = parse_timecode)]
        start: Option<Duration>,
        /// Length of the part to search.
        #[arg(long, value_parser = parse_timecode)]
        duration: Option<Duration>,
        /// File listing library paths to leave out of the index, one per line.
        #[arg(long)]
        exclude_from: Option<PathBuf>,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
    /// Measure the peak store and the index of a library.
    Stats {
        /// Root directory of the indexed library.
        #[arg(long)]
        library: PathBuf,
        /// Output format.
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
}

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    /// An aligned table for people.
    Human,
    /// JSON on stdout, with a `schema_version`.
    Json,
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    color_error_reports(cli.color);
    let console = Console::new(cli.color, cli.quiet);
    let jobs = cli.jobs.map_or_else(
        || std::thread::available_parallelism().map_or(1, NonZeroUsize::get),
        NonZeroUsize::get,
    );
    match cli.command {
        Command::Index {
            library,
            max_track_minutes,
        } => index::run(&library, &cli.peaks_dir, jobs, max_track_minutes, &console),
        Command::Identify {
            audio,
            library,
            start,
            duration,
            exclude_from,
            format,
        } => identify::run(&identify::Request {
            audio: &audio,
            library: &library,
            peaks_dir: &cli.peaks_dir,
            excerpt: Excerpt { start, duration },
            exclude_from: exclude_from.as_deref(),
            format,
            style: Style::for_stdout(cli.color),
            jobs,
            console: &console,
        }),
        Command::Stats { library, format } => {
            stats::run(&library, &cli.peaks_dir, format, &console)
        }
    }
}

/// miette decides on colour by itself unless told.
fn color_error_reports(color: ColorChoice) {
    let forced = match color {
        ColorChoice::Auto => return,
        ColorChoice::Always => true,
        ColorChoice::Never => false,
    };
    // Fails only when a hook is already installed, and none is.
    let _ = miette::set_hook(Box::new(move |_| {
        Box::new(miette::MietteHandlerOpts::new().color(forced).build())
    }));
}

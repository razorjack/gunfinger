//! Command-line interface of Gunfinger.

mod catalog;
mod identify;
mod index;
mod stats;

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use gunfinger_core::decode::Excerpt;
use gunfinger_core::timecode::parse_timecode;

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
        #[arg(long, value_enum, default_value_t = Format::Human)]
        format: Format,
    },
    /// Measure the peak store and the index of a library.
    Stats {
        /// Root directory of the indexed library.
        #[arg(long)]
        library: PathBuf,
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
    let jobs = cli.jobs.map_or_else(
        || std::thread::available_parallelism().map_or(1, NonZeroUsize::get),
        NonZeroUsize::get,
    );
    match cli.command {
        Command::Index {
            library,
            max_track_minutes,
        } => index::run(&library, &cli.peaks_dir, jobs, max_track_minutes),
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
            jobs,
        }),
        Command::Stats { library, format } => stats::run(&library, &cli.peaks_dir, format),
    }
}

//! Command-line interface of Gunfinger.

mod index;

use std::num::NonZeroUsize;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Extract the peaks of every audio file in a library into the peak store.
    Index {
        /// Root directory of the library; asset identities are relative to it.
        library: PathBuf,
        /// Files decoded in parallel [default: available parallelism].
        #[arg(long, env = "GUNFINGER_JOBS")]
        jobs: Option<NonZeroUsize>,
        /// Files longer than this are mixes or album rips and are skipped.
        #[arg(long, default_value_t = 20)]
        max_track_minutes: u64,
    },
}

fn main() -> miette::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Index {
            library,
            jobs,
            max_track_minutes,
        } => index::run(&library, &cli.peaks_dir, jobs, max_track_minutes),
    }
}

fn default_jobs() -> usize {
    std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

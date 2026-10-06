//! Command-line interface of Gunfinger.

mod catalog;
mod config;
mod console;
mod doctor;
mod explain;
mod export;
mod identify;
mod index;
mod listen;
mod names;
mod output;
mod playback;
mod prune;
mod report;
mod review;
mod stats;
mod style;
mod survey;
mod table;
mod timeline;

use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::time::Duration;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use config::{Given, Settings};
use console::{Console, Verbosity};
use gunfinger_core::decode::Excerpt;
use gunfinger_core::timecode::parse_timecode;
use miette::IntoDiagnostic;
use output::ReportFormat;
use playback::PlaybackChoice;
use report::Report;
use style::{ColorChoice, Style};

/// Identify tracks from your own collection inside DJ mixes.
#[derive(Parser)]
#[command(name = "gunfinger", version)]
struct Cli {
    /// Configuration file [default: $XDG_CONFIG_HOME/gunfinger/config.toml].
    #[arg(long, global = true, env = "GUNFINGER_CONFIG")]
    config: Option<PathBuf>,

    /// Directory of the peak store [default: work/peaks].
    #[arg(long, global = true, env = "GUNFINGER_PEAKS_DIR")]
    peaks_dir: Option<PathBuf>,

    /// Worker threads [default: available parallelism].
    #[arg(long, global = true, env = "GUNFINGER_JOBS")]
    jobs: Option<NonZeroUsize>,

    /// When to colour human output [default: auto].
    #[arg(long, global = true, value_enum)]
    color: Option<ColorChoice>,

    /// Print only results, warnings and errors.
    #[arg(long, short, global = true, conflicts_with = "verbose")]
    quiet: bool,

    /// Also print timings, the size of the index and every library file
    /// left out of it.
    #[arg(long, short, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Extract the peaks of every audio file in a library into the peak store.
    Index {
        /// Root directory of the library; asset identities are relative to it
        /// [default: `library` in the configuration file].
        library: Option<PathBuf>,
        /// Files longer than this are mixes or album rips and are skipped.
        #[arg(long, default_value_t = 20)]
        max_track_minutes: u64,
        /// Try files again that an earlier run failed on or found too long;
        /// otherwise they are passed over until they change.
        #[arg(long)]
        retry_skipped: bool,
    },
    /// Find library tracks inside recordings.
    Identify {
        /// The recordings to search, typically DJ mixes.
        #[arg(required = true)]
        audio: Vec<PathBuf>,
        /// Root directory of the indexed library [default: `library` in the
        /// configuration file].
        #[arg(long)]
        library: Option<PathBuf>,
        /// Start of the part to search (seconds, M:SS or H:MM:SS).
        #[arg(long, value_parser = parse_timecode)]
        start: Option<Duration>,
        /// Length of the part to search.
        #[arg(long, value_parser = parse_timecode)]
        duration: Option<Duration>,
        /// How the records were played: `turntable` (vinyl: pitch and tempo
        /// together), `key-lock` (CDJ master tempo: tempo only) or `both`;
        /// one alone searches about a third faster [default: `playback` in
        /// the configuration file, else both].
        #[arg(long, value_enum)]
        playback: Option<PlaybackChoice>,
        /// File listing library paths to leave out of the index, one per line.
        #[arg(long)]
        exclude_from: Option<PathBuf>,
        /// Output format.
        #[arg(long, short, value_enum, default_value_t = ReportFormat::Human)]
        format: ReportFormat,
        /// Also write each recording's JSON report into this directory, as
        /// <recording name>.json; recordings with a report there are passed
        /// over.
        #[arg(long)]
        save_dir: Option<PathBuf>,
        /// Search recordings again that already have a report in --save-dir.
        #[arg(long, requires = "save_dir")]
        again: bool,
    },
    /// List every candidate around a moment of a recording, weak ones
    /// included, with what each lacks for the next level.
    Explain {
        /// The recording, typically a DJ mix.
        audio: PathBuf,
        /// Root directory of the indexed library [default: `library` in the
        /// configuration file].
        #[arg(long)]
        library: Option<PathBuf>,
        /// The moment to explain (seconds, M:SS or H:MM:SS).
        #[arg(long, value_parser = parse_timecode)]
        at: Duration,
        /// How much to search on either side of the moment.
        #[arg(long, value_parser = parse_timecode, default_value = "60")]
        around: Duration,
        /// How the records were played [default: `playback` in the
        /// configuration file, else both].
        #[arg(long, value_enum)]
        playback: Option<PlaybackChoice>,
        /// Only assets whose path contains this text (ignoring case).
        #[arg(long)]
        asset: Option<String>,
        /// Candidates to list.
        #[arg(long, default_value_t = 12)]
        limit: usize,
        /// File listing library paths to leave out of the index, one per line.
        #[arg(long)]
        exclude_from: Option<PathBuf>,
    },
    /// Write a saved JSON report of `identify` in another format.
    Show {
        /// A report written by `identify --format json`.
        report: PathBuf,
        /// Output format.
        #[arg(long, short, value_enum, default_value_t = ReportFormat::Human)]
        format: ReportFormat,
    },
    /// Play the recording at a moment, then each track found there from the
    /// same place and at the same speed.
    Listen {
        /// A report written by `identify --format json`.
        report: PathBuf,
        /// The moment in the recording (seconds, M:SS or H:MM:SS).
        #[arg(long, value_parser = parse_timecode)]
        at: Duration,
        /// Seconds of each clip.
        #[arg(long, default_value_t = 8.0)]
        seconds: f64,
        /// Library root, when it has moved since the report was written.
        #[arg(long)]
        library: Option<PathBuf>,
        /// Print the ffplay commands instead of playing.
        #[arg(long)]
        print: bool,
    },
    /// Step through a saved report by ear: list its plays, then play the
    /// recording and the tracks found in any of them.
    Review {
        /// A report written by `identify --format json`.
        report: PathBuf,
        /// Seconds of each clip.
        #[arg(long, default_value_t = 8.0)]
        seconds: f64,
        /// Library root, when it has moved since the report was written.
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// Print a completion script for a shell, for example
    /// `gunfinger completions zsh > ~/.zfunc/_gunfinger`.
    Completions { shell: clap_complete::Shell },
    /// Print the man page, for example `gunfinger man > gunfinger.1`.
    Man,
    /// Check FFmpeg, the settings, the library and the peak store.
    Doctor {
        /// Root directory of the library [default: `library` in the
        /// configuration file].
        #[arg(long)]
        library: Option<PathBuf>,
    },
    /// Delete peak records of files no longer in the library, and leftovers
    /// of interrupted runs. Lists them unless --yes is given.
    Prune {
        /// Root directory of the library [default: `library` in the
        /// configuration file].
        #[arg(long)]
        library: Option<PathBuf>,
        /// Delete what would be listed.
        #[arg(long)]
        yes: bool,
        /// Prune even when most of the store would go.
        #[arg(long)]
        force: bool,
    },
    /// Measure the peak store and the index of a library.
    Stats {
        /// Root directory of the indexed library [default: `library` in the
        /// configuration file].
        #[arg(long)]
        library: Option<PathBuf>,
        /// Output format.
        #[arg(long, short, value_enum, default_value_t = Format::Human)]
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
    let settings = Settings::resolve(Given {
        config: cli.config,
        peaks_dir: cli.peaks_dir,
        jobs: cli.jobs,
        color: cli.color,
    })?;
    color_error_reports(settings.color);
    let verbosity = match (cli.quiet, cli.verbose) {
        (true, _) => Verbosity::Quiet,
        (_, true) => Verbosity::Verbose,
        _ => Verbosity::Normal,
    };
    let console = Console::new(settings.color, verbosity);
    run(cli.command, &settings, &console)
}

fn run(command: Command, settings: &Settings, console: &Console) -> miette::Result<()> {
    let peaks_dir = &settings.peaks_dir;
    let jobs = settings.jobs;
    let stdout_style = Style::for_stdout(settings.color);
    match command {
        Command::Index {
            library,
            max_track_minutes,
            retry_skipped,
        } => index::run(
            &settings.library(library)?,
            peaks_dir,
            jobs,
            max_track_minutes,
            retry_skipped,
            console,
        ),
        Command::Identify {
            audio,
            library,
            start,
            duration,
            playback,
            exclude_from,
            format,
            save_dir,
            again,
        } => identify::run(&identify::Request {
            audio: &audio,
            library: &settings.library(library)?,
            peaks_dir,
            excerpt: Excerpt { start, duration },
            playback: playback.unwrap_or(settings.playback),
            exclude_from: exclude_from.as_deref(),
            format,
            save_dir: save_dir.as_deref(),
            again,
            style: stdout_style,
            jobs,
            console,
        }),
        Command::Explain {
            audio,
            library,
            at,
            around,
            playback,
            asset,
            limit,
            exclude_from,
        } => explain::run(&explain::Request {
            audio: &audio,
            library: &settings.library(library)?,
            peaks_dir,
            exclude_from: exclude_from.as_deref(),
            at,
            around,
            playback: playback.unwrap_or(settings.playback),
            asset: asset.as_deref(),
            limit,
            style: stdout_style,
            jobs,
            console,
        }),
        Command::Show { report, format } => {
            let report = Report::read(&report)?;
            print!("{}", output::render(&report, format, stdout_style)?);
            Ok(())
        }
        Command::Listen {
            report,
            at,
            seconds,
            library,
            print,
        } => {
            let mut report = Report::read(&report)?;
            if let Some(library) = library {
                report.library = catalog::absolute(&library);
            }
            let clips = listen::clips(&report, at.as_secs_f64())?;
            if print {
                print!("{}", listen::commands(&clips, seconds));
                Ok(())
            } else {
                listen::play(&clips, seconds, console)
            }
        }
        Command::Review {
            report,
            seconds,
            library,
        } => {
            let mut report = Report::read(&report)?;
            if let Some(library) = library {
                report.library = catalog::absolute(&library);
            }
            review::run(&report, seconds, stdout_style, console)
        }
        Command::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "gunfinger",
                &mut std::io::stdout(),
            );
            Ok(())
        }
        Command::Man => clap_mangen::Man::new(Cli::command())
            .render(&mut std::io::stdout())
            .into_diagnostic(),
        Command::Doctor { library } => doctor::run(settings, library, stdout_style),
        Command::Prune {
            library,
            yes,
            force,
        } => prune::run(&prune::Request {
            library: &settings.library(library)?,
            peaks_dir,
            yes,
            force,
            console,
        }),
        Command::Stats { library, format } => {
            stats::run(&settings.library(library)?, peaks_dir, format, console)
        }
    }
}

/// miette decides on colour by itself unless told. Errors in the
/// configuration file itself are reported before this, with its choice.
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

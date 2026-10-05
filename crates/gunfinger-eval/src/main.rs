//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod calibrate;
mod clusters;
mod manifest;
mod render;
mod rng;
mod scan;
mod scoring;
mod survival;
mod sweep;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gunfinger_core::library::Library;
use gunfinger_core::store::PeakStore;
use gunfinger_core::timecode::format_timecode;

#[derive(Parser)]
#[command(name = "gunfinger-eval")]
struct Cli {
    /// Corpus root containing `library/` and `sets/`.
    #[arg(long, default_value = "corpus")]
    corpus: PathBuf,
    /// Directory for derived data: rendered audio, reports.
    #[arg(long, default_value = "work")]
    work: PathBuf,
    /// Peak store of the library, as written by `gunfinger index`.
    #[arg(long, default_value = "work/peaks")]
    peaks_dir: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check every set manifest against the library.
    Validate,
    /// Measure hash survival against residual speed error.
    Survival {
        /// Library assets to measure, relative to the library root.
        assets: Vec<String>,
    },
    /// Find duplicate clusters by matching the library against itself.
    Clusters,
    /// Run the seeded speed sweep.
    Sweep {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
    /// Search a set's audio and score it against its manifest.
    Scan {
        /// Name of the set directory under `sets/`.
        set: String,
        /// Leave this many referenced tracks (with their clusters) out of the
        /// index; their slots must then produce no confident detection.
        #[arg(long)]
        leave_out: Option<usize>,
        /// Seed for choosing the tracks to leave out.
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
    /// Report the confidence margin from the sweep and development reports.
    Calibrate {
        /// The development set whose scans (and leave-outs) are read.
        #[arg(long, default_value = "stakka-skynet-knowledge")]
        set: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Validate => validate(&cli.corpus),
        Command::Survival { assets } => open_library(&cli.corpus, &cli.peaks_dir)
            .and_then(|(library, store)| survival::run(&library, &store, &assets, &cli.work)),
        Command::Clusters => find_clusters(&cli),
        Command::Sweep { seed } => run_sweep(&cli, seed),
        Command::Calibrate { ref set } => calibrate::run(&cli.work.join("reports"), set),
        Command::Scan {
            ref set,
            leave_out,
            seed,
        } => run_scan(
            &cli,
            set,
            leave_out
                .map(|count| scan::LeaveOut { count, seed })
                .as_ref(),
        ),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn validate(corpus: &std::path::Path) -> Result<(), String> {
    let library = Library::scan(&corpus.join("library")).map_err(|error| error.to_string())?;
    let sets_dir = corpus.join("sets");
    let mut failures = 0;
    for name in manifest::set_names(&sets_dir)? {
        match manifest::load_set(&sets_dir, &name, &library) {
            Ok(set) => {
                let referenced = set
                    .tracks
                    .iter()
                    .filter(|track| track.is_referenced())
                    .count();
                let references: usize = set.tracks.iter().map(|track| track.references.len()).sum();
                println!(
                    "{name}: valid; {} tracks, {referenced} referenced ({references} reference files), {} absent",
                    set.tracks.len(),
                    set.tracks.len() - referenced
                );
                eprintln!("{name}: {} ({})", set.title, set.audio.display());
                for track in &set.tracks {
                    eprintln!(
                        "  {} {} [{} references]",
                        format_timecode(track.start),
                        track.label(),
                        track.references.len()
                    );
                }
            }
            Err(problems) => {
                failures += 1;
                println!("{name}: {} problems", problems.len());
                for problem in problems {
                    println!("  {problem}");
                }
            }
        }
    }
    if failures == 0 {
        Ok(())
    } else {
        Err(format!("{failures} manifests are invalid"))
    }
}

fn open_library(
    corpus: &std::path::Path,
    peaks_dir: &std::path::Path,
) -> Result<(Library, PeakStore), String> {
    let library = Library::scan(&corpus.join("library")).map_err(|error| error.to_string())?;
    let store = PeakStore::open(peaks_dir).map_err(|error| error.to_string())?;
    Ok((library, store))
}

fn jobs() -> usize {
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
}

fn clusters_path(cli: &Cli) -> PathBuf {
    cli.work.join("reports").join("duplicate-clusters.json")
}

fn find_clusters(cli: &Cli) -> Result<(), String> {
    let (library, store) = open_library(&cli.corpus, &cli.peaks_dir)?;
    let clusters = clusters::find(&library, &store, jobs())?;
    write_json(&clusters_path(cli), &clusters)?;
    println!("{}", clusters.criterion);
    println!("{} clusters with duplicates:", clusters.duplicates.len());
    for members in &clusters.duplicates {
        println!("  {}", members.join("  |  "));
    }
    Ok(())
}

fn run_sweep(cli: &Cli, seed: u64) -> Result<(), String> {
    let (library, store) = open_library(&cli.corpus, &cli.peaks_dir)?;
    let clusters = clusters::Clusters::load(&clusters_path(cli))?;
    let report = sweep::run(&library, &store, &clusters, &cli.work, seed, jobs())?;
    write_json(
        &cli.work
            .join("reports")
            .join(format!("sweep-seed-{seed}.json")),
        &report,
    )?;
    sweep::print_summary(&report);
    Ok(())
}

fn write_json(path: &std::path::Path, value: &impl serde::Serialize) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    std::fs::write(path, text).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

fn run_scan(cli: &Cli, set: &str, leave_out: Option<&scan::LeaveOut>) -> Result<(), String> {
    let (library, store) = open_library(&cli.corpus, &cli.peaks_dir)?;
    let clusters = clusters::Clusters::load(&clusters_path(cli))?;
    let sets_dir = cli.corpus.join("sets");
    let report = scan::run(
        &sets_dir,
        set,
        &library,
        &store,
        &clusters,
        leave_out,
        jobs(),
    )?;
    let name = match leave_out {
        Some(leave_out) => format!(
            "scan-{set}-leave-out-{}-seed-{}.json",
            leave_out.count, leave_out.seed
        ),
        None => format!("scan-{set}.json"),
    };
    write_json(&cli.work.join("reports").join(name), &report)?;
    scan::print_summary(&report);
    Ok(())
}

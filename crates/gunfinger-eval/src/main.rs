//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod calibrate;
mod clusters;
mod manifest;
mod regress;
mod render;
mod rng;
mod robust;
mod scan;
mod scoring;
mod survival;
mod sweep;

use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gunfinger_core::library::Library;
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::scan::LeaveOut;

#[derive(Parser)]
#[command(name = "gunfinger-eval")]
struct Cli {
    #[command(flatten)]
    paths: Paths,

    #[command(subcommand)]
    command: Command,
}

/// Where the harness reads from and writes to.
#[derive(clap::Args)]
struct Paths {
    /// Corpus root containing `library/` and `sets/`.
    #[arg(long, default_value = "corpus")]
    corpus: PathBuf,
    /// Directory for derived data: rendered audio and `reports/`.
    #[arg(long, default_value = "work")]
    work: PathBuf,
    /// Peak store of the library, as written by `gunfinger index`.
    #[arg(long, default_value = "work/peaks")]
    peaks_dir: PathBuf,
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
    /// Measure recall and wrong answers on the sweep's excerpts under
    /// transformations: EQ, noise, codecs, blends, speech, skips, pitch rides,
    /// speeds outside the ladder and key lock.
    Robust {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
        /// Run only these conditions (and the control), by name.
        #[arg(long, value_delimiter = ',')]
        only: Vec<String>,
    },
    /// Save the reports of the standard evaluation (sweep, development scan
    /// and leave-outs) as a named baseline under `work/baselines/`.
    Baseline {
        name: String,
        #[arg(long, default_value = "stakka-skynet-knowledge")]
        set: String,
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
    /// Rerun the standard evaluation and report what changed against a
    /// saved baseline.
    Regress {
        /// Name of the baseline under `work/baselines/`.
        name: String,
        /// Compare the existing reports without running the evaluation.
        #[arg(long)]
        no_rerun: bool,
        #[arg(long, default_value = "stakka-skynet-knowledge")]
        set: String,
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
}

fn main() -> ExitCode {
    let Cli { paths, command } = Cli::parse();
    match run(&paths, command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(paths: &Paths, command: Command) -> Result<(), String> {
    match command {
        Command::Validate => manifest::validate_all(&paths.sets(), &paths.library()?),
        Command::Survival { assets } => {
            survival::run(&paths.library()?, &paths.store()?, &assets, &paths.work)
        }
        Command::Clusters => find_clusters(paths),
        Command::Sweep { seed } => run_sweep(paths, seed),
        Command::Scan {
            set,
            leave_out,
            seed,
        } => run_scan(
            paths,
            &set,
            leave_out.map(|count| LeaveOut { count, seed }).as_ref(),
        ),
        Command::Calibrate { set } => calibrate::run(&paths.reports(), &set),
        Command::Robust { seed, only } => run_robust(paths, seed, &only),
        Command::Baseline { name, set, seed } => {
            regress::save(&paths.reports(), &paths.baseline(&name), &set, seed)
        }
        Command::Regress {
            name,
            no_rerun,
            set,
            seed,
        } => {
            if !no_rerun {
                run_standard_evaluation(paths, &set, seed)?;
            }
            regress::compare(&paths.baseline(&name), &paths.reports(), &set, seed)
        }
    }
}

fn run_robust(paths: &Paths, seed: u64, only: &[String]) -> Result<(), String> {
    let report = robust::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        seed,
        only,
        jobs(),
    )?;
    write_json(
        &paths.reports().join(format!("robust-seed-{seed}.json")),
        &report,
    )?;
    robust::print_summary(&report);
    Ok(())
}

/// The sweep, the development scan and its leave-outs.
fn run_standard_evaluation(paths: &Paths, set: &str, seed: u64) -> Result<(), String> {
    run_sweep(paths, seed)?;
    run_scan(paths, set, None)?;
    for count in regress::LEAVE_OUTS {
        run_scan(paths, set, Some(&LeaveOut { count, seed }))?;
    }
    Ok(())
}

fn find_clusters(paths: &Paths) -> Result<(), String> {
    let clusters = clusters::find(&paths.library()?, &paths.store()?, jobs())?;
    write_json(&paths.clusters_file(), &clusters)?;
    clusters::print_summary(&clusters);
    Ok(())
}

fn run_sweep(paths: &Paths, seed: u64) -> Result<(), String> {
    let report = sweep::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        seed,
        jobs(),
    )?;
    write_json(
        &paths.reports().join(format!("sweep-seed-{seed}.json")),
        &report,
    )?;
    sweep::print_summary(&report);
    Ok(())
}

fn run_scan(paths: &Paths, set: &str, leave_out: Option<&LeaveOut>) -> Result<(), String> {
    let report = scan::run(
        &paths.sets(),
        set,
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
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
    write_json(&paths.reports().join(name), &report)?;
    scan::print_summary(&report);
    Ok(())
}

impl Paths {
    fn library(&self) -> Result<Library, String> {
        let root = self.corpus.join("library");
        Library::scan(&root)
            .map_err(|error| format!("cannot read the library at {}: {error}", root.display()))
    }

    fn store(&self) -> Result<PeakStore, String> {
        PeakStore::open(&self.peaks_dir).map_err(|error| error.to_string())
    }

    fn sets(&self) -> PathBuf {
        self.corpus.join("sets")
    }

    fn reports(&self) -> PathBuf {
        self.work.join("reports")
    }

    fn baseline(&self, name: &str) -> PathBuf {
        self.work.join("baselines").join(name)
    }

    fn clusters_file(&self) -> PathBuf {
        self.reports().join("duplicate-clusters.json")
    }

    fn clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.clusters_file())
    }
}

/// Worker threads: one per core.
fn jobs() -> usize {
    std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    fs::write(path, text).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

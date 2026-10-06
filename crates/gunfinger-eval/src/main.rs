//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod calibrate;
mod clusters;
mod grid;
mod hash_cost;
mod manifest;
mod memory;
mod mixes;
mod regress;
mod related;
mod render;
mod rng;
mod robust;
mod scan;
mod scoring;
mod survival;
mod sweep;
mod synthetic;

use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use gunfinger_core::library::Library;
use gunfinger_core::speed::{Rung, key_lock_ladder, ladder};
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::scan::LeaveOut;

#[derive(Parser)]
#[command(name = "gunfinger-eval")]
struct Cli {
    #[command(flatten)]
    paths: Paths,

    /// Worker threads [default: one per core].
    #[arg(long, global = true)]
    jobs: Option<NonZeroUsize>,

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
    /// Rungs searched by sweep, scan, robust and regress; the default is
    /// `identify`'s. Reports of other ladders go to `reports/ladder-<name>/`,
    /// so that calibrate and regress read one ladder at a time.
    #[arg(long, global = true, value_enum, default_value_t = Ladder::Both)]
    ladder: Ladder,
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
    /// Estimate what pairs and triplets of peaks cost (postings, lookups)
    /// and keep (hashes at the true alignment) on the robustness excerpts.
    HashCost {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
    /// Find duplicate clusters by matching the library against itself.
    Clusters,
    /// List recordings that share material (remixes, VIPs, samples) by
    /// matching the library against itself.
    Related,
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
        /// A changed index; the report is then named `scale-scan-...` so
        /// that calibrate and regress never read it.
        #[command(flatten)]
        variant: IndexVariant,
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
        #[command(flatten)]
        variant: IndexVariant,
    },
    /// Render seeded mixes of library tracks with exact truth (speeds,
    /// bass swaps, crossfades, cuts, plays of 20-60 s, a returning track,
    /// held-out tracks), search them and score them against the truth.
    Mixes {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
        #[arg(long, default_value_t = 8)]
        count: usize,
    },
    /// Slide brief plays of indexed tracks across the 10 s window grid in
    /// 1 s steps, at several lengths and source positions, and compare
    /// the frozen rule with minimum aligned spans.
    Grid {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
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
    /// Measure resident memory after loading the peak records, building the
    /// index and searching a set's audio, the way `identify` does; with
    /// `--synthetic-copies`, on the scale proxy. Run under
    /// `/usr/bin/time -l` for the kernel's peak.
    Memory {
        #[arg(long, default_value = "stakka-skynet-knowledge")]
        set: String,
        #[arg(long, default_value_t = 0)]
        synthetic_copies: usize,
        /// Stop after this phase.
        #[arg(long, value_enum, default_value_t = memory::Phase::Searched)]
        until: memory::Phase,
        /// Search only this many minutes from the start of the set's audio.
        #[arg(long)]
        minutes: Option<u64>,
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
    let Cli {
        paths,
        jobs,
        command,
    } = Cli::parse();
    let jobs = jobs.map_or_else(available_parallelism, NonZeroUsize::get);
    match run(&paths, jobs, command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(paths: &Paths, jobs: usize, command: Command) -> Result<(), String> {
    match command {
        Command::Validate => manifest::validate_all(&paths.sets(), &paths.library()?),
        Command::Survival { assets } => {
            survival::run(&paths.library()?, &paths.store()?, &assets, &paths.work)
        }
        Command::HashCost { seed } => hash_cost::run(
            &paths.library()?,
            &paths.store()?,
            &paths.clusters()?,
            seed,
            &paths.work,
            jobs,
        ),
        Command::Clusters => find_clusters(paths, jobs),
        Command::Related => {
            let related =
                related::find(&paths.library()?, &paths.store()?, &paths.clusters()?, jobs)?;
            write_json(&paths.reports().join("related-recordings.json"), &related)?;
            related::print_summary(&related);
            Ok(())
        }
        Command::Sweep { seed } => run_sweep(paths, seed, jobs),
        Command::Scan {
            set,
            leave_out,
            seed,
            variant,
        } => run_scan(
            paths,
            &set,
            leave_out.map(|count| LeaveOut { count, seed }).as_ref(),
            &variant,
            jobs,
        ),
        Command::Calibrate { set } => calibrate::run(&paths.reports(), &set),
        Command::Robust {
            seed,
            only,
            variant,
        } => run_robust(paths, seed, &only, &variant, jobs),
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
                run_standard_evaluation(paths, &set, seed, jobs)?;
            }
            regress::compare(&paths.baseline(&name), &paths.reports(), &set, seed)
        }
        Command::Mixes { seed, count } => {
            let report = mixes::run(
                &paths.library()?,
                &paths.store()?,
                &paths.clusters()?,
                &paths.work,
                &mixes::Options {
                    seed,
                    count,
                    ladder_name: paths.ladder.name(),
                    ladder: &paths.ladder.rungs(),
                    jobs,
                },
            )?;
            write_json(
                &paths.reports().join(format!("mixes-seed-{seed}.json")),
                &report,
            )?;
            mixes::print_summary(&report);
            Ok(())
        }
        Command::Grid { seed } => {
            let report = grid::run(
                &paths.library()?,
                &paths.store()?,
                &paths.clusters()?,
                &paths.work,
                &grid::Options {
                    seed,
                    ladder_name: paths.ladder.name(),
                    ladder: &paths.ladder.rungs(),
                    jobs,
                },
            )?;
            write_json(
                &paths.reports().join(format!("grid-seed-{seed}.json")),
                &report,
            )?;
            grid::print_summary(&report);
            Ok(())
        }
        Command::Memory {
            set,
            synthetic_copies,
            until,
            minutes,
        } => {
            let report = memory::run(
                &paths.sets(),
                &paths.library()?,
                &paths.store()?,
                &memory::Options {
                    set: &set,
                    synthetic_copies,
                    until,
                    minutes,
                    ladder: &paths.ladder.rungs(),
                    jobs,
                },
            )?;
            write_json(
                &paths.work.join("memory").join(format!(
                    "memory-copies-{synthetic_copies}-jobs-{jobs}-until-{}.json",
                    format!("{until:?}").to_lowercase()
                )),
                &report,
            )?;
            memory::print_summary(&report);
            Ok(())
        }
    }
}

/// The turntable ladder, key-locked rungs at the same tempos, or both (the
/// default, as for `identify`).
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Ladder {
    Turntable,
    KeyLock,
    Both,
}

impl Ladder {
    fn name(self) -> &'static str {
        match self {
            Ladder::Turntable => "turntable",
            Ladder::KeyLock => "key-lock",
            Ladder::Both => "both",
        }
    }

    fn rungs(self) -> Vec<Rung> {
        match self {
            Ladder::Turntable => ladder(),
            Ladder::KeyLock => key_lock_ladder(),
            Ladder::Both => ladder().into_iter().chain(key_lock_ladder()).collect(),
        }
    }
}

/// Changes to the index for scale and cost experiments.
#[derive(clap::Args)]
struct IndexVariant {
    /// Add this many time-reversed, stretched copies of every indexed record
    /// to the index, to measure a larger library.
    #[arg(long, default_value_t = 0)]
    synthetic_copies: usize,
    /// Empty this share of the fullest posting lists (0.01 is the fullest
    /// 1%), to measure what common hashes cost and contribute.
    #[arg(long, default_value_t = 0.0)]
    drop_fullest: f64,
}

impl IndexVariant {
    fn check(&self) -> Result<(), String> {
        if self.synthetic_copies > synthetic::MAX_COPIES {
            return Err(format!(
                "at most {} synthetic copies are distinct",
                synthetic::MAX_COPIES
            ));
        }
        if !(0.0..1.0).contains(&self.drop_fullest) {
            return Err(String::from("--drop-fullest is a share below 1"));
        }
        Ok(())
    }

    /// Report name suffix; empty for the unchanged index.
    fn suffix(&self) -> String {
        let mut suffix = String::new();
        if self.synthetic_copies > 0 {
            suffix.push_str(&format!("-copies-{}", self.synthetic_copies));
        }
        if self.drop_fullest > 0.0 {
            suffix.push_str(&format!("-drop-{}", self.drop_fullest));
        }
        suffix
    }
}

const UNCHANGED_INDEX: IndexVariant = IndexVariant {
    synthetic_copies: 0,
    drop_fullest: 0.0,
};

fn run_robust(
    paths: &Paths,
    seed: u64,
    only: &[String],
    variant: &IndexVariant,
    jobs: usize,
) -> Result<(), String> {
    let ladder = paths.ladder;
    variant.check()?;
    let report = robust::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &robust::Options {
            seed,
            only,
            ladder_name: ladder.name(),
            ladder: &ladder.rungs(),
            synthetic_copies: variant.synthetic_copies,
            drop_fullest: variant.drop_fullest,
            jobs,
        },
    )?;
    let suffix = variant.suffix();
    write_json(
        &paths
            .reports()
            .join(format!("robust-seed-{seed}{suffix}.json")),
        &report,
    )?;
    robust::print_summary(&report);
    Ok(())
}

/// The sweep, the development scan and its leave-outs.
fn run_standard_evaluation(paths: &Paths, set: &str, seed: u64, jobs: usize) -> Result<(), String> {
    run_sweep(paths, seed, jobs)?;
    run_scan(paths, set, None, &UNCHANGED_INDEX, jobs)?;
    for count in regress::LEAVE_OUTS {
        run_scan(
            paths,
            set,
            Some(&LeaveOut { count, seed }),
            &UNCHANGED_INDEX,
            jobs,
        )?;
    }
    Ok(())
}

fn find_clusters(paths: &Paths, jobs: usize) -> Result<(), String> {
    let clusters = clusters::find(&paths.library()?, &paths.store()?, jobs)?;
    write_json(&paths.clusters_file(), &clusters)?;
    clusters::print_summary(&clusters);
    Ok(())
}

fn run_sweep(paths: &Paths, seed: u64, jobs: usize) -> Result<(), String> {
    let report = sweep::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        seed,
        &paths.ladder.rungs(),
        jobs,
    )?;
    write_json(
        &paths.reports().join(format!("sweep-seed-{seed}.json")),
        &report,
    )?;
    sweep::print_summary(&report);
    Ok(())
}

fn run_scan(
    paths: &Paths,
    set: &str,
    leave_out: Option<&LeaveOut>,
    variant: &IndexVariant,
    jobs: usize,
) -> Result<(), String> {
    variant.check()?;
    let report = scan::run(
        &paths.sets(),
        set,
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &scan::Options {
            leave_out,
            synthetic_copies: variant.synthetic_copies,
            drop_fullest: variant.drop_fullest,
            ladder: &paths.ladder.rungs(),
            jobs,
        },
    )?;
    let suffix = variant.suffix();
    let name = match leave_out {
        _ if !suffix.is_empty() => format!("scale-scan-{set}{suffix}.json"),
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
        let reports = self.work.join("reports");
        match self.ladder {
            Ladder::Both => reports,
            other => reports.join(format!("ladder-{}", other.name())),
        }
    }

    fn baseline(&self, name: &str) -> PathBuf {
        self.work.join("baselines").join(name)
    }

    /// Clusters come from library audio alone, whatever the ladder.
    fn clusters_file(&self) -> PathBuf {
        self.work.join("reports").join("duplicate-clusters.json")
    }

    fn clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.clusters_file())
    }
}

/// One worker thread per core.
fn available_parallelism() -> usize {
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

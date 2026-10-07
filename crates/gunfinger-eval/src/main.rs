//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod calibrate;
mod clusters;
mod fullest;
mod grid;
mod hash_cost;
mod library_map;
mod loss;
mod manifest;
mod matching;
mod memory;
mod mixes;
mod padding;
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
mod tempo;

use std::collections::BTreeSet;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::{Rung, key_lock_ladder_with_extra_rungs, ladder_with_extra_rungs};
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::library_map::LibraryMap;
use crate::matching::Matching;
use crate::padding::{Padding, SecondLibrary};
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
    /// Saved sweep panels: each seed's held-out recordings and excerpts,
    /// drawn the first time the seed is used and kept as the library grows.
    #[arg(long, default_value = "docs/panels")]
    panels: PathBuf,
    /// Rungs searched by sweep, scan, robust and regress; the default is
    /// `identify`'s. Reports of other ladders go to `reports/ladder-<name>/`,
    /// so that calibrate and regress read one ladder at a time.
    #[arg(long, global = true, value_enum, default_value_t = Ladder::Both)]
    ladder: Ladder,
    /// Extend each ladder by this many rungs past either end, a step
    /// (0.4%) apart; reports go to `reports/extra-rungs-<n>/`.
    #[arg(long, global = true, default_value_t = 0)]
    extra_rungs: u32,
    /// Opt-in matching changes; their reports go to
    /// `reports/variant-<name>/`.
    #[command(flatten)]
    matching: Matching,
    /// The peak store of a larger library the corpus was drawn from, which
    /// holds a copy of every corpus file; its library need not be mounted.
    /// `map-library` pairs corpus files with their copies there and
    /// `clusters --from-peaks` finds the corpus recordings' other rips;
    /// then `sweep`, `scan`, `robust` and `memory` search an index of the
    /// corpus and the other library's remaining records (named
    /// `second-library/<path>`), counting those rips as correct. Reports go
    /// to `reports/library-<store directory name>/`.
    #[arg(long, global = true)]
    other_peaks_dir: Option<PathBuf>,
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
    /// Pair each corpus file with its copies in the store of
    /// --other-peaks-dir: files with an identical peak record.
    MapLibrary,
    /// Find duplicate clusters by matching the library against itself.
    Clusters {
        /// Search each file's stored peaks instead of its decoded audio,
        /// and compare the result with the clusters in use rather than
        /// replacing them.
        #[arg(long)]
        from_peaks: bool,
    },
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
    /// Attribute where a set's identified plays lose evidence: hits per
    /// window against the reference hashes heard, on the rung, at the
    /// fitted and the best local speed, beside a clean render of the same
    /// stretch. For the development set only.
    Loss {
        #[arg(long, default_value = "stakka-skynet-knowledge")]
        set: String,
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
        #[command(flatten)]
        variant: IndexVariant,
        /// Stop after this phase.
        #[arg(long, value_enum, default_value_t = memory::Phase::Searched)]
        until: memory::Phase,
        /// Search only this many minutes from the start of the set's audio.
        #[arg(long)]
        minutes: Option<u64>,
        /// Keep the first pass's lines and count them, instead of
        /// searching the way `identify` does.
        #[arg(long)]
        count_lines: bool,
    },
    /// Count each indexed record's postings in the fullest posting lists,
    /// the lists `--skip-fullest` sets aside when looking for candidates.
    Fullest {
        /// The share of non-empty lists, as for `--skip-fullest`.
        #[arg(long, default_value_t = 0.01)]
        share: f64,
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
    match paths
        .matching
        .check()
        .and_then(|()| run(&paths, jobs, command))
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(paths: &Paths, jobs: usize, command: Command) -> Result<(), String> {
    let takes_other_library = matches!(
        command,
        Command::MapLibrary
            | Command::Clusters { from_peaks: true }
            | Command::Sweep { .. }
            | Command::Scan { .. }
            | Command::Calibrate { .. }
            | Command::Robust { .. }
            | Command::Baseline { .. }
            | Command::Regress { .. }
            | Command::Memory { .. }
            | Command::Fullest { .. }
    );
    if paths.other_peaks_dir.is_some() && !takes_other_library {
        return Err(String::from(
            "this command does not take --other-peaks-dir; map-library, clusters --from-peaks, sweep, scan, calibrate, robust, baseline, regress, memory and fullest do",
        ));
    }
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
            &paths.panels,
            &paths.work,
            jobs,
        ),
        Command::MapLibrary => {
            let (Some(other), Some(map_file)) = (paths.other_store()?, paths.map_file()) else {
                return Err(String::from("map-library needs --other-peaks-dir"));
            };
            let (other_assets, problems) = other
                .current_sources(&Profile::CURRENT)
                .map_err(|error| error.to_string())?;
            for problem in &problems {
                eprintln!("left out: {problem}");
            }
            let map =
                library_map::build(&paths.library()?, &paths.store()?, &other, &other_assets)?;
            write_json(&map_file, &map)?;
            library_map::print_summary(&map);
            Ok(())
        }
        Command::Clusters { from_peaks } => find_clusters(paths, from_peaks, jobs),
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
        Command::Calibrate { set } => calibrate::run(&paths.reports(), &set, paths.matching.rule()),
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
                    panels: &paths.panels,
                    count,
                    ladder_name: paths.ladder.name(),
                    ladder: &paths.rungs(),
                    matching: &paths.matching,
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
                    panels: &paths.panels,
                    ladder_name: paths.ladder.name(),
                    ladder: &paths.rungs(),
                    matching: &paths.matching,
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
        Command::Loss { set } => {
            let report = loss::run(
                &paths.sets(),
                &paths.library()?,
                &paths.store()?,
                &paths.clusters()?,
                &paths.work,
                &loss::Options {
                    set: &set,
                    ladder: &paths.rungs(),
                    jobs,
                },
            )?;
            write_json(&paths.reports().join(format!("loss-{set}.json")), &report)?;
            loss::print_summary(&report);
            Ok(())
        }
        Command::Fullest { share } => {
            if !(share > 0.0 && share < 1.0) {
                return Err(String::from("--share is a share above 0 and below 1"));
            }
            let report = fullest::run(
                &paths.library()?,
                &paths.store()?,
                &paths.padding(&UNCHANGED_INDEX)?,
                share,
            )?;
            write_json(
                &paths.base_reports().join(format!("fullest-{share}.json")),
                &report,
            )?;
            fullest::print_summary(&report);
            Ok(())
        }
        Command::Memory {
            set,
            variant,
            until,
            minutes,
            count_lines,
        } => {
            let report = memory::run(
                &paths.sets(),
                &paths.library()?,
                &paths.store()?,
                &memory::Options {
                    set: &set,
                    padding: &paths.padding(&variant)?,
                    until,
                    minutes,
                    count_lines,
                    ladder: &paths.rungs(),
                    matching: paths.matching,
                    jobs,
                },
            )?;
            let matcher = paths
                .matching
                .name()
                .map_or_else(String::new, |name| format!("-{name}"));
            write_json(
                &paths.work.join("memory").join(format!(
                    "memory{}{}-jobs-{jobs}-until-{}{matcher}{}.json",
                    paths
                        .other_name()
                        .map_or_else(String::new, |name| format!("-library-{name}")),
                    variant.suffix(),
                    format!("{until:?}").to_lowercase(),
                    if count_lines { "-lines" } else { "" }
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

    fn rungs(self, extra: u32) -> Vec<Rung> {
        let turntable = || ladder_with_extra_rungs(extra);
        let key_lock = || key_lock_ladder_with_extra_rungs(extra);
        match self {
            Ladder::Turntable => turntable(),
            Ladder::KeyLock => key_lock(),
            Ladder::Both => turntable().into_iter().chain(key_lock()).collect(),
        }
    }
}

/// A larger index for scale experiments (`padding`).
#[derive(clap::Args)]
struct IndexVariant {
    /// Add this many time-reversed, stretched copies of every indexed record
    /// to the index, to measure a larger library.
    #[arg(long, default_value_t = 0)]
    synthetic_copies: usize,
    /// Add the current peak records of a second library to the index, named
    /// `second-library/<path>`, to measure a larger real library: the
    /// library's root directory...
    #[arg(long, requires = "second_peaks_dir")]
    second_library: Option<PathBuf>,
    /// ...and its peak store, as `gunfinger index <root> --peaks-dir`
    /// wrote it.
    #[arg(long, requires = "second_library")]
    second_peaks_dir: Option<PathBuf>,
}

impl IndexVariant {
    /// What to add to the index, with the second library opened.
    fn padding(&self) -> Result<Padding, String> {
        if self.synthetic_copies > synthetic::MAX_COPIES {
            return Err(format!(
                "at most {} synthetic copies are distinct",
                synthetic::MAX_COPIES
            ));
        }
        let second = match (&self.second_library, &self.second_peaks_dir) {
            (Some(root), Some(peaks_dir)) => Some(SecondLibrary::open(root, peaks_dir)?),
            _ => None,
        };
        Ok(Padding {
            copies: self.synthetic_copies,
            second,
        })
    }

    /// Report name suffix; empty for the unchanged index.
    fn suffix(&self) -> String {
        variant_suffix(self.synthetic_copies, self.second_library.is_some())
    }
}

fn variant_suffix(synthetic_copies: usize, second_library: bool) -> String {
    let mut suffix = String::new();
    if second_library {
        suffix.push_str("-second-library");
    }
    if synthetic_copies > 0 {
        suffix.push_str(&format!("-copies-{synthetic_copies}"));
    }
    suffix
}

const UNCHANGED_INDEX: IndexVariant = IndexVariant {
    synthetic_copies: 0,
    second_library: None,
    second_peaks_dir: None,
};

fn run_robust(
    paths: &Paths,
    seed: u64,
    only: &[String],
    variant: &IndexVariant,
    jobs: usize,
) -> Result<(), String> {
    let ladder = paths.ladder;
    paths.check_panel(seed)?;
    let padding = paths.padding(variant)?;
    let report = robust::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &robust::Options {
            seed,
            panels: &paths.panels,
            only,
            ladder_name: ladder.name(),
            ladder: &paths.rungs(),
            padding: &padding,
            matching: &paths.matching,
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

fn find_clusters(paths: &Paths, from_peaks: bool, jobs: usize) -> Result<(), String> {
    if let (Some(other), Some(map_file)) = (paths.other_store()?, paths.map_file()) {
        return find_clusters_around(paths, &other, &map_file, jobs);
    }
    let source = if from_peaks {
        clusters::Source::Peaks
    } else {
        clusters::Source::Audio
    };
    let clusters = clusters::find(&paths.library()?, &paths.store()?, source, jobs)?;
    clusters::print_summary(&clusters);
    if from_peaks {
        write_json(
            &paths
                .work
                .join("reports")
                .join("duplicate-clusters-from-peaks.json"),
            &clusters,
        )?;
        if let Ok(in_use) = paths.clusters() {
            clusters::print_differences("peaks", &clusters, "audio", &in_use);
        }
    } else {
        write_json(&paths.clusters_file(), &clusters)?;
    }
    Ok(())
}

/// The corpus clusters with the other library's rips of the corpus
/// recordings, which the evaluations against both libraries read.
fn find_clusters_around(
    paths: &Paths,
    other: &PeakStore,
    map_file: &Path,
    jobs: usize,
) -> Result<(), String> {
    let map = LibraryMap::load(map_file)?;
    let corpus = paths.corpus_clusters()?;
    let profile = Profile::CURRENT;
    let (other_assets, problems) = other
        .current_sources(&profile)
        .map_err(|error| error.to_string())?;
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let (queries, _) = load_records(
        &paths.library()?,
        &paths.store()?,
        &profile,
        &BTreeSet::new(),
    );
    let copies = map.copied();
    let pairs = clusters::find_around(queries, other, &other_assets, &copies, jobs)?;
    clusters::print_around(&corpus, &pairs, &copies);
    let name = map
        .other_library
        .unwrap_or_else(|| other.dir().display().to_string());
    let merged = clusters::merged(&corpus, pairs, &name);
    write_json(&paths.clusters_file(), &merged)?;
    println!(
        "{} clusters with duplicates, written to {}",
        merged.duplicates.len(),
        paths.clusters_file().display()
    );
    Ok(())
}

fn run_sweep(paths: &Paths, seed: u64, jobs: usize) -> Result<(), String> {
    paths.check_panel(seed)?;
    let report = sweep::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &sweep::Options {
            seed,
            panels: &paths.panels,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            ladder: &paths.rungs(),
            matching: &paths.matching,
            jobs,
        },
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
    let padding = paths.padding(variant)?;
    let report = scan::run(
        &paths.sets(),
        set,
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &scan::Options {
            leave_out,
            padding: &padding,
            matching: &paths.matching,
            ladder: &paths.rungs(),
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

    fn rungs(&self) -> Vec<Rung> {
        self.ladder.rungs(self.extra_rungs)
    }

    fn reports(&self) -> PathBuf {
        let mut reports = self.base_reports();
        if self.ladder != Ladder::Both {
            reports.push(format!("ladder-{}", self.ladder.name()));
        }
        if self.extra_rungs > 0 {
            reports.push(format!("extra-rungs-{}", self.extra_rungs));
        }
        if let Some(name) = self.matching.name() {
            reports.push(format!("variant-{name}"));
        }
        reports
    }

    fn baseline(&self, name: &str) -> PathBuf {
        self.work.join("baselines").join(name)
    }

    /// `reports/`, or with another library its own directory there.
    fn base_reports(&self) -> PathBuf {
        let reports = self.work.join("reports");
        match self.other_name() {
            Some(name) => reports.join(format!("library-{name}")),
            None => reports,
        }
    }

    /// Clusters come from library audio alone, whatever the ladder; with
    /// another library, they are the corpus clusters with the other
    /// library's rips of the corpus recordings (`clusters --from-peaks`).
    fn clusters_file(&self) -> PathBuf {
        self.base_reports().join("duplicate-clusters.json")
    }

    fn clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.clusters_file())
    }

    /// The corpus clusters, also when another library is given.
    fn corpus_clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.work.join("reports").join("duplicate-clusters.json"))
    }

    /// The name of the other library's store directory.
    fn other_name(&self) -> Option<String> {
        let dir = self.other_peaks_dir.as_ref()?;
        let name = dir
            .canonicalize()
            .unwrap_or_else(|_| dir.clone())
            .file_name()?
            .to_string_lossy()
            .into_owned();
        Some(name)
    }

    /// The other library's store, which must exist.
    fn other_store(&self) -> Result<Option<PeakStore>, String> {
        let Some(dir) = &self.other_peaks_dir else {
            return Ok(None);
        };
        if !dir.is_dir() {
            return Err(format!("no peak store at {}", dir.display()));
        }
        PeakStore::open(dir)
            .map(Some)
            .map_err(|error| error.to_string())
    }

    fn map_file(&self) -> Option<PathBuf> {
        self.other_peaks_dir
            .as_ref()
            .map(|_| self.base_reports().join("library-map.json"))
    }

    /// With another library, the seed's panel must have been drawn from
    /// the corpus alone: its clusters would put the other library's files
    /// among the held-out recordings.
    fn check_panel(&self, seed: u64) -> Result<(), String> {
        let panel = self.panels.join(format!("sweep-seed-{seed}.json"));
        if self.other_peaks_dir.is_some() && !panel.exists() {
            return Err(format!(
                "{} does not exist; draw it with `sweep --seed {seed}` without --other-peaks-dir first",
                panel.display()
            ));
        }
        Ok(())
    }

    /// The variant's padding, with the other library's records less the
    /// copies corpus files stand for.
    fn padding(&self, variant: &IndexVariant) -> Result<Padding, String> {
        let mut padding = variant.padding()?;
        let (Some(store), Some(map_file)) = (self.other_store()?, self.map_file()) else {
            return Ok(padding);
        };
        if padding.second.is_some() {
            return Err(String::from(
                "--second-library and --other-peaks-dir cannot be combined",
            ));
        }
        let map = LibraryMap::load(&map_file)?;
        padding.second = Some(SecondLibrary::from_store(store, &map.stood_for())?);
        Ok(padding)
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

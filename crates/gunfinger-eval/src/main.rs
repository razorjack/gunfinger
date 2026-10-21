//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod audition;
mod calibrate;
mod clusters;
mod fullest;
mod gaps;
mod grid;
mod hash_cost;
mod holders;
mod identifiability;
mod library_map;
mod loss;
mod manifest;
mod matching;
mod memory;
mod mixes;
mod padding;
mod pair;
mod pair_review;
mod platform;
mod recall;
mod regress;
mod related;
mod render;
mod rng;
mod robust;
mod scan;
mod scoring;
mod seconds;
mod self_match;
mod shared;
mod shared_mixes;
mod survival;
mod sweep;
mod synthetic;
mod tempo;
mod verifier;
mod versions;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::Duration;

use clap::{Parser, Subcommand, ValueEnum};
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::ignore::{self, IGNORE_FILE};
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::{Rung, key_lock_ladder_with_extra_rungs, ladder_with_extra_rungs};
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::library_map::LibraryMap;
use crate::manifest::Referable;
use crate::matching::Matching;
use crate::padding::{Padding, SECOND_LIBRARY_PREFIX, SavedIndex, SecondLibrary};
use crate::scan::{LeaveOut, SlotRun};

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
    /// The verdicts by ear on pairs of files (same recording or
    /// different), which `clusters` follows over its coverage rule.
    #[arg(long, default_value = "docs/pair-verdicts.txt")]
    verdicts: PathBuf,
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
    /// With --other-peaks-dir: index only this many of the other library's
    /// records, a seeded random choice, for measuring against the size of
    /// the index. Reports go to `reports/library-<store>/sample-<count>/`;
    /// the map and the clusters are the whole library's.
    #[arg(long, global = true, requires = "other_peaks_dir")]
    other_sample: Option<usize>,
    /// With --other-peaks-dir: whether runs load the index of the corpus
    /// and the other library's records, saved in
    /// `indexes/library-<store directory name>.index` (ADR 0014). Runs that
    /// leave files out remove them from it. Not used with --other-sample,
    /// synthetic copies or `memory`.
    #[arg(long, global = true, value_enum, default_value_t = SavedIndexUse::Use)]
    saved_index: SavedIndexUse,
    /// Measure every detection of sweep, scan, robust, mixes and grid, and
    /// those of recall the second pass counted, with the peak verifier, a
    /// diagnostic that leaves detections unchanged. Reports go to
    /// `reports/verified/` below the usual directory.
    #[arg(long, global = true)]
    verify: bool,
}

#[derive(Subcommand)]
enum Command {
    /// Check every set manifest against the library; with --other-peaks-dir,
    /// `second-library/<path>` references against the other store.
    Validate {
        /// Check only this set (a directory under `sets/`).
        set: Option<String>,
    },
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
        /// With --other-peaks-dir: search only this many corpus files,
        /// spread over the library, and no further rounds, for timing. The
        /// clusters in use are not replaced; the pairs go to
        /// `duplicate-clusters-sample.json`.
        #[arg(long, requires = "from_peaks")]
        sample: Option<usize>,
        /// With --other-peaks-dir: also search the other library's files
        /// this set's manifest references (repeatable), so that their rips
        /// join their clusters.
        #[arg(long = "manifest", requires = "from_peaks")]
        manifests: Vec<String>,
        /// With --other-peaks-dir: also search the development sources of
        /// the recall panel of this seed.
        #[arg(long, requires = "from_peaks")]
        recall_panel: Option<u64>,
        /// When a chain of joins links two files judged
        /// different, cut the chain's join with the fewest hits per second
        /// of aligned span and list it, instead of stopping. A cut is a
        /// guess to judge by ear, not a verdict.
        #[arg(long)]
        cut_sparsest: bool,
        /// With --other-peaks-dir: cluster the last run's pairs again with
        /// the current verdicts, without searching (seconds, not hours):
        /// those of the clusters in use, or of the report given, such as
        /// the `duplicate-clusters-stopped.json` a search keeps when a
        /// verdict stops it. Pairs with a file pruned or ignored since are
        /// dropped; files indexed since that run are not searched.
        #[arg(
            long,
            value_name = "REPORT",
            num_args = 0..=1,
            requires = "from_peaks",
            conflicts_with_all = ["sample", "manifests", "recall_panel"]
        )]
        reuse_pairs: Option<Option<PathBuf>>,
    },
    /// List recordings that share material (remixes, VIPs, samples) by
    /// matching the library against itself.
    Related,
    /// Step through the listening pack by ear: each item without a verdict
    /// in --verdicts shows its sheet in short and plays its clips with
    /// ffplay; `s` or `d` appends the `same` or `different` line the sheet
    /// shows. At the end it prints the `clusters --reuse-pairs` command that
    /// applies the new verdicts.
    PairReview {
        /// The listening pack, with `README.md` and the item folders
        /// [default: <work>/listening].
        #[arg(long)]
        pack: Option<PathBuf>,
    },
    /// Serve the listening page (ADR 0016) on 127.0.0.1 and open it in the
    /// default browser, to answer the questions of the question
    /// files by ear, one key per answer. Answers go to the answer log and,
    /// for pairs, `same` and `different` to --verdicts.
    Audition {
        /// Question files [default: every `*.json` in
        /// <work>/audition/questions/, in name order].
        files: Vec<PathBuf>,
        /// The answer log [default: <work>/audition/answers.jsonl].
        #[arg(long)]
        answers: Option<PathBuf>,
        /// Print the page's address without opening a browser.
        #[arg(long)]
        no_open: bool,
        /// The port to listen on [default: a free one].
        #[arg(long, default_value_t = 0)]
        port: u16,
    },
    /// Write a question file for `audition` from evidence: the listening
    /// pack, release folders, or a report's detections.
    Questions {
        #[command(subcommand)]
        source: QuestionSource,
    },
    /// Search each pair's first file against its second alone, on several
    /// ladders and at the fitted speed, and list every alignment: why two
    /// rips cover less of each other than the clustering rule needs.
    Pair {
        /// A file of pairs, one per line: two paths separated by a tab,
        /// corpus library paths or `second-library/<path>` with
        /// --other-peaks-dir. The report is `pairs-<file stem>.json`.
        pairs: PathBuf,
    },
    /// Search queries cut from a passage two different recordings share
    /// (alone, looped, with the played file's own material before or
    /// after it, through a weak rip, under a blend, a short break
    /// repeated), with the played recording in the index and left out,
    /// and measure every detection with the peak verifier. With
    /// --other-peaks-dir every index holds that library too.
    Shared {
        /// The scenarios: JSON with `scenarios`, each naming `name`,
        /// `played`, `related`, `passage_start_seconds`,
        /// `passage_end_seconds` and optionally `played_recording` and
        /// `related_recording` (files of each recording beyond its
        /// duplicate cluster), and optionally `blend_partner` (`file`,
        /// `start_seconds`). The report is `<plan file stem>.json`, with
        /// `shared-` put first when the stem does not start with it.
        plan: PathBuf,
    },
    /// Session 21's rendered mixes: for each seed, one mix per played file
    /// of a plan, playing it whole among drawn tracks, searched with the
    /// played recording indexed, left out and (where another rip is
    /// indexed) with the played file alone left out; and the plan's other
    /// audio searched with everything indexed. Every detection carries
    /// the verifier's share, the file's own material on each side, and,
    /// when confident, the strongest other indexed file holding its
    /// passage. The report is `<plan file stem>.json`.
    SharedMixes {
        /// The plan, as `scripts/analysis/shared_mixes.py plan` writes it.
        plan: PathBuf,
    },
    /// The library's map of shared passages (`shared-mixes`'s holder
    /// lookup) for a list of files and spans: each searched against the
    /// index without the files its lookup names, listing every file found
    /// with 30 hits or more. The report is `holders-<file stem>.json`.
    Holders {
        /// JSON: a list of `name`, `asset`, `track_start_seconds`,
        /// `track_end_seconds` and optionally `left_out`.
        lookups: PathBuf,
    },
    /// Proposal 5's diagnostic: for each direction of one half of a
    /// shared-material panel, how closely the related file and the played
    /// recording's other rips reproduce the shared passage, in pair hashes,
    /// peaks and audio (searched against each file alone). The report is
    /// `identifiability-<half>.json`.
    Identifiability {
        /// The panel, as `scripts/analysis/shared_panel.py` writes it.
        panel: PathBuf,
        #[arg(long, value_enum)]
        half: Half,
    },
    /// The diagnostic version verifier (sessions 24 and 26): for each play
    /// of a list, each segment's line measured from the reference's own
    /// hits, the reference rendered along it and compared with the mix in
    /// 2 s patches (supported, contradicted or unobservable; distinctive
    /// or held by a sibling), with three summaries per play. Decides
    /// nothing.
    Versions {
        /// JSON: `plays`, as `scripts/analysis/version_plays.py` writes them.
        plays: PathBuf,
        /// Where to write the timelines and summaries.
        #[arg(long)]
        out: PathBuf,
    },
    /// With --other-peaks-dir: draw the recall panel for a seed from the
    /// other library's records outside the corpus recordings' clusters
    /// (kept in `panels/recall-seed-<seed>.json`), render its development
    /// half's excerpts from that library's audio, and search them.
    Recall {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
        /// Draw the panel and render the excerpts, without searching.
        #[arg(long)]
        prepare: bool,
        /// Leave each indexed source's record and its identical copies out
        /// of the index, so that only its recording's other rips can
        /// answer; a source without one is unobservable, scored like a
        /// held-out source (`-other-rips` in the report's name).
        #[arg(long)]
        other_rips_only: bool,
        /// The query sets, searched in this order against one index, each
        /// written as it finishes: the excerpts as rendered (`clean`),
        /// under `robust`'s combined damage (`damaged`), or their first
        /// 15 and 20 s between held-out audio (`brief`, `brief-damaged`).
        #[arg(long, value_enum, value_delimiter = ',', default_value = "clean")]
        queries: Vec<recall::QuerySet>,
    },
    /// Run the seeded speed sweep.
    Sweep {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
        /// With --other-peaks-dir: search only the indexed excerpts whose
        /// recording has another rip that is not an identical copy, with
        /// each such excerpt's source file and its identical copies left
        /// out of the index, so that only the other rips can answer
        /// (`sweep-seed-<seed>-other-rips.json`).
        #[arg(long)]
        other_rips: bool,
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
        /// Leave these manifest slots (with their clusters) out of the
        /// index instead, as runs of consecutive slots: `3,7-8,12-14`. The
        /// report is `scan-<set>-leave-out-slots-3_7-8_12-14.json`.
        #[arg(
            long,
            value_delimiter = ',',
            value_parser = SlotRun::parse,
            conflicts_with = "leave_out"
        )]
        leave_out_slots: Vec<SlotRun>,
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
        /// `long`: plays of 2-6 minutes joined by crossfades of 10-60 s,
        /// with runs of 1-4 held-out tracks between indexed ones (report
        /// `mixes-long-seed-<seed>.json`).
        #[arg(long, value_enum, default_value = "short")]
        kind: mixes::MixKind,
    },
    /// Slide brief plays of indexed tracks across the 10 s window grid in
    /// 1 s steps, at several lengths and source positions, and compare
    /// the frozen rule with minimum aligned spans.
    Grid {
        #[arg(long, default_value_t = 2026)]
        seed: u64,
    },
    /// Score saved `mixes`, `shared-mixes` or `scan` reports second by
    /// second (Sonnleitner et al. 2016, section 5): seconds labelled
    /// correctly, possibly or wrongly, or not at all, while an indexed
    /// track plays, and silence while none does (specificity); and where
    /// each identified track's labels start and end. Searches nothing and leaves the reports and
    /// their scores as they are. The scores go to `seconds/` (with another
    /// library, `seconds/library-<store directory name>/`)
    Seconds {
        /// Reports written by `mixes`, `shared-mixes` (each rendered mix
        /// under each index) or `scan` (a scan of the larger library with
        /// --other-peaks-dir).
        #[arg(required = true)]
        reports: Vec<PathBuf>,
        /// Seconds either side of each manifest border in which both
        /// neighbouring tracks are right and an unlabelled second is
        /// counted apart. Scans only: generated and rendered mixes have
        /// exact truth.
        #[arg(long, default_value_t = 30.0)]
        margin: f64,
        /// Also score with this minimum span from the first to the last
        /// aligned hit in place of 3 windows, as `grid` compares.
        #[arg(long)]
        min_span: Vec<f64>,
        /// For scans, also where each track's labels start and end against
        /// the manifest's approximate starts.
        #[arg(long)]
        boundaries: bool,
    },
    /// List the gaps of saved `mixes`, `shared-mixes` or `scan` reports:
    /// the stretches no segment of a confident play covers, at each minimum
    /// length, with how many tracks each holds by its length over the
    /// median interval between the found entries (method 1), against the
    /// truth: the left-out, held-out or absent tracks each gap holds
    /// (`docs/research/2026-10-21-gap-measures.md`). Searches nothing. The
    /// listings go to `gaps/` (with another library,
    /// `gaps/library-<store directory name>/`); the summary counts every
    /// gap of every report given.
    Gaps {
        #[arg(required = true)]
        reports: Vec<PathBuf>,
        /// Seconds either side of each manifest border that count as a
        /// transition. Scans only: generated and rendered mixes have exact
        /// truth.
        #[arg(long, default_value_t = 30.0)]
        margin: f64,
        #[arg(long, value_delimiter = ',', default_values_t = gaps::MIN_LENGTHS)]
        min_length: Vec<f64>,
    },
    /// A prototype of counting the tracks in a gap by matching the gap
    /// against itself: each listed gap of saved `mixes` or `scan` reports
    /// (as `gaps` lists them, at one minimum length) is decoded, its pair
    /// hashes matched with its own at speed 1.0, and the stretches joined
    /// by repeats counted as blocks, at several thresholds, beside the
    /// truth and method 1's estimate. The listing goes to
    /// `gaps/self-match-<report stem>.json` (with another library, under
    /// `gaps/library-<store directory name>/`).
    SelfMatch {
        #[arg(required = true)]
        reports: Vec<PathBuf>,
        #[arg(long, default_value_t = 30.0)]
        margin: f64,
        #[arg(long, default_value_t = 30.0)]
        min_length: f64,
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
    /// `/usr/bin/time -l` (macOS) or `/usr/bin/time -v` (Linux) for the
    /// kernel's peak.
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

/// Where `questions` takes its evidence from.
#[derive(Subcommand)]
enum QuestionSource {
    /// The listening pack's items, in its README's order, as pair
    /// questions; written to <work>/audition/questions/01-pack.json.
    Pack {
        /// The pack [default: <work>/listening].
        #[arg(long)]
        pack: Option<PathBuf>,
        /// Question texts and extra lanes by item number (TOML; see
        /// `docs/audition-questions.md`).
        #[arg(long)]
        plan: Option<PathBuf>,
        /// `pair` reports whose exact alignments replace the sheets'
        /// rounded ones, and which place extra lanes (repeatable).
        #[arg(long = "pair-report")]
        pair_reports: Vec<PathBuf>,
        #[command(flatten)]
        roots: SecondLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Release folders from a plan (TOML), each asked "is this a mixed
    /// CD?"; written to <work>/audition/questions/02-folders.json.
    Folders {
        plan: PathBuf,
        #[command(flatten)]
        roots: SecondLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// The plays of a saved `identify` report (`-f json`), each asked "is
    /// this detection right?"; written to
    /// <work>/audition/questions/03-detections-<report name>.json.
    Detections {
        report: PathBuf,
        #[command(flatten)]
        roots: SecondLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// The pairs of a `pair` report, each asked "do these different
    /// recordings share material where they line up, or is it chance?";
    /// written to <work>/audition/questions/04-shared-<report name>.json.
    Shared {
        report: PathBuf,
        #[command(flatten)]
        roots: SecondLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

/// Where the files named `second-library/<path>` are.
#[derive(clap::Args)]
struct SecondLibraryRoot {
    /// The root of the other library's files (the NAS).
    #[arg(long, default_value = "/Volumes/atlas/Music/dnb")]
    second_library_root: PathBuf,
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
        Command::Validate { .. }
            | Command::MapLibrary
            | Command::Clusters {
                from_peaks: true,
                ..
            }
            | Command::Sweep { .. }
            | Command::Scan { .. }
            | Command::Calibrate { .. }
            | Command::Robust { .. }
            | Command::Baseline { .. }
            | Command::Regress { .. }
            | Command::Memory { .. }
            | Command::Fullest { .. }
            | Command::Pair { .. }
            | Command::PairReview { .. }
            | Command::Recall { .. }
            | Command::Shared { .. }
            | Command::SharedMixes { .. }
            | Command::Holders { .. }
            | Command::Identifiability { .. }
            | Command::Versions { .. }
            | Command::Seconds { .. }
            | Command::Gaps { .. }
            | Command::SelfMatch { .. }
    );
    let takes_a_sample = matches!(
        command,
        Command::Sweep { .. }
            | Command::Scan { .. }
            | Command::Calibrate { .. }
            | Command::Baseline { .. }
            | Command::Regress { .. }
            | Command::Memory { .. }
    );
    if paths.other_sample.is_some() && !takes_a_sample {
        return Err(String::from(
            "this command does not take --other-sample; sweep, scan, calibrate, baseline, regress and memory do",
        ));
    }
    if paths.other_peaks_dir.is_some() && !takes_other_library {
        return Err(String::from(
            "this command does not take --other-peaks-dir; validate, map-library, clusters --from-peaks, sweep, scan, calibrate, robust, baseline, regress, memory, fullest, pair, pair-review, recall, shared, shared-mixes, holders, identifiability, versions, seconds, gaps and self-match do",
        ));
    }
    match command {
        Command::Validate { set } => manifest::validate(
            &paths.sets(),
            set.as_deref(),
            &paths.referable(&paths.library()?)?,
        ),
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
                .current_sources(&Profile::CURRENT, jobs)
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
        Command::Clusters {
            from_peaks,
            sample,
            manifests,
            recall_panel,
            cut_sparsest,
            reuse_pairs,
        } => {
            let extra = ExtraQueries {
                manifests,
                recall_panel,
            };
            let pairs = match (sample, reuse_pairs) {
                (Some(count), _) => PairSource::Sample(count),
                (None, Some(report)) => PairSource::LastRun(report),
                (None, None) => PairSource::Search,
            };
            let contradictions = if cut_sparsest {
                clusters::Contradictions::CutSparsest
            } else {
                clusters::Contradictions::Stop
            };
            find_clusters(paths, from_peaks, &pairs, &extra, contradictions, jobs)
        }
        Command::Related => {
            let related =
                related::find(&paths.library()?, &paths.store()?, &paths.clusters()?, jobs)?;
            write_json(&paths.reports().join("related-recordings.json"), &related)?;
            related::print_summary(&related);
            Ok(())
        }
        Command::Shared { plan } => run_shared(paths, &plan, jobs),
        Command::SharedMixes { plan } => run_shared_mixes(paths, &plan, jobs),
        Command::Holders { lookups } => run_holders(paths, &lookups, jobs),
        Command::Identifiability { panel, half } => run_identifiability(paths, &panel, half),
        Command::Versions { plays, out } => run_versions(paths, &plays, &out, jobs),
        Command::PairReview { pack } => {
            let pack = pack.unwrap_or_else(|| paths.work.join("listening"));
            let items = pair_review::items(&pack)?;
            let outcome = pair_review::review(
                &items,
                &paths.verdicts,
                &mut std::io::stdin().lock(),
                &mut std::io::stdout(),
                pair_review::ffplay,
            )?;
            println!(
                "\n{} verdicts written, {} items skipped, {} judged before",
                outcome.written, outcome.skipped, outcome.judged_before
            );
            if outcome.cut_left > 0 {
                println!("{} cut joins still without a verdict", outcome.cut_left);
            }
            if outcome.written > 0 {
                println!(
                    "To apply them to the last clusters run:\n  {}",
                    pair_review::reuse_command(paths.other_peaks_dir.as_deref())
                );
            }
            Ok(())
        }
        Command::Audition {
            files,
            answers,
            no_open,
            port,
        } => audition::server::serve(&audition::server::Options {
            question_files: question_files(&paths.work, files)?,
            answers: answers.unwrap_or_else(|| audition::answers::default_log(&paths.work)),
            verdicts: paths.verdicts.clone(),
            work: paths.work.clone(),
            open_browser: !no_open,
            port,
        }),
        Command::Questions { source } => make_questions(paths, source),
        Command::Pair { pairs } => {
            let text = fs::read_to_string(&pairs)
                .map_err(|error| format!("cannot read {}: {error}", pairs.display()))?;
            let listed: Vec<(String, String)> = text
                .lines()
                .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
                .map(|line| {
                    line.split_once('\t')
                        .map(|(query, found)| (query.to_owned(), found.to_owned()))
                        .ok_or_else(|| format!("not two tab-separated paths: {line}"))
                })
                .collect::<Result<_, _>>()?;
            let other = paths.other_store()?;
            let reports = pair::run(
                &pair::Sources {
                    library: &paths.library()?,
                    store: &paths.store()?,
                    other: other.as_ref(),
                },
                &listed,
            )?;
            let stem = pairs.file_stem().map_or_else(
                || String::from("pairs"),
                |stem| stem.to_string_lossy().into_owned(),
            );
            write_json(
                &paths.base_reports().join(format!("pairs-{stem}.json")),
                &reports,
            )?;
            pair::print_summary(&reports);
            Ok(())
        }
        Command::Sweep { seed, other_rips } => run_sweep(paths, seed, other_rips, jobs),
        Command::Recall {
            seed,
            prepare,
            other_rips_only,
            queries,
        } => run_recall(paths, seed, prepare, other_rips_only, &queries, jobs),
        Command::Scan {
            set,
            leave_out,
            seed,
            leave_out_slots,
            variant,
        } => {
            let leave_out = match leave_out {
                Some(count) => Some(LeaveOut::Drawn { count, seed }),
                None if !leave_out_slots.is_empty() => Some(LeaveOut::Slots(leave_out_slots)),
                None => None,
            };
            run_scan(paths, &set, leave_out.as_ref(), &variant, jobs)
        }
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
        Command::Mixes { seed, count, kind } => {
            let report = mixes::run(
                &paths.library()?,
                &paths.store()?,
                &paths.clusters()?,
                &paths.work,
                &mixes::Options {
                    seed,
                    kind,
                    panels: &paths.panels,
                    count,
                    ladder_name: paths.ladder.name(),
                    ladder: &paths.rungs(),
                    matching: &paths.matching,
                    verify: paths.verify,
                    jobs,
                },
            )?;
            write_json(
                &paths
                    .reports()
                    .join(format!("mixes-{}.json", kind.name(seed))),
                &report,
            )?;
            mixes::print_summary(&report, kind);
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
                    verify: paths.verify,
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
        Command::Seconds {
            reports,
            margin,
            min_span,
            boundaries,
        } => score_seconds(paths, &reports, margin, &min_span, boundaries),
        Command::Gaps {
            reports,
            margin,
            min_length,
        } => list_gaps(paths, &reports, margin, &min_length),
        Command::SelfMatch {
            reports,
            margin,
            min_length,
        } => match_gaps_against_themselves(paths, &reports, margin, min_length, jobs),
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
                    // The measurement is the build's memory: never the saved index.
                    padding: &Padding {
                        saved: None,
                        ..paths.padding(&variant)?
                    },
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

/// Whether runs with --other-peaks-dir use the saved index, as
/// `identify --saved-index`.
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
enum SavedIndexUse {
    /// Load it when it was built from what the run would index; otherwise
    /// build the index and save it over the file.
    Use,
    /// Build the index and save it over the file.
    Rebuild,
    /// Build the index in memory only.
    Off,
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
            saved: None,
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
            verify: paths.verify,
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
    run_sweep(paths, seed, false, jobs)?;
    run_scan(paths, set, None, &UNCHANGED_INDEX, jobs)?;
    for count in regress::LEAVE_OUTS {
        run_scan(
            paths,
            set,
            Some(&LeaveOut::Drawn { count, seed }),
            &UNCHANGED_INDEX,
            jobs,
        )?;
    }
    Ok(())
}

/// The other library's files a clusters run searches beside the corpus
/// files: those the named sets' manifests reference, and the development
/// sources of a recall panel.
/// Where `clusters` with another library takes its pairs from.
#[derive(Clone, PartialEq, Eq)]
enum PairSource {
    /// A search of every query, round after round.
    Search,
    /// One round over this many corpus files, for timing.
    Sample(usize),
    /// A report of an earlier run, without searching: the clusters in use
    /// unless another is given.
    LastRun(Option<PathBuf>),
}

struct ExtraQueries {
    manifests: Vec<String>,
    recall_panel: Option<u64>,
}

impl ExtraQueries {
    /// Their names, `second-library/<path>`.
    fn files(
        &self,
        paths: &Paths,
        other: &PeakStore,
        referable: &Referable,
    ) -> Result<BTreeSet<String>, String> {
        let mut files = BTreeSet::new();
        for name in &self.manifests {
            let set = manifest::load_set(&paths.sets(), name, referable)
                .map_err(|problems| format!("{name}: {}", problems.join("; ")))?;
            files.extend(
                set.tracks
                    .into_iter()
                    .flat_map(|track| track.references)
                    .filter(|reference| reference.starts_with(SECOND_LIBRARY_PREFIX)),
            );
        }
        if let Some(seed) = self.recall_panel {
            let (panel, _) = recall_panel(paths, other, seed)?;
            files.extend(panel.searched_sources().map(|source| source.asset.clone()));
        }
        Ok(files)
    }
}

fn find_clusters(
    paths: &Paths,
    from_peaks: bool,
    pairs: &PairSource,
    extra: &ExtraQueries,
    contradictions: clusters::Contradictions,
    jobs: usize,
) -> Result<(), String> {
    if let (Some(other), Some(map_file)) = (paths.other_store()?, paths.map_file()) {
        return find_clusters_around(paths, &other, &map_file, pairs, extra, contradictions, jobs);
    }
    if *pairs != PairSource::Search {
        return Err(String::from(
            "--sample and --reuse-pairs need --other-peaks-dir",
        ));
    }
    if !extra.manifests.is_empty() || extra.recall_panel.is_some() {
        return Err(String::from(
            "--manifest and --recall-panel need --other-peaks-dir: they add the other library's files to the queries",
        ));
    }
    let source = if from_peaks {
        clusters::Source::Peaks
    } else {
        clusters::Source::Audio
    };
    let library = paths.library()?;
    let verdicts = verdicts_on(paths, &clusters::Searched::corpus(&library))?;
    let clusters = clusters::find(
        &library,
        &paths.store()?,
        source,
        &verdicts,
        contradictions,
        jobs,
    )?;
    clusters::print_summary(&clusters);
    clusters::print_cut_links(&clusters);
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

/// The verdicts by ear on files the run searches; each other verdict is
/// printed and adds nothing.
fn verdicts_on(paths: &Paths, searched: &clusters::Searched) -> Result<clusters::Verdicts, String> {
    let (verdicts, outside) = paths.verdicts()?.on(searched);
    if !outside.is_empty() {
        println!(
            "{} verdicts name a file this run does not search; they add nothing:",
            outside.len()
        );
        for message in outside {
            println!("  {message}");
        }
    }
    Ok(verdicts)
}

/// The ignore file of the library the other store names. A store-only run
/// may find that library unmounted; then only the records tell which files
/// the run searches, and a prune has deleted those of ignored files.
fn other_ignore_file(other: &PeakStore) -> Result<Option<clusters::IgnoreFile>, String> {
    let Some(root) = other.library().map_err(|error| error.to_string())? else {
        return Ok(None);
    };
    let root = PathBuf::from(root);
    if !root.is_dir() {
        println!(
            "cannot read the other library at {}, so its ignore file is not checked: verdicts link files with a current record",
            root.display()
        );
        return Ok(None);
    }
    let patterns = ignore::read(&root).map_err(|error| error.to_string())?;
    Ok(patterns.map(|patterns| clusters::IgnoreFile {
        path: root.join(IGNORE_FILE),
        patterns,
    }))
}

/// The corpus clusters with the other library's rips of the corpus
/// recordings, which the evaluations against both libraries read.
fn find_clusters_around(
    paths: &Paths,
    other: &PeakStore,
    map_file: &Path,
    source: &PairSource,
    extra: &ExtraQueries,
    contradictions: clusters::Contradictions,
    jobs: usize,
) -> Result<(), String> {
    let map = LibraryMap::load(map_file)?;
    let corpus = paths.corpus_clusters()?;
    let profile = Profile::CURRENT;
    let (other_assets, problems) = other
        .current_sources(&profile, jobs)
        .map_err(|error| error.to_string())?;
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let library = paths.library()?;
    let searched =
        clusters::Searched::corpus(&library).with_other(&other_assets, other_ignore_file(other)?);
    let verdicts = verdicts_on(paths, &searched)?;
    let name = map
        .other_library
        .clone()
        .unwrap_or_else(|| other.dir().display().to_string());
    let copies = map.copied();
    if let PairSource::LastRun(report) = source {
        let report = report.clone().unwrap_or_else(|| paths.clusters_file());
        let last = Clusters::load(&report)?;
        let (pairs, dropped) = clusters::as_measured(last.pairs, &searched, &verdicts);
        println!(
            "{} pairs of an earlier run ({}), clustered again without searching; {dropped} dropped: a file pruned or ignored since",
            pairs.len(),
            report.display()
        );
        return write_merged(
            paths,
            &corpus,
            pairs,
            &copies,
            &verdicts,
            &name,
            contradictions,
        );
    }
    if *source == PairSource::Search {
        check_verdicts_against_last_run(
            paths,
            &corpus,
            &searched,
            &verdicts,
            &name,
            contradictions,
        )?;
    }
    let (mut queries, _) = load_records(&library, &paths.store()?, &profile, &BTreeSet::new());
    let referable = Referable::corpus(&library).with_other(&other_assets, other.dir());
    let extra_files = extra.files(paths, other, &referable)?;
    let extra_queries = clusters::other_queries(other, &other_assets, &extra_files, &copies)?;
    println!(
        "{} queries: {} corpus files and {} of the other library's ({} named, the others copies of corpus files)",
        queries.len() + extra_queries.len(),
        queries.len(),
        extra_queries.len(),
        extra_files.len()
    );
    queries.extend(extra_queries);
    if let PairSource::Sample(count) = *source {
        let every = (queries.len() / count.max(1)).max(1);
        let sampled: Vec<_> = queries.into_iter().step_by(every).take(count).collect();
        let pairs = clusters::find_around(
            sampled,
            other,
            &other_assets,
            &copies,
            &verdicts,
            clusters::Rounds::First,
            jobs,
        )?;
        clusters::print_around(&corpus, &pairs, &copies);
        return write_json(
            &paths.base_reports().join("duplicate-clusters-sample.json"),
            &pairs,
        );
    }
    let pairs = clusters::find_around(
        queries,
        other,
        &other_assets,
        &copies,
        &verdicts,
        clusters::Rounds::UntilNoneIsNew,
        jobs,
    )?;
    let kept = pairs.clone();
    write_merged(
        paths,
        &corpus,
        pairs,
        &copies,
        &verdicts,
        &name,
        contradictions,
    )
    .or_else(|error| {
        let stopped = paths
            .library_reports()
            .join("duplicate-clusters-stopped.json");
        let unclustered = Clusters {
            criterion: format!("{}; stopped before clustering: {error}", corpus.criterion),
            duplicates: Vec::new(),
            pairs: kept,
            cut_links: Vec::new(),
        };
        write_json(&stopped, &unclustered)?;
        Err(format!(
            "{error}\nThe search's pairs are kept in {0}; after the verdict, `clusters --from-peaks --reuse-pairs {0}` clusters them without searching.",
            stopped.display()
        ))
    })
}

/// Prints what the pairs found, joins them to the corpus clusters with the
/// verdicts applied, and writes the clusters in use.
fn write_merged(
    paths: &Paths,
    corpus: &Clusters,
    pairs: Vec<clusters::Pair>,
    copies: &BTreeSet<String>,
    verdicts: &clusters::Verdicts,
    name: &str,
    contradictions: clusters::Contradictions,
) -> Result<(), String> {
    clusters::print_around(corpus, &pairs, copies);
    let merged = clusters::merged(corpus, pairs, verdicts, name, contradictions)?;
    clusters::print_cut_links(&merged);
    write_json(&paths.clusters_file(), &merged)?;
    println!(
        "{} clusters with duplicates, written to {}",
        merged.duplicates.len(),
        paths.clusters_file().display()
    );
    Ok(())
}

/// Fails before searching the other library again (85 minutes for the
/// NAS) when the verdicts contradict the joins of the last run, which the
/// new search would most likely find again.
fn check_verdicts_against_last_run(
    paths: &Paths,
    corpus: &Clusters,
    searched: &clusters::Searched,
    verdicts: &clusters::Verdicts,
    name: &str,
    contradictions: clusters::Contradictions,
) -> Result<(), String> {
    let Ok(last) = paths.clusters() else {
        return Ok(());
    };
    let (pairs, _) = clusters::as_measured(last.pairs, searched, verdicts);
    clusters::merged(corpus, pairs, verdicts, name, contradictions)
        .map(|_| ())
        .map_err(|error| {
            format!(
                "{error}\n(Checked against the pairs of the last run, {}, before searching again.)",
                paths.clusters_file().display()
            )
        })
}

fn run_sweep(paths: &Paths, seed: u64, other_rips: bool, jobs: usize) -> Result<(), String> {
    paths.check_panel(seed)?;
    let map = match (other_rips, paths.map_file()) {
        (false, _) => None,
        (true, Some(map_file)) => Some(LibraryMap::load(&map_file)?),
        (true, None) => return Err(String::from("--other-rips needs --other-peaks-dir")),
    };
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
            verify: paths.verify,
            other_rips: map.as_ref(),
            jobs,
        },
    )?;
    let name = if other_rips {
        format!("sweep-seed-{seed}-other-rips.json")
    } else {
        format!("sweep-seed-{seed}.json")
    };
    write_json(&paths.reports().join(name), &report)?;
    sweep::print_summary(&report);
    Ok(())
}

/// The plan's scenarios, rendered into `<work>/shared/<plan name>-<render
/// key>/` (`shared::render_key`); the report is `<plan name>.json`, with
/// `shared-` put first when the name does not start with it.
fn run_shared(paths: &Paths, plan_file: &Path, jobs: usize) -> Result<(), String> {
    let text = fs::read_to_string(plan_file)
        .map_err(|error| format!("cannot read {}: {error}", plan_file.display()))?;
    let plan: shared::Plan =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", plan_file.display()))?;
    let name = plan_file.file_stem().map_or_else(
        || String::from("shared-material"),
        |stem| stem.to_string_lossy().into_owned(),
    );
    let second_root = second_library_root(paths.other_store()?.as_ref())?;
    let library = paths.library()?;
    let renders = paths
        .work
        .join("shared")
        .join(format!("{name}-{}", shared::render_key(&plan)));
    eprintln!("queries rendered in {}", renders.display());
    let report = shared::run(
        &plan,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        &renders,
        &shared::Options {
            ladder: &paths.rungs(),
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            jobs,
        },
    )?;
    let report_name = if name.starts_with("shared") {
        name
    } else {
        format!("shared-{name}")
    };
    write_json(
        &paths.reports().join(format!("{report_name}.json")),
        &report,
    )?;
    shared::print_summary(&report);
    Ok(())
}

/// The plan's mixes, rendered into `<work>/shared-mixes/<plan name>/`
/// (each kept while its truth file matches); the report is `<plan
/// name>.json`.
fn run_shared_mixes(paths: &Paths, plan_file: &Path, jobs: usize) -> Result<(), String> {
    let text = fs::read_to_string(plan_file)
        .map_err(|error| format!("cannot read {}: {error}", plan_file.display()))?;
    let plan: shared_mixes::Plan =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", plan_file.display()))?;
    let name = plan_file.file_stem().map_or_else(
        || String::from("shared-mixes"),
        |stem| stem.to_string_lossy().into_owned(),
    );
    let second_root = second_library_root(paths.other_store()?.as_ref())?;
    let library = paths.library()?;
    let report = shared_mixes::run(
        &plan,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        &paths.work.join("shared-mixes").join(&name),
        &shared_mixes::Options {
            ladder: &paths.rungs(),
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            sets: &paths.sets(),
            jobs,
        },
    )?;
    write_json(&paths.reports().join(format!("{name}.json")), &report)?;
    shared_mixes::print_summary(&report);
    Ok(())
}

/// The holders of each lookup's span; the report is `holders-<file
/// stem>.json`.
fn run_holders(paths: &Paths, lookups_file: &Path, jobs: usize) -> Result<(), String> {
    let text = fs::read_to_string(lookups_file)
        .map_err(|error| format!("cannot read {}: {error}", lookups_file.display()))?;
    let lookups: Vec<holders::Lookup> = serde_json::from_str(&text)
        .map_err(|error| format!("{}: {error}", lookups_file.display()))?;
    let name = lookups_file.file_stem().map_or_else(
        || String::from("lookups"),
        |stem| stem.to_string_lossy().into_owned(),
    );
    let stood_for = match paths.map_file() {
        Some(map_file) => LibraryMap::load(&map_file)?.standing_corpus_files(SECOND_LIBRARY_PREFIX),
        None => BTreeMap::new(),
    };
    let looked = holders::run(
        &lookups,
        &paths.library()?,
        &paths.store()?,
        &holders::Options {
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            stood_for: &stood_for,
            jobs,
        },
    )?;
    write_json(
        &paths.reports().join(format!("holders-{name}.json")),
        &looked,
    )?;
    eprintln!(
        "{} lookups, {} without peaks at hand",
        looked.len(),
        looked
            .iter()
            .filter(|looked| looked.error.is_some())
            .count()
    );
    Ok(())
}

fn run_versions(paths: &Paths, plays_file: &Path, out: &Path, jobs: usize) -> Result<(), String> {
    let text = fs::read_to_string(plays_file)
        .map_err(|error| format!("cannot read {}: {error}", plays_file.display()))?;
    let list: versions::PlayList = serde_json::from_str(&text)
        .map_err(|error| format!("{}: {error}", plays_file.display()))?;
    let other = paths.other_store()?;
    let second_root = second_library_root(other.as_ref())?;
    let library = paths.library()?;
    let verified = versions::run(
        &list.plays,
        &pair::Sources {
            library: &library,
            store: &paths.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        jobs,
    )?;
    write_json(out, &verified)?;
    versions::print_summary(&verified);
    Ok(())
}

/// A half of a shared-material panel.
#[derive(Clone, Copy, clap::ValueEnum)]
enum Half {
    Development,
    Validation,
}

impl Half {
    fn name(self) -> &'static str {
        match self {
            Half::Development => "development",
            Half::Validation => "validation",
        }
    }
}

fn run_identifiability(paths: &Paths, panel_file: &Path, half: Half) -> Result<(), String> {
    let text = fs::read_to_string(panel_file)
        .map_err(|error| format!("cannot read {}: {error}", panel_file.display()))?;
    let panel: identifiability::Panel = serde_json::from_str(&text)
        .map_err(|error| format!("{}: {error}", panel_file.display()))?;
    let other = paths.other_store()?;
    let second_root = second_library_root(other.as_ref())?;
    let library = paths.library()?;
    let report = identifiability::run(
        &panel,
        half.name(),
        &pair::Sources {
            library: &library,
            store: &paths.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
    )?;
    write_json(
        &paths
            .base_reports()
            .join(format!("identifiability-{}.json", half.name())),
        &report,
    )?;
    identifiability::print_summary(&report);
    Ok(())
}

/// The root of the other library's files, as its store names it.
fn second_library_root(other: Option<&PeakStore>) -> Result<Option<PathBuf>, String> {
    let Some(other) = other else {
        return Ok(None);
    };
    let root = other
        .library()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| {
            format!(
                "{} names no library, so the files there cannot be found",
                other.dir().display()
            )
        })?;
    Ok(Some(PathBuf::from(root)))
}

fn run_recall(
    paths: &Paths,
    seed: u64,
    prepare: bool,
    other_rips_only: bool,
    sets: &[recall::QuerySet],
    jobs: usize,
) -> Result<(), String> {
    let Some(other) = paths.other_store()? else {
        return Err(String::from("recall needs --other-peaks-dir"));
    };
    let (panel, assets) = recall_panel(paths, &other, seed)?;
    let root = other
        .library()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| {
            format!(
                "{} names no library, so the panel's audio cannot be found",
                other.dir().display()
            )
        })?;
    let root = Path::new(&root);
    let dir = paths.work.join("recall");
    let clean = recall::render(&panel, root, &dir.join(format!("seed-{seed}")), jobs)?;
    let damaged = if sets.iter().any(|set| set.damaged()) {
        recall::render_damaged(
            &panel,
            root,
            &dir.join(format!("seed-{seed}-damaged")),
            jobs,
        )?
    } else {
        Vec::new()
    };
    println!(
        "recall panel seed {seed}: {} development sources, {} excerpts, {} damaged, in {}",
        panel.development.sources.len(),
        clean.len(),
        damaged.len(),
        dir.display()
    );
    if prepare {
        return Ok(());
    }
    let clusters = paths.clusters()?;
    let left_out = if other_rips_only {
        recall::own_records(&panel, &clusters, &other, &assets)
    } else {
        BTreeSet::new()
    };
    let sets: Vec<(recall::QuerySet, &[recall::Rendered])> = sets
        .iter()
        .map(|&set| {
            (
                set,
                if set.damaged() {
                    &damaged[..]
                } else {
                    &clean[..]
                },
            )
        })
        .collect();
    recall::run(
        &paths.library()?,
        &paths.store()?,
        &clusters,
        &sets,
        &recall::Options {
            seed,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            ladder: &paths.rungs(),
            matching: &paths.matching,
            left_out: &left_out,
            verify: paths.verify,
            jobs,
        },
        |report| {
            let name = report.query_set.report_name(seed, other_rips_only);
            write_json(&paths.reports().join(name), &report)?;
            recall::print_summary(&report);
            Ok(())
        },
    )
}

/// The recall panel of `seed`, drawn the first time from the other
/// library's records outside the clusters in use and the copies of corpus
/// files.
fn recall_panel(
    paths: &Paths,
    other: &PeakStore,
    seed: u64,
) -> Result<(recall::Panel, Vec<Asset>), String> {
    let Some(map_file) = paths.map_file() else {
        return Err(String::from("the recall panel needs --other-peaks-dir"));
    };
    let mut corpus_recordings: BTreeSet<String> =
        paths.clusters()?.duplicates.into_iter().flatten().collect();
    corpus_recordings.extend(
        LibraryMap::load(&map_file)?
            .copied()
            .into_iter()
            .map(|path| format!("{SECOND_LIBRARY_PREFIX}{path}")),
    );
    let (assets, problems) = other
        .current_sources(&Profile::CURRENT, available_parallelism())
        .map_err(|error| error.to_string())?;
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let panel = recall::Panel::for_seed(other, &assets, &corpus_recordings, seed, &paths.panels)?;
    Ok((panel, assets))
}

fn run_scan(
    paths: &Paths,
    set: &str,
    leave_out: Option<&LeaveOut>,
    variant: &IndexVariant,
    jobs: usize,
) -> Result<(), String> {
    let padding = paths.padding(variant)?;
    let library = paths.library()?;
    let report = scan::run(
        &paths.sets(),
        set,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &scan::Options {
            referable: &paths.referable(&library)?,
            leave_out,
            padding: &padding,
            matching: &paths.matching,
            ladder: &paths.rungs(),
            verify: paths.verify,
            jobs,
        },
    )?;
    let suffix = variant.suffix();
    let name = match leave_out {
        _ if !suffix.is_empty() => format!("scale-scan-{set}{suffix}.json"),
        Some(LeaveOut::Drawn { count, seed }) => {
            format!("scan-{set}-leave-out-{count}-seed-{seed}.json")
        }
        Some(LeaveOut::Slots(runs)) => format!(
            "scan-{set}-leave-out-slots-{}.json",
            runs.iter()
                .map(|run| run.name())
                .collect::<Vec<_>>()
                .join("_")
        ),
        None => format!("scan-{set}.json"),
    };
    write_json(&paths.reports().join(name), &report)?;
    scan::print_summary(&report);
    Ok(())
}

/// The library, if nothing vanished while it was listed (ADR 0015): an
/// evaluation on a library missing files that are still there would not be
/// repeatable.
fn nothing_vanished(library: Library) -> Result<Library, String> {
    match library.vanished.first() {
        None => Ok(library),
        Some(first) => Err(format!(
            "files or folders vanished while it was listed (`{first}`, {} in all); run again when nothing moves files in it",
            library.vanished.len()
        )),
    }
}

impl Paths {
    fn library(&self) -> Result<Library, String> {
        let root = self.corpus.join("library");
        Library::scan(&root)
            .map_err(|error| error.to_string())
            .and_then(nothing_vanished)
            .map_err(|error| format!("cannot read the library at {}: {error}", root.display()))
    }

    fn store(&self) -> Result<PeakStore, String> {
        PeakStore::open(&self.peaks_dir).map_err(|error| error.to_string())
    }

    /// The files a manifest's references may name: the corpus library's
    /// and, with --other-peaks-dir, those the other store holds a current
    /// record of.
    fn referable(&self, library: &Library) -> Result<Referable, String> {
        let referable = Referable::corpus(library);
        let Some(store) = self.other_store()? else {
            return Ok(referable);
        };
        let (assets, problems) = store
            .current_sources(&Profile::CURRENT, available_parallelism())
            .map_err(|error| error.to_string())?;
        for problem in &problems {
            eprintln!("left out: {problem}");
        }
        Ok(referable.with_other(&assets, store.dir()))
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
        if self.verify {
            reports.push("verified");
        }
        reports
    }

    fn baseline(&self, name: &str) -> PathBuf {
        self.work.join("baselines").join(name)
    }

    /// `reports/`, or with another library its own directory there, and a
    /// sample of it a directory below that.
    fn base_reports(&self) -> PathBuf {
        let library = self.library_reports();
        match self.other_sample {
            Some(count) => library.join(format!("sample-{count}")),
            None => library,
        }
    }

    /// `reports/`, or with another library its own directory there: where
    /// its map and clusters are, whether or not a sample of it is searched.
    fn library_reports(&self) -> PathBuf {
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
        self.library_reports().join("duplicate-clusters.json")
    }

    /// Where `seconds` writes: `seconds/`, or with another library its own
    /// directory there, as for reports.
    fn seconds_dir(&self) -> PathBuf {
        let dir = self.work.join("seconds");
        match self.other_name() {
            Some(name) => dir.join(format!("library-{name}")),
            None => dir,
        }
    }

    /// Where `gaps` writes, beside `seconds_dir`.
    fn gaps_dir(&self) -> PathBuf {
        let dir = self.work.join("gaps");
        match self.other_name() {
            Some(name) => dir.join(format!("library-{name}")),
            None => dir,
        }
    }

    fn clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.clusters_file())
    }

    fn verdicts(&self) -> Result<clusters::Verdicts, String> {
        clusters::Verdicts::load(&self.verdicts)
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
            .map(|_| self.library_reports().join("library-map.json"))
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
    /// copies corpus files stand for, and their saved index.
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
        let second = SecondLibrary::from_store(store, &map.stood_for())?;
        padding.second = Some(match self.other_sample {
            Some(count) => second.sampled(count, OTHER_SAMPLE_SEED),
            None => second,
        });
        padding.saved = self.saved_index();
        Ok(padding)
    }

    /// The saved index of the corpus and the whole other library, unless
    /// the run searches a sample of it or uses none.
    fn saved_index(&self) -> Option<SavedIndex> {
        if self.other_sample.is_some() {
            return None;
        }
        let file = self
            .work
            .join("indexes")
            .join(format!("library-{}.index", self.other_name()?));
        match self.saved_index {
            SavedIndexUse::Use => Some(SavedIndex::Use(file)),
            SavedIndexUse::Rebuild => Some(SavedIndex::Rebuild(file)),
            SavedIndexUse::Off => None,
        }
    }
}

/// The draw of `--other-sample`: one subset per size, the same in every run.
const OTHER_SAMPLE_SEED: u64 = 2026;

/// One worker thread per core.
fn available_parallelism() -> usize {
    std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

/// The question files named, or every `*.json` of the default folder in
/// name order.
fn question_files(work: &Path, named: Vec<PathBuf>) -> Result<Vec<PathBuf>, String> {
    if !named.is_empty() {
        return Ok(named);
    }
    let folder = work.join("audition").join("questions");
    let mut files: Vec<PathBuf> = fs::read_dir(&folder)
        .map_err(|error| {
            format!(
                "cannot read {} ({error}); make questions first with `gunfinger-eval questions pack` or name question files",
                folder.display()
            )
        })?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "json"))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("{} holds no question files", folder.display()));
    }
    Ok(files)
}

fn make_questions(paths: &Paths, source: QuestionSource) -> Result<(), String> {
    let questions_dir = paths.work.join("audition").join("questions");
    let libraries = |roots: &SecondLibraryRoot| -> Result<audition::names::Libraries, String> {
        let absolute = |path: &Path| {
            std::path::absolute(path)
                .map_err(|error| format!("cannot resolve {}: {error}", path.display()))
        };
        Ok(audition::names::Libraries {
            corpus: absolute(&paths.corpus.join("library"))?,
            second: absolute(&roots.second_library_root)?,
        })
    };
    let (mut file, out) = match source {
        QuestionSource::Pack {
            pack,
            plan,
            pair_reports,
            roots,
            out,
        } => {
            let plan = plan
                .as_deref()
                .map(audition::pack::Plan::load)
                .transpose()?
                .unwrap_or_default();
            let file = audition::pack::questions(
                &pack.unwrap_or_else(|| paths.work.join("listening")),
                &plan,
                &audition::pack::Reports::load(&pair_reports)?,
                &libraries(&roots)?,
            )?;
            (
                file,
                out.unwrap_or_else(|| questions_dir.join("01-pack.json")),
            )
        }
        QuestionSource::Folders { plan, roots, out } => {
            let envelopes = audition::audio::Envelopes::below(&paths.work);
            let file = audition::folders::questions(
                &audition::folders::Plan::load(&plan)?,
                &libraries(&roots)?,
                |path| {
                    envelopes
                        .get(path)
                        .map(|envelope| envelope.duration_seconds)
                },
            )?;
            (
                file,
                out.unwrap_or_else(|| questions_dir.join("02-folders.json")),
            )
        }
        QuestionSource::Detections { report, roots, out } => {
            let envelopes = audition::audio::Envelopes::below(&paths.work);
            let file = audition::detections::questions(
                &audition::detections::Report::load(&report)?,
                &libraries(&roots)?,
                |path| {
                    envelopes
                        .get(path)
                        .map(|envelope| envelope.duration_seconds)
                },
            )?;
            let name = report.file_stem().map_or_else(
                || String::from("report"),
                |stem| stem.to_string_lossy().into_owned(),
            );
            (
                file,
                out.unwrap_or_else(|| questions_dir.join(format!("03-detections-{name}.json"))),
            )
        }
        QuestionSource::Shared { report, roots, out } => {
            let reports = audition::pack::Reports::load(std::slice::from_ref(&report))?;
            let reports: Vec<_> = reports
                .reports
                .into_iter()
                .map(|(_, report)| report)
                .collect();
            let file = audition::shared::questions(&reports, &libraries(&roots)?)?;
            let name = report.file_stem().map_or_else(
                || String::from("report"),
                |stem| stem.to_string_lossy().into_owned(),
            );
            (
                file,
                out.unwrap_or_else(|| questions_dir.join(format!("04-shared-{name}.json"))),
            )
        }
    };
    // The whole command, so the next agent can make the file again.
    file.made_by = std::iter::once(String::from("gunfinger-eval"))
        .chain(std::env::args().skip(1))
        .collect::<Vec<_>>()
        .join(" ");
    audition::schema::save(&out, &file)?;
    println!(
        "wrote {} questions to {}",
        file.questions.len(),
        out.display()
    );
    Ok(())
}

/// The kinds of saved report `seconds` and `gaps` read.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportKind {
    Mixes,
    Scan,
    /// `shared-mixes`: rendered mixes, each under several indexes.
    Rendered,
}

/// A mix of a saved report with its truth.
struct Judged {
    /// The mix's or the set's name; for a rendered mix, `<index>: <mix>`.
    name: String,
    /// The index a rendered mix was searched with.
    index: Option<String>,
    truth: Vec<seconds::Truth>,
    segments: Vec<seconds::Segment>,
    duration_seconds: f64,
    /// `margin` for a scan, whose borders are approximate; 0 for exact truth.
    margin: f64,
    /// The mix's audio, where the report names it: a set's, or a
    /// generated mix's render.
    audio: Option<PathBuf>,
}

/// Reads `reports`, each with the mixes it holds and their truth.
fn judged_reports(
    paths: &Paths,
    reports: &[PathBuf],
    clusters: &Clusters,
    margin: f64,
) -> Result<Vec<(PathBuf, ReportKind, Vec<Judged>)>, String> {
    let mut loaded = Vec::new();
    for path in reports {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let report: serde_json::Value =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
        loaded.push((path, report));
    }
    let referable = if loaded.iter().any(|(_, report)| report.get("set").is_some()) {
        Some(paths.referable(&paths.library()?)?)
    } else {
        None
    };
    loaded
        .into_iter()
        .map(|(path, report)| {
            let (kind, judged) = judged(paths, path, report, clusters, referable.as_ref(), margin)?;
            Ok((path.clone(), kind, judged))
        })
        .collect()
}

fn judged(
    paths: &Paths,
    path: &Path,
    report: serde_json::Value,
    clusters: &Clusters,
    referable: Option<&Referable>,
    margin: f64,
) -> Result<(ReportKind, Vec<Judged>), String> {
    let parse = |error: serde_json::Error| format!("{}: {error}", path.display());
    if report.get("runs").is_some() {
        let report: shared_mixes::Report = serde_json::from_value(report).map_err(parse)?;
        let judged = report
            .runs
            .iter()
            .filter_map(|run| {
                // Runs on the development mixes have no planned truth.
                let rendered = report
                    .mixes
                    .iter()
                    .find(|rendered| rendered.mix.name == run.name)?;
                let left_out: BTreeSet<String> = run.left_out.iter().cloned().collect();
                Some(Judged {
                    name: format!("{}: {}", run.index, run.name),
                    index: Some(run.index.clone()),
                    truth: seconds::rendered_truth(&rendered.mix, clusters, &left_out),
                    segments: run
                        .detections
                        .iter()
                        .map(|measured| seconds::Segment::from(&measured.found))
                        .collect(),
                    duration_seconds: rendered.mix.seconds,
                    margin: 0.0,
                    audio: None,
                })
            })
            .collect();
        return Ok((ReportKind::Rendered, judged));
    }
    match (report.get("mixes").is_some(), referable) {
        (true, _) => {
            let report: mixes::MixesReport = serde_json::from_value(report).map_err(parse)?;
            let judged = report
                .mixes
                .iter()
                .map(|result| Judged {
                    name: result.mix.name.clone(),
                    index: None,
                    truth: seconds::mix_truth(&result.mix, clusters),
                    segments: result
                        .detections
                        .iter()
                        .map(seconds::Segment::from)
                        .collect(),
                    duration_seconds: result.mix.seconds,
                    margin: 0.0,
                    audio: Some(
                        report
                            .kind
                            .renders(&paths.work, report.seed)
                            .join(format!("{}.mp3", result.mix.name)),
                    ),
                })
                .collect();
            Ok((ReportKind::Mixes, judged))
        }
        (false, Some(referable)) => {
            let report: scan::ScanReport = serde_json::from_value(report).map_err(parse)?;
            let set = manifest::load_set(&paths.sets(), &report.set, referable)
                .map_err(|problems| format!("{}: {}", report.set, problems.join("; ")))?;
            let left_out: BTreeSet<String> = report.left_out_assets.iter().cloned().collect();
            Ok((
                ReportKind::Scan,
                vec![Judged {
                    name: report.set.clone(),
                    index: None,
                    truth: seconds::set_truth(&set, report.duration_seconds, clusters, &left_out),
                    segments: report
                        .detections
                        .iter()
                        .map(seconds::Segment::from)
                        .collect(),
                    duration_seconds: report.duration_seconds,
                    margin,
                    audio: Some(set.audio.clone()),
                }],
            ))
        }
        (false, None) => Err(format!(
            "{} is neither a mixes nor a scan report",
            path.display()
        )),
    }
}

/// The name a report's scores or gaps are written under: its stem, and
/// for a scan the margin.
fn judged_name(path: &Path, kind: ReportKind, margin: f64) -> String {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    match kind {
        ReportKind::Scan => format!("{stem}-margin-{margin:.0}.json"),
        ReportKind::Mixes | ReportKind::Rendered => format!("{stem}.json"),
    }
}

/// `seconds`: each report scored under today's rule and each minimum span.
fn score_seconds(
    paths: &Paths,
    reports: &[PathBuf],
    margin: f64,
    min_spans: &[f64],
    boundaries: bool,
) -> Result<(), String> {
    let rules: Vec<seconds::Rule> = std::iter::once(seconds::Rule::Windows)
        .chain(min_spans.iter().map(|&span| seconds::Rule::MinSpan(span)))
        .collect();
    let clusters = paths.clusters()?;
    for (path, kind, judged) in judged_reports(paths, reports, &clusters, margin)? {
        let scored: Vec<seconds::Scored> = judged
            .iter()
            .flat_map(|mix| {
                rules.iter().map(move |&rule| {
                    let mut score = seconds::score(
                        &mix.truth,
                        &mix.segments,
                        mix.duration_seconds,
                        mix.margin,
                        rule,
                    );
                    if kind == ReportKind::Scan && !boundaries {
                        score.boundaries.clear();
                    }
                    seconds::Scored {
                        name: mix.name.clone(),
                        rule: rule.name(),
                        score,
                    }
                })
            })
            .collect();
        write_json(
            &paths.seconds_dir().join(judged_name(&path, kind, margin)),
            &scored,
        )?;
        if kind == ReportKind::Rendered {
            let mut indexes: Vec<&str> = Vec::new();
            for index in judged.iter().filter_map(|mix| mix.index.as_deref()) {
                if !indexes.contains(&index) {
                    indexes.push(index);
                }
            }
            for index in indexes {
                let prefix = format!("{index}: ");
                let here: Vec<&seconds::Scored> = scored
                    .iter()
                    .filter(|scored| scored.name.starts_with(&prefix))
                    .collect();
                seconds::print_summary(&format!("{}, {index}", path.display()), &here);
            }
        } else {
            seconds::print_summary(
                &path.display().to_string(),
                &scored.iter().collect::<Vec<_>>(),
            );
        }
    }
    Ok(())
}

/// `gaps`: each report's gaps at each minimum length, and a summary of
/// all of them.
fn list_gaps(
    paths: &Paths,
    reports: &[PathBuf],
    margin: f64,
    min_lengths: &[f64],
) -> Result<(), String> {
    let clusters = paths.clusters()?;
    let mut every: Vec<gaps::Listing> = Vec::new();
    for (path, kind, judged) in judged_reports(paths, reports, &clusters, margin)? {
        let listings: Vec<gaps::Listing> = judged
            .iter()
            .map(|mix| {
                gaps::list(
                    &mix.name,
                    &mix.truth,
                    &mix.segments,
                    mix.duration_seconds,
                    mix.margin,
                    min_lengths,
                )
            })
            .collect();
        write_json(
            &paths.gaps_dir().join(judged_name(&path, kind, margin)),
            &listings,
        )?;
        gaps::print_summary(
            &path.display().to_string(),
            &listings.iter().collect::<Vec<_>>(),
            min_lengths,
        );
        every.extend(listings);
    }
    if reports.len() > 1 {
        gaps::print_summary(
            &format!("all {} reports", reports.len()),
            &every.iter().collect::<Vec<_>>(),
            min_lengths,
        );
    }
    Ok(())
}

/// One gap matched against itself, beside its truth.
#[derive(Serialize)]
struct SelfMatched {
    mix: String,
    start_seconds: f64,
    end_seconds: f64,
    missed_tracks: usize,
    indexed_tracks: usize,
    estimate: Option<u32>,
    blocks: Vec<self_match::Blocks>,
}

/// `self-match`: every listed gap of each report, matched against itself,
/// one gap per worker.
fn match_gaps_against_themselves(
    paths: &Paths,
    reports: &[PathBuf],
    margin: f64,
    min_length: f64,
    jobs: usize,
) -> Result<(), String> {
    let clusters = paths.clusters()?;
    let profile = Profile::CURRENT;
    for (path, kind, judged) in judged_reports(paths, reports, &clusters, margin)? {
        let mut gaps: Vec<(&Path, String, gaps::Gap)> = Vec::new();
        for mix in &judged {
            let Some(audio) = &mix.audio else { continue };
            let listing = gaps::list(
                &mix.name,
                &mix.truth,
                &mix.segments,
                mix.duration_seconds,
                mix.margin,
                &[min_length],
            );
            for gap in listing.at.into_iter().flat_map(|at| at.gaps) {
                gaps.push((audio, mix.name.clone(), gap));
            }
        }
        let matched: Vec<SelfMatched> = map_in_order(&gaps, jobs, |(audio, mix, gap)| {
            let excerpt = Excerpt {
                start: Some(Duration::from_secs_f64(gap.start_seconds)),
                duration: Some(Duration::from_secs_f64(gap.length_seconds())),
            };
            let samples = decode(audio, profile.sample_rate, excerpt)
                .map_err(|error| format!("{}: {error}", audio.display()))?
                .samples;
            Ok(SelfMatched {
                mix: mix.clone(),
                start_seconds: gap.start_seconds,
                end_seconds: gap.end_seconds,
                missed_tracks: gap.missed_tracks.len(),
                indexed_tracks: gap.indexed_tracks.len(),
                estimate: gap.estimate,
                blocks: self_match::blocks(&samples, &profile),
            })
        })
        .into_iter()
        .collect::<Result<_, String>>()?;
        let name = format!("self-match-{}", judged_name(&path, kind, margin));
        write_json(&paths.gaps_dir().join(name), &matched)?;
        println!(
            "{}: {} gaps of at least {min_length:.0} s",
            path.display(),
            matched.len()
        );
        for gap in &matched {
            let counts: Vec<String> = gap
                .blocks
                .iter()
                .map(|blocks| format!("{}@{}", blocks.spans.len(), blocks.min_hits))
                .collect();
            println!(
                "  {} {:.0}-{:.0} s: {} missed, estimate {}, blocks {}",
                gap.mix,
                gap.start_seconds,
                gap.end_seconds,
                gap.missed_tracks,
                gap.estimate
                    .map_or_else(|| String::from("-"), |n| n.to_string()),
                counts.join(" ")
            );
        }
    }
    Ok(())
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    }
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    fs::write(path, text).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

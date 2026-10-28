//! The harness's global options and what they locate: the library, the
//! peak stores, the clusters and verdicts, the report directories, and
//! the additions to the index of scale experiments.

use std::path::PathBuf;

use clap::ValueEnum;
use gunfinger_core::library::Library;
use gunfinger_core::speed::{Rung, key_lock_ladder_with_extra_rungs, ladder_with_extra_rungs};
use gunfinger_core::store::PeakStore;

use crate::clusters::{self, Clusters};
use crate::library_map::LibraryMap;
use crate::manifest::Referable;
use crate::matching::Matching;
use crate::padding::{OtherLibrary, Padding, SavedIndex, current_sources};
use crate::{available_parallelism, synthetic, titles};

/// The harness's global options: where it reads and writes, how it
/// searches, and the larger library the corpus was drawn from.
#[derive(clap::Args)]
pub struct Harness {
    #[command(flatten)]
    pub paths: Paths,
    #[command(flatten)]
    pub search: SearchOptions,
    #[command(flatten)]
    pub other: OtherLibraryOptions,
}

/// Where the harness reads from and writes to.
#[derive(clap::Args)]
pub struct Paths {
    /// Corpus root containing `library/` and `sets/`.
    #[arg(long, default_value = "corpus")]
    pub corpus: PathBuf,
    /// Directory for derived data: rendered audio and `reports/`.
    #[arg(long, default_value = "work")]
    pub work: PathBuf,
    /// Peak store of the library, as written by `gunfinger index`.
    #[arg(long, default_value = "work/peaks")]
    pub peaks_dir: PathBuf,
    /// Saved sweep panels: each seed's held-out recordings and excerpts,
    /// drawn the first time the seed is used and kept as the library grows.
    #[arg(long, default_value = "docs/panels")]
    pub panels: PathBuf,
    /// The verdicts by ear on pairs of files (same recording or
    /// different), which `clusters` follows over its coverage rule.
    #[arg(long, default_value = "docs/pair-verdicts.txt")]
    pub verdicts: PathBuf,
    /// The judgments that two files carry the same track title although
    /// they are different recordings (ADR 0022): scoring credits either
    /// for the other's title, and counts such credits apart.
    #[arg(long, default_value = "docs/same-titles.txt")]
    pub same_titles: PathBuf,
}

/// How the harness searches, and where reports of other settings go.
#[derive(clap::Args)]
pub struct SearchOptions {
    /// Rungs searched by sweep, scan, robust and regress; the default is
    /// `identify`'s. Reports of other ladders go to `reports/ladder-<name>/`,
    /// so that calibrate and regress read one ladder at a time.
    #[arg(long, global = true, value_enum, default_value_t = Ladder::Both)]
    pub ladder: Ladder,
    /// Extend each ladder by this many rungs past either end, a step
    /// (0.4%) apart; reports go to `reports/extra-rungs-<n>/`.
    #[arg(long, global = true, default_value_t = 0)]
    pub extra_rungs: u32,
    /// Opt-in matching changes; their reports go to
    /// `reports/variant-<name>/`.
    #[command(flatten)]
    pub matching: Matching,
    /// Measure every detection of sweep, scan, robust, mixes and grid, and
    /// those of recall the second pass counted, with the peak verifier, a
    /// diagnostic that leaves detections unchanged. Reports go to
    /// `reports/verified/` below the usual directory.
    #[arg(long, global = true)]
    pub verify: bool,
}

/// The larger library the corpus was drawn from, known from its peak store.
#[derive(clap::Args)]
pub struct OtherLibraryOptions {
    /// The peak store of a larger library the corpus was drawn from, which
    /// holds a copy of every corpus file; its library need not be mounted.
    /// `map-library` pairs corpus files with their copies there and
    /// `clusters --from-peaks` finds the corpus recordings' other rips;
    /// then `sweep`, `scan`, `robust` and `memory` search an index of the
    /// corpus and the other library's remaining records (named
    /// `second-library/<path>`), counting those rips as correct. Reports go
    /// to `reports/library-<store directory name>/`.
    #[arg(long, global = true)]
    pub other_peaks_dir: Option<PathBuf>,
    /// With --other-peaks-dir: index only this many of the other library's
    /// records, a seeded random choice, for measuring against the size of
    /// the index. Reports go to `reports/library-<store>/sample-<count>/`;
    /// the map and the clusters are the whole library's.
    #[arg(long, global = true, requires = "other_peaks_dir")]
    pub other_sample: Option<usize>,
    /// With --other-peaks-dir: whether runs load the index of the corpus
    /// and the other library's records, saved in
    /// `indexes/library-<store directory name>.index` (ADR 0014). Runs that
    /// leave files out remove them from it. Not used with --other-sample,
    /// synthetic copies or `memory`.
    #[arg(long, global = true, value_enum, default_value_t = SavedIndexUse::Use)]
    pub saved_index: SavedIndexUse,
}

/// Whether runs with --other-peaks-dir use the saved index, as
/// `identify --saved-index`.
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SavedIndexUse {
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
pub enum Ladder {
    Turntable,
    KeyLock,
    Both,
}

impl Ladder {
    pub fn name(self) -> &'static str {
        match self {
            Ladder::Turntable => "turntable",
            Ladder::KeyLock => "key-lock",
            Ladder::Both => "both",
        }
    }

    pub fn rungs(self, extra: u32) -> Vec<Rung> {
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
pub struct IndexVariant {
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
    pub fn padding(&self) -> Result<Padding, String> {
        if self.synthetic_copies > synthetic::MAX_COPIES {
            return Err(format!(
                "at most {} synthetic copies are distinct",
                synthetic::MAX_COPIES
            ));
        }
        let other = match (&self.second_library, &self.second_peaks_dir) {
            (Some(root), Some(peaks_dir)) => Some(OtherLibrary::open(root, peaks_dir)?),
            _ => None,
        };
        Ok(Padding {
            copies: self.synthetic_copies,
            other,
            saved: None,
            exact: None,
        })
    }

    /// Report name suffix; empty for the unchanged index.
    pub fn suffix(&self) -> String {
        variant_suffix(self.synthetic_copies, self.second_library.is_some())
    }
}

pub fn variant_suffix(synthetic_copies: usize, second_library: bool) -> String {
    let mut suffix = String::new();
    if second_library {
        suffix.push_str("-second-library");
    }
    if synthetic_copies > 0 {
        suffix.push_str(&format!("-copies-{synthetic_copies}"));
    }
    suffix
}

pub const UNCHANGED_INDEX: IndexVariant = IndexVariant {
    synthetic_copies: 0,
    second_library: None,
    second_peaks_dir: None,
};

/// The library, if nothing vanished while it was listed (ADR 0015): an
/// evaluation on a library missing files that are still there would not be
/// repeatable.
pub fn nothing_vanished(library: Library) -> Result<Library, String> {
    match library.vanished.first() {
        None => Ok(library),
        Some(first) => Err(format!(
            "files or folders vanished while it was listed (`{first}`, {} in all); run again when nothing moves files in it",
            library.vanished.len()
        )),
    }
}

// What the options locate: the library, the peak stores, and the
// clusters, verdicts and judgments the evaluations read.
impl Harness {
    pub fn library(&self) -> Result<Library, String> {
        let root = self.paths.corpus.join("library");
        Library::scan(&root)
            .map_err(|error| error.to_string())
            .and_then(nothing_vanished)
            .map_err(|error| format!("cannot read the library at {}: {error}", root.display()))
    }

    pub fn store(&self) -> Result<PeakStore, String> {
        PeakStore::open(&self.paths.peaks_dir).map_err(|error| error.to_string())
    }

    /// The other library's store, which must exist.
    pub fn other_store(&self) -> Result<Option<PeakStore>, String> {
        let Some(dir) = &self.other.other_peaks_dir else {
            return Ok(None);
        };
        if !dir.is_dir() {
            return Err(format!("no peak store at {}", dir.display()));
        }
        PeakStore::open(dir)
            .map(Some)
            .map_err(|error| error.to_string())
    }

    /// The name of the other library's store directory.
    pub fn other_name(&self) -> Option<String> {
        let dir = self.other.other_peaks_dir.as_ref()?;
        let name = dir
            .canonicalize()
            .unwrap_or_else(|_| dir.clone())
            .file_name()?
            .to_string_lossy()
            .into_owned();
        Some(name)
    }

    /// The files a manifest's references may name: the corpus library's
    /// and, with --other-peaks-dir, those the other store holds a current
    /// record of.
    pub fn referable(&self, library: &Library) -> Result<Referable, String> {
        let referable = Referable::corpus(library);
        let Some(store) = self.other_store()? else {
            return Ok(referable);
        };
        let assets = current_sources(&store, available_parallelism())?;
        Ok(referable.with_other(&assets, store.dir()))
    }

    pub fn clusters(&self) -> Result<Clusters, String> {
        Clusters::load(&self.clusters_file())
    }

    /// The corpus clusters, also when another library is given.
    pub fn corpus_clusters(&self) -> Result<Clusters, String> {
        Clusters::load(
            &self
                .paths
                .work
                .join("reports")
                .join("duplicate-clusters.json"),
        )
    }

    pub fn verdicts(&self) -> Result<clusters::Verdicts, String> {
        clusters::Verdicts::load(&self.paths.verdicts)
    }

    pub fn same_titles(&self) -> Result<titles::SameTitles, String> {
        titles::SameTitles::load(&self.paths.same_titles)
    }
}

// Where the harness reads sets and writes reports, which depends on
// the search options and the other library.
impl Harness {
    pub fn sets(&self) -> PathBuf {
        self.paths.corpus.join("sets")
    }

    pub fn reports(&self) -> PathBuf {
        let mut reports = self.base_reports();
        if self.search.ladder != Ladder::Both {
            reports.push(format!("ladder-{}", self.search.ladder.name()));
        }
        if self.search.extra_rungs > 0 {
            reports.push(format!("extra-rungs-{}", self.search.extra_rungs));
        }
        if let Some(name) = self.search.matching.name() {
            reports.push(format!("variant-{name}"));
        }
        if self.search.verify {
            reports.push("verified");
        }
        reports
    }

    /// `reports/`, or with another library its own directory there, and a
    /// sample of it a directory below that.
    pub fn base_reports(&self) -> PathBuf {
        let library = self.library_reports();
        match self.other.other_sample {
            Some(count) => library.join(format!("sample-{count}")),
            None => library,
        }
    }

    /// `reports/`, or with another library its own directory there: where
    /// its map and clusters are, whether or not a sample of it is searched.
    pub fn library_reports(&self) -> PathBuf {
        let reports = self.paths.work.join("reports");
        match self.other_name() {
            Some(name) => reports.join(format!("library-{name}")),
            None => reports,
        }
    }

    pub fn baseline(&self, name: &str) -> PathBuf {
        self.paths.work.join("baselines").join(name)
    }

    /// Clusters come from library audio alone, whatever the ladder; with
    /// another library, they are the corpus clusters with the other
    /// library's rips of the corpus recordings (`clusters --from-peaks`).
    pub fn clusters_file(&self) -> PathBuf {
        self.library_reports().join("duplicate-clusters.json")
    }

    pub fn map_file(&self) -> Option<PathBuf> {
        self.other
            .other_peaks_dir
            .as_ref()
            .map(|_| self.library_reports().join("library-map.json"))
    }

    /// Where `seconds` writes: `seconds/`, or with another library its own
    /// directory there, as for reports.
    pub fn seconds_dir(&self) -> PathBuf {
        let dir = self.paths.work.join("seconds");
        match self.other_name() {
            Some(name) => dir.join(format!("library-{name}")),
            None => dir,
        }
    }

    /// Where `gaps` writes, beside `seconds_dir`.
    pub fn gaps_dir(&self) -> PathBuf {
        let dir = self.paths.work.join("gaps");
        match self.other_name() {
            Some(name) => dir.join(format!("library-{name}")),
            None => dir,
        }
    }
}

// What is searched: the ladder and the index.
impl Harness {
    pub fn rungs(&self) -> Vec<Rung> {
        self.search.ladder.rungs(self.search.extra_rungs)
    }

    /// The variant's padding, with the other library's records less the
    /// copies corpus files stand for, and their saved index.
    pub fn padding(&self, variant: &IndexVariant) -> Result<Padding, String> {
        let mut padding = variant.padding()?;
        let (Some(store), Some(map_file)) = (self.other_store()?, self.map_file()) else {
            return Ok(padding);
        };
        if padding.other.is_some() {
            return Err(String::from(
                "--second-library and --other-peaks-dir cannot be combined",
            ));
        }
        let map = LibraryMap::load(&map_file)?;
        let second = OtherLibrary::from_store(store, &map.stood_for())?;
        padding.other = Some(match self.other.other_sample {
            Some(count) => second.sampled(count, OTHER_SAMPLE_SEED),
            None => second,
        });
        padding.saved = self.saved_index();
        Ok(padding)
    }

    /// The saved index of the corpus and the whole other library, unless
    /// the run searches a sample of it or uses none.
    pub fn saved_index(&self) -> Option<SavedIndex> {
        if self.other.other_sample.is_some() {
            return None;
        }
        let file = self
            .paths
            .work
            .join("indexes")
            .join(format!("library-{}.index", self.other_name()?));
        match self.other.saved_index {
            SavedIndexUse::Use => Some(SavedIndex::Use(file)),
            SavedIndexUse::Rebuild => Some(SavedIndex::Rebuild(file)),
            SavedIndexUse::Off => None,
        }
    }

    /// With another library, the seed's panel must have been drawn from
    /// the corpus alone: its clusters would put the other library's files
    /// among the held-out recordings.
    pub fn check_panel(&self, seed: u64) -> Result<(), String> {
        let panel = self.paths.panels.join(format!("sweep-seed-{seed}.json"));
        if self.other.other_peaks_dir.is_some() && !panel.exists() {
            return Err(format!(
                "{} does not exist; draw it with `sweep --seed {seed}` without --other-peaks-dir first",
                panel.display()
            ));
        }
        Ok(())
    }
}

/// The draw of `--other-sample`: one subset per size, the same in every run.
pub const OTHER_SAMPLE_SEED: u64 = 2026;

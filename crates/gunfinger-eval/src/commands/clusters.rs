//! The `clusters` command: duplicate clusters of the library, and with the
//! other library its rips of the corpus recordings.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use gunfinger_core::indexing::load_records;
use gunfinger_core::library::ignore::{self, IGNORE_FILE};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;

use super::recall::recall_panel;
use crate::Paths;
use crate::clusters::Clusters;
use crate::files::write_json;
use crate::library_map::LibraryMap;
use crate::manifest::Referable;
use crate::padding::{SECOND_LIBRARY_PREFIX, current_sources};
use crate::{clusters, manifest};

#[derive(clap::Args)]
pub struct ClustersArgs {
    /// Search each file's stored peaks instead of its decoded audio,
    /// and compare the result with the clusters in use rather than
    /// replacing them.
    #[arg(long)]
    pub from_peaks: bool,
    /// With --other-peaks-dir: search only this many corpus files,
    /// spread over the library, and no further rounds, for timing. The
    /// clusters in use are not replaced; the pairs go to
    /// `duplicate-clusters-sample.json`.
    #[arg(long, requires = "from_peaks")]
    pub sample: Option<usize>,
    /// With --other-peaks-dir: also search the other library's files
    /// this set's manifest references (repeatable), so that their rips
    /// join their clusters.
    #[arg(long = "manifest", requires = "from_peaks")]
    pub manifests: Vec<String>,
    /// With --other-peaks-dir: also search the development sources of
    /// the recall panel of this seed.
    #[arg(long, requires = "from_peaks")]
    pub recall_panel: Option<u64>,
    /// When a chain of joins links two files judged
    /// different, cut the chain's join with the fewest hits per second
    /// of aligned span and list it, instead of stopping. A cut is a
    /// guess to judge by ear, not a verdict.
    #[arg(long)]
    pub cut_sparsest: bool,
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
    pub reuse_pairs: Option<Option<PathBuf>>,
}

pub fn clusters(paths: &Paths, jobs: usize, args: ClustersArgs) -> Result<(), String> {
    let ClustersArgs {
        from_peaks,
        sample,
        manifests,
        recall_panel,
        cut_sparsest,
        reuse_pairs,
    } = args;
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
pub fn other_ignore_file(other: &PeakStore) -> Result<Option<clusters::IgnoreFile>, String> {
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
    let other_assets = current_sources(other, jobs)?;
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

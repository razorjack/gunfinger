//! The `recall` command: a panel drawn from the other library beyond the
//! corpus recordings.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use gunfinger_core::library::Asset;
use gunfinger_core::store::PeakStore;

use crate::available_parallelism;
use crate::files::write_json;
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::library_map::LibraryMap;
use crate::padding::{OTHER_LIBRARY_PREFIX, current_sources};
use crate::stereo::FrontEnd;
use crate::{recall, sweep};

#[derive(clap::Args)]
pub struct RecallArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    /// Draw the panel and render the excerpts, without searching.
    #[arg(long)]
    pub prepare: bool,
    /// Leave each indexed source's record and its identical copies out
    /// of the index, so that only its recording's other rips can
    /// answer; a source without one is unobservable, scored like a
    /// held-out source (`-other-rips` in the report's name).
    #[arg(long)]
    pub other_rips_only: bool,
    /// The query sets, searched in this order against one index, each
    /// written as it finishes: the excerpts as rendered (`clean`),
    /// under `robust`'s combined damage (`damaged`), or their first
    /// 15 and 20 s between held-out audio (`brief`, `brief-damaged`).
    #[arg(long, value_enum, value_delimiter = ',', default_value = "clean")]
    pub queries: Vec<recall::QuerySet>,
    #[command(flatten)]
    pub study: RecallStudy,
}

pub fn recall(harness: &Harness, jobs: usize, args: RecallArgs) -> Result<(), String> {
    let RecallArgs {
        seed,
        prepare,
        other_rips_only,
        queries: sets,
        study,
    } = args;
    let Some(other) = harness.other_store()? else {
        return Err(String::from("recall needs --other-peaks-dir"));
    };
    if study.set_aside_like.is_some()
        && (harness.search.matching.single_pass || harness.search.matching.drop_fullest > 0.0)
    {
        return Err(String::from(
            "--set-aside-like sets aside the default matcher's lists; it takes neither --single-pass nor --drop-fullest",
        ));
    }
    if study.trace_candidates && harness.search.matching.front_end != FrontEnd::Downmix {
        return Err(String::from(
            "the first pass is traced on one channel: --trace-candidates takes the downmix front end only",
        ));
    }
    let (panel, assets) = recall_panel(harness, &other, seed)?;
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
    let dir = harness.paths.work.join("recall");
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
    let clusters = harness.clusters()?;
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
        &harness.library()?,
        &harness.store()?,
        &clusters,
        &sets,
        &recall::Options {
            same_titles: &harness.same_titles()?,
            seed,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            left_out: &left_out,
            verify: harness.search.verify,
            trace: if study.trace_candidates {
                sweep::Trace::Candidates
            } else {
                sweep::Trace::Off
            },
            set_aside_like: study.set_aside_like.as_deref(),
            jobs,
        },
        |report| {
            let name = report
                .query_set
                .report_name(seed, other_rips_only, &study.suffix());
            write_json(&harness.reports().join(name), &report)?;
            recall::print_summary(&report);
            Ok(())
        },
    )
}

/// Recall options for tracing what moves its detections (session 59). Each
/// adds to the report's name, so no default report is written over.
#[derive(clap::Args)]
pub struct RecallStudy {
    /// Keep each query's first-pass candidates (detections of at least
    /// 10 hits, which the second pass measures again) with their speed
    /// and span; the report's name ends in `-trace`.
    #[arg(long)]
    trace_candidates: bool,
    /// Search the index with the lists set aside that the harness's index
    /// saved in this file sets aside with the same files left out, and
    /// record how the two differ: for comparing with an index of an
    /// earlier store. The file is only read. Default matcher only; the
    /// report's name ends in `-set-aside-like`.
    #[arg(long, value_name = "FILE")]
    set_aside_like: Option<PathBuf>,
}

impl RecallStudy {
    fn suffix(&self) -> String {
        let mut suffix = String::new();
        if self.set_aside_like.is_some() {
            suffix.push_str("-set-aside-like");
        }
        if self.trace_candidates {
            suffix.push_str("-trace");
        }
        suffix
    }
}

/// The recall panel of `seed`, drawn the first time from the other
/// library's records outside the clusters in use and the copies of corpus
/// files.
pub fn recall_panel(
    harness: &Harness,
    other: &PeakStore,
    seed: u64,
) -> Result<(recall::Panel, Vec<Asset>), String> {
    let Some(map_file) = harness.map_file() else {
        return Err(String::from("the recall panel needs --other-peaks-dir"));
    };
    let mut corpus_recordings: BTreeSet<String> = harness
        .clusters()?
        .duplicates
        .into_iter()
        .flatten()
        .collect();
    corpus_recordings.extend(
        LibraryMap::load(&map_file)?
            .copied()
            .into_iter()
            .map(|path| format!("{OTHER_LIBRARY_PREFIX}{path}")),
    );
    let assets = current_sources(other, available_parallelism())?;
    let panel = recall::Panel::for_seed(
        other,
        &assets,
        &corpus_recordings,
        seed,
        &harness.paths.panels,
    )?;
    Ok((panel, assets))
}

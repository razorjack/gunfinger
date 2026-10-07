//! `gunfinger prune`: delete peak records and skip notes of files that are
//! no longer in the library, and temporary files left by interrupted runs.
//!
//! A wrong `--library` (a typo, an unmounted share, another library sharing
//! the store) makes every record look orphaned, so prune lists what it would
//! delete unless `--yes` is given, and refuses outright when the library is
//! empty or most of the store would go.

use std::path::Path;

use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, Stored};
use miette::{IntoDiagnostic, miette};

use crate::catalog::scan_library;
use crate::console::Console;
use crate::survey::survey;

/// Paths listed before `--yes`.
const LISTED: usize = 20;
/// Without `--force`, prune refuses to delete more than this share of the
/// store's records and notes.
const MAX_SHARE: f64 = 0.5;

pub struct Request<'a> {
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub yes: bool,
    pub force: bool,
    pub console: &'a Console,
}

pub fn run(request: &Request) -> miette::Result<()> {
    let library = scan_library(request.library, request.console)?;
    if library.assets.is_empty() {
        return Err(miette!(
            help = "check --library, and that the share is mounted",
            "the library at {} has no audio files; nothing is pruned",
            request.library.display()
        ));
    }
    let store = PeakStore::open(request.peaks_dir).into_diagnostic()?;
    store.check_library(request.library).into_diagnostic()?;
    let survey = survey(&library, &store, &Profile::CURRENT)?;
    let share = survey.orphans.len() as f64 / survey.records_and_notes().max(1) as f64;
    if share > MAX_SHARE && !request.force {
        return Err(miette!(
            help = "check --library; a peak store shared by two libraries holds records of both. --force prunes anyway",
            "{} of {} records and notes in {} are for files not in {}",
            survey.orphans.len(),
            survey.records_and_notes(),
            request.peaks_dir.display(),
            request.library.display()
        ));
    }

    let doomed: Vec<(&Stored, String)> = survey
        .orphans
        .iter()
        .map(|orphan| (&orphan.stored, orphan.source.clone()))
        .chain(survey.leftovers.iter().map(|stored| {
            (
                stored,
                format!("temporary file {}", stored.file().display()),
            )
        }))
        .collect();
    if doomed.is_empty() {
        request.console.info("nothing to prune");
        return Ok(());
    }
    if !request.yes {
        println!("would delete {} files:", doomed.len());
        for (_, what) in doomed.iter().take(LISTED) {
            println!("  {what}");
        }
        if doomed.len() > LISTED {
            println!("  ... and {} more", doomed.len() - LISTED);
        }
        println!("run again with --yes to delete them");
        return Ok(());
    }
    let mut freed = 0;
    for (stored, _) in &doomed {
        freed += std::fs::metadata(stored.file()).map_or(0, |metadata| metadata.len());
        store.remove(stored).into_diagnostic()?;
    }
    println!(
        "deleted {} files, {:.1} MB",
        doomed.len(),
        freed as f64 / 1e6
    );
    Ok(())
}

//! `gunfinger index`: bring the peak store up to date with a library.

use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use gunfinger_core::indexing::{IndexingOptions, Outcome, index_library};
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, SkipReason};
use miette::{IntoDiagnostic, WrapErr};

use crate::console::Console;

pub fn run(
    library_root: &Path,
    peaks_dir: &Path,
    jobs: usize,
    max_track_minutes: u64,
    retry_skipped: bool,
    console: &Console,
) -> miette::Result<()> {
    let started = Instant::now();
    let library = Library::scan(library_root)
        .into_diagnostic()
        .wrap_err_with(|| format!("could not read the library at {}", library_root.display()))?;
    let store = PeakStore::open(peaks_dir).into_diagnostic()?;
    let options = IndexingOptions {
        jobs,
        max_track: Duration::from_secs(max_track_minutes * 60),
        retry_skipped,
    };
    console.info(format_args!(
        "indexing {} audio files from {} into {} with {} jobs",
        library.assets.len(),
        library_root.display(),
        store.dir().display(),
        options.jobs
    ));

    let finished = AtomicUsize::new(0);
    let total = library.assets.len();
    let outcomes = index_library(
        &library,
        &store,
        &Profile::CURRENT,
        &options,
        |asset, outcome| {
            let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
            match outcome {
                Outcome::Extracted { peaks } => {
                    console.info(format_args!(
                        "[{count}/{total}] {peaks} peaks: {}",
                        asset.path
                    ));
                }
                Outcome::UpToDate | Outcome::Remembered(_) => {}
                Outcome::TooLong => console.warning(format_args!(
                    "[{count}/{total}] skipped {}: longer than {max_track_minutes} minutes",
                    asset.path
                )),
                Outcome::Failed { reason } => {
                    console.error(format_args!("[{count}/{total}] {reason}"));
                }
            }
        },
    );

    print_summary(&library, &outcomes, started.elapsed(), console);
    Ok(())
}

fn print_summary(library: &Library, outcomes: &[Outcome], elapsed: Duration, console: &Console) {
    let count =
        |wanted: fn(&Outcome) -> bool| outcomes.iter().filter(|outcome| wanted(outcome)).count();
    let remembered: Vec<&SkipReason> = outcomes
        .iter()
        .filter_map(|outcome| match outcome {
            Outcome::Remembered(reason) => Some(reason),
            _ => None,
        })
        .collect();
    console.info(format_args!(
        "done in {:.1} s: {} extracted, {} up to date, {} too long, {} failed, {} passed over as before",
        elapsed.as_secs_f64(),
        count(|outcome| matches!(outcome, Outcome::Extracted { .. })),
        count(|outcome| matches!(outcome, Outcome::UpToDate)),
        count(|outcome| matches!(outcome, Outcome::TooLong)),
        count(|outcome| matches!(outcome, Outcome::Failed { .. })),
        remembered.len(),
    ));
    if !remembered.is_empty() {
        let failed = remembered
            .iter()
            .filter(|reason| matches!(reason, SkipReason::Failed(_)))
            .count();
        console.info(format_args!(
            "  {failed} failed and {} were too long in an earlier run and have not changed since; --retry-skipped tries them again",
            remembered.len() - failed
        ));
    }
    if library.skipped_total() > 0 {
        console.info(format_args!(
            "skipped {} other files:",
            library.skipped_total()
        ));
        for (reason, count) in &library.skipped {
            console.info(format_args!("  {count:>5}  {reason}"));
        }
    }
    for (asset, outcome) in library.assets.iter().zip(outcomes) {
        if let Outcome::Failed { reason } = outcome {
            console.error(format_args!("{}: {reason}", asset.path));
        }
    }
}

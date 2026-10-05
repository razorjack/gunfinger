//! `gunfinger index`: bring the peak store up to date with a library.

use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use gunfinger_core::indexing::{IndexingOptions, Outcome, index_library};
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;
use miette::{IntoDiagnostic, WrapErr};

pub fn run(
    library_root: &Path,
    peaks_dir: &Path,
    jobs: Option<NonZeroUsize>,
    max_track_minutes: u64,
) -> miette::Result<()> {
    let started = Instant::now();
    let library = Library::scan(library_root)
        .into_diagnostic()
        .wrap_err_with(|| format!("could not read the library at {}", library_root.display()))?;
    let store = PeakStore::open(peaks_dir).into_diagnostic()?;
    let options = IndexingOptions {
        jobs: jobs.map_or_else(crate::default_jobs, NonZeroUsize::get),
        max_track: Duration::from_secs(max_track_minutes * 60),
    };
    eprintln!(
        "indexing {} audio files from {} into {} with {} jobs",
        library.assets.len(),
        library_root.display(),
        store.dir().display(),
        options.jobs
    );

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
                    eprintln!("[{count}/{total}] {peaks} peaks: {}", asset.path);
                }
                Outcome::UpToDate => {}
                Outcome::TooLong => {
                    eprintln!(
                        "[{count}/{total}] warning: skipped {}: longer than {max_track_minutes} minutes",
                        asset.path
                    );
                }
                Outcome::Failed { reason } => {
                    eprintln!("[{count}/{total}] error: {reason}");
                }
            }
        },
    );

    print_summary(&library, &outcomes, started.elapsed());
    Ok(())
}

fn print_summary(library: &Library, outcomes: &[Outcome], elapsed: Duration) {
    let count =
        |wanted: fn(&Outcome) -> bool| outcomes.iter().filter(|outcome| wanted(outcome)).count();
    eprintln!(
        "done in {:.1} s: {} extracted, {} up to date, {} too long, {} failed",
        elapsed.as_secs_f64(),
        count(|outcome| matches!(outcome, Outcome::Extracted { .. })),
        count(|outcome| matches!(outcome, Outcome::UpToDate)),
        count(|outcome| matches!(outcome, Outcome::TooLong)),
        count(|outcome| matches!(outcome, Outcome::Failed { .. })),
    );
    eprintln!("skipped {} other files:", library.skipped_total());
    for (reason, count) in &library.skipped {
        eprintln!("  {count:>5}  {reason}");
    }
    for (asset, outcome) in library.assets.iter().zip(outcomes) {
        if let Outcome::Failed { reason } = outcome {
            eprintln!("failed: {}: {reason}", asset.path);
        }
    }
}

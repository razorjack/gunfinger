//! Loading the in-memory index of a library from the peak store.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::time::Instant;

use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakRecord, PeakStore};
use miette::{IntoDiagnostic, WrapErr, miette};

/// The peak records of a library and the index built from them.
pub struct Catalog {
    pub store: PeakStore,
    pub records: Vec<PeakRecord>,
    pub index: Index,
}

impl Catalog {
    /// Scans the library, loads the current peak record of every asset not
    /// listed in `exclude_from`, and builds the index. Assets without a
    /// current record are reported on stderr and left out.
    pub fn open(
        library_root: &Path,
        peaks_dir: &Path,
        exclude_from: Option<&Path>,
    ) -> miette::Result<Catalog> {
        let started = Instant::now();
        let library = Library::scan(library_root)
            .into_diagnostic()
            .wrap_err_with(|| {
                format!("could not read the library at {}", library_root.display())
            })?;
        let store = PeakStore::open(peaks_dir).into_diagnostic()?;
        let excluded = match exclude_from {
            Some(path) => read_exclusions(path)?,
            None => BTreeSet::new(),
        };
        let (records, problems) = load_records(&library, &store, &Profile::CURRENT, &excluded);
        if !problems.is_empty() {
            eprintln!(
                "warning: {} assets are left out because they have no current peak record:",
                problems.len()
            );
            for problem in &problems {
                eprintln!("  {problem}");
            }
        }
        if records.is_empty() {
            return Err(miette!(
                help = "run `gunfinger index {}` first",
                "no indexed assets in {}",
                library_root.display()
            ));
        }
        let index = Index::build(&records).into_diagnostic()?;
        eprintln!(
            "index: {} assets ({} excluded), {} postings, {:.1} MB, built in {:.1} s",
            records.len(),
            excluded.len(),
            index.posting_count(),
            index.size_bytes() as f64 / 1e6,
            started.elapsed().as_secs_f64()
        );
        Ok(Catalog {
            store,
            records,
            index,
        })
    }
}

/// One asset path per line, relative to the library root. Blank lines and
/// lines starting with `#` are ignored.
fn read_exclusions(path: &Path) -> miette::Result<BTreeSet<String>> {
    let text = fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("could not read the exclusion list {}", path.display()))?;
    Ok(text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
        .collect())
}

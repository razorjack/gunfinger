//! Loading the in-memory index of a library from the peak store.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakRecord, PeakStore};
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::console::Console;

/// The peak records of a library and the index built from them.
pub struct Catalog {
    /// The library root, absolute when it can be resolved.
    pub root: PathBuf,
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
        console: &Console,
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
            let mut message = format!(
                "{} assets are left out because they have no current peak record:",
                problems.len()
            );
            for problem in &problems {
                message.push_str(&format!("\n  {problem}"));
            }
            console.warning(message);
        }
        if records.is_empty() {
            return Err(miette!(
                help = "run `gunfinger index {}` first",
                "no indexed assets in {}",
                library_root.display()
            ));
        }
        let index = Index::build(&records).into_diagnostic()?;
        console.info(format_args!(
            "index: {} assets ({} excluded), {} postings, {:.1} MB, built in {:.1} s",
            records.len(),
            excluded.len(),
            index.posting_count(),
            index.size_bytes() as f64 / 1e6,
            started.elapsed().as_secs_f64()
        ));
        Ok(Catalog {
            root: absolute(library_root),
            store,
            records,
            index,
        })
    }
}

/// Reports outlive the working directory they were written in.
pub fn absolute(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
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

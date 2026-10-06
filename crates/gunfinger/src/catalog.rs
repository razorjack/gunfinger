//! Loading the in-memory index of a library from the peak store.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gunfinger_core::index::Index;
use gunfinger_core::indexing::build_index;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, StoreError};
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::console::Console;

/// The index of a library, built from its peak store.
pub struct Catalog {
    pub library: Library,
    pub store: PeakStore,
    pub index: Index,
}

impl Catalog {
    /// Scans the library and builds the index from the current peak record
    /// of every asset not listed in `exclude_from`, reading one record at a
    /// time. Assets without a current record are reported on stderr and
    /// left out.
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
        let (index, problems) =
            build_index(&library, &store, &Profile::CURRENT, &excluded).into_diagnostic()?;
        report_left_out(&problems, console);
        if index.assets().is_empty() {
            return Err(miette!(
                help = "run `gunfinger index {}` first",
                "no indexed assets in {}",
                library_root.display()
            ));
        }
        console.detail(format_args!(
            "index: {} assets ({} excluded), {} postings, {:.1} MB, built in {:.1} s",
            index.assets().len(),
            excluded.len(),
            index.posting_count(),
            index.size_bytes() as f64 / 1e6,
            started.elapsed().as_secs_f64()
        ));
        Ok(Catalog {
            library,
            store,
            index,
        })
    }
}

/// Library files without a current peak record. Files `index` passed over
/// on purpose (damaged or too long) are listed with `--verbose` only, as
/// indexing again would not change them. Files not indexed yet always get
/// a warning, because the search silently misses their tracks; so do
/// unreadable records.
fn report_left_out(problems: &[StoreError], console: &Console) {
    let mut passed_over = Vec::new();
    let mut not_indexed = Vec::new();
    let mut broken = Vec::new();
    for problem in problems {
        match problem {
            StoreError::Skipped { asset, reason } => {
                passed_over.push(format!("{asset} ({reason})"));
            }
            StoreError::Missing { asset } => not_indexed.push(asset.clone()),
            StoreError::Stale { asset } => {
                not_indexed.push(format!("{asset} (changed since it was indexed)"));
            }
            StoreError::Corrupt { .. } | StoreError::Io { .. } => broken.push(problem.to_string()),
        }
    }
    if !passed_over.is_empty() {
        console.detail(listed(
            &format!(
                "{} left out because `index` passed over {} (`index --retry-skipped` tries again):",
                files(passed_over.len()),
                if passed_over.len() == 1 { "it" } else { "them" }
            ),
            &passed_over,
        ));
    }
    if !not_indexed.is_empty() {
        let summary = if not_indexed.len() == 1 {
            String::from("1 library file is not indexed, so its track cannot be found")
        } else {
            format!(
                "{} library files are not indexed, so their tracks cannot be found",
                not_indexed.len()
            )
        } + "; run `gunfinger index`";
        if console.is_verbose() {
            console.warning(listed(&format!("{summary}:"), &not_indexed));
        } else {
            console.warning(format_args!("{summary} (--verbose lists them)"));
        }
    }
    if !broken.is_empty() {
        console.warning(listed(
            &format!("{} cannot be read:", records(broken.len())),
            &broken,
        ));
    }
}

fn files(count: usize) -> String {
    if count == 1 {
        String::from("1 file")
    } else {
        format!("{count} files")
    }
}

fn records(count: usize) -> String {
    if count == 1 {
        String::from("1 peak record")
    } else {
        format!("{count} peak records")
    }
}

fn listed(heading: &str, items: &[String]) -> String {
    let mut text = heading.to_owned();
    for item in items {
        text.push_str(&format!("\n  {item}"));
    }
    text
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

//! Loading the in-memory index of a library from the peak store.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use gunfinger_core::index::Index;
use gunfinger_core::indexing::{
    BuiltIndex, TrackLength, build_index, indexable_assets, library_revision,
};
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, StoreError};
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::console::Console;

/// The index of a library, built from its peak store.
pub struct Catalog {
    pub library: Library,
    pub store: PeakStore,
    pub index: Index,
    /// The library revision of the indexed assets.
    pub revision: String,
}

impl Catalog {
    /// Scans the library and builds the index from the current peak record
    /// of every asset not listed in `exclude_from` and within
    /// `track_length`, reading one record at a time. Assets without a
    /// current record are reported on stderr and left out.
    pub fn open(
        library_root: &Path,
        peaks_dir: &Path,
        exclude_from: Option<&Path>,
        track_length: TrackLength,
        console: &Console,
    ) -> miette::Result<Catalog> {
        let started = Instant::now();
        let library = scan_library(library_root, console)?;
        let store = PeakStore::open(peaks_dir).into_diagnostic()?;
        store.check_library(library_root).into_diagnostic()?;
        let excluded = match exclude_from {
            Some(path) => read_exclusions(path)?,
            None => BTreeSet::new(),
        };
        let BuiltIndex {
            index,
            revision,
            problems,
            outside,
        } = build_index(&library, &store, &Profile::CURRENT, &excluded, track_length)
            .into_diagnostic()?;
        report_left_out(&problems, console);
        report_outside(&outside, track_length, console);
        if index.assets().is_empty() {
            return Err(miette!(
                help = format!("run `gunfinger index {}` first", library_root.display()),
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
            revision,
        })
    }

    /// The library revision an index built now would have, reading only the
    /// headers of the peak records: enough to tell whether a saved report
    /// is current without building the index.
    pub fn revision_now(
        library_root: &Path,
        peaks_dir: &Path,
        exclude_from: Option<&Path>,
        track_length: TrackLength,
        console: &Console,
    ) -> miette::Result<String> {
        let library = scan_library(library_root, console)?;
        let store = PeakStore::open(peaks_dir).into_diagnostic()?;
        store.check_library(library_root).into_diagnostic()?;
        let excluded = match exclude_from {
            Some(path) => read_exclusions(path)?,
            None => BTreeSet::new(),
        };
        Ok(library_revision(indexable_assets(
            &library,
            &store,
            &Profile::CURRENT,
            &excluded,
            track_length,
        )))
    }
}

/// Scans the library, counting the files seen on a terminal: a large
/// network share takes minutes to list.
pub fn scan_library(root: &Path, console: &Console) -> miette::Result<Library> {
    let library = Library::scan_with_progress(root, |seen| {
        if seen % 100 == 0 {
            console.progress(format_args!("listing {}: {seen} files", root.display()));
        }
    });
    console.progress_done();
    library
        .into_diagnostic()
        .wrap_err_with(|| format!("could not read the library at {}", root.display()))
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
            StoreError::Corrupt { .. }
            | StoreError::Io { .. }
            | StoreError::OtherLibrary { .. } => {
                broken.push(problem.to_string());
            }
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

/// Indexed files the current range leaves out, with `--verbose` only: the
/// range was set on purpose.
fn report_outside(outside: &[(String, Duration)], track_length: TrackLength, console: &Console) {
    if outside.is_empty() {
        return;
    }
    let lengths: Vec<String> = outside
        .iter()
        .map(|(path, length)| format!("{path} ({})", format_timecode(*length)))
        .collect();
    console.detail(listed(
        &format!(
            "{} left out because the track length is {track_length} (--min-track, --max-track):",
            files(outside.len())
        ),
        &lengths,
    ));
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

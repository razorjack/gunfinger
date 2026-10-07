//! Loading the in-memory index of a library from the peak store. Without a
//! library to read, the index holds the store's own current records: each
//! names its file, so the store alone is enough to identify tracks.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use gunfinger_core::index::Index;
use gunfinger_core::indexing::{
    BuiltIndex, TrackLength, build_index, indexable_assets, library_revision,
};
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, StoreError};
use gunfinger_core::tags::Tags;
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::config;
use crate::console::Console;

/// What a search's index is built from: a library and its peak store, less
/// the files excluded and those outside the track length range.
pub struct Source<'a> {
    /// The library given or configured, if any.
    pub library: Option<&'a Path>,
    pub peaks_dir: &'a Path,
    /// Use the store's own records without reading the library.
    pub store_only: bool,
    pub exclude_from: Option<&'a Path>,
    pub track_length: TrackLength,
}

/// The files an index can be built from: the library's, or those of the
/// store's current records when there is no library to read.
pub struct Indexable {
    /// The library root that reports name, where the tracks can be played
    /// from; empty when the store names none and none was given.
    pub root: PathBuf,
    pub store: PeakStore,
    assets: Vec<Asset>,
    /// Records whose header could not be read, without a library.
    problems: Vec<StoreError>,
    excluded: BTreeSet<String>,
    track_length: TrackLength,
    store_only: bool,
}

impl Indexable {
    /// Lists the library, or the store's records when the library cannot be
    /// read, none is given or `store_only` asks for it, saying which in one
    /// line. A store that names another library is refused unless
    /// `store_only` is given.
    pub fn find(source: &Source, console: &Console) -> miette::Result<Indexable> {
        let excluded = match source.exclude_from {
            Some(path) => read_exclusions(path)?,
            None => BTreeSet::new(),
        };
        let found =
            |root: PathBuf, store: PeakStore, (assets, problems): Records, store_only| Indexable {
                root,
                store,
                assets,
                problems,
                excluded,
                track_length: source.track_length,
                store_only,
            };
        let peaks_dir = source.peaks_dir;
        if source.store_only {
            let Some((store, records)) = store_records(peaks_dir)? else {
                return Err(miette!(
                    help = "`gunfinger index` fills it from a library",
                    "no current peak records in {}",
                    peaks_dir.display()
                ));
            };
            let root = match store.library().into_diagnostic()? {
                Some(name) => PathBuf::from(name),
                None => source.library.map(Path::to_path_buf).unwrap_or_default(),
            };
            return Ok(found(root, store, records, true));
        }
        let Some(root) = source.library else {
            let Some((store, records)) = store_records(peaks_dir)? else {
                return Err(miette!(
                    help = format!(
                        "{}, or --peaks-dir with a peak store",
                        config::library_help()
                    ),
                    "no library given, and no peak records in {}",
                    peaks_dir.display()
                ));
            };
            console.info(format_args!(
                "no library given; searching the {} records in the peak store {}",
                records.0.len(),
                peaks_dir.display()
            ));
            let root = store.library().into_diagnostic()?.unwrap_or_default();
            return Ok(found(PathBuf::from(root), store, records, true));
        };
        match list(root, console) {
            Ok(library) => {
                let store = PeakStore::open(peaks_dir).into_diagnostic()?;
                store.check_library(root).into_diagnostic()?;
                Ok(found(
                    library.root,
                    store,
                    (library.assets, Vec::new()),
                    false,
                ))
            }
            Err(error) => {
                let Some((store, records)) = store_records(peaks_dir)? else {
                    return Err(error).into_diagnostic().wrap_err_with(|| {
                        format!("could not read the library at {}", root.display())
                    });
                };
                store.check_library(root).into_diagnostic()?;
                console.warning(format_args!(
                    "cannot read the library at {} ({error}); searching the {} records in its peak store instead",
                    root.display(),
                    records.0.len()
                ));
                Ok(found(root.to_path_buf(), store, records, true))
            }
        }
    }

    /// The library revision an index built now would have, reading only the
    /// headers of the peak records: enough to tell whether a saved report
    /// is current without building the index.
    pub fn revision(&self) -> String {
        library_revision(indexable_assets(
            &self.assets,
            &self.store,
            &Profile::CURRENT,
            &self.excluded,
            self.track_length,
        ))
    }
}

/// The files of a store's current records, and the records whose header
/// could not be read.
pub type Records = (Vec<Asset>, Vec<StoreError>);

/// The store at `dir` and its current records, unless it does not exist or
/// holds none. A missing store is not created.
pub fn store_records(dir: &Path) -> miette::Result<Option<(PeakStore, Records)>> {
    if !dir.is_dir() {
        return Ok(None);
    }
    let store = PeakStore::open(dir).into_diagnostic()?;
    let records = store.current_sources(&Profile::CURRENT).into_diagnostic()?;
    Ok((!records.0.is_empty()).then_some((store, records)))
}

/// The index of a library, built from its peak store.
pub struct Catalog {
    /// The library root, empty when not known (see `Indexable::root`).
    pub root: PathBuf,
    pub store: PeakStore,
    pub index: Index,
    /// The library revision of the indexed assets.
    pub revision: String,
    /// The indexed files by path.
    pub sources: BTreeMap<String, Asset>,
}

impl Catalog {
    /// Builds the index from the current peak record of every indexable
    /// file not excluded and within the track length range, reading one
    /// record at a time. Files without a current record are reported on
    /// stderr and left out.
    pub fn open(indexable: Indexable, console: &Console) -> miette::Result<Catalog> {
        let started = Instant::now();
        let Indexable {
            root,
            store,
            assets,
            mut problems,
            excluded,
            track_length,
            store_only,
        } = indexable;
        let built = build_index(&assets, &store, &Profile::CURRENT, &excluded, track_length)
            .into_diagnostic()?;
        let BuiltIndex {
            index,
            revision,
            outside,
            sources,
            ..
        } = built;
        problems.extend(built.problems);
        report_left_out(&problems, console);
        report_outside(&outside, track_length, console);
        if index.assets().is_empty() {
            return Err(if store_only {
                miette!(
                    help = "--verbose lists the files left out",
                    "none of the records in {} is searchable",
                    store.dir().display()
                )
            } else {
                miette!(
                    help = format!("run `gunfinger index {}` first", root.display()),
                    "no indexed assets in {}",
                    root.display()
                )
            });
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
            root,
            store,
            index,
            revision,
            sources: sources
                .into_iter()
                .map(|source| (source.path.clone(), source))
                .collect(),
        })
    }

    /// The tags the store holds for an indexed file.
    pub fn tags(&self, path: &str) -> Option<Tags> {
        self.store.tags(self.sources.get(path)?)
    }
}

/// Scans the library, counting the files seen on a terminal: a large
/// network share takes minutes to list.
pub fn scan_library(root: &Path, console: &Console) -> miette::Result<Library> {
    list(root, console)
        .into_diagnostic()
        .wrap_err_with(|| format!("could not read the library at {}", root.display()))
}

fn list(root: &Path, console: &Console) -> io::Result<Library> {
    let library = Library::scan_with_progress(root, |seen| {
        if seen % 100 == 0 {
            console.progress(format_args!("listing {}: {seen} files", root.display()));
        }
    });
    console.progress_done();
    library
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
    path.canonicalize()
        .or_else(|_| std::path::absolute(path))
        .unwrap_or_else(|_| path.to_path_buf())
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

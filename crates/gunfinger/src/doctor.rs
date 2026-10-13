//! `gunfinger doctor`: check the tools, the settings, the library and the
//! peak store, and say what to do about each problem.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use gunfinger_core::hash;
use gunfinger_core::index::saved::{self, Provenance, Unusable};
use gunfinger_core::index::{MAX_ASSETS, addressable_length};
use gunfinger_core::indexing::{TrackLength, plan_index};
use gunfinger_core::library::ignore::IGNORE_FILE;
use gunfinger_core::library::{Asset, Library, ScanError};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, Stored};
use miette::miette;

use crate::catalog::{absolute, list, store_records, vanished_entries};
use crate::config::{self, Settings};
use crate::console::Console;
use crate::style::Style;
use crate::survey::{Absence, counted, counted_kinds, survey};
use crate::table::timecode;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Ok,
    Note,
    Warning,
    Problem,
}

struct Checkup {
    style: Style,
    problems: usize,
}

impl Checkup {
    fn section(&self, title: &str) {
        println!("{}", self.style.bold(title));
    }

    fn line(&mut self, status: Status, message: impl std::fmt::Display) {
        let label = match status {
            Status::Ok => self.style.green(&format!("{:<8}", "ok")),
            Status::Note => self.style.dim(&format!("{:<8}", "note")),
            Status::Warning => self.style.yellow(&format!("{:<8}", "warning")),
            Status::Problem => {
                self.problems += 1;
                self.style.red(&format!("{:<8}", "problem"))
            }
        };
        println!("  {label} {message}");
    }

    fn setting(&self, name: &str, value: impl std::fmt::Display) {
        println!("  {} {value}", self.style.dim(&format!("{name:<12}")));
    }
}

pub fn run(
    settings: &Settings,
    library: Option<PathBuf>,
    style: Style,
    console: &Console,
) -> miette::Result<()> {
    let mut checkup = Checkup { style, problems: 0 };
    check_tools(&mut checkup);

    checkup.section("settings");
    checkup.setting(
        "config file",
        settings
            .file
            .as_ref()
            .map_or_else(|| String::from("none"), |path| path.display().to_string()),
    );
    let library = settings.library(library).ok();
    checkup.setting(
        "library",
        library.as_ref().map_or_else(
            || String::from("none given"),
            |path| path.display().to_string(),
        ),
    );
    checkup.setting("peak store", settings.peaks_dir.display());
    checkup.setting("jobs", settings.jobs);
    checkup.setting("playback", settings.playback.name());
    checkup.setting("track length", settings.track_length);

    match library {
        Some(library) => check_library(
            &mut checkup,
            &library,
            &settings.peaks_dir,
            settings.track_length,
            settings.jobs,
            console,
        ),
        None => {
            checkup.section("library");
            check_store_alone(
                &mut checkup,
                Status::Note,
                "no library given (--library, or `library` in the configuration file)",
                None,
                &settings.peaks_dir,
                settings.track_length,
                settings.jobs,
            );
        }
    }

    if checkup.problems > 0 {
        return Err(miette!(
            "doctor found {} problem{}",
            checkup.problems,
            if checkup.problems == 1 { "" } else { "s" }
        ));
    }
    Ok(())
}

fn check_tools(checkup: &mut Checkup) {
    checkup.section("tools");
    for (tool, needed_for) in [
        ("ffmpeg", "index and identify"),
        ("ffprobe", "index, and track names in tracklists"),
        ("ffplay", "listen"),
    ] {
        match version(tool) {
            Some(version) => checkup.line(Status::Ok, format!("{tool} {version}")),
            None if tool == "ffplay" => checkup.line(
                Status::Warning,
                format!("{tool} not found; needed for {needed_for} only"),
            ),
            None => checkup.line(
                Status::Problem,
                format!(
                    "{tool} not found; needed for {needed_for}. Install FFmpeg (for example `brew install ffmpeg`)"
                ),
            ),
        }
    }
}

/// The version word of `<tool> -version`'s first line.
fn version(tool: &str) -> Option<String> {
    let output = Command::new(tool)
        .arg("-version")
        .stdin(Stdio::null())
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&output.stdout);
    let first = text.lines().next()?;
    Some(first.split_whitespace().nth(2).unwrap_or(first).to_owned())
}

/// Without a library to read, `identify`, `explain` and `stats` search the
/// store's own records: a problem only when it holds none.
fn check_store_alone(
    checkup: &mut Checkup,
    status: Status,
    reason: &str,
    root: Option<&Path>,
    peaks_dir: &Path,
    length: TrackLength,
    jobs: usize,
) {
    let (store, (sources, unreadable)) = match store_records(peaks_dir, jobs) {
        Ok(Some(found)) => found,
        Ok(None) => {
            checkup.line(
                Status::Problem,
                format!("{reason}, and the peak store holds no current records"),
            );
            return;
        }
        Err(error) => {
            checkup.line(Status::Problem, format!("{reason}; {error}"));
            return;
        }
    };
    checkup.line(
        status,
        format!(
            "{reason}; identify, explain and stats search the {} records in the peak store",
            sources.len()
        ),
    );
    checkup.section("peak store");
    if let Some(root) = root
        && let Err(error) = store.check_library(root)
    {
        checkup.line(Status::Problem, error);
        return;
    }
    match store.library() {
        Ok(Some(name)) => checkup.line(Status::Ok, format!("it names the library at {name}")),
        _ => checkup.line(
            Status::Note,
            "it does not name its library, so reports cannot say where to listen to the tracks",
        ),
    }
    let bytes: u64 = sources
        .iter()
        .filter_map(|source| store.record_bytes(&source.path).ok())
        .sum();
    checkup.line(
        Status::Ok,
        format!(
            "{} current records, {:.1} MB on disk",
            sources.len(),
            bytes as f64 / 1e6
        ),
    );
    let untagged = sources
        .iter()
        .filter(|source| store.tags(source).is_none())
        .count();
    if untagged > 0 {
        checkup.line(
            Status::Note,
            format!(
                "{untagged} of {} current records have no stored tags, so tracklists name them by file name",
                sources.len()
            ),
        );
    }
    if !unreadable.is_empty() {
        checkup.line(
            Status::Problem,
            format!(
                "{} records cannot be read; `gunfinger index` with the library writes them again",
                unreadable.len()
            ),
        );
    }
    let plan = plan_index(
        &sources,
        &store,
        &Profile::CURRENT,
        &BTreeSet::new(),
        length,
        jobs,
    );
    check_saved_index(
        checkup,
        &store,
        plan.sources,
        Searched::StoreRecords,
        length,
        jobs,
    );
}

/// Whether the library has an ignore file, and how many audio files each
/// pattern leaves out. One that leaves out none is most likely a typo or a
/// folder renamed since.
fn check_ignore_file(checkup: &mut Checkup, library: &Library) {
    let Some(file) = &library.ignore_file else {
        checkup.line(
            Status::Note,
            format!("no {IGNORE_FILE} at the library root; no file is left out"),
        );
        return;
    };
    checkup.line(
        Status::Ok,
        format!(
            "read {}: {}, {} left out",
            file.display(),
            counted(library.ignored.len(), "pattern"),
            counted(library.ignored_total(), "audio file")
        ),
    );
    for ignored in &library.ignored {
        let pattern = &ignored.pattern;
        if ignored.paths.is_empty() {
            checkup.line(
                Status::Warning,
                format!(
                    "line {}, `{}`, leaves out no audio file: check it for a typo or a renamed folder, or whether an earlier pattern already leaves out its files",
                    pattern.line, pattern.text
                ),
            );
        } else {
            checkup.line(
                Status::Ok,
                format!(
                    "line {}, `{}`, leaves out {}",
                    pattern.line,
                    pattern.text,
                    counted(ignored.paths.len(), "audio file")
                ),
            );
        }
    }
}

fn check_library(
    checkup: &mut Checkup,
    root: &Path,
    peaks_dir: &Path,
    length: TrackLength,
    jobs: usize,
    console: &Console,
) {
    checkup.section("library");
    let library = match list(root, console) {
        Ok(library) => library,
        Err(ScanError::Ignore(error)) => {
            checkup.line(Status::Problem, error);
            return;
        }
        Err(ScanError::Io(error)) => {
            check_store_alone(
                checkup,
                Status::Warning,
                &format!("could not read the library at {}: {error}", root.display()),
                Some(root),
                peaks_dir,
                length,
                jobs,
            );
            return;
        }
    };
    let status = if library.assets.is_empty() {
        Status::Problem
    } else {
        Status::Ok
    };
    checkup.line(
        status,
        format!(
            "{} audio files, {} other files and folders passed over",
            library.assets.len(),
            library.skipped_total()
        ),
    );
    if !library.vanished.is_empty() {
        checkup.line(
            Status::Note,
            format!(
                "these counts leave out what vanished while the library was listed (moved or deleted meanwhile, by a download for example): {}",
                vanished_entries(&library)
            ),
        );
    }
    check_ignore_file(checkup, &library);

    checkup.section("peak store");
    let profile = Profile::CURRENT;
    let store = match PeakStore::open(peaks_dir) {
        Ok(store) => store,
        Err(error) => {
            checkup.line(Status::Problem, error);
            return;
        }
    };
    match store.check_library(root) {
        Err(error) => {
            checkup.line(Status::Problem, error);
            return;
        }
        Ok(()) if matches!(store.library(), Ok(Some(_))) => {
            checkup.line(Status::Ok, "it names this library as its own");
        }
        Ok(()) => checkup.line(
            Status::Note,
            "it does not name its library yet; the next `gunfinger index` names this one",
        ),
    }
    let survey = match survey(&library, &store, &profile) {
        Ok(survey) => survey,
        Err(error) => {
            checkup.line(Status::Problem, error);
            return;
        }
    };
    checkup.line(
        Status::Ok,
        format!(
            "{} current records, {:.1} MB on disk",
            survey.current,
            survey.bytes as f64 / 1e6
        ),
    );
    if survey.tagged < survey.current {
        checkup.line(
            Status::Note,
            format!(
                "{} of {} current records have no stored tags; `gunfinger index` reads them from the files",
                survey.current - survey.tagged,
                survey.current
            ),
        );
    }
    let passed_over = survey.failed + survey.too_short + survey.too_long;
    if passed_over > 0 {
        checkup.line(
            Status::Note,
            format!(
                "{passed_over} files passed over: {} failed to decode, {} too short, {} too long (`index --retry-skipped` tries again)",
                survey.failed, survey.too_short, survey.too_long
            ),
        );
    }
    if survey.unindexed > 0 {
        checkup.line(
            Status::Warning,
            format!(
                "{} audio files are not indexed yet: run `gunfinger index`",
                survey.unindexed
            ),
        );
    }
    if survey.stale > 0 {
        checkup.line(
            Status::Warning,
            format!(
                "{} records are out of date (the file changed, or a new profile): run `gunfinger index`",
                survey.stale
            ),
        );
    }
    let gone = survey.orphans(Absence::Gone);
    if !gone.is_empty() {
        let advice = if library.vanished.is_empty() {
            "`gunfinger prune` lists them and `gunfinger prune --yes` deletes them"
        } else {
            "some may be in what vanished while the library was listed; `gunfinger prune` waits for a listing in which nothing vanishes"
        };
        checkup.line(
            Status::Warning,
            format!("{} gone from this library: {advice}", counted_kinds(&gone)),
        );
    }
    let ignored = survey.orphans(Absence::Ignored);
    if !ignored.is_empty() {
        let searched = if ignored
            .iter()
            .any(|orphan| matches!(orphan.stored, Stored::Record { .. }))
        {
            "; until then, searches with --store-only still use their peak records"
        } else {
            ""
        };
        checkup.line(
            Status::Warning,
            format!(
                "{} that {IGNORE_FILE} leaves out: `gunfinger prune` lists them and `gunfinger prune --yes` deletes them{searched}",
                counted_kinds(&ignored)
            ),
        );
    }
    if !survey.leftovers.is_empty() {
        checkup.line(
            Status::Note,
            format!(
                "{} temporary files left by interrupted runs: `gunfinger prune` removes them",
                survey.leftovers.len()
            ),
        );
    }
    for stored in &survey.unreadable {
        checkup.line(
            Status::Problem,
            format!(
                "{} is not a peak record this version can read; move it out of the peak store",
                stored.file().display()
            ),
        );
    }

    checkup.section("index limits");
    let within: Vec<Duration> = survey
        .current_lengths
        .iter()
        .copied()
        .filter(|&track| length.admits(track))
        .collect();
    // Files without a current record (not indexed yet, or changed since)
    // are counted: `index` will most likely add them.
    let indexed = within.len() + survey.unindexed;
    let share = indexed as f64 / MAX_ASSETS as f64;
    checkup.line(
        if share > 0.8 {
            Status::Warning
        } else {
            Status::Ok
        },
        format!(
            "{indexed} of {MAX_ASSETS} assets ({:.1}%); beyond that see docs/adr/0010",
            share * 100.0
        ),
    );
    let left_out = library.assets.len() - indexed;
    if left_out > 0 {
        checkup.line(
            Status::Note,
            format!(
                "{left_out} audio files not counted: their length is outside the track length range ({length}), or `index` passed over them"
            ),
        );
    }
    let longest = within
        .iter()
        .max()
        .copied()
        .unwrap_or_default()
        .as_secs_f64();
    let addressable = addressable_length().as_secs_f64();
    checkup.line(
        if longest > addressable {
            Status::Problem
        } else {
            Status::Ok
        },
        format!(
            "longest track {}; the index addresses {}",
            timecode(longest),
            timecode(addressable)
        ),
    );

    let searched = plan_index(
        &library.assets,
        &store,
        &Profile::CURRENT,
        &BTreeSet::new(),
        length,
        jobs,
    );
    check_saved_index(
        checkup,
        &store,
        searched.sources,
        Searched::Library,
        length,
        jobs,
    );
}

/// What the saved index is checked against: a search of the library's
/// files, or of the store's own records (`--store-only`, or no library).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Searched {
    Library,
    StoreRecords,
}

/// Whether the saved index of the peak store (ADR 0011) is current for a
/// search of `sources`, from its header and length only, and what else the
/// index folder holds. When it is out of date for the library, it may still
/// be current for searches of the store's own records. Reports only;
/// deletes nothing.
fn check_saved_index(
    checkup: &mut Checkup,
    store: &PeakStore,
    sources: Vec<Asset>,
    searched: Searched,
    length: TrackLength,
    jobs: usize,
) {
    checkup.section("saved index");
    let Some(dir) = config::index_dir() else {
        checkup.line(
            Status::Note,
            "no home folder to keep a saved index in (set XDG_CACHE_HOME or HOME); searches build the index each time",
        );
        return;
    };
    let name = saved::file_name(&absolute(store.dir()));
    let path = dir.join(&name);
    let provenance = |sources: Vec<Asset>| Provenance {
        profile: Profile::CURRENT.id(),
        hash_design: hash::design(),
        track_length: length,
        sources,
    };
    let size = fs::metadata(&path).map_or(0, |metadata| metadata.len());
    let file = format!("{}, {}", path.display(), bytes(size));
    match saved::examine(&path, &provenance(sources)) {
        Ok(()) => checkup.line(
            Status::Ok,
            format!(
                "{file}: current by its header and length; searches load it, checking the rest as they read it"
            ),
        ),
        Err(Unusable::Missing) => checkup.line(
            Status::Note,
            format!(
                "none yet; the next identify, explain or stats builds the index and saves it as {}",
                path.display()
            ),
        ),
        Err(Unusable::Stale(why)) => {
            let current_for_store = searched == Searched::Library
                && store
                    .current_sources(&Profile::CURRENT, jobs)
                    .is_ok_and(|(sources, _)| {
                        let plan = plan_index(
                            &sources,
                            store,
                            &Profile::CURRENT,
                            &BTreeSet::new(),
                            length,
                            jobs,
                        );
                        saved::examine(&path, &provenance(plan.sources)).is_ok()
                    });
            checkup.line(
                Status::Note,
                if current_for_store {
                    format!(
                        "{file}: current for searches with --store-only, out of date for the library ({why}); a search of the library builds the index again and saves it over the file"
                    )
                } else {
                    format!(
                        "{file}: out of date ({why}); the next search builds the index again and saves it over the file"
                    )
                },
            );
        }
        Err(Unusable::Unreadable(why)) => checkup.line(
            Status::Warning,
            format!(
                "{file}: cannot be read ({why}); the next search builds the index again and saves it over the file"
            ),
        ),
    }
    check_other_saved_files(checkup, &dir, &name);
}

/// The other files of the index folder: saved indexes no peak store of
/// these settings uses (another store's, or this one's at another path)
/// and unfinished saves.
fn check_other_saved_files(checkup: &mut Checkup, dir: &Path, own: &str) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut others: Vec<(String, u64)> = entries
        .flatten()
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let name = entry.file_name().to_string_lossy().into_owned();
            (metadata.is_file() && name != own).then_some((name, metadata.len()))
        })
        .collect();
    if others.is_empty() {
        return;
    }
    others.sort();
    checkup.line(
        Status::Note,
        format!(
            "{} in {}, {} in all; doctor deletes nothing",
            counted(others.len(), "other file"),
            dir.display(),
            bytes(others.iter().map(|(_, size)| size).sum())
        ),
    );
    for (name, size) in &others {
        let what = if name.ends_with(".partial") && name.starts_with(&format!("{own}.")) {
            "an unfinished save of this store's index; the next search deletes it once it is 10 minutes old"
        } else if name.ends_with(".partial") {
            "an unfinished save of another store's index; the next search of that store deletes it once it is 10 minutes old"
        } else if name.ends_with(".index") {
            "belongs to no peak store of these settings: the saved index of another store, named after its folder, or of this store at another path; delete it if that store is gone"
        } else {
            "not a saved index"
        };
        checkup.line(Status::Note, format!("{name}, {}: {what}", bytes(*size)));
    }
}

/// A file size for people: megabytes, or gigabytes from 1 GB.
fn bytes(size: u64) -> String {
    let size = size as f64;
    if size >= 1e9 {
        format!("{:.1} GB", size / 1e9)
    } else {
        format!("{:.1} MB", size / 1e6)
    }
}

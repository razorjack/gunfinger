//! `gunfinger doctor`: check the tools, the settings, the library and the
//! peak store, and say what to do about each problem.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use gunfinger_core::index::{MAX_ASSETS, addressable_length};
use gunfinger_core::indexing::TrackLength;
use gunfinger_core::library::ignore::IGNORE_FILE;
use gunfinger_core::library::{Library, ScanError};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, Stored};
use miette::miette;

use crate::catalog::{list, store_records};
use crate::config::Settings;
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
) {
    let (store, (sources, unreadable)) = match store_records(peaks_dir) {
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
        checkup.line(
            Status::Warning,
            format!(
                "{} gone from this library: `gunfinger prune` lists them and `gunfinger prune --yes` deletes them",
                counted_kinds(&gone)
            ),
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
}

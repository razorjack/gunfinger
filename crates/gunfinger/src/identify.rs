//! `gunfinger identify`: find library tracks inside recordings. Several
//! recordings share one index build; `--save-dir` keeps a JSON report of
//! each, and recordings already reported there with the same settings are
//! passed over.

use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

use gunfinger_core::decode::{Audio, Excerpt, decode};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Matcher, search_with_progress};
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::catalog::{Catalog, Indexable, Source, absolute};
use crate::console::Console;
use crate::output::{ReportFormat, render};
use crate::playback::PlaybackChoice;
use crate::report::{QuerySource, Report, SearchSettings};
use crate::style::Style;

pub struct Request<'a> {
    pub audio: &'a [PathBuf],
    pub source: Source<'a>,
    pub excerpt: Excerpt,
    pub playback: PlaybackChoice,
    pub matcher: Matcher,
    pub format: ReportFormat,
    /// Where to write each recording's JSON report, named after it.
    pub save_dir: Option<&'a Path>,
    /// Search recordings again that already have a report in `save_dir`.
    pub again: bool,
    pub style: Style,
    pub jobs: usize,
    pub console: &'a Console,
}

pub fn run(request: &Request) -> miette::Result<()> {
    let several = request.audio.len() > 1;
    if several && !request.format.is_for_people() {
        return Err(miette!(
            help = "--save-dir keeps one JSON report per recording; `gunfinger show` renders each in any format",
            "several recordings can only be printed in the tracklist, human or timeline format"
        ));
    }
    // Checked first: listing the library and building the index take
    // seconds, and FFmpeg's own message for a missing file is hard to read.
    for audio in request.audio {
        File::open(audio)
            .into_diagnostic()
            .wrap_err_with(|| format!("cannot read {}", audio.display()))?;
    }
    let indexable = Indexable::find(&request.source, request.console)?;
    let jobs = plan(request, &indexable)?;
    if jobs.is_empty() {
        request
            .console
            .info("every recording already has a report (--again searches them anyway)");
        return Ok(());
    }
    let (catalog, first) = open_while_decoding(request, indexable, &jobs[0].0);
    let catalog = catalog?;
    let mut first = Some(first);
    let mut failed = 0;
    for (audio, saved) in &jobs {
        if several {
            println!("{}", request.style.bold(&format!("== {}", audio.display())));
        }
        let decoded = first
            .take()
            .unwrap_or_else(|| decode_recording(request, audio));
        match decoded.map(|decoded| identify(request, &catalog, audio, decoded)) {
            Ok(report) => {
                if let Some(saved) = saved {
                    let json = render(&report, ReportFormat::Json, request.style)?;
                    write_atomically(saved, &json)
                        .into_diagnostic()
                        .wrap_err_with(|| format!("cannot write {}", saved.display()))?;
                }
                print!("{}", render(&report, request.format, request.style)?);
            }
            Err(error) if several => {
                failed += 1;
                request.console.error(format_args!("{error:?}"));
            }
            Err(error) => return Err(error),
        }
        if several {
            println!();
        }
    }
    if failed > 0 {
        return Err(miette!(
            "{failed} of {} recordings could not be searched",
            jobs.len()
        ));
    }
    Ok(())
}

/// Each recording with the report file it gets, leaving out those already
/// reported with the same settings. Two recordings with the same name would
/// share a report file.
fn plan(
    request: &Request,
    indexable: &Indexable,
) -> miette::Result<Vec<(PathBuf, Option<PathBuf>)>> {
    let Some(dir) = request.save_dir else {
        return Ok(request
            .audio
            .iter()
            .map(|audio| (audio.clone(), None))
            .collect());
    };
    fs::create_dir_all(dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot create {}", dir.display()))?;
    // Reading the record headers is enough for the library revision; the
    // index itself is built only if something needs searching.
    let any_saved = request
        .audio
        .iter()
        .any(|audio| report_path(dir, audio).exists());
    let current = if any_saved && !request.again {
        Some(SearchSettings::current(
            request.matcher,
            &indexable.revision(),
        ))
    } else {
        None
    };
    let mut seen: BTreeMap<PathBuf, &PathBuf> = BTreeMap::new();
    let mut jobs = Vec::new();
    for audio in request.audio {
        let report = report_path(dir, audio);
        check_report_owner(&report, audio)?;
        if let Some(other) = seen.insert(report.clone(), audio) {
            return Err(miette!(
                "{} and {} would both be reported in {}",
                other.display(),
                audio.display(),
                report.display()
            ));
        }
        let search = match &current {
            Some(settings) if report.exists() => {
                match outdated(request, audio, &report, settings) {
                    Some(reason) => {
                        request.console.info(format_args!(
                            "searching {} again: its report {reason}",
                            audio.display()
                        ));
                        true
                    }
                    None => false,
                }
            }
            _ => true,
        };
        if search {
            jobs.push((audio.clone(), Some(report)));
        }
    }
    Ok(jobs)
}

/// A forced rerun replaces only this recording's report.
fn check_report_owner(path: &Path, audio: &Path) -> miette::Result<()> {
    if let Ok(report) = Report::read(path)
        && report.query.path != absolute(audio)
    {
        return Err(miette!(
            help =
                "use another --save-dir or rename this recording to give it a distinct report name",
            "{} already reports {}; refusing to overwrite it with {}",
            path.display(),
            report.query.path.display(),
            audio.display()
        ));
    }
    Ok(())
}

/// Why the saved report at `path` does not answer this request, if it does
/// not: it cannot be read, or another recording, part of it, playback or
/// search setting made it.
fn outdated(
    request: &Request,
    audio: &Path,
    path: &Path,
    settings: &SearchSettings,
) -> Option<String> {
    let Ok(report) = Report::read(path) else {
        return Some(String::from("cannot be read"));
    };
    let query = &report.query;
    if query.path != absolute(audio) {
        return Some(format!("is of {}", query.path.display()));
    }
    if query.source.is_none() || query.source != QuerySource::read(audio).ok() {
        return Some(String::from(
            "does not match the recording's current size and modification time",
        ));
    }
    let start = request.excerpt.start.unwrap_or_default().as_secs_f64();
    let duration = request
        .excerpt
        .duration
        .map(|duration| duration.as_secs_f64());
    if query.start_seconds != start || query.requested_duration_seconds != duration {
        return Some(String::from("covers another part of the recording"));
    }
    match query.playback {
        Some(playback) if playback == request.playback => {}
        Some(playback) => {
            return Some(format!(
                "was searched with `--playback {}`",
                playback.name()
            ));
        }
        None => return Some(String::from("does not record its playback")),
    }
    match &report.search {
        Some(saved) => saved
            .difference(settings)
            .map(|difference| format!("was made with {difference}")),
        None => Some(String::from("does not record its search settings")),
    }
}

/// Writes through a temporary file and a rename, so an interrupted run
/// leaves the previous report under the final name, never a truncated one.
fn write_atomically(path: &Path, text: &str) -> io::Result<()> {
    let temporary = path.with_extension(format!("tmp{}", std::process::id()));
    let written = write_synced(&temporary, text).and_then(|()| fs::rename(&temporary, path));
    if written.is_err() {
        // Only clutter would be left behind.
        let _ = fs::remove_file(&temporary);
    }
    written
}

fn write_synced(path: &Path, text: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(text.as_bytes())?;
    file.sync_all()
}

fn report_path(dir: &Path, audio: &Path) -> PathBuf {
    let stem = audio.file_stem().map_or_else(
        || String::from("recording"),
        |stem| stem.to_string_lossy().into_owned(),
    );
    dir.join(format!("{stem}.json"))
}

/// Opens the catalog and decodes the first recording at the same time:
/// loading the saved index waits on the disk and decoding on FFmpeg, about
/// 3.5 s and 2.6 s for the 2003 mix at NAS scale (experiment 0098). If no
/// thread can be started, the recording is decoded afterwards. Both end
/// before either result is used, so a catalog error waits for the decode.
fn open_while_decoding(
    request: &Request,
    indexable: Indexable,
    audio: &Path,
) -> (miette::Result<Catalog>, miette::Result<Decoded>) {
    thread::scope(|scope| {
        let decoding =
            thread::Builder::new().spawn_scoped(scope, || decode_recording(request, audio));
        let catalog = Catalog::open(indexable, request.matcher, request.console);
        let decoded = match decoding {
            Ok(decoding) => decoding
                .join()
                .unwrap_or_else(|payload| std::panic::resume_unwind(payload)),
            Err(_) => decode_recording(request, audio),
        };
        (catalog, decoded)
    })
}

/// A recording's audio, with the file's identity and how long decoding took.
struct Decoded {
    source: QuerySource,
    audio: Audio,
    took: Duration,
}

fn decode_recording(request: &Request, audio_path: &Path) -> miette::Result<Decoded> {
    let started = Instant::now();
    let source = QuerySource::read(audio_path).into_diagnostic()?;
    let audio =
        decode(audio_path, Profile::CURRENT.sample_rate, request.excerpt).into_diagnostic()?;
    if source != QuerySource::read(audio_path).into_diagnostic()? {
        return Err(miette!(
            "{} changed while being decoded; search it again once the file is stable",
            audio_path.display()
        ));
    }
    Ok(Decoded {
        source,
        audio,
        took: started.elapsed(),
    })
}

fn identify(request: &Request, catalog: &Catalog, audio_path: &Path, decoded: Decoded) -> Report {
    let profile = Profile::CURRENT;
    let started = Instant::now();
    let Decoded {
        source,
        audio,
        took: decoding,
    } = decoded;
    let ladder = request.playback.rungs();
    let detections = search_with_progress(
        &catalog.index,
        &audio.samples,
        &profile,
        &ladder,
        request.jobs,
        catalog.matcher.options(),
        |done, parts| {
            request
                .console
                .progress(format_args!("searching: {}%", 100 * done / parts));
        },
    );
    request.console.progress_done();
    request.console.detail(format_args!(
        "searched {} of audio in {:.1} s (decoding {:.1} s)",
        format_timecode(audio.duration()),
        (decoding + started.elapsed()).as_secs_f64(),
        decoding.as_secs_f64()
    ));
    let mut report = Report::new(
        catalog,
        &absolute(audio_path),
        request.excerpt,
        audio.duration(),
        request.playback,
        &detections,
    );
    report.query.source = Some(source);
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::SavedIndex;
    use crate::console::Verbosity;
    use crate::style::ColorChoice;
    use gunfinger_core::indexing::TrackLength;
    use std::time::Duration;

    #[test]
    fn saved_reports_belong_to_one_recording_even_across_runs() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/tests/report-owner");
        fs::create_dir_all(&dir).unwrap();
        let audio = dir.join("first/set.wav");
        let other = dir.join("second/set.wav");
        let saved = report_path(&dir, &audio);
        assert_eq!(saved, report_path(&dir, &other));
        let mut report = crate::table::tests::report();
        report.query.path = absolute(&audio);
        let original = serde_json::to_string(&report).unwrap();
        fs::write(&saved, &original).unwrap();
        assert!(check_report_owner(&saved, &audio).is_ok());
        assert!(check_report_owner(&saved, &other).is_err());
        assert_eq!(fs::read_to_string(&saved).unwrap(), original);
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn replacing_query_audio_invalidates_a_saved_report() {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/tests/query-identity");
        fs::create_dir_all(&dir).unwrap();
        let audio = dir.join("mix.wav");
        let saved = dir.join("mix.json");
        fs::write(&audio, b"original audio").unwrap();
        let console = Console::new(ColorChoice::Never, Verbosity::Quiet);
        let request = Request {
            audio: &[],
            source: Source {
                library: None,
                peaks_dir: &dir,
                store_only: true,
                exclude_from: None,
                track_length: TrackLength {
                    min: Duration::ZERO,
                    max: Duration::from_secs(1000),
                },
                saved_index: SavedIndex::Off,
                jobs: 2,
            },
            excerpt: Excerpt::default(),
            playback: PlaybackChoice::Both,
            matcher: Matcher::default(),
            format: ReportFormat::Json,
            save_dir: None,
            again: false,
            style: Style::for_stdout(ColorChoice::Never),
            jobs: 1,
            console: &console,
        };
        let settings = SearchSettings::current(request.matcher, "library");
        let mut report = crate::table::tests::report();
        report.query.path = absolute(&audio);
        report.query.source = Some(QuerySource::read(&audio).unwrap());
        report.search = Some(settings.clone());
        fs::write(&saved, serde_json::to_string(&report).unwrap()).unwrap();
        assert!(outdated(&request, &audio, &saved, &settings).is_none());
        fs::write(&audio, b"replacement audio of another length").unwrap();
        assert!(
            outdated(&request, &audio, &saved, &settings)
                .unwrap()
                .contains("size and modification time")
        );
        report.query.source = None;
        fs::write(&saved, serde_json::to_string(&report).unwrap()).unwrap();
        assert!(outdated(&request, &audio, &saved, &settings).is_some());
        fs::remove_dir_all(dir).unwrap();
    }
}

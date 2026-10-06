//! `gunfinger identify`: find library tracks inside recordings. Several
//! recordings share one index build; `--save-dir` keeps a JSON report of
//! each, and recordings already reported there are passed over.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::search_with_progress;
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr, miette};

use crate::catalog::{Catalog, absolute};
use crate::console::Console;
use crate::output::{ReportFormat, render};
use crate::playback::PlaybackChoice;
use crate::report::Report;
use crate::style::Style;

pub struct Request<'a> {
    pub audio: &'a [PathBuf],
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub excerpt: Excerpt,
    pub playback: PlaybackChoice,
    pub exclude_from: Option<&'a Path>,
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
            "several recordings can only be printed in the human, timeline or tracklist format"
        ));
    }
    let jobs = plan(request)?;
    if jobs.is_empty() {
        request
            .console
            .info("every recording already has a report (--again searches them anyway)");
        return Ok(());
    }
    let catalog = Catalog::open(
        request.library,
        request.peaks_dir,
        request.exclude_from,
        request.console,
    )?;
    let mut failed = 0;
    for (audio, saved) in &jobs {
        if several {
            println!("{}", request.style.bold(&format!("== {}", audio.display())));
        }
        match identify(request, &catalog, audio) {
            Ok(report) => {
                if let Some(saved) = saved {
                    let json = render(&report, ReportFormat::Json, request.style)?;
                    std::fs::write(saved, json)
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
/// reported. Two recordings with the same name would share a report file.
fn plan(request: &Request) -> miette::Result<Vec<(PathBuf, Option<PathBuf>)>> {
    // Checked first: building the index takes seconds, and FFmpeg's own
    // message for a missing file is hard to read.
    for audio in request.audio {
        std::fs::File::open(audio)
            .into_diagnostic()
            .wrap_err_with(|| format!("cannot read {}", audio.display()))?;
    }
    let Some(dir) = request.save_dir else {
        return Ok(request
            .audio
            .iter()
            .map(|audio| (audio.clone(), None))
            .collect());
    };
    std::fs::create_dir_all(dir)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot create {}", dir.display()))?;
    let mut seen: BTreeMap<PathBuf, &PathBuf> = BTreeMap::new();
    let mut jobs = Vec::new();
    for audio in request.audio {
        let report = report_path(dir, audio);
        if let Some(other) = seen.insert(report.clone(), audio) {
            return Err(miette!(
                "{} and {} would both be reported in {}",
                other.display(),
                audio.display(),
                report.display()
            ));
        }
        if request.again || !report.exists() {
            jobs.push((audio.clone(), Some(report)));
        }
    }
    Ok(jobs)
}

fn report_path(dir: &Path, audio: &Path) -> PathBuf {
    let stem = audio.file_stem().map_or_else(
        || String::from("recording"),
        |stem| stem.to_string_lossy().into_owned(),
    );
    dir.join(format!("{stem}.json"))
}

fn identify(request: &Request, catalog: &Catalog, audio_path: &Path) -> miette::Result<Report> {
    let profile = Profile::CURRENT;
    let started = Instant::now();
    let audio = decode(audio_path, profile.sample_rate, request.excerpt).into_diagnostic()?;
    let decoded = started.elapsed();
    let ladder = request.playback.rungs();
    let detections = search_with_progress(
        &catalog.index,
        &audio.samples,
        &profile,
        &ladder,
        request.jobs,
        |done| {
            request
                .console
                .progress(format_args!("searching: {done} of {} rungs", ladder.len()));
        },
    );
    request.console.progress_done();
    request.console.info(format_args!(
        "searched {} of audio in {:.1} s (decoding {:.1} s)",
        format_timecode(audio.duration()),
        started.elapsed().as_secs_f64(),
        decoded.as_secs_f64()
    ));
    let offset = request.excerpt.start.unwrap_or_default().as_secs_f64();
    Ok(Report::new(
        catalog,
        &absolute(audio_path),
        offset,
        audio.duration(),
        &detections,
    ))
}

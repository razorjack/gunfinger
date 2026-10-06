//! `gunfinger identify`: find library tracks inside a recording.

use std::path::Path;
use std::time::Instant;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::search;
use gunfinger_core::speed::ladder;
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr};

use crate::catalog::{Catalog, absolute};
use crate::console::Console;
use crate::output::{ReportFormat, render};
use crate::report::Report;
use crate::style::Style;

pub struct Request<'a> {
    pub audio: &'a Path,
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub excerpt: Excerpt,
    pub exclude_from: Option<&'a Path>,
    pub format: ReportFormat,
    pub style: Style,
    pub jobs: usize,
    pub console: &'a Console,
}

pub fn run(request: &Request) -> miette::Result<()> {
    // Checked first: building the index takes seconds, and FFmpeg's own
    // message for a missing file is hard to read.
    std::fs::File::open(request.audio)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot read {}", request.audio.display()))?;
    let catalog = Catalog::open(
        request.library,
        request.peaks_dir,
        request.exclude_from,
        request.console,
    )?;
    let profile = Profile::CURRENT;
    let started = Instant::now();
    let audio = decode(request.audio, profile.sample_rate, request.excerpt).into_diagnostic()?;
    let decoded = started.elapsed();
    let detections = search(
        &catalog.index,
        &audio.samples,
        &profile,
        &ladder(),
        request.jobs,
    );
    request.console.info(format_args!(
        "searched {} of audio in {:.1} s (decoding {:.1} s)",
        format_timecode(audio.duration()),
        started.elapsed().as_secs_f64(),
        decoded.as_secs_f64()
    ));

    let offset = request.excerpt.start.unwrap_or_default().as_secs_f64();
    let report = Report::new(
        &catalog,
        &absolute(request.audio),
        offset,
        audio.duration(),
        &detections,
    );
    print!("{}", render(&report, request.format, request.style)?);
    Ok(())
}

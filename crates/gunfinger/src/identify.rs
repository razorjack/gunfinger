//! `gunfinger identify`: find library tracks inside a recording.

use std::path::Path;
use std::time::{Duration, Instant};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search};
use gunfinger_core::speed::ladder;
use gunfinger_core::timecode::format_timecode;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::Format;
use crate::catalog::Catalog;

/// Sub-threshold candidates shown after the detections.
const CANDIDATES_SHOWN: usize = 10;
/// Candidates seen in a single window are almost always chance.
const MIN_CANDIDATE_WINDOWS: u32 = 2;

pub struct Request<'a> {
    pub audio: &'a Path,
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub excerpt: Excerpt,
    pub exclude_from: Option<&'a Path>,
    pub format: Format,
    pub jobs: usize,
}

pub fn run(request: &Request) -> miette::Result<()> {
    let catalog = Catalog::open(request.library, request.peaks_dir, request.exclude_from)?;
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
    eprintln!(
        "searched {} of audio in {:.1} s (decoding {:.1} s)",
        format_timecode(audio.duration()),
        started.elapsed().as_secs_f64(),
        decoded.as_secs_f64()
    );

    let offset = request.excerpt.start.unwrap_or_default().as_secs_f64();
    let report = Report::new(&catalog, request, offset, audio.duration(), &detections);
    match request.format {
        Format::Human => print_table(&report),
        Format::Json => println!(
            "{}",
            serde_json::to_string_pretty(&report).into_diagnostic()?
        ),
    }
    Ok(())
}

#[derive(Serialize)]
struct Report<'a> {
    schema_version: u32,
    query: Query<'a>,
    /// Confident detections, in order of start time.
    detections: Vec<Found<'a>>,
    /// The strongest sub-threshold candidates.
    candidates: Vec<Found<'a>>,
}

#[derive(Serialize)]
struct Query<'a> {
    path: &'a Path,
    start_seconds: f64,
    duration_seconds: f64,
}

#[derive(Serialize)]
struct Found<'a> {
    asset: &'a str,
    start_seconds: f64,
    end_seconds: f64,
    speed: f64,
    windows: u32,
    hits: u32,
    confident: bool,
}

impl<'a> Report<'a> {
    fn new(
        catalog: &'a Catalog,
        request: &'a Request,
        offset: f64,
        duration: Duration,
        detections: &[Detection],
    ) -> Report<'a> {
        let found = |detection: &Detection| Found {
            asset: &catalog.index.asset(detection.asset).path,
            start_seconds: offset + detection.start_seconds,
            end_seconds: offset + detection.end_seconds,
            speed: detection.speed.0,
            windows: detection.evidence.windows,
            hits: detection.evidence.hits,
            confident: detection.evidence.is_confident(),
        };
        let mut confident: Vec<Found> = detections
            .iter()
            .filter(|detection| detection.evidence.is_confident())
            .map(found)
            .collect();
        confident.sort_by(|a, b| a.start_seconds.total_cmp(&b.start_seconds));
        let candidates = detections
            .iter()
            .filter(|detection| {
                !detection.evidence.is_confident()
                    && detection.evidence.windows >= MIN_CANDIDATE_WINDOWS
            })
            .take(CANDIDATES_SHOWN)
            .map(found)
            .collect();
        Report {
            schema_version: 1,
            query: Query {
                path: request.audio,
                start_seconds: offset,
                duration_seconds: duration.as_secs_f64(),
            },
            detections: confident,
            candidates,
        }
    }
}

fn print_table(report: &Report) {
    println!(
        "{:<17} {:>7}  {:<10} {:>6}  asset",
        "time", "speed", "confidence", "hits"
    );
    for found in &report.detections {
        print_row(found);
    }
    if !report.candidates.is_empty() {
        println!("\nsub-threshold candidates:");
        for found in &report.candidates {
            print_row(found);
        }
    }
}

fn print_row(found: &Found) {
    let span = format!(
        "{}-{}",
        format_timecode(Duration::from_secs_f64(found.start_seconds)),
        format_timecode(Duration::from_secs_f64(found.end_seconds))
    );
    let confidence = if found.confident { "confident" } else { "low" };
    println!(
        "{span:<17} {:>+6.2}%  {confidence:<10} {:>6}  {}",
        (found.speed - 1.0) * 100.0,
        found.hits,
        found.asset
    );
}

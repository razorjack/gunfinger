//! `gunfinger identify`: find library tracks inside a recording.

use std::path::Path;
use std::time::{Duration, Instant};

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::plays::{self, Play};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search};
use gunfinger_core::speed::ladder;
use gunfinger_core::timecode::format_timecode;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::Format;
use crate::catalog::Catalog;

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
    /// Every confident or possible play, in order of start time.
    plays: Vec<FoundPlay<'a>>,
}

#[derive(Serialize)]
struct Query<'a> {
    path: &'a Path,
    start_seconds: f64,
    duration_seconds: f64,
}

#[derive(Serialize)]
struct FoundPlay<'a> {
    asset: &'a str,
    start_seconds: f64,
    end_seconds: f64,
    /// Where the first segment starts and the last ends in the track.
    track_start_seconds: f64,
    track_end_seconds: f64,
    /// The strongest segment's speed and confidence.
    speed: f64,
    confidence: &'static str,
    /// Summed over the segments.
    windows: u32,
    hits: u32,
    segments: Vec<Segment>,
}

#[derive(Serialize)]
struct Segment {
    start_seconds: f64,
    end_seconds: f64,
    track_start_seconds: f64,
    track_end_seconds: f64,
    speed: f64,
    confidence: &'static str,
    windows: u32,
    hits: u32,
}

impl<'a> Report<'a> {
    fn new(
        catalog: &'a Catalog,
        request: &'a Request,
        offset: f64,
        duration: Duration,
        detections: &[Detection],
    ) -> Report<'a> {
        let plays = plays::group(detections)
            .iter()
            .map(|play| FoundPlay::new(catalog, play, offset))
            .collect();
        Report {
            schema_version: 2,
            query: Query {
                path: request.audio,
                start_seconds: offset,
                duration_seconds: duration.as_secs_f64(),
            },
            plays,
        }
    }
}

impl<'a> FoundPlay<'a> {
    fn new(catalog: &'a Catalog, play: &Play, offset: f64) -> FoundPlay<'a> {
        let total = play.total_evidence();
        FoundPlay {
            asset: &catalog.index.asset(play.asset).path,
            start_seconds: offset + play.start_seconds(),
            end_seconds: offset + play.end_seconds(),
            track_start_seconds: play.track_start_seconds(),
            track_end_seconds: play.track_end_seconds(),
            speed: play.speed().0,
            confidence: label(play.confidence()),
            windows: total.windows,
            hits: total.hits,
            segments: play
                .segments()
                .iter()
                .map(|segment| Segment {
                    start_seconds: offset + segment.start_seconds,
                    end_seconds: offset + segment.end_seconds,
                    track_start_seconds: segment.track_start_seconds,
                    track_end_seconds: segment.track_end_seconds,
                    speed: segment.speed.0,
                    confidence: label(segment.evidence.confidence()),
                    windows: segment.evidence.windows,
                    hits: segment.evidence.hits,
                })
                .collect(),
        }
    }
}

fn label(confidence: Confidence) -> &'static str {
    match confidence {
        Confidence::Confident => "confident",
        Confidence::Possible => "possible",
        Confidence::Weak => "weak",
    }
}

/// One row per play; a play of several segments lists them underneath.
/// `in track` is the part of the track that was heard.
fn print_table(report: &Report) {
    println!(
        "{:<19} {:<13} {:>7}  {:<10} {:>6}  asset",
        "time", "in track", "speed", "confidence", "hits"
    );
    for play in &report.plays {
        print_row(
            &span(play.start_seconds, play.end_seconds),
            &span(play.track_start_seconds, play.track_end_seconds),
            play.speed,
            play.confidence,
            play.hits,
            play.asset,
        );
        if play.segments.len() > 1 {
            for segment in &play.segments {
                print_row(
                    &format!("  {}", span(segment.start_seconds, segment.end_seconds)),
                    &span(segment.track_start_seconds, segment.track_end_seconds),
                    segment.speed,
                    segment.confidence,
                    segment.hits,
                    "",
                );
            }
        }
    }
}

fn print_row(time: &str, track: &str, speed: f64, confidence: &str, hits: u32, asset: &str) {
    let row = format!(
        "{time:<19} {track:<13} {:>+6.2}%  {confidence:<10} {hits:>6}  {asset}",
        (speed - 1.0) * 100.0
    );
    println!("{}", row.trim_end());
}

fn span(start_seconds: f64, end_seconds: f64) -> String {
    format!(
        "{}-{}",
        format_timecode(Duration::from_secs_f64(start_seconds)),
        format_timecode(Duration::from_secs_f64(end_seconds))
    )
}

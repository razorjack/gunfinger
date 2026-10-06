//! `gunfinger identify`: find library tracks inside a recording.

use std::path::Path;
use std::time::{Duration, Instant};

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::plays::{self, Play, SameAudio};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search};
use gunfinger_core::speed::ladder;
use gunfinger_core::timecode::format_timecode;
use miette::{IntoDiagnostic, WrapErr};
use serde::Serialize;

use crate::Format;
use crate::catalog::Catalog;
use crate::console::Console;
use crate::style::Style;

pub struct Request<'a> {
    pub audio: &'a Path,
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub excerpt: Excerpt,
    pub exclude_from: Option<&'a Path>,
    pub format: Format,
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
    let report = Report::new(&catalog, request, offset, audio.duration(), &detections);
    match request.format {
        Format::Human => print!("{}", table(&report, request.style)),
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
    /// Every confident or possible play, in order of start time. Plays of
    /// assets with the same audio are one play.
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
    /// Other assets with exactly the same detections: copies of the file or
    /// rips with identical peaks.
    same_audio: Vec<&'a str>,
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
        let plays = plays::merge_same_audio(plays::group(detections))
            .iter()
            .map(|same| FoundPlay::new(catalog, same, offset))
            .collect();
        Report {
            schema_version: 3,
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
    fn new(catalog: &'a Catalog, same: &SameAudio, offset: f64) -> FoundPlay<'a> {
        let path = |asset| catalog.index.asset(asset).path.as_str();
        let play: &Play = &same.play;
        let total = play.total_evidence();
        FoundPlay {
            asset: path(play.asset),
            same_audio: same.also.iter().map(|&asset| path(asset)).collect(),
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

/// One row per play; assets with the same audio and a play's segments are
/// listed underneath. `in track` is the part of the track that was heard.
fn table(report: &Report, style: Style) -> String {
    let mut lines = vec![style.bold(&format!(
        "{:<19} {:<13} {:>7}  {:<10} {:>6}  asset",
        "time", "in track", "speed", "confidence", "hits"
    ))];
    for play in &report.plays {
        let confidence = format!("{:<10}", play.confidence);
        let confidence = match play.confidence {
            "confident" => style.green(&confidence),
            "possible" => style.yellow(&confidence),
            _ => confidence,
        };
        lines.push(row(
            &span(play.start_seconds, play.end_seconds),
            &span(play.track_start_seconds, play.track_end_seconds),
            play.speed,
            &confidence,
            play.hits,
            play.asset,
        ));
        for asset in &play.same_audio {
            lines.push(style.dim(&format!("{:62}also {asset}", "")));
        }
        if play.segments.len() > 1 {
            for segment in &play.segments {
                lines.push(style.dim(&row(
                    &format!("  {}", span(segment.start_seconds, segment.end_seconds)),
                    &span(segment.track_start_seconds, segment.track_end_seconds),
                    segment.speed,
                    &format!("{:<10}", segment.confidence),
                    segment.hits,
                    "",
                )));
            }
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

/// `confidence` comes padded to its column, possibly painted.
fn row(time: &str, track: &str, speed: f64, confidence: &str, hits: u32, asset: &str) -> String {
    let row = format!(
        "{time:<19} {track:<13} {:>+6.2}%  {confidence} {hits:>6}  {asset}",
        (speed - 1.0) * 100.0
    );
    row.trim_end().to_owned()
}

fn span(start_seconds: f64, end_seconds: f64) -> String {
    format!(
        "{}-{}",
        format_timecode(Duration::from_secs_f64(start_seconds)),
        format_timecode(Duration::from_secs_f64(end_seconds))
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::ColorChoice;

    fn segment(start_seconds: f64, end_seconds: f64, track_start_seconds: f64) -> Segment {
        Segment {
            start_seconds,
            end_seconds,
            track_start_seconds,
            track_end_seconds: track_start_seconds + (end_seconds - start_seconds) * 1.06,
            speed: 1.06,
            confidence: "confident",
            windows: 4,
            hits: 400,
        }
    }

    fn report() -> Report<'static> {
        Report {
            schema_version: 3,
            query: Query {
                path: Path::new("mix.wav"),
                start_seconds: 0.0,
                duration_seconds: 150.0,
            },
            plays: vec![FoundPlay {
                asset: "c.wav",
                same_audio: vec!["copy-of-c.wav"],
                start_seconds: 96.0,
                end_seconds: 140.0,
                track_start_seconds: 20.0,
                track_end_seconds: 70.0,
                speed: 1.06,
                confidence: "confident",
                windows: 8,
                hits: 800,
                segments: vec![segment(96.0, 111.0, 20.0), segment(112.0, 140.0, 41.0)],
            }],
        }
    }

    #[test]
    fn the_table_lists_same_audio_and_segments_under_the_play() {
        let table = table(&report(), Style::for_stdout(ColorChoice::Never));

        assert_eq!(
            table,
            "\
time                in track        speed  confidence   hits  asset
1:36-2:20           0:20-1:10      +6.00%  confident     800  c.wav
                                                              also copy-of-c.wav
  1:36-1:51         0:20-0:35      +6.00%  confident     400
  1:52-2:20         0:41-1:10      +6.00%  confident     400
"
        );
    }

    #[test]
    fn colour_marks_the_confidence_without_moving_columns() {
        let table = table(&report(), Style::for_stdout(ColorChoice::Always));

        let play_row = table.lines().nth(1).unwrap_or_default();
        assert!(play_row.contains("+6.00%  \x1b[32mconfident \x1b[0m    800  c.wav"));
    }
}

//! `gunfinger explain`: every candidate around a moment of a recording,
//! weak ones included, with its evidence and what it lacks for the next
//! level. Answers "why was nothing found here?" and "why only possible?".
//! `--windows` adds the evidence of each 10 s window: the lines of hits
//! that chains join into detections.

use std::path::Path;
use std::time::Duration;

use gunfinger_core::confidence::{Confidence, Evidence, MIN_HITS, MIN_POSSIBLE_HITS, MIN_WINDOWS};
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::{AssetId, Index};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Trace, WINDOW_SECONDS, WindowLine, trace_with_progress};
use gunfinger_core::speed::Playback;
use miette::{IntoDiagnostic, WrapErr};

use crate::catalog::Catalog;
use crate::console::Console;
use crate::playback::PlaybackChoice;
use crate::report::Level;
use crate::style::Style;
use crate::table::{named, span, timecode};

pub struct Request<'a> {
    pub audio: &'a Path,
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub exclude_from: Option<&'a Path>,
    pub at: Duration,
    pub around: Duration,
    pub playback: PlaybackChoice,
    /// Only assets whose path contains this, ignoring case.
    pub asset: Option<&'a str>,
    pub limit: usize,
    /// Also list each window's lines for the candidates' assets.
    pub windows: bool,
    pub style: Style,
    pub jobs: usize,
    pub console: &'a Console,
}

pub fn run(request: &Request) -> miette::Result<()> {
    std::fs::File::open(request.audio)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot read {}", request.audio.display()))?;
    let catalog = Catalog::open(
        request.library,
        request.peaks_dir,
        request.exclude_from,
        request.console,
    )?;
    let start = on_window_grid(request.at.saturating_sub(request.around));
    let excerpt = Excerpt {
        start: Some(start),
        duration: Some(request.at - start + request.around),
    };
    let profile = Profile::CURRENT;
    let audio = decode(request.audio, profile.sample_rate, excerpt).into_diagnostic()?;
    let ladder = request.playback.rungs();
    let trace = trace_with_progress(
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

    let offset = start.as_secs_f64();
    println!(
        "searched {} around {}; hits count only this stretch",
        span(offset, offset + audio.duration().as_secs_f64()),
        timecode(request.at.as_secs_f64())
    );
    println!(
        "confident: {MIN_HITS} hits or more in {MIN_WINDOWS} windows or more; possible: {MIN_POSSIBLE_HITS} hits or more"
    );
    println!();
    let candidates = candidates(&trace.detections, &catalog.index, request.asset);
    if candidates.is_empty() {
        println!("{}", nothing_found(&catalog.index, request.asset));
    } else {
        print_candidates(request, &catalog.index, &trace, &candidates, offset);
    }
    if request.windows {
        print_windows(request, &catalog.index, &trace, &candidates, offset);
    }
    Ok(())
}

/// Searches of a whole recording cut it into windows from its start; an
/// excerpt that starts on that grid has the same windows.
fn on_window_grid(start: Duration) -> Duration {
    let windows = (start.as_secs_f64() / WINDOW_SECONDS).floor();
    Duration::from_secs_f64(windows * WINDOW_SECONDS)
}

fn print_candidates(
    request: &Request,
    index: &Index,
    trace: &Trace,
    candidates: &[usize],
    offset: f64,
) {
    println!(
        "{}",
        request.style.bold(&format!(
            "{:>3}  {:<19} {:<13} {:>7} {:>6} {:>7}  {:<10} {:<38} asset",
            "#", "time", "in track", "speed", "hits", "windows", "level", "short of"
        ))
    );
    for (rank, &candidate) in candidates.iter().enumerate().take(request.limit) {
        let detection = &trace.detections[candidate];
        let level = Level::from(detection.evidence.confidence());
        let cell = format!("{:<10}", level.label());
        let cell = match level {
            Level::Confident => request.style.green(&cell),
            Level::Possible => request.style.yellow(&cell),
            Level::Weak => request.style.dim(&cell),
        };
        println!(
            "{:>3}  {:<19} {:<13} {:>+6.2}% {:>6} {:>7}  {cell} {:<38} {}",
            rank + 1,
            span(
                offset + detection.start_seconds,
                offset + detection.end_seconds
            ),
            span(detection.track_start_seconds, detection.track_end_seconds),
            (detection.speed.0 - 1.0) * 100.0,
            detection.evidence.hits,
            detection.evidence.windows,
            short_of(detection.evidence),
            named(
                &index.asset(detection.asset).path,
                detection.playback.into()
            )
        );
    }
    if candidates.len() > request.limit {
        println!(
            "... and {} weaker (--limit shows more)",
            candidates.len() - request.limit
        );
    }
}

/// Each window's lines for the assets of the candidates shown (or, when
/// there is no candidate, of every asset matching `--asset`), with the
/// number of the candidate whose chain took each line. A line in no candidate's chain stayed alone in its
/// window, or its chain overlapped a stronger one of the same asset.
/// Chained lines come first, then the strongest others.
fn print_windows(
    request: &Request,
    index: &Index,
    trace: &Trace,
    candidates: &[usize],
    offset: f64,
) {
    let mut taken_by = vec![None; trace.lines.len()];
    for (rank, &candidate) in candidates.iter().enumerate() {
        for &line in &trace.chains[candidate] {
            taken_by[line] = Some(rank + 1);
        }
    }
    let shown: Vec<AssetId> = candidates
        .iter()
        .take(request.limit)
        .map(|&candidate| trace.detections[candidate].asset)
        .collect();
    let wanted = request
        .asset
        .filter(|_| candidates.is_empty())
        .map(str::to_lowercase);
    let numbered: Vec<usize> = (0..trace.lines.len()).collect();
    for same_asset in numbered.chunk_by(|&a, &b| trace.lines[a].asset == trace.lines[b].asset) {
        let asset = trace.lines[same_asset[0]].asset;
        let path = &index.asset(asset).path;
        let listed = match &wanted {
            Some(wanted) => path.to_lowercase().contains(wanted),
            None => shown.contains(&asset),
        };
        if !listed {
            continue;
        }
        println!();
        println!(
            "{}",
            request.style.bold(&format!("{path}: lines per window"))
        );
        println!(
            "{}",
            request.style.dim(&format!(
                "  {:<13} {:<13} {:>5} {:>7}  {:<9} {:<9} chain",
                "window", "hits from-to", "hits", "rung", "playback", "in track"
            ))
        );
        for same_window in
            same_asset.chunk_by(|&a, &b| trace.lines[a].window == trace.lines[b].window)
        {
            let mut ordered = same_window.to_vec();
            // Lines arrive strongest first; chained ones move ahead.
            ordered.sort_by_key(|&line| taken_by[line].is_none());
            for &line in ordered.iter().take(LINES_PER_WINDOW) {
                println!(
                    "{}",
                    window_line(&trace.lines[line], taken_by[line], offset)
                );
            }
            if ordered.len() > LINES_PER_WINDOW {
                println!(
                    "{}",
                    request.style.dim(&format!(
                        "  {:<27} {} weaker lines",
                        "",
                        ordered.len() - LINES_PER_WINDOW
                    ))
                );
            }
        }
    }
}

/// Lines shown per window: a played record's line, a few alternatives at
/// other offsets or rungs, and repeats of its own sections.
const LINES_PER_WINDOW: usize = 4;

fn window_line(line: &WindowLine, taken_by: Option<usize>, offset: f64) -> String {
    let window_start = offset + f64::from(line.window) * WINDOW_SECONDS;
    let playback = match line.playback {
        Playback::Turntable => "turntable",
        Playback::KeyLocked => "key lock",
    };
    format!(
        "  {:<13} {:<13} {:>5} {:>+6.2}%  {:<9} {:<9} {}",
        span(window_start, window_start + WINDOW_SECONDS),
        span(offset + line.start_seconds, offset + line.end_seconds),
        line.hits,
        (line.speed.0 - 1.0) * 100.0,
        playback,
        timecode(line.track_seconds),
        taken_by.map_or_else(|| String::from("-"), |rank| format!("#{rank}"))
    )
}

/// Indexes of the detections strongest first, of matching assets only when
/// `asset` is given.
fn candidates(detections: &[Detection], index: &Index, asset: Option<&str>) -> Vec<usize> {
    let wanted = asset.map(str::to_lowercase);
    let mut candidates: Vec<usize> = (0..detections.len())
        .filter(|&candidate| {
            wanted.as_ref().is_none_or(|wanted| {
                index
                    .asset(detections[candidate].asset)
                    .path
                    .to_lowercase()
                    .contains(wanted)
            })
        })
        .collect();
    candidates.sort_by_key(|&candidate| std::cmp::Reverse(detections[candidate].evidence.hits));
    candidates
}

fn nothing_found(index: &Index, asset: Option<&str>) -> String {
    let Some(asset) = asset else {
        return String::from(
            "no candidate at all: no asset has hits on one line in two windows or more",
        );
    };
    let wanted = asset.to_lowercase();
    let matching = index
        .assets()
        .iter()
        .filter(|entry| entry.path.to_lowercase().contains(&wanted))
        .count();
    if matching == 0 {
        format!("no asset in the index has \"{asset}\" in its path")
    } else {
        format!(
            "{matching} assets match \"{asset}\", and none has hits on one line in two windows or more here"
        )
    }
}

/// What the evidence lacks for the next level.
fn short_of(evidence: Evidence) -> String {
    match evidence.confidence() {
        Confidence::Confident => String::new(),
        Confidence::Possible => {
            let mut missing = Vec::new();
            if evidence.hits < MIN_HITS {
                missing.push(format!("{} more hits", MIN_HITS - evidence.hits));
            }
            let windows = MIN_WINDOWS.saturating_sub(evidence.windows);
            if windows > 0 {
                let plural = if windows == 1 { "" } else { "s" };
                missing.push(format!("{windows} more window{plural}"));
            }
            format!("confident: {}", missing.join(", "))
        }
        Confidence::Weak => format!("possible: {} more hits", MIN_POSSIBLE_HITS - evidence.hits),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_level_names_what_it_lacks() {
        let evidence = |hits, windows| Evidence::new(windows, hits);

        assert_eq!(short_of(evidence(MIN_HITS, MIN_WINDOWS)), "");
        assert_eq!(
            short_of(evidence(91, 2)),
            "confident: 109 more hits, 1 more window"
        );
        assert_eq!(short_of(evidence(450, 1)), "confident: 2 more windows");
        assert_eq!(short_of(evidence(45, 4)), "possible: 15 more hits");
    }
}

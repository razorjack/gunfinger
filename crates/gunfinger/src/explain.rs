//! `gunfinger explain`: every candidate around a moment of a recording,
//! weak ones included, with its evidence and what it lacks for the next
//! level. Answers "why was nothing found here?" and "why only possible?".

use std::path::Path;
use std::time::Duration;

use gunfinger_core::confidence::{Confidence, Evidence, MIN_HITS, MIN_POSSIBLE_HITS, MIN_WINDOWS};
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search};
use gunfinger_core::speed::ladder;
use miette::{IntoDiagnostic, WrapErr};

use crate::catalog::Catalog;
use crate::console::Console;
use crate::report::Level;
use crate::style::Style;
use crate::table::{span, timecode};

pub struct Request<'a> {
    pub audio: &'a Path,
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub exclude_from: Option<&'a Path>,
    pub at: Duration,
    pub around: Duration,
    /// Only assets whose path contains this, ignoring case.
    pub asset: Option<&'a str>,
    pub limit: usize,
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
    let start = request.at.saturating_sub(request.around);
    let excerpt = Excerpt {
        start: Some(start),
        duration: Some(request.at - start + request.around),
    };
    let profile = Profile::CURRENT;
    let audio = decode(request.audio, profile.sample_rate, excerpt).into_diagnostic()?;
    let detections = search(
        &catalog.index,
        &audio.samples,
        &profile,
        &ladder(),
        request.jobs,
    );

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
    let candidates = candidates(&detections, &catalog.index, request.asset);
    if candidates.is_empty() {
        println!("{}", nothing_found(&catalog.index, request.asset));
        return Ok(());
    }
    println!(
        "{}",
        request.style.bold(&format!(
            "{:<19} {:<13} {:>7} {:>6} {:>7}  {:<10} {:<38} asset",
            "time", "in track", "speed", "hits", "windows", "level", "short of"
        ))
    );
    for detection in candidates.iter().take(request.limit) {
        let level = Level::from(detection.evidence.confidence());
        let cell = format!("{:<10}", level.label());
        let cell = match level {
            Level::Confident => request.style.green(&cell),
            Level::Possible => request.style.yellow(&cell),
            Level::Weak => request.style.dim(&cell),
        };
        println!(
            "{:<19} {:<13} {:>+6.2}% {:>6} {:>7}  {cell} {:<38} {}",
            span(
                offset + detection.start_seconds,
                offset + detection.end_seconds
            ),
            span(detection.track_start_seconds, detection.track_end_seconds),
            (detection.speed.0 - 1.0) * 100.0,
            detection.evidence.hits,
            detection.evidence.windows,
            short_of(detection.evidence),
            catalog.index.asset(detection.asset).path
        );
    }
    if candidates.len() > request.limit {
        println!(
            "... and {} weaker (--limit shows more)",
            candidates.len() - request.limit
        );
    }
    Ok(())
}

/// Detections strongest first, of matching assets only when `asset` is
/// given.
fn candidates<'a>(
    detections: &'a [Detection],
    index: &Index,
    asset: Option<&str>,
) -> Vec<&'a Detection> {
    let wanted = asset.map(str::to_lowercase);
    let mut candidates: Vec<&Detection> = detections
        .iter()
        .filter(|detection| {
            wanted.as_ref().is_none_or(|wanted| {
                index
                    .asset(detection.asset)
                    .path
                    .to_lowercase()
                    .contains(wanted)
            })
        })
        .collect();
    candidates.sort_by_key(|detection| std::cmp::Reverse(detection.evidence.hits));
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
        let evidence = |hits, windows| Evidence { windows, hits };

        assert_eq!(short_of(evidence(MIN_HITS, MIN_WINDOWS)), "");
        assert_eq!(
            short_of(evidence(91, 2)),
            "confident: 109 more hits, 1 more window"
        );
        assert_eq!(short_of(evidence(450, 1)), "confident: 2 more windows");
        assert_eq!(short_of(evidence(45, 4)), "possible: 15 more hits");
    }
}

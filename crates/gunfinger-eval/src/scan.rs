//! Scanning a set: search its audio and score the result against the
//! manifest, optionally with some referenced tracks left out of the index.

use std::collections::BTreeSet;
use std::path::Path;
use std::time::{Duration, Instant};

use gunfinger_core::confidence::Pass;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::plays;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakStore;
use gunfinger_core::timecode::format_timecode;
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::manifest::{Set, load_set};
use crate::matching::Matching;
use crate::padding::Padding;
use crate::rng::Rng;
use crate::scoring::{Found, FoundPlay, Score, score};
use crate::verifier::{Verification, Verifier};

/// Which referenced tracks to leave out of the index: `count` of them, drawn
/// with `seed`.
pub struct LeaveOut {
    pub count: usize,
    pub seed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct ScanReport {
    pub set: String,
    pub audio_file: String,
    pub duration_seconds: f64,
    pub wall_seconds: f64,
    /// Labels of the tracks whose whole clusters were left out of the index.
    pub left_out_tracks: Vec<String>,
    pub left_out_assets: Vec<String>,
    pub score: Score,
    /// Every detection with at least two windows, strongest first.
    pub detections: Vec<Found>,
    /// Every confident or possible play, in order of start time.
    pub plays: Vec<FoundPlay>,
}

/// How the index differs from the library's, and the worker threads.
pub struct Options<'a> {
    pub leave_out: Option<&'a LeaveOut>,
    /// What is added to the index to measure a larger library.
    pub padding: &'a Padding,
    pub matching: &'a Matching,
    pub ladder: &'a [Rung],
    /// Measure every detection with the peak verifier.
    pub verify: bool,
    pub jobs: usize,
}

pub fn run(
    sets_dir: &Path,
    set_name: &str,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    options: &Options,
) -> Result<ScanReport, String> {
    let Options {
        leave_out,
        padding,
        matching,
        ladder,
        verify,
        jobs,
    } = *options;
    let set = load_set(sets_dir, set_name, library).map_err(|problems| problems.join("; "))?;
    let profile = Profile::CURRENT;
    let (left_out_tracks, left_out_assets) = match leave_out {
        Some(leave_out) => draw_left_out(&set, clusters, leave_out),
        None => (Vec::new(), BTreeSet::new()),
    };
    let (records, _) = load_records(library, store, &profile, &left_out_assets);
    let index = matching.index(padding.index(&records, &profile, &left_out_assets)?);
    let verifier = verify.then(|| Verifier::new(&records, padding.second.as_ref()));
    drop(records);

    let started = Instant::now();
    let audio = decode(&set.audio, profile.sample_rate, Excerpt::default())
        .map_err(|error| error.to_string())?;
    let detections = matching.search(&index, &audio.samples, &profile, ladder, jobs);
    let wall_seconds = started.elapsed().as_secs_f64();
    let plays: Vec<FoundPlay> = plays::group(&detections)
        .iter()
        .map(|play| FoundPlay {
            asset: index.asset(play.asset).path.clone(),
            start_seconds: play.start_seconds(),
            end_seconds: play.end_seconds(),
            segments: play
                .segments()
                .iter()
                .map(|segment| found(&index, segment, None))
                .collect(),
        })
        .collect();
    let detections: Vec<Found> = detections
        .iter()
        .map(|detection| {
            let verified = verifier
                .as_ref()
                .and_then(|verifier| verifier.verify(&index, detection, &audio.samples, &profile));
            found(&index, detection, verified)
        })
        .collect();
    let duration_seconds = audio.duration().as_secs_f64();

    Ok(ScanReport {
        set: set_name.to_owned(),
        audio_file: set
            .audio
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default(),
        duration_seconds,
        wall_seconds,
        left_out_tracks,
        left_out_assets: left_out_assets.into_iter().collect(),
        score: score(&set, duration_seconds, &detections, &plays, clusters),
        detections,
        plays,
    })
}

fn found(index: &Index, detection: &Detection, verified: Option<Verification>) -> Found {
    Found {
        asset: index.asset(detection.asset).path.clone(),
        start_seconds: detection.start_seconds,
        end_seconds: detection.end_seconds,
        speed: detection.speed.0,
        windows: detection.evidence.windows,
        hits: detection.evidence.hits,
        confident: detection.evidence.is_confident(),
        fitted: detection.evidence.pass == Pass::Fitted,
        verified,
    }
}

/// Draws referenced tracks and collects every member of their references'
/// clusters, so no rip of a left-out track stays in the index.
fn draw_left_out(
    set: &Set,
    clusters: &Clusters,
    leave_out: &LeaveOut,
) -> (Vec<String>, BTreeSet<String>) {
    let mut referenced: Vec<_> = set
        .tracks
        .iter()
        .filter(|track| track.is_referenced())
        .collect();
    Rng::new(leave_out.seed).shuffle(&mut referenced);
    referenced.truncate(leave_out.count);
    referenced.sort_by_key(|track| track.position);
    let assets = referenced
        .iter()
        .flat_map(|track| &track.references)
        .flat_map(|reference| clusters.cluster_of(reference))
        .collect();
    (
        referenced.iter().map(|track| track.label()).collect(),
        assets,
    )
}

pub fn print_summary(report: &ScanReport) {
    let score = &report.score;
    println!(
        "{}: {}/{} referenced tracks identified, {} wrong identifications; {} of audio scanned in {:.1} s",
        report.set,
        score.identified,
        score.referenced,
        score.wrong,
        timecode(report.duration_seconds),
        report.wall_seconds
    );
    println!(
        "possible tier: {} more referenced tracks found as possible, {} possible plays match no track",
        score.possible,
        score.unmatched_possible.len()
    );
    if !report.left_out_tracks.is_empty() {
        println!(
            "left out of the index: {}",
            report.left_out_tracks.join("; ")
        );
    }
    for track in &score.tracks {
        let mark = match (track.referenced, track.identified, track.possible) {
            (true, true, _) => "found",
            (true, false, true) => "possible",
            (true, false, false) => "MISSED",
            (false, _, _) => "absent",
        };
        let evidence = track
            .strongest_candidate
            .as_ref()
            .map_or_else(String::new, |found| {
                format!(
                    "  best {} hits/{} windows at {:+.2}% {}-{}",
                    found.hits,
                    found.windows,
                    (found.speed - 1.0) * 100.0,
                    timecode(found.start_seconds),
                    timecode(found.end_seconds),
                )
            });
        println!("  {mark:<8} {}{evidence}", track.label);
        for asset in &track.credited_through_cluster {
            println!("           credited through cluster: {asset}");
        }
    }
    for wrong in &score.wrong_identifications {
        println!(
            "  WRONG  {} {}-{} {:+.2}% {} hits/{} windows",
            wrong.asset,
            timecode(wrong.start_seconds),
            timecode(wrong.end_seconds),
            (wrong.speed - 1.0) * 100.0,
            wrong.hits,
            wrong.windows
        );
    }
    for play in &score.unmatched_possible {
        println!(
            "  POSSIBLE, NO TRACK  {} {}-{} {} hits in {} segments",
            play.asset,
            timecode(play.start_seconds),
            timecode(play.end_seconds),
            play.hits(),
            play.segments.len()
        );
    }
}

fn timecode(seconds: f64) -> String {
    format_timecode(Duration::from_secs_f64(seconds))
}

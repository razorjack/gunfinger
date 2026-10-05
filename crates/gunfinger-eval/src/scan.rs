//! Scanning a set: search its audio and score the result against the
//! manifest, optionally with some referenced tracks left out of the index.

use std::collections::BTreeSet;
use std::path::Path;
use std::time::Instant;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::search;
use gunfinger_core::speed::ladder;
use gunfinger_core::store::PeakStore;
use gunfinger_core::timecode::format_timecode;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::manifest::{Set, load_set};
use crate::rng::Rng;
use crate::scoring::{Found, Score, score};

/// Which referenced tracks to leave out of the index: `count` of them, drawn
/// with `seed`.
pub struct LeaveOut {
    pub count: usize,
    pub seed: u64,
}

#[derive(Serialize)]
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
}

pub fn run(
    sets_dir: &Path,
    set_name: &str,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    leave_out: Option<&LeaveOut>,
    jobs: usize,
) -> Result<ScanReport, String> {
    let set = load_set(sets_dir, set_name, library).map_err(|problems| problems.join("; "))?;
    let profile = Profile::CURRENT;
    let (left_out_tracks, left_out_assets) = match leave_out {
        Some(leave_out) => draw_left_out(&set, clusters, leave_out),
        None => (Vec::new(), BTreeSet::new()),
    };
    let (records, _) = load_records(library, store, &profile, &left_out_assets);
    let index = Index::build(&records).map_err(|error| error.to_string())?;

    let started = Instant::now();
    let audio = decode(&set.audio, profile.sample_rate, Excerpt::default())
        .map_err(|error| error.to_string())?;
    let detections: Vec<Found> = search(&index, &audio.samples, &profile, &ladder(), jobs)
        .iter()
        .map(|detection| Found {
            asset: index.asset(detection.asset).path.clone(),
            start_seconds: detection.start_seconds,
            end_seconds: detection.end_seconds,
            speed: detection.speed.0,
            windows: detection.evidence.windows,
            hits: detection.evidence.hits,
            confident: detection.evidence.is_confident(),
        })
        .collect();
    let wall_seconds = started.elapsed().as_secs_f64();
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
        score: score(&set, duration_seconds, &detections, clusters),
        detections,
    })
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
        format_timecode(std::time::Duration::from_secs_f64(report.duration_seconds)),
        report.wall_seconds
    );
    if !report.left_out_tracks.is_empty() {
        println!(
            "left out of the index: {}",
            report.left_out_tracks.join("; ")
        );
    }
    for track in &score.tracks {
        let mark = match (track.referenced, track.identified) {
            (true, true) => "found ",
            (true, false) => "MISSED",
            (false, _) => "absent",
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
                    format_timecode(std::time::Duration::from_secs_f64(found.start_seconds)),
                    format_timecode(std::time::Duration::from_secs_f64(found.end_seconds)),
                )
            });
        println!("  {mark} {}{evidence}", track.label);
        for asset in &track.credited_through_cluster {
            println!("         credited through cluster: {asset}");
        }
    }
    for wrong in &score.wrong_identifications {
        println!(
            "  WRONG  {} {}-{} {:+.2}% {} hits/{} windows",
            wrong.asset,
            format_timecode(std::time::Duration::from_secs_f64(wrong.start_seconds)),
            format_timecode(std::time::Duration::from_secs_f64(wrong.end_seconds)),
            (wrong.speed - 1.0) * 100.0,
            wrong.hits,
            wrong.windows
        );
    }
}

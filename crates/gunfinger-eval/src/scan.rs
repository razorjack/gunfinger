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
use crate::manifest::{Referable, Set, Track, load_set};
use crate::matching::Matching;
use crate::padding::Padding;
use crate::rng::Rng;
use crate::scoring::{Found, FoundPlay, Score, score};
use crate::titles::SameTitles;
use crate::verifier::{Verification, Verifier};

/// Which tracks to leave out of the index, each with its whole cluster.
pub enum LeaveOut {
    /// `count` referenced tracks, drawn with `seed`.
    Drawn { count: usize, seed: u64 },
    /// The tracks in these runs of consecutive manifest slots, so that each
    /// run leaves a gap of known tracks between kept ones.
    Slots(Vec<SlotRun>),
}

/// Manifest positions `first` to `last`, both included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SlotRun {
    pub first: u32,
    pub last: u32,
}

impl SlotRun {
    /// Parses `7` or `7-9`.
    pub fn parse(text: &str) -> Result<SlotRun, String> {
        let slot = |text: &str| {
            text.trim()
                .parse::<u32>()
                .map_err(|_| format!("`{text}` is not a slot number"))
        };
        let (first, last) = match text.split_once('-') {
            Some((first, last)) => (slot(first)?, slot(last)?),
            None => (slot(text)?, slot(text)?),
        };
        if first > last {
            return Err(format!("the slots `{text}` run backwards"));
        }
        Ok(SlotRun { first, last })
    }

    fn holds(self, position: u32) -> bool {
        (self.first..=self.last).contains(&position)
    }

    /// As it is written: `7` or `7-9`.
    pub fn name(self) -> String {
        if self.first == self.last {
            self.first.to_string()
        } else {
            format!("{}-{}", self.first, self.last)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ScanReport {
    pub set: String,
    pub audio_file: String,
    pub duration_seconds: f64,
    pub wall_seconds: f64,
    /// Labels of the tracks whose whole clusters were left out of the index.
    pub left_out_tracks: Vec<String>,
    /// The slots named with `--leave-out-slots`, as written.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub left_out_slots: Vec<String>,
    pub left_out_assets: Vec<String>,
    pub score: Score,
    /// Every detection with at least two windows, strongest first.
    pub detections: Vec<Found>,
    /// Every confident or possible play, in order of start time.
    pub plays: Vec<FoundPlay>,
}

/// How the index differs from the library's, and the worker threads.
pub struct Options<'a> {
    /// The files the manifest's references may name.
    pub referable: &'a Referable,
    pub leave_out: Option<&'a LeaveOut>,
    /// What is added to the index to measure a larger library.
    pub padding: &'a Padding,
    pub matching: &'a Matching,
    pub ladder: &'a [Rung],
    /// The files credited to another recording's title (ADR 0022).
    pub same_titles: &'a SameTitles,
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
        referable,
        leave_out,
        padding,
        matching,
        ladder,
        same_titles,
        verify,
        jobs,
    } = *options;
    let set = load_set(sets_dir, set_name, referable).map_err(|problems| problems.join("; "))?;
    let profile = Profile::CURRENT;
    let (left_out_tracks, left_out_assets) = match leave_out {
        Some(leave_out) => left_out(&set, clusters, leave_out)?,
        None => (Vec::new(), BTreeSet::new()),
    };
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let index = matching.index(padding.index(&records, &profile, &left_out_assets)?);
    same_titles.warn_outside(&index, &left_out_assets);
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
        left_out_slots: match leave_out {
            Some(LeaveOut::Slots(runs)) => runs.iter().map(|run| run.name()).collect(),
            _ => Vec::new(),
        },
        left_out_assets: left_out_assets.into_iter().collect(),
        score: score(
            &set,
            duration_seconds,
            &detections,
            &plays,
            clusters,
            same_titles,
        ),
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

/// The tracks to leave out, drawn or named, with every member of their
/// references' clusters, so no rip of a left-out track stays in the index.
fn left_out(
    set: &Set,
    clusters: &Clusters,
    leave_out: &LeaveOut,
) -> Result<(Vec<String>, BTreeSet<String>), String> {
    let mut tracks: Vec<&Track> = match leave_out {
        LeaveOut::Drawn { count, seed } => {
            let mut referenced: Vec<&Track> = set
                .tracks
                .iter()
                .filter(|track| track.is_referenced())
                .collect();
            Rng::new(*seed).shuffle(&mut referenced);
            referenced.truncate(*count);
            referenced
        }
        LeaveOut::Slots(runs) => named_slots(set, runs)?,
    };
    tracks.sort_by_key(|track| track.position);
    let assets = tracks
        .iter()
        .flat_map(|track| &track.references)
        .flat_map(|reference| clusters.cluster_of(reference))
        .collect();
    Ok((tracks.iter().map(|track| track.label()).collect(), assets))
}

/// The manifest's tracks in `runs`; every slot named must be in it.
fn named_slots<'a>(set: &'a Set, runs: &[SlotRun]) -> Result<Vec<&'a Track>, String> {
    for run in runs {
        for position in run.first..=run.last {
            if !set.tracks.iter().any(|track| track.position == position) {
                return Err(format!("the manifest has no slot {position}"));
            }
        }
    }
    Ok(set
        .tracks
        .iter()
        .filter(|track| runs.iter().any(|run| run.holds(track.position)))
        .collect())
}

pub fn print_summary(report: &ScanReport) {
    let score = &report.score;
    println!(
        "{}: {}/{} referenced tracks identified, {} wrong identifications, {} same-title credits; {} of audio scanned in {:.1} s",
        report.set,
        score.identified,
        score.referenced,
        score.wrong,
        score.same_title,
        timecode(report.duration_seconds),
        report.wall_seconds
    );
    println!(
        "possible tier: {} more referenced tracks found as possible, {} possible plays match no track",
        score.possible,
        score.unmatched_possible.len()
    );
    if !report.left_out_slots.is_empty() {
        println!("slots left out: {}", report.left_out_slots.join(","));
    }
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
        for asset in &track.credited_through_title {
            println!("           credited through the same title: {asset}");
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

#[cfg(test)]
mod tests {
    use super::*;

    fn set_of(references: &[&[&str]]) -> Set {
        Set {
            title: String::from("A mix"),
            audio: std::path::PathBuf::from("mix.mp3"),
            tracks: references
                .iter()
                .zip(1..)
                .map(|(references, position)| Track {
                    position,
                    artist: String::from("Artist"),
                    title: format!("Track {position}"),
                    start: Duration::from_secs(u64::from(position) * 300),
                    references: references.iter().map(|&file| file.to_owned()).collect(),
                    set_aside: Vec::new(),
                })
                .collect(),
        }
    }

    #[test]
    fn slot_runs_are_single_slots_or_ranges() {
        assert_eq!(SlotRun::parse("7"), Ok(SlotRun { first: 7, last: 7 }));
        assert_eq!(
            SlotRun::parse("12-14"),
            Ok(SlotRun {
                first: 12,
                last: 14
            })
        );
        assert_eq!(
            SlotRun::parse("12-14").map(SlotRun::name),
            Ok(String::from("12-14"))
        );
        assert!(SlotRun::parse("9-8").is_err());
        assert!(SlotRun::parse("x").is_err());
    }

    #[test]
    fn named_slots_leave_out_their_tracks_and_whole_clusters() {
        let set = set_of(&[&["a.mp3"], &["b.mp3"], &["c.mp3"], &[], &["e.mp3"]]);
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("b.mp3"), String::from("b rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let runs = vec![SlotRun { first: 2, last: 2 }, SlotRun { first: 4, last: 5 }];

        let (tracks, assets) = left_out(&set, &clusters, &LeaveOut::Slots(runs)).unwrap();

        assert_eq!(
            tracks,
            [
                " 2. Artist - Track 2",
                " 4. Artist - Track 4",
                " 5. Artist - Track 5"
            ]
        );
        assert_eq!(
            assets,
            BTreeSet::from([
                String::from("b rip.mp3"),
                String::from("b.mp3"),
                String::from("e.mp3")
            ])
        );
    }

    #[test]
    fn a_slot_the_manifest_does_not_have_is_refused() {
        let set = set_of(&[&["a.mp3"], &["b.mp3"]]);
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: Vec::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        let refused = left_out(
            &set,
            &clusters,
            &LeaveOut::Slots(vec![SlotRun { first: 2, last: 3 }]),
        );

        assert_eq!(refused, Err(String::from("the manifest has no slot 3")));
    }
}

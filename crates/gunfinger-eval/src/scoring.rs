//! Scoring the detections of a set against its manifest.
//!
//! A referenced track is identified when a confident detection of one of its
//! references, or of an asset in the same duplicate cluster, overlaps its
//! window: from `TOLERANCE` before its start to `TOLERANCE` after the next
//! track's start. Every other confident detection is a wrong identification,
//! including any in a slot marked absent.

use std::collections::BTreeSet;

use serde::Serialize;

use crate::clusters::Clusters;
use crate::manifest::Set;

/// Manifest start times are approximate and neighbouring tracks overlap.
pub const TOLERANCE_SECONDS: f64 = 90.0;

/// A detection as the scorer sees it.
#[derive(Debug, Clone, Serialize)]
pub struct Found {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speed: f64,
    pub windows: u32,
    pub hits: u32,
    pub confident: bool,
}

#[derive(Debug, Serialize)]
pub struct Score {
    pub referenced: usize,
    pub identified: usize,
    pub wrong: usize,
    pub tracks: Vec<TrackScore>,
    /// Confident detections credited to no track.
    pub wrong_identifications: Vec<Found>,
}

#[derive(Debug, Serialize)]
pub struct TrackScore {
    pub label: String,
    pub start_seconds: f64,
    pub referenced: bool,
    pub identified: bool,
    /// Confident detections that identify this track.
    pub credited: Vec<Found>,
    /// Credited detections of an asset that is not itself a reference but
    /// shares a duplicate cluster with one: listed for the owner to confirm.
    pub credited_through_cluster: Vec<String>,
    /// The strongest detection of a reference or a cluster member inside the
    /// window, confident or not: the evidence for a miss.
    pub strongest_candidate: Option<Found>,
}

pub fn score(set: &Set, duration_seconds: f64, found: &[Found], clusters: &Clusters) -> Score {
    let mut credited_anywhere = vec![false; found.len()];
    let mut tracks = Vec::new();
    for (position, track) in set.tracks.iter().enumerate() {
        let window_start = track.start.as_secs_f64() - TOLERANCE_SECONDS;
        let next_start = set
            .tracks
            .get(position + 1)
            .map_or(duration_seconds, |next| next.start.as_secs_f64());
        let window_end = next_start + TOLERANCE_SECONDS;
        let references: BTreeSet<&str> = track.references.iter().map(String::as_str).collect();
        let accepted: BTreeSet<String> = track
            .references
            .iter()
            .flat_map(|reference| clusters.cluster_of(reference))
            .collect();

        let mut credited = Vec::new();
        let mut through_cluster = BTreeSet::new();
        let mut strongest_candidate: Option<&Found> = None;
        for (index, detection) in found.iter().enumerate() {
            let overlaps =
                detection.start_seconds < window_end && window_start < detection.end_seconds;
            if !overlaps || !accepted.contains(&detection.asset) {
                continue;
            }
            if strongest_candidate.is_none_or(|strongest| detection.hits > strongest.hits) {
                strongest_candidate = Some(detection);
            }
            if detection.confident {
                credited_anywhere[index] = true;
                credited.push(detection.clone());
                if !references.contains(detection.asset.as_str()) {
                    through_cluster.insert(detection.asset.clone());
                }
            }
        }
        tracks.push(TrackScore {
            label: track.label(),
            start_seconds: track.start.as_secs_f64(),
            referenced: track.is_referenced(),
            identified: track.is_referenced() && !credited.is_empty(),
            credited,
            credited_through_cluster: through_cluster.into_iter().collect(),
            strongest_candidate: strongest_candidate.cloned(),
        });
    }

    let wrong_identifications: Vec<Found> = found
        .iter()
        .zip(&credited_anywhere)
        .filter(|(detection, credited)| detection.confident && !**credited)
        .map(|(detection, _)| detection.clone())
        .collect();
    Score {
        referenced: tracks.iter().filter(|track| track.referenced).count(),
        identified: tracks.iter().filter(|track| track.identified).count(),
        wrong: wrong_identifications.len(),
        tracks,
        wrong_identifications,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use super::*;
    use crate::manifest::Track;

    fn track(position: u32, start: u64, references: &[&str]) -> Track {
        Track {
            position,
            artist: "Artist".to_owned(),
            title: format!("Title {position}"),
            start: Duration::from_secs(start),
            references: references.iter().map(|path| (*path).to_owned()).collect(),
        }
    }

    fn set() -> Set {
        Set {
            title: "Mix".to_owned(),
            audio: PathBuf::from("mix.m4a"),
            tracks: vec![
                track(1, 0, &["a.mp3"]),
                track(2, 300, &[]),
                track(3, 600, &["c.mp3"]),
            ],
        }
    }

    fn clusters() -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates: vec![vec!["c.mp3".to_owned(), "c-rip.mp3".to_owned()]],
            pairs: Vec::new(),
        }
    }

    fn found(asset: &str, start: f64, end: f64, confident: bool) -> Found {
        Found {
            asset: asset.to_owned(),
            start_seconds: start,
            end_seconds: end,
            speed: 1.0,
            windows: 10,
            hits: 500,
            confident,
        }
    }

    #[test]
    fn a_detection_inside_the_window_identifies_its_track() {
        let detections = [
            found("a.mp3", 10.0, 280.0, true),
            found("c.mp3", 610.0, 900.0, true),
        ];

        let score = score(&set(), 900.0, &detections, &clusters());

        assert_eq!((score.referenced, score.identified, score.wrong), (2, 2, 0));
    }

    #[test]
    fn the_window_extends_ninety_seconds_past_the_next_start() {
        // Track 1 ends where track 2 starts (300 s); its window ends at 390 s.
        let late = [found("a.mp3", 385.0, 500.0, true)];
        let too_late = [found("a.mp3", 391.0, 500.0, true)];

        assert_eq!(score(&set(), 900.0, &late, &clusters()).identified, 1);
        let missed = score(&set(), 900.0, &too_late, &clusters());
        assert_eq!((missed.identified, missed.wrong), (0, 1));
    }

    #[test]
    fn a_cluster_member_is_credited_and_listed_separately() {
        let detections = [found("c-rip.mp3", 620.0, 800.0, true)];

        let score = score(&set(), 900.0, &detections, &clusters());

        assert_eq!(score.identified, 1);
        assert_eq!(score.tracks[2].credited_through_cluster, ["c-rip.mp3"]);
    }

    #[test]
    fn anything_in_an_absent_slot_is_wrong() {
        let detections = [
            found("x.mp3", 320.0, 500.0, true),
            found("y.mp3", 320.0, 500.0, false),
        ];

        let score = score(&set(), 900.0, &detections, &clusters());

        assert_eq!(score.wrong, 1);
        assert_eq!(score.wrong_identifications[0].asset, "x.mp3");
    }

    #[test]
    fn a_weak_detection_is_evidence_but_not_an_identification() {
        let detections = [found("a.mp3", 10.0, 100.0, false)];

        let score = score(&set(), 900.0, &detections, &clusters());

        assert_eq!(score.identified, 0);
        assert!(score.tracks[0].strongest_candidate.is_some());
    }
}

//! Scoring the detections of a set against its manifest.
//!
//! A referenced track is identified when a confident detection of one of its
//! references, or of an asset in the same duplicate cluster, overlaps its
//! window: from `TOLERANCE` before its start to `TOLERANCE` after the next
//! track's start. Every other confident detection is a wrong identification,
//! including any in a slot marked absent.
//!
//! Possible plays are scored separately and never change the identified and
//! wrong counts (ADR 0006): a referenced track that is not identified but has
//! a play of an accepted asset in its window was found as possible, and a
//! possible play that matches no track is a false alarm of that tier.

use std::collections::BTreeSet;

use gunfinger_core::confidence::{Confidence, Evidence, Pass};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::manifest::{Set, Track};
use crate::verifier::Verification;

/// Manifest start times are approximate and neighbouring tracks overlap.
pub const TOLERANCE_SECONDS: f64 = 90.0;

/// A detection as the scorer sees it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Found {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speed: f64,
    pub windows: u32,
    pub hits: u32,
    pub confident: bool,
    /// Counted by the second pass (`Pass::Fitted`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fitted: bool,
    /// The peak verifier's measures, with `--verify`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub verified: Vec<Verification>,
}

impl Found {
    pub fn evidence(&self) -> Evidence {
        evidence(self.windows, self.hits, self.fitted)
    }
}

/// Evidence as reports store it: the counts, and whether the second pass
/// counted them.
pub fn evidence(windows: u32, hits: u32, fitted: bool) -> Evidence {
    Evidence {
        windows,
        hits,
        pass: if fitted { Pass::Fitted } else { Pass::Ladder },
    }
}

/// A play (`gunfinger_core::plays`) as the scorer sees it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundPlay {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub segments: Vec<Found>,
}

impl FoundPlay {
    /// Recomputed from the segments' evidence with the current thresholds,
    /// so a stored report can be re-examined under a changed rule.
    pub fn confidence(&self) -> Confidence {
        self.segments
            .iter()
            .map(|segment| segment.evidence().confidence())
            .max()
            .unwrap_or(Confidence::Weak)
    }

    pub fn hits(&self) -> u32 {
        self.segments.iter().map(|segment| segment.hits).sum()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Score {
    pub referenced: usize,
    pub identified: usize,
    pub wrong: usize,
    /// Referenced tracks not identified but found as possible.
    pub possible: usize,
    pub tracks: Vec<TrackScore>,
    /// Confident detections credited to no track.
    pub wrong_identifications: Vec<Found>,
    /// The strongest detections, confident or not, that match no track: the
    /// measured null of this scan.
    pub false_candidates: Vec<Found>,
    /// Possible plays that match no track.
    pub unmatched_possible: Vec<FoundPlay>,
}

/// False candidates kept in a score.
const FALSE_CANDIDATES_KEPT: usize = 10;

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackScore {
    pub label: String,
    pub start_seconds: f64,
    pub referenced: bool,
    pub identified: bool,
    /// Not identified, but a confident or possible play of an accepted asset
    /// overlaps the window.
    pub possible: bool,
    /// Confident detections that identify this track.
    pub credited: Vec<Found>,
    /// Credited detections of an asset that is not itself a reference but
    /// shares a duplicate cluster with one: listed for the owner to confirm.
    pub credited_through_cluster: Vec<String>,
    /// The strongest detection of a reference or a cluster member inside the
    /// window, confident or not: the evidence for a miss.
    pub strongest_candidate: Option<Found>,
}

/// A track's window of the mix and the assets that may be credited to it.
struct Slot<'a> {
    track: &'a Track,
    start_seconds: f64,
    end_seconds: f64,
    accepted: BTreeSet<String>,
}

impl Slot<'_> {
    fn covers(&self, asset: &str, start_seconds: f64, end_seconds: f64) -> bool {
        start_seconds < self.end_seconds
            && self.start_seconds < end_seconds
            && self.accepted.contains(asset)
    }
}

fn slots<'a>(set: &'a Set, duration_seconds: f64, clusters: &Clusters) -> Vec<Slot<'a>> {
    set.tracks
        .iter()
        .enumerate()
        .map(|(position, track)| {
            let next_start = set
                .tracks
                .get(position + 1)
                .map_or(duration_seconds, |next| next.start.as_secs_f64());
            Slot {
                track,
                start_seconds: track.start.as_secs_f64() - TOLERANCE_SECONDS,
                end_seconds: next_start + TOLERANCE_SECONDS,
                accepted: track
                    .references
                    .iter()
                    .flat_map(|reference| clusters.cluster_of(reference))
                    .collect(),
            }
        })
        .collect()
}

pub fn score(
    set: &Set,
    duration_seconds: f64,
    found: &[Found],
    plays: &[FoundPlay],
    clusters: &Clusters,
) -> Score {
    let slots = slots(set, duration_seconds, clusters);
    let tracks: Vec<TrackScore> = slots
        .iter()
        .map(|slot| score_track(slot, found, plays))
        .collect();

    let mut unmatched: Vec<&Found> = found
        .iter()
        .filter(|detection| {
            !slots.iter().any(|slot| {
                slot.covers(
                    &detection.asset,
                    detection.start_seconds,
                    detection.end_seconds,
                )
            })
        })
        .collect();
    unmatched.sort_by_key(|detection| std::cmp::Reverse(detection.hits));
    let wrong_identifications: Vec<Found> = unmatched
        .iter()
        .filter(|detection| detection.confident)
        .map(|detection| (*detection).clone())
        .collect();
    let unmatched_possible = plays
        .iter()
        .filter(|play| {
            play.confidence() == Confidence::Possible
                && !slots
                    .iter()
                    .any(|slot| slot.covers(&play.asset, play.start_seconds, play.end_seconds))
        })
        .cloned()
        .collect();

    Score {
        referenced: tracks.iter().filter(|track| track.referenced).count(),
        identified: tracks.iter().filter(|track| track.identified).count(),
        wrong: wrong_identifications.len(),
        possible: tracks.iter().filter(|track| track.possible).count(),
        tracks,
        wrong_identifications,
        false_candidates: unmatched
            .into_iter()
            .take(FALSE_CANDIDATES_KEPT)
            .cloned()
            .collect(),
        unmatched_possible,
    }
}

fn score_track(slot: &Slot, found: &[Found], plays: &[FoundPlay]) -> TrackScore {
    let track = slot.track;
    let in_slot: Vec<&Found> = found
        .iter()
        .filter(|detection| {
            slot.covers(
                &detection.asset,
                detection.start_seconds,
                detection.end_seconds,
            )
        })
        .collect();
    let credited: Vec<Found> = in_slot
        .iter()
        .filter(|detection| detection.confident)
        .map(|detection| (*detection).clone())
        .collect();
    let through_cluster: BTreeSet<String> = credited
        .iter()
        .filter(|detection| !track.references.contains(&detection.asset))
        .map(|detection| detection.asset.clone())
        .collect();
    let mut strongest_candidate: Option<&Found> = None;
    for detection in in_slot {
        if strongest_candidate.is_none_or(|strongest| detection.hits > strongest.hits) {
            strongest_candidate = Some(detection);
        }
    }
    let identified = track.is_referenced() && !credited.is_empty();
    let possible = track.is_referenced()
        && !identified
        && plays
            .iter()
            .any(|play| slot.covers(&play.asset, play.start_seconds, play.end_seconds));

    TrackScore {
        label: track.label(),
        start_seconds: track.start.as_secs_f64(),
        referenced: track.is_referenced(),
        identified,
        possible,
        credited,
        credited_through_cluster: through_cluster.into_iter().collect(),
        strongest_candidate: strongest_candidate.cloned(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::Duration;

    use gunfinger_core::confidence::MIN_POSSIBLE_HITS;

    use super::*;

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
            fitted: false,
            verified: Vec::new(),
        }
    }

    fn possible_play(asset: &str, start: f64, end: f64) -> FoundPlay {
        FoundPlay {
            asset: asset.to_owned(),
            start_seconds: start,
            end_seconds: end,
            segments: vec![Found {
                hits: MIN_POSSIBLE_HITS,
                windows: 2,
                ..found(asset, start, end, false)
            }],
        }
    }

    #[test]
    fn a_detection_inside_the_window_identifies_its_track() {
        let detections = [
            found("a.mp3", 10.0, 280.0, true),
            found("c.mp3", 610.0, 900.0, true),
        ];

        let score = score(&set(), 900.0, &detections, &[], &clusters());

        assert_eq!((score.referenced, score.identified, score.wrong), (2, 2, 0));
    }

    #[test]
    fn the_window_extends_ninety_seconds_past_the_next_start() {
        // Track 1 ends where track 2 starts (300 s); its window ends at 390 s.
        let late = [found("a.mp3", 385.0, 500.0, true)];
        let too_late = [found("a.mp3", 391.0, 500.0, true)];

        assert_eq!(score(&set(), 900.0, &late, &[], &clusters()).identified, 1);
        let missed = score(&set(), 900.0, &too_late, &[], &clusters());
        assert_eq!((missed.identified, missed.wrong), (0, 1));
    }

    #[test]
    fn a_cluster_member_is_credited_and_listed_separately() {
        let detections = [found("c-rip.mp3", 620.0, 800.0, true)];

        let score = score(&set(), 900.0, &detections, &[], &clusters());

        assert_eq!(score.identified, 1);
        assert_eq!(score.tracks[2].credited_through_cluster, ["c-rip.mp3"]);
    }

    #[test]
    fn anything_in_an_absent_slot_is_wrong() {
        let detections = [
            found("x.mp3", 320.0, 500.0, true),
            found("y.mp3", 320.0, 500.0, false),
        ];

        let score = score(&set(), 900.0, &detections, &[], &clusters());

        assert_eq!(score.wrong, 1);
        assert_eq!(score.wrong_identifications[0].asset, "x.mp3");
        assert_eq!(score.false_candidates.len(), 2);
    }

    #[test]
    fn a_weak_detection_is_evidence_but_not_an_identification() {
        let detections = [found("a.mp3", 10.0, 100.0, false)];

        let score = score(&set(), 900.0, &detections, &[], &clusters());

        assert_eq!(score.identified, 0);
        assert!(score.tracks[0].strongest_candidate.is_some());
    }

    #[test]
    fn possible_plays_are_counted_apart_from_identifications() {
        let plays = [
            possible_play("a.mp3", 10.0, 100.0),
            possible_play("x.mp3", 320.0, 400.0),
        ];

        let score = score(&set(), 900.0, &[], &plays, &clusters());

        assert_eq!((score.identified, score.wrong, score.possible), (0, 0, 1));
        assert!(score.tracks[0].possible);
        assert_eq!(score.unmatched_possible.len(), 1);
        assert_eq!(score.unmatched_possible[0].asset, "x.mp3");
    }
}

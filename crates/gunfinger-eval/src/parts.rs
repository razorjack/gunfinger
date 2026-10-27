//! Library files that may hold more than one recording: a track of a mixed
//! CD that blends into its neighbours, an upload cut from a mix, an upload
//! that opens with another record. In each, a stretch at the file's start
//! or end lines up with another recording somewhere inside it, and the
//! rest of the file is a different recording. Such a file joins unrelated
//! recordings in the clusters and would print a wrong title in a
//! tracklist; the user decides by ear which to leave out (ADR 0009).
//!
//! Each query file's stored peaks are searched against the other library's
//! index, as `clusters` searches: by default only its first and last
//! `WINDOW_SECONDS`, which is where such a stretch lies. Each detection
//! of another file is measured again over the whole query against that file
//! alone, at its speed, and a file with an edge part is searched again
//! whole, so that every part of it is listed. The rules and thresholds are
//! the pre-registration's (`docs/research/2026-10-27-files-holding-several-recordings.md`).
//! A candidate is a question for the ear, not a verdict.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use gunfinger_core::confidence::FITTED_RULE;
use gunfinger_core::index::Index;
use gunfinger_core::indexing::build_index;
use gunfinger_core::library::Asset;
use gunfinger_core::library::ignore::{Pattern, leaving_out};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, SKIPPED_SHARE, search_peaks};
use gunfinger_core::speed::{self, Playback, Rung};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::{ANY_LENGTH, Clusters, MIN_COVERAGE, MIN_HITS_TO_MEASURE};
use crate::padding::SECOND_LIBRARY_PREFIX;

/// The seconds searched at each end of a file.
const WINDOW_SECONDS: f64 = 60.0;
/// A file shorter than this is searched whole: its two windows would
/// leave little between them.
const WHOLE_BELOW_SECONDS: f64 = 150.0;
/// An edge part starts or ends this close to the file's start or end.
const EDGE_SECONDS: f64 = 15.0;
/// A part long and strong enough to be a play.
const MIN_PART_SECONDS: f64 = 20.0;
const MIN_PART_HITS: u32 = FITTED_RULE.min_possible_hits;
/// An edge part leaves at least this much of the file outside it.
const MIN_REST_SECONDS: f64 = 30.0;
/// The found file holds at least this much before an edge part at the
/// file's start, or after one at its end: a transition plays the outgoing
/// record past its start and the incoming one before its end, where a
/// remix sharing an intro or outro lines up start to start or end to end.
const MIN_INSIDE_SECONDS: f64 = 30.0;
/// A second part reaches at least this much further into the file than
/// the edge part.
const MIN_BEYOND_SECONDS: f64 = 20.0;
/// An edge part this strong is worth the ear without a second part.
const STRONG_EDGE_HITS: u32 = 100;

/// How much of each query file the first search covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Searched {
    /// Its first and last `WINDOW_SECONDS`, then the whole file if it has
    /// an edge part.
    Ends,
    Whole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub criterion: String,
    pub searched: String,
    /// Query files searched.
    pub queries: usize,
    /// Query files left out: matched by the ignore file or the leave-out
    /// list, or without a current record.
    pub left_out: Vec<String>,
    /// Strongest first.
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub file: String,
    pub seconds: f64,
    /// The hits of the strongest edge part.
    pub strength: u32,
    /// An edge part and a second part elsewhere, or an edge part of
    /// `STRONG_EDGE_HITS` or more.
    pub worth_the_ear: bool,
    /// Every part, in file order.
    pub parts: Vec<Part>,
}

/// A stretch of the query file that lines up with another file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Part {
    pub found: String,
    pub found_seconds: f64,
    /// The first member of the found file's duplicate cluster, or the file
    /// itself when it has none.
    pub cluster: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub found_start_seconds: f64,
    pub found_end_seconds: f64,
    pub speed: f64,
    pub hits: u32,
    pub role: Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// An edge part at the file's start.
    Start,
    /// An edge part at the file's end.
    End,
    /// A second part elsewhere, of another recording than an edge part's.
    Elsewhere,
    /// Neither: listed for the ear.
    Other,
}

impl Part {
    fn seconds(&self) -> f64 {
        self.end_seconds - self.start_seconds
    }

    /// The part's role as an edge part of a file `length` seconds long.
    fn edge(&self, length: f64) -> Option<Role> {
        let play = self.seconds() >= MIN_PART_SECONDS && self.hits >= MIN_PART_HITS;
        let not_a_copy =
            self.seconds() < MIN_COVERAGE * length && length - self.seconds() >= MIN_REST_SECONDS;
        if !(play && not_a_copy) {
            return None;
        }
        if self.start_seconds <= EDGE_SECONDS && self.found_start_seconds >= MIN_INSIDE_SECONDS {
            Some(Role::Start)
        } else if self.end_seconds >= length - EDGE_SECONDS
            && self.found_seconds - self.found_end_seconds >= MIN_INSIDE_SECONDS
        {
            Some(Role::End)
        } else {
            None
        }
    }

    /// Whether the part is a second part beside `edge`, an edge part.
    fn elsewhere_of(&self, edge: &Part) -> bool {
        let play = self.seconds() >= MIN_PART_SECONDS && self.hits >= MIN_PART_HITS;
        let other_recording = self.found != edge.found && self.cluster != edge.cluster;
        let further = match edge.role {
            Role::Start => self.end_seconds >= edge.end_seconds + MIN_BEYOND_SECONDS,
            Role::End => self.start_seconds <= edge.start_seconds - MIN_BEYOND_SECONDS,
            Role::Elsewhere | Role::Other => false,
        };
        play && other_recording && further
    }
}

/// The candidate a file of `seconds` makes from its parts, or `None`
/// without an edge part.
pub fn assess(file: &str, seconds: f64, mut parts: Vec<Part>) -> Option<Candidate> {
    for part in &mut parts {
        part.role = part.edge(seconds).unwrap_or(Role::Other);
    }
    let edges: Vec<Part> = parts
        .iter()
        .filter(|part| matches!(part.role, Role::Start | Role::End))
        .cloned()
        .collect();
    let strength = edges.iter().map(|part| part.hits).max()?;
    for part in &mut parts {
        if part.role == Role::Other && edges.iter().any(|edge| part.elsewhere_of(edge)) {
            part.role = Role::Elsewhere;
        }
    }
    let second_part = parts.iter().any(|part| part.role == Role::Elsewhere);
    parts.sort_by(|a, b| {
        a.start_seconds
            .total_cmp(&b.start_seconds)
            .then(b.hits.cmp(&a.hits))
    });
    Some(Candidate {
        file: file.to_owned(),
        seconds,
        strength,
        worth_the_ear: second_part || strength >= STRONG_EDGE_HITS,
        parts,
    })
}

/// The stretches of `peaks` the first search covers, each with the time
/// it starts at; peak times are counted from the stretch's start, on whole
/// frames, so that detections shift back by exactly that time.
fn stretches(peaks: &[Peak], seconds: f64, searched: Searched) -> Vec<(f64, Vec<Peak>)> {
    if searched == Searched::Whole || seconds < WHOLE_BELOW_SECONDS {
        return vec![(0.0, peaks.to_vec())];
    }
    let profile = Profile::CURRENT;
    let last = profile.frames(seconds - WINDOW_SECONDS).floor();
    let first_end = profile.frames(WINDOW_SECONDS);
    let head: Vec<Peak> = peaks
        .iter()
        .filter(|peak| peak.frame < first_end)
        .copied()
        .collect();
    let tail: Vec<Peak> = peaks
        .iter()
        .filter(|peak| peak.frame >= last)
        .map(|peak| Peak {
            frame: peak.frame - last,
            ..*peak
        })
        .collect();
    vec![(0.0, head), (profile.seconds(last), tail)]
}

/// `detection`, found in a stretch starting at `offset`, in the whole
/// file's time.
fn shifted(detection: &Detection, offset: f64) -> Detection {
    Detection {
        start_seconds: detection.start_seconds + offset,
        end_seconds: detection.end_seconds + offset,
        ..detection.clone()
    }
}

/// The whole query measured again against the found file alone, at the
/// detection's speed and in every posting list: the strongest alignment
/// that overlaps `detection` in the query, or `detection` when none does.
fn measured_again(query: &[Peak], found: &PeakRecord, detection: &Detection) -> Detection {
    let profile = Profile::CURRENT;
    let Ok(index) = Index::build(std::slice::from_ref(found)) else {
        return detection.clone();
    };
    let rung = [match detection.playback {
        Playback::Turntable => Rung::Turntable(detection.speed),
        Playback::KeyLocked => Rung::KeyLocked(detection.speed),
    }];
    search_peaks(&index, query, &profile, &rung, 1)
        .into_iter()
        .filter(|again| {
            again.start_seconds < detection.end_seconds
                && detection.start_seconds < again.end_seconds
        })
        .max_by_key(|again| again.evidence.hits)
        .unwrap_or_else(|| detection.clone())
}

/// What the parts search reads: the other library's index and records,
/// and its clusters.
struct Library<'a> {
    index: Index,
    store: &'a PeakStore,
    by_path: BTreeMap<&'a str, &'a Asset>,
    cluster_of: BTreeMap<String, String>,
}

impl Library<'_> {
    /// The parts of `record` that `stretches` of it find, each measured
    /// again over the whole file, one per alignment.
    fn parts(&self, record: &PeakRecord, searched: Searched) -> Result<Vec<Part>, String> {
        let profile = Profile::CURRENT;
        let ladder = speed::ladder();
        let query = &record.header.source.path;
        let mut parts: Vec<Part> = Vec::new();
        for (offset, peaks) in stretches(&record.peaks, record.header.duration_seconds, searched) {
            for detection in search_peaks(&self.index, &peaks, &profile, &ladder, 1) {
                if detection.evidence.hits < MIN_HITS_TO_MEASURE {
                    continue;
                }
                let found = &self.index.asset(detection.asset).path;
                if found == query {
                    continue;
                }
                let Some(&asset) = self.by_path.get(found.as_str()) else {
                    continue;
                };
                let found_record = self
                    .store
                    .load(asset, &profile)
                    .map_err(|error| error.to_string())?;
                let measured =
                    measured_again(&record.peaks, &found_record, &shifted(&detection, offset));
                let part = self.part(found, found_record.header.duration_seconds, &measured);
                if !parts.contains(&part) {
                    parts.push(part);
                }
            }
        }
        Ok(parts)
    }

    fn part(&self, found: &str, found_seconds: f64, detection: &Detection) -> Part {
        let named = format!("{SECOND_LIBRARY_PREFIX}{found}");
        Part {
            cluster: self
                .cluster_of
                .get(&named)
                .cloned()
                .unwrap_or_else(|| named.clone()),
            found: named,
            found_seconds,
            start_seconds: detection.start_seconds,
            end_seconds: detection.end_seconds,
            found_start_seconds: detection.track_start_seconds,
            found_end_seconds: detection.track_end_seconds,
            speed: detection.speed.0,
            hits: detection.evidence.hits,
            role: Role::Other,
        }
    }

    /// The candidate `record` makes, or `None`.
    fn candidate(
        &self,
        record: &PeakRecord,
        searched: Searched,
    ) -> Result<Option<Candidate>, String> {
        let file = format!("{SECOND_LIBRARY_PREFIX}{}", record.header.source.path);
        let seconds = record.header.duration_seconds;
        let first = self.parts(record, searched)?;
        let whole_already = searched == Searched::Whole || seconds < WHOLE_BELOW_SECONDS;
        if whole_already {
            return Ok(assess(&file, seconds, first));
        }
        if assess(&file, seconds, first).is_none() {
            return Ok(None);
        }
        Ok(assess(&file, seconds, self.parts(record, Searched::Whole)?))
    }
}

/// The files of `queries` (store paths) that are left out: matched by
/// `ignore`, equal to or inside a path of `leave_out`, or without a
/// current record in `assets`.
pub fn left_out(
    queries: &[String],
    assets: &[Asset],
    ignore: &[Pattern],
    leave_out: &[String],
) -> Vec<String> {
    let current: BTreeSet<&str> = assets.iter().map(|asset| asset.path.as_str()).collect();
    queries
        .iter()
        .filter(|query| {
            leaving_out(ignore, query).is_some()
                || leave_out.iter().any(|path| {
                    *query == path
                        || query
                            .strip_prefix(path.as_str())
                            .is_some_and(|rest| rest.starts_with('/'))
                })
                || !current.contains(query.as_str())
        })
        .cloned()
        .collect()
}

/// Search each of `queries` (store paths of `store`) that `left_out`
/// keeps, against an index of all of `assets`.
pub fn find(
    store: &PeakStore,
    assets: &[Asset],
    clusters: &Clusters,
    queries: &[String],
    left_out: Vec<String>,
    searched: Searched,
    jobs: usize,
) -> Result<Report, String> {
    let profile = Profile::CURRENT;
    let built = build_index(assets, store, &profile, &BTreeSet::new(), ANY_LENGTH, jobs)
        .map_err(|error| error.to_string())?;
    for problem in &built.problems {
        eprintln!("left out of the index: {problem}");
    }
    let library = Library {
        index: built.index.skipping_fullest(SKIPPED_SHARE),
        store,
        by_path: assets
            .iter()
            .map(|asset| (asset.path.as_str(), asset))
            .collect(),
        cluster_of: clusters
            .duplicates
            .iter()
            .flat_map(|members| {
                members
                    .iter()
                    .map(|member| (member.clone(), members[0].clone()))
            })
            .collect(),
    };
    let skipped: BTreeSet<&str> = left_out.iter().map(String::as_str).collect();
    let searched_files: Vec<&Asset> = queries
        .iter()
        .filter(|query| !skipped.contains(query.as_str()))
        .filter_map(|query| library.by_path.get(query.as_str()).copied())
        .collect();
    let finished = AtomicUsize::new(0);
    let found = map_in_order(&searched_files, jobs, |&asset| {
        let record = store
            .load(asset, &profile)
            .map_err(|error| error.to_string())?;
        let candidate = library.candidate(&record, searched);
        let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("[{count}/{}] {}", searched_files.len(), asset.path);
        candidate
    });
    let mut candidates = Vec::new();
    for candidate in found {
        candidates.extend(candidate?);
    }
    candidates.sort_by(|a, b| b.strength.cmp(&a.strength).then(a.file.cmp(&b.file)));
    Ok(Report {
        criterion: criterion(),
        searched: match searched {
            Searched::Ends => format!(
                "first and last {WINDOW_SECONDS} s, whole below {WHOLE_BELOW_SECONDS} s and when an edge part is found"
            ),
            Searched::Whole => String::from("whole"),
        },
        queries: searched_files.len(),
        left_out,
        candidates,
    })
}

fn criterion() -> String {
    format!(
        "parts of {MIN_HITS_TO_MEASURE} hits or more, measured again alone; an edge part: {MIN_PART_SECONDS} s and {MIN_PART_HITS} hits or more, within {EDGE_SECONDS} s of the file's start or end, under {MIN_COVERAGE} of the file and leaving {MIN_REST_SECONDS} s of it, with {MIN_INSIDE_SECONDS} s of the found file before it (at the start) or after it (at the end); a second part elsewhere: another cluster, {MIN_PART_SECONDS} s and {MIN_PART_HITS} hits, {MIN_BEYOND_SECONDS} s further in; worth the ear: a second part, or an edge part of {STRONG_EDGE_HITS} hits"
    )
}

pub fn print_summary(report: &Report) {
    let worth = report
        .candidates
        .iter()
        .filter(|candidate| candidate.worth_the_ear)
        .count();
    println!(
        "{} files searched ({}), {} left out: {} candidates, {worth} worth the ear",
        report.queries,
        report.searched,
        report.left_out.len(),
        report.candidates.len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A part of `found` over `start`-`end` of the file, at `found_start`
    /// of a found file `found_seconds` long, at speed 1.
    fn part(
        found: &str,
        (start, end): (f64, f64),
        found_start: f64,
        found_seconds: f64,
        hits: u32,
    ) -> Part {
        Part {
            found: found.to_owned(),
            found_seconds,
            cluster: found.to_owned(),
            start_seconds: start,
            end_seconds: end,
            found_start_seconds: found_start,
            found_end_seconds: found_start + (end - start),
            speed: 1.0,
            hits,
            role: Role::Other,
        }
    }

    fn roles(candidate: &Candidate) -> Vec<(&str, Role)> {
        candidate
            .parts
            .iter()
            .map(|part| (part.found.as_str(), part.role))
            .collect()
    }

    #[test]
    fn a_file_that_opens_with_the_end_of_another_record_is_a_candidate() {
        // The track-fix Drumtrip's saved pairs: Propaganda's 278-318 s
        // over its first 40 s, Known Unknown over 10-175 s.
        let parts = vec![
            part("known-unknown.m4a", (10.0, 175.0), 55.0, 260.0, 1_860),
            part("propaganda.m4a", (1.0, 40.0), 278.0, 360.0, 207),
        ];

        let candidate = assess("drumtrip.mp3", 190.0, parts).unwrap();

        assert_eq!(candidate.strength, 207);
        assert!(candidate.worth_the_ear);
        assert_eq!(
            roles(&candidate),
            [
                ("propaganda.m4a", Role::Start),
                ("known-unknown.m4a", Role::Elsewhere)
            ]
        );
    }

    #[test]
    fn a_strong_edge_part_alone_is_worth_the_ear_and_a_weak_one_is_listed() {
        // Bacteria's 160-226 s over the first 65 s of a cut; nothing else.
        let strong = vec![part("bacteria.mp3", (1.0, 65.0), 160.0, 420.0, 149)];
        let weak = vec![part("bacteria.mp3", (1.0, 65.0), 160.0, 420.0, 70)];

        let strong = assess("space-cake-cut.m4a", 300.0, strong).unwrap();
        let weak = assess("space-cake-cut.m4a", 300.0, weak).unwrap();

        assert!(strong.worth_the_ear);
        assert_eq!(roles(&strong), [("bacteria.mp3", Role::Start)]);
        assert!(!weak.worth_the_ear);
        assert_eq!(weak.strength, 70);
    }

    #[test]
    fn a_mixed_cd_track_has_edge_parts_at_both_ends() {
        // The previous track's end over the start; the next track's start
        // over the end.
        let parts = vec![
            part("previous.mp3", (0.0, 35.0), 300.0, 335.0, 90),
            part("next.mp3", (330.0, 360.0), 0.0, 400.0, 80),
        ];

        let candidate = assess("track.mp3", 360.0, parts).unwrap();

        assert_eq!(
            roles(&candidate),
            [("previous.mp3", Role::Start), ("next.mp3", Role::End)]
        );
        assert_eq!(candidate.strength, 90);
        assert!(!candidate.worth_the_ear);
    }

    #[test]
    fn a_shared_intro_or_outro_is_not_an_edge_part() {
        // A remix that shares the original's first 40 s, or its last 40 s:
        // start to start, end to end.
        let intro = vec![part("original.mp3", (0.0, 40.0), 0.0, 300.0, 400)];
        let outro = vec![part("original.mp3", (320.0, 360.0), 260.0, 300.0, 400)];

        assert!(assess("remix.mp3", 360.0, intro).is_none());
        assert!(assess("remix.mp3", 360.0, outro).is_none());
    }

    #[test]
    fn a_copy_and_a_short_or_weak_part_are_not_edge_parts() {
        // A copy that starts 40 s into the found file covers 90% of the file.
        let copy = vec![part("rip.mp3", (0.0, 324.0), 40.0, 400.0, 3_000)];
        // A part that leaves less than 30 s of the file.
        let most = vec![part("rip.mp3", (0.0, 40.0), 40.0, 400.0, 300)];
        let short = vec![part("other.mp3", (0.0, 15.0), 100.0, 400.0, 300)];
        let weak = vec![part("other.mp3", (0.0, 40.0), 100.0, 400.0, 59)];
        let inside = vec![part("other.mp3", (30.0, 70.0), 100.0, 400.0, 300)];

        assert!(assess("file.mp3", 360.0, copy).is_none());
        assert!(assess("file.mp3", 65.0, most).is_none());
        assert!(assess("file.mp3", 360.0, short).is_none());
        assert!(assess("file.mp3", 360.0, weak).is_none());
        assert!(assess("file.mp3", 360.0, inside).is_none());
    }

    #[test]
    fn a_second_part_is_another_recording_reaching_further_in() {
        // Another file of the edge part's cluster, from 30 s on.
        let mut same_cluster = part("crash-2.mp3", (30.0, 120.0), 202.0, 400.0, 500);
        same_cluster.cluster = String::from("crash.mp3");
        let parts = vec![
            part("crash.mp3", (0.0, 56.0), 172.0, 400.0, 300),
            same_cluster,
            // Overlaps the transition and reaches only 10 s further.
            part("dub.mp3", (40.0, 66.0), 0.0, 300.0, 90),
            part("t-minus.mp3", (68.0, 234.0), 141.0, 340.0, 356),
        ];

        let candidate = assess("t-minus-vocal.m4a", 300.0, parts).unwrap();

        assert_eq!(
            roles(&candidate),
            [
                ("crash.mp3", Role::Start),
                ("crash-2.mp3", Role::Other),
                ("dub.mp3", Role::Other),
                ("t-minus.mp3", Role::Elsewhere)
            ]
        );
        assert!(candidate.worth_the_ear);
    }

    #[test]
    fn a_second_part_before_an_end_part_counts() {
        let parts = vec![
            part("own-copy.mp3", (0.0, 300.0), 0.0, 300.0, 2_000),
            part("next.mp3", (310.0, 360.0), 0.0, 400.0, 80),
        ];

        let candidate = assess("cut.m4a", 360.0, parts).unwrap();

        assert_eq!(
            roles(&candidate),
            [("own-copy.mp3", Role::Elsewhere), ("next.mp3", Role::End)]
        );
        assert!(candidate.worth_the_ear);
    }

    fn peak(frame: f64) -> Peak {
        Peak {
            frame,
            bin: 100.0,
            magnitude: -3.0,
        }
    }

    #[test]
    fn the_first_search_covers_each_end_counted_from_its_own_start() {
        let profile = Profile::CURRENT;
        let at = |seconds: f64| peak(profile.frames(seconds));
        let peaks = vec![at(1.0), at(59.0), at(61.0), at(200.0), at(241.0), at(299.5)];

        let stretches = stretches(&peaks, 300.0, Searched::Ends);

        let [(head_start, head), (tail_start, tail)] = &stretches[..] else {
            panic!("{} stretches", stretches.len());
        };
        assert_eq!(*head_start, 0.0);
        assert_eq!(head, &[at(1.0), at(59.0)]);
        // The tail starts on a whole frame at or before 240 s.
        assert!(*tail_start <= 240.0 && *tail_start > 239.9, "{tail_start}");
        let frames: Vec<f64> = tail.iter().map(|peak| peak.frame).collect();
        let first = profile.frames(*tail_start);
        assert_eq!(first.fract(), 0.0);
        assert_eq!(frames, [at(241.0).frame - first, at(299.5).frame - first]);
    }

    #[test]
    fn a_short_file_and_a_whole_search_cover_the_whole_file() {
        let peaks = vec![peak(10.0), peak(5_000.0)];

        assert_eq!(
            stretches(&peaks, 140.0, Searched::Ends),
            [(0.0, peaks.clone())]
        );
        assert_eq!(
            stretches(&peaks, 300.0, Searched::Whole),
            [(0.0, peaks.clone())]
        );
    }

    #[test]
    fn queries_are_left_out_by_the_ignore_file_the_list_and_a_missing_record() {
        let asset = |path: &str| Asset {
            path: path.to_owned(),
            size: 1,
            modified: gunfinger_core::library::Timestamp {
                seconds: 0,
                nanos: 0,
            },
        };
        let assets = [
            asset("kept/a.mp3"),
            asset("mixed/b.mp3"),
            asset("mixed-not/c.mp3"),
            asset("ignored/d.mp3"),
        ];
        let ignore = gunfinger_core::library::ignore::parse("ignored/\n").unwrap();
        let queries: Vec<String> = [
            "kept/a.mp3",
            "mixed/b.mp3",
            "mixed-not/c.mp3",
            "ignored/d.mp3",
            "gone/e.mp3",
        ]
        .map(String::from)
        .to_vec();

        let left = left_out(&queries, &assets, &ignore, &[String::from("mixed")]);

        assert_eq!(left, ["mixed/b.mp3", "ignored/d.mp3", "gone/e.mp3"]);
    }
}

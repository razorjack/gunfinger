//! `questions parts`: is a library file one recording throughout, or does
//! it change record? One question per candidate of a `parts` report: the
//! file is the timeline's reference lane A, and each recording that lines
//! up with an edge part or a second part elsewhere has a lane, its
//! strongest file placed by that part's alignment, so that the listener
//! hears where A stops following one recording and follows another.
//! Answers go to the answer log only; the user decides what the ignore
//! file leaves out (ADR 0009), and nothing reads the answers (ADR 0017).

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

use gunfinger_core::timecode::format_timecode;

use crate::audition::names::Libraries;
use crate::audition::placement::{self, Alignment};
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Span, Timeline,
};
use crate::parts::{Candidate, Part, Report, Role};

/// Lanes after A: the page switches lanes with the keys 1 to 9.
const MAX_PART_LANES: usize = 8;

pub fn load(path: &Path) -> Result<Report, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
}

/// One question per file of `files`, in that order, each a candidate of
/// `reports`; without `files`, every candidate worth the ear, in the
/// reports' order.
pub fn questions(
    reports: &[Report],
    files: Option<&[String]>,
    libraries: &Libraries,
) -> Result<QuestionFile, String> {
    let candidates: BTreeMap<&str, &Candidate> = reports
        .iter()
        .flat_map(|report| &report.candidates)
        .map(|candidate| (candidate.file.as_str(), candidate))
        .collect();
    let asked: Vec<&Candidate> = match files {
        Some(files) => files
            .iter()
            .map(|file| {
                candidates
                    .get(file.as_str())
                    .copied()
                    .ok_or_else(|| format!("{file} is not a candidate of the reports"))
            })
            .collect::<Result<_, String>>()?,
        None => reports
            .iter()
            .flat_map(|report| &report.candidates)
            .filter(|candidate| candidate.worth_the_ear)
            .collect(),
    };
    let questions = asked
        .iter()
        .enumerate()
        .map(|(index, candidate)| {
            question(index + 1, candidate, libraries)
                .map_err(|error| format!("{}: {error}", candidate.file))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: String::from("One recording or several"),
        made_by: String::from("gunfinger-eval questions parts"),
        questions,
    })
}

/// The parts that get a lane: each recording's strongest edge part or
/// second part, at most `MAX_PART_LANES` by hits, in file order.
fn laned(candidate: &Candidate) -> Vec<&Part> {
    let mut strongest: BTreeMap<&str, &Part> = BTreeMap::new();
    for part in candidate
        .parts
        .iter()
        .filter(|part| part.role != Role::Other)
    {
        let kept = strongest.entry(part.cluster.as_str()).or_insert(part);
        if part.hits > kept.hits {
            *kept = part;
        }
    }
    let mut parts: Vec<&Part> = strongest.into_values().collect();
    parts.sort_by_key(|part| std::cmp::Reverse(part.hits));
    parts.truncate(MAX_PART_LANES);
    parts.sort_by(|a, b| a.start_seconds.total_cmp(&b.start_seconds));
    parts
}

fn question(
    number: usize,
    candidate: &Candidate,
    libraries: &Libraries,
) -> Result<Question, String> {
    let a = libraries.path(&candidate.file);
    let timeline = (0.0, candidate.seconds);
    let mut lanes = vec![Lane::new(
        "A",
        vec![Piece {
            file: a.clone(),
            start: timeline.0,
            end: timeline.1,
            offset: 0.0,
            speed: 1.0,
            label: None,
        }],
    )];
    let mut markers = Vec::new();
    let mut spans = Vec::new();
    let mut details = vec![format!(
        "A is the file ({}). Each other lane plays a recording at the alignment of its part of A, before and after it too; the shading shows where it lines up.",
        timecode(candidate.seconds)
    )];
    let mut files = vec![candidate.file.clone()];
    for (part, name) in laned(candidate).into_iter().zip('B'..) {
        let name = name.to_string();
        let file = libraries.path(&part.found);
        let alignment = Alignment {
            start: part.start_seconds,
            end: part.end_seconds,
            offset: part.found_start_seconds - part.speed * part.start_seconds,
            speed: part.speed,
            label: format!("{name}'s part"),
            hits: part.hits,
        };
        let lane = placement::aligned_lane(
            &name,
            &file,
            part.found_seconds,
            timeline,
            std::slice::from_ref(&alignment),
        );
        if lane.pieces.is_empty() {
            return Err(format!("lane {name} has no audio on A's timeline"));
        }
        let line = format!(
            "{name}: {} over A {}-{} ({}), from {} {}-{}, {} hits, played at {:+.2}%",
            stem(&file),
            timecode(part.start_seconds),
            timecode(part.end_seconds),
            role(part.role),
            name,
            timecode(part.found_start_seconds),
            timecode(part.found_end_seconds),
            part.hits,
            (part.speed - 1.0) * 100.0,
        );
        markers.push(Marker {
            at: part.start_seconds,
            label: format!("{name} starts lining up"),
            kind: MarkerKind::Listen,
        });
        markers.push(Marker {
            at: part.end_seconds,
            label: format!("{name} stops lining up"),
            kind: MarkerKind::Edge,
        });
        spans.push(Span {
            start: part.start_seconds,
            end: part.end_seconds,
            lanes: vec![0, lanes.len()],
            label: line.clone(),
        });
        details.push(line);
        files.push(part.found.clone());
        lanes.push(lane);
    }
    let others = candidate
        .parts
        .iter()
        .filter(|part| part.role == Role::Other)
        .count();
    if others > 0 {
        details.push(format!(
            "{others} weaker or overlapping parts have no lane (the report lists them)"
        ));
    }
    markers.sort_by(|x, y| x.at.total_cmp(&y.at));

    Ok(Question {
        id: QuestionId::derive("parts", &[&candidate.file]),
        kind: String::from("parts"),
        title: format!("{number}. {}", stem(&a)),
        text: String::from(
            "Is A one recording throughout, or does it change record, so that it holds more than one recording (a cut from a mix, a mixed CD's track, an upload that opens with another record)?",
        ),
        details,
        files,
        timeline: Timeline {
            start: timeline.0,
            end: timeline.1,
            reference: Some(0),
        },
        lanes,
        markers,
        spans,
        choices: vec![
            choice('w', "one recording throughout", "one"),
            choice('c', "it changes record: more than one recording", "several"),
            choice('u', "unsure", "unsure"),
        ],
        effect: Effect::Log,
    })
}

fn role(role: Role) -> &'static str {
    match role {
        Role::Start => "an edge part at A's start",
        Role::End => "an edge part at A's end",
        Role::Elsewhere => "a second part",
        Role::Other => "another part",
    }
}

fn timecode(seconds: f64) -> String {
    format_timecode(Duration::from_secs_f64(seconds.max(0.0)))
}

fn choice(key: char, label: &str, value: &str) -> Choice {
    Choice {
        key,
        label: label.to_owned(),
        value: value.to_owned(),
    }
}

fn stem(path: &Path) -> String {
    path.file_stem().map_or_else(
        || path.display().to_string(),
        |stem| stem.to_string_lossy().into_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::schema::check;
    use std::path::PathBuf;

    /// A cut: Bacteria's 2:40-3:44 over its first 64 s, then two copies of
    /// Space Cake (one cluster) and a weak part with no lane.
    const REPORT: &str = r#"{
      "criterion": "test", "searched": "whole", "queries": 2, "left_out": [],
      "candidates": [
        {"file": "second-library/uploads/Space Cake [CUT].m4a", "seconds": 300.0,
         "strength": 149, "worth_the_ear": true, "parts": [
          {"found": "second-library/eps/Bacteria.mp3", "found_seconds": 420.0,
           "cluster": "second-library/eps/Bacteria.mp3", "start_seconds": 1.0, "end_seconds": 65.0,
           "found_start_seconds": 160.0, "found_end_seconds": 226.0, "speed": 1.03, "hits": 149,
           "role": "start"},
          {"found": "second-library/eps/Space Cake.mp3", "found_seconds": 360.0,
           "cluster": "second-library/eps/Space Cake.mp3", "start_seconds": 50.0, "end_seconds": 299.0,
           "found_start_seconds": 0.0, "found_end_seconds": 249.0, "speed": 1.0, "hits": 900,
           "role": "elsewhere"},
          {"found": "second-library/uploads/Space Cake.m4a", "found_seconds": 361.0,
           "cluster": "second-library/eps/Space Cake.mp3", "start_seconds": 52.0, "end_seconds": 299.0,
           "found_start_seconds": 2.0, "found_end_seconds": 249.0, "speed": 1.0, "hits": 700,
           "role": "elsewhere"},
          {"found": "second-library/eps/Dub.mp3", "found_seconds": 300.0,
           "cluster": "second-library/eps/Dub.mp3", "start_seconds": 100.0, "end_seconds": 110.0,
           "found_start_seconds": 20.0, "found_end_seconds": 30.0, "speed": 1.0, "hits": 31,
           "role": "other"}]},
        {"file": "second-library/eps/Weak.mp3", "seconds": 300.0,
         "strength": 70, "worth_the_ear": false, "parts": [
          {"found": "second-library/eps/Bacteria.mp3", "found_seconds": 420.0,
           "cluster": "second-library/eps/Bacteria.mp3", "start_seconds": 0.0, "end_seconds": 30.0,
           "found_start_seconds": 100.0, "found_end_seconds": 130.0, "speed": 1.0, "hits": 70,
           "role": "start"}]}
      ]
    }"#;

    fn libraries() -> Libraries {
        Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            second: PathBuf::from("/nas/music"),
        }
    }

    #[test]
    fn each_recording_of_a_candidate_has_a_lane_at_its_parts_alignment() {
        let report: Report = serde_json::from_str(REPORT).unwrap();

        let file = questions(&[report], None, &libraries()).unwrap();
        check(&file).unwrap();

        let [question] = &file.questions[..] else {
            panic!("{} questions", file.questions.len());
        };
        assert_eq!(question.kind, "parts");
        assert_eq!(
            question.id,
            QuestionId::derive("parts", &["second-library/uploads/Space Cake [CUT].m4a"])
        );
        assert_eq!(question.title, "1. Space Cake [CUT]");
        assert_eq!(
            question.files,
            [
                "second-library/uploads/Space Cake [CUT].m4a",
                "second-library/eps/Bacteria.mp3",
                "second-library/eps/Space Cake.mp3"
            ]
        );
        assert!(matches!(question.effect, Effect::Log));
        let values: Vec<&str> = question.choices.iter().map(|c| c.value.as_str()).collect();
        assert_eq!(values, ["one", "several", "unsure"]);
        assert_eq!(question.timeline.reference, Some(0));

        let names: Vec<&str> = question
            .lanes
            .iter()
            .map(|lane| lane.name.as_str())
            .collect();
        assert_eq!(names, ["A", "B", "C"]);
        let a = &question.lanes[0].pieces[0];
        assert_eq!(
            a.file,
            PathBuf::from("/nas/music/uploads/Space Cake [CUT].m4a")
        );
        assert_eq!((a.start, a.end, a.offset, a.speed), (0.0, 300.0, 0.0, 1.0));
        // Bacteria at 160 s when A is at 1 s, 1.03 s of it per second of A.
        let b = &question.lanes[1].pieces[0];
        assert_eq!(b.file, PathBuf::from("/nas/music/eps/Bacteria.mp3"));
        assert!((b.offset - (160.0 - 1.03)).abs() < 1e-9, "{b:?}");
        assert_eq!(b.speed, 1.03);
        assert!((b.file_time(65.0) - 225.92).abs() < 1e-9);
        // The cluster's stronger file, not the second copy.
        let c = &question.lanes[2].pieces[0];
        assert_eq!(c.file, PathBuf::from("/nas/music/eps/Space Cake.mp3"));
        assert_eq!(question.spans.len(), 2);
        assert_eq!(question.spans[1].lanes, [0, 2]);
        assert_eq!(question.markers.len(), 4);
        assert!(
            question
                .markers
                .windows(2)
                .all(|pair| pair[0].at <= pair[1].at)
        );
        assert!(
            question
                .details
                .iter()
                .any(|line| line.contains("1 weaker"))
        );
        assert!(question.details[1].starts_with("B: Bacteria over A 0:01-1:05"));
    }

    #[test]
    fn listed_files_are_asked_in_their_order_also_when_not_worth_the_ear() {
        let report: Report = serde_json::from_str(REPORT).unwrap();
        let files = [
            String::from("second-library/eps/Weak.mp3"),
            String::from("second-library/uploads/Space Cake [CUT].m4a"),
        ];

        let file = questions(&[report], Some(&files), &libraries()).unwrap();
        check(&file).unwrap();

        let titles: Vec<&str> = file.questions.iter().map(|q| q.title.as_str()).collect();
        assert_eq!(titles, ["1. Weak", "2. Space Cake [CUT]"]);
    }

    #[test]
    fn a_listed_file_that_is_not_a_candidate_is_an_error() {
        let report: Report = serde_json::from_str(REPORT).unwrap();
        let files = [String::from("second-library/eps/Bacteria.mp3")];

        let error = questions(&[report], Some(&files), &libraries()).unwrap_err();

        assert!(error.contains("is not a candidate"), "{error}");
    }
}

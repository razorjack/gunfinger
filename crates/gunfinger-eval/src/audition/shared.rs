//! `questions shared`: do two different recordings share material where a
//! search lines them up, such as a drum break, a loop or a sample, or is
//! the match chance? One question per pair of a `pair` report: A is the
//! timeline's reference lane, and B is placed by every alignment of the
//! report's strongest audio search, strongest first. Unlike lane B of
//! `questions pack`, it follows alignments below the hits of a possible
//! play (`placement::followed`): they are what the question is about.
//! Answers go to the answer log only, as evaluation ground truth for the
//! strongest unrelated matches the confidence rule and the possible tier
//! are measured against (ADR 0006); the product reads none (ADR 0017).

use std::collections::BTreeSet;

use gunfinger_core::confidence::FITTED_RULE;
use gunfinger_core::timecode::format_seconds;

use crate::audition::names::Libraries;
use crate::audition::placement::{self, Alignment};
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Span, Timeline,
};
use crate::files::stem;
use crate::pair::{PairReport, Search};

/// `pair`'s searches of the audio, as a mix is searched, are named so.
const AUDIO_SEARCH: &str = "audio";

/// Markers closer than this are one stop for Up and Down.
const SAME_MOMENT: f64 = 0.5;

/// One question per pair, in the reports' order.
pub fn questions(reports: &[PairReport], libraries: &Libraries) -> Result<QuestionFile, String> {
    let mut asked = BTreeSet::new();
    let questions = reports
        .iter()
        .enumerate()
        .map(|(index, report)| {
            let named = |error: String| format!("{} ~ {}: {error}", report.query, report.found);
            if !asked.insert(BTreeSet::from([&report.query, &report.found])) {
                return Err(named(String::from(
                    "the pair appears twice in the report (in either order)",
                )));
            }
            question(index + 1, report, libraries).map_err(named)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: String::from("Shared material or chance"),
        made_by: String::from("gunfinger-eval questions shared"),
        questions,
    })
}

fn question(number: usize, report: &PairReport, libraries: &Libraries) -> Result<Question, String> {
    let search = strongest_audio_search(report).ok_or(
        "the report has no alignment from a search of the audio (`pair` searches the audio only when the first file of the pair is in the corpus library)",
    )?;
    let alignments = placed_alignments(search);
    let (a, b) = (libraries.path(&report.query), libraries.path(&report.found));
    let timeline = (0.0, report.query_seconds);
    let b_lane = placement::aligned_lane("B", &b, report.found_seconds, timeline, &alignments);
    if b_lane.pieces.is_empty() {
        return Err(String::from("lane B has no audio on A's timeline"));
    }
    let lanes = vec![
        Lane::new(
            "A",
            vec![Piece {
                file: a.clone(),
                start: timeline.0,
                end: timeline.1,
                offset: 0.0,
                speed: 1.0,
                label: None,
            }],
        ),
        b_lane,
    ];

    let annotated = Annotated::of(search);

    Ok(Question {
        id: QuestionId::derive("shared", &[&report.query, &report.found]),
        kind: String::from("shared"),
        title: format!("{number}. {} ~ {}", stem(&a), stem(&b)),
        text: String::from(
            "A and B are different recordings that line up where the lanes are shaded. Do they share material there, such as a drum break, a loop or a sample, or is the match chance?",
        ),
        details: annotated.details,
        files: vec![report.query.clone(), report.found.clone()],
        timeline: Timeline {
            start: timeline.0,
            end: timeline.1,
            reference: Some(0),
        },
        lanes,
        markers: one_per_moment(annotated.markers),
        spans: annotated.spans,
        choices: vec![
            Choice::new('s', "shared material (a break, loop or sample)", "shared"),
            Choice::new('c', "chance", "chance"),
            Choice::new('u', "unsure", "unsure"),
        ],
        effect: Effect::Log,
    })
}

/// The markers in time order, those less than `SAME_MOMENT` after the one
/// before joined to it.
fn one_per_moment(mut markers: Vec<Marker>) -> Vec<Marker> {
    markers.sort_by(|a, b| a.at.total_cmp(&b.at));
    let mut joined: Vec<Marker> = Vec::new();
    for marker in markers {
        match joined.last_mut() {
            Some(last) if marker.at - last.at < SAME_MOMENT => {
                last.label = format!("{}, {}", last.label, marker.label);
                if marker.kind == MarkerKind::Listen {
                    last.kind = MarkerKind::Listen;
                }
            }
            _ => joined.push(marker),
        }
    }
    joined
}

/// The search's alignments, numbered in the report's order, for
/// `placement`.
fn placed_alignments(search: &Search) -> Vec<Alignment> {
    search
        .alignments
        .iter()
        .enumerate()
        .map(|(index, alignment)| Alignment {
            start: alignment.query_start_seconds,
            end: alignment.query_end_seconds,
            offset: alignment.offset_seconds,
            speed: alignment.speed,
            label: format!("alignment {}", index + 1),
            hits: alignment.hits,
        })
        .collect()
}

/// Where each alignment of the search starts and ends on A's timeline,
/// with a line describing it as a span and a detail.
struct Annotated {
    markers: Vec<Marker>,
    spans: Vec<Span>,
    details: Vec<String>,
}

impl Annotated {
    fn of(search: &Search) -> Annotated {
        let mut markers = Vec::new();
        let mut spans = Vec::new();
        let mut details = vec![format!(
            "B follows every alignment of the search \"{}\" in the pair report, strongest first, also those below the {} hits of a possible play:",
            search.name, FITTED_RULE.min_possible_hits
        )];
        for (position, alignment) in (1..).zip(&search.alignments) {
            let playback = if alignment.key_locked {
                "key-locked: B's tempo differs, not its pitch"
            } else {
                "turntable: pitch and tempo together"
            };
            let line = format!(
                "alignment {position}: A {}-{} ~ B {}-{}, {} hits in {} windows, B played at {:+.2}% ({playback})",
                format_seconds(alignment.query_start_seconds),
                format_seconds(alignment.query_end_seconds),
                format_seconds(alignment.found_start_seconds),
                format_seconds(alignment.found_end_seconds),
                alignment.hits,
                alignment.windows,
                (alignment.speed - 1.0) * 100.0,
            );
            markers.push(Marker {
                at: alignment.query_start_seconds,
                label: format!("alignment {position} starts: {} hits", alignment.hits),
                kind: MarkerKind::Listen,
            });
            markers.push(Marker {
                at: alignment.query_end_seconds,
                label: format!("alignment {position} ends"),
                kind: MarkerKind::Edge,
            });
            if alignment.query_start_seconds < alignment.query_end_seconds {
                spans.push(Span {
                    start: alignment.query_start_seconds,
                    end: alignment.query_end_seconds,
                    lanes: vec![0, 1],
                    label: line.clone(),
                });
            }
            details.push(line);
        }
        if search
            .alignments
            .iter()
            .any(|alignment| alignment.key_locked)
        {
            details.push(String::from(
                "Speed matching (M) plays B at A's tempo by changing its playback rate, so a key-locked alignment also moves B's pitch by the same ratio; turn it off to hear B at its own pitch (the blend, B, always matches speed)",
            ));
        }
        Annotated {
            markers,
            spans,
            details,
        }
    }
}

/// The audio search whose strongest alignment has the most hits (each
/// search lists its alignments strongest first).
fn strongest_audio_search(report: &PairReport) -> Option<&Search> {
    report
        .searches
        .iter()
        .filter(|search| search.name.starts_with(AUDIO_SEARCH) && !search.alignments.is_empty())
        .max_by_key(|search| search.alignments[0].hits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::schema::check;
    use std::path::PathBuf;

    /// Two searches of the audio and one of the peaks: the second audio
    /// search holds the strongest alignment, key-locked, and a weaker one
    /// that overlaps it on A.
    const REPORT: &str = r#"[{
      "query": "Artist - Tune.mp3",
      "found": "second-library/label/Other - Remix.opus",
      "query_seconds": 400.0,
      "found_seconds": 300.0,
      "searches": [
        {"name": "peaks, turntable ladder", "alignments": [
          {"query_start_seconds": 10.0, "query_end_seconds": 40.0, "found_start_seconds": 5.0,
           "found_end_seconds": 35.0, "speed": 1.0, "key_locked": false, "hits": 90, "windows": 3,
           "coverage": 0.1, "offset_seconds": -5.0}]},
        {"name": "audio, both ladders", "alignments": [
          {"query_start_seconds": 200.0, "query_end_seconds": 230.0, "found_start_seconds": 20.0,
           "found_end_seconds": 51.2, "speed": 1.04, "key_locked": false, "hits": 20, "windows": 3,
           "coverage": 0.1, "offset_seconds": -188.0}]},
        {"name": "audio, both ladders, second pass", "alignments": [
          {"query_start_seconds": 112.0, "query_end_seconds": 158.0, "found_start_seconds": 34.4,
           "found_end_seconds": 78.5, "speed": 0.9587, "key_locked": true, "hits": 60, "windows": 5,
           "coverage": 0.13, "offset_seconds": -73.3},
          {"query_start_seconds": 150.0, "query_end_seconds": 190.0, "found_start_seconds": 30.0,
           "found_end_seconds": 68.3, "speed": 0.9585, "key_locked": true, "hits": 41, "windows": 4,
           "coverage": 0.1, "offset_seconds": -113.8}]}
      ]
    }]"#;

    fn libraries() -> Libraries {
        Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            second: PathBuf::from("/nas/music"),
        }
    }

    #[test]
    fn b_follows_every_alignment_of_the_strongest_audio_search() {
        let reports: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();

        let file = questions(&reports, &libraries()).unwrap();
        check(&file).unwrap();

        let [question] = &file.questions[..] else {
            panic!("{} questions", file.questions.len());
        };
        assert_eq!(question.kind, "shared");
        assert_eq!(
            question.id,
            QuestionId::derive(
                "shared",
                &[
                    "Artist - Tune.mp3",
                    "second-library/label/Other - Remix.opus"
                ]
            )
        );
        assert_eq!(
            question.files,
            [
                "Artist - Tune.mp3",
                "second-library/label/Other - Remix.opus"
            ]
        );
        assert_eq!(question.title, "1. Artist - Tune ~ Other - Remix");
        assert!(matches!(question.effect, Effect::Log));
        let values: Vec<&str> = question.choices.iter().map(|c| c.value.as_str()).collect();
        assert_eq!(values, ["shared", "chance", "unsure"]);

        let [a] = &question.lanes[0].pieces[..] else {
            panic!("lane A: {:?}", question.lanes[0].pieces);
        };
        assert_eq!(
            a.file,
            PathBuf::from("/repo/corpus/library/Artist - Tune.mp3")
        );
        assert_eq!((a.start, a.end, a.offset, a.speed), (0.0, 400.0, 0.0, 1.0));
        // Before 112 s, B follows the 60-hit alignment from its own start
        // (B's time 0 at A's 73.3 / 0.9587 s); the 41-hit one takes over
        // where the 60-hit one ends and carries on to A's end.
        let b = &question.lanes[1].pieces;
        assert_eq!(b.len(), 2, "{b:?}");
        assert_eq!(
            b[0].file,
            PathBuf::from("/nas/music/label/Other - Remix.opus")
        );
        assert!((b[0].start - 73.3 / 0.9587).abs() < 1e-9, "{b:?}");
        assert_eq!(b[0].end, 158.0);
        assert_eq!((b[0].offset, b[0].speed), (-73.3, 0.9587));
        assert_eq!((b[1].start, b[1].end), (158.0, 400.0));
        assert_eq!((b[1].offset, b[1].speed), (-113.8, 0.9585));
        // The 60-hit alignment is the main one: B can also play it
        // throughout, or from its start at its speed.
        let main = &question.lanes[1].main.as_deref().unwrap()[0];
        assert_eq!((main.offset, main.speed), (-73.3, 0.9587));
        let from_start = &question.lanes[1].from_start.as_deref().unwrap()[0];
        assert_eq!((from_start.start, from_start.offset), (0.0, 0.0));
        assert_eq!(from_start.speed, 0.9587);
        assert_eq!(question.spans.len(), 2);
        assert_eq!(question.markers.len(), 4);
        assert!(
            question
                .markers
                .windows(2)
                .all(|pair| pair[0].at <= pair[1].at)
        );
        assert!(question.details[0].contains("\"audio, both ladders, second pass\""));
        assert!(
            question
                .details
                .iter()
                .any(|line| line.contains("key-locked"))
        );
        assert!(
            question
                .details
                .iter()
                .any(|line| line.contains("Speed matching (M)"))
        );
    }

    #[test]
    fn markers_less_than_half_a_second_apart_are_one_stop() {
        let mut reports: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();
        let second = &mut reports[0].searches[2].alignments[1];
        second.query_start_seconds = 158.05;
        second.query_end_seconds = 190.0;

        let file = questions(&reports, &libraries()).unwrap();

        let markers = &file.questions[0].markers;
        assert_eq!(markers.len(), 3, "{markers:?}");
        assert_eq!(markers[1].at, 158.0);
        assert_eq!(markers[1].kind, MarkerKind::Listen);
        assert!(markers[1].label.contains("alignment 1 ends"));
        assert!(markers[1].label.contains("alignment 2 starts"));
    }

    #[test]
    fn an_alignment_of_no_length_has_no_span() {
        let mut reports: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();
        let second = &mut reports[0].searches[2].alignments[1];
        second.query_start_seconds = 170.0;
        second.query_end_seconds = 170.0;

        let file = questions(&reports, &libraries()).unwrap();
        check(&file).unwrap();

        assert_eq!(file.questions[0].spans.len(), 1);
    }

    #[test]
    fn a_pair_asked_twice_is_an_error() {
        let mut reports: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();
        let mut reversed: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();
        let pair = &mut reversed[0];
        std::mem::swap(&mut pair.query, &mut pair.found);
        reports.append(&mut reversed);

        let error = questions(&reports, &libraries()).unwrap_err();

        assert!(error.contains("appears twice"), "{error}");
        assert!(error.contains("Other - Remix.opus"), "{error}");
    }

    #[test]
    fn a_pair_without_an_audio_alignment_is_an_error() {
        let mut reports: Vec<PairReport> = serde_json::from_str(REPORT).unwrap();
        reports[0]
            .searches
            .retain(|search| !search.name.starts_with(AUDIO_SEARCH));

        let error = questions(&reports, &libraries()).unwrap_err();

        assert!(
            error.contains("no alignment from a search of the audio"),
            "{error}"
        );
    }
}

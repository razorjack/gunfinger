//! `questions breaks`: does a file that `break-search` lists play the same
//! passage as the query's span, such as the same break, loop or sample,
//! or is the match chance? One question per picked holder (session 86):
//! A is the query file around its span, the timeline's reference lane;
//! B is the holder, placed by the alignment the search found. Answers go
//! to the answer log only, as evaluation ground truth for break search's
//! precision (experiment 0296); the product reads none (ADR 0017).

use std::path::Path;

use gunfinger_core::timecode::format_seconds;
use serde::Deserialize;

use crate::audition::names::Libraries;
use crate::audition::placement::{self, Alignment};
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Span, Timeline,
};
use crate::break_search::Played;
use crate::files::{read_json, stem};

/// Seconds of A played before and after the span.
const CONTEXT_SECONDS: f64 = 10.0;

/// One holder to ask about, picked from a `break-search` report. Unknown
/// fields are refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pick {
    /// The query's name in the report.
    pub query: String,
    /// The query file and its span.
    pub file: String,
    pub file_seconds: f64,
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// The holder and where the search lines it up with the query.
    pub holder: String,
    pub holder_seconds: f64,
    pub query_start_seconds: f64,
    pub query_end_seconds: f64,
    pub found_start_seconds: f64,
    pub found_end_seconds: f64,
    pub hits: u32,
    pub playback: Played,
    /// Why it was picked, such as "rank 2 of 347".
    pub why: String,
}

pub fn load(path: &Path) -> Result<Vec<Pick>, String> {
    read_json(path)
}

/// One question per pick, in the picks' order.
pub fn questions(picks: &[Pick], libraries: &Libraries) -> Result<QuestionFile, String> {
    let questions = picks
        .iter()
        .enumerate()
        .map(|(index, pick)| {
            question(index + 1, pick, libraries)
                .map_err(|error| format!("{} ~ {}: {error}", pick.query, pick.holder))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: String::from("The same passage or chance"),
        made_by: String::from("gunfinger-eval questions breaks"),
        questions,
    })
}

fn question(number: usize, pick: &Pick, libraries: &Libraries) -> Result<Question, String> {
    let (a, b) = (libraries.path(&pick.file), libraries.path(&pick.holder));
    let timeline = (
        (pick.start_seconds - CONTEXT_SECONDS).max(0.0),
        (pick.end_seconds + CONTEXT_SECONDS).min(pick.file_seconds),
    );
    if timeline.0 >= timeline.1 {
        return Err(String::from("the span lies outside the query file"));
    }
    let alignment = alignment(pick)?;
    let b_lane = placement::aligned_lane(
        "B",
        &b,
        pick.holder_seconds,
        timeline,
        std::slice::from_ref(&alignment),
    );
    if b_lane.pieces.is_empty() {
        return Err(String::from("lane B has no audio on A's timeline"));
    }
    let span = format!(
        "A {}-{}, the passage searched",
        format_seconds(pick.start_seconds),
        format_seconds(pick.end_seconds)
    );
    Ok(Question {
        id: QuestionId::derive(
            "break",
            &[
                &pick.file,
                &format!("{:.2}-{:.2}", pick.start_seconds, pick.end_seconds),
                &pick.holder,
            ],
        ),
        kind: String::from("break"),
        title: format!(
            "{number}. {} {} ~ {}",
            stem(&a),
            format_seconds(pick.start_seconds),
            stem(&b)
        ),
        text: String::from(
            "The shaded span of A is a passage a search looked for; B is a file the search lists, placed where it lines up. Does B play the same passage there, such as the same break, loop or sample, or is the match chance?",
        ),
        details: vec![
            format!("{} ({})", pick.query, pick.why),
            format!(
                "A {}-{} ~ B {}-{}, {} hits, B {} at {:+.2}%{}",
                format_seconds(pick.query_start_seconds),
                format_seconds(pick.query_end_seconds),
                format_seconds(pick.found_start_seconds),
                format_seconds(pick.found_end_seconds),
                pick.hits,
                match pick.playback {
                    Played::Turntable => "turntable",
                    Played::KeyLocked => "key-locked",
                },
                (alignment.speed - 1.0) * 100.0,
                match pick.playback {
                    Played::Turntable => "",
                    Played::KeyLocked =>
                        " (key-locked: B plays here at that speed with its pitch moved too)",
                }
            ),
        ],
        files: vec![pick.file.clone(), pick.holder.clone()],
        timeline: Timeline {
            start: timeline.0,
            end: timeline.1,
            reference: Some(0),
        },
        lanes: vec![
            Lane::new(
                "A",
                vec![Piece {
                    file: a,
                    start: timeline.0,
                    end: timeline.1,
                    offset: 0.0,
                    speed: 1.0,
                    label: None,
                }],
            ),
            b_lane,
        ],
        markers: vec![Marker {
            at: pick.start_seconds,
            label: String::from("the passage starts"),
            kind: MarkerKind::Listen,
        }],
        spans: vec![Span {
            start: pick.start_seconds,
            end: pick.end_seconds,
            lanes: vec![0, 1],
            label: span,
        }],
        choices: vec![
            Choice::new('s', "the same passage (break, loop or sample)", "same"),
            Choice::new('c', "chance", "chance"),
            Choice::new('u', "unsure", "unsure"),
        ],
        effect: Effect::Log,
    })
}

/// The alignment as file time = offset + speed × query time, its speed
/// taken from the two spans the search reported.
fn alignment(pick: &Pick) -> Result<Alignment, String> {
    let query = pick.query_end_seconds - pick.query_start_seconds;
    let found = pick.found_end_seconds - pick.found_start_seconds;
    if query <= 0.0 || found <= 0.0 {
        return Err(String::from("the alignment has no length"));
    }
    let speed = found / query;
    Ok(Alignment {
        start: pick.query_start_seconds,
        end: pick.query_end_seconds,
        offset: pick.found_start_seconds - speed * pick.query_start_seconds,
        speed,
        label: String::from("the search's alignment"),
        hits: pick.hits,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::audition::schema::check;

    const PICKS: &str = r#"[
      {"query": "special-branch-250", "file": "second-library/ufo/special_branch.mp3",
       "file_seconds": 400.0, "start_seconds": 250.0, "end_seconds": 270.0,
       "holder": "Ray Keith - Dark Soldier.mp3", "holder_seconds": 420.0,
       "query_start_seconds": 251.0, "query_end_seconds": 269.0,
       "found_start_seconds": 316.0, "found_end_seconds": 334.9,
       "hits": 74, "playback": "turntable", "why": "rank 1 of 204"}
    ]"#;

    fn libraries() -> Libraries {
        Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            other: PathBuf::from("/nas/music"),
        }
    }

    #[test]
    fn b_plays_the_holder_where_the_search_lines_it_up() {
        let picks: Vec<Pick> = serde_json::from_str(PICKS).unwrap();

        let file = questions(&picks, &libraries()).unwrap();
        check(&file).unwrap();

        let [question] = &file.questions[..] else {
            panic!("{} questions", file.questions.len());
        };
        assert_eq!(
            (question.timeline.start, question.timeline.end),
            (240.0, 280.0)
        );
        assert_eq!(
            question.lanes[0].pieces[0].file,
            PathBuf::from("/nas/music/ufo/special_branch.mp3")
        );
        let b = &question.lanes[1].pieces[0];
        assert_eq!(
            b.file,
            PathBuf::from("/repo/corpus/library/Ray Keith - Dark Soldier.mp3")
        );
        assert!((b.file_time(251.0) - 316.0).abs() < 1e-9);
        assert!((b.file_time(269.0) - 334.9).abs() < 1e-9);
        assert_eq!(question.kind, "break");
        assert_eq!(
            question.files,
            [
                "second-library/ufo/special_branch.mp3",
                "Ray Keith - Dark Soldier.mp3"
            ]
        );
    }

    #[test]
    fn the_id_depends_on_the_span_and_the_holder() {
        let picks: Vec<Pick> = serde_json::from_str(PICKS).unwrap();
        let first = questions(&picks, &libraries()).unwrap().questions[0]
            .id
            .clone();
        let mut moved: Vec<Pick> = serde_json::from_str(PICKS).unwrap();
        moved[0].start_seconds = 251.0;

        assert_ne!(
            questions(&moved, &libraries()).unwrap().questions[0].id,
            first
        );
    }
}

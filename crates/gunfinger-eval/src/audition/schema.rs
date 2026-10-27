//! The question file: what the listening page draws and plays, and what an
//! answer does. One schema, versioned; `docs/audition-questions.md`
//! documents every field and `check` enforces every rule written there.
//!
//! The page knows no kinds of question. A question is lanes of pieces on a
//! timeline, markers, shaded spans and choices; producers (`pack`,
//! `folders`, `detections`, `shared`) turn evidence into these.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::files::{read_json, write_text};

pub const SCHEMA_VERSION: u32 = 1;

/// Letters the page uses for its controls; a choice cannot take them.
/// Space, Tab, the digits and the arrows are controls too, but no choice
/// key can be one, as choice keys are letters.
pub const CONTROL_KEYS: [char; 7] = ['b', 'j', 'k', 'l', 'm', 'o', 'z'];

/// Lanes are switched with the keys 1 to 9.
const MAX_LANES: usize = 9;

/// Lanes play at most this far from their natural speed, beyond any
/// turntable's range.
const SPEED_RANGE: (f64, f64) = (0.5, 2.0);

/// A file of questions, as a producer writes it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuestionFile {
    pub schema_version: u32,
    /// A heading for this file's questions in the page's queue.
    pub title: String,
    /// The command that wrote the file.
    pub made_by: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Question {
    pub id: QuestionId,
    pub kind: String,
    pub title: String,
    pub text: String,
    #[serde(default)]
    pub details: Vec<String>,
    /// The files the answer is about, as the harness names them (corpus
    /// library paths, `second-library/<path>`, absolute paths otherwise).
    pub files: Vec<String>,
    pub timeline: Timeline,
    pub lanes: Vec<Lane>,
    #[serde(default)]
    pub markers: Vec<Marker>,
    #[serde(default)]
    pub spans: Vec<Span>,
    pub choices: Vec<Choice>,
    pub effect: Effect,
}

/// `<kind>-<16 hex digits>`, derived from the kind and the files, so that
/// making the questions again keeps their answers attached.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QuestionId(String);

impl QuestionId {
    /// FNV-1a 64 over the kind and the sorted parts, each followed by a NUL
    /// byte. The parts are the question's files, plus a distinguishing part
    /// where one kind can ask twice about the same files.
    pub fn derive(kind: &str, parts: &[&str]) -> QuestionId {
        let mut sorted = parts.to_vec();
        sorted.sort_unstable();
        let mut digest = Fnv1a::new();
        digest.write(kind.as_bytes());
        digest.write(&[0]);
        for part in sorted {
            digest.write(part.as_bytes());
            digest.write(&[0]);
        }
        QuestionId(format!("{kind}-{:016x}", digest.finish()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn is_well_formed(&self, kind: &str) -> bool {
        self.0
            .strip_prefix(kind)
            .and_then(|rest| rest.strip_prefix('-'))
            .is_some_and(|digits| {
                digits.len() == 16 && digits.bytes().all(|byte| byte.is_ascii_hexdigit())
            })
    }
}

/// 64-bit FNV-1a: small and stable across builds, unlike `DefaultHasher`.
pub struct Fnv1a(u64);

impl Fnv1a {
    pub fn new() -> Fnv1a {
        Fnv1a(0xCBF2_9CE4_8422_2325)
    }

    pub fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 ^= u64::from(byte);
            self.0 = self.0.wrapping_mul(0x0100_0000_01B3);
        }
    }

    pub fn finish(&self) -> u64 {
        self.0
    }
}

/// The time axis every lane, marker and span is placed on, in seconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Timeline {
    pub start: f64,
    pub end: f64,
    /// The lane whose own file time the timeline is, when there is one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lane {
    pub name: String,
    /// In timeline order, not overlapping; gaps are silent. What the lane
    /// plays by default: for an aligned lane, every alignment followed.
    pub pieces: Vec<Piece>,
    /// The lane at its main alignment (the most hits) over the whole
    /// timeline, which the page's `O` key switches to: two rips of one
    /// recording then play the same moment end to end.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main: Option<Vec<Piece>>,
    /// The lane's file from its start at the timeline's start, at the main
    /// alignment's speed: the lane with no offset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_start: Option<Vec<Piece>>,
}

impl Lane {
    /// A lane with one placement.
    pub fn new(name: &str, pieces: Vec<Piece>) -> Lane {
        Lane {
            name: name.to_owned(),
            pieces,
            main: None,
            from_start: None,
        }
    }

    /// Each placement the page can play, by its field's name.
    pub fn placements(&self) -> impl Iterator<Item = (&'static str, &[Piece])> {
        [
            ("pieces", Some(&self.pieces)),
            ("main", self.main.as_ref()),
            ("from_start", self.from_start.as_ref()),
        ]
        .into_iter()
        .filter_map(|(name, pieces)| pieces.map(|pieces| (name, pieces.as_slice())))
    }
}

/// A span of one file placed on the timeline: file time =
/// `offset + speed × t` for timeline time `t` in `[start, end)`
/// (`placement` holds the arithmetic).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Piece {
    pub file: PathBuf,
    pub start: f64,
    pub end: f64,
    pub offset: f64,
    /// File seconds per timeline second.
    pub speed: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Marker {
    pub at: f64,
    pub label: String,
    pub kind: MarkerKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarkerKind {
    /// A moment worth hearing.
    Listen,
    /// Where lanes start or stop lining up.
    Edge,
    /// Where a lane goes on in another file or another place of its file.
    Join,
}

/// A shaded stretch of some lanes, such as where two files align.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Span {
    pub start: f64,
    pub end: f64,
    pub lanes: Vec<usize>,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Choice {
    pub key: char,
    pub label: String,
    pub value: String,
}

impl Choice {
    pub fn new(key: char, label: &str, value: &str) -> Choice {
        Choice {
            key,
            label: label.to_owned(),
            value: value.to_owned(),
        }
    }
}

/// Where an answer is written besides the answer log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Effect {
    /// The answer log only.
    Log,
    /// `same` and `different` also go to the pair verdicts file, with the
    /// question's two files; a verdict there on the pair or on a related
    /// pair answers the question.
    PairVerdict {
        #[serde(default)]
        related: Vec<[String; 2]>,
    },
}

/// The choices' values a pair verdict question may have.
pub const PAIR_VALUES: [&str; 3] = ["same", "different", "unsure"];

/// Reads and checks a question file.
pub fn load(path: &Path) -> Result<QuestionFile, String> {
    let file: QuestionFile = read_json(path)?;
    check(&file).map_err(|error| format!("{}: {error}", path.display()))?;
    Ok(file)
}

/// Checks a question file, then writes it as pretty JSON.
pub fn save(path: &Path, file: &QuestionFile) -> Result<(), String> {
    check(file)?;
    let text = serde_json::to_string_pretty(file).map_err(|error| error.to_string())?;
    write_text(path, &(text + "\n"))
}

/// Every rule of the reference that the types do not already enforce.
pub fn check(file: &QuestionFile) -> Result<(), String> {
    if file.schema_version != SCHEMA_VERSION {
        return Err(format!(
            "schema_version {} is not {SCHEMA_VERSION}; make the file again with this build's `gunfinger-eval questions`",
            file.schema_version
        ));
    }
    let mut ids = BTreeSet::new();
    for question in &file.questions {
        check_question(question).map_err(|error| format!("question {}: {error}", question.id.0))?;
        if !ids.insert(&question.id) {
            return Err(format!("question {} appears twice", question.id.0));
        }
    }
    Ok(())
}

fn check_question(question: &Question) -> Result<(), String> {
    let kind = &question.kind;
    if kind.is_empty()
        || !kind
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte == b'-')
    {
        return Err(format!(
            "kind `{kind}` must be lowercase letters and hyphens"
        ));
    }
    if !question.id.is_well_formed(kind) {
        return Err(format!(
            "the id must be `{kind}-` and 16 hex digits (QuestionId::derive)"
        ));
    }
    if question.files.is_empty() {
        return Err(String::from(
            "files lists nothing; the answer log needs them",
        ));
    }
    check_timeline(question)?;
    for (number, lane) in question.lanes.iter().enumerate() {
        check_lane(lane, &question.timeline)
            .map_err(|error| format!("lane {} ({}): {error}", number + 1, lane.name))?;
    }
    let inside = |at: f64| question.timeline.start <= at && at <= question.timeline.end;
    for marker in &question.markers {
        if !marker.at.is_finite() || !inside(marker.at) {
            return Err(format!(
                "marker `{}` at {} lies outside the timeline",
                marker.label, marker.at
            ));
        }
    }
    for span in &question.spans {
        if !(span.start < span.end && inside(span.start) && inside(span.end)) {
            return Err(format!(
                "span `{}` ({}-{}) must run forward inside the timeline",
                span.label, span.start, span.end
            ));
        }
        if let Some(lane) = span
            .lanes
            .iter()
            .find(|&&lane| lane >= question.lanes.len())
        {
            return Err(format!(
                "span `{}` shades lane index {lane}, but there are {} lanes",
                span.label,
                question.lanes.len()
            ));
        }
    }
    check_choices(&question.choices)?;
    check_effect(question)
}

fn check_timeline(question: &Question) -> Result<(), String> {
    let timeline = &question.timeline;
    if !(timeline.start.is_finite() && timeline.end.is_finite() && timeline.start < timeline.end) {
        return Err(format!(
            "the timeline {}-{} must run forward",
            timeline.start, timeline.end
        ));
    }
    if question.lanes.is_empty() || question.lanes.len() > MAX_LANES {
        return Err(format!(
            "{} lanes; a question has 1 to {MAX_LANES}",
            question.lanes.len()
        ));
    }
    if let Some(reference) = timeline.reference {
        let lane = question
            .lanes
            .get(reference)
            .ok_or_else(|| format!("the reference lane index {reference} does not exist"))?;
        if lane
            .pieces
            .iter()
            .any(|piece| piece.offset != 0.0 || piece.speed != 1.0)
        {
            return Err(format!(
                "the reference lane ({}) must play its file's own time: offset 0, speed 1",
                lane.name
            ));
        }
        if lane.placements().count() > 1 {
            return Err(format!(
                "the reference lane ({}) has one placement: no `main` or `from_start`",
                lane.name
            ));
        }
    }
    Ok(())
}

fn check_lane(lane: &Lane, timeline: &Timeline) -> Result<(), String> {
    for (name, pieces) in lane.placements() {
        check_pieces(pieces, timeline).map_err(|error| format!("{name}: {error}"))?;
    }
    Ok(())
}

fn check_pieces(pieces: &[Piece], timeline: &Timeline) -> Result<(), String> {
    if pieces.is_empty() {
        return Err(String::from("no pieces"));
    }
    let mut previous_end = timeline.start;
    for piece in pieces {
        let finite = [piece.start, piece.end, piece.offset, piece.speed]
            .iter()
            .all(|value| value.is_finite());
        if !(finite && piece.start < piece.end) {
            return Err(format!(
                "piece {}-{} must run forward",
                piece.start, piece.end
            ));
        }
        if piece.start < previous_end || piece.end > timeline.end {
            return Err(format!(
                "piece {}-{} overlaps the one before or leaves the timeline",
                piece.start, piece.end
            ));
        }
        if !(SPEED_RANGE.0..=SPEED_RANGE.1).contains(&piece.speed) {
            return Err(format!(
                "speed {} is outside {}-{}",
                piece.speed, SPEED_RANGE.0, SPEED_RANGE.1
            ));
        }
        if piece.file_time(piece.start) < -0.001 {
            return Err(format!(
                "piece {}-{} starts {:.3} s before its file does",
                piece.start,
                piece.end,
                -piece.file_time(piece.start)
            ));
        }
        if !piece.file.is_absolute() {
            return Err(format!("{} is not an absolute path", piece.file.display()));
        }
        previous_end = piece.end;
    }
    Ok(())
}

fn check_choices(choices: &[Choice]) -> Result<(), String> {
    if choices.is_empty() {
        return Err(String::from("no choices"));
    }
    let mut keys = BTreeSet::new();
    let mut values = BTreeSet::new();
    for choice in choices {
        if !choice.key.is_ascii_lowercase() {
            return Err(format!(
                "choice `{}`: its key `{}` must be a lowercase letter",
                choice.value, choice.key
            ));
        }
        if CONTROL_KEYS.contains(&choice.key) {
            return Err(format!(
                "choice `{}`: `{}` is a control key of the page ({}); choose another letter",
                choice.value,
                choice.key,
                CONTROL_KEYS.iter().collect::<String>()
            ));
        }
        if !keys.insert(choice.key) {
            return Err(format!("two choices share the key `{}`", choice.key));
        }
        if choice.value.is_empty() || !values.insert(choice.value.as_str()) {
            return Err(format!(
                "choice values must be unique and not empty (`{}`)",
                choice.value
            ));
        }
    }
    Ok(())
}

fn check_effect(question: &Question) -> Result<(), String> {
    match &question.effect {
        Effect::Log => Ok(()),
        Effect::PairVerdict { .. } => {
            if question.files.len() != 2 {
                return Err(String::from(
                    "a pair_verdict question names exactly two files",
                ));
            }
            let values: BTreeSet<&str> = question
                .choices
                .iter()
                .map(|choice| choice.value.as_str())
                .collect();
            let known = values.iter().all(|value| PAIR_VALUES.contains(value));
            if !(known && values.contains("same") && values.contains("different")) {
                return Err(String::from(
                    "a pair_verdict question's choices are `same`, `different` and optionally `unsure`",
                ));
            }
            Ok(())
        }
    }
}

/// Ids unique across all the files the server loads.
pub fn check_unique(files: &[QuestionFile]) -> Result<(), String> {
    let mut ids = BTreeSet::new();
    for question in files.iter().flat_map(|file| &file.questions) {
        if !ids.insert(&question.id) {
            return Err(format!(
                "question {} appears in two question files; load one of them",
                question.id.0
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A valid two-lane pair question on a 100 s timeline.
    pub(crate) fn pair_question() -> Question {
        let files = ["second-library/a.mp3", "second-library/b.opus"];
        Question {
            id: QuestionId::derive("pair", &files),
            kind: String::from("pair"),
            title: String::from("A ~ B"),
            text: String::from("The same recording?"),
            details: Vec::new(),
            files: files.map(String::from).to_vec(),
            timeline: Timeline {
                start: 0.0,
                end: 100.0,
                reference: Some(0),
            },
            lanes: vec![
                Lane::new("A", vec![piece("/music/a.mp3", 0.0, 100.0, 0.0, 1.0)]),
                Lane {
                    name: String::from("B"),
                    pieces: vec![
                        piece("/music/b.opus", 2.0, 60.0, -2.0, 1.01),
                        piece("/music/b.opus", 60.0, 100.0, -10.0, 1.01),
                    ],
                    main: Some(vec![piece("/music/b.opus", 1.99, 100.0, -2.0, 1.01)]),
                    from_start: Some(vec![piece("/music/b.opus", 0.0, 100.0, 0.0, 1.01)]),
                },
            ],
            markers: vec![Marker {
                at: 60.0,
                label: String::from("alignment 2 starts"),
                kind: MarkerKind::Edge,
            }],
            spans: vec![Span {
                start: 2.0,
                end: 58.0,
                lanes: vec![0, 1],
                label: String::from("alignment 1"),
            }],
            choices: vec![
                choice('s', "same"),
                choice('d', "different"),
                choice('u', "unsure"),
            ],
            effect: Effect::PairVerdict {
                related: Vec::new(),
            },
        }
    }

    pub(crate) fn piece(file: &str, start: f64, end: f64, offset: f64, speed: f64) -> Piece {
        Piece {
            file: PathBuf::from(file),
            start,
            end,
            offset,
            speed,
            label: None,
        }
    }

    fn choice(key: char, value: &str) -> Choice {
        Choice {
            key,
            label: value.to_owned(),
            value: value.to_owned(),
        }
    }

    fn file_of(question: Question) -> QuestionFile {
        QuestionFile {
            schema_version: SCHEMA_VERSION,
            title: String::from("test"),
            made_by: String::from("test"),
            questions: vec![question],
        }
    }

    fn error_of(change: impl FnOnce(&mut Question)) -> String {
        let mut question = pair_question();
        change(&mut question);
        check(&file_of(question)).unwrap_err()
    }

    #[test]
    fn a_valid_file_passes_and_round_trips_through_json() {
        let file = file_of(pair_question());
        check(&file).unwrap();

        let text = serde_json::to_string(&file).unwrap();
        let back: QuestionFile = serde_json::from_str(&text).unwrap();

        assert_eq!(back, file);
        assert!(text.contains(r#""effect":{"type":"pair_verdict","related":[]}"#));
        assert!(text.contains(r#""kind":"edge""#));
    }

    #[test]
    fn ids_depend_on_the_kind_and_the_files_not_their_order() {
        let id = QuestionId::derive("pair", &["x/a.mp3", "y/b.mp3"]);

        assert_eq!(id, QuestionId::derive("pair", &["y/b.mp3", "x/a.mp3"]));
        assert_ne!(id, QuestionId::derive("folder", &["x/a.mp3", "y/b.mp3"]));
        assert_ne!(id, QuestionId::derive("pair", &["x/a.mp3", "y/c.mp3"]));
        assert_ne!(
            QuestionId::derive("pair", &["ab", "c"]),
            QuestionId::derive("pair", &["a", "bc"])
        );
        // Pinned: a different digest would detach every answer in the log.
        assert_eq!(id.as_str(), "pair-d04bf28e576eff29");
    }

    #[test]
    fn a_choice_key_that_is_a_control_key_is_refused() {
        let error = error_of(|question| question.choices[2].key = 'm');

        assert!(error.contains("`m` is a control key"), "{error}");
    }

    #[test]
    fn choice_keys_are_unique_lowercase_letters() {
        assert!(error_of(|question| question.choices[1].key = 's').contains("share the key"));
        assert!(error_of(|question| question.choices[1].key = 'D').contains("lowercase letter"));
        assert!(error_of(|question| question.choices[1].key = '1').contains("lowercase letter"));
    }

    #[test]
    fn pieces_run_forward_in_order_inside_the_timeline() {
        assert!(error_of(|question| question.lanes[1].pieces[1].start = 50.0).contains("overlaps"));
        assert!(
            error_of(|question| question.lanes[1].pieces[1].end = 120.0)
                .contains("leaves the timeline")
        );
        assert!(error_of(|question| question.lanes[1].pieces[0].speed = 3.0).contains("speed"));
        assert!(
            error_of(|question| question.lanes[1].pieces[0].file = PathBuf::from("b.opus"))
                .contains("absolute")
        );
        assert!(error_of(|question| question.lanes[1].pieces.clear()).contains("no pieces"));
        assert!(
            error_of(|question| question.lanes[1].pieces[0].offset = -3.0)
                .contains("before its file")
        );
    }

    #[test]
    fn the_reference_lane_plays_its_own_time() {
        let error = error_of(|question| question.timeline.reference = Some(1));

        assert!(error.contains("offset 0, speed 1"), "{error}");
    }

    #[test]
    fn the_reference_lane_has_one_placement() {
        let error = error_of(|question| {
            question.lanes[0].main = Some(question.lanes[0].pieces.clone());
        });

        assert!(error.contains("one placement"), "{error}");
    }

    #[test]
    fn other_placements_follow_the_rules_of_pieces() {
        let empty = error_of(|question| question.lanes[1].main = Some(Vec::new()));
        let early = error_of(|question| {
            question.lanes[1].from_start.as_mut().unwrap()[0].offset = -1.0;
        });

        assert!(empty.contains("main: no pieces"), "{empty}");
        assert!(early.contains("from_start: piece"), "{early}");
        assert!(early.contains("before its file"), "{early}");
    }

    #[test]
    fn a_lane_without_other_placements_still_loads() {
        let mut question = pair_question();
        question.lanes[1].main = None;
        question.lanes[1].from_start = None;
        let text = serde_json::to_string(&file_of(question)).unwrap();

        let back: QuestionFile = serde_json::from_str(&text).unwrap();

        assert!(!text.contains("from_start"));
        check(&back).unwrap();
        assert_eq!(back.questions[0].lanes[1].main, None);
    }

    #[test]
    fn markers_and_spans_stay_inside_the_timeline() {
        assert!(
            error_of(|question| question.markers[0].at = 101.0).contains("outside the timeline")
        );
        assert!(error_of(|question| question.spans[0].lanes = vec![2]).contains("lane index 2"));
        assert!(error_of(|question| question.spans[0].end = 1.0).contains("run forward"));
    }

    #[test]
    fn a_pair_verdict_names_two_files_and_the_pair_values() {
        assert!(error_of(|question| question.files.truncate(1)).contains("exactly two files"));
        assert!(
            error_of(|question| question.choices[2].value = String::from("maybe"))
                .contains("`same`, `different`")
        );
    }

    #[test]
    fn ids_must_match_the_kind_and_be_unique() {
        assert!(error_of(|question| question.kind = String::from("folder")).contains("16 hex"));
        let file = QuestionFile {
            questions: vec![pair_question(), pair_question()],
            ..file_of(pair_question())
        };
        assert!(check(&file).unwrap_err().contains("appears twice"));
        assert!(check_unique(&[file_of(pair_question()), file_of(pair_question())]).is_err());
    }

    #[test]
    fn another_schema_version_or_an_unknown_field_is_refused() {
        let mut file = file_of(pair_question());
        file.schema_version = 2;
        assert!(check(&file).unwrap_err().contains("schema_version 2"));

        let text = serde_json::to_string(&file_of(pair_question()))
            .unwrap()
            .replace(r#""made_by""#, r#""colour":"red","made_by""#);
        assert!(serde_json::from_str::<QuestionFile>(&text).is_err());
    }
}

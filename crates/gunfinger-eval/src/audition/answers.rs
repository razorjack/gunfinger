//! The listening answers: an append-only log of JSON lines
//! (`work/audition/answers.jsonl`), written only on a keypress: a
//! choice's key, and undo (`Z`). A question's answer is its latest answer
//! record that no undo record took back; a verdict that the question's
//! effect already records (the pair verdicts file) comes first.
//!
//! Every answer and undo reads the log and the verdicts file again, so a
//! hand edit in between is seen; the server runs them one at a time. Two
//! servers on one log are not locked against each other (the standard
//! library's file lock needs Rust 1.89, above the workspace's 1.85); since
//! each write reads both files first, they interleave correctly unless two
//! answers land within the same few milliseconds.
//!
//! An answer writes its effect first, then its log record. If the record
//! cannot be written after a verdict line was appended, the line stays
//! without a record, `Z` cannot take it back, and the error says so.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::audition::effects::{self, Applied, Recorded, Refusal, TakenBack};
use crate::audition::schema::{Question, QuestionId};
use crate::clusters::Verdicts;

/// One line of the log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    /// Counts the records from 1.
    pub seq: u64,
    /// UTC, `2026-10-12T08:15:02Z`.
    pub time: String,
    pub action: Action,
    pub question: QuestionId,
    pub kind: String,
    pub title: String,
    pub files: Vec<String>,
    /// The choice's value.
    pub choice: String,
    /// The bytes the answer appended to the verdicts file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verdict_line: Option<String>,
    /// For an undo: the answer it takes back.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub undoes: Option<u64>,
    /// For an undo: what happened to the answer's verdict line.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verdicts: Option<TakenBack>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Answer,
    Undo,
}

/// A question's current answer, and where it comes from.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct State {
    pub value: String,
    pub source: Source,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "from", rename_all = "snake_case")]
pub enum Source {
    /// The answer log's record `seq`.
    Log { seq: u64 },
    /// The effect's own file, such as the pair verdicts file.
    Recorded { pair: [String; 2], related: bool },
}

/// An answer written, with what to tell the listener.
#[derive(Debug, PartialEq)]
pub struct Answered {
    pub record: Record,
    pub message: String,
}

/// What undo did.
#[derive(Debug, PartialEq)]
pub struct Undone {
    pub record: Option<Record>,
    pub message: String,
}

/// The answer log and the verdicts file the effects write to.
#[derive(Debug, Clone)]
pub struct AnswerBook {
    pub log: PathBuf,
    pub verdicts: PathBuf,
}

impl AnswerBook {
    /// Every record of the log; none when it does not exist yet.
    pub fn records(&self) -> Result<Vec<Record>, String> {
        let text = match fs::read_to_string(&self.log) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => return Err(format!("cannot read {}: {error}", self.log.display())),
        };
        text.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(index, line)| {
                serde_json::from_str(line)
                    .map_err(|error| format!("{}, line {}: {error}", self.log.display(), index + 1))
            })
            .collect()
    }

    /// The current answer of each question that has one.
    pub fn states<'a>(
        &self,
        questions: impl IntoIterator<Item = &'a Question>,
    ) -> Result<BTreeMap<QuestionId, State>, String> {
        let records = self.records()?;
        let latest = latest_in_force(&records);
        let verdicts = Verdicts::load(&self.verdicts)?;
        Ok(questions
            .into_iter()
            .filter_map(|question| {
                state(question, &latest, &verdicts).map(|state| (question.id.clone(), state))
            })
            .collect())
    }

    /// Writes the answer of `key` to `question`: its effect first, then the
    /// log (opened before the effect, so a log that cannot be written stops
    /// the answer before anything else is).
    pub fn answer(&self, question: &Question, key: char, time: &str) -> Result<Answered, Refusal> {
        let choice = question
            .choices
            .iter()
            .find(|choice| choice.key == key)
            .ok_or_else(|| {
                Refusal::NoSuchKey(format!(
                    "question {} has no choice `{key}`",
                    question.id.as_str()
                ))
            })?;
        let records = self.records().map_err(Refusal::Io)?;
        let mut log = self.open_log().map_err(Refusal::Io)?;
        let applied = effects::apply(question, &choice.value, &self.verdicts)?;
        let (verdict_line, message) = match applied {
            Applied::Nothing => (None, format!("answered `{}`", choice.value)),
            Applied::AlreadyRecorded(note) => (None, note),
            Applied::VerdictLine(line) => (
                Some(line),
                format!(
                    "answered `{}`; appended to {}",
                    choice.value,
                    self.verdicts.display()
                ),
            ),
        };
        let record = Record {
            seq: next_seq(&records),
            time: time.to_owned(),
            action: Action::Answer,
            question: question.id.clone(),
            kind: question.kind.clone(),
            title: question.title.clone(),
            files: question.files.clone(),
            choice: choice.value.clone(),
            verdict_line,
            undoes: None,
            verdicts: None,
        };
        if let Err(error) = self.write(&mut log, &record) {
            return Err(Refusal::Io(match &record.verdict_line {
                Some(line) => format!(
                    "{error}; the line `{}` was appended to {} without a record, so Z cannot take it back: remove it by hand if you do not want it",
                    line.trim(),
                    self.verdicts.display()
                ),
                None => error,
            }));
        }
        Ok(Answered { record, message })
    }

    /// Takes back the most recent answer still in force, if its question is
    /// one of `loaded` (the page shows what it takes back; an answer to a
    /// question not loaded is left for a session that loads it).
    pub fn undo(&self, loaded: &BTreeSet<&QuestionId>, time: &str) -> Result<Undone, String> {
        let records = self.records()?;
        let Some(answer) = in_force(&records).last().copied() else {
            return Ok(Undone {
                record: None,
                message: String::from("nothing to undo"),
            });
        };
        if !loaded.contains(&answer.question) {
            return Ok(Undone {
                record: None,
                message: format!(
                    "the latest answer, `{}` to \"{}\", is to a question not loaded here; load its question file to take it back",
                    answer.choice, answer.title
                ),
            });
        }
        let mut log = self.open_log()?;
        let taken_back = match &answer.verdict_line {
            Some(line) => effects::take_back(line, &self.verdicts)?,
            None => TakenBack::Nothing,
        };
        let record = Record {
            seq: next_seq(&records),
            time: time.to_owned(),
            action: Action::Undo,
            undoes: Some(answer.seq),
            verdicts: Some(taken_back),
            verdict_line: None,
            ..answer.clone()
        };
        self.write(&mut log, &record)?;
        let what = format!("took back `{}` to \"{}\"", answer.choice, answer.title);
        let message = match taken_back {
            TakenBack::Removed => format!(
                "{what} and removed its line from {}",
                self.verdicts.display()
            ),
            TakenBack::Left => format!(
                "{what} in the log; {} has changed since, so its line was left there: remove it by hand if you want",
                self.verdicts.display()
            ),
            TakenBack::Nothing => what,
        };
        Ok(Undone {
            record: Some(record),
            message,
        })
    }

    fn open_log(&self) -> Result<File, String> {
        if let Some(dir) = self.log.parent() {
            fs::create_dir_all(dir)
                .map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
        }
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log)
            .map_err(|error| format!("cannot open {}: {error}", self.log.display()))
    }

    fn write(&self, log: &mut File, record: &Record) -> Result<(), String> {
        let line = serde_json::to_string(record).map_err(|error| error.to_string())? + "\n";
        log.write_all(line.as_bytes())
            .map_err(|error| format!("cannot write {}: {error}", self.log.display()))
    }
}

/// Every answer no undo took back, in log order.
fn in_force(records: &[Record]) -> Vec<&Record> {
    let undone: BTreeSet<u64> = records.iter().filter_map(|record| record.undoes).collect();
    records
        .iter()
        .filter(|record| record.action == Action::Answer && !undone.contains(&record.seq))
        .collect()
}

/// Each question's latest answer in force.
fn latest_in_force(records: &[Record]) -> BTreeMap<&QuestionId, &Record> {
    in_force(records)
        .into_iter()
        .map(|record| (&record.question, record))
        .collect()
}

fn state(
    question: &Question,
    latest: &BTreeMap<&QuestionId, &Record>,
    verdicts: &Verdicts,
) -> Option<State> {
    if let Some(Recorded {
        value,
        pair,
        related,
    }) = effects::recorded(question, verdicts)
    {
        return Some(State {
            value,
            source: Source::Recorded { pair, related },
        });
    }
    latest.get(&question.id).map(|record| State {
        value: record.choice.clone(),
        source: Source::Log { seq: record.seq },
    })
}

fn next_seq(records: &[Record]) -> u64 {
    records.iter().map(|record| record.seq).max().unwrap_or(0) + 1
}

/// The current time in UTC, `YYYY-MM-DDTHH:MM:SSZ`.
pub fn now() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_secs());
    utc(seconds)
}

/// Seconds since 1970 as a UTC time, by the days-to-civil algorithm
/// (Howard Hinnant, "chrono-Compatible Low-Level Date Algorithms").
fn utc(seconds: u64) -> String {
    let days = seconds / 86_400;
    let (hour, minute, second) = (seconds / 3600 % 24, seconds / 60 % 60, seconds % 60);
    // Shift the epoch to 0000-03-01 so that leap days end each 400-year era.
    let z = days + 719_468;
    let era = z / 146_097;
    let day_of_era = z % 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_index = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_index + 2) / 5 + 1;
    let month = if month_index < 10 {
        month_index + 3
    } else {
        month_index - 9
    };
    let year = year_of_era + era * 400 + u64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Where the answers go by default, below the work directory.
pub fn default_log(work: &Path) -> PathBuf {
    work.join("audition").join("answers.jsonl")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::schema::Effect;
    use crate::audition::schema::tests::pair_question;

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-audition-answers-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn book(dir: &Path) -> AnswerBook {
        AnswerBook {
            log: dir.join("answers.jsonl"),
            verdicts: dir.join("pair-verdicts.txt"),
        }
    }

    const SAME: &str = "same\tsecond-library/a.mp3\tsecond-library/b.opus\n";

    #[test]
    fn a_pair_answer_goes_to_the_log_and_the_verdicts_file() {
        let dir = scratch("pair");
        let book = book(&dir);
        fs::write(&book.verdicts, "# verdicts\nsame\tx.mp3\ty.mp3").unwrap();
        let question = pair_question();

        let answered = book.answer(&question, 's', "2026-10-12T08:00:00Z").unwrap();

        assert_eq!(
            fs::read_to_string(&book.verdicts).unwrap(),
            format!("# verdicts\nsame\tx.mp3\ty.mp3\n{SAME}")
        );
        assert_eq!(answered.record.seq, 1);
        assert_eq!(
            answered.record.verdict_line.as_deref(),
            Some(format!("\n{SAME}").as_str())
        );
        let log = fs::read_to_string(&book.log).unwrap();
        assert_eq!(log.lines().count(), 1);
        assert!(log.contains(r#""action":"answer""#), "{log}");
        assert!(log.contains(r#""choice":"same""#), "{log}");
        assert!(
            log.contains(r#""files":["second-library/a.mp3","second-library/b.opus"]"#),
            "{log}"
        );
        let states = book.states([&question]).unwrap();
        assert_eq!(states[&question.id].value, "same");
    }

    #[test]
    fn unsure_and_log_effects_write_the_log_only() {
        let dir = scratch("unsure");
        let book = book(&dir);
        let mut folder = pair_question();
        folder.effect = Effect::Log;

        book.answer(&pair_question(), 'u', "t1").unwrap();
        book.answer(&folder, 'd', "t2").unwrap();

        assert!(!book.verdicts.exists());
        assert_eq!(book.records().unwrap().len(), 2);
    }

    #[test]
    fn an_unknown_key_is_refused_and_writes_nothing() {
        let dir = scratch("key");
        let book = book(&dir);

        let refusal = book.answer(&pair_question(), 'x', "t").unwrap_err();

        assert!(matches!(refusal, Refusal::NoSuchKey(_)));
        assert!(!book.log.exists());
    }

    #[test]
    fn a_pair_judged_in_the_verdicts_file_shows_answered_and_refuses_a_contradiction() {
        let dir = scratch("judged");
        let book = book(&dir);
        let mut question = pair_question();
        question.effect = Effect::PairVerdict {
            related: vec![[String::from("c.mp3"), String::from("second-library/b.opus")]],
        };
        fs::write(&book.verdicts, "different\tsecond-library/b.opus\tc.mp3\n").unwrap();

        let states = book.states([&question]).unwrap();
        assert_eq!(states[&question.id].value, "different");
        assert_eq!(
            states[&question.id].source,
            Source::Recorded {
                pair: [String::from("c.mp3"), String::from("second-library/b.opus")],
                related: true
            }
        );

        let refusal = book.answer(&question, 's', "t").unwrap_err();
        assert!(
            matches!(refusal, Refusal::Contradicts(ref message) if message.contains("says `different`"))
        );
        let again = book.answer(&question, 'd', "t").unwrap();
        assert!(again.message.contains("logged only"), "{}", again.message);
        assert_eq!(
            fs::read_to_string(&book.verdicts).unwrap(),
            "different\tsecond-library/b.opus\tc.mp3\n"
        );
    }

    #[test]
    fn undo_takes_back_the_latest_answer_then_the_one_before() {
        let dir = scratch("undo");
        let book = book(&dir);
        let pair = pair_question();
        let mut folder = pair_question();
        folder.effect = Effect::Log;
        folder.id = QuestionId::derive("pair", &["folder"]);

        let loaded = BTreeSet::from([&pair.id, &folder.id]);

        book.answer(&pair, 's', "t1").unwrap();
        book.answer(&folder, 'u', "t2").unwrap();
        let first = book.undo(&loaded, "t3").unwrap();
        let second = book.undo(&loaded, "t4").unwrap();
        let third = book.undo(&loaded, "t5").unwrap();

        assert_eq!(first.record.as_ref().unwrap().undoes, Some(2));
        assert_eq!(
            first.record.as_ref().unwrap().verdicts,
            Some(TakenBack::Nothing)
        );
        assert_eq!(second.record.as_ref().unwrap().undoes, Some(1));
        assert_eq!(
            second.record.as_ref().unwrap().verdicts,
            Some(TakenBack::Removed)
        );
        assert!(
            second.message.contains("removed its line"),
            "{}",
            second.message
        );
        assert!(second.message.contains(&pair.title), "{}", second.message);
        assert_eq!(third.record, None);
        assert_eq!(fs::read_to_string(&book.verdicts).unwrap(), "");
        assert!(book.states([&pair, &folder]).unwrap().is_empty());
        assert_eq!(book.records().unwrap().len(), 4);
    }

    #[test]
    fn undo_leaves_the_verdicts_file_alone_once_another_line_follows() {
        let dir = scratch("undo-left");
        let book = book(&dir);
        book.answer(&pair_question(), 'd', "t1").unwrap();
        let mut verdicts = fs::read_to_string(&book.verdicts).unwrap();
        verdicts.push_str("same\tp.mp3\tq.mp3\n");
        fs::write(&book.verdicts, &verdicts).unwrap();

        let undone = book
            .undo(&BTreeSet::from([&pair_question().id]), "t2")
            .unwrap();

        assert_eq!(undone.record.unwrap().verdicts, Some(TakenBack::Left));
        assert!(undone.message.contains("left there"), "{}", undone.message);
        assert_eq!(fs::read_to_string(&book.verdicts).unwrap(), verdicts);
    }

    #[test]
    fn a_new_answer_replaces_the_old_and_undo_brings_it_back() {
        let dir = scratch("again");
        let book = book(&dir);
        let mut question = pair_question();
        question.effect = Effect::Log;

        book.answer(&question, 's', "t1").unwrap();
        book.answer(&question, 'u', "t2").unwrap();
        assert_eq!(
            book.states([&question]).unwrap()[&question.id].value,
            "unsure"
        );

        book.undo(&BTreeSet::from([&question.id]), "t3").unwrap();
        assert_eq!(
            book.states([&question]).unwrap()[&question.id].value,
            "same"
        );
    }

    #[test]
    fn undo_leaves_a_line_edited_to_end_the_same_way() {
        let dir = scratch("undo-edited");
        let book = book(&dir);
        let question = pair_question();
        book.answer(&question, 's', "t1").unwrap();
        fs::write(&book.verdicts, format!("# {SAME}")).unwrap();

        let undone = book.undo(&BTreeSet::from([&question.id]), "t2").unwrap();

        assert_eq!(undone.record.unwrap().verdicts, Some(TakenBack::Left));
        assert_eq!(
            fs::read_to_string(&book.verdicts).unwrap(),
            format!("# {SAME}")
        );
    }

    #[test]
    fn undo_takes_back_nothing_of_a_question_not_loaded() {
        let dir = scratch("undo-unloaded");
        let book = book(&dir);
        let question = pair_question();
        book.answer(&question, 's', "t1").unwrap();

        let undone = book.undo(&BTreeSet::new(), "t2").unwrap();

        assert_eq!(undone.record, None);
        assert!(undone.message.contains("not loaded"), "{}", undone.message);
        assert!(
            undone.message.contains(&question.title),
            "{}",
            undone.message
        );
        assert_eq!(fs::read_to_string(&book.verdicts).unwrap(), SAME);
        assert_eq!(book.records().unwrap().len(), 1);
    }

    #[test]
    fn unsure_is_refused_where_the_verdicts_file_already_judges_the_pair() {
        let dir = scratch("unsure-judged");
        let book = book(&dir);
        fs::write(&book.verdicts, SAME).unwrap();

        let refusal = book.answer(&pair_question(), 'u', "t").unwrap_err();

        assert!(
            matches!(refusal, Refusal::Contradicts(ref message) if message.contains("says `same`"))
        );
        assert!(book.records().unwrap().is_empty());
    }

    #[test]
    fn a_damaged_log_line_is_named() {
        let dir = scratch("damaged");
        let book = book(&dir);
        fs::write(&book.log, "{\"seq\":1}\n").unwrap();

        let error = book.records().unwrap_err();

        assert!(error.contains("line 1"), "{error}");
    }

    #[test]
    fn times_are_utc() {
        assert_eq!(utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(utc(1_791_706_502), "2026-10-11T08:15:02Z");
    }
}

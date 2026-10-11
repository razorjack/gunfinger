//! What an answer writes besides the answer log, by the question's
//! `effect`. `log` writes nothing more. `pair_verdict` appends `same` or
//! `different` with the question's two files to the pair verdicts file, in
//! the format `pair-review` writes, and undo removes that line only while
//! it is still the end of the file, byte for byte.
//!
//! A new place for answers is a new `schema::Effect` variant and its arm
//! in `recorded`, `apply` and (if it writes) `take_back`.

use std::fs::{self, OpenOptions};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::audition::schema::{Effect, Question};
use crate::clusters::Verdicts;

/// An answer the effect's own file already records.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Recorded {
    pub value: String,
    /// The two files the verdict names: the question's or a related pair.
    pub pair: [String; 2],
    pub related: bool,
}

/// Why an answer was not written.
#[derive(Debug, PartialEq)]
pub enum Refusal {
    /// The question has no choice with this key.
    NoSuchKey(String),
    /// Writing it would contradict what is already recorded.
    Contradicts(String),
    /// A file could not be read or written.
    Io(String),
}

/// What applying an answer's effect did.
#[derive(Debug, PartialEq)]
pub enum Applied {
    Nothing,
    /// The effect's file already held this answer; nothing appended.
    AlreadyRecorded(String),
    /// These bytes were appended to the verdicts file.
    VerdictLine(String),
}

/// What undo did to an appended verdict line.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TakenBack {
    Removed,
    /// Other lines were written after it; the file was left alone.
    Left,
    /// The answer had appended nothing.
    #[serde(rename = "none")]
    Nothing,
}

/// The answer the question's effect already records, if any: for a pair, a
/// verdict on its two files or on a related pair (`Verdicts::judges`).
pub fn recorded(question: &Question, verdicts: &Verdicts) -> Option<Recorded> {
    match &question.effect {
        Effect::Log => None,
        Effect::PairVerdict { related } => {
            let own = [
                question.files.first()?.clone(),
                question.files.get(1)?.clone(),
            ];
            std::iter::once((own, false))
                .chain(related.iter().map(|pair| (pair.clone(), true)))
                .find_map(|(pair, related)| {
                    let same = verdicts.verdict(&pair[0], &pair[1])?;
                    Some(Recorded {
                        value: String::from(if same { "same" } else { "different" }),
                        pair,
                        related,
                    })
                })
        }
    }
}

/// Writes what the answer `value` to `question` writes besides the log.
pub fn apply(question: &Question, value: &str, verdicts_file: &Path) -> Result<Applied, Refusal> {
    match &question.effect {
        Effect::Log => Ok(Applied::Nothing),
        Effect::PairVerdict { .. } => {
            let same = match value {
                "same" => true,
                "different" => false,
                _ => return Ok(Applied::Nothing),
            };
            let verdicts = Verdicts::load(verdicts_file).map_err(Refusal::Io)?;
            if let Some(recorded) = recorded(question, &verdicts) {
                let pair = format!("{} ~ {}", recorded.pair[0], recorded.pair[1]);
                return if recorded.value == value {
                    Ok(Applied::AlreadyRecorded(format!(
                        "{} already says `{value}` for {pair}; logged only",
                        verdicts_file.display()
                    )))
                } else {
                    Err(Refusal::Contradicts(format!(
                        "{} already says `{}` for {pair}; to change it, take back the answer that wrote it (Z) or edit the file",
                        verdicts_file.display(),
                        recorded.value
                    )))
                };
            }
            let [a, b] = [&question.files[0], &question.files[1]];
            Verdicts::append(verdicts_file, same, a, b)
                .map(Applied::VerdictLine)
                .map_err(Refusal::Io)
        }
    }
}

/// Removes `appended` from the end of the verdicts file if it is still
/// there, byte for byte; otherwise leaves the file alone.
pub fn take_back(appended: &str, verdicts_file: &Path) -> Result<TakenBack, String> {
    let bytes = match fs::read(verdicts_file) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(TakenBack::Left),
        Err(error) => return Err(format!("cannot read {}: {error}", verdicts_file.display())),
    };
    if !bytes.ends_with(appended.as_bytes()) {
        return Ok(TakenBack::Left);
    }
    let length = (bytes.len() - appended.len()) as u64;
    OpenOptions::new()
        .write(true)
        .open(verdicts_file)
        .and_then(|file| file.set_len(length))
        .map_err(|error| format!("cannot shorten {}: {error}", verdicts_file.display()))?;
    Ok(TakenBack::Removed)
}

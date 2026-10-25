//! What the server serves, without HTTP: the loaded question files, the
//! files they list (the only files the server decodes, describes or
//! reveals), and the answers, written one at a time.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

use serde::Serialize;

use crate::audition::answers::{self, AnswerBook, State};
use crate::audition::audio::WINDOW_SECONDS;
use crate::audition::effects::Refusal;
use crate::audition::schema::{self, CONTROL_KEYS, Question, QuestionFile, QuestionId};

pub struct Session {
    files: Vec<(PathBuf, QuestionFile)>,
    listed: Listed,
    /// One writer at a time: answers and undos run under this lock.
    book: Mutex<AnswerBook>,
}

/// Every file a lane of a loaded question plays, in any placement.
#[derive(Debug, Default)]
pub struct Listed(BTreeSet<PathBuf>);

impl Listed {
    pub fn of<'a>(files: impl IntoIterator<Item = &'a QuestionFile>) -> Listed {
        Listed(
            files
                .into_iter()
                .flat_map(|file| &file.questions)
                .flat_map(|question| &question.lanes)
                .flat_map(|lane| lane.placements().flat_map(|(_, pieces)| pieces))
                .map(|piece| piece.file.clone())
                .collect(),
        )
    }

    /// The listed file named exactly `path`. Paths are compared as given,
    /// never resolved, so no spelling reaches another file.
    pub fn get(&self, path: &str) -> Option<&Path> {
        self.0.get(Path::new(path)).map(PathBuf::as_path)
    }
}

/// What the page loads: every question with its current answer.
#[derive(Serialize)]
pub struct Overview<'a> {
    pub files: Vec<FileView<'a>>,
    pub window_seconds: f64,
    pub control_keys: String,
    pub answers: String,
    pub verdicts: String,
}

#[derive(Serialize)]
pub struct FileView<'a> {
    pub title: &'a str,
    pub path: String,
    pub questions: Vec<QuestionView<'a>>,
}

#[derive(Serialize)]
pub struct QuestionView<'a> {
    #[serde(flatten)]
    pub question: &'a Question,
    pub state: Option<State>,
}

/// After an answer or an undo: what to tell the listener and every
/// question's answer.
#[derive(Serialize)]
pub struct Reply {
    pub message: String,
    pub question: Option<QuestionId>,
    pub states: BTreeMap<QuestionId, State>,
}

impl Session {
    pub fn load(paths: &[PathBuf], book: AnswerBook) -> Result<Session, String> {
        let files = paths
            .iter()
            .map(|path| schema::load(path).map(|file| (path.clone(), file)))
            .collect::<Result<Vec<_>, String>>()?;
        let loaded: Vec<QuestionFile> = files.iter().map(|(_, file)| file.clone()).collect();
        schema::check_unique(&loaded)?;
        // A damaged log or verdicts file is better found now than on the
        // first keypress.
        book.states(loaded.iter().flat_map(|file| &file.questions))?;
        Ok(Session {
            listed: Listed::of(&loaded),
            files,
            book: Mutex::new(book),
        })
    }

    pub fn listed(&self) -> &Listed {
        &self.listed
    }

    fn questions(&self) -> impl Iterator<Item = &Question> {
        self.files.iter().flat_map(|(_, file)| &file.questions)
    }

    pub fn overview(&self) -> Result<Overview<'_>, String> {
        let book = self.book.lock().unwrap_or_else(PoisonError::into_inner);
        let mut states = book.states(self.questions())?;
        Ok(Overview {
            files: self
                .files
                .iter()
                .map(|(path, file)| FileView {
                    title: &file.title,
                    path: path.display().to_string(),
                    questions: file
                        .questions
                        .iter()
                        .map(|question| QuestionView {
                            question,
                            state: states.remove(&question.id),
                        })
                        .collect(),
                })
                .collect(),
            window_seconds: WINDOW_SECONDS,
            control_keys: CONTROL_KEYS.iter().collect(),
            answers: book.log.display().to_string(),
            verdicts: book.verdicts.display().to_string(),
        })
    }

    /// `key` was pressed on question `id`.
    pub fn answer(&self, id: &str, key: char) -> Result<Reply, Refusal> {
        let question = self
            .questions()
            .find(|question| question.id.as_str() == id)
            .ok_or_else(|| Refusal::NoSuchKey(format!("no question {id} is loaded")))?;
        let book = self.book.lock().unwrap_or_else(PoisonError::into_inner);
        let answered = book.answer(question, key, &answers::now())?;
        Ok(Reply {
            message: answered.message,
            question: Some(question.id.clone()),
            states: book.states(self.questions()).map_err(Refusal::Io)?,
        })
    }

    /// `Z` was pressed.
    pub fn undo(&self) -> Result<Reply, String> {
        let loaded = self.questions().map(|question| &question.id).collect();
        let book = self.book.lock().unwrap_or_else(PoisonError::into_inner);
        let undone = book.undo(&loaded, &answers::now())?;
        Ok(Reply {
            message: undone.message,
            question: undone.record.map(|record| record.question),
            states: book.states(self.questions())?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::schema::SCHEMA_VERSION;
    use crate::audition::schema::tests::{pair_question, piece};

    #[test]
    fn the_files_of_every_placement_are_listed() {
        let mut question = pair_question();
        question.lanes[1].main = Some(vec![piece("/music/b-main.opus", 2.0, 100.0, -2.0, 1.01)]);
        let file = QuestionFile {
            schema_version: SCHEMA_VERSION,
            title: String::from("test"),
            made_by: String::from("test"),
            questions: vec![question],
        };

        let listed = Listed::of([&file]);

        assert!(listed.get("/music/a.mp3").is_some());
        assert!(listed.get("/music/b.opus").is_some());
        assert!(listed.get("/music/b-main.opus").is_some());
        assert!(listed.get("/music/c.opus").is_none());
    }
}

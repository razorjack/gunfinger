//! `questions folders`: is a release folder a mixed CD, each track running
//! into the next? One question per folder of a plan (TOML), with one lane
//! that plays every boundary as the CD would: the last 20 s of track `n`
//! straight into the first 20 s of track `n + 1`, for every `n`, the
//! folder's audio files in name order. A marker sits at each boundary.
//! Answers go to the answer log only; turning them into ignore patterns is
//! for an agent on request. A continuous CD (a few seconds of
//! crossfade between otherwise separate tracks) has its own answer;
//! session 14's folders held three.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::library::Library;
use serde::Deserialize;

use crate::audition::names::Libraries;
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Timeline,
};

/// The folders to ask about.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    /// Seconds heard on each side of a boundary.
    #[serde(default = "default_excerpt_seconds")]
    pub excerpt_seconds: f64,
    pub folder: Vec<PlannedFolder>,
}

fn default_excerpt_seconds() -> f64 {
    20.0
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedFolder {
    pub title: String,
    /// Absolute.
    pub path: PathBuf,
    /// Only the files whose names start with this, such as one disc's
    /// (`2` for `201-...`, `202-...`).
    #[serde(default)]
    pub name_prefix: Option<String>,
    /// A line shown under the question.
    #[serde(default)]
    pub note: Option<String>,
}

impl Plan {
    pub fn load(path: &Path) -> Result<Plan, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        toml::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }
}

/// One question per folder; `length` gives an audio file's decoded length
/// in seconds.
pub fn questions(
    plan: &Plan,
    libraries: &Libraries,
    length: impl Fn(&Path) -> Result<f64, String>,
) -> Result<QuestionFile, String> {
    let questions = plan
        .folder
        .iter()
        .map(|folder| {
            question(folder, plan.excerpt_seconds, libraries, &length)
                .map_err(|error| format!("{}: {error}", folder.path.display()))
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: String::from("Mixed CDs?"),
        made_by: String::from("gunfinger-eval questions folders"),
        questions,
    })
}

fn question(
    folder: &PlannedFolder,
    excerpt: f64,
    libraries: &Libraries,
    length: &impl Fn(&Path) -> Result<f64, String>,
) -> Result<Question, String> {
    let tracks = tracks(folder)?;
    if tracks.len() < 2 {
        return Err(format!(
            "{} audio files; a boundary needs two",
            tracks.len()
        ));
    }
    let lengths = tracks
        .iter()
        .map(|track| length(track))
        .collect::<Result<Vec<f64>, String>>()?;
    let name = |index: usize| {
        tracks[index]
            .file_name()
            .map_or_else(String::new, |name| name.to_string_lossy().into_owned())
    };

    let mut pieces = Vec::new();
    let mut markers = Vec::new();
    let mut at = 0.0;
    for index in 0..tracks.len() - 1 {
        if index > 0 {
            markers.push(Marker {
                at,
                label: format!(
                    "track {}: from its first {excerpt:.0} s to its last",
                    index + 1
                ),
                kind: MarkerKind::Join,
            });
        }
        let tail = excerpt.min(lengths[index]);
        pieces.push(Piece {
            file: tracks[index].clone(),
            start: at,
            end: at + tail,
            offset: lengths[index] - tail - at,
            speed: 1.0,
            label: Some(format!(
                "track {} ({}), last {tail:.0} s",
                index + 1,
                name(index)
            )),
        });
        at += tail;
        markers.push(Marker {
            at,
            label: format!(
                "track {} → track {}: {} → {}",
                index + 1,
                index + 2,
                name(index),
                name(index + 1)
            ),
            kind: MarkerKind::Listen,
        });
        let head = excerpt.min(lengths[index + 1]);
        pieces.push(Piece {
            file: tracks[index + 1].clone(),
            start: at,
            end: at + head,
            offset: -at,
            speed: 1.0,
            label: Some(format!(
                "track {} ({}), first {head:.0} s",
                index + 2,
                name(index + 1)
            )),
        });
        at += head;
    }

    let files: Vec<String> = tracks.iter().map(|track| libraries.name(track)).collect();
    let parts: Vec<&str> = files.iter().map(String::as_str).collect();
    let mut details = vec![
        format!("{}/", folder.path.display()),
        format!(
            "{} audio files in name order, {} boundaries: the last {excerpt:.0} s of each track, then the first {excerpt:.0} s of the next",
            tracks.len(),
            tracks.len() - 1
        ),
    ];
    details.extend(folder.note.clone());
    Ok(Question {
        id: QuestionId::derive("folder", &parts),
        kind: String::from("folder"),
        title: folder.title.clone(),
        text: String::from(
            "Is this a mixed CD (a DJ mix: each track blended into the next), a continuous CD (a few seconds of crossfade between otherwise separate tracks), or separate tracks?",
        ),
        details,
        files,
        timeline: Timeline {
            start: 0.0,
            end: at,
            reference: None,
        },
        lanes: vec![Lane::new("CD", pieces)],
        markers,
        spans: Vec::new(),
        choices: vec![
            choice('y', "mixed CD", "mixed"),
            choice('c', "continuous: short crossfades", "continuous"),
            choice('n', "separate tracks", "separate"),
            choice('u', "unsure", "unsure"),
        ],
        effect: Effect::Log,
    })
}

/// The folder's own audio files (not its subfolders'), as the library
/// lists audio, with the name prefix, in name order.
fn tracks(folder: &PlannedFolder) -> Result<Vec<PathBuf>, String> {
    let library = Library::scan(&folder.path).map_err(|error| error.to_string())?;
    let mut names: Vec<&str> = library
        .assets
        .iter()
        .map(|asset| asset.path.as_str())
        .filter(|path| !path.contains('/'))
        .filter(|name| {
            folder
                .name_prefix
                .as_deref()
                .is_none_or(|prefix| name.starts_with(prefix))
        })
        .collect();
    names.sort_unstable();
    Ok(names
        .into_iter()
        .map(|name| folder.path.join(name))
        .collect())
}

fn choice(key: char, label: &str, value: &str) -> Choice {
    Choice {
        key,
        label: label.to_owned(),
        value: value.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::audio::Envelopes;
    use crate::audition::audio::tests::tone;
    use crate::audition::schema::check;

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-audition-folders-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn every_boundary_plays_the_end_of_one_track_into_the_start_of_the_next() {
        let dir = scratch("cd");
        let cd = dir.join("nas").join("artist-album");
        fs::create_dir_all(cd.join("scans")).unwrap();
        for (name, seconds) in [
            ("01-intro.mp3", 3.0),
            ("02-tune.mp3", 5.0),
            ("03-outro.mp3", 4.0),
        ] {
            if tone(&cd.join(name), seconds, 440).is_none() {
                eprintln!("skipped: ffmpeg cannot generate audio here");
                return;
            }
        }
        tone(&cd.join("scans").join("hidden-track.mp3"), 2.0, 220).unwrap();
        fs::write(cd.join("00-album.nfo"), "release notes").unwrap();
        let plan: Plan = toml::from_str(&format!(
            "excerpt_seconds = 2.0\n[[folder]]\ntitle = \"Artist - Album\"\npath = \"{}\"\nnote = \"NFO: mixed by\"\n",
            cd.display()
        ))
        .unwrap();
        let libraries = Libraries {
            corpus: dir.join("corpus"),
            second: dir.join("nas"),
        };
        let envelopes = Envelopes {
            dir: dir.join("envelopes"),
        };

        let file = questions(&plan, &libraries, |path| {
            envelopes
                .get(path)
                .map(|envelope| envelope.duration_seconds)
        })
        .unwrap();
        check(&file).unwrap();

        let question = &file.questions[0];
        assert_eq!(
            question.files,
            [
                "second-library/artist-album/01-intro.mp3",
                "second-library/artist-album/02-tune.mp3",
                "second-library/artist-album/03-outro.mp3"
            ]
        );
        let pieces = &question.lanes[0].pieces;
        assert_eq!(pieces.len(), 4);
        // The last 2 s of track 1 (about 3 s long), at timeline 0.
        let intro_end = pieces[0].file_time(pieces[0].end);
        assert!(
            (intro_end
                - envelopes
                    .get(&cd.join("01-intro.mp3"))
                    .unwrap()
                    .duration_seconds)
                .abs()
                < 1e-9
        );
        assert!((pieces[0].file_time(0.0) - (intro_end - 2.0)).abs() < 1e-9);
        // Then track 2 from its start, at timeline 2.
        assert_eq!((pieces[1].start, pieces[1].file_time(2.0)), (2.0, 0.0));
        assert_eq!(pieces[2].file, cd.join("02-tune.mp3"));
        assert_eq!(pieces[3].file_time(pieces[3].start), 0.0);
        assert_eq!(question.timeline.end, 8.0);
        let kinds: Vec<(f64, MarkerKind)> = question
            .markers
            .iter()
            .map(|marker| (marker.at, marker.kind))
            .collect();
        assert_eq!(
            kinds,
            [
                (2.0, MarkerKind::Listen),
                (4.0, MarkerKind::Join),
                (6.0, MarkerKind::Listen)
            ]
        );
        assert_eq!(question.details[2], "NFO: mixed by");
        assert_eq!(
            question.id,
            QuestionId::derive(
                "folder",
                &[
                    "second-library/artist-album/03-outro.mp3",
                    "second-library/artist-album/01-intro.mp3",
                    "second-library/artist-album/02-tune.mp3",
                ]
            )
        );
    }

    #[test]
    fn a_name_prefix_keeps_one_disc() {
        let dir = scratch("disc");
        for name in [
            "101-a.mp3",
            "102-b.mp3",
            "201-c.mp3",
            "202-d.mp3",
            "203-e.mp3",
        ] {
            if tone(&dir.join(name), 1.0, 440).is_none() {
                eprintln!("skipped: ffmpeg cannot generate audio here");
                return;
            }
        }
        let folder = PlannedFolder {
            title: String::from("Disc 2"),
            path: dir.clone(),
            name_prefix: Some(String::from("2")),
            note: None,
        };

        let tracks = tracks(&folder).unwrap();

        assert_eq!(
            tracks,
            [
                dir.join("201-c.mp3"),
                dir.join("202-d.mp3"),
                dir.join("203-e.mp3")
            ]
        );
    }
}

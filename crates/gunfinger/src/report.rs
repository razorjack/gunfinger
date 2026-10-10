//! The result of `identify`: what is printed as JSON and what every other
//! output format is rendered from. A saved JSON report can be rendered again
//! without searching (`gunfinger show`).

use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::Excerpt;
use gunfinger_core::hash;
use gunfinger_core::plays::{self, SameAudio};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{self, Detection, Matcher};
use gunfinger_core::speed;
use gunfinger_core::tags::Tags;
use miette::{IntoDiagnostic, WrapErr, miette};
use serde::{Deserialize, Serialize};

use crate::catalog::{Catalog, absolute};
use crate::names::TrackName;
use crate::playback::PlaybackChoice;

/// Version 3 merged plays of the same audio; fields may be added without a
/// new version.
pub const SCHEMA_VERSION: u32 = 3;

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub schema_version: u32,
    pub query: Query,
    /// Root of the library the asset paths are relative to.
    #[serde(default)]
    pub library: PathBuf,
    /// How the search was made; absent in reports written before it was
    /// recorded.
    #[serde(default)]
    pub search: Option<SearchSettings>,
    /// Every confident or possible play, in order of start time. Plays of
    /// assets with the same audio are one play.
    pub plays: Vec<FoundPlay>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    pub path: PathBuf,
    pub start_seconds: f64,
    pub duration_seconds: f64,
    /// The playbacks searched for; absent in reports written before it was
    /// recorded.
    #[serde(default)]
    pub playback: Option<PlaybackChoice>,
    /// The `--duration` asked for; absent when the search ran to the end
    /// of the recording.
    #[serde(default)]
    pub requested_duration_seconds: Option<f64>,
}

/// Everything besides the recording and the playback that decides what a
/// search finds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchSettings {
    /// The front-end profile of the peaks.
    pub profile: String,
    pub hashes: String,
    /// Lines, chains and the speeds of the ladder.
    pub matching: String,
    pub confidence: String,
    /// A digest of the indexed files' paths, sizes and modification times:
    /// it changes when files are added, changed, indexed, removed or
    /// excluded.
    pub library_revision: String,
}

impl SearchSettings {
    pub fn current(matcher: Matcher, library_revision: &str) -> SearchSettings {
        SearchSettings {
            profile: Profile::CURRENT.id(),
            hashes: hash::design(),
            matching: search::design(matcher),
            confidence: matcher.pass().rule().to_string(),
            library_revision: library_revision.to_owned(),
        }
    }

    /// The first setting in which `self` differs from `other`, in words.
    pub fn difference(&self, other: &SearchSettings) -> Option<&'static str> {
        if self.profile != other.profile {
            Some("another peak profile")
        } else if self.hashes != other.hashes {
            Some("another hash design")
        } else if self.matching != other.matching {
            Some("other matching settings")
        } else if self.confidence != other.confidence {
            Some("another confidence rule")
        } else if self.library_revision != other.library_revision {
            Some(
                "another revision of the library (files added, changed, indexed or excluded since)",
            )
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundPlay {
    pub asset: String,
    /// Other assets with exactly the same detections: copies of the file or
    /// rips with identical peaks.
    #[serde(default)]
    pub same_audio: Vec<String>,
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Where the first segment starts and the last ends in the track.
    #[serde(default)]
    pub track_start_seconds: f64,
    #[serde(default)]
    pub track_end_seconds: f64,
    /// The strongest segment's speed, playback and confidence.
    pub speed: f64,
    #[serde(default)]
    pub playback: Playback,
    pub confidence: Level,
    /// Summed over the segments.
    pub windows: u32,
    pub hits: u32,
    pub segments: Vec<Segment>,
    /// Set on a possible play that lies entirely inside a confident play
    /// of another recording: most likely material the two recordings share
    /// (a remix carrying the original's lead), not a play of its own.
    /// Display only; derived from the plays whenever a report is made or
    /// read.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares_material_with: Option<SharedWith>,
    /// Set on a confident play when confident plays of other files over the
    /// same passage (`same_recording`) carry another title in their tags:
    /// those plays, without choosing which title is right. Display only;
    /// derived from the plays whenever a report is made or read.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub other_titles: Vec<OtherTitle>,
    /// The asset's tags as the peak store held them. Absent in older
    /// reports and for files indexed before tags were stored; their names
    /// are read from the files.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<PlayTags>,
}

/// A file's tags, empty when it has none.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlayTags {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
}

impl From<Tags> for PlayTags {
    fn from(tags: Tags) -> PlayTags {
        PlayTags {
            artist: tags.artist,
            title: tags.title,
            album: tags.album,
        }
    }
}

/// A confident play over the same passage whose file has another title.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OtherTitle {
    /// Its number, counted from 1 in the report's order.
    pub play: usize,
    pub asset: String,
    /// `artist - title` as its tags give them.
    pub title: String,
}

/// The confident play a possible play lies inside.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SharedWith {
    /// Its number, counted from 1 in the report's order (as `review`
    /// numbers plays).
    pub play: usize,
    pub asset: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub start_seconds: f64,
    pub end_seconds: f64,
    #[serde(default)]
    pub track_start_seconds: f64,
    #[serde(default)]
    pub track_end_seconds: f64,
    pub speed: f64,
    #[serde(default)]
    pub playback: Playback,
    pub confidence: Level,
    pub windows: u32,
    pub hits: u32,
}

/// How a play was played; under key lock `speed` is the tempo and the
/// pitch is unchanged.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Playback {
    #[default]
    Turntable,
    KeyLocked,
}

impl From<speed::Playback> for Playback {
    fn from(playback: speed::Playback) -> Playback {
        match playback {
            speed::Playback::Turntable => Playback::Turntable,
            speed::Playback::KeyLocked => Playback::KeyLocked,
        }
    }
}

/// `Confidence` as the report spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Level {
    Weak,
    Possible,
    Confident,
}

impl Level {
    pub fn label(self) -> &'static str {
        match self {
            Level::Weak => "weak",
            Level::Possible => "possible",
            Level::Confident => "confident",
        }
    }
}

impl From<Confidence> for Level {
    fn from(confidence: Confidence) -> Level {
        match confidence {
            Confidence::Weak => Level::Weak,
            Confidence::Possible => Level::Possible,
            Confidence::Confident => Level::Confident,
        }
    }
}

impl Report {
    /// `excerpt` is the part of the recording searched; the detections'
    /// times are relative to its start, and `duration` is what was decoded.
    pub fn new(
        catalog: &Catalog,
        audio: &Path,
        excerpt: Excerpt,
        duration: Duration,
        playback: PlaybackChoice,
        detections: &[Detection],
    ) -> Report {
        let offset = excerpt.start.unwrap_or_default().as_secs_f64();
        let mut plays: Vec<FoundPlay> = plays::merge_same_audio(plays::group(detections))
            .iter()
            .map(|same| FoundPlay::new(catalog, same, offset))
            .collect();
        mark_shared_material(&mut plays);
        mark_other_titles(&mut plays);
        Report {
            schema_version: SCHEMA_VERSION,
            query: Query {
                path: audio.to_path_buf(),
                start_seconds: offset,
                duration_seconds: duration.as_secs_f64(),
                playback: Some(playback),
                requested_duration_seconds: excerpt.duration.map(|duration| duration.as_secs_f64()),
            },
            library: absolute(&catalog.root),
            search: Some(SearchSettings::current(catalog.matcher, &catalog.revision)),
            plays,
        }
    }

    pub fn read(path: &Path) -> miette::Result<Report> {
        let text = std::fs::read_to_string(path)
            .into_diagnostic()
            .wrap_err_with(|| format!("cannot read {}", path.display()))?;
        let mut report: Report = serde_json::from_str(&text)
            .into_diagnostic()
            .wrap_err_with(|| format!("{} is not an identify report", path.display()))?;
        if report.schema_version < 2 || report.schema_version > SCHEMA_VERSION {
            return Err(miette!(
                help = "write it again with `gunfinger identify --format json`",
                "{} has schema version {}; this gunfinger reads 2 to {SCHEMA_VERSION}",
                path.display(),
                report.schema_version
            ));
        }
        mark_shared_material(&mut report.plays);
        mark_other_titles(&mut report.plays);
        Ok(report)
    }
}

/// Marks each possible play that lies entirely inside a confident play of
/// another recording with the first such play. A remix played in its own
/// right next to the original reaches past the original's play, or is
/// confident itself, so it is not marked.
fn mark_shared_material(plays: &mut [FoundPlay]) {
    let marks: Vec<Option<SharedWith>> = plays
        .iter()
        .map(|play| {
            if play.confidence != Level::Possible {
                return None;
            }
            plays
                .iter()
                .position(|other| {
                    other.confidence == Level::Confident
                        && other.asset != play.asset
                        && !other.same_audio.contains(&play.asset)
                        && other.start_seconds <= play.start_seconds
                        && play.end_seconds <= other.end_seconds
                })
                .map(|index| SharedWith {
                    play: index + 1,
                    asset: plays[index].asset.clone(),
                })
        })
        .collect();
    for (play, mark) in plays.iter_mut().zip(marks) {
        play.shares_material_with = mark;
    }
}

/// Plays of different assets are one recording when they overlap for at
/// least this share of the shorter play...
const SAME_RECORDING_OVERLAP: f64 = 0.5;
/// ...and the track would have started at the same mix time, to within this
/// many seconds. Uploads of one track agree to 1.6 s in the development mix
/// at NAS scale; the remixes sharing material with a played track lie 86
/// and 193 s away (experiment 0044's report).
const SAME_RECORDING_SECONDS: f64 = 5.0;

/// Marks each confident play with the confident plays of the same
/// recording (`same_recording`) whose tags give another title. Titles are
/// compared by their letters and digits, ignoring case; artists are not
/// compared, and files without a title tag are left out, as their names
/// are not read for titles.
fn mark_other_titles(plays: &mut [FoundPlay]) {
    let marks: Vec<Vec<OtherTitle>> = plays
        .iter()
        .map(|play| {
            let Some(own) = play.compared_title() else {
                return Vec::new();
            };
            plays
                .iter()
                .enumerate()
                .filter(|(_, other)| {
                    other.asset != play.asset
                        && play.same_recording(other)
                        && other.compared_title().is_some_and(|title| title != own)
                })
                .map(|(index, other)| OtherTitle {
                    play: index + 1,
                    asset: other.asset.clone(),
                    title: other.tagged_name().unwrap_or_default(),
                })
                .collect()
        })
        .collect();
    for (play, mark) in plays.iter_mut().zip(marks) {
        play.other_titles = mark;
    }
}

impl FoundPlay {
    /// Whether two plays are one recording on two records: they overlap for
    /// most of the shorter one, and the track would have started at the
    /// same mix time. A possible play marked as sharing material with
    /// another recording is a recording of its own, whatever lines up.
    pub fn same_recording(&self, other: &FoundPlay) -> bool {
        if self.shares_material_with.is_some() || other.shares_material_with.is_some() {
            return false;
        }
        let overlap =
            self.end_seconds.min(other.end_seconds) - self.start_seconds.max(other.start_seconds);
        let shorter =
            (self.end_seconds - self.start_seconds).min(other.end_seconds - other.start_seconds);
        overlap >= SAME_RECORDING_OVERLAP * shorter
            && (self.track_started_at() - other.track_started_at()).abs() <= SAME_RECORDING_SECONDS
    }

    /// The mix time at which the play's track would have started, had it
    /// been played from its beginning at the play's speed: the place in the
    /// track with the record's own speed taken out. Rips and uploads of one
    /// recording at different native speeds agree on it.
    fn track_started_at(&self) -> f64 {
        self.start_seconds - self.track_start_seconds / self.speed
    }

    /// A confident play's title tag as titles are compared: its letters and
    /// digits in lower case, words separated by one space.
    fn compared_title(&self) -> Option<String> {
        if self.confidence != Level::Confident {
            return None;
        }
        let title = self.tags.as_ref()?.title.as_deref()?;
        let words: Vec<String> = title
            .split(|c: char| !c.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .map(str::to_lowercase)
            .collect();
        (!words.is_empty()).then(|| words.join(" "))
    }

    /// `artist - title` from the tags, when they hold a title.
    pub fn tagged_name(&self) -> Option<String> {
        let tags = self.tags.as_ref()?;
        let title = tags.title.as_deref()?;
        Some(TrackName::named(tags.artist.as_deref(), Some(title), &self.asset).full())
    }

    fn new(catalog: &Catalog, same: &SameAudio, offset: f64) -> FoundPlay {
        let path = |asset| catalog.index.asset(asset).path.clone();
        let play = &same.play;
        let total = play.total_evidence();
        let asset = path(play.asset);
        FoundPlay {
            tags: catalog.tags(&asset).map(PlayTags::from),
            asset,
            same_audio: same.also.iter().map(|&asset| path(asset)).collect(),
            start_seconds: offset + play.start_seconds(),
            end_seconds: offset + play.end_seconds(),
            track_start_seconds: play.track_start_seconds(),
            track_end_seconds: play.track_end_seconds(),
            speed: play.speed().0,
            playback: play.playback().into(),
            confidence: play.confidence().into(),
            windows: total.windows,
            hits: total.hits,
            segments: play
                .segments()
                .iter()
                .map(|segment| Segment {
                    start_seconds: offset + segment.start_seconds,
                    end_seconds: offset + segment.end_seconds,
                    track_start_seconds: segment.track_start_seconds,
                    track_end_seconds: segment.track_end_seconds,
                    speed: segment.speed.0,
                    playback: segment.playback.into(),
                    confidence: segment.evidence.confidence().into(),
                    windows: segment.evidence.windows,
                    hits: segment.evidence.hits,
                })
                .collect(),
            shares_material_with: None,
            other_titles: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::tests::report;

    fn marks(plays: &[FoundPlay]) -> Vec<Option<usize>> {
        plays
            .iter()
            .map(|play| play.shares_material_with.as_ref().map(|shared| shared.play))
            .collect()
    }

    fn tagged(artist: &str, title: &str) -> Option<PlayTags> {
        Some(PlayTags {
            artist: Some(artist.to_owned()),
            title: Some(title.to_owned()),
            album: None,
        })
    }

    #[test]
    fn confident_plays_of_one_passage_with_other_titles_name_each_other() {
        let mut plays = report().plays;
        plays[1].tags = tagged("Future Cut", "Sex Drive");
        let found_again = |asset: &str, tags| FoundPlay {
            asset: String::from(asset),
            tags,
            ..plays[1].clone()
        };
        let upload = found_again("upload.m4a", tagged("FUTURE CUT", "the specialist"));
        let rip = found_again("rip.mp3", tagged("Future Cut", "The Specialist"));
        let untagged = found_again("untagged.mp3", None);
        let possible = FoundPlay {
            confidence: Level::Possible,
            ..found_again("weak.mp3", tagged("Someone", "Something Else"))
        };
        plays.extend([upload, rip, untagged, possible]);

        mark_shared_material(&mut plays);
        mark_other_titles(&mut plays);

        let others = |index: usize| -> Vec<(usize, &str)> {
            plays[index]
                .other_titles
                .iter()
                .map(|other| (other.play, other.title.as_str()))
                .collect()
        };
        assert_eq!(
            others(1),
            [
                (5, "FUTURE CUT - the specialist"),
                (6, "Future Cut - The Specialist")
            ]
        );
        assert_eq!(others(4), [(2, "Future Cut - Sex Drive")]);
        assert_eq!(others(5), [(2, "Future Cut - Sex Drive")]);
        for unmarked in [0, 2, 3, 6, 7] {
            assert!(others(unmarked).is_empty(), "play {}", unmarked + 1);
        }
    }

    #[test]
    fn a_possible_play_inside_a_confident_play_of_another_recording_is_marked() {
        let mut plays = report().plays;
        // insert.wav (possible, 1:24-1:34) inside a stretched b.wav.
        plays[1].end_seconds = 95.0;
        // A copy of c.wav, possible, inside c.wav's play: the same audio.
        let mut copy = plays[3].clone();
        copy.asset = String::from("copy-of-c.wav");
        copy.confidence = Level::Possible;
        plays[3].same_audio.push(String::from("copy-of-c.wav"));
        plays.push(copy);
        // A remix that reaches past the original's play.
        let mut remix = plays[2].clone();
        remix.asset = String::from("remix.wav");
        remix.end_seconds = 100.0;
        plays.push(remix);

        mark_shared_material(&mut plays);

        assert_eq!(marks(&plays), [None, None, Some(2), None, None, None]);
        assert_eq!(
            plays[2].shares_material_with,
            Some(SharedWith {
                play: 2,
                asset: String::from("b.wav")
            })
        );
    }
}

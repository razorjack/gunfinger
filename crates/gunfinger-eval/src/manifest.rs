//! Set manifests (`tracklist.toml`): the ground truth of a DJ set.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::library::Library;
use gunfinger_core::timecode::{format_timecode, parse_timecode};
use serde::Deserialize;

/// A set directory with its manifest, validated against the library.
#[derive(Debug)]
pub struct Set {
    pub title: String,
    pub audio: PathBuf,
    pub tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    pub position: u32,
    pub artist: String,
    pub title: String,
    pub start: Duration,
    /// Asset paths relative to the library root. Empty when the track was
    /// played but is deliberately absent from the library.
    pub references: Vec<String>,
}

impl Track {
    pub fn is_referenced(&self) -> bool {
        !self.references.is_empty()
    }

    pub fn label(&self) -> String {
        format!("{:>2}. {} - {}", self.position, self.artist, self.title)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawManifest {
    title: String,
    audio: String,
    #[serde(rename = "track")]
    tracks: Vec<RawTrack>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTrack {
    position: u32,
    artist: String,
    title: String,
    start: String,
    reference: Vec<String>,
    #[expect(
        dead_code,
        reason = "free text for people; parsed only to validate the schema"
    )]
    note: Option<String>,
}

/// Every set directory under `sets_dir` that has a `tracklist.toml`, sorted by
/// name. Directories without one are ignored.
fn set_names(sets_dir: &Path) -> Result<Vec<String>, String> {
    let entries = fs::read_dir(sets_dir)
        .map_err(|error| format!("cannot read {}: {error}", sets_dir.display()))?;
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("tracklist.toml").is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    names.sort();
    Ok(names)
}

/// Loads a set and checks it against the library. Returns every problem
/// found rather than stopping at the first.
pub fn load_set(sets_dir: &Path, name: &str, library: &Library) -> Result<Set, Vec<String>> {
    let dir = sets_dir.join(name);
    let manifest_path = dir.join("tracklist.toml");
    let text = fs::read_to_string(&manifest_path)
        .map_err(|error| vec![format!("cannot read {}: {error}", manifest_path.display())])?;
    let raw: RawManifest = toml::from_str(&text)
        .map_err(|error| vec![format!("{}: {error}", manifest_path.display())])?;

    let mut problems = Vec::new();
    let audio = dir.join(&raw.audio);
    if !audio.is_file() {
        problems.push(format!("audio file {} does not exist", audio.display()));
    }
    let assets: BTreeSet<&str> = library
        .assets
        .iter()
        .map(|asset| asset.path.as_str())
        .collect();
    let mut tracks = Vec::new();
    for (expected_position, raw_track) in (1..).zip(raw.tracks) {
        let label = format!("track {}", raw_track.position);
        if raw_track.position != expected_position {
            problems.push(format!("{label}: expected position {expected_position}"));
        }
        let start = parse_timecode(&raw_track.start).unwrap_or_else(|error| {
            problems.push(format!("{label}: {error}"));
            Duration::ZERO
        });
        if tracks
            .last()
            .is_some_and(|previous: &Track| previous.start >= start)
        {
            problems.push(format!(
                "{label}: start {} is not after the previous track",
                raw_track.start
            ));
        }
        for reference in &raw_track.reference {
            if !assets.contains(reference.as_str()) {
                problems.push(format!(
                    "{label}: reference {reference} is not an audio asset of the library"
                ));
            }
        }
        tracks.push(Track {
            position: raw_track.position,
            artist: raw_track.artist,
            title: raw_track.title,
            start,
            references: raw_track.reference,
        });
    }
    if tracks.is_empty() {
        problems.push("the manifest lists no tracks".to_owned());
    }

    if problems.is_empty() {
        Ok(Set {
            title: raw.title,
            audio,
            tracks,
        })
    } else {
        Err(problems)
    }
}

/// Validates every set under `sets_dir` against the library: a summary per
/// set on stdout, the track lists on stderr.
pub fn validate_all(sets_dir: &Path, library: &Library) -> Result<(), String> {
    let mut invalid = 0;
    for name in set_names(sets_dir)? {
        match load_set(sets_dir, &name, library) {
            Ok(set) => {
                let referenced = set
                    .tracks
                    .iter()
                    .filter(|track| track.is_referenced())
                    .count();
                let references: usize = set.tracks.iter().map(|track| track.references.len()).sum();
                println!(
                    "{name}: valid; {} tracks, {referenced} referenced ({references} reference files), {} absent",
                    set.tracks.len(),
                    set.tracks.len() - referenced
                );
                eprintln!("{name}: {} ({})", set.title, set.audio.display());
                for track in &set.tracks {
                    eprintln!(
                        "  {} {} [{} references]",
                        format_timecode(track.start),
                        track.label(),
                        track.references.len()
                    );
                }
            }
            Err(problems) => {
                invalid += 1;
                println!("{name}: {} problems", problems.len());
                for problem in problems {
                    println!("  {problem}");
                }
            }
        }
    }
    if invalid == 0 {
        Ok(())
    } else {
        Err(format!("{invalid} manifests are invalid"))
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};

    use super::*;

    fn library_with(paths: &[&str]) -> Library {
        Library {
            assets: paths
                .iter()
                .map(|path| Asset {
                    path: (*path).to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                })
                .collect(),
            ..Library::default()
        }
    }

    fn set_dir(name: &str, manifest: &str) -> PathBuf {
        let sets =
            std::env::temp_dir().join(format!("gunfinger-sets-{}-{name}", std::process::id()));
        let dir = sets.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("tracklist.toml"), manifest).unwrap();
        fs::write(dir.join("mix.m4a"), b"").unwrap();
        sets
    }

    const VALID: &str = r#"
        title = "A mix"
        audio = "mix.m4a"

        [[track]]
        position  = 1
        artist    = "Bad Company"
        title     = "The Nine"
        start     = "0:00"
        reference = ["Bad Company - The Nine.mp3"]

        [[track]]
        position  = 2
        artist    = "Somebody"
        title     = "Absent"
        start     = "4:30"
        reference = []
        note      = "deliberately absent"
    "#;

    #[test]
    fn a_valid_manifest_loads() {
        let sets = set_dir("valid", VALID);

        let set = load_set(
            &sets,
            "valid",
            &library_with(&["Bad Company - The Nine.mp3"]),
        )
        .unwrap();

        assert_eq!(set.tracks.len(), 2);
        assert_eq!(set.tracks[1].start, Duration::from_secs(270));
        assert!(set.tracks[0].is_referenced());
        assert!(!set.tracks[1].is_referenced());
    }

    #[test]
    fn every_problem_is_reported() {
        let manifest = VALID
            .replace("\"4:30\"", "\"0:00\"")
            .replace("position  = 2", "position  = 3");
        let sets = set_dir("broken", &manifest);

        let problems = load_set(&sets, "broken", &library_with(&[])).unwrap_err();

        assert_eq!(problems.len(), 3, "{problems:?}");
        assert!(problems[0].contains("not an audio asset"));
        assert!(problems[1].contains("expected position 2"));
        assert!(problems[2].contains("not after the previous track"));
    }

    #[test]
    fn unknown_fields_are_rejected() {
        let sets = set_dir("unknown", &VALID.replace("note      =", "notes     ="));

        assert!(
            load_set(
                &sets,
                "unknown",
                &library_with(&["Bad Company - The Nine.mp3"])
            )
            .is_err()
        );
    }
}

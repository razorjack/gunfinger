//! Set manifests (`tracklist.toml`): the ground truth of a DJ set.
//!
//! A reference names a corpus library file, or a file of the larger
//! library the corpus was drawn from as `second-library/<path>`, the name
//! the harness gives it with `--other-peaks-dir`. Without that library a
//! reference to it is set aside, so a track with no other reference counts
//! as absent.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::library::{Asset, Library};
use gunfinger_core::timecode::{format_timecode, parse_timecode};
use serde::Deserialize;

use crate::padding::SECOND_LIBRARY_PREFIX;

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
    /// The references that name files of the libraries searched: corpus
    /// library paths, and `second-library/<path>` with the other library.
    /// Empty when the track was played but is absent from them.
    pub references: Vec<String>,
    /// References to the other library's files, set aside when it is not
    /// searched.
    pub set_aside: Vec<String>,
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

/// The files a manifest's references may name.
pub struct Referable {
    corpus: BTreeSet<String>,
    /// The other library's files with a current peak record, named
    /// `second-library/<path>`, and its store; `None` when it is not
    /// searched.
    other: Option<(BTreeSet<String>, PathBuf)>,
}

impl Referable {
    pub fn corpus(library: &Library) -> Referable {
        Referable {
            corpus: library
                .assets
                .iter()
                .map(|asset| asset.path.clone())
                .collect(),
            other: None,
        }
    }

    /// With the files `store` (the other library's) holds a current record of.
    pub fn with_other(self, assets: &[Asset], store: &Path) -> Referable {
        let files = assets
            .iter()
            .map(|asset| format!("{SECOND_LIBRARY_PREFIX}{}", asset.path))
            .collect();
        Referable {
            other: Some((files, store.to_owned())),
            ..self
        }
    }

    fn standing(&self, reference: &str) -> Standing {
        match (reference.starts_with(SECOND_LIBRARY_PREFIX), &self.other) {
            (false, _) if self.corpus.contains(reference) => Standing::Kept,
            (false, _) => Standing::Problem(format!(
                "reference {reference} is not an audio asset of the library"
            )),
            (true, None) => Standing::SetAside,
            (true, Some((files, _))) if files.contains(reference) => Standing::Kept,
            (true, Some((_, store))) => Standing::Problem(format!(
                "reference {reference} has no current peak record in {}",
                store.display()
            )),
        }
    }
}

/// Where a reference stands: kept, set aside, or a problem.
enum Standing {
    Kept,
    SetAside,
    Problem(String),
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

/// Loads a set and checks its references against the files they may name.
/// Returns every problem found rather than stopping at the first.
pub fn load_set(sets_dir: &Path, name: &str, referable: &Referable) -> Result<Set, Vec<String>> {
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
        let mut references = Vec::new();
        let mut set_aside = Vec::new();
        for reference in raw_track.reference {
            match referable.standing(&reference) {
                Standing::Kept => references.push(reference),
                Standing::SetAside => set_aside.push(reference),
                Standing::Problem(problem) => problems.push(format!("{label}: {problem}")),
            }
        }
        tracks.push(Track {
            position: raw_track.position,
            artist: raw_track.artist,
            title: raw_track.title,
            start,
            references,
            set_aside,
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

/// Validates the set `only`, or every set under `sets_dir`, against the
/// files its references may name: a summary per set on stdout, the track
/// lists on stderr.
pub fn validate(sets_dir: &Path, only: Option<&str>, referable: &Referable) -> Result<(), String> {
    let names = match only {
        Some(name) => vec![name.to_owned()],
        None => set_names(sets_dir)?,
    };
    let mut invalid = 0;
    for name in names {
        match load_set(sets_dir, &name, referable) {
            Ok(set) => {
                let referenced = set
                    .tracks
                    .iter()
                    .filter(|track| track.is_referenced())
                    .count();
                let references: usize = set.tracks.iter().map(|track| track.references.len()).sum();
                let set_aside: usize = set.tracks.iter().map(|track| track.set_aside.len()).sum();
                let only_set_aside = set
                    .tracks
                    .iter()
                    .filter(|track| !track.is_referenced() && !track.set_aside.is_empty())
                    .count();
                let aside = if set_aside == 0 {
                    String::new()
                } else {
                    format!(
                        "; {set_aside} references to the other library set aside without --other-peaks-dir, {only_set_aside} of the absent tracks have only those"
                    )
                };
                println!(
                    "{name}: valid; {} tracks, {referenced} referenced ({references} reference files), {} absent{aside}",
                    set.tracks.len(),
                    set.tracks.len() - referenced
                );
                eprintln!("{name}: {} ({})", set.title, set.audio.display());
                for track in &set.tracks {
                    eprintln!(
                        "  {} {} [{} references, {} set aside]",
                        format_timecode(track.start),
                        track.label(),
                        track.references.len(),
                        track.set_aside.len()
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
    use gunfinger_core::library::Timestamp;

    use super::*;

    fn asset(path: &str) -> Asset {
        Asset {
            path: path.to_owned(),
            size: 1,
            modified: Timestamp {
                seconds: 0,
                nanos: 0,
            },
        }
    }

    fn library_with(paths: &[&str]) -> Referable {
        Referable::corpus(&Library {
            assets: paths.iter().map(|path| asset(path)).collect(),
            ..Library::default()
        })
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

    const ON_THE_OTHER_LIBRARY: &str = r#"
        title = "A mix"
        audio = "mix.m4a"

        [[track]]
        position  = 1
        artist    = "Bad Company"
        title     = "The Nine"
        start     = "0:00"
        reference = ["Bad Company - The Nine.mp3", "second-library/rips/the_nine.mp3"]

        [[track]]
        position  = 2
        artist    = "Kemal & Rob Data"
        title     = "Konspiracy"
        start     = "4:30"
        reference = ["second-library/rips/konspiracy.mp3"]
    "#;

    #[test]
    fn references_to_the_other_library_are_set_aside_without_it() {
        let sets = set_dir("set-aside", ON_THE_OTHER_LIBRARY);

        let set = load_set(
            &sets,
            "set-aside",
            &library_with(&["Bad Company - The Nine.mp3"]),
        )
        .unwrap();

        assert_eq!(set.tracks[0].references, ["Bad Company - The Nine.mp3"]);
        assert_eq!(
            set.tracks[0].set_aside,
            ["second-library/rips/the_nine.mp3"]
        );
        assert!(
            !set.tracks[1].is_referenced(),
            "absent at the corpus's size"
        );
        assert_eq!(set.tracks[1].set_aside.len(), 1);
    }

    #[test]
    fn references_to_the_other_library_need_a_current_record_with_it() {
        let sets = set_dir("other", ON_THE_OTHER_LIBRARY);
        let store = Path::new("/peaks");
        let with = |records: &[&str]| {
            let records: Vec<Asset> = records.iter().map(|path| asset(path)).collect();
            library_with(&["Bad Company - The Nine.mp3"]).with_other(&records, store)
        };

        let set = load_set(
            &sets,
            "other",
            &with(&["rips/the_nine.mp3", "rips/konspiracy.mp3"]),
        )
        .unwrap();
        let problems = load_set(&sets, "other", &with(&["rips/the_nine.mp3"])).unwrap_err();

        assert_eq!(
            set.tracks[1].references,
            ["second-library/rips/konspiracy.mp3"]
        );
        assert!(set.tracks.iter().all(|track| track.set_aside.is_empty()));
        assert_eq!(
            problems,
            [
                "track 2: reference second-library/rips/konspiracy.mp3 has no current peak record in /peaks"
            ]
        );
    }
}

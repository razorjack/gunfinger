//! Finding the audio files of a library.
//!
//! An asset's identity is its path relative to the library root, with `/`
//! separators. Nothing is read from file names beyond the extension. The
//! owner can leave files and folders out with an ignore file at the root
//! (`ignore`).

pub mod ignore;

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use ignore::{Entry, IGNORE_FILE, IgnoreError, Pattern};

const AUDIO_EXTENSIONS: [&str; 7] = ["mp3", "m4a", "opus", "ogg", "flac", "wav", "aiff"];

/// An audio file in the library, as found on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Path relative to the library root, `/`-separated.
    pub path: String,
    pub size: u64,
    pub modified: Timestamp,
}

impl Asset {
    /// Whether the file at `path` still has this size and modification
    /// time.
    pub fn matches_file(&self, path: &Path) -> bool {
        fs::metadata(path).is_ok_and(|metadata| {
            metadata.len() == self.size
                && Timestamp::of(metadata.modified().unwrap_or(UNIX_EPOCH)) == self.modified
        })
    }
}

/// A file modification time with nanosecond precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub seconds: i64,
    pub nanos: u32,
}

impl Timestamp {
    fn of(time: SystemTime) -> Self {
        match time.duration_since(UNIX_EPOCH) {
            Ok(after) => Timestamp {
                seconds: i64::try_from(after.as_secs()).unwrap_or(i64::MAX),
                nanos: after.subsec_nanos(),
            },
            // Before 1970: precision is irrelevant, only equality matters.
            Err(before) => Timestamp {
                seconds: -i64::try_from(before.duration().as_secs()).unwrap_or(i64::MAX),
                nanos: 0,
            },
        }
    }
}

/// The audio assets under a root, sorted by path, and what was passed over
/// or left out.
#[derive(Debug, Default)]
pub struct Library {
    pub root: PathBuf,
    pub assets: Vec<Asset>,
    /// Skipped files and folders by reason, such as `extension .nfo`,
    /// `hidden file` or `hidden folder`.
    pub skipped: BTreeMap<String, usize>,
    /// The ignore file read at the root; `None` when there is none.
    pub ignore_file: Option<PathBuf>,
    /// Each of its patterns, in file order, with the audio files it left
    /// out.
    pub ignored: Vec<Ignored>,
}

/// A pattern of the ignore file and what it left out.
#[derive(Debug)]
pub struct Ignored {
    pub pattern: Pattern,
    /// Paths relative to the root, sorted, of the audio files that would
    /// be assets without the pattern. A file inside an ignored folder
    /// counts for the first pattern that left out its outermost ignored
    /// folder; any other file for the first pattern that matches it.
    pub paths: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ScanError {
    /// The library could not be listed, such as an unmounted share.
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Ignore(#[from] IgnoreError),
}

impl Library {
    /// Walks `root` recursively, leaving out what its ignore file lists.
    /// Directory symlinks are not followed, so a link loop cannot trap the
    /// scan.
    pub fn scan(root: &Path) -> Result<Library, ScanError> {
        Library::scan_with_progress(root, |_| {})
    }

    /// As `scan`, calling `progress` with the number of files seen so far
    /// after each one: a large network share takes minutes to list.
    pub fn scan_with_progress(
        root: &Path,
        mut progress: impl FnMut(usize),
    ) -> Result<Library, ScanError> {
        let patterns = ignore::read(root)?;
        let mut library = Library {
            root: root.to_owned(),
            ignore_file: patterns.is_some().then(|| root.join(IGNORE_FILE)),
            ignored: patterns
                .unwrap_or_default()
                .into_iter()
                .map(|pattern| Ignored {
                    pattern,
                    paths: Vec::new(),
                })
                .collect(),
            ..Library::default()
        };
        library.scan_dir(root, None, &mut progress)?;
        library.assets.sort_by(|a, b| a.path.cmp(&b.path));
        for ignored in &mut library.ignored {
            ignored.paths.sort();
        }
        Ok(library)
    }

    pub fn absolute_path(&self, asset: &Asset) -> PathBuf {
        self.root.join(&asset.path)
    }

    pub fn skipped_total(&self) -> usize {
        self.skipped.values().sum()
    }

    /// The audio files the ignore file left out, by path.
    pub fn ignored_paths(&self) -> impl Iterator<Item = &str> {
        self.ignored
            .iter()
            .flat_map(|ignored| ignored.paths.iter().map(String::as_str))
    }

    pub fn ignored_total(&self) -> usize {
        self.ignored.iter().map(|ignored| ignored.paths.len()).sum()
    }

    /// `ignored_by` is the pattern, by its place in `ignored`, that left out
    /// `dir` or a folder above it.
    fn scan_dir(
        &mut self,
        dir: &Path,
        ignored_by: Option<usize>,
        progress: &mut impl FnMut(usize),
    ) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let kind = entry.file_type()?;
            if kind.is_dir() {
                if is_hidden(&path) {
                    // Such as a downloader's `.incomplete/` of partial files.
                    self.skip("hidden folder");
                } else {
                    let ignored_by = ignored_by.or_else(|| self.pattern_for(&path, Entry::Folder));
                    self.scan_dir(&path, ignored_by, progress)?;
                }
                continue;
            }
            if kind.is_file() {
                self.consider_file(&path, &entry.metadata()?, ignored_by);
            } else {
                self.skip("symlink or special file");
            }
            progress(self.assets.len() + self.ignored_total() + self.skipped_total());
        }
        Ok(())
    }

    /// The first pattern that leaves out the file or folder at `path`.
    fn pattern_for(&self, path: &Path, entry: Entry) -> Option<usize> {
        let relative = self.relative_path(path)?;
        self.ignored
            .iter()
            .position(|ignored| ignored.pattern.matches(&relative, entry))
    }

    fn consider_file(&mut self, path: &Path, metadata: &fs::Metadata, ignored_by: Option<usize>) {
        if is_hidden(path) {
            // Includes `.DS_Store` and the `._*` resource forks macOS leaves
            // on network shares, which carry audio extensions but no audio.
            self.skip("hidden file");
            return;
        }
        let extension = path
            .extension()
            .map(|extension| extension.to_string_lossy().to_lowercase());
        let Some(extension) = extension else {
            self.skip("no extension");
            return;
        };
        if !AUDIO_EXTENSIONS.contains(&extension.as_str()) {
            self.skip(&format!("extension .{extension}"));
            return;
        }
        let Some(relative) = self.relative_path(path) else {
            self.skip("path is not valid UTF-8");
            return;
        };
        match ignored_by.or_else(|| self.pattern_for(path, Entry::File)) {
            Some(pattern) => self.ignored[pattern].paths.push(relative),
            None => self.assets.push(Asset {
                path: relative,
                size: metadata.len(),
                modified: Timestamp::of(metadata.modified().unwrap_or(UNIX_EPOCH)),
            }),
        }
    }

    fn relative_path(&self, path: &Path) -> Option<String> {
        let relative = path.strip_prefix(&self.root).ok()?;
        let parts: Option<Vec<&str>> = relative.iter().map(|part| part.to_str()).collect();
        Some(parts?.join("/"))
    }

    fn skip(&mut self, reason: &str) {
        *self.skipped.entry(reason.to_owned()).or_default() += 1;
    }
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .is_some_and(|name| name.to_string_lossy().starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(path: &Path) {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, b"x").unwrap();
    }

    #[test]
    fn scanning_keeps_audio_and_counts_the_rest() {
        let root = std::env::temp_dir().join(format!("gunfinger-library-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("Artist - Title.MP3"));
        touch(&root.join("release/01-track.flac"));
        touch(&root.join("release/info.nfo"));
        touch(&root.join("release/cover.jpg"));
        touch(&root.join("release/02-track.mp3_bad_or_incomplete"));
        touch(&root.join("release/._01-track.flac"));

        let library = Library::scan(&root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        let paths: Vec<&str> = library
            .assets
            .iter()
            .map(|asset| asset.path.as_str())
            .collect();
        assert_eq!(paths, ["Artist - Title.MP3", "release/01-track.flac"]);
        assert_eq!(library.skipped["extension .nfo"], 1);
        assert_eq!(library.skipped["extension .jpg"], 1);
        assert_eq!(library.skipped["extension .mp3_bad_or_incomplete"], 1);
        assert_eq!(library.skipped["hidden file"], 1);
        assert_eq!(library.skipped_total(), 4);
    }

    #[test]
    fn hidden_folders_are_passed_over_whole() {
        let root = std::env::temp_dir().join(format!("gunfinger-hidden-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("channel/Artist - Title.m4a"));
        touch(&root.join("channel/.incomplete/Artist - Partial.m4a"));
        touch(&root.join("channel/.incomplete/nested/Artist - Other.m4a"));
        touch(&root.join(".trash/Artist - Deleted.mp3"));

        let library = Library::scan(&root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        let paths: Vec<&str> = library
            .assets
            .iter()
            .map(|asset| asset.path.as_str())
            .collect();
        assert_eq!(paths, ["channel/Artist - Title.m4a"]);
        assert_eq!(library.skipped["hidden folder"], 2);
        assert_eq!(library.skipped_total(), 2);
    }

    #[test]
    fn the_ignore_file_leaves_out_folders_and_files() {
        let root = std::env::temp_dir().join(format!("gunfinger-ignored-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("Mixed CD (2000)/CD1/01-track.mp3"));
        touch(&root.join("Mixed CD (2000)/CD2/01-track.mp3"));
        touch(&root.join("Mixed CD (2000)/CD2/info.nfo"));
        touch(&root.join("Album [Virus]/CD1/01-track.mp3"));
        touch(&root.join("Album [Virus]/CD2/01-track.mp3"));
        touch(&root.join("Album [Virus]/CD2/02-track.mp3"));
        touch(&root.join("Artist - Title.mp3"));
        touch(&root.join("Artist - Title (radio edit).mp3"));
        fs::write(
            root.join(".gunfingerignore"),
            "# Mixed CDs\n\
             /Mixed CD (2000)/\n\
             /Album [Virus]/CD2/\n\
             *(radio edit).mp3\n\
             /Renamed since/\n",
        )
        .unwrap();

        let library = Library::scan(&root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        let paths: Vec<&str> = library
            .assets
            .iter()
            .map(|asset| asset.path.as_str())
            .collect();
        assert_eq!(
            paths,
            ["Album [Virus]/CD1/01-track.mp3", "Artist - Title.mp3"]
        );
        assert_eq!(library.ignore_file, Some(root.join(".gunfingerignore")));
        let left_out: Vec<(&str, Vec<&str>)> = library
            .ignored
            .iter()
            .map(|ignored| {
                (
                    ignored.pattern.text.as_str(),
                    ignored.paths.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        assert_eq!(
            left_out,
            [
                (
                    "/Mixed CD (2000)/",
                    vec![
                        "Mixed CD (2000)/CD1/01-track.mp3",
                        "Mixed CD (2000)/CD2/01-track.mp3"
                    ]
                ),
                (
                    "/Album [Virus]/CD2/",
                    vec![
                        "Album [Virus]/CD2/01-track.mp3",
                        "Album [Virus]/CD2/02-track.mp3"
                    ]
                ),
                ("*(radio edit).mp3", vec!["Artist - Title (radio edit).mp3"]),
                ("/Renamed since/", vec![]),
            ]
        );
        assert_eq!(library.ignored_total(), 5);
        assert_eq!(library.skipped["hidden file"], 1, "the ignore file itself");
        assert_eq!(library.skipped["extension .nfo"], 1);
    }

    #[test]
    fn a_file_left_out_by_two_patterns_counts_for_its_outermost_folder() {
        let root = std::env::temp_dir().join(format!("gunfinger-overlap-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("Album/CD2/01-track.mp3"));
        fs::write(root.join(".gunfingerignore"), "/Album/CD2/\n/Album/\n").unwrap();

        let library = Library::scan(&root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert!(library.assets.is_empty());
        assert!(library.ignored[0].paths.is_empty());
        assert_eq!(library.ignored[1].paths, ["Album/CD2/01-track.mp3"]);
    }

    #[test]
    fn without_an_ignore_file_nothing_is_left_out() {
        let root = std::env::temp_dir().join(format!("gunfinger-unignored-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("Mixed CD (2000)/CD1/01-track.mp3"));

        let library = Library::scan(&root).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert_eq!(library.assets.len(), 1);
        assert_eq!(library.ignore_file, None);
        assert!(library.ignored.is_empty());
    }

    #[test]
    fn an_invalid_ignore_file_stops_the_scan() {
        let root =
            std::env::temp_dir().join(format!("gunfinger-bad-ignore-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        touch(&root.join("Album/CD1/01-track.mp3"));
        fs::write(root.join(".gunfingerignore"), "/Album/\n!/Album/CD1/\n").unwrap();

        let scanned = Library::scan(&root);
        fs::remove_dir_all(&root).unwrap();

        assert!(
            matches!(
                scanned,
                Err(ScanError::Ignore(IgnoreError::Invalid { line: 2, .. }))
            ),
            "{scanned:?}"
        );
    }
}

//! Finding the audio files of a library.
//!
//! An asset's identity is its path relative to the library root, with `/`
//! separators. Nothing is read from file names beyond the extension.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const AUDIO_EXTENSIONS: [&str; 7] = ["mp3", "m4a", "opus", "ogg", "flac", "wav", "aiff"];

/// An audio file in the library, as found on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// Path relative to the library root, `/`-separated.
    pub path: String,
    pub size: u64,
    pub modified: Timestamp,
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

/// The audio assets under a root, sorted by path, and what was passed over.
#[derive(Debug, Default)]
pub struct Library {
    pub root: PathBuf,
    pub assets: Vec<Asset>,
    /// Skipped files by reason, such as `extension .nfo` or `hidden file`.
    pub skipped: BTreeMap<String, usize>,
}

impl Library {
    /// Walks `root` recursively. Directory symlinks are not followed, so a
    /// link loop cannot trap the scan.
    pub fn scan(root: &Path) -> io::Result<Library> {
        let mut library = Library {
            root: root.to_owned(),
            ..Library::default()
        };
        library.scan_dir(root)?;
        library.assets.sort_by(|a, b| a.path.cmp(&b.path));
        Ok(library)
    }

    pub fn absolute_path(&self, asset: &Asset) -> PathBuf {
        self.root.join(&asset.path)
    }

    pub fn skipped_total(&self) -> usize {
        self.skipped.values().sum()
    }

    fn scan_dir(&mut self, dir: &Path) -> io::Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let kind = entry.file_type()?;
            if kind.is_dir() {
                self.scan_dir(&path)?;
            } else if kind.is_file() {
                self.consider_file(&path, &entry.metadata()?);
            } else {
                self.skip("symlink or special file");
            }
        }
        Ok(())
    }

    fn consider_file(&mut self, path: &Path, metadata: &fs::Metadata) {
        let name = path.file_name().map(|name| name.to_string_lossy());
        if name.is_some_and(|name| name.starts_with('.')) {
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
        self.assets.push(Asset {
            path: relative,
            size: metadata.len(),
            modified: Timestamp::of(metadata.modified().unwrap_or(UNIX_EPOCH)),
        });
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
}

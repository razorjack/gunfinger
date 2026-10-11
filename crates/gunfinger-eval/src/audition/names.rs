//! How the harness names a file, and back: a corpus library path
//! (`DJ Trace - Sonar.m4a`), `second-library/<path>` for a file of the
//! other library (the NAS), an absolute path for anything else. The pair
//! verdicts file and the clusters use these names; lanes need absolute
//! paths.

use std::path::{Path, PathBuf};

use crate::padding::SECOND_LIBRARY_PREFIX;

/// The two library roots names are relative to, both absolute.
#[derive(Debug, Clone)]
pub struct Libraries {
    pub corpus: PathBuf,
    pub second: PathBuf,
}

impl Libraries {
    /// The harness name of an absolute path.
    pub fn name(&self, path: &Path) -> String {
        if let Ok(relative) = path.strip_prefix(&self.second) {
            return format!("{SECOND_LIBRARY_PREFIX}{}", relative.display());
        }
        match path.strip_prefix(&self.corpus) {
            Ok(relative) => relative.display().to_string(),
            Err(_) => path.display().to_string(),
        }
    }

    /// The absolute path of a harness name.
    pub fn path(&self, name: &str) -> PathBuf {
        match name.strip_prefix(SECOND_LIBRARY_PREFIX) {
            Some(relative) => self.second.join(relative),
            None if Path::new(name).is_absolute() => PathBuf::from(name),
            None => self.corpus.join(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_paths_are_each_others_inverse() {
        let libraries = Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            second: PathBuf::from("/Volumes/nas/music"),
        };

        for (name, path) in [
            ("DJ X - Tune.m4a", "/repo/corpus/library/DJ X - Tune.m4a"),
            (
                "second-library/label/b.opus",
                "/Volumes/nas/music/label/b.opus",
            ),
            ("/elsewhere/mix.mp3", "/elsewhere/mix.mp3"),
        ] {
            assert_eq!(libraries.path(name), PathBuf::from(path));
            assert_eq!(libraries.name(Path::new(path)), name);
        }
    }
}

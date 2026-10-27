//! The judgments that two library files carry the same track title
//! although they are different recordings (ADR 0022): a near-identical
//! version, which the verdicts by ear keep out of the recording's
//! cluster. Read from a plain text file (`docs/same-titles.txt`), our
//! judgments written by hand: one judgment per line, `same-title` and the
//! two files, separated by tabs; `#` starts a comment line.
//!
//! A judgment relates the two files' duplicate clusters, for scoring
//! only: a detection of a file of one, where the truth is the other,
//! names the right title. Judgments are not followed through chains.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::index::Index;

use crate::clusters::Clusters;

const KEYWORD: &str = "same-title";

#[derive(Debug, Default)]
pub struct SameTitles {
    /// The file they were read from, for messages.
    file: PathBuf,
    /// Pairs of files in path order, with their line in the file.
    pairs: BTreeMap<(String, String), usize>,
}

impl SameTitles {
    /// The judgments in `path`; none when the file does not exist.
    pub fn load(path: &Path) -> Result<SameTitles, String> {
        match fs::read_to_string(path) {
            Ok(text) => SameTitles::parse(&text)
                .map(|titles| SameTitles {
                    file: path.to_owned(),
                    ..titles
                })
                .map_err(|error| format!("{}, {error}", path.display())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(SameTitles::default()),
            Err(error) => Err(format!("cannot read {}: {error}", path.display())),
        }
    }

    pub fn parse(text: &str) -> Result<SameTitles, String> {
        let mut lines: BTreeMap<(String, String), usize> = BTreeMap::new();
        for (index, line) in text.lines().enumerate() {
            let number = index + 1;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.first().map(|keyword| keyword.trim()) != Some(KEYWORD) {
                return Err(format!("line {number}: a judgment starts with `{KEYWORD}`"));
            }
            let [_, a, b] = fields[..] else {
                return Err(format!(
                    "line {number}: `{KEYWORD}` and two files, separated by tabs"
                ));
            };
            if a == b {
                return Err(format!(
                    "line {number}: {a} is judged against itself; name two files"
                ));
            }
            if let Some(earlier) = lines.insert(ordered(a, b), number) {
                return Err(format!(
                    "line {number}: {a} and {b} are already judged on line {earlier}; keep one line"
                ));
            }
        }
        Ok(SameTitles {
            file: PathBuf::new(),
            pairs: lines,
        })
    }

    /// The files outside `recording` (a cluster's members) whose
    /// detections name its title: the cluster of every file judged against
    /// one of its members.
    pub fn credited(&self, clusters: &Clusters, recording: &BTreeSet<String>) -> BTreeSet<String> {
        self.pairs
            .keys()
            .filter_map(|(a, b)| {
                if recording.contains(a) {
                    Some(b)
                } else if recording.contains(b) {
                    Some(a)
                } else {
                    None
                }
            })
            .flat_map(|partner| clusters.cluster_of(partner))
            .filter(|file| !recording.contains(file))
            .collect()
    }

    /// Warns of every judged file that is neither in `index` nor left out
    /// of it by the run (`left_out`), so that a misspelt name, which would
    /// credit nothing, does not pass unnoticed.
    pub fn warn_outside(&self, index: &Index, left_out: &BTreeSet<String>) {
        let indexed: BTreeSet<&str> = index
            .assets()
            .iter()
            .map(|asset| asset.path.as_str())
            .collect();
        for ((a, b), line) in &self.pairs {
            for file in [a, b] {
                if !indexed.contains(file.as_str()) && !left_out.contains(file) {
                    eprintln!(
                        "{}, line {line}: {file} is not a file this run indexes; the judgment adds nothing",
                        self.file.display()
                    );
                }
            }
        }
    }
}

/// Whether a report has no same-title credits to write: their counts are
/// left out when zero, so earlier reports read the same.
// serde's `skip_serializing_if` passes a reference.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn no_credits(count: &usize) -> bool {
    *count == 0
}

fn ordered(a: &str, b: &str) -> (String, String) {
    if a <= b {
        (a.to_owned(), b.to_owned())
    } else {
        (b.to_owned(), a.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn clusters(duplicates: &[&[&str]]) -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates: duplicates
                .iter()
                .map(|members| members.iter().map(|member| (*member).to_owned()).collect())
                .collect(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        }
    }

    fn set(files: &[&str]) -> BTreeSet<String> {
        files.iter().map(|file| (*file).to_owned()).collect()
    }

    #[test]
    fn a_judgment_credits_the_other_files_cluster_to_either_side() {
        let titles = SameTitles::parse(
            "# A comment, then an empty line.\n\nsame-title\tb-rip.mp3\tsecond-library/version.opus\n",
        )
        .unwrap();
        let clusters = clusters(&[
            &["a-rip.mp3", "b-rip.mp3"],
            &[
                "second-library/version.opus",
                "second-library/version-copy.m4a",
            ],
        ]);
        assert_eq!(
            titles.credited(&clusters, &clusters.cluster_of("a-rip.mp3")),
            set(&[
                "second-library/version.opus",
                "second-library/version-copy.m4a"
            ])
        );
        assert_eq!(
            titles.credited(
                &clusters,
                &clusters.cluster_of("second-library/version-copy.m4a")
            ),
            set(&["a-rip.mp3", "b-rip.mp3"])
        );
        assert!(
            titles
                .credited(&clusters, &clusters.cluster_of("other.mp3"))
                .is_empty()
        );
    }

    #[test]
    fn judgments_are_not_followed_through_chains() {
        let titles =
            SameTitles::parse("same-title\ta.mp3\tb.mp3\nsame-title\tb.mp3\tc.mp3\n").unwrap();
        let clusters = clusters(&[]);
        assert_eq!(
            titles.credited(&clusters, &clusters.cluster_of("a.mp3")),
            set(&["b.mp3"])
        );
        assert_eq!(
            titles.credited(&clusters, &clusters.cluster_of("b.mp3")),
            set(&["a.mp3", "c.mp3"])
        );
    }

    #[test]
    fn the_committed_file_parses_and_holds_the_china_cup_judgment() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/same-titles.txt");
        let titles = SameTitles::load(&path).unwrap();
        assert!(titles.pairs.contains_key(&ordered(
            "extra/b-bad_company-china_cup.mp3",
            "second-library/__youtube_archivists/sicknote-classic-dnb/ƆƐI3C - China Cup.opus"
        )));
    }

    #[test]
    fn a_missing_file_holds_no_judgments() {
        let titles = SameTitles::load(Path::new("/nonexistent/same-titles.txt")).unwrap();
        assert!(titles.pairs.is_empty());
    }

    #[test]
    fn errors_name_the_file_and_the_line() {
        let dir = std::env::temp_dir().join(format!("gunfinger-titles-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("same-titles.txt");
        fs::write(&path, "# comment\nsame\ta.mp3\tb.mp3\n").unwrap();
        let error = SameTitles::load(&path).unwrap_err();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(
            error,
            format!(
                "{}, line 2: a judgment starts with `same-title`",
                path.display()
            )
        );
    }

    #[test]
    fn malformed_lines_are_refused() {
        let cases = [
            ("same-title\ta.mp3\n", "line 1: `same-title` and two files"),
            (
                "same-title\ta.mp3\tb.mp3\tc.mp3\n",
                "line 1: `same-title` and two files",
            ),
            ("same-title a.mp3 b.mp3\n", "line 1: a judgment starts with"),
            (
                "same-title\ta.mp3\ta.mp3\n",
                "line 1: a.mp3 is judged against itself",
            ),
            (
                "same-title\ta.mp3\tb.mp3\n\nsame-title\tb.mp3\ta.mp3\n",
                "line 3: b.mp3 and a.mp3 are already judged on line 1",
            ),
        ];
        for (text, expected) in cases {
            let error = SameTitles::parse(text).unwrap_err();
            assert!(error.starts_with(expected), "{text:?}: {error}");
        }
    }
}

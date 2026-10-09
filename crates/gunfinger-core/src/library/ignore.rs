//! The library's ignore file: `.gunfingerignore` at its root lists the
//! files and folders the owner leaves out of the library, such as the
//! tracks of a DJ-mixed CD, which already blend into each other.
//!
//! The syntax is a subset of `.gitignore`. One pattern per line; blank
//! lines and lines starting with `#` are skipped and trailing spaces
//! trimmed. `*` matches any characters within one name and `?` one
//! character; `**` as a whole name matches any number of folders. A
//! trailing `/` matches folders only. A pattern with a `/` at its start or
//! in its middle is a path from the root; otherwise it matches a name at
//! any depth. Matching is case-sensitive, as asset paths are. Unlike Git,
//! square brackets are literal, because release folders are full of them
//! (`[Virus]`), and negation (`!`) and backslash escapes are refused.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The name of the ignore file at a library's root.
pub const IGNORE_FILE: &str = ".gunfingerignore";

/// One line of an ignore file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    /// As written, without trailing spaces.
    pub text: String,
    /// The line it is on, counted from 1.
    pub line: usize,
    shape: Shape,
    target: Target,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Shape {
    /// No `/` other than a trailing one: a name at any depth.
    Name(String),
    /// The names of a path from the root; a name `**` stands for any
    /// number of them.
    FromRoot(Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    FilesAndFolders,
    /// Written with a trailing `/`.
    Folders,
}

/// What a library path names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Entry {
    File,
    Folder,
}

/// Why a line is not a pattern this file accepts. Each message says what
/// to write instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum Problem {
    #[error(
        "`!` (negation) is not supported: the file only lists what to leave out. To keep part of a folder, list the parts to leave out instead, such as `/Album/CD2/` rather than `/Album/` and `!/Album/CD1/`"
    )]
    Negation,
    #[error(
        "a backslash is not supported: write each name as it is, with `/` between folders; `*` and `?` match themselves too, and brackets are literal"
    )]
    Backslash,
    #[error(
        "the pattern names no file or folder: write one, such as `/Album/` for a folder at the root"
    )]
    NoName,
    #[error("`//` leaves an empty name: write a single `/` between folders")]
    EmptyName,
}

#[derive(Debug, thiserror::Error)]
pub enum IgnoreError {
    #[error("could not read the ignore file {path}: {source}")]
    Unreadable { path: PathBuf, source: io::Error },
    #[error("{path}, line {line}: {problem}")]
    Invalid {
        path: PathBuf,
        line: usize,
        problem: Problem,
    },
}

/// The patterns of the ignore file at `root`, or `None` when it has none.
pub fn read(root: &Path) -> Result<Option<Vec<Pattern>>, IgnoreError> {
    let path = root.join(IGNORE_FILE);
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        // A root that is missing or not a folder fails when it is listed.
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::NotFound | io::ErrorKind::NotADirectory
            ) =>
        {
            return Ok(None);
        }
        Err(source) => return Err(IgnoreError::Unreadable { path, source }),
    };
    parse(&text)
        .map(Some)
        .map_err(|(line, problem)| IgnoreError::Invalid {
            path,
            line,
            problem,
        })
}

/// The patterns of an ignore file's text, or the first invalid line and
/// what is wrong with it.
pub fn parse(text: &str) -> Result<Vec<Pattern>, (usize, Problem)> {
    let mut patterns = Vec::new();
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        let text = line.trim_end_matches(' ');
        if text.is_empty() || text.starts_with('#') {
            continue;
        }
        let pattern = Pattern::parse(text, number).map_err(|problem| (number, problem))?;
        patterns.push(pattern);
    }
    Ok(patterns)
}

impl Pattern {
    fn parse(text: &str, line: usize) -> Result<Pattern, Problem> {
        if text.starts_with('!') {
            return Err(Problem::Negation);
        }
        if text.contains('\\') {
            return Err(Problem::Backslash);
        }
        let (body, target) = match text.strip_suffix('/') {
            Some(body) => (body, Target::Folders),
            None => (text, Target::FilesAndFolders),
        };
        let shape = match body.strip_prefix('/') {
            Some(path) => Shape::FromRoot(path.split('/').map(str::to_owned).collect()),
            None if body.contains('/') => {
                Shape::FromRoot(body.split('/').map(str::to_owned).collect())
            }
            None => Shape::Name(body.to_owned()),
        };
        match &shape {
            Shape::Name(name) if name.is_empty() => return Err(Problem::NoName),
            Shape::FromRoot(names) if names.iter().all(String::is_empty) => {
                return Err(Problem::NoName);
            }
            Shape::FromRoot(names) if names.iter().any(String::is_empty) => {
                return Err(Problem::EmptyName);
            }
            _ => {}
        }
        Ok(Pattern {
            text: text.to_owned(),
            line,
            shape,
            target,
        })
    }

    /// Whether this pattern leaves out the file or folder at `path`,
    /// relative to the library root with `/` between names, as asset paths
    /// are written.
    pub fn matches(&self, path: &str, entry: Entry) -> bool {
        if self.target == Target::Folders && entry == Entry::File {
            return false;
        }
        match &self.shape {
            Shape::Name(pattern) => path
                .rsplit('/')
                .next()
                .is_some_and(|name| name_matches(pattern, name)),
            Shape::FromRoot(patterns) => {
                let names: Vec<&str> = path.split('/').collect();
                names_match(patterns, &names)
            }
        }
    }
}

/// Whether `names` match `patterns` one for one. A pattern `**` matches
/// any number of names: none when more patterns follow, as `a/**/b`
/// matches `a/b`; at least one at the end, as `a/**` matches what is inside
/// `a`, not `a` itself.
fn names_match(patterns: &[String], names: &[&str]) -> bool {
    match patterns.split_first() {
        None => names.is_empty(),
        Some((first, rest)) if first == "**" => {
            if rest.is_empty() {
                return !names.is_empty();
            }
            (0..=names.len()).any(|skipped| names_match(rest, &names[skipped..]))
        }
        Some((first, rest)) => names
            .split_first()
            .is_some_and(|(name, others)| name_matches(first, name) && names_match(rest, others)),
    }
}

/// Whether one name matches a pattern for one name: `*` matches any run of
/// characters (so does `**` inside a name, as in Git), `?` exactly one, and
/// every other character itself.
fn name_matches(pattern: &str, name: &str) -> bool {
    let mut pattern_chars = pattern.chars();
    match pattern_chars.next() {
        None => name.is_empty(),
        Some('*') => {
            let rest = pattern_chars.as_str();
            name.char_indices()
                .map(|(at, _)| at)
                .chain([name.len()])
                .any(|at| name_matches(rest, &name[at..]))
        }
        Some(wanted) => {
            let mut name_chars = name.chars();
            name_chars.next().is_some_and(|first| {
                (wanted == '?' || wanted == first)
                    && name_matches(pattern_chars.as_str(), name_chars.as_str())
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pattern(text: &str) -> Pattern {
        let patterns = parse(text).unwrap();
        assert_eq!(patterns.len(), 1, "{text:?} is one pattern");
        patterns.into_iter().next().unwrap()
    }

    fn leaves_out_file(text: &str, path: &str) -> bool {
        pattern(text).matches(path, Entry::File)
    }

    fn leaves_out_folder(text: &str, path: &str) -> bool {
        pattern(text).matches(path, Entry::Folder)
    }

    #[test]
    fn a_pattern_with_a_leading_slash_is_a_path_from_the_root() {
        assert!(leaves_out_folder("/Mixes", "Mixes"));
        assert!(!leaves_out_folder("/Mixes", "Old/Mixes"));
        assert!(leaves_out_file("/Mixes", "Mixes"));
    }

    #[test]
    fn a_pattern_with_a_slash_in_its_middle_is_a_path_from_the_root() {
        assert!(leaves_out_folder("Album/CD2", "Album/CD2"));
        assert!(!leaves_out_folder("Album/CD2", "Label/Album/CD2"));
        assert!(!leaves_out_folder("Album/CD2", "Album"));
    }

    #[test]
    fn a_pattern_without_a_slash_matches_a_name_at_any_depth() {
        assert!(leaves_out_folder("CD2", "CD2"));
        assert!(leaves_out_folder("CD2", "Album/CD2"));
        assert!(leaves_out_file("intro.mp3", "Label/Album/intro.mp3"));
        assert!(!leaves_out_folder("CD2", "Album/CD2 bonus"));
        assert!(!leaves_out_folder("CD2", "CD2/Album"), "only the last name");
    }

    #[test]
    fn a_trailing_slash_matches_folders_only() {
        assert!(leaves_out_folder("/Album/", "Album"));
        assert!(!leaves_out_file("/Album/", "Album"));
        assert!(leaves_out_folder("CD2/", "Album/CD2"));
        assert!(!leaves_out_file("CD2/", "Album/CD2"));
        assert!(leaves_out_file("/Album", "Album"), "without it, files too");
    }

    #[test]
    fn a_star_matches_within_one_name() {
        assert!(leaves_out_file("*.mp3", "Album/01 Intro.mp3"));
        assert!(leaves_out_folder(
            "/Dangerous Drums*/",
            "Dangerous Drums Volume 2"
        ));
        assert!(leaves_out_file("/Album/*", "Album/01.mp3"));
        assert!(!leaves_out_file("/Album/*", "Album/CD1/01.mp3"));
        assert!(!leaves_out_file("/A*b", "A/b"), "a star never crosses `/`");
        assert!(leaves_out_file("/*", "anything.mp3"));
        assert!(leaves_out_file("/a*b*c", "abc"), "a star can match nothing");
    }

    #[test]
    fn a_double_star_name_matches_any_number_of_folders() {
        assert!(leaves_out_folder("**/CD2/", "CD2"));
        assert!(leaves_out_folder("**/CD2/", "Label/Album/CD2"));
        assert!(leaves_out_file("/Label/**/intro.mp3", "Label/intro.mp3"));
        assert!(leaves_out_file(
            "/Label/**/intro.mp3",
            "Label/A/B/intro.mp3"
        ));
        assert!(!leaves_out_file("/Label/**/intro.mp3", "Other/A/intro.mp3"));
        assert!(leaves_out_file("/Label/**", "Label/A/01.mp3"));
        assert!(
            !leaves_out_folder("/Label/**", "Label"),
            "what is inside, not the folder"
        );
        assert!(
            leaves_out_file("/La**el", "Label"),
            "inside a name it is `*`"
        );
        assert!(!leaves_out_file("/La**el", "La/el"));
    }

    #[test]
    fn a_question_mark_matches_one_character() {
        assert!(leaves_out_folder("/CD?/", "CD1"));
        assert!(!leaves_out_folder("/CD?/", "CD10"));
        assert!(!leaves_out_folder("/CD?/", "CD"));
        assert!(
            leaves_out_file("?.mp3", "：.mp3"),
            "one character, not one byte"
        );
        assert!(!leaves_out_file("/a?b", "a/b"));
    }

    #[test]
    fn square_brackets_are_literal() {
        let creeps = "/2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/";
        assert!(leaves_out_folder(
            creeps,
            "2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2"
        ));
        assert!(!leaves_out_folder(
            creeps,
            "2001 - VRSCD003 - Ed Rush & Optical - The Creeps V/CD2"
        ));
        assert!(leaves_out_folder("[TECH012]", "Label/[TECH012]"));
        assert!(!leaves_out_folder("[TECH012]", "Label/T"));
        assert!(leaves_out_folder("*[a-z]*", "x[a-z]y"));
        assert!(!leaves_out_folder("*[a-z]*", "b"));
    }

    #[test]
    fn names_with_ampersands_escapes_and_full_width_colons_match_as_written() {
        let creeps = "/2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/cd2%2F10_-_Ed_Rush_%26_Optical_-_Bleep_Bleep.mp3";
        assert!(leaves_out_file(
            creeps,
            "2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/cd2%2F10_-_Ed_Rush_%26_Optical_-_Bleep_Bleep.mp3"
        ));
        assert!(leaves_out_file("cd2%2F*", "Album/CD2/cd2%2F01.mp3"));
        assert!(
            !leaves_out_file("/cd2%2F01.mp3", "cd2/01.mp3"),
            "`%2F` is three characters, not a folder"
        );
        assert!(leaves_out_folder("/Artist：Title/", "Artist：Title"));
        assert!(!leaves_out_folder("/Artist：Title/", "Artist:Title"));
    }

    #[test]
    fn matching_is_case_sensitive() {
        assert!(!leaves_out_folder("/cd2/", "CD2"));
        assert!(!leaves_out_file("*.MP3", "a.mp3"));
    }

    #[test]
    fn comments_blank_lines_and_trailing_spaces_are_skipped() {
        let text = "# Mixed CDs\n\n   \n/Album/CD2/  \n# another comment\n*.wav\n";
        let patterns = parse(text).unwrap();
        let written: Vec<(&str, usize)> = patterns
            .iter()
            .map(|pattern| (pattern.text.as_str(), pattern.line))
            .collect();
        assert_eq!(written, [("/Album/CD2/", 4), ("*.wav", 6)]);
        assert!(patterns[0].matches("Album/CD2", Entry::Folder));
    }

    #[test]
    fn the_owners_first_file_parses() {
        let text = "# Mixed CDs: each track already blends into the next one, so its\n\
                    # matches join unrelated recordings. Listed by the owner, 2026-10-09.\n\
                    /Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/\n\
                    /2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]/CD2/\n";
        let patterns = parse(text).unwrap();
        assert_eq!(patterns.len(), 2);
        assert!(patterns[0].matches(
            "Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)",
            Entry::Folder
        ));
        let creeps = "2001 - VRSCD003 - Ed Rush & Optical - The Creeps [Virus]";
        assert!(!patterns[1].matches(creeps, Entry::Folder));
        assert!(!patterns[1].matches(&format!("{creeps}/CD1"), Entry::Folder));
        assert!(patterns[1].matches(&format!("{creeps}/CD2"), Entry::Folder));
    }

    #[test]
    fn negation_and_backslashes_are_refused_with_their_line() {
        assert_eq!(
            parse("# keep CD1\n!/Album/CD1/"),
            Err((2, Problem::Negation))
        );
        assert_eq!(parse(r"/Album\ Two/"), Err((1, Problem::Backslash)));
        assert_eq!(parse(r"\!important.mp3"), Err((1, Problem::Backslash)));
        assert_eq!(parse("Album\\CD2"), Err((1, Problem::Backslash)));
    }

    #[test]
    fn patterns_without_a_name_are_refused() {
        assert_eq!(parse("/"), Err((1, Problem::NoName)));
        assert_eq!(parse("//"), Err((1, Problem::NoName)));
        assert_eq!(parse("/Album//CD2/"), Err((1, Problem::EmptyName)));
    }

    #[test]
    fn errors_name_the_file_and_line_and_say_what_to_write() {
        let root = std::env::temp_dir().join(format!("gunfinger-ignore-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join(IGNORE_FILE), "/Album/\n!/Album/CD1/\n").unwrap();

        let error = read(&root).unwrap_err();
        fs::remove_dir_all(&root).unwrap();

        let message = error.to_string();
        let expected_start = format!(
            "{}, line 2: `!` (negation)",
            root.join(IGNORE_FILE).display()
        );
        assert!(message.starts_with(&expected_start), "{message}");
        assert!(message.contains("such as `/Album/CD2/`"), "{message}");
    }

    #[test]
    fn no_file_means_no_patterns() {
        let root = std::env::temp_dir().join(format!("gunfinger-no-ignore-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let patterns = read(&root).unwrap();
        let missing_root = read(&root.join("missing")).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert_eq!(patterns, None);
        assert_eq!(missing_root, None);
    }
}

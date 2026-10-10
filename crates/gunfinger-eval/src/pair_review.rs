//! `pair-review`: the owner's verdicts on the listening pack, by ear.
//!
//! The pack (`scripts/analysis/listening_pack.py`, `work/listening/`) lists
//! its items in `README.md` in the order to listen; each item's folder holds
//! `sheet.txt` and clips. For every item without a verdict, the review shows
//! the sheet in short, plays the clips one at a time through `ffplay` (the
//! audio output of `gunfinger listen` and `review`), and on the owner's key
//! appends the verdict line the sheet shows to the verdicts file. It writes
//! nothing else, and nothing without a key.

use std::fs::{self, OpenOptions};
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::clusters::Verdicts;

const KEYS: &str =
    "s same, d different, Enter or n next clip, p previous clip, r replay, k skip the item, q quit";

/// One item of the pack.
#[derive(Debug, PartialEq)]
pub struct Item {
    /// Its number in the README's listening order.
    pub number: usize,
    pub folder: PathBuf,
    /// The sheet's lines shown before listening: its kind, both full paths
    /// with their lengths, the clusters' alignment and where the files stop
    /// lining up.
    pub summary: Vec<String>,
    /// The two files as the verdicts file names them.
    pub pair: (String, String),
    /// Other pairs of files between the same two clusters: a verdict on any
    /// of them judges the item too.
    pub related: Vec<(String, String)>,
    /// In the order to play: by number, each file in turn before the two in
    /// stereo.
    pub clips: Vec<PathBuf>,
    /// A join that `clusters --cut-sparsest` cut.
    pub cut: bool,
}

/// What the review did.
#[derive(Debug, Default, PartialEq)]
pub struct Outcome {
    pub written: usize,
    pub skipped: usize,
    pub judged_before: usize,
    /// Cut joins still without a verdict when the review ended.
    pub cut_left: usize,
}

/// The pack's items in the README's order.
pub fn items(pack: &Path) -> Result<Vec<Item>, String> {
    let readme = pack.join("README.md");
    let text = fs::read_to_string(&readme)
        .map_err(|error| format!("cannot read {}: {error}", readme.display()))?;
    let mut items = Vec::new();
    for line in text.lines() {
        let Some((number, folder)) = listed_item(line) else {
            continue;
        };
        items.push(item(number, &pack.join(folder))?);
    }
    if items.is_empty() {
        return Err(format!(
            "{} lists no items (lines such as \"1. `folder/` ...\")",
            readme.display()
        ));
    }
    Ok(items)
}

/// `12. \`folder/\` (...)` as its number and folder.
fn listed_item(line: &str) -> Option<(usize, &str)> {
    let (number, rest) = line.split_once(". `")?;
    let number = number.trim().parse().ok()?;
    let (folder, _) = rest.split_once("/`")?;
    Some((number, folder))
}

fn item(number: usize, folder: &Path) -> Result<Item, String> {
    let sheet_path = folder.join("sheet.txt");
    let sheet = fs::read_to_string(&sheet_path)
        .map_err(|error| format!("cannot read {}: {error}", sheet_path.display()))?;
    let lines: Vec<&str> = sheet.lines().collect();
    let pair = lines
        .iter()
        .rev()
        .find_map(|line| verdict_line(line))
        .ok_or_else(|| {
            format!(
                "{} has no verdict line (`same|different`, then the two paths)",
                sheet_path.display()
            )
        })?;
    let mut clips: Vec<PathBuf> = fs::read_dir(folder)
        .map_err(|error| format!("cannot read {}: {error}", folder.display()))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "mp3"))
        .collect();
    clips.sort_by_key(|path| clip_order(path));
    Ok(Item {
        number,
        folder: folder.to_path_buf(),
        summary: summary(&lines),
        pair,
        related: block(&lines, "Other pairs between the same two clusters:")
            .iter()
            .filter_map(|line| related_pair(line))
            .collect(),
        clips,
        cut: lines
            .first()
            .is_some_and(|kind| kind.starts_with("Cut join")),
    })
}

/// `same|different<TAB>a<TAB>b`, the sheet's template for a verdict.
fn verdict_line(line: &str) -> Option<(String, String)> {
    let mut fields = line.trim_start().split('\t');
    if fields.next() != Some("same|different") {
        return None;
    }
    match (fields.next(), fields.next(), fields.next()) {
        (Some(a), Some(b), None) => Some((a.to_owned(), b.to_owned())),
        _ => None,
    }
}

/// `  a ~ b (98.0%, 338 hits)` as its two files.
fn related_pair(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    let pair = line.rfind(" (").map_or(line, |end| &line[..end]);
    let (a, b) = pair.split_once(" ~ ")?;
    Some((a.to_owned(), b.to_owned()))
}

/// The sheet's first lines up to the alignments, and where the files stop
/// lining up.
fn summary(lines: &[&str]) -> Vec<String> {
    let mut summary: Vec<String> = lines
        .iter()
        .take_while(|line| !line.starts_with("Supported time"))
        .filter(|line| !line.trim().is_empty())
        .map(|line| (*line).to_owned())
        .collect();
    if let Some(start) = lines
        .iter()
        .position(|line| line.starts_with("Where the files stop lining up"))
    {
        summary.push(lines[start].to_owned());
        summary.extend(
            block(lines, lines[start])
                .iter()
                .map(|line| (*line).to_owned()),
        );
    }
    summary
}

/// The indented lines after `heading`, up to the next blank line.
fn block<'a>(lines: &[&'a str], heading: &str) -> Vec<&'a str> {
    lines
        .iter()
        .skip_while(|line| **line != heading)
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .copied()
        .collect()
}

/// Clips are numbered `N-...`; within a number, `a-then-b` sorts before
/// `stereo`.
fn clip_order(path: &Path) -> (usize, String) {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    let number = name
        .split('-')
        .next()
        .and_then(|number| number.parse().ok())
        .unwrap_or(usize::MAX);
    (number, name)
}

/// What a line typed during an item asks for.
#[derive(Debug, PartialEq)]
enum Key {
    Same,
    Different,
    Next,
    Previous,
    Replay,
    Skip,
    Quit,
    Unknown,
}

fn key(line: &str) -> Key {
    match line.trim() {
        "s" | "same" => Key::Same,
        "d" | "different" => Key::Different,
        "" | "n" | "next" => Key::Next,
        "p" | "previous" => Key::Previous,
        "r" | "replay" => Key::Replay,
        "k" | "skip" => Key::Skip,
        "q" | "quit" => Key::Quit,
        _ => Key::Unknown,
    }
}

/// Steps through the items without a verdict in `verdicts_file`, reading
/// keys from `input`, writing to `output` and playing clips with `play`.
/// Each verdict is appended as soon as it is given, so quitting keeps it.
pub fn review(
    items: &[Item],
    verdicts_file: &Path,
    input: &mut impl BufRead,
    output: &mut impl Write,
    mut play: impl FnMut(&Path) -> Result<(), String>,
) -> Result<Outcome, String> {
    let verdicts = Verdicts::load(verdicts_file)?;
    let judged = |item: &Item| {
        std::iter::once(&item.pair)
            .chain(&item.related)
            .any(|(a, b)| verdicts.judges(a, b))
    };
    let mut outcome = Outcome::default();
    let open: Vec<&Item> = items
        .iter()
        .filter(|item| {
            let before = judged(item);
            outcome.judged_before += usize::from(before);
            !before
        })
        .collect();
    let say = |output: &mut dyn Write, text: &str| {
        writeln!(output, "{text}").map_err(|error| error.to_string())
    };
    say(
        output,
        &format!(
            "{} of {} items without a verdict; keys: {KEYS}",
            open.len(),
            items.len()
        ),
    )?;
    let mut quit = false;
    for (position, item) in open.iter().enumerate() {
        if quit {
            outcome.cut_left += usize::from(item.cut);
            continue;
        }
        say(output, "")?;
        say(
            output,
            &format!("[{}/{}] item {}", position + 1, open.len(), item.number),
        )?;
        for line in &item.summary {
            say(output, line)?;
        }
        match listen(item, input, output, &mut play)? {
            Decision::Verdict(same) => {
                append(verdicts_file, same, &item.pair)?;
                outcome.written += 1;
                let word = if same { "same" } else { "different" };
                say(
                    output,
                    &format!("wrote `{word}` to {}", verdicts_file.display()),
                )?;
            }
            Decision::Skip => {
                outcome.skipped += 1;
                outcome.cut_left += usize::from(item.cut);
            }
            Decision::Quit => {
                quit = true;
                outcome.cut_left += usize::from(item.cut);
            }
        }
    }
    Ok(outcome)
}

/// How an item's review ended.
enum Decision {
    /// `true` for the same recording.
    Verdict(bool),
    Skip,
    Quit,
}

/// Plays the item's clips until a verdict, a skip or a quit; the end of the
/// input quits.
fn listen(
    item: &Item,
    input: &mut impl BufRead,
    output: &mut impl Write,
    play: &mut impl FnMut(&Path) -> Result<(), String>,
) -> Result<Decision, String> {
    let written = |result: std::io::Result<()>| result.map_err(|error| error.to_string());
    let mut clip = 0;
    let mut play_now = true;
    loop {
        if play_now {
            match item.clips.get(clip) {
                Some(path) => {
                    let name = path
                        .file_name()
                        .map(|name| name.to_string_lossy().into_owned())
                        .unwrap_or_default();
                    written(writeln!(
                        output,
                        "playing clip {}/{}: {name}",
                        clip + 1,
                        item.clips.len()
                    ))?;
                    if let Err(error) = play(path) {
                        written(writeln!(output, "could not play {name}: {error}"))?;
                    }
                }
                None => written(writeln!(output, "this item has no clips"))?,
            }
        }
        written(write!(output, "> "))?;
        written(output.flush())?;
        let mut line = String::new();
        if input
            .read_line(&mut line)
            .map_err(|error| error.to_string())?
            == 0
        {
            return Ok(Decision::Quit);
        }
        play_now = true;
        match key(&line) {
            Key::Same => return Ok(Decision::Verdict(true)),
            Key::Different => return Ok(Decision::Verdict(false)),
            Key::Skip => return Ok(Decision::Skip),
            Key::Quit => return Ok(Decision::Quit),
            Key::Replay => {}
            Key::Next if clip + 1 < item.clips.len() => clip += 1,
            Key::Previous if clip > 0 => clip -= 1,
            Key::Next | Key::Previous => {
                written(writeln!(output, "no further clip that way"))?;
                play_now = false;
            }
            Key::Unknown => {
                written(writeln!(output, "keys: {KEYS}"))?;
                play_now = false;
            }
        }
    }
}

/// Appends `same|different<TAB>a<TAB>b`, starting a new line if the file
/// does not end with one.
fn append(verdicts_file: &Path, same: bool, (a, b): &(String, String)) -> Result<(), String> {
    let ends_with_newline = fs::read(verdicts_file)
        .map_or(true, |bytes| bytes.last().is_none_or(|&last| last == b'\n'));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(verdicts_file)
        .map_err(|error| format!("cannot open {}: {error}", verdicts_file.display()))?;
    let word = if same { "same" } else { "different" };
    let line = format!(
        "{}{word}\t{a}\t{b}\n",
        if ends_with_newline { "" } else { "\n" }
    );
    file.write_all(line.as_bytes())
        .map_err(|error| format!("cannot write {}: {error}", verdicts_file.display()))
}

/// Plays a clip to the end with `ffplay`, as `gunfinger listen` does.
pub fn ffplay(path: &Path) -> Result<(), String> {
    let status = Command::new("ffplay")
        .args(["-hide_banner", "-loglevel", "error", "-nodisp", "-autoexit"])
        .arg(path)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("cannot run ffplay ({error}); install FFmpeg with ffplay"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("ffplay could not play {}", path.display()))
    }
}

/// The command that applies the new verdicts to the last clusters run.
pub fn reuse_command(other_store: Option<&Path>, outcome: &Outcome) -> String {
    let store = other_store.map_or_else(
        || String::from("<the other library's peak store>"),
        |store| store.display().to_string(),
    );
    let cut = if outcome.cut_left > 0 {
        " --cut-sparsest"
    } else {
        ""
    };
    format!("gunfinger-eval --other-peaks-dir {store} clusters --from-peaks --reuse-pairs{cut}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-pair-review-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn sheet(kind: &str, a: &str, b: &str, related: &[(&str, &str)]) -> String {
        let mut text = format!(
            "{kind}\n\nA: /Volumes/atlas/Music/dnb/{a}\n   (6:06.1)\nB: /Volumes/atlas/Music/dnb/{b}\n   (6:04.7)\n\n\
Clusters' alignment: coverage 53.7% of the shorter file, 474 hits\n\
Supported time at the fitted speed: 0.66 of the shorter file\n\n\
Alignments at the fitted speed (B time = offset + speed x A time):\n  1. A 0:07.6-1:58.6 = B 0:30.2-2:24.9\n\n\
Where the files stop lining up (in A's time; B placed by alignment 1):\n  1:58.6: alignment 1 ends\n\n\
Clips (8 s each):\n  1-control-a-then-b.mp3\n\n"
        );
        if !related.is_empty() {
            text.push_str("Other pairs between the same two clusters:\n");
            for (x, y) in related {
                text.push_str(&format!("  {x} ~ {y} (52.8%, 466 hits)\n"));
            }
            text.push('\n');
        }
        text.push_str(&format!(
            "To record a verdict, a line in docs/pair-verdicts.txt (tabs between the fields):\n  same|different\tsecond-library/{a}\tsecond-library/{b}\n"
        ));
        text
    }

    /// A pack of three items: a cut join, a borderline pair judged through
    /// another pair of its clusters, and a control join.
    fn pack(dir: &Path) -> PathBuf {
        let pack = dir.join("listening");
        let items = [
            (
                "cut-join-sonar",
                sheet("Cut join", "x/Sonar (Revision).opus", "y/sonar.mp3", &[]),
            ),
            (
                "borderline-fire",
                sheet(
                    "Borderline",
                    "a/fire.mp3",
                    "b/fire (vip).mp3",
                    &[(
                        "second-library/c/fire.mp3",
                        "second-library/b/fire (vip).mp3",
                    )],
                ),
            ),
            (
                "control-join-nine",
                sheet("Control join", "n/nine.mp3", "m/the nine.mp3", &[]),
            ),
        ];
        let mut readme = String::from("# Listening pack\n\nIn the order to listen:\n\n");
        for (number, (folder, text)) in items.iter().enumerate() {
            let folder_path = pack.join(folder);
            fs::create_dir_all(&folder_path).unwrap();
            fs::write(folder_path.join("sheet.txt"), text).unwrap();
            for clip in [
                "2-at-1m58s6-stereo.mp3",
                "1-control-stereo.mp3",
                "2-at-1m58s6-a-then-b.mp3",
                "1-control-a-then-b.mp3",
            ] {
                fs::write(folder_path.join(clip), b"").unwrap();
            }
            readme.push_str(&format!(
                "{}. `{folder}/` (kind, 50% coverage, 400 hits): a ~ b\n",
                number + 1
            ));
        }
        fs::write(pack.join("README.md"), readme).unwrap();
        pack
    }

    fn names(paths: &[PathBuf]) -> Vec<String> {
        paths
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect()
    }

    #[test]
    fn items_come_in_the_readme_order_with_their_sheet_in_short() {
        let dir = scratch("items");

        let items = items(&pack(&dir)).unwrap();

        assert_eq!(
            items.iter().map(|item| item.number).collect::<Vec<_>>(),
            [1, 2, 3]
        );
        let cut = &items[0];
        assert!(cut.cut && !items[1].cut);
        assert_eq!(
            cut.pair,
            (
                String::from("second-library/x/Sonar (Revision).opus"),
                String::from("second-library/y/sonar.mp3")
            )
        );
        assert_eq!(
            cut.summary,
            [
                "Cut join",
                "A: /Volumes/atlas/Music/dnb/x/Sonar (Revision).opus",
                "   (6:06.1)",
                "B: /Volumes/atlas/Music/dnb/y/sonar.mp3",
                "   (6:04.7)",
                "Clusters' alignment: coverage 53.7% of the shorter file, 474 hits",
                "Where the files stop lining up (in A's time; B placed by alignment 1):",
                "  1:58.6: alignment 1 ends",
            ]
        );
        assert_eq!(
            names(&cut.clips),
            [
                "1-control-a-then-b.mp3",
                "1-control-stereo.mp3",
                "2-at-1m58s6-a-then-b.mp3",
                "2-at-1m58s6-stereo.mp3"
            ]
        );
        assert_eq!(
            items[1].related,
            [(
                String::from("second-library/c/fire.mp3"),
                String::from("second-library/b/fire (vip).mp3")
            )]
        );
    }

    #[test]
    fn keys_step_through_the_clips_and_append_the_verdict() {
        let dir = scratch("review");
        let items = items(&pack(&dir)).unwrap();
        let verdicts = dir.join("pair-verdicts.txt");
        let before = "# verdicts\nsame\tsecond-library/b/fire (vip).mp3\tsecond-library/c/fire.mp3";
        fs::write(&verdicts, before).unwrap();
        let mut played = Vec::new();
        let mut output = Vec::new();

        let outcome = review(
            &items,
            &verdicts,
            &mut "\np\nr\nx\nd\nk\n".as_bytes(),
            &mut output,
            |clip| {
                played.push(clip.to_path_buf());
                Ok(())
            },
        )
        .unwrap();

        assert_eq!(
            outcome,
            Outcome {
                written: 1,
                skipped: 1,
                judged_before: 1,
                cut_left: 0
            }
        );
        assert_eq!(
            fs::read_to_string(&verdicts).unwrap(),
            format!(
                "{before}\ndifferent\tsecond-library/x/Sonar (Revision).opus\tsecond-library/y/sonar.mp3\n"
            )
        );
        assert_eq!(
            names(&played),
            [
                "1-control-a-then-b.mp3",
                "1-control-stereo.mp3",
                "1-control-a-then-b.mp3",
                "1-control-a-then-b.mp3",
                "1-control-a-then-b.mp3"
            ]
        );
        let shown = String::from_utf8(output).unwrap();
        assert!(
            shown.starts_with("2 of 3 items without a verdict"),
            "{shown}"
        );
        assert!(shown.contains("[2/2] item 3\nControl join\n"), "{shown}");
        assert!(!shown.contains("item 2\n"), "{shown}");
    }

    #[test]
    fn quitting_keeps_the_verdicts_given_and_asks_for_the_cut() {
        let dir = scratch("quit");
        let items = items(&pack(&dir)).unwrap();
        let verdicts = dir.join("pair-verdicts.txt");

        let outcome = review(
            &items,
            &verdicts,
            &mut "k\ns\nq\n".as_bytes(),
            &mut Vec::new(),
            |_| Err(String::from("no audio output")),
        )
        .unwrap();

        assert_eq!(outcome.written, 1);
        assert_eq!(outcome.cut_left, 1);
        assert_eq!(
            fs::read_to_string(&verdicts).unwrap(),
            "same\tsecond-library/a/fire.mp3\tsecond-library/b/fire (vip).mp3\n"
        );
        assert_eq!(
            reuse_command(Some(Path::new("/peaks/nas")), &outcome),
            "gunfinger-eval --other-peaks-dir /peaks/nas clusters --from-peaks --reuse-pairs --cut-sparsest"
        );
        let again = review(
            &items,
            &verdicts,
            &mut "".as_bytes(),
            &mut Vec::new(),
            |_| Ok(()),
        )
        .unwrap();
        assert_eq!(again.judged_before, 1);
        assert_eq!(again.written, 0);
    }
}

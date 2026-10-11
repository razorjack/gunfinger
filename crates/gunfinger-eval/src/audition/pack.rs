//! `questions pack`: the listening pack (`work/listening/`, written by
//! `scripts/analysis/listening_pack.py`) as pair questions, in its
//! README's order.
//!
//! Each item becomes lane A (its first file, the timeline's reference),
//! lane B placed by the sheet's alignments at the fitted speed, markers
//! where the files stop lining up and at the clips' moments, and the
//! alignments shaded. The sheet rounds an alignment to 0.1 s; when a
//! `pair` report holds the same pair and its alignments round to the
//! sheet's, their exact numbers are used. A plan (TOML) adds what the
//! sheets do not hold: the question texts, and extra lanes placed on A's
//! timeline by a `pair` report of the extra file against A.
//!
//! An answer of `same` or `different` goes to the pair verdicts file with
//! the sheet's two files, as `pair-review` writes it; the sheet's other
//! pairs between the same two clusters judge the question too.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::timecode::parse_timecode;
use serde::Deserialize;

use crate::audition::names::Libraries;
use crate::audition::placement::{self, Alignment};
use crate::audition::schema::{
    Choice, Effect, Lane, Marker, MarkerKind, Piece, Question, QuestionFile, QuestionId,
    SCHEMA_VERSION, Span, Timeline,
};
use crate::pair::PairReport;
use crate::pair_review;

/// What the sheets do not hold, by item number.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    /// The text of every item the plan does not name.
    #[serde(default)]
    pub default_text: Option<String>,
    #[serde(default)]
    pub item: Vec<PlannedItem>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlannedItem {
    pub number: usize,
    pub text: String,
    #[serde(default)]
    pub extra_lanes: Vec<ExtraLane>,
}

/// Another file to hear beside A and B, by its harness name.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtraLane {
    pub name: String,
    pub file: String,
}

impl Plan {
    pub fn load(path: &Path) -> Result<Plan, String> {
        let text = fs::read_to_string(path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        toml::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    fn item(&self, number: usize) -> Option<&PlannedItem> {
        self.item.iter().find(|item| item.number == number)
    }
}

/// The pair reports the exact alignments come from, with their files'
/// names for messages.
pub struct Reports {
    pub reports: Vec<(PathBuf, PairReport)>,
}

impl Reports {
    pub fn load(paths: &[PathBuf]) -> Result<Reports, String> {
        let reports = paths
            .iter()
            .map(|path| {
                let text = fs::read_to_string(path)
                    .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
                let reports: Vec<PairReport> = serde_json::from_str(&text)
                    .map_err(|error| format!("{}: {error}", path.display()))?;
                Ok(reports.into_iter().map(|report| (path.clone(), report)))
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(Reports {
            reports: reports.into_iter().flatten().collect(),
        })
    }

    /// The report with `query` searched against `found`.
    fn of(&self, query: &str, found: &str) -> Option<&(PathBuf, PairReport)> {
        self.reports
            .iter()
            .find(|(_, report)| report.query == query && report.found == found)
    }
}

/// The search whose alignments the sheets list.
const FITTED: &str = "peaks, at the fitted speed";

/// The pack's items as one question file.
pub fn questions(
    pack: &Path,
    plan: &Plan,
    reports: &Reports,
    libraries: &Libraries,
) -> Result<QuestionFile, String> {
    let questions = pair_review::items(pack)?
        .iter()
        .map(|item| {
            question(item, plan, reports, libraries).map_err(|error| {
                format!("item {} ({}): {error}", item.number, item.folder.display())
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(QuestionFile {
        schema_version: SCHEMA_VERSION,
        title: String::from("Listening pack"),
        made_by: format!("gunfinger-eval questions pack --pack {}", pack.display()),
        questions,
    })
}

fn question(
    item: &pair_review::Item,
    plan: &Plan,
    reports: &Reports,
    libraries: &Libraries,
) -> Result<Question, String> {
    let sheet_path = item.folder.join("sheet.txt");
    let text = fs::read_to_string(&sheet_path)
        .map_err(|error| format!("cannot read {}: {error}", sheet_path.display()))?;
    let sheet = Sheet::parse(&text)?;
    let (a_name, b_name) = (&item.pair.0, &item.pair.1);
    let report = reports.of(a_name, b_name);
    let a_seconds = report.map_or(sheet.a.1, |(_, report)| report.query_seconds);
    let b_seconds = report.map_or(sheet.b.1, |(_, report)| report.found_seconds);
    let (alignments, source) = match report
        .and_then(|(path, report)| exact(&sheet.alignments, report).map(|exact| (path, exact)))
    {
        Some((path, exact)) => (exact, format!("Alignments: exact, from {}", path.display())),
        None => (
            sheet.alignments.clone(),
            String::from("Alignments: from the sheet, rounded to 0.1 s"),
        ),
    };
    let timeline = (0.0, a_seconds);

    let mut lanes = vec![
        Lane {
            name: String::from("A"),
            pieces: vec![Piece {
                file: sheet.a.0.clone(),
                start: 0.0,
                end: a_seconds,
                offset: 0.0,
                speed: 1.0,
                label: None,
            }],
        },
        Lane {
            name: String::from("B"),
            pieces: b_pieces(&sheet.b.0, b_seconds, timeline, &alignments),
        },
    ];
    let mut spans = spans_of(&alignments, 1);
    let mut details = vec![sheet.kind.clone()];
    details.extend(sheet.details.iter().cloned());
    details.push(source);
    if alignments.is_empty() {
        details.push(String::from(
            "B found nothing of A at the fitted speed: lane B plays from its start, not aligned",
        ));
    }

    let planned = plan.item(item.number);
    for extra in planned.map_or(&[][..], |planned| &planned.extra_lanes) {
        let (alignments, seconds, from) = extra_alignments(a_name, &extra.file, reports)?;
        let lane = lanes.len();
        lanes.push(Lane {
            name: extra.name.clone(),
            pieces: placement::tile(&libraries.path(&extra.file), seconds, timeline, &alignments),
        });
        spans.extend(spans_of(&alignments, lane));
        details.push(format!(
            "Lane {} ({}): {}, placed on A by {} alignments of {}",
            lane + 1,
            extra.name,
            extra.file,
            alignments.len(),
            from.display()
        ));
    }
    if let Some(lane) = lanes.iter().find(|lane| lane.pieces.is_empty()) {
        return Err(format!("lane {} has no audio on A's timeline", lane.name));
    }

    let files = vec![a_name.clone(), b_name.clone()];
    Ok(Question {
        id: QuestionId::derive("pair", &[a_name, b_name]),
        kind: String::from("pair"),
        title: format!(
            "{}. {}: {} ~ {}",
            item.number,
            sheet.kind,
            stem(&sheet.a.0),
            stem(&sheet.b.0)
        ),
        text: planned.map_or_else(
            || {
                plan.default_text
                    .clone()
                    .unwrap_or_else(|| String::from("Are A and B the same recording?"))
            },
            |planned| planned.text.clone(),
        ),
        details,
        files,
        timeline: Timeline {
            start: timeline.0,
            end: timeline.1,
            reference: Some(0),
        },
        lanes,
        markers: markers(&sheet, a_seconds),
        spans,
        choices: vec![
            choice('s', "same recording", "same"),
            choice('d', "different recordings", "different"),
            choice('u', "unsure", "unsure"),
        ],
        effect: Effect::PairVerdict {
            related: item
                .related
                .iter()
                .map(|(a, b)| [a.clone(), b.clone()])
                .collect(),
        },
    })
}

/// Lane B by the alignments, or from its start when there are none.
fn b_pieces(
    file: &Path,
    seconds: f64,
    timeline: (f64, f64),
    alignments: &[Alignment],
) -> Vec<Piece> {
    if alignments.is_empty() {
        let unaligned = Piece {
            file: file.to_path_buf(),
            start: timeline.0,
            end: timeline.1,
            offset: 0.0,
            speed: 1.0,
            label: Some(String::from("not aligned")),
        };
        return placement::clip_to_file(unaligned, seconds)
            .into_iter()
            .collect();
    }
    placement::tile(file, seconds, timeline, alignments)
}

/// The report's alignments, if each rounds to the sheet's.
fn exact(sheet: &[Alignment], report: &PairReport) -> Option<Vec<Alignment>> {
    let fitted = report
        .searches
        .iter()
        .find(|search| search.name == FITTED)?;
    if fitted.alignments.len() != sheet.len() {
        return None;
    }
    let near = |exact: f64, rounded: f64, step: f64| (exact - rounded).abs() <= step / 2.0 + 1e-9;
    sheet
        .iter()
        .zip(&fitted.alignments)
        .map(|(rounded, exact)| {
            let agrees = near(exact.query_start_seconds, rounded.start, 0.1)
                && near(exact.query_end_seconds, rounded.end, 0.1)
                && near(exact.offset_seconds, rounded.offset, 0.1)
                && near(exact.speed, rounded.speed, 0.0001);
            agrees.then(|| Alignment {
                start: exact.query_start_seconds,
                end: exact.query_end_seconds,
                offset: exact.offset_seconds,
                speed: exact.speed,
                label: rounded.label.clone(),
            })
        })
        .collect()
}

/// An extra file placed on A's timeline by a `pair` report of the two, in
/// either direction: its alignments, its length and the report's file.
fn extra_alignments<'a>(
    a: &str,
    extra: &str,
    reports: &'a Reports,
) -> Result<(Vec<Alignment>, f64, &'a Path), String> {
    let fitted = |report: &PairReport| -> Vec<(Alignment, u32)> {
        report
            .searches
            .iter()
            .find(|search| search.name == FITTED)
            .map(|search| {
                search
                    .alignments
                    .iter()
                    .enumerate()
                    .map(|(index, alignment)| {
                        let placed = Alignment {
                            start: alignment.query_start_seconds,
                            end: alignment.query_end_seconds,
                            offset: alignment.offset_seconds,
                            speed: alignment.speed,
                            label: format!("alignment {}: {} hits", index + 1, alignment.hits),
                        };
                        (placed, alignment.hits)
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    if let Some((path, report)) = reports.of(a, extra) {
        return Ok((strong(fitted(report)), report.found_seconds, path));
    }
    if let Some((path, report)) = reports.of(extra, a) {
        let on_a = fitted(report)
            .into_iter()
            .map(|(alignment, hits)| (alignment.inverted(), hits))
            .collect();
        return Ok((strong(on_a), report.query_seconds, path));
    }
    Err(format!(
        "no pair report holds {extra} and {a}; write the two names, tab-separated, to a file, run `gunfinger-eval --other-peaks-dir <store> pair <file>` and pass its report with --pair-report"
    ))
}

/// An extra lane follows only the alignments with at least a tenth of the
/// strongest one's hits: a pair report of two versions also lists chance
/// alignments of a dozen hits, which would make the lane jump minutes
/// away for a second or two. (The sheets' alignments of B are taken as the
/// sheet lists them.)
fn strong(alignments: Vec<(Alignment, u32)>) -> Vec<Alignment> {
    let strongest = alignments.iter().map(|(_, hits)| *hits).max().unwrap_or(0);
    alignments
        .into_iter()
        .filter(|(_, hits)| *hits * 10 >= strongest)
        .map(|(alignment, _)| alignment)
        .collect()
}

fn spans_of(alignments: &[Alignment], lane: usize) -> Vec<Span> {
    alignments
        .iter()
        .map(|alignment| Span {
            start: alignment.start,
            end: alignment.end,
            lanes: vec![0, lane],
            label: format!(
                "{}: offset {:+.2} s, speed {:.2}%",
                alignment.label,
                alignment.offset,
                alignment.speed * 100.0
            ),
        })
        .filter(|span| span.start < span.end)
        .collect()
}

/// Where the files stop lining up, and the clips' moments not already
/// marked so.
fn markers(sheet: &Sheet, a_seconds: f64) -> Vec<Marker> {
    const SAME_MOMENT: f64 = 0.5;
    let inside = |at: f64| (0.0..=a_seconds).contains(&at);
    let mut markers: Vec<Marker> = sheet
        .stops
        .iter()
        .filter(|(at, _)| inside(*at))
        .map(|(at, why)| Marker {
            at: *at,
            label: why.clone(),
            kind: MarkerKind::Edge,
        })
        .collect();
    for (at, why) in &sheet.clips {
        let marked = markers
            .iter()
            .any(|marker| (marker.at - at).abs() < SAME_MOMENT);
        if inside(*at) && !marked {
            markers.push(Marker {
                at: *at,
                label: format!("clip: {why}"),
                kind: MarkerKind::Listen,
            });
        }
    }
    markers.sort_by(|a, b| a.at.total_cmp(&b.at));
    markers
}

fn choice(key: char, label: &str, value: &str) -> Choice {
    Choice {
        key,
        label: label.to_owned(),
        value: value.to_owned(),
    }
}

fn stem(path: &Path) -> String {
    path.file_stem().map_or_else(
        || path.display().to_string(),
        |stem| stem.to_string_lossy().into_owned(),
    )
}

/// What a pack sheet says, as `listening_pack.py` writes it.
#[derive(Debug, PartialEq)]
struct Sheet {
    /// `Cut join`, `Borderline`, `Control join`, `Sparse join`.
    kind: String,
    /// Each file's absolute path and length.
    a: (PathBuf, f64),
    b: (PathBuf, f64),
    /// The clusters' alignment and the supported time, as written.
    details: Vec<String>,
    /// At the fitted speed, strongest first, in A's time.
    alignments: Vec<Alignment>,
    /// Where the files stop lining up, in A's time.
    stops: Vec<(f64, String)>,
    /// Each clip's middle in A's time, and why it was cut there.
    clips: Vec<(f64, String)>,
}

impl Sheet {
    fn parse(text: &str) -> Result<Sheet, String> {
        let lines: Vec<&str> = text.lines().collect();
        let kind = lines
            .first()
            .map(|line| line.trim().to_owned())
            .filter(|kind| !kind.is_empty())
            .ok_or("the sheet is empty")?;
        let file = |label: &str| -> Result<(PathBuf, f64), String> {
            let at = lines
                .iter()
                .position(|line| line.starts_with(label))
                .ok_or_else(|| format!("no `{label}` line"))?;
            let length = lines
                .get(at + 1)
                .map(|line| line.trim().trim_start_matches('(').trim_end_matches(')'))
                .ok_or_else(|| format!("no length after `{label}`"))?;
            Ok((PathBuf::from(&lines[at][label.len()..]), seconds(length)?))
        };
        let details = lines
            .iter()
            .filter(|line| {
                line.starts_with("Clusters' alignment") || line.starts_with("Supported time")
            })
            .map(|line| (*line).to_owned())
            .collect();
        Ok(Sheet {
            kind,
            a: file("A: ")?,
            b: file("B: ")?,
            details,
            alignments: block(&lines, "Alignments at the fitted speed")
                .iter()
                .filter(|line| !line.trim_start().starts_with("none"))
                .map(|line| alignment(line))
                .collect::<Result<_, _>>()?,
            stops: block(&lines, "Where the files stop lining up")
                .iter()
                .filter_map(|line| stop(line))
                .collect(),
            clips: block(&lines, "Clips (")
                .iter()
                .filter_map(|line| clip(line))
                .collect(),
        })
    }
}

/// The indented lines after the line starting with `heading`, up to the
/// next blank line.
fn block<'a>(lines: &[&'a str], heading: &str) -> Vec<&'a str> {
    lines
        .iter()
        .skip_while(|line| !line.starts_with(heading))
        .skip(1)
        .take_while(|line| !line.trim().is_empty())
        .copied()
        .collect()
}

fn seconds(timecode: &str) -> Result<f64, String> {
    parse_timecode(timecode)
        .map(|time| time.as_secs_f64())
        .map_err(|error| error.to_string())
}

/// `  1. A 0:40.2-5:20.0 = B 0:18.1-4:57.5, speed 0.9984, offset -22.0 s, 3324 hits in 26 windows, supported 0.74`
fn alignment(line: &str) -> Result<Alignment, String> {
    let malformed = || format!("not an alignment: `{line}`");
    let (number, rest) = line.trim().split_once(". A ").ok_or_else(malformed)?;
    let (a_span, rest) = rest.split_once(" = B ").ok_or_else(malformed)?;
    let fields: Vec<&str> = rest.split(", ").collect();
    let field = |name: &str| {
        fields
            .iter()
            .find_map(|field| field.strip_prefix(name))
            .ok_or_else(malformed)
    };
    let (start, end) = a_span.split_once('-').ok_or_else(malformed)?;
    let speed = field("speed ")?.parse::<f64>().map_err(|_| malformed())?;
    let offset = field("offset ")?
        .trim_end_matches(" s")
        .parse::<f64>()
        .map_err(|_| malformed())?;
    let hits = fields
        .iter()
        .find(|field| field.contains(" hits"))
        .map_or("", |field| field.split(" in ").next().unwrap_or(""));
    Ok(Alignment {
        start: seconds(start)?,
        end: seconds(end)?,
        offset,
        speed,
        label: format!("alignment {number}: {hits}"),
    })
}

/// `  0:40.2: alignment 1 starts (B 0:18.1, ...)`; the other lines of the
/// block (`nowhere inside both files`, `(no clip at ...)`) are not places.
fn stop(line: &str) -> Option<(f64, String)> {
    let (at, why) = line.trim().split_once(": ")?;
    Some((seconds(at).ok()?, why.to_owned()))
}

/// `  2-at-0m40s2-a-then-b.mp3 and 2-at-0m40s2-stereo.mp3: A 0:36.2-0:44.2, B 0:14.1-0:22.1 (why)`
/// as the middle of its A span and why.
fn clip(line: &str) -> Option<(f64, String)> {
    let (_, rest) = line.split_once(".mp3: A ")?;
    let (a_span, rest) = rest.split_once(", B ")?;
    let (start, end) = a_span.split_once('-')?;
    let why = rest.split_once(" (")?.1.strip_suffix(')')?;
    Some((
        (seconds(start).ok()? + seconds(end).ok()?) / 2.0,
        why.to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-audition-pack-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    const EDIT_SHEET: &str = "Borderline

A: /nas/album/01-tune.mp3
   (7:00.5)
B: /nas/compilation/111-tune.mp3
   (5:52.2)

Clusters' alignment: coverage 79.3% of the shorter file, 3321 hits, A at 0.9984 times B's speed (-0.16%); A 0:40.2-5:20.0, B 0:18.1-4:57.5
Supported time at the fitted speed: 0.99 of the shorter file (seconds holding an aligned hit, summed over the alignments)

Alignments at the fitted speed (B time = offset + speed x A time):
  1. A 0:40.2-5:20.0 = B 0:18.1-4:57.5, speed 0.9984, offset -22.0 s, 3324 hits in 26 windows, supported 0.74
  2. A 5:34.6-6:47.5 = B 4:27.4-5:40.2, speed 0.9984, offset -66.6 s, 932 hits in 8 windows, supported 0.20

Where the files stop lining up (in A's time; B placed by alignment 1):
  0:40.2: alignment 1 starts (B 0:18.1, offset -22.0 s, 3324 hits)
  5:34.6: alignment 2 starts (B 4:27.4, offset -66.6 s, 932 hits)
  (no clip at 6:47.5: B has no audio there under alignment 1)

Clips (8 s each; B resampled to A's speed, pitch and tempo together):
  1-control-a-then-b.mp3 and 1-control-stereo.mp3: A 2:56.1-3:04.1, B 2:33.8-2:41.8 (inside alignment 1, as a control)
  2-at-0m40s2-a-then-b.mp3 and 2-at-0m40s2-stereo.mp3: A 0:36.2-0:44.2, B 0:14.1-0:22.1 (alignment 1 starts (B 0:18.1, offset -22.0 s, 3324 hits))
  -a-then-b: A, then the same moment of B; -stereo: A left, B right

Other pairs between the same two clusters:
  second-library/other/tune.mp3 ~ second-library/compilation/111-tune.mp3 (52.8%, 466 hits)

To record a verdict, a line in docs/pair-verdicts.txt (tabs between the fields):
  same|different\tsecond-library/album/01-tune.mp3\tsecond-library/compilation/111-tune.mp3
";

    const CONTROL_SHEET: &str = "Control join

A: /nas/ep/A - Tune.mp3
   (7:03.6)
B: /nas/ep-rip/a-tune.mp3
   (7:03.6)

Alignments at the fitted speed (B time = offset + speed x A time):
  1. A 0:00.0-7:01.8 = B 0:00.0-7:01.8, speed 1.0000, offset +0.0 s, 33014 hits in 43 windows, supported 1.00

Where the files stop lining up (in A's time; B placed by alignment 1):
  nowhere inside both files: alignment 1 runs from end to end

Clips (8 s each; B resampled to A's speed, pitch and tempo together):
  1-control-a-then-b.mp3 and 1-control-stereo.mp3: A 3:26.9-3:34.9, B 3:26.9-3:34.9 (inside alignment 1, as a control)

To record a verdict, a line in docs/pair-verdicts.txt (tabs between the fields):
  same|different\tsecond-library/ep/A - Tune.mp3\tsecond-library/ep-rip/a-tune.mp3
";

    /// A pack of two items: an edit and a control join.
    fn pack(dir: &Path) -> PathBuf {
        let pack = dir.join("listening");
        for (folder, sheet) in [
            ("borderline-edit", EDIT_SHEET),
            ("control-join-tune", CONTROL_SHEET),
        ] {
            fs::create_dir_all(pack.join(folder)).unwrap();
            fs::write(pack.join(folder).join("sheet.txt"), sheet).unwrap();
        }
        fs::write(
            pack.join("README.md"),
            "# Listening pack\n\n1. `borderline-edit/` (borderline): a ~ b\n2. `control-join-tune/` (control join): a ~ b\n",
        )
        .unwrap();
        pack
    }

    fn libraries() -> Libraries {
        Libraries {
            corpus: PathBuf::from("/repo/corpus/library"),
            second: PathBuf::from("/nas"),
        }
    }

    fn report(query: &str, found: &str, alignments: &[(f64, f64, f64, f64)]) -> PairReport {
        let text = serde_json::json!({
            "query": query,
            "found": found,
            "query_seconds": 420.545,
            "found_seconds": 352.183,
            "searches": [{
                "name": FITTED,
                "alignments": alignments.iter().map(|&(start, end, offset, speed)| serde_json::json!({
                    "query_start_seconds": start, "query_end_seconds": end,
                    "found_start_seconds": 0.0, "found_end_seconds": 0.0,
                    "speed": speed, "key_locked": false, "hits": 100, "windows": 3,
                    "coverage": 0.5, "offset_seconds": offset
                })).collect::<Vec<_>>()
            }]
        });
        serde_json::from_value(text).unwrap()
    }

    #[test]
    fn the_sheet_is_read_in_full() {
        let sheet = Sheet::parse(EDIT_SHEET).unwrap();

        assert_eq!(sheet.kind, "Borderline");
        assert_eq!(sheet.a, (PathBuf::from("/nas/album/01-tune.mp3"), 420.5));
        assert_eq!(sheet.b.1, 352.2);
        assert_eq!(sheet.alignments.len(), 2);
        assert_eq!(
            (
                sheet.alignments[1].start,
                sheet.alignments[1].offset,
                sheet.alignments[1].speed
            ),
            (334.6, -66.6, 0.9984)
        );
        assert_eq!(sheet.alignments[0].label, "alignment 1: 3324 hits");
        assert_eq!(
            sheet.stops.iter().map(|(at, _)| *at).collect::<Vec<_>>(),
            [40.2, 334.6]
        );
        assert_eq!(sheet.clips.len(), 2);
        assert!((sheet.clips[0].0 - 180.1).abs() < 1e-9);
        assert_eq!(sheet.clips[0].1, "inside alignment 1, as a control");
        assert_eq!(sheet.details.len(), 2);
    }

    #[test]
    fn items_become_pair_questions_in_the_readme_order() {
        let dir = scratch("items");
        let plan: Plan = toml::from_str(
            "default_text = \"Control join: optional.\"\n[[item]]\nnumber = 1\ntext = \"Is B an edit of A?\"\n",
        )
        .unwrap();

        let file = questions(
            &pack(&dir),
            &plan,
            &Reports {
                reports: Vec::new(),
            },
            &libraries(),
        )
        .unwrap();
        crate::audition::schema::check(&file).unwrap();

        let [edit, control] = &file.questions[..] else {
            panic!("two questions expected");
        };
        assert_eq!(edit.title, "1. Borderline: 01-tune ~ 111-tune");
        assert_eq!(edit.text, "Is B an edit of A?");
        assert_eq!(control.text, "Control join: optional.");
        assert_eq!(
            edit.files,
            [
                "second-library/album/01-tune.mp3",
                "second-library/compilation/111-tune.mp3"
            ]
        );
        assert_eq!(
            edit.id,
            QuestionId::derive(
                "pair",
                &[
                    "second-library/album/01-tune.mp3",
                    "second-library/compilation/111-tune.mp3"
                ]
            )
        );
        assert_eq!(
            edit.effect,
            Effect::PairVerdict {
                related: vec![[
                    String::from("second-library/other/tune.mp3"),
                    String::from("second-library/compilation/111-tune.mp3")
                ]]
            }
        );
        let b: Vec<(f64, f64, f64)> = edit.lanes[1]
            .pieces
            .iter()
            .map(|piece| (piece.start, piece.end, piece.offset))
            .collect();
        // Alignment 1 from where B starts (22.0 / 0.9984 s) until alignment
        // 2 starts, then alignment 2 to the end of B.
        assert_eq!(b.len(), 2);
        assert!((b[0].0 - 22.0 / 0.9984).abs() < 1e-9 && b[0].1 == 334.6 && b[0].2 == -22.0);
        assert!(b[1].0 == 334.6 && (b[1].1 - (352.2 + 66.6) / 0.9984).abs() < 1e-9);
        let markers: Vec<(f64, MarkerKind)> = edit
            .markers
            .iter()
            .map(|marker| (marker.at, marker.kind))
            .collect();
        // The clip at 0:40.2 sits on an edge already marked.
        assert_eq!(markers.len(), 3);
        assert_eq!((markers[0].0, markers[0].1), (40.2, MarkerKind::Edge));
        assert_eq!(markers[1].1, MarkerKind::Listen);
        assert!(
            edit.details
                .iter()
                .any(|line| line.contains("rounded to 0.1 s"))
        );
        assert_eq!(control.lanes[1].pieces.len(), 1);
        assert!(
            control
                .markers
                .iter()
                .all(|marker| marker.kind == MarkerKind::Listen)
        );
    }

    #[test]
    fn a_pair_report_that_agrees_with_the_sheet_gives_the_exact_alignments() {
        let dir = scratch("exact");
        let a = "second-library/album/01-tune.mp3";
        let b = "second-library/compilation/111-tune.mp3";
        let agreeing = report(
            a,
            b,
            &[
                (40.199, 319.99, -21.990_123, 0.998_423_5),
                (334.61, 407.5, -66.6, 0.9984),
            ],
        );
        let disagreeing = report(
            a,
            b,
            &[
                (40.4, 319.99, -21.99, 0.9984),
                (334.61, 407.5, -66.6, 0.9984),
            ],
        );

        let with = |report: PairReport| {
            let reports = Reports {
                reports: vec![(PathBuf::from("pairs.json"), report)],
            };
            questions(&pack(&dir), &Plan::default(), &reports, &libraries())
                .unwrap()
                .questions
                .remove(0)
        };
        let exact = with(agreeing);
        let rounded = with(disagreeing);

        assert_eq!(exact.lanes[1].pieces[0].offset, -21.990_123);
        assert_eq!(exact.timeline.end, 420.545);
        assert!(
            exact
                .details
                .iter()
                .any(|line| line == "Alignments: exact, from pairs.json")
        );
        assert_eq!(rounded.lanes[1].pieces[0].offset, -22.0);
    }

    #[test]
    fn an_extra_lane_is_placed_on_a_by_its_pair_report_in_either_direction() {
        let dir = scratch("extra");
        let a = "second-library/album/01-tune.mp3";
        let plan: Plan = toml::from_str(
            "[[item]]\nnumber = 1\ntext = \"Original or edit?\"\nextra_lanes = [{ name = \"original\", file = \"Original.m4a\" }]\n",
        )
        .unwrap();
        // The original was searched against A: A time = 0.7 + 0.995 x its time.
        let reports = Reports {
            reports: vec![(
                PathBuf::from("supported.json"),
                report("Original.m4a", a, &[(60.0, 160.0, 0.7, 0.995)]),
            )],
        };

        let question = questions(&pack(&dir), &plan, &reports, &libraries())
            .unwrap()
            .questions
            .remove(0);

        let lane = &question.lanes[2];
        assert_eq!(lane.name, "original");
        assert_eq!(
            lane.pieces[0].file,
            PathBuf::from("/repo/corpus/library/Original.m4a")
        );
        let at = 100.0;
        let original = lane.pieces[0].file_time(at);
        assert!((0.7 + 0.995 * original - at).abs() < 1e-9);
        assert!(question.spans.iter().any(|span| span.lanes == [0, 2]));

        let missing = questions(
            &pack(&dir),
            &plan,
            &Reports {
                reports: Vec::new(),
            },
            &libraries(),
        )
        .unwrap_err();
        assert!(
            missing.contains("no pair report holds Original.m4a"),
            "{missing}"
        );
    }
}

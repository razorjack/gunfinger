//! Saved generated-mix reports scored again with today's scorer, from
//! their saved detections, without searching: `mixes` reports (each mix's
//! `score`) and `grid` reports (each brief play's `level`). The rescored
//! report is written beside the saved one, which is never replaced.
//!
//! The scorer reads today's clusters and same-title judgments. A play
//! whose rescored placement (its strongest detection by hits, its
//! boundaries) or whose mix's wrong answers differ from the saved ones
//! has drifted for another reason than the level rule, and is counted
//! apart.

use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use crate::clusters::Clusters;
use crate::files::{read_json, write_json};
use crate::mixes::{self, MixFound, MixResult, MixesReport};
use crate::scoring::Level;
use crate::titles::SameTitles;

/// Appended to a rescored report's stem.
const RESCORED: &str = "-rescored";

/// Plays counted by level: confident, possible, weak, none.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Levels {
    pub confident: usize,
    pub possible: usize,
    pub weak: usize,
    pub none: usize,
}

impl Levels {
    fn add(&mut self, level: Level) {
        match level {
            Level::Confident => self.confident += 1,
            Level::Possible => self.possible += 1,
            Level::Weak => self.weak += 1,
            Level::None => self.none += 1,
        }
    }
}

/// What rescoring changed in one report.
#[derive(Debug, Default, Serialize)]
pub struct Rescored {
    /// A `grid` report, whose plays are its queries' brief plays.
    pub grid: bool,
    pub plays: usize,
    /// Indexed plays by level, as saved and as rescored.
    pub indexed_before: Levels,
    pub indexed_after: Levels,
    /// Held-out plays by level, as saved and as rescored.
    pub held_out_before: Levels,
    pub held_out_after: Levels,
    /// Plays whose level changed, as `(before, after, count)`.
    pub moves: Vec<(Level, Level, usize)>,
    /// Rescored plays whose level is above their strongest detection's:
    /// the rule's own effect, whatever else drifted.
    pub by_rule: usize,
    /// Plays whose rescored strongest detection or boundaries differ from
    /// the saved ones, or whose saved level is not their saved strongest
    /// detection's: scored under other clusters, judgments or thresholds.
    pub drifted_plays: usize,
    /// Mixes whose wrong answers or same-title credits differ.
    pub drifted_mixes: usize,
    /// A residual pilot the report holds, not rescored: its arms keep
    /// only each play's strongest detection.
    pub residual_not_rescored: bool,
}

impl Rescored {
    fn moved(&mut self, before: Level, after: Level) {
        if before == after {
            return;
        }
        match self
            .moves
            .iter_mut()
            .find(|(from, to, _)| *from == before && *to == after)
        {
            Some((_, _, count)) => *count += 1,
            None => self.moves.push((before, after, 1)),
        }
    }

    fn counted(&mut self, held_out: bool, before: Level, after: Level) {
        self.plays += 1;
        let (was, is) = if held_out {
            (&mut self.held_out_before, &mut self.held_out_after)
        } else {
            (&mut self.indexed_before, &mut self.indexed_after)
        };
        was.add(before);
        is.add(after);
        self.moved(before, after);
    }
}

/// `<stem>-rescored.json` beside `path`; an error for a report that is
/// already a rescored one.
pub fn rescored_path(path: &Path) -> Result<PathBuf, String> {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("{}: no file name", path.display()))?;
    if stem.ends_with(RESCORED) {
        return Err(format!("{} is already rescored", path.display()));
    }
    Ok(path.with_file_name(format!("{stem}{RESCORED}.json")))
}

/// Rescores the report at `path` and writes it beside it.
pub fn rescore_file(
    path: &Path,
    clusters: &Clusters,
    titles: &SameTitles,
) -> Result<(PathBuf, Rescored), String> {
    let out = rescored_path(path)?;
    let mut report: Value = read_json(path)?;
    let rescored = rescore(&mut report, clusters, titles)
        .map_err(|error| format!("{}: {error}", path.display()))?;
    write_json(&out, &report)?;
    Ok((out, rescored))
}

/// Rescores `report` in place: a `mixes` report's scores or a `grid`
/// report's levels.
pub fn rescore(
    report: &mut Value,
    clusters: &Clusters,
    titles: &SameTitles,
) -> Result<Rescored, String> {
    if report.get("mixes").is_some() {
        rescore_mixes(report, clusters, titles)
    } else if report.get("briefs").is_some() {
        rescore_grid(report)
    } else {
        Err(String::from("neither a mixes nor a grid report"))
    }
}

fn rescore_mixes(
    report: &mut Value,
    clusters: &Clusters,
    titles: &SameTitles,
) -> Result<Rescored, String> {
    let mut rescored = Rescored {
        residual_not_rescored: report.get("residual").is_some_and(|value| !value.is_null()),
        ..Rescored::default()
    };
    let mixes = report
        .get_mut("mixes")
        .and_then(Value::as_array_mut)
        .ok_or("`mixes` is not a list")?;
    for saved in mixes {
        let result: MixResult =
            serde_json::from_value(saved.clone()).map_err(|error| error.to_string())?;
        let score = mixes::score(&result.mix, &result.detections, clusters, titles);
        for (before, after) in result.score.plays.iter().zip(&score.plays) {
            rescored.counted(before.truth.held_out, before.level, after.level);
            let level_of = |best: &Option<MixFound>| {
                best.as_ref()
                    .map_or(Level::None, |found| found.confidence().into())
            };
            let placed_alike = before.best == after.best
                && before.start_late_seconds == after.start_late_seconds
                && before.end_early_seconds == after.end_early_seconds
                && before.position_error_seconds == after.position_error_seconds;
            if !placed_alike || before.level != level_of(&before.best) {
                rescored.drifted_plays += 1;
            }
            if after.level != level_of(&after.best) {
                rescored.by_rule += 1;
            }
        }
        if result.score.plays.len() != score.plays.len()
            || result.score.wrong_confident != score.wrong_confident
            || result.score.wrong_possible != score.wrong_possible
            || result.score.false_candidates != score.false_candidates
            || result.score.same_title != score.same_title
        {
            rescored.drifted_mixes += 1;
        }
        saved["score"] = serde_json::to_value(&score).map_err(|error| error.to_string())?;
    }
    Ok(rescored)
}

fn rescore_grid(report: &mut Value) -> Result<Rescored, String> {
    let mut rescored = Rescored {
        grid: true,
        ..Rescored::default()
    };
    let queries = report
        .get_mut("queries")
        .and_then(Value::as_array_mut)
        .ok_or("`queries` is not a list")?;
    for query in queries {
        let before: Level = serde_json::from_value(query["level"].clone())
            .map_err(|error| format!("a query's level: {error}"))?;
        let matching: Vec<MixFound> = serde_json::from_value(query["matching"].clone())
            .map_err(|error| format!("a query's matching detections: {error}"))?;
        let strongest = matching.iter().max_by_key(|found| found.hits);
        if before != strongest.map_or(Level::None, |found| found.confidence().into()) {
            rescored.drifted_plays += 1;
        }
        let after = matching
            .iter()
            .map(MixFound::confidence)
            .max()
            .map_or(Level::None, Level::from);
        if after != strongest.map_or(Level::None, |found| found.confidence().into()) {
            rescored.by_rule += 1;
        }
        rescored.counted(false, before, after);
        query["level"] = serde_json::to_value(after).map_err(|error| error.to_string())?;
    }
    Ok(rescored)
}

/// One line per report and, for a `mixes` report, its summary as
/// rescored.
pub fn print(path: &Path, out: &Path, rescored: &Rescored) {
    let levels = |levels: &Levels| {
        format!(
            "{} confident, {} possible, {} weak, {} none",
            levels.confident, levels.possible, levels.weak, levels.none
        )
    };
    println!("{} -> {}", path.display(), out.display());
    println!(
        "  indexed plays before: {}; after: {}",
        levels(&rescored.indexed_before),
        levels(&rescored.indexed_after)
    );
    if rescored.held_out_before != Levels::default() {
        println!(
            "  held-out plays before: {}; after: {}",
            levels(&rescored.held_out_before),
            levels(&rescored.held_out_after)
        );
    }
    let moves: Vec<String> = rescored
        .moves
        .iter()
        .map(|(from, to, count)| format!("{from} -> {to}: {count}"))
        .collect();
    println!(
        "  of {} plays, levels changed: {}; above the strongest detection's: {}; drifted otherwise: {} plays, {} mixes",
        rescored.plays,
        if moves.is_empty() {
            String::from("none")
        } else {
            moves.join(", ")
        },
        rescored.by_rule,
        rescored.drifted_plays,
        rescored.drifted_mixes
    );
    if rescored.residual_not_rescored {
        println!("  the residual pilot is not rescored: its arms keep no detections");
    }
}

/// The rescored `mixes` report's summary, as `mixes` prints it.
pub fn print_mixes_summary(out: &Path) -> Result<(), String> {
    let report: MixesReport = read_json(out)?;
    mixes::print_summary(&report, report.kind);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mixes::{PlannedMix, PlannedPlay};

    fn play(asset: &str, held_out: bool, start_seconds: f64, end_seconds: f64) -> PlannedPlay {
        PlannedPlay {
            asset: asset.to_owned(),
            held_out,
            source_start_seconds: 0.0,
            speed: 1.0,
            start_seconds,
            end_seconds,
            fade_in_seconds: 0.0,
            fade_out_seconds: 0.0,
            bass_cut_until: None,
            bass_cut_from: None,
            ride: None,
            gain_db: 0.0,
            scenario: None,
            side: None,
            drop_seconds: None,
        }
    }

    fn found(asset: &str, start_seconds: f64, windows: u32, hits: u32) -> MixFound {
        MixFound {
            asset: asset.to_owned(),
            start_seconds,
            end_seconds: start_seconds + 10.0 * f64::from(windows),
            track_start_seconds: start_seconds,
            speed: 1.0,
            windows,
            hits,
            fitted: true,
            verified: None,
        }
    }

    fn no_clusters() -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates: Vec::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        }
    }

    /// A report as the scorer before session 77 wrote it: each play's
    /// level is its strongest detection's by hits.
    fn saved_report() -> Value {
        let mix = PlannedMix {
            name: String::from("mix"),
            seconds: 200.0,
            plays: vec![
                play("a.mp3", false, 0.0, 100.0),
                play("b.mp3", false, 90.0, 200.0),
            ],
        };
        let detections = vec![
            found("a.mp3", 5.0, 2, 3000),
            found("a.mp3", 30.0, 5, 1000),
            found("b.mp3", 100.0, 6, 900),
        ];
        let mut score = mixes::score(&mix, &detections, &no_clusters(), &SameTitles::default());
        for play in &mut score.plays {
            play.level = play
                .best
                .as_ref()
                .map_or(Level::None, |found| found.confidence().into());
            play.at_level = None;
        }
        serde_json::json!({
            "seed": 1,
            "ladder": "both",
            "indexed_assets": 2,
            "an_old_field": "kept",
            "mixes": [{"mix": mix, "score": score, "detections": detections}],
        })
    }

    #[test]
    fn a_saved_mixes_report_takes_the_highest_level_and_keeps_its_other_fields() {
        let mut report = saved_report();
        let rescored = rescore(&mut report, &no_clusters(), &SameTitles::default()).unwrap();

        assert_eq!(rescored.plays, 2);
        assert_eq!(rescored.indexed_before.confident, 1);
        assert_eq!(rescored.indexed_before.possible, 1);
        assert_eq!(rescored.indexed_after.confident, 2);
        assert_eq!(rescored.moves, vec![(Level::Possible, Level::Confident, 1)]);
        assert_eq!(rescored.by_rule, 1);
        assert_eq!(rescored.drifted_plays, 0);
        assert_eq!(rescored.drifted_mixes, 0);
        assert_eq!(report["an_old_field"], "kept");
        let first = &report["mixes"][0]["score"]["plays"][0];
        assert_eq!(first["level"], "confident");
        assert_eq!(first["best"]["hits"], 3000);
        assert_eq!(first["at_level"]["hits"], 1000);
    }

    #[test]
    fn a_play_scored_under_other_clusters_counts_as_drifted() {
        let mut report = saved_report();
        // As if the saved strongest detection had been of another file.
        report["mixes"][0]["score"]["plays"][1]["best"]["hits"] = serde_json::json!(800);
        let rescored = rescore(&mut report, &no_clusters(), &SameTitles::default()).unwrap();
        assert_eq!(rescored.drifted_plays, 1);
    }

    #[test]
    fn a_grid_query_takes_the_highest_level_of_its_matching_detections() {
        let mut report = serde_json::json!({
            "seed": 1,
            "ladder": "both",
            "indexed_assets": 2,
            "briefs": [],
            "queries": [
                {"brief": 0, "seconds": 20.0, "offset_seconds": 0, "level": "possible",
                 "matching": [found("a.mp3", 5.0, 2, 3000), found("a.mp3", 6.0, 3, 1000)],
                 "wrong_confident": 0, "strongest_false_hits": 0},
                {"brief": 0, "seconds": 20.0, "offset_seconds": 1, "level": "none",
                 "matching": [], "wrong_confident": 0, "strongest_false_hits": 0},
            ],
        });
        let rescored = rescore(&mut report, &no_clusters(), &SameTitles::default()).unwrap();
        assert_eq!(report["queries"][0]["level"], "confident");
        assert_eq!(report["queries"][1]["level"], "none");
        assert_eq!(rescored.moves, vec![(Level::Possible, Level::Confident, 1)]);
        assert_eq!(rescored.drifted_plays, 0);
    }

    #[test]
    fn the_rescored_report_goes_beside_the_saved_one_never_over_it() {
        assert_eq!(
            rescored_path(Path::new("work/reports/mixes-seed-7.json")).unwrap(),
            PathBuf::from("work/reports/mixes-seed-7-rescored.json")
        );
        assert!(rescored_path(Path::new("work/reports/mixes-seed-7-rescored.json")).is_err());
    }
}

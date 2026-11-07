//! Regression checks: the standard evaluation (the sweep, the development
//! scan and its leave-outs) compared with a saved baseline, so the effect of
//! a change on results is known before it is committed. Test-set reports are
//! never saved or compared.
//!
//! Before comparing, each pair of reports is checked to describe the same
//! queries (the sweep: count, then each query's asset, start, speed and
//! held-out status, in order) or the same scan (set, audio, leave-outs and
//! the manifest's tracks); a pair that does not is printed as
//! "incomparable", apart from "changed" and "identical". The baseline's
//! `provenance.json`, where it has one, is compared with the current
//! inputs. A baseline saved before a report joined the evaluation is
//! "incomplete": what it holds is compared, and the rest is named.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Pass;
use serde_json::Value;

use crate::calibrate::{Calibration, Sample};
use crate::files::{read_json, write_json};
use crate::provenance::{self, Provenance};
use crate::scan::ScanReport;
use crate::scoring::Found;
use crate::sweep::{Query, SweepReport};

/// Leave-out counts of the standard evaluation.
pub const LEAVE_OUTS: [usize; 2] = [3, 11];
/// Changed detections listed per scan.
const CHANGES_SHOWN: usize = 10;
/// Queries listed per kind of mismatch.
const MISMATCHES_SHOWN: usize = 3;
/// Fields that differ between runs of the same build on the same inputs.
const TIMING_FIELDS: [&str; 1] = ["wall_seconds"];

/// The development sets scanned whole besides `--set` (brief 78): with
/// Stakka's, they hold every development track and the only real version
/// plays.
const OTHER_DEVELOPMENT_SETS: [&str; 2] =
    ["razorjack-2003-03-29", "nocny-trans-vertical-2002-08-23"];

/// The development sets scanned whole besides `set`.
pub fn other_sets(set: &str) -> impl Iterator<Item = &'static str> {
    OTHER_DEVELOPMENT_SETS
        .into_iter()
        .filter(move |other| *other != set)
}

/// Report files of the standard evaluation, by name: `set`'s (as
/// `set_report_names`), then the other development sets' scans.
pub fn report_names(set: &str, seed: u64) -> Vec<String> {
    let mut names = set_report_names(set, seed);
    names.extend(other_sets(set).map(|other| format!("scan-{other}.json")));
    names
}

/// The sweep, `set`'s scan and its leave-outs: the reports calibrate
/// reads for `set`.
fn set_report_names(set: &str, seed: u64) -> Vec<String> {
    let mut names = vec![
        format!("sweep-seed-{seed}.json"),
        format!("scan-{set}.json"),
    ];
    for count in LEAVE_OUTS {
        names.push(format!("scan-{set}-leave-out-{count}-seed-{seed}.json"));
    }
    names
}

/// Copies the standard reports into `baseline`, with `provenance` beside
/// them.
pub fn save(
    reports: &Path,
    baseline: &Path,
    set: &str,
    seed: u64,
    provenance: &Provenance,
) -> Result<(), String> {
    fs::create_dir_all(baseline)
        .map_err(|error| format!("cannot create {}: {error}", baseline.display()))?;
    for name in report_names(set, seed) {
        let from = reports.join(&name);
        fs::copy(&from, baseline.join(&name)).map_err(|error| {
            format!(
                "cannot copy {} ({error}); run the standard evaluation first",
                from.display()
            )
        })?;
    }
    write_json(&baseline.join(provenance::FILE), provenance)?;
    println!("saved baseline {} with its provenance", baseline.display());
    Ok(())
}

/// Writes `provenance` beside a baseline saved without one, leaving its
/// reports as they are. Refused when the baseline has a provenance file
/// already, or when a current report differs from the baseline's apart
/// from timing: the provenance then describes the build and inputs that
/// reproduced these reports.
pub fn save_provenance_only(
    reports: &Path,
    baseline: &Path,
    set: &str,
    seed: u64,
    provenance: &Provenance,
) -> Result<(), String> {
    let file = baseline.join(provenance::FILE);
    if file.exists() {
        return Err(format!("{} exists already", file.display()));
    }
    let build = std::env::current_exe().map_err(|error| error.to_string())?;
    for name in report_names(set, seed) {
        // A report the baseline was saved without is not described.
        if !baseline.join(&name).exists() {
            continue;
        }
        if modified(&reports.join(&name))? < modified(&build)? {
            return Err(format!(
                "{name} is older than this build; run regress with it first"
            ));
        }
        if !same_apart_from_timing(&baseline.join(&name), &reports.join(&name))? {
            return Err(format!(
                "{name} differs from the baseline's; run regress first and save a new baseline"
            ));
        }
    }
    write_json(
        &file,
        &Provenance {
            after_the_fact: true,
            ..provenance.clone()
        },
    )?;
    println!(
        "wrote {} beside the baseline's reports, which are unchanged",
        file.display()
    );
    Ok(())
}

fn modified(path: &Path) -> Result<std::time::SystemTime, String> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}

/// What a comparison found.
#[derive(Debug, Default)]
pub struct Verdict {
    /// Pairs of reports that differ.
    pub changed: Vec<String>,
    /// Pairs of reports that do not describe the same queries or scan, and
    /// inputs of the provenance that differ.
    pub incomparable: Vec<String>,
    /// Reports the baseline does not hold.
    pub incomplete: Vec<String>,
}

impl Verdict {
    /// Whether every report the baseline holds is equal to the current one.
    pub fn identical(&self) -> bool {
        self.changed.is_empty() && self.incomparable.is_empty()
    }

    /// Whether the baseline holds every report of the evaluation.
    pub fn complete(&self) -> bool {
        self.incomplete.is_empty()
    }
}

/// Prints what changed between the baseline and the current reports, with
/// `now` the current provenance.
pub fn compare(
    baseline: &Path,
    reports: &Path,
    set: &str,
    seed: u64,
    now: &Provenance,
) -> Result<Verdict, String> {
    println!("compared with baseline {}", baseline.display());
    let mut verdict = Verdict::default();
    compare_provenance(baseline, now, &mut verdict)?;
    let names = report_names(set, seed);
    let (held, missing): (Vec<&String>, Vec<&String>) =
        names.iter().partition(|name| baseline.join(name).exists());
    for name in missing {
        println!("{name}: incomplete: the baseline has no such report");
        verdict.incomplete.push(name.clone());
    }
    for name in &held {
        if name.starts_with("sweep-") {
            compare_sweeps(
                name,
                &read_json(&baseline.join(name))?,
                &read_json(&reports.join(name))?,
                &mut verdict,
            );
        } else {
            compare_scans(
                name,
                &read_json(&baseline.join(name))?,
                &read_json(&reports.join(name))?,
                &mut verdict,
            );
        }
    }
    compare_calibrations(
        &standard_calibration(baseline, set, seed)?,
        &standard_calibration(reports, set, seed)?,
    );
    for name in held {
        if !same_apart_from_timing(&baseline.join(name), &reports.join(name))?
            && !verdict.changed.contains(name)
            && !verdict.incomparable.contains(name)
        {
            println!("{name}: differs from the baseline's apart from timing");
            verdict.changed.push(name.clone());
        }
    }
    let incomplete = if verdict.complete() {
        String::new()
    } else {
        format!(
            "; incomplete: the baseline has no {}",
            verdict.incomplete.join(", ")
        )
    };
    if verdict.identical() && verdict.complete() {
        println!("regress: every report equal to its baseline's apart from timing");
    } else if verdict.identical() {
        println!(
            "regress: every report the baseline holds equal to its baseline's apart from timing{incomplete}"
        );
    } else {
        println!(
            "regress: {} changed, {} incomparable{incomplete}",
            listed(&verdict.changed),
            listed(&verdict.incomparable)
        );
    }
    Ok(verdict)
}

fn listed(names: &[String]) -> String {
    if names.is_empty() {
        String::from("none")
    } else {
        names.join(", ")
    }
}

fn compare_provenance(
    baseline: &Path,
    now: &Provenance,
    verdict: &mut Verdict,
) -> Result<(), String> {
    let file = baseline.join(provenance::FILE);
    if !file.exists() {
        println!(
            "provenance: the baseline has no {} (saved before session 77); inputs not compared",
            provenance::FILE
        );
        return Ok(());
    }
    let saved: Provenance = read_json(&file)?;
    if saved.build != now.build {
        println!(
            "provenance: another build ({} -> {})",
            short(&saved.build),
            short(&now.build)
        );
    }
    for (field, before, after) in saved.differences(now) {
        println!("provenance: {field} differs: {before} -> {after}");
        verdict.incomparable.push(format!("provenance ({field})"));
    }
    Ok(())
}

fn short(digest: &str) -> &str {
    digest.get(..12).unwrap_or(digest)
}

/// Whether two report files are equal apart from `TIMING_FIELDS`.
fn same_apart_from_timing(a: &Path, b: &Path) -> Result<bool, String> {
    let mut a: Value = read_json(a)?;
    let mut b: Value = read_json(b)?;
    strip_timing(&mut a);
    strip_timing(&mut b);
    Ok(a == b)
}

fn strip_timing(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for field in TIMING_FIELDS {
                map.remove(field);
            }
            map.values_mut().for_each(strip_timing);
        }
        Value::Array(values) => values.iter_mut().for_each(strip_timing),
        _ => {}
    }
}

/// The calibration evidence of `set`'s standard reports in `dir`. Other
/// sweeps and scans there (other seeds, other leave-outs) are left out,
/// because the baseline holds only the standard reports, and so are the
/// other development sets' scans, as `calibrate --set` leaves them out.
fn standard_calibration(dir: &Path, set: &str, seed: u64) -> Result<Calibration, String> {
    let names = set_report_names(set, seed);
    let (sweep_name, scan_names) = names.split_first().ok_or("no reports")?;
    let scans: Vec<PathBuf> = scan_names.iter().map(|name| dir.join(name)).collect();
    Calibration::from_files(&[dir.join(sweep_name)], &scans)
}

/// A query as the sweep drew it: what it plays, from where, at which
/// speed, and whether its file was held out.
fn identity(query: &Query) -> (String, u64, u64, bool) {
    (
        query.asset.clone(),
        query.start_seconds.to_bits(),
        query.speed_percent.to_bits(),
        query.held_out,
    )
}

fn describe(query: &Query) -> String {
    format!(
        "{} at {:.1} s, {:+.0}%{}",
        query.asset,
        query.start_seconds,
        query.speed_percent,
        if query.held_out { ", held out" } else { "" }
    )
}

/// Why two sweeps do not describe the same queries, if they do not.
fn sweep_mismatch(before: &SweepReport, after: &SweepReport) -> Option<String> {
    if before.seed != after.seed {
        return Some(format!("seed {} -> {}", before.seed, after.seed));
    }
    let speeds = |report: &SweepReport| -> Vec<u64> {
        report
            .per_speed
            .iter()
            .map(|row| row.speed_percent.to_bits())
            .collect()
    };
    if speeds(before) != speeds(after) {
        return Some(format!(
            "speed rows {} -> {}",
            before.per_speed.len(),
            after.per_speed.len()
        ));
    }
    let a: Vec<_> = before.queries.iter().map(identity).collect();
    let b: Vec<_> = after.queries.iter().map(identity).collect();
    if a == b {
        return None;
    }
    let missing: Vec<String> = before
        .queries
        .iter()
        .filter(|query| !b.contains(&identity(query)))
        .take(MISMATCHES_SHOWN)
        .map(describe)
        .collect();
    let added: Vec<String> = after
        .queries
        .iter()
        .filter(|query| !a.contains(&identity(query)))
        .take(MISMATCHES_SHOWN)
        .map(describe)
        .collect();
    let mut reason = format!("{} queries -> {}", a.len(), b.len());
    if missing.is_empty() && added.is_empty() {
        let (mut sorted_a, mut sorted_b) = (a.clone(), b.clone());
        sorted_a.sort();
        sorted_b.sort();
        reason.push_str(if sorted_a == sorted_b {
            ", the same queries in another order"
        } else {
            ", the same queries with other repeats"
        });
    }
    if !missing.is_empty() {
        reason.push_str(&format!("; not in the new report: {}", missing.join("; ")));
    }
    if !added.is_empty() {
        reason.push_str(&format!("; not in the baseline: {}", added.join("; ")));
    }
    Some(reason)
}

/// Whether two outcomes differ only in how they were scored (right, same
/// title, partner), with the same file, evidence and speed.
fn scored_otherwise(a: &Query, b: &Query) -> bool {
    a.detections.len() == b.detections.len()
        && a.detections.iter().zip(&b.detections).all(|(x, y)| {
            x.asset == y.asset
                && x.confident == y.confident
                && x.windows == y.windows
                && x.hits == y.hits
                && x.speed_percent == y.speed_percent
                && x.fitted == y.fitted
                && x.verified == y.verified
        })
}

fn compare_sweeps(name: &str, before: &SweepReport, after: &SweepReport, verdict: &mut Verdict) {
    if let Some(reason) = sweep_mismatch(before, after) {
        println!("{name}: incomparable: {reason}");
        verdict.incomparable.push(name.to_owned());
        return;
    }
    if before.indexed_assets != after.indexed_assets {
        println!(
            "{name}: indexed assets {} -> {}",
            before.indexed_assets, after.indexed_assets
        );
    }
    let pairs = || before.queries.iter().zip(&after.queries);
    let same_queries = pairs()
        .filter(|(a, b)| a.detections == b.detections)
        .count();
    let rescored = pairs()
        .filter(|(a, b)| a.detections != b.detections && scored_otherwise(a, b))
        .count();
    println!(
        "{name}: {same_queries} of {} queries with identical detections",
        after.queries.len()
    );
    if rescored > 0 {
        println!(
            "  {rescored} queries with the same detections scored otherwise (right, same title or partner)"
        );
    }
    let mut rows_changed = false;
    for (a, b) in before.per_speed.iter().zip(&after.per_speed) {
        if a.recalled != b.recalled
            || a.wrong_answers != b.wrong_answers
            || a.same_title != b.same_title
            || (a.max_speed_error_percent - b.max_speed_error_percent).abs() > 1e-9
        {
            rows_changed = true;
            println!(
                "  {:+.0}%: recalled {} -> {}, wrong {} -> {}, same-title {} -> {}, max speed error {:.3}% -> {:.3}%",
                a.speed_percent,
                a.recalled,
                b.recalled,
                a.wrong_answers,
                b.wrong_answers,
                a.same_title,
                b.same_title,
                a.max_speed_error_percent,
                b.max_speed_error_percent
            );
        }
    }
    if same_queries != after.queries.len() || rows_changed {
        verdict.changed.push(name.to_owned());
    }
}

/// Why two scans do not describe the same scan, if they do not.
fn scan_mismatch(before: &ScanReport, after: &ScanReport) -> Option<String> {
    let tracks = |report: &ScanReport| -> Vec<(String, u64, bool)> {
        report
            .score
            .tracks
            .iter()
            .map(|track| {
                (
                    track.label.clone(),
                    track.start_seconds.to_bits(),
                    track.referenced,
                )
            })
            .collect()
    };
    let checks = [
        ("set", before.set != after.set),
        ("audio file", before.audio_file != after.audio_file),
        (
            "left-out tracks",
            before.left_out_tracks != after.left_out_tracks,
        ),
        (
            "left-out slots",
            before.left_out_slots != after.left_out_slots,
        ),
        (
            "left-out files",
            before.left_out_assets != after.left_out_assets,
        ),
        ("the manifest's tracks", tracks(before) != tracks(after)),
    ];
    let differ: Vec<&str> = checks
        .iter()
        .filter(|(_, differs)| *differs)
        .map(|(what, _)| *what)
        .collect();
    (!differ.is_empty()).then(|| format!("{} differ", differ.join(", ")))
}

fn compare_scans(name: &str, before: &ScanReport, after: &ScanReport, verdict: &mut Verdict) {
    if let Some(reason) = scan_mismatch(before, after) {
        println!("{name}: incomparable: {reason}");
        verdict.incomparable.push(name.to_owned());
        return;
    }
    let (a, b) = (&before.score, &after.score);
    println!(
        "{name}: identified {} -> {}, wrong {} -> {}, same-title {} -> {}, possible {} -> {}, unmatched possible plays {} -> {}",
        a.identified,
        b.identified,
        a.wrong,
        b.wrong,
        a.same_title,
        b.same_title,
        a.possible,
        b.possible,
        a.unmatched_possible.len(),
        b.unmatched_possible.len()
    );
    if before.detections == after.detections {
        println!("  all {} detections identical", after.detections.len());
        return;
    }
    verdict.changed.push(name.to_owned());
    let changes = detection_changes(&before.detections, &after.detections);
    println!(
        "  detections {} -> {}: {} changed, {} removed, {} added",
        before.detections.len(),
        after.detections.len(),
        changes.changed.len(),
        changes.removed.len(),
        changes.added.len()
    );
    let mut shown = 0;
    for (old, new) in &changes.changed {
        if (old.confident || new.confident) && shown < CHANGES_SHOWN {
            println!("  changed {}: {old:?} -> {new:?}", new.asset);
            shown += 1;
        }
    }
    for (label, list) in [("removed", &changes.removed), ("added", &changes.added)] {
        for found in list
            .iter()
            .filter(|found| found.confident)
            .take(CHANGES_SHOWN)
        {
            println!(
                "  {label} confident {} at {:.0} s, {} hits",
                found.asset, found.start_seconds, found.hits
            );
        }
    }
}

struct Changes<'a> {
    changed: Vec<(&'a Found, &'a Found)>,
    removed: Vec<&'a Found>,
    added: Vec<&'a Found>,
}

/// Pairs each old detection with the new detection of the same asset that
/// overlaps it most.
fn detection_changes<'a>(before: &'a [Found], after: &'a [Found]) -> Changes<'a> {
    let mut matched = vec![false; after.len()];
    let mut changes = Changes {
        changed: Vec::new(),
        removed: Vec::new(),
        added: Vec::new(),
    };
    for old in before {
        let best = after
            .iter()
            .enumerate()
            .filter(|(index, new)| !matched[*index] && new.asset == old.asset)
            .map(|(index, new)| (index, overlap(old, new)))
            .filter(|&(_, seconds)| seconds > 0.0)
            .max_by(|a, b| a.1.total_cmp(&b.1));
        match best {
            Some((index, _)) => {
                matched[index] = true;
                let new = &after[index];
                if new != old {
                    changes.changed.push((old, new));
                }
            }
            None => changes.removed.push(old),
        }
    }
    changes.added = after
        .iter()
        .zip(&matched)
        .filter(|(_, matched)| !**matched)
        .map(|(found, _)| found)
        .collect();
    changes
}

fn overlap(a: &Found, b: &Found) -> f64 {
    a.end_seconds.min(b.end_seconds) - a.start_seconds.max(b.start_seconds)
}

fn compare_calibrations(before: &Calibration, after: &Calibration) {
    let hits = |sample: Option<&Sample>| sample.map_or(0, |sample| sample.evidence.hits);
    println!(
        "calibrate: weakest identifying {} -> {}, strongest false {} -> {}, missed {} -> {}, false accepted {} -> {}",
        hits(before.weakest_identifying()),
        hits(after.weakest_identifying()),
        hits(before.strongest_false()),
        hits(after.strongest_false()),
        before.missed(),
        after.missed(),
        before.accepted_false(),
        after.accepted_false()
    );
    // One threshold on both sides, so the counts compare.
    let rule = Pass::Ladder.rule();
    println!(
        "calibrate: false candidates at {} hits or more {} -> {}, strongest on audio not in the index {} -> {}",
        rule.min_possible_hits / 2,
        before.near_possible(rule).len(),
        after.near_possible(rule).len(),
        hits(before.strongest_not_indexed()),
        hits(after.strongest_not_indexed())
    );
    println!(
        "calibrate: same-title credits {} -> {}",
        before.same_title_credits, after.same_title_credits
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::files::write_text;
    use crate::sweep::Outcome;

    /// A sweep of one indexed query whose own file is found with `hits`.
    fn sweep(seed: u64, hits: u32) -> SweepReport {
        SweepReport {
            seed,
            held_out_clusters: 0,
            indexed_assets: 1,
            per_speed: Vec::new(),
            queries: vec![Query {
                asset: "own.mp3".to_owned(),
                held_out: false,
                start_seconds: 0.0,
                speed_percent: 0.0,
                detections: vec![Outcome {
                    asset: "own.mp3".to_owned(),
                    correct: true,
                    same_title: false,
                    confident: true,
                    windows: 10,
                    hits,
                    speed_percent: 0.0,
                    fitted: true,
                    partner: false,
                    verified: None,
                }],
                candidates: None,
            }],
            set_aside: None,
            set_aside_like: None,
        }
    }

    const EMPTY_SCAN: &str = r#"{"set": "set", "audio_file": "mix.mp3",
        "duration_seconds": 60.0, "wall_seconds": 1.0, "left_out_tracks": [],
        "left_out_assets": [], "score": {"referenced": 0, "identified": 0,
        "wrong": 0, "possible": 0, "tracks": [], "wrong_identifications": [],
        "false_candidates": [], "unmatched_possible": []}, "detections": [],
        "plays": []}"#;

    #[test]
    fn the_calibration_compared_reads_only_the_standard_reports() {
        let dir = std::env::temp_dir().join(format!("gunfinger-regress-{}", std::process::id()));
        let names = report_names("set", 1);
        write_json(&dir.join(&names[0]), &sweep(1, 300)).unwrap();
        for name in &names[1..] {
            write_text(&dir.join(name), EMPTY_SCAN).unwrap();
        }
        // Another seed's sweep, weaker, and a scan of another leave-out.
        write_json(&dir.join("sweep-seed-2.json"), &sweep(2, 250)).unwrap();
        let every_seed = Calibration::collect(&dir, "set").unwrap();
        write_text(
            &dir.join("scan-set-leave-out-5-seed-1.json"),
            "not a report",
        )
        .unwrap();

        let standard = standard_calibration(&dir, "set", 1).unwrap();
        let every_scan = Calibration::collect(&dir, "set");
        fs::remove_file(dir.join("scan-set.json")).unwrap();
        let missing = standard_calibration(&dir, "set", 1);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(every_seed.weakest_identifying().unwrap().evidence.hits, 250);
        assert_eq!(standard.weakest_identifying().unwrap().evidence.hits, 300);
        assert!(every_scan.is_err(), "collect reads the other leave-out");
        assert!(missing.is_err());
    }

    /// A sweep of `assets` at 0%, each found with 400 hits.
    fn queries(assets: &[&str]) -> SweepReport {
        let mut report = sweep(2026, 400);
        let template = report.queries.remove(0);
        report.queries = assets
            .iter()
            .enumerate()
            .map(|(number, asset)| Query {
                asset: (*asset).to_owned(),
                start_seconds: 10.0 * number as f64,
                detections: vec![Outcome {
                    asset: (*asset).to_owned(),
                    ..serde_json::from_value(serde_json::to_value(&template.detections[0]).unwrap())
                        .unwrap()
                }],
                ..serde_json::from_value(serde_json::to_value(&template).unwrap()).unwrap()
            })
            .collect();
        report
    }

    fn compared(before: &SweepReport, after: &SweepReport) -> Verdict {
        let mut verdict = Verdict::default();
        compare_sweeps("sweep.json", before, after, &mut verdict);
        verdict
    }

    #[test]
    fn a_sweep_with_a_query_deleted_is_incomparable() {
        let before = queries(&["a.mp3", "b.mp3", "c.mp3"]);
        let mut after = queries(&["a.mp3", "b.mp3", "c.mp3"]);
        after.queries.remove(1);
        let reason = sweep_mismatch(&before, &after).unwrap();
        assert!(reason.starts_with("3 queries -> 2"), "{reason}");
        assert!(reason.contains("not in the new report: b.mp3"), "{reason}");
        let verdict = compared(&before, &after);
        assert_eq!(verdict.incomparable, ["sweep.json"]);
        assert!(verdict.changed.is_empty());
        // The shorter report first: the lists no longer zip to "2 of 2".
        assert!(!compared(&after, &before).incomparable.is_empty());
    }

    #[test]
    fn a_sweep_with_its_queries_reordered_is_incomparable() {
        let before = queries(&["a.mp3", "b.mp3", "c.mp3"]);
        let mut after = queries(&["a.mp3", "b.mp3", "c.mp3"]);
        after.queries.swap(0, 2);
        let reason = sweep_mismatch(&before, &after).unwrap();
        assert!(
            reason.contains("the same queries in another order"),
            "{reason}"
        );
        assert_eq!(compared(&before, &after).incomparable, ["sweep.json"]);
    }

    #[test]
    fn changed_truth_with_unchanged_detections_is_not_identical() {
        let before = queries(&["a.mp3", "b.mp3"]);
        // A query's file held out now: other truth for the same audio.
        let mut held_out = queries(&["a.mp3", "b.mp3"]);
        held_out.queries[1].held_out = true;
        assert_eq!(compared(&before, &held_out).incomparable, ["sweep.json"]);
        // The same detection scored wrong now, as other clusters would.
        let mut scored = queries(&["a.mp3", "b.mp3"]);
        scored.queries[0].detections[0].correct = false;
        assert!(scored_otherwise(&before.queries[0], &scored.queries[0]));
        let verdict = compared(&before, &scored);
        assert!(verdict.incomparable.is_empty());
        assert_eq!(verdict.changed, ["sweep.json"]);
        assert!(compared(&before, &queries(&["a.mp3", "b.mp3"])).identical());
    }

    #[test]
    fn a_scan_of_another_leave_out_is_incomparable() {
        let before: ScanReport = serde_json::from_str(EMPTY_SCAN).unwrap();
        let mut after: ScanReport = serde_json::from_str(EMPTY_SCAN).unwrap();
        assert_eq!(scan_mismatch(&before, &after), None);
        after.left_out_assets = vec![String::from("a.mp3")];
        assert_eq!(
            scan_mismatch(&before, &after).as_deref(),
            Some("left-out files differ")
        );
        let mut verdict = Verdict::default();
        compare_scans("scan.json", &before, &after, &mut verdict);
        assert_eq!(verdict.incomparable, ["scan.json"]);
    }

    #[test]
    fn a_baseline_without_provenance_is_read_and_one_with_other_inputs_is_incomparable() {
        let dir = std::env::temp_dir().join(format!("gunfinger-provenance-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let now = crate::provenance::tests::provenance();
        let mut verdict = Verdict::default();
        compare_provenance(&dir, &now, &mut verdict).unwrap();
        assert!(
            verdict.identical(),
            "a missing file is a note, not a difference"
        );

        let rebuilt = Provenance {
            build: String::from("another build"),
            ..now.clone()
        };
        write_json(&dir.join(provenance::FILE), &rebuilt).unwrap();
        compare_provenance(&dir, &now, &mut verdict).unwrap();
        assert!(verdict.identical(), "another build alone is not an input");

        let other = Provenance {
            clusters: String::from("other clusters"),
            ..now.clone()
        };
        write_json(&dir.join(provenance::FILE), &other).unwrap();
        compare_provenance(&dir, &now, &mut verdict).unwrap();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(verdict.incomparable, ["provenance (clusters)"]);
    }

    #[test]
    fn a_difference_outside_the_detections_is_changed_and_timing_is_not() {
        let root = std::env::temp_dir().join(format!("gunfinger-compare-{}", std::process::id()));
        let (reports, baseline) = (root.join("reports"), root.join("baseline"));
        let names = report_names("set", 1);
        for dir in [&reports, &baseline] {
            write_json(&dir.join(&names[0]), &sweep(1, 300)).unwrap();
            for name in &names[1..] {
                write_text(&dir.join(name), EMPTY_SCAN).unwrap();
            }
        }
        write_text(
            &reports.join(&names[1]),
            &EMPTY_SCAN.replace(r#""wall_seconds": 1.0"#, r#""wall_seconds": 2.0"#),
        )
        .unwrap();
        let now = crate::provenance::tests::provenance();
        let same = compare(&baseline, &reports, "set", 1, &now).unwrap();
        let mut more = sweep(1, 300);
        more.indexed_assets = 2;
        write_json(&reports.join(&names[0]), &more).unwrap();
        let changed = compare(&baseline, &reports, "set", 1, &now).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert!(same.identical(), "{same:?}");
        assert_eq!(changed.changed, [names[0].clone()]);
        assert!(changed.incomparable.is_empty());
    }

    #[test]
    fn the_evaluation_names_every_development_set_once() {
        let stakka = report_names("stakka-skynet-knowledge", 2026);
        let razorjack = report_names("razorjack-2003-03-29", 2026);

        assert_eq!(
            stakka,
            [
                "sweep-seed-2026.json",
                "scan-stakka-skynet-knowledge.json",
                "scan-stakka-skynet-knowledge-leave-out-3-seed-2026.json",
                "scan-stakka-skynet-knowledge-leave-out-11-seed-2026.json",
                "scan-razorjack-2003-03-29.json",
                "scan-nocny-trans-vertical-2002-08-23.json",
            ]
        );
        assert_eq!(
            razorjack
                .iter()
                .filter(|name| *name == "scan-razorjack-2003-03-29.json")
                .count(),
            1
        );
        assert!(razorjack.contains(&"scan-nocny-trans-vertical-2002-08-23.json".to_owned()));
    }

    #[test]
    fn a_baseline_without_the_other_sets_scans_is_incomplete_not_changed() {
        let root =
            std::env::temp_dir().join(format!("gunfinger-incomplete-{}", std::process::id()));
        let (reports, baseline) = (root.join("reports"), root.join("baseline"));
        let names = report_names("set", 1);
        let held = set_report_names("set", 1);
        for (dir, names) in [(&reports, &names), (&baseline, &held)] {
            write_json(&dir.join(&names[0]), &sweep(1, 300)).unwrap();
            for name in &names[1..] {
                write_text(&dir.join(name), EMPTY_SCAN).unwrap();
            }
        }
        let now = crate::provenance::tests::provenance();

        let verdict = compare(&baseline, &reports, "set", 1, &now).unwrap();
        fs::remove_dir_all(&root).unwrap();

        assert!(verdict.identical(), "{verdict:?}");
        assert!(!verdict.complete());
        assert_eq!(
            verdict.incomplete,
            [
                "scan-razorjack-2003-03-29.json",
                "scan-nocny-trans-vertical-2002-08-23.json"
            ]
        );
        assert!(verdict.changed.is_empty() && verdict.incomparable.is_empty());
    }

    #[test]
    fn provenance_is_written_beside_an_old_baseline_only_when_the_reports_agree() {
        let root = std::env::temp_dir().join(format!("gunfinger-beside-{}", std::process::id()));
        let (reports, baseline) = (root.join("reports"), root.join("baseline"));
        let names = report_names("set", 1);
        for dir in [&reports, &baseline] {
            write_json(&dir.join(&names[0]), &sweep(1, 300)).unwrap();
            for name in &names[1..] {
                write_text(&dir.join(name), EMPTY_SCAN).unwrap();
            }
        }
        // Timing alone may differ.
        write_text(
            &reports.join(&names[1]),
            &EMPTY_SCAN.replace(r#""wall_seconds": 1.0"#, r#""wall_seconds": 2.0"#),
        )
        .unwrap();
        let now = crate::provenance::tests::provenance();
        write_json(&reports.join(&names[0]), &sweep(1, 301)).unwrap();
        let refused = save_provenance_only(&reports, &baseline, "set", 1, &now);
        write_json(&reports.join(&names[0]), &sweep(1, 300)).unwrap();
        let written = save_provenance_only(&reports, &baseline, "set", 1, &now);
        let again = save_provenance_only(&reports, &baseline, "set", 1, &now);
        let saved: Result<Provenance, String> = read_json(&baseline.join(provenance::FILE));
        fs::remove_dir_all(&root).unwrap();

        assert!(refused.is_err());
        assert!(written.is_ok(), "{written:?}");
        assert!(again.is_err(), "an existing provenance file is kept");
        assert!(saved.unwrap().after_the_fact);
    }

    #[test]
    fn timing_speed_confidence_and_pass_changes_are_counted() {
        let old = Found {
            asset: "track.wav".into(),
            start_seconds: 10.0,
            end_seconds: 40.0,
            speed: 1.0,
            windows: 3,
            hits: 300,
            confident: true,
            fitted: true,
            verified: None,
        };
        for field in 0..4 {
            let mut new = old.clone();
            match field {
                0 => new.start_seconds += 1.0,
                1 => new.speed = 1.02,
                2 => new.confident = false,
                _ => new.fitted = false,
            }
            let changes = detection_changes(std::slice::from_ref(&old), std::slice::from_ref(&new));
            assert_eq!(changes.changed.len(), 1);
            assert!(changes.added.is_empty());
            assert!(changes.removed.is_empty());
        }
        assert!(
            detection_changes(std::slice::from_ref(&old), std::slice::from_ref(&old))
                .changed
                .is_empty()
        );
    }
}

//! Regression checks: the standard evaluation (the sweep, the development
//! scan and its leave-outs) compared with a saved baseline, so the effect of
//! a change on results is known before it is committed. Test-set reports are
//! never saved or compared.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Pass;

use crate::calibrate::{Calibration, Sample};
use crate::files::read_json;
use crate::scan::ScanReport;
use crate::scoring::Found;
use crate::sweep::SweepReport;

/// Leave-out counts of the standard evaluation.
pub const LEAVE_OUTS: [usize; 2] = [3, 11];
/// Changed detections listed per scan.
const CHANGES_SHOWN: usize = 10;

/// Report files of the standard evaluation, by name.
pub fn report_names(set: &str, seed: u64) -> Vec<String> {
    let mut names = vec![
        format!("sweep-seed-{seed}.json"),
        format!("scan-{set}.json"),
    ];
    for count in LEAVE_OUTS {
        names.push(format!("scan-{set}-leave-out-{count}-seed-{seed}.json"));
    }
    names
}

/// Copies the standard reports into `baseline`.
pub fn save(reports: &Path, baseline: &Path, set: &str, seed: u64) -> Result<(), String> {
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
    println!("saved baseline {}", baseline.display());
    Ok(())
}

/// Prints what changed between the baseline and the current reports.
pub fn compare(baseline: &Path, reports: &Path, set: &str, seed: u64) -> Result<(), String> {
    println!("compared with baseline {}", baseline.display());
    let names = report_names(set, seed);
    let (sweep_name, scan_names) = names.split_first().ok_or("no reports")?;
    compare_sweeps(
        sweep_name,
        &read_json(&baseline.join(sweep_name))?,
        &read_json(&reports.join(sweep_name))?,
    );
    for name in scan_names {
        compare_scans(
            name,
            &read_json(&baseline.join(name))?,
            &read_json(&reports.join(name))?,
        );
    }
    compare_calibrations(
        &standard_calibration(baseline, set, seed)?,
        &standard_calibration(reports, set, seed)?,
    );
    Ok(())
}

/// The calibration evidence of the standard reports in `dir`. Other sweeps
/// and scans there (other seeds, other leave-outs) are left out, because
/// the baseline holds only the standard reports.
fn standard_calibration(dir: &Path, set: &str, seed: u64) -> Result<Calibration, String> {
    let names = report_names(set, seed);
    let (sweep_name, scan_names) = names.split_first().ok_or("no reports")?;
    let scans: Vec<PathBuf> = scan_names.iter().map(|name| dir.join(name)).collect();
    Calibration::from_files(&[dir.join(sweep_name)], &scans)
}

fn compare_sweeps(name: &str, before: &SweepReport, after: &SweepReport) {
    let same_queries = before
        .queries
        .iter()
        .zip(&after.queries)
        .filter(|(a, b)| a.detections == b.detections)
        .count();
    println!(
        "{name}: {same_queries} of {} queries with identical detections",
        after.queries.len()
    );
    for (a, b) in before.per_speed.iter().zip(&after.per_speed) {
        if a.recalled != b.recalled
            || a.wrong_answers != b.wrong_answers
            || a.same_title != b.same_title
            || (a.max_speed_error_percent - b.max_speed_error_percent).abs() > 1e-9
        {
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
}

fn compare_scans(name: &str, before: &ScanReport, after: &ScanReport) {
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
    use crate::files::{write_json, write_text};
    use crate::sweep::{Outcome, Query};

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

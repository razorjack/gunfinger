//! The confidence margin: how far the evidence of correct detections lies
//! above the strongest false candidate, over the sweep and the development
//! set with its leave-outs; and where the possible tier's threshold lies
//! against the false candidates. Test-set reports are never read.
//!
//! Confidence is recomputed from the stored evidence with the current rule,
//! so reports written under an earlier rule can be re-examined.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::{Evidence, MIN_HITS, MIN_POSSIBLE_HITS, MIN_WINDOWS};
use serde::de::DeserializeOwned;

use crate::scan::ScanReport;
use crate::scoring::Found;
use crate::sweep::SweepReport;

const NO_EVIDENCE: Evidence = Evidence {
    windows: 0,
    hits: 0,
};

/// One detection's evidence and where it came from.
struct Sample {
    evidence: Evidence,
    source: String,
}

pub fn run(reports: &Path, development_set: &str) -> Result<(), String> {
    // The strongest correct detection of each indexed sweep query and each
    // referenced development track: the one that identifies it.
    let mut identifying = Vec::new();
    let mut false_candidates = Vec::new();
    // Every detection on a held-out sweep excerpt: audio whose recording is
    // not in the index at all.
    let mut not_indexed = Vec::new();

    for path in report_files(reports, "sweep-seed-")? {
        let sweep: SweepReport = read(&path)?;
        for query in &sweep.queries {
            let source = format!("sweep {} at {:+.0}%", query.asset, query.speed_percent);
            let mut best: Option<Sample> = None;
            for outcome in &query.detections {
                let sample = Sample {
                    evidence: Evidence {
                        windows: outcome.windows,
                        hits: outcome.hits,
                    },
                    source: format!("{source}: {}", outcome.asset),
                };
                if query.held_out {
                    not_indexed.push(Sample {
                        evidence: sample.evidence,
                        source: sample.source.clone(),
                    });
                }
                if !outcome.correct {
                    false_candidates.push(sample);
                } else if best
                    .as_ref()
                    .is_none_or(|best| sample.evidence.hits > best.evidence.hits)
                {
                    best = Some(sample);
                }
            }
            if !query.held_out {
                identifying.push(best.unwrap_or(Sample {
                    evidence: NO_EVIDENCE,
                    source: format!("{source}: nothing found"),
                }));
            }
        }
    }
    for path in report_files(reports, &format!("scan-{development_set}"))? {
        let scan: ScanReport = read(&path)?;
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        for track in &scan.score.tracks {
            let left_out = scan.left_out_tracks.contains(&track.label);
            if !track.referenced || left_out {
                continue;
            }
            identifying.push(Sample {
                evidence: track
                    .strongest_candidate
                    .as_ref()
                    .map_or(NO_EVIDENCE, Found::evidence),
                source: format!("{name}: {}", track.label),
            });
        }
        for found in &scan.score.false_candidates {
            false_candidates.push(Sample {
                evidence: found.evidence(),
                source: format!("{name}: {} at {:.0} s", found.asset, found.start_seconds),
            });
        }
    }

    let weakest = identifying
        .iter()
        .min_by_key(|sample| sample.evidence.hits)
        .ok_or("no sweep or development reports")?;
    let strongest = false_candidates
        .iter()
        .max_by_key(|sample| sample.evidence.hits)
        .ok_or("no false candidates")?;
    let missed = identifying
        .iter()
        .filter(|sample| !sample.evidence.is_confident())
        .count();
    let accepted_false = false_candidates
        .iter()
        .filter(|sample| sample.evidence.is_confident())
        .count();

    println!("rule: hits >= {MIN_HITS} and windows >= {MIN_WINDOWS}");
    println!(
        "identifying detections: {} ({missed} below the rule)",
        identifying.len()
    );
    println!(
        "false candidates: {} ({accepted_false} accepted by the rule)",
        false_candidates.len()
    );
    println!("weakest identifying: {}", describe(weakest));
    println!("strongest false:     {}", describe(strongest));
    println!(
        "margin: {:.2}x in hits; the threshold is {:.2}x the strongest false and {:.2}x below the weakest identifying",
        f64::from(weakest.evidence.hits) / f64::from(strongest.evidence.hits),
        f64::from(MIN_HITS) / f64::from(strongest.evidence.hits),
        f64::from(weakest.evidence.hits) / f64::from(MIN_HITS)
    );
    print_possible_tier(&false_candidates, &not_indexed);
    Ok(())
}

/// The possible tier (ADR 0006) claims "this recording, or one sharing
/// material with it". Its rule: the threshold is at least twice every false
/// candidate that is not a remix or version of the played recording. The
/// harness cannot tell a remix from an unrelated record, so it lists every
/// false candidate at half the threshold or more for a person to check.
fn print_possible_tier(false_candidates: &[Sample], not_indexed: &[Sample]) {
    println!();
    println!("possible tier: hits >= {MIN_POSSIBLE_HITS}");
    if let Some(strongest) = not_indexed.iter().max_by_key(|sample| sample.evidence.hits) {
        println!(
            "strongest on audio not in the index: {}; the threshold is {:.2}x that",
            describe(strongest),
            f64::from(MIN_POSSIBLE_HITS) / f64::from(strongest.evidence.hits)
        );
    }
    let half = MIN_POSSIBLE_HITS / 2;
    let mut strong: Vec<&Sample> = false_candidates
        .iter()
        .filter(|sample| sample.evidence.hits >= half)
        .collect();
    strong.sort_by_key(|sample| std::cmp::Reverse(sample.evidence.hits));
    let shown = strong
        .iter()
        .filter(|sample| sample.evidence.hits >= MIN_POSSIBLE_HITS)
        .count();
    println!(
        "false candidates at {half} hits or more: {} ({shown} shown as possible); each must be a remix or version of the played recording",
        strong.len()
    );
    for sample in strong {
        println!("  {}", describe(sample));
    }
    if let Some(strongest) = false_candidates
        .iter()
        .filter(|sample| sample.evidence.hits < half)
        .max_by_key(|sample| sample.evidence.hits)
    {
        println!(
            "strongest false candidate under {half} hits: {}",
            describe(strongest)
        );
    }
}

fn describe(sample: &Sample) -> String {
    format!(
        "{} hits, {} windows ({})",
        sample.evidence.hits, sample.evidence.windows, sample.source
    )
}

fn report_files(dir: &Path, prefix: &str) -> Result<Vec<PathBuf>, String> {
    let entries =
        fs::read_dir(dir).map_err(|error| format!("cannot read {}: {error}", dir.display()))?;
    let mut paths: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(prefix) && name.ends_with(".json"))
        })
        .collect();
    paths.sort();
    Ok(paths)
}

fn read<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
}

//! What longer hashes would cost and gain, estimated without a search
//! (experiment 0014). For each way of hashing a peak's neighbourhood:
//! postings per second of library audio (memory), postings a query looks up
//! per second (lookup cost, and the chance hits that come with it), and
//! query hashes that meet their reference at the true alignment per second
//! (the evidence for a true match), on the robustness excerpts.
//!
//! A triplet joins an anchor with two points of its target zone, so it is
//! far more specific than a pair but needs three peaks to survive. Panako's
//! triplets (Six & Leman 2014) hash time ratios to be invariant to speed;
//! these keep exact coordinates, like the pairs.

use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::hash::{Point, pair_hash, targets};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::{SpeedRatio, points_at_speed};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::Serialize;

use crate::clusters::Clusters;
use crate::sweep::{Draw, Plan};

#[derive(Clone, Copy, Debug)]
enum Hashing {
    /// A pair with each of the first `n` points of the target zone.
    Pairs(usize),
    /// A triplet with every two of the first `n` points of the target zone.
    Triplets(usize),
}

const HASHINGS: [Hashing; 5] = [
    Hashing::Pairs(1),
    Hashing::Pairs(2),
    Hashing::Pairs(3),
    Hashing::Triplets(2),
    Hashing::Triplets(3),
];
/// Robustness conditions whose excerpts keep one fixed alignment with the
/// reference (no skips, ramps or key lock).
const CONDITIONS: [&str; 10] = [
    "control",
    "low-pass-800hz",
    "telephone",
    "echo",
    "clipping-12db",
    "pink-noise-snr-10db",
    "pink-noise-snr-0db",
    "mp3-32k",
    "blend--6db",
    "blend-0db",
];
/// The robustness excerpts: the sweep's first indexed draws at two speeds.
const EXCERPTS: usize = 40;
const SPEEDS: [f64; 2] = [0.95, 1.03];
/// Residual speed errors: on the rung, and the worst case between rungs.
const RESIDUALS_PERCENT: [f64; 3] = [-0.2, 0.0, 0.2];
const EXCERPT_SECONDS: f64 = 30.0;
const ALIGNMENT_TOLERANCE: f64 = 2.0;

#[derive(Serialize)]
struct Report {
    seed: u64,
    hashings: Vec<LibraryRow>,
    /// Aligned query hashes per second between rungs, by condition, in the
    /// order of `hashings`.
    conditions: Vec<ConditionRow>,
}

#[derive(Serialize)]
struct LibraryRow {
    hashing: String,
    postings_per_second: f64,
    distinct_hashes: usize,
    /// Postings of a random posting's hash: the list a lookup of a typical
    /// hash scans.
    mean_list_scanned: f64,
    /// Postings in the fullest 1% of distinct hashes.
    share_in_fullest_percent: f64,
    /// On the control excerpts.
    query_hashes_per_second: f64,
    lookups_per_second: f64,
    /// With the fullest 1% of distinct hashes emptied, as
    /// `Index::without_fullest(0.01)` does (experiment 0013).
    lookups_per_second_fullest_dropped: f64,
    aligned_per_second_on_rung: f64,
    aligned_per_second_between_rungs: f64,
}

#[derive(Serialize)]
struct ConditionRow {
    condition: String,
    aligned_per_second_between_rungs: Vec<f64>,
}

/// Per hashing: query hashes, postings looked up, aligned hashes on the
/// rung and between rungs.
#[derive(Clone, Copy, Default)]
struct Counts {
    query_hashes: u64,
    lookups: u64,
    lookups_fullest_dropped: u64,
    on_rung: u64,
    between_rungs: u64,
}

struct Excerpt30<'a> {
    condition: &'static str,
    path: PathBuf,
    draw: &'a Draw,
    speed: f64,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    seed: u64,
    panels: &Path,
    work: &Path,
    jobs: usize,
) -> Result<(), String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let plan = Plan::for_seed(&records, clusters, seed, panels)?;
    let library_seconds: f64 = records
        .iter()
        .map(|record| record.header.duration_seconds)
        .sum();
    let library_hashes: Vec<HashMap<u64, u32>> = map_in_order(&HASHINGS, jobs, |&hashing| {
        let mut counts: HashMap<u64, u32> = HashMap::new();
        for record in &records {
            let points: Vec<Point> = record.peaks.iter().map(Point::from).collect();
            for_each_hash(&points, hashing, |hash, _| {
                *counts.entry(hash).or_default() += 1;
            });
        }
        counts
    });
    let longest_kept: Vec<u32> = library_hashes.iter().map(longest_kept).collect();

    let dir = work.join("robust").join(format!("seed-{seed}"));
    let mut excerpts = Vec::new();
    for condition in CONDITIONS {
        for speed in SPEEDS {
            for (number, draw) in plan
                .draws
                .iter()
                .filter(|draw| !draw.held_out)
                .take(EXCERPTS)
                .enumerate()
            {
                excerpts.push(Excerpt30 {
                    condition,
                    path: rendered(&dir.join(condition), number, speed)?,
                    draw,
                    speed,
                });
            }
        }
    }
    let measured: Vec<Result<Vec<Counts>, String>> = map_in_order(&excerpts, jobs, |excerpt| {
        let record = records
            .iter()
            .find(|record| record.header.source.path == excerpt.draw.asset)
            .ok_or_else(|| format!("{} has no peak record", excerpt.draw.asset))?;
        let audio = decode(&excerpt.path, profile.sample_rate, Excerpt::default())
            .map_err(|error| error.to_string())?;
        Ok(measure(
            excerpt,
            record,
            &audio.samples,
            &library_hashes,
            &longest_kept,
        ))
    });
    let measured = measured.into_iter().collect::<Result<Vec<_>, String>>()?;

    let query_seconds = (EXCERPTS * SPEEDS.len()) as f64 * EXCERPT_SECONDS;
    let per_condition = |condition: &str| -> Vec<Counts> {
        let mut sums = vec![Counts::default(); HASHINGS.len()];
        for (excerpt, counts) in excerpts.iter().zip(&measured) {
            if excerpt.condition == condition {
                for (sum, count) in sums.iter_mut().zip(counts) {
                    sum.query_hashes += count.query_hashes;
                    sum.lookups += count.lookups;
                    sum.lookups_fullest_dropped += count.lookups_fullest_dropped;
                    sum.on_rung += count.on_rung;
                    sum.between_rungs += count.between_rungs;
                }
            }
        }
        sums
    };
    let control = per_condition("control");
    let report = Report {
        seed,
        hashings: HASHINGS
            .iter()
            .zip(&library_hashes)
            .zip(&control)
            .map(|((&hashing, counts), control)| {
                library_row(hashing, counts, library_seconds, control, query_seconds)
            })
            .collect(),
        conditions: CONDITIONS
            .iter()
            .map(|&condition| ConditionRow {
                condition: condition.to_owned(),
                aligned_per_second_between_rungs: per_condition(condition)
                    .iter()
                    .map(|counts| counts.between_rungs as f64 / (2.0 * query_seconds))
                    .collect(),
            })
            .collect(),
    };
    print(&report);
    let path = work
        .join("reports")
        .join(format!("hash-cost-seed-{seed}.json"));
    let json = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    fs::write(&path, json).map_err(|error| format!("{}: {error}", path.display()))
}

/// The robustness harness's rendering of excerpt `number` at `speed`, in
/// whatever format its condition uses.
fn rendered(dir: &Path, number: usize, speed: f64) -> Result<PathBuf, String> {
    let stem = format!("{number:03}-{speed:.3}.");
    let entries = fs::read_dir(dir).map_err(|error| {
        format!(
            "{} ({error}); run `gunfinger-eval robust` first",
            dir.display()
        )
    })?;
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with(&stem))
        })
        .ok_or_else(|| format!("{}: no excerpt {stem}*", dir.display()))
}

fn for_each_hash(points: &[Point], hashing: Hashing, mut emit: impl FnMut(u64, usize)) {
    for anchor in 0..points.len() {
        let a = points[anchor];
        match hashing {
            Hashing::Pairs(count) => {
                for target in targets(points, anchor).take(count) {
                    emit(u64::from(pair_hash(a, points[target]).0), anchor);
                }
            }
            Hashing::Triplets(count) => {
                let near: Vec<usize> = targets(points, anchor).take(count).collect();
                for (position, &b) in near.iter().enumerate() {
                    for &c in &near[position + 1..] {
                        let ab = u64::from(pair_hash(a, points[b]).0);
                        let ac = u64::from(pair_hash(a, points[c]).0);
                        emit((ab << 32) | ac, anchor);
                    }
                }
            }
        }
    }
}

/// The longest posting list `Index::without_fullest(0.01)` keeps.
fn longest_kept(counts: &HashMap<u64, u32>) -> u32 {
    let mut lengths: Vec<u32> = counts.values().copied().collect();
    lengths.sort_unstable_by(|a, b| b.cmp(a));
    let dropped = (0.01 * lengths.len() as f64).round() as usize;
    lengths.get(dropped).copied().unwrap_or(0)
}

fn measure(
    excerpt: &Excerpt30,
    record: &PeakRecord,
    samples: &[f32],
    library_hashes: &[HashMap<u64, u32>],
    longest_kept: &[u32],
) -> Vec<Counts> {
    let profile = Profile::CURRENT;
    let start_frame = profile.frames(excerpt.draw.start_seconds);
    let end_frame = start_frame + profile.frames(EXCERPT_SECONDS * excerpt.speed);
    let margin = profile.frames(1.0);
    // The reference around the excerpt, with a margin for pairs that start
    // just before it.
    let reference: Vec<Point> = record
        .peaks
        .iter()
        .map(Point::from)
        .filter(|point| (start_frame - margin..end_frame + margin).contains(&point.frame))
        .collect();
    let queries: Vec<(f64, Vec<Point>)> = RESIDUALS_PERCENT
        .iter()
        .map(|residual| {
            let rung = SpeedRatio(excerpt.speed * (1.0 + residual / 100.0));
            (*residual, points_at_speed(samples, &profile, rung))
        })
        .collect();

    HASHINGS
        .iter()
        .zip(library_hashes)
        .zip(longest_kept)
        .map(|((&hashing, library), &longest_kept)| {
            let mut frames_of: HashMap<u64, Vec<f64>> = HashMap::new();
            for_each_hash(&reference, hashing, |hash, anchor| {
                frames_of
                    .entry(hash)
                    .or_default()
                    .push(reference[anchor].frame);
            });
            let mut counts = Counts::default();
            for (residual, points) in &queries {
                let rung = excerpt.speed * (1.0 + residual / 100.0);
                for_each_hash(points, hashing, |hash, anchor| {
                    let expected = start_frame + points[anchor].frame / rung * excerpt.speed;
                    let aligned = frames_of.get(&hash).is_some_and(|frames| {
                        frames
                            .iter()
                            .any(|frame| (frame - expected).abs() <= ALIGNMENT_TOLERANCE)
                    });
                    if *residual == 0.0 {
                        counts.query_hashes += 1;
                        let postings = library.get(&hash).copied().unwrap_or(0);
                        counts.lookups += u64::from(postings);
                        if postings <= longest_kept {
                            counts.lookups_fullest_dropped += u64::from(postings);
                        }
                        counts.on_rung += u64::from(aligned);
                    } else {
                        counts.between_rungs += u64::from(aligned);
                    }
                });
            }
            counts
        })
        .collect()
}

fn library_row(
    hashing: Hashing,
    counts: &HashMap<u64, u32>,
    library_seconds: f64,
    control: &Counts,
    query_seconds: f64,
) -> LibraryRow {
    let postings: u64 = counts.values().map(|&count| u64::from(count)).sum();
    let squares: f64 = counts.values().map(|&count| f64::from(count).powi(2)).sum();
    let mut lengths: Vec<u32> = counts.values().copied().collect();
    lengths.sort_unstable_by(|a, b| b.cmp(a));
    let fullest: u64 = lengths[..lengths.len().div_ceil(100)]
        .iter()
        .map(|&count| u64::from(count))
        .sum();
    LibraryRow {
        hashing: format!("{hashing:?}"),
        postings_per_second: postings as f64 / library_seconds,
        distinct_hashes: counts.len(),
        mean_list_scanned: squares / postings as f64,
        share_in_fullest_percent: 100.0 * fullest as f64 / postings as f64,
        query_hashes_per_second: control.query_hashes as f64 / query_seconds,
        lookups_per_second: control.lookups as f64 / query_seconds,
        lookups_per_second_fullest_dropped: control.lookups_fullest_dropped as f64 / query_seconds,
        aligned_per_second_on_rung: control.on_rung as f64 / query_seconds,
        aligned_per_second_between_rungs: control.between_rungs as f64 / (2.0 * query_seconds),
    }
}

fn print(report: &Report) {
    println!(
        "{:<12} {:>7} {:>9} {:>6} {:>6} {:>8} {:>9} {:>9} {:>7} {:>7}",
        "hashing",
        "post/s",
        "distinct",
        "list",
        "top1%",
        "query/s",
        "lookups/s",
        "w/o top1%",
        "true/s",
        "between"
    );
    for row in &report.hashings {
        println!(
            "{:<12} {:>7.1} {:>9} {:>6.1} {:>5.1}% {:>8.1} {:>9.0} {:>9.0} {:>7.1} {:>7.1}",
            row.hashing,
            row.postings_per_second,
            row.distinct_hashes,
            row.mean_list_scanned,
            row.share_in_fullest_percent,
            row.query_hashes_per_second,
            row.lookups_per_second,
            row.lookups_per_second_fullest_dropped,
            row.aligned_per_second_on_rung,
            row.aligned_per_second_between_rungs
        );
    }
    println!("\naligned hashes per second between rungs (share of control)");
    print!("{:<20}", "condition");
    for row in &report.hashings {
        print!(" {:>13}", row.hashing);
    }
    println!();
    let control = &report.conditions[0].aligned_per_second_between_rungs;
    for row in &report.conditions {
        print!("{:<20}", row.condition);
        for (value, base) in row.aligned_per_second_between_rungs.iter().zip(control) {
            print!(" {:>6.1} ({:>3.0}%)", value, 100.0 * value / base);
        }
        println!();
    }
}

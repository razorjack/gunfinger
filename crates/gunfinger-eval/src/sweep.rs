//! The self-generated speed sweep.
//!
//! About a fifth of the duplicate clusters are held out of the index. Thirty
//! second excerpts of indexed and held-out assets are played at turntable
//! speeds and encoded as MP3. Indexed excerpts must return their own cluster;
//! held-out excerpts must return nothing confident. Everything is drawn from
//! a seed, so a run is repeatable.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search};
use gunfinger_core::speed::ladder;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::render::{Encoding, render_excerpt};
use crate::rng::Rng;

pub const SPEEDS_PERCENT: [f64; 9] = [-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0, 8.0];
const HELD_OUT_SHARE: f64 = 0.2;
const INDEXED_EXCERPTS: usize = 60;
const HELD_OUT_EXCERPTS: usize = 20;
const EXCERPT_SECONDS: f64 = 30.0;

#[derive(Serialize, Deserialize)]
pub struct SweepReport {
    pub seed: u64,
    pub held_out_clusters: usize,
    pub indexed_assets: usize,
    pub per_speed: Vec<SpeedRow>,
    pub queries: Vec<Query>,
}

#[derive(Serialize, Deserialize)]
pub struct SpeedRow {
    pub speed_percent: f64,
    pub indexed_excerpts: usize,
    pub recalled: usize,
    pub recall: f64,
    pub wrong_answers: usize,
    pub mean_speed_error_percent: f64,
    pub max_speed_error_percent: f64,
}

/// One excerpt at one speed and what the search made of it.
#[derive(Serialize, Deserialize)]
pub struct Query {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
    pub speed_percent: f64,
    /// Every detection with at least two windows, strongest first.
    pub detections: Vec<Outcome>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub asset: String,
    /// Whether the asset is in the excerpt's own cluster.
    pub correct: bool,
    pub confident: bool,
    pub windows: u32,
    pub hits: u32,
    pub speed_percent: f64,
}

/// An excerpt to render: which asset, from where.
struct Draw {
    asset: String,
    held_out: bool,
    start_seconds: f64,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    seed: u64,
    jobs: usize,
) -> Result<SweepReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let mut rng = Rng::new(seed);
    let held_out = held_out_assets(&records, clusters, &mut rng);
    let indexed: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !held_out.contains(&record.header.source.path))
        .cloned()
        .collect();
    let index = Index::build(&indexed).map_err(|error| error.to_string())?;
    let draws = draw_excerpts(&records, &held_out, &mut rng);
    let dir = work.join("sweep").join(format!("seed-{seed}"));
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let renders = render_all(library, &draws, &dir, jobs)?;

    // One excerpt per thread, each searched on a single thread.
    let searched: Vec<Result<Query, String>> =
        map_in_order(&renders, jobs, |(draw, speed_percent, path)| {
            let audio = decode(path, profile.sample_rate, Excerpt::default())
                .map_err(|error| error.to_string())?;
            let own_cluster = clusters.cluster_of(&draw.asset);
            let detections = search(&index, &audio.samples, &profile, &ladder(), 1);
            Ok(Query {
                asset: draw.asset.clone(),
                held_out: draw.held_out,
                start_seconds: draw.start_seconds,
                speed_percent: *speed_percent,
                detections: detections
                    .iter()
                    .map(|detection| outcome(&index, detection, &own_cluster))
                    .collect(),
            })
        });
    let queries = searched
        .into_iter()
        .collect::<Result<Vec<Query>, String>>()?;

    Ok(SweepReport {
        seed,
        held_out_clusters: clusters_held_out(clusters, &held_out),
        indexed_assets: indexed.len(),
        per_speed: SPEEDS_PERCENT
            .iter()
            .map(|&speed| speed_row(speed, &queries))
            .collect(),
        queries,
    })
}

/// Shuffles the clusters and holds out the first fifth of them, with every
/// member.
fn held_out_assets(records: &[PeakRecord], clusters: &Clusters, rng: &mut Rng) -> BTreeSet<String> {
    let mut keys: Vec<BTreeSet<String>> = records
        .iter()
        .map(|record| clusters.cluster_of(&record.header.source.path))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    rng.shuffle(&mut keys);
    let count = (keys.len() as f64 * HELD_OUT_SHARE).round() as usize;
    keys.into_iter().take(count).flatten().collect()
}

fn clusters_held_out(clusters: &Clusters, held_out: &BTreeSet<String>) -> usize {
    held_out
        .iter()
        .map(|asset| clusters.cluster_of(asset))
        .collect::<BTreeSet<_>>()
        .len()
}

fn draw_excerpts(records: &[PeakRecord], held_out: &BTreeSet<String>, rng: &mut Rng) -> Vec<Draw> {
    let long_enough: Vec<&PeakRecord> = records
        .iter()
        .filter(|record| record.header.duration_seconds >= 2.0 * EXCERPT_SECONDS)
        .collect();
    let mut draws = Vec::new();
    for (wanted, want_held_out) in [(INDEXED_EXCERPTS, false), (HELD_OUT_EXCERPTS, true)] {
        let mut pool: Vec<&&PeakRecord> = long_enough
            .iter()
            .filter(|record| held_out.contains(&record.header.source.path) == want_held_out)
            .collect();
        rng.shuffle(&mut pool);
        for record in pool.into_iter().take(wanted) {
            // Stay clear of intros and outros, which are often sparse.
            let duration = record.header.duration_seconds;
            let earliest = 0.15 * duration;
            let latest = 0.85 * duration - EXCERPT_SECONDS;
            draws.push(Draw {
                asset: record.header.source.path.clone(),
                held_out: want_held_out,
                start_seconds: earliest + rng.unit() * (latest - earliest).max(0.0),
            });
        }
    }
    draws
}

/// Renders every excerpt at every speed on `jobs` threads, reusing files
/// from an earlier run with the same seed.
fn render_all<'d>(
    library: &Library,
    draws: &'d [Draw],
    dir: &Path,
    jobs: usize,
) -> Result<Vec<(&'d Draw, f64, PathBuf)>, String> {
    let work: Vec<(&Draw, f64, PathBuf)> = draws
        .iter()
        .enumerate()
        .flat_map(|(number, draw)| {
            SPEEDS_PERCENT.iter().map(move |&speed| {
                let name = format!("{number:03}-{speed:+.0}.{}", Encoding::Mp3.extension());
                (draw, speed, dir.join(name))
            })
        })
        .collect();
    let failures: Vec<String> = map_in_order(&work, jobs, |(draw, speed, path)| {
        if path.exists() {
            return None;
        }
        let source = library.root.join(&draw.asset);
        let speed_ratio = 1.0 + speed / 100.0;
        render_excerpt(
            &source,
            draw.start_seconds,
            EXCERPT_SECONDS,
            speed_ratio,
            Encoding::Mp3,
            path,
        )
        .err()
    })
    .into_iter()
    .flatten()
    .collect();
    match failures.first() {
        Some(failure) => Err(failure.clone()),
        None => Ok(work),
    }
}

fn outcome(index: &Index, detection: &Detection, own_cluster: &BTreeSet<String>) -> Outcome {
    let asset = &index.asset(detection.asset).path;
    Outcome {
        asset: asset.clone(),
        correct: own_cluster.contains(asset),
        confident: detection.evidence.is_confident(),
        windows: detection.evidence.windows,
        hits: detection.evidence.hits,
        speed_percent: (detection.speed.0 - 1.0) * 100.0,
    }
}

fn speed_row(speed_percent: f64, queries: &[Query]) -> SpeedRow {
    let at_speed: Vec<&Query> = queries
        .iter()
        .filter(|query| query.speed_percent == speed_percent)
        .collect();
    let indexed: Vec<&&Query> = at_speed.iter().filter(|query| !query.held_out).collect();
    let mut errors = Vec::new();
    for query in &indexed {
        let best_correct = query
            .detections
            .iter()
            .find(|outcome| outcome.correct && outcome.confident);
        if let Some(outcome) = best_correct {
            errors.push((outcome.speed_percent - speed_percent).abs());
        }
    }
    let wrong_answers = at_speed
        .iter()
        .flat_map(|query| &query.detections)
        .filter(|outcome| outcome.confident && !outcome.correct)
        .count();
    SpeedRow {
        speed_percent,
        indexed_excerpts: indexed.len(),
        recalled: errors.len(),
        recall: errors.len() as f64 / indexed.len().max(1) as f64,
        wrong_answers,
        mean_speed_error_percent: errors.iter().sum::<f64>() / errors.len().max(1) as f64,
        max_speed_error_percent: errors.iter().copied().fold(0.0, f64::max),
    }
}

pub fn print_summary(report: &SweepReport) {
    println!(
        "sweep seed {}: {} indexed assets, {} clusters held out",
        report.seed, report.indexed_assets, report.held_out_clusters
    );
    println!(
        "{:>7} {:>8} {:>8} {:>6} {:>12} {:>11}",
        "speed", "recalled", "recall", "wrong", "mean error", "max error"
    );
    for row in &report.per_speed {
        println!(
            "{:>+6.0}% {:>4}/{:<3} {:>7.1}% {:>6} {:>11.3}% {:>10.3}%",
            row.speed_percent,
            row.recalled,
            row.indexed_excerpts,
            row.recall * 100.0,
            row.wrong_answers,
            row.mean_speed_error_percent,
            row.max_speed_error_percent
        );
    }
}

//! Hash survival against residual speed error.
//!
//! A query played at speed `s` is searched on a ladder of assumed speeds. On
//! the rung nearest `s` a residual error remains; this measures what fraction
//! of query hashes still meet their reference hash at the right time. It sets
//! the ladder step (experiment 0001).

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::hash::{PairHash, Point, for_each_pair};
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::{SpeedRatio, points_at_speed};
use gunfinger_core::store::PeakStore;

use crate::render::{Encoding, render_excerpt};

const SPEEDS: [f64; 5] = [0.92, 0.96, 1.0, 1.04, 1.08];
const RESIDUALS_PERCENT: [f64; 9] = [-0.5, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.5];
/// Deliberately off the 16 ms frame grid: a real query never shares the
/// reference's frame boundaries.
const EXCERPT_START: f64 = 90.0053;
const EXCERPT_SECONDS: f64 = 30.0;
/// Frames of slack when checking that a hit lies where it should.
const ALIGNMENT_TOLERANCE: f64 = 2.0;

pub fn run(
    library: &Library,
    store: &PeakStore,
    assets: &[String],
    work: &Path,
) -> Result<(), String> {
    let profile = Profile::CURRENT;
    let dir = work.join("survival");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    // Summed over assets, by speed and residual.
    let mut survival_sum = vec![vec![0.0; RESIDUALS_PERCENT.len()]; SPEEDS.len()];

    for asset_path in assets {
        let asset = library
            .assets
            .iter()
            .find(|asset| &asset.path == asset_path)
            .ok_or_else(|| format!("{asset_path} is not in the library"))?;
        let record = store
            .load(asset, &profile)
            .map_err(|error| error.to_string())?;
        let points: Vec<Point> = record.peaks.iter().map(Point::from).collect();
        let reference = reference_hashes(&points);
        let start_frame = profile.frames(EXCERPT_START);

        for (speed_index, &speed) in SPEEDS.iter().enumerate() {
            let rendered = dir.join(format!("speed-{speed:.4}.wav"));
            let source = library.absolute_path(asset);
            render_excerpt(
                &source,
                EXCERPT_START,
                EXCERPT_SECONDS,
                speed,
                Encoding::Lossless,
                &rendered,
            )?;
            let audio = decode(&rendered, profile.sample_rate, Excerpt::default())
                .map_err(|error| error.to_string())?;
            for (residual_index, residual) in RESIDUALS_PERCENT.iter().enumerate() {
                let rung = SpeedRatio(speed * (1.0 + residual / 100.0));
                let query = points_at_speed(&audio.samples, &profile, rung);
                survival_sum[speed_index][residual_index] +=
                    survival(&query, rung, &reference, start_frame, speed);
            }
        }
        eprintln!("measured {asset_path}");
    }

    println!(
        "hash survival (%) by residual speed error, mean of {} assets",
        assets.len()
    );
    print!("{:>9}", "residual");
    for speed in SPEEDS {
        print!(" {:>7}", format!("@{speed:.2}"));
    }
    println!();
    for (residual_index, residual) in RESIDUALS_PERCENT.iter().enumerate() {
        print!("{residual:>8.1}%");
        for row in &survival_sum {
            print!(
                " {:>7.1}",
                100.0 * row[residual_index] / assets.len() as f64
            );
        }
        println!();
    }
    Ok(())
}

fn reference_hashes(points: &[Point]) -> HashMap<PairHash, Vec<f64>> {
    let mut hashes: HashMap<PairHash, Vec<f64>> = HashMap::new();
    for_each_pair(points, |hash, anchor| {
        hashes.entry(hash).or_default().push(points[anchor].frame);
    });
    hashes
}

/// Fraction of query hashes found in the reference at the frame the true
/// speed predicts. Query points are in reference coordinates for `rung`.
fn survival(
    query: &[Point],
    rung: SpeedRatio,
    reference: &HashMap<PairHash, Vec<f64>>,
    start_frame: f64,
    true_speed: f64,
) -> f64 {
    let mut total = 0_usize;
    let mut aligned = 0_usize;
    for_each_pair(query, |hash, anchor| {
        total += 1;
        let query_frame = query[anchor].frame / rung.0;
        let expected = start_frame + query_frame * true_speed;
        let found = reference.get(&hash).is_some_and(|frames| {
            frames
                .iter()
                .any(|frame| (frame - expected).abs() <= ALIGNMENT_TOLERANCE)
        });
        if found {
            aligned += 1;
        }
    });
    aligned as f64 / total.max(1) as f64
}

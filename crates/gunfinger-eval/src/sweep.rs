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

use gunfinger_core::confidence::Pass;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::padding::Padding;
use crate::render::{Encoding, render_excerpt};
use crate::rng::Rng;
use crate::verifier::{Verification, Verifier};

pub const SPEEDS_PERCENT: [f64; 9] = [-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0, 8.0];
const HELD_OUT_SHARE: f64 = 0.2;
const INDEXED_EXCERPTS: usize = 60;
const HELD_OUT_EXCERPTS: usize = 20;
pub const EXCERPT_SECONDS: f64 = 30.0;

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
    /// Counted by the second pass (`Pass::Fitted`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fitted: bool,
    /// The peak verifier's measures, with `--verify`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<Verification>,
}

/// An excerpt to render: which asset, from where.
#[derive(Serialize, Deserialize)]
pub struct Draw {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
}

/// The sweep's seeded choice: the held-out assets and the excerpts, indexed
/// ones first. Other harness commands reuse it to work on the same audio.
pub struct Plan {
    pub held_out: BTreeSet<String>,
    pub draws: Vec<Draw>,
}

/// A plan as first drawn for a seed, kept in `panels/` and reused, so that
/// the same excerpts and held-out recordings are measured as the library
/// grows. Assets added later are indexed, unless they are duplicates of a
/// held-out one.
#[derive(Serialize, Deserialize)]
struct Panel {
    seed: u64,
    /// Library assets when the panel was drawn.
    drawn_from_assets: usize,
    held_out: BTreeSet<String>,
    draws: Vec<Draw>,
}

impl Plan {
    /// The panel saved for `seed` in `panels`; drawn and saved there the
    /// first time. It is always read back from the file, so that every run
    /// sees the same rounding of its start times.
    pub fn for_seed(
        records: &[PeakRecord],
        clusters: &Clusters,
        seed: u64,
        panels: &Path,
    ) -> Result<Plan, String> {
        let path = panels.join(format!("sweep-seed-{seed}.json"));
        if !path.exists() {
            let plan = Plan::draw(records, clusters, seed);
            let panel = Panel {
                seed,
                drawn_from_assets: records.len(),
                held_out: plan.held_out,
                draws: plan.draws,
            };
            fs::create_dir_all(panels)
                .map_err(|error| format!("cannot create {}: {error}", panels.display()))?;
            let text = serde_json::to_string_pretty(&panel).map_err(|error| error.to_string())?;
            fs::write(&path, text)
                .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        }
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let panel: Panel =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
        let present: BTreeSet<&str> = records
            .iter()
            .map(|record| record.header.source.path.as_str())
            .collect();
        let missing: Vec<&str> = panel
            .draws
            .iter()
            .map(|draw| draw.asset.as_str())
            .filter(|asset| !present.contains(asset))
            .collect();
        if let Some(first) = missing.first() {
            return Err(format!(
                "{} draws an excerpt from {} assets no longer in the library, such as {first}",
                path.display(),
                missing.len()
            ));
        }
        Ok(Plan {
            held_out: panel
                .held_out
                .iter()
                .flat_map(|asset| clusters.cluster_of(asset))
                .collect(),
            draws: panel.draws,
        })
    }

    fn draw(records: &[PeakRecord], clusters: &Clusters, seed: u64) -> Plan {
        let mut rng = Rng::new(seed);
        let held_out = held_out_assets(records, clusters, &mut rng);
        let draws = draw_excerpts(records, &held_out, &mut rng);
        Plan { held_out, draws }
    }
}

pub struct Options<'a> {
    pub seed: u64,
    /// Where the seed's panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    /// What is added to the index: another library's records.
    pub padding: &'a Padding,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// Measure every detection with the peak verifier.
    pub verify: bool,
    pub jobs: usize,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<SweepReport, String> {
    let Options {
        seed,
        panels,
        padding,
        ladder,
        matching,
        verify,
        jobs,
    } = *options;
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let verifier = verify.then(|| Verifier::new(&records, padding.second.as_ref()));
    let Plan { held_out, draws } = Plan::for_seed(&records, clusters, seed, panels)?;
    let indexed: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !held_out.contains(&record.header.source.path))
        .cloned()
        .collect();
    let index = matching.index(padding.index(&indexed, &profile, &held_out)?);
    let dir = work.join("sweep").join(format!("seed-{seed}"));
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let renders = render_all(library, &draws, &dir, jobs)?;

    // One excerpt per thread, each searched on a single thread.
    let searched: Vec<Result<Query, String>> =
        map_in_order(&renders, jobs, |(draw, speed_percent, path)| {
            let audio = decode(path, profile.sample_rate, Excerpt::default())
                .map_err(|error| error.to_string())?;
            let own_cluster = clusters.cluster_of(&draw.asset);
            let detections = matching.search(&index, &audio.samples, &profile, ladder, 1);
            Ok(Query {
                asset: draw.asset.clone(),
                held_out: draw.held_out,
                start_seconds: draw.start_seconds,
                speed_percent: *speed_percent,
                detections: detections
                    .iter()
                    .map(|detection| {
                        let verified = verifier.as_ref().and_then(|verifier| {
                            verifier.verify(&index, detection, &audio.samples, &profile)
                        });
                        outcome(&index, detection, &own_cluster, verified)
                    })
                    .collect(),
            })
        });
    let queries = searched
        .into_iter()
        .collect::<Result<Vec<Query>, String>>()?;

    Ok(SweepReport {
        seed,
        held_out_clusters: clusters_held_out(clusters, &held_out),
        indexed_assets: index.assets().len(),
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
                let name = format!("{number:03}-{speed:+.0}.{}", Encoding::Mp3(128).extension());
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
            Encoding::Mp3(128),
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

fn outcome(
    index: &Index,
    detection: &Detection,
    own_cluster: &BTreeSet<String>,
    verified: Option<Verification>,
) -> Outcome {
    let asset = &index.asset(detection.asset).path;
    Outcome {
        asset: asset.clone(),
        correct: own_cluster.contains(asset),
        confident: detection.evidence.is_confident(),
        windows: detection.evidence.windows,
        hits: detection.evidence.hits,
        speed_percent: (detection.speed.0 - 1.0) * 100.0,
        fitted: detection.evidence.pass == Pass::Fitted,
        verified,
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

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::store::RecordHeader;

    use super::*;

    fn record(path: &str) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: String::new(),
                source: Asset {
                    path: path.to_owned(),
                    size: 0,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 300.0,
            },
            peaks: Vec::new(),
        }
    }

    fn clusters(duplicates: Vec<Vec<String>>) -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates,
            pairs: Vec::new(),
        }
    }

    #[test]
    fn a_panel_keeps_its_draws_as_the_library_grows() {
        let panels = std::env::temp_dir().join(format!("gunfinger-panels-{}", std::process::id()));
        let library: Vec<PeakRecord> = (0..50).map(|n| record(&format!("{n:02}.mp3"))).collect();
        let first = Plan::for_seed(&library, &clusters(Vec::new()), 7, &panels).unwrap();
        let held_out = first.held_out.first().unwrap().clone();

        // A new rip of a held-out recording, and unrelated new tracks.
        let rip = String::from("new rip.mp3");
        let mut grown = library;
        grown.extend((0..30).map(|n| record(&format!("new {n:02}.mp3"))));
        grown.push(record(&rip));
        let grown_clusters = clusters(vec![vec![held_out, rip.clone()]]);
        let again = Plan::for_seed(&grown, &grown_clusters, 7, &panels).unwrap();
        let removed = Plan::for_seed(&grown[1..], &grown_clusters, 7, &panels);
        fs::remove_dir_all(&panels).unwrap();

        let excerpts = |plan: &Plan| -> Vec<(String, u64)> {
            plan.draws
                .iter()
                .map(|draw| (draw.asset.clone(), draw.start_seconds.to_bits()))
                .collect()
        };
        assert_eq!(excerpts(&again), excerpts(&first));
        assert!(again.held_out.is_superset(&first.held_out));
        assert_eq!(again.held_out.len(), first.held_out.len() + 1);
        assert!(again.held_out.contains(&rip));
        assert!(
            removed.is_err() == first.draws.iter().any(|draw| draw.asset == "00.mp3"),
            "a panel whose excerpt's asset is gone is an error"
        );
    }
}

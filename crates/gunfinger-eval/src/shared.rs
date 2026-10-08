//! Shared-material scenarios (roadmap, "A confidence statistic for shared
//! material"): queries built from a passage two different recordings share,
//! searched with the played recording in the index and left out, and
//! measured by the peak verifier.
//!
//! Each scenario names the played file, the related file that shares the
//! passage with it (both judged different recordings by the owner), and
//! the passage in the played file (`gunfinger-eval pair` finds it). Its
//! queries, at native speed:
//! - the passage alone, 10, 20 and 30 s (no longer than the passage);
//! - the passage looped to 60, 120 and 240 s;
//! - from the passage's start, 10, 20 and 30 s beyond its end, into the
//!   played file's own material.
//!
//! Each runs against three indexes: everything; the played file's whole
//! cluster left out, the related file kept; and, as a positive control,
//! the played file alone left out while another rip of it stays.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::render::{Encoding, Playback, encode, render_excerpt, render_samples};
use crate::verifier::{Verification, Verifier};

const ALONE_SECONDS: [f64; 3] = [10.0, 20.0, 30.0];
const LOOPED_SECONDS: [f64; 3] = [60.0, 120.0, 240.0];
const BEYOND_SECONDS: [f64; 3] = [10.0, 20.0, 30.0];

/// The scenarios to run, as written by hand.
#[derive(Deserialize)]
pub struct Plan {
    pub scenarios: Vec<Scenario>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub played: String,
    pub related: String,
    /// The shared passage in the played file, in seconds.
    pub passage_start_seconds: f64,
    pub passage_end_seconds: f64,
}

#[derive(Serialize)]
pub struct SharedReport {
    pub scenarios: Vec<Scenario>,
    pub results: Vec<QueryResult>,
}

/// One query searched against one index.
#[derive(Serialize)]
pub struct QueryResult {
    pub scenario: String,
    /// `alone`, `looped` or `beyond`.
    pub kind: String,
    pub query: String,
    pub seconds: f64,
    /// `everything`, `played left out` or `source left out`.
    pub index: String,
    pub left_out: Vec<String>,
    /// Every detection with two windows or more, strongest first.
    pub detections: Vec<SharedFound>,
}

#[derive(Serialize)]
pub struct SharedFound {
    pub asset: String,
    /// `played` (the played file's cluster), `related` (the related
    /// file's) or `other`.
    pub role: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speed: f64,
    pub windows: u32,
    pub hits: u32,
    pub level: String,
    pub verified: Option<Verification>,
}

/// How a query is cut from the played file.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Alone,
    Looped,
    Beyond,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Alone => "alone",
            Kind::Looped => "looped",
            Kind::Beyond => "beyond",
        }
    }
}

/// A query to render: where in the played file, how long, looped or not.
struct Query {
    kind: Kind,
    name: String,
    start_seconds: f64,
    /// Of the played file's audio; a looped query repeats it.
    source_seconds: f64,
    seconds: f64,
}

pub struct Options<'a> {
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub jobs: usize,
}

pub fn run(
    plan: &Plan,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<SharedReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let verifier = Verifier::new(&records, None);
    let dir = work.join("shared");
    fs::create_dir_all(&dir).map_err(|error| error.to_string())?;
    let mut results = Vec::new();
    for scenario in &plan.scenarios {
        let queries = queries(scenario);
        for query in &queries {
            render(library, scenario, query, &dir)?;
        }
        let played = clusters.cluster_of(&scenario.played);
        let related = clusters.cluster_of(&scenario.related);
        for (index_name, left_out) in indexes(scenario, &played) {
            let index = options.matching.index(
                Index::build(&without(&records, &left_out)).map_err(|error| error.to_string())?,
            );
            let searched: Vec<Result<QueryResult, String>> =
                map_in_order(&queries, options.jobs, |query| {
                    let path = dir.join(format!("{}.mp3", query.name));
                    let audio = decode(&path, profile.sample_rate, Excerpt::default())
                        .map_err(|error| error.to_string())?;
                    let detections = options.matching.search(
                        &index,
                        &audio.samples,
                        &profile,
                        options.ladder,
                        1,
                    );
                    Ok(QueryResult {
                        scenario: scenario.name.clone(),
                        kind: query.kind.name().to_owned(),
                        query: query.name.clone(),
                        seconds: query.seconds,
                        index: index_name.to_owned(),
                        left_out: left_out.iter().cloned().collect(),
                        detections: detections
                            .iter()
                            .map(|detection| {
                                let asset = &index.asset(detection.asset).path;
                                let role = if played.contains(asset) {
                                    "played"
                                } else if related.contains(asset) {
                                    "related"
                                } else {
                                    "other"
                                };
                                SharedFound {
                                    asset: asset.clone(),
                                    role: role.to_owned(),
                                    start_seconds: detection.start_seconds,
                                    end_seconds: detection.end_seconds,
                                    speed: detection.speed.0,
                                    windows: detection.evidence.windows,
                                    hits: detection.evidence.hits,
                                    level: format!("{:?}", detection.evidence.confidence())
                                        .to_lowercase(),
                                    verified: verifier.verify(
                                        &index,
                                        detection,
                                        &audio.samples,
                                        &profile,
                                    ),
                                }
                            })
                            .collect(),
                    })
                });
            for result in searched {
                results.push(result?);
            }
        }
    }
    Ok(SharedReport {
        scenarios: plan.scenarios.clone(),
        results,
    })
}

fn queries(scenario: &Scenario) -> Vec<Query> {
    let start = scenario.passage_start_seconds;
    let passage = scenario.passage_end_seconds - start;
    let mut queries = Vec::new();
    let mut alone: Vec<f64> = ALONE_SECONDS
        .iter()
        .map(|&seconds| seconds.min(passage))
        .collect();
    alone.dedup_by(|a, b| (*a - *b).abs() < 1e-9);
    for seconds in alone {
        queries.push(Query {
            kind: Kind::Alone,
            name: format!("{}-alone-{seconds:.0}s", scenario.name),
            start_seconds: start,
            source_seconds: seconds,
            seconds,
        });
    }
    for seconds in LOOPED_SECONDS {
        queries.push(Query {
            kind: Kind::Looped,
            name: format!("{}-looped-{seconds:.0}s", scenario.name),
            start_seconds: start,
            source_seconds: passage,
            seconds,
        });
    }
    for beyond in BEYOND_SECONDS {
        queries.push(Query {
            kind: Kind::Beyond,
            name: format!("{}-beyond-{beyond:.0}s", scenario.name),
            start_seconds: start,
            source_seconds: passage + beyond,
            seconds: passage + beyond,
        });
    }
    queries
}

/// Renders `query` from the played file unless an earlier run did; a
/// looped query repeats the passage's samples end to end.
fn render(library: &Library, scenario: &Scenario, query: &Query, dir: &Path) -> Result<(), String> {
    let path = dir.join(format!("{}.mp3", query.name));
    if path.exists() {
        return Ok(());
    }
    let source = library.root.join(&scenario.played);
    if query.kind != Kind::Looped {
        return render_excerpt(
            &source,
            query.start_seconds,
            query.seconds,
            1.0,
            Encoding::Mp3(128),
            &path,
        );
    }
    let passage = render_samples(
        &source,
        query.start_seconds,
        query.source_seconds,
        Playback::Turntable(1.0),
        None,
    )?;
    let length = (query.seconds / query.source_seconds * passage.len() as f64) as usize;
    let looped: Vec<f32> = passage.iter().copied().cycle().take(length).collect();
    encode(&looped, None, Encoding::Mp3(128), &path)
}

/// The indexes a scenario runs against, by name, as the files left out.
fn indexes(
    scenario: &Scenario,
    played: &BTreeSet<String>,
) -> Vec<(&'static str, BTreeSet<String>)> {
    let mut indexes = vec![
        ("everything", BTreeSet::new()),
        ("played left out", played.clone()),
    ];
    if played.len() > 1 {
        indexes.push(("source left out", BTreeSet::from([scenario.played.clone()])));
    }
    indexes
}

fn without(records: &[PeakRecord], left_out: &BTreeSet<String>) -> Vec<PeakRecord> {
    records
        .iter()
        .filter(|record| !left_out.contains(&record.header.source.path))
        .cloned()
        .collect()
}

pub fn print_summary(report: &SharedReport) {
    for result in &report.results {
        let strongest = |role: &str| {
            result
                .detections
                .iter()
                .filter(|found| found.role == role)
                .max_by_key(|found| found.hits)
                .map_or_else(
                    || String::from("-"),
                    |found| {
                        let share = found
                            .verified
                            .as_ref()
                            .map_or(0.0, |verified| verified.aligned.share);
                        format!(
                            "{} hits/{} windows {} share {share:.2}",
                            found.hits, found.windows, found.level
                        )
                    },
                )
        };
        println!(
            "{:<28} {:<16} played {:<40} related {:<40} other {}",
            result.query,
            result.index,
            strongest("played"),
            strongest("related"),
            strongest("other")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scenario(passage: f64) -> Scenario {
        Scenario {
            name: String::from("x"),
            played: String::from("a.mp3"),
            related: String::from("b.mp3"),
            passage_start_seconds: 100.0,
            passage_end_seconds: 100.0 + passage,
        }
    }

    #[test]
    fn queries_alone_are_no_longer_than_the_passage() {
        let queries = queries(&scenario(18.0));

        let alone: Vec<f64> = queries
            .iter()
            .filter(|query| query.kind == Kind::Alone)
            .map(|query| query.seconds)
            .collect();
        let beyond: Vec<f64> = queries
            .iter()
            .filter(|query| query.kind == Kind::Beyond)
            .map(|query| query.seconds)
            .collect();
        assert_eq!(alone, [10.0, 18.0]);
        assert_eq!(beyond, [28.0, 38.0, 48.0]);
        assert_eq!(queries.len(), 8);
    }

    #[test]
    fn the_positive_control_needs_another_rip() {
        let alone = BTreeSet::from([String::from("a.mp3")]);
        let ripped = BTreeSet::from([String::from("a.mp3"), String::from("a-rip.mp3")]);

        assert_eq!(indexes(&scenario(18.0), &alone).len(), 2);
        assert_eq!(indexes(&scenario(18.0), &ripped).len(), 3);
    }
}

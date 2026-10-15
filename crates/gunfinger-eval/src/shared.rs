//! Shared-material scenarios (roadmap, "A confidence statistic for shared
//! material"; experiments 0055, 0111): queries built from a passage two
//! different recordings share, searched with the played recording in the
//! index and left out, and measured by the peak verifier.
//!
//! Each scenario names the played file, the related file that shares the
//! passage with it (both judged different recordings by ear), and
//! the passage in the played file (`gunfinger-eval pair` finds it). Its
//! queries, at native speed:
//! - `alone`: the passage alone, 10, 20 and 30 s (no longer than the
//!   passage);
//! - `looped`: the passage looped to 60, 120 and 240 s;
//! - `beyond`: from the passage's start, 10, 20 and 30 s beyond its end,
//!   into the played file's own material;
//! - `before`: 5, 10 and 20 s of the played file's own material before
//!   the passage, then up to 30 s of the passage (only where the file has
//!   that much before it);
//! - `weak`: the 30 s `alone` and the 20 s `beyond` queries through a
//!   dull, low-bitrate transfer, a weak rip of the played recording;
//! - `blend`: the 20 s `beyond` query with the plan's partner mixed in at
//!   equal level over its last 20 s, masking the distinctive material
//!   (only when the plan names a partner);
//! - `break`: the passage's first 5 s repeated to 60 and 240 s.
//!
//! Each runs against three indexes: everything; the played recording left
//! out (its duplicate cluster and the plan's other files of it), the
//! related file kept; and, as a positive control, the played file alone
//! left out while another indexed rip of it stays. With a larger library
//! (`--other-peaks-dir`) every index also holds its records (`padding`),
//! and its files are rendered from its root. A query that cannot be
//! rendered is reported with its error and not searched.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::padding::{Padding, SECOND_LIBRARY_PREFIX};
use crate::render::{
    Encoding, Playback, RENDER_RATE, encode, limited, render_excerpt, render_samples, rms,
};
use crate::verifier::{Verification, Verifier};

const ALONE_SECONDS: [f64; 3] = [10.0, 20.0, 30.0];
const LOOPED_SECONDS: [f64; 3] = [60.0, 120.0, 240.0];
const BEYOND_SECONDS: [f64; 3] = [10.0, 20.0, 30.0];
const BEFORE_SECONDS: [f64; 3] = [5.0, 10.0, 20.0];
/// Of the passage, after a `before` query's context.
const BEFORE_PASSAGE_SECONDS: f64 = 30.0;
/// The `alone` query a weak rip repeats...
const WEAK_ALONE_SECONDS: f64 = 30.0;
/// ...and the context of the `beyond` query a weak rip and a blend repeat.
const CONTEXT_SECONDS: f64 = 20.0;
/// A dull transfer: rumble and the top cut, the bass lifted, then a low
/// bitrate.
const WEAK_RIP_FILTER: &str = "highpass=f=40,lowpass=f=3000,lowpass=f=3000,bass=g=6:f=100";
const WEAK_RIP_ENCODING: Encoding = Encoding::Mp3(64);
const BREAK_SECONDS: f64 = 5.0;
const BREAK_LOOPED_SECONDS: [f64; 2] = [60.0, 240.0];

/// The scenarios to run, as written by hand or by
/// `scripts/analysis/shared_panel.py`.
#[derive(Deserialize)]
pub struct Plan {
    /// Mixed into the `blend` queries; without one, none are made.
    #[serde(default)]
    pub blend_partner: Option<Partner>,
    pub scenarios: Vec<Scenario>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Partner {
    pub file: String,
    pub start_seconds: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub name: String,
    pub played: String,
    pub related: String,
    /// The shared passage in the played file, in seconds.
    pub passage_start_seconds: f64,
    pub passage_end_seconds: f64,
    /// Files of each recording beyond its duplicate cluster, such as rips
    /// indexed after the clusters were made.
    #[serde(default)]
    pub played_recording: Vec<String>,
    #[serde(default)]
    pub related_recording: Vec<String>,
}

#[derive(Serialize)]
pub struct SharedReport {
    pub blend_partner: Option<Partner>,
    pub scenarios: Vec<Scenario>,
    pub results: Vec<QueryResult>,
}

/// One query searched against one index.
#[derive(Serialize)]
pub struct QueryResult {
    pub scenario: String,
    /// `alone`, `looped`, `beyond`, `before`, `weak`, `blend` or `break`.
    pub kind: String,
    pub query: String,
    pub seconds: f64,
    /// Where the query starts in the played file.
    pub start_seconds: f64,
    /// `everything`, `played left out` or `source left out`.
    pub index: String,
    pub left_out: Vec<String>,
    /// Why the query could not be rendered; it was then not searched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// Every detection with two windows or more, strongest first.
    pub detections: Vec<SharedFound>,
}

#[derive(Serialize)]
pub struct SharedFound {
    pub asset: String,
    /// `played` (the played recording's files), `related` (the related
    /// recording's), `partner` (the blend partner's cluster) or `other`.
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
#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Alone,
    Looped,
    Beyond,
    Before,
    Weak,
    Blend,
    Break,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Alone => "alone",
            Kind::Looped => "looped",
            Kind::Beyond => "beyond",
            Kind::Before => "before",
            Kind::Weak => "weak",
            Kind::Blend => "blend",
            Kind::Break => "break",
        }
    }
}

/// A query to render: where in the played file, how long, looped or not.
struct Query {
    kind: Kind,
    name: String,
    start_seconds: f64,
    /// Of the played file's audio; a looped query or a break repeats it.
    source_seconds: f64,
    seconds: f64,
}

/// Where the plan's files are: the library's root, and the larger
/// library's for `second-library/` paths.
pub struct Roots<'a> {
    pub library: &'a Path,
    pub second: Option<&'a Path>,
}

impl Roots<'_> {
    pub(crate) fn path(&self, file: &str) -> Result<PathBuf, String> {
        match file.strip_prefix(SECOND_LIBRARY_PREFIX) {
            None => Ok(self.library.join(file)),
            Some(path) => self.second.map(|root| root.join(path)).ok_or_else(|| {
                format!("{file} is in the other library, whose root is unknown (--other-peaks-dir)")
            }),
        }
    }
}

pub struct Options<'a> {
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub padding: &'a Padding,
    pub jobs: usize,
}

pub fn run(
    plan: &Plan,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    roots: &Roots,
    dir: &Path,
    options: &Options,
) -> Result<SharedReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let verifier = Verifier::new(&records, options.padding.second.as_ref());
    fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    let partner = plan
        .blend_partner
        .as_ref()
        .map(|partner| clusters.cluster_of(&partner.file))
        .unwrap_or_default();
    let indexed = |file: &str| {
        records
            .iter()
            .any(|record| record.header.source.path == file)
            || options
                .padding
                .second
                .as_ref()
                .is_some_and(|second| second.holds(file))
    };
    let mut results = Vec::new();
    for scenario in &plan.scenarios {
        let queries = queries(scenario, plan.blend_partner.is_some());
        let rendered = render_each(roots, scenario, &queries, plan.blend_partner.as_ref(), dir);
        let played = recording(clusters, &scenario.played, &scenario.played_recording);
        let related = recording(clusters, &scenario.related, &scenario.related_recording);
        for (index_name, left_out) in indexes(&scenario.played, &played, &indexed) {
            let index = options
                .matching
                .index(options.padding.index(&records, &profile, &left_out)?);
            let searched: Vec<Result<QueryResult, String>> =
                map_in_order(&rendered, options.jobs, |(query, error)| {
                    let result =
                        |error: Option<String>, detections: Vec<SharedFound>| QueryResult {
                            scenario: scenario.name.clone(),
                            kind: query.kind.name().to_owned(),
                            query: query.name.clone(),
                            seconds: query.seconds,
                            start_seconds: query.start_seconds,
                            index: index_name.to_owned(),
                            left_out: left_out.iter().cloned().collect(),
                            error,
                            detections,
                        };
                    if let Some(error) = error {
                        return Ok(result(Some(error.clone()), Vec::new()));
                    }
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
                    Ok(result(
                        None,
                        detections
                            .iter()
                            .map(|detection| {
                                let asset = &index.asset(detection.asset).path;
                                let role = if played.contains(asset) {
                                    "played"
                                } else if related.contains(asset) {
                                    "related"
                                } else if partner.contains(asset) {
                                    "partner"
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
                    ))
                });
            for result in searched {
                results.push(result?);
            }
        }
    }
    Ok(SharedReport {
        blend_partner: plan.blend_partner.clone(),
        scenarios: plan.scenarios.clone(),
        results,
    })
}

/// A recording's files: `file`'s duplicate cluster and `also`.
pub(crate) fn recording(clusters: &Clusters, file: &str, also: &[String]) -> BTreeSet<String> {
    let mut files = clusters.cluster_of(file);
    files.extend(also.iter().cloned());
    files
}

fn queries(scenario: &Scenario, with_partner: bool) -> Vec<Query> {
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
    for before in BEFORE_SECONDS.into_iter().filter(|&before| before <= start) {
        let seconds = before + passage.min(BEFORE_PASSAGE_SECONDS);
        queries.push(Query {
            kind: Kind::Before,
            name: format!("{}-before-{before:.0}s", scenario.name),
            start_seconds: start - before,
            source_seconds: seconds,
            seconds,
        });
    }
    let weak_alone = passage.min(WEAK_ALONE_SECONDS);
    queries.push(Query {
        kind: Kind::Weak,
        name: format!("{}-weak-alone-{weak_alone:.0}s", scenario.name),
        start_seconds: start,
        source_seconds: weak_alone,
        seconds: weak_alone,
    });
    let with_context = passage + CONTEXT_SECONDS;
    queries.push(Query {
        kind: Kind::Weak,
        name: format!("{}-weak-beyond-{CONTEXT_SECONDS:.0}s", scenario.name),
        start_seconds: start,
        source_seconds: with_context,
        seconds: with_context,
    });
    if with_partner {
        queries.push(Query {
            kind: Kind::Blend,
            name: format!("{}-blend-beyond-{CONTEXT_SECONDS:.0}s", scenario.name),
            start_seconds: start,
            source_seconds: with_context,
            seconds: with_context,
        });
    }
    let piece = passage.min(BREAK_SECONDS);
    for seconds in BREAK_LOOPED_SECONDS {
        queries.push(Query {
            kind: Kind::Break,
            name: format!("{}-break-{piece:.0}s-looped-{seconds:.0}s", scenario.name),
            start_seconds: start,
            source_seconds: piece,
            seconds,
        });
    }
    queries
}

/// Renders `query` from the played file unless an earlier run did.
fn render(
    roots: &Roots,
    scenario: &Scenario,
    query: &Query,
    partner: Option<&Partner>,
    dir: &Path,
) -> Result<(), String> {
    let path = dir.join(format!("{}.mp3", query.name));
    if crate::render::is_complete(&path) {
        return Ok(());
    }
    let source = roots.path(&scenario.played)?;
    let native = Playback::Turntable(1.0);
    match query.kind {
        Kind::Alone | Kind::Beyond | Kind::Before => render_excerpt(
            &source,
            query.start_seconds,
            query.seconds,
            1.0,
            Encoding::Mp3(128),
            &path,
        ),
        Kind::Looped | Kind::Break => {
            let piece = render_samples(
                &source,
                query.start_seconds,
                query.source_seconds,
                native,
                None,
            )?;
            encode(
                &looped(&piece, query.seconds / query.source_seconds),
                None,
                Encoding::Mp3(128),
                &path,
            )
        }
        Kind::Weak => {
            let samples = render_samples(
                &source,
                query.start_seconds,
                query.seconds,
                native,
                Some(WEAK_RIP_FILTER),
            )?;
            encode(&samples, None, WEAK_RIP_ENCODING, &path)
        }
        Kind::Blend => {
            let partner = partner.ok_or("a blend query needs the plan's blend_partner")?;
            let samples =
                render_samples(&source, query.start_seconds, query.seconds, native, None)?;
            let partner = render_samples(
                &roots.path(&partner.file)?,
                partner.start_seconds,
                CONTEXT_SECONDS,
                native,
                None,
            )?;
            let context_from = query.seconds - CONTEXT_SECONDS;
            let from = (context_from * f64::from(RENDER_RATE)) as usize;
            encode(
                &limited(blended(&samples, &partner, from)),
                None,
                Encoding::Mp3(128),
                &path,
            )
        }
    }
}

/// `piece` repeated end to end for `times` its length.
fn looped(piece: &[f32], times: f64) -> Vec<f32> {
    let length = (times * piece.len() as f64) as usize;
    piece.iter().copied().cycle().take(length).collect()
}

/// `partner` added to `samples` from sample `from` on, at the level (RMS)
/// of the samples it covers.
fn blended(samples: &[f32], partner: &[f32], from: usize) -> Vec<f32> {
    let covered = &samples[from.min(samples.len())..];
    let covered = &covered[..covered.len().min(partner.len())];
    let gain = rms(covered) / rms(partner).max(1e-12);
    let mut out = samples.to_vec();
    for (slot, &other) in out.iter_mut().skip(from).zip(partner) {
        *slot += (gain * f64::from(other)) as f32;
    }
    out
}

/// The indexes a played file runs against, by name, as the files left
/// out: `played` is its recording's files. The positive control needs
/// another rip of the played recording that the index holds: a copy a
/// corpus file stands for is not indexed.
pub(crate) fn indexes(
    played_file: &str,
    played: &BTreeSet<String>,
    indexed: &dyn Fn(&str) -> bool,
) -> Vec<(&'static str, BTreeSet<String>)> {
    let mut indexes = vec![
        ("everything", BTreeSet::new()),
        ("played left out", played.clone()),
    ];
    if played
        .iter()
        .any(|file| file != played_file && indexed(file))
    {
        indexes.push(("source left out", BTreeSet::from([played_file.to_owned()])));
    }
    indexes
}

/// Renders each query, or says why it could not be rendered.
fn render_each<'q>(
    roots: &Roots,
    scenario: &Scenario,
    queries: &'q [Query],
    partner: Option<&Partner>,
    dir: &Path,
) -> Vec<(&'q Query, Option<String>)> {
    queries
        .iter()
        .map(|query| {
            let error = render(roots, scenario, query, partner, dir).err();
            if let Some(error) = &error {
                eprintln!("{}: not rendered: {error}", query.name);
            }
            (query, error)
        })
        .collect()
}

/// A digest of what the plan's queries sound like besides the source files:
/// the plan and the settings above. Rendered queries are kept in a
/// directory named with it, so a changed plan or setting renders afresh.
pub fn render_key(plan: &Plan) -> String {
    let settings = format!(
        "{ALONE_SECONDS:?} {LOOPED_SECONDS:?} {BEYOND_SECONDS:?} {BEFORE_SECONDS:?} \
         {BEFORE_PASSAGE_SECONDS} {WEAK_ALONE_SECONDS} {CONTEXT_SECONDS} {WEAK_RIP_FILTER} \
         {WEAK_RIP_ENCODING:?} {BREAK_SECONDS} {BREAK_LOOPED_SECONDS:?} {RENDER_RATE}"
    );
    let plan = serde_json::to_vec(&(&plan.blend_partner, &plan.scenarios)).unwrap_or_default();
    format!("{:016x}", fnv1a(&[settings.as_bytes(), &plan].concat()))
}

/// 64-bit FNV-1a.
fn fnv1a(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |hash, &byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0000_0100_0000_01b3)
    })
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
            played_recording: Vec::new(),
            related_recording: Vec::new(),
        }
    }

    fn of_kind(queries: &[Query], kind: Kind) -> Vec<(f64, f64, f64)> {
        queries
            .iter()
            .filter(|query| query.kind == kind)
            .map(|query| (query.start_seconds, query.source_seconds, query.seconds))
            .collect()
    }

    #[test]
    fn queries_alone_are_no_longer_than_the_passage() {
        let queries = queries(&scenario(18.0), false);

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
    }

    #[test]
    fn context_before_the_passage_is_the_played_files_own_material() {
        let queries = queries(&scenario(45.0), false);

        assert_eq!(
            of_kind(&queries, Kind::Before),
            [(95.0, 35.0, 35.0), (90.0, 40.0, 40.0), (80.0, 50.0, 50.0)]
        );
    }

    #[test]
    fn context_before_needs_that_much_of_the_file_before_the_passage() {
        let mut early = scenario(18.0);
        early.passage_start_seconds = 8.0;
        early.passage_end_seconds = 26.0;

        let queries = queries(&early, false);

        assert_eq!(of_kind(&queries, Kind::Before), [(3.0, 23.0, 23.0)]);
    }

    #[test]
    fn a_weak_rip_repeats_the_longest_alone_and_the_20_s_beyond_queries() {
        let long = queries(&scenario(45.0), false);
        let short = queries(&scenario(18.0), false);

        assert_eq!(
            of_kind(&long, Kind::Weak),
            [(100.0, 30.0, 30.0), (100.0, 65.0, 65.0)]
        );
        assert_eq!(
            of_kind(&short, Kind::Weak),
            [(100.0, 18.0, 18.0), (100.0, 38.0, 38.0)]
        );
    }

    #[test]
    fn a_blend_is_made_only_with_a_partner() {
        assert!(of_kind(&queries(&scenario(18.0), false), Kind::Blend).is_empty());
        assert_eq!(
            of_kind(&queries(&scenario(18.0), true), Kind::Blend),
            [(100.0, 38.0, 38.0)]
        );
    }

    #[test]
    fn a_break_repeats_the_passages_first_seconds() {
        let queries = queries(&scenario(18.0), false);

        assert_eq!(
            of_kind(&queries, Kind::Break),
            [(100.0, 5.0, 60.0), (100.0, 5.0, 240.0)]
        );
    }

    #[test]
    fn query_names_are_distinct() {
        let queries = queries(&scenario(45.0), true);
        let names: BTreeSet<&str> = queries.iter().map(|query| query.name.as_str()).collect();

        assert_eq!(names.len(), queries.len());
        assert_eq!(queries.len(), 3 + 3 + 3 + 3 + 2 + 1 + 2);
    }

    #[test]
    fn a_piece_loops_end_to_end() {
        assert_eq!(looped(&[1.0, 2.0], 2.5), [1.0, 2.0, 1.0, 2.0, 1.0]);
    }

    #[test]
    fn the_partner_plays_at_equal_level_over_the_context_only() {
        let samples = [0.5_f32; 10];
        let partner = [2.0_f32; 4];

        let out = blended(&samples, &partner, 6);

        assert_eq!(out[..6], [0.5; 6]);
        assert_eq!(out[6..], [1.0; 4]);
    }

    #[test]
    fn files_of_the_other_library_are_found_under_its_root() {
        let roots = Roots {
            library: Path::new("/corpus/library"),
            second: Some(Path::new("/nas/dnb")),
        };
        let corpus_only = Roots {
            library: Path::new("/corpus/library"),
            second: None,
        };

        assert_eq!(
            roots.path("extra/a.mp3"),
            Ok(PathBuf::from("/corpus/library/extra/a.mp3"))
        );
        assert_eq!(
            roots.path("second-library/x/b.m4a"),
            Ok(PathBuf::from("/nas/dnb/x/b.m4a"))
        );
        assert!(corpus_only.path("second-library/x/b.m4a").is_err());
    }

    #[test]
    fn a_recording_holds_its_cluster_and_the_plans_other_files() {
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("a-rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        let files = recording(&clusters, "a.mp3", &[String::from("a-upload.m4a")]);

        assert_eq!(
            files,
            BTreeSet::from([
                String::from("a.mp3"),
                String::from("a-rip.mp3"),
                String::from("a-upload.m4a")
            ])
        );
        assert_eq!(indexes("a.mp3", &files, &|_| true)[1].1, files);
    }

    #[test]
    fn the_positive_control_needs_another_rip() {
        let alone = BTreeSet::from([String::from("a.mp3")]);
        let ripped = BTreeSet::from([String::from("a.mp3"), String::from("a-rip.mp3")]);

        assert_eq!(indexes("a.mp3", &alone, &|_| true).len(), 2);
        assert_eq!(indexes("a.mp3", &ripped, &|_| true).len(), 3);
    }

    #[test]
    fn a_rip_the_index_does_not_hold_is_no_positive_control() {
        let ripped = BTreeSet::from([String::from("a.mp3"), String::from("a-rip.mp3")]);
        let only_the_played_file = |file: &str| file == "a.mp3";

        assert_eq!(indexes("a.mp3", &ripped, &only_the_played_file).len(), 2);
    }

    #[test]
    fn a_query_that_cannot_be_rendered_is_reported() {
        let dir = std::env::temp_dir().join(format!("gunfinger-shared-{}", std::process::id()));
        let roots = Roots {
            library: &dir,
            second: None,
        };
        let scenario = scenario(60.0);
        let queries = queries(&scenario, false);

        let rendered = render_each(&roots, &scenario, &queries, None, &dir);

        assert_eq!(rendered.len(), queries.len());
        assert!(rendered.iter().all(|(_, error)| error.is_some()));
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn the_render_key_follows_the_plan() {
        let plan = |end: f64| Plan {
            blend_partner: None,
            scenarios: vec![Scenario {
                passage_end_seconds: end,
                ..scenario(60.0)
            }],
        };

        assert_eq!(render_key(&plan(160.0)), render_key(&plan(160.0)));
        assert_ne!(render_key(&plan(160.0)), render_key(&plan(161.0)));
    }
}

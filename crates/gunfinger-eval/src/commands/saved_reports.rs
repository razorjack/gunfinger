//! Commands that read saved reports again and search nothing new:
//! `seconds`, `gaps` and `self-match`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::gaps::{Estimate, estimate, length_range};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::repeats::{self, RECORD_HITS, Repeats};
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakRecord;
use serde::Serialize;

use crate::clusters::Clusters;
use crate::files::{read_json, write_json};
use crate::harness::Harness;
use crate::manifest::Referable;
use crate::matching::Matching;
use crate::padding::SecondLibrary;
use crate::{gaps, manifest, mixes, scan, seconds, shared_mixes};

#[derive(clap::Args)]
pub struct SecondsArgs {
    /// Reports written by `mixes`, `shared-mixes` (each rendered mix
    /// under each index) or `scan` (a scan of the larger library with
    /// --other-peaks-dir).
    #[arg(required = true)]
    pub reports: Vec<PathBuf>,
    /// Seconds either side of each manifest border in which both
    /// neighbouring tracks are right and an unlabelled second is
    /// counted apart. Scans only: generated and rendered mixes have
    /// exact truth.
    #[arg(long, default_value_t = 30.0)]
    pub margin: f64,
    /// Also score with this minimum span from the first to the last
    /// aligned hit in place of 3 windows, as `grid` compares.
    #[arg(long)]
    pub min_span: Vec<f64>,
    /// For scans, also where each track's labels start and end against
    /// the manifest's approximate starts.
    #[arg(long)]
    pub boundaries: bool,
}

#[derive(clap::Args)]
pub struct GapsArgs {
    #[arg(required = true)]
    pub reports: Vec<PathBuf>,
    /// Seconds either side of each manifest border that count as a
    /// transition. Scans only: generated and rendered mixes have exact
    /// truth.
    #[arg(long, default_value_t = 30.0)]
    pub margin: f64,
    #[arg(long, value_delimiter = ',', default_values_t = gaps::MIN_LENGTHS)]
    pub min_length: Vec<f64>,
}

#[derive(clap::Args)]
pub struct SelfMatchArgs {
    #[arg(required = true)]
    pub reports: Vec<PathBuf>,
    #[arg(long, default_value_t = 30.0)]
    pub margin: f64,
    #[arg(long, default_value_t = 30.0)]
    pub min_length: f64,
    /// Also count the blocks with repeats played at these speed ratios
    /// to their first hearing (`blocks_at_ratios`), for pitch rides.
    #[arg(long, value_delimiter = ',')]
    pub ratios: Vec<f64>,
    /// Also search each gap against the records of its neighbours, the
    /// confident plays that end where it starts or start where it ends
    /// (`neighbours`), so that blocks of a found track's own material
    /// can be told apart.
    #[arg(long)]
    pub neighbours: bool,
}

/// `seconds`: each report scored under today's rule and each minimum span.
pub fn seconds(harness: &Harness, args: SecondsArgs) -> Result<(), String> {
    let SecondsArgs {
        reports,
        margin,
        min_span: min_spans,
        boundaries,
    } = args;
    let rules: Vec<seconds::Rule> = std::iter::once(seconds::Rule::Windows)
        .chain(min_spans.iter().map(|&span| seconds::Rule::MinSpan(span)))
        .collect();
    let clusters = harness.clusters()?;
    for (path, kind, judged) in judged_reports(harness, &reports, &clusters, margin)? {
        let scored: Vec<seconds::Scored> = judged
            .iter()
            .flat_map(|mix| {
                rules.iter().map(move |&rule| {
                    let mut score = seconds::score(
                        &mix.truth,
                        &mix.segments,
                        mix.duration_seconds,
                        mix.margin,
                        rule,
                    );
                    if kind == ReportKind::Scan && !boundaries {
                        score.boundaries.clear();
                    }
                    seconds::Scored {
                        name: mix.name.clone(),
                        rule: rule.name(),
                        score,
                    }
                })
            })
            .collect();
        write_json(
            &harness.seconds_dir().join(judged_name(&path, kind, margin)),
            &scored,
        )?;
        if kind == ReportKind::Rendered {
            let mut indexes: Vec<&str> = Vec::new();
            for index in judged.iter().filter_map(|mix| mix.index.as_deref()) {
                if !indexes.contains(&index) {
                    indexes.push(index);
                }
            }
            for index in indexes {
                let prefix = format!("{index}: ");
                let here: Vec<&seconds::Scored> = scored
                    .iter()
                    .filter(|scored| scored.name.starts_with(&prefix))
                    .collect();
                seconds::print_summary(&format!("{}, {index}", path.display()), &here);
            }
        } else {
            seconds::print_summary(
                &path.display().to_string(),
                &scored.iter().collect::<Vec<_>>(),
            );
        }
    }
    Ok(())
}

/// `gaps`: each report's gaps at each minimum length, and a summary of
/// all of them.
pub fn gaps(harness: &Harness, args: GapsArgs) -> Result<(), String> {
    let GapsArgs {
        reports,
        margin,
        min_length: min_lengths,
    } = args;
    let clusters = harness.clusters()?;
    let mut every: Vec<gaps::Listing> = Vec::new();
    for (path, kind, judged) in judged_reports(harness, &reports, &clusters, margin)? {
        let listings: Vec<gaps::Listing> = judged
            .iter()
            .map(|mix| {
                gaps::list(
                    &mix.name,
                    &mix.truth,
                    &mix.segments,
                    mix.duration_seconds,
                    mix.margin,
                    &min_lengths,
                )
            })
            .collect();
        write_json(
            &harness.gaps_dir().join(judged_name(&path, kind, margin)),
            &listings,
        )?;
        gaps::print_summary(
            &path.display().to_string(),
            &listings.iter().collect::<Vec<_>>(),
            &min_lengths,
        );
        every.extend(listings);
    }
    if reports.len() > 1 {
        gaps::print_summary(
            &format!("all {} reports", reports.len()),
            &every.iter().collect::<Vec<_>>(),
            &min_lengths,
        );
    }
    Ok(())
}

pub fn self_match(harness: &Harness, jobs: usize, args: SelfMatchArgs) -> Result<(), String> {
    let SelfMatchArgs {
        reports,
        margin,
        min_length,
        ratios,
        neighbours,
    } = args;
    match_gaps_against_themselves(
        harness,
        &reports,
        &SelfMatchOptions {
            margin,
            min_length,
            ratios: &ratios,
            neighbours,
        },
        jobs,
    )
}

/// The kinds of saved report `seconds` and `gaps` read.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ReportKind {
    Mixes,
    Scan,
    /// `shared-mixes`: rendered mixes, each under several indexes.
    Rendered,
}

/// A mix of a saved report with its truth.
struct Judged {
    /// The mix's or the set's name; for a rendered mix, `<index>: <mix>`.
    name: String,
    /// The index a rendered mix was searched with.
    index: Option<String>,
    truth: Vec<seconds::Truth>,
    segments: Vec<seconds::Segment>,
    duration_seconds: f64,
    /// `margin` for a scan, whose borders are approximate; 0 for exact truth.
    margin: f64,
    /// The mix's audio, where the report names it: a set's, or a
    /// generated mix's render.
    audio: Option<PathBuf>,
}

/// Reads `reports`, each with the mixes it holds and their truth.
fn judged_reports(
    harness: &Harness,
    reports: &[PathBuf],
    clusters: &Clusters,
    margin: f64,
) -> Result<Vec<(PathBuf, ReportKind, Vec<Judged>)>, String> {
    let mut loaded = Vec::new();
    for path in reports {
        let report: serde_json::Value = read_json(path)?;
        loaded.push((path, report));
    }
    let referable = if loaded.iter().any(|(_, report)| report.get("set").is_some()) {
        Some(harness.referable(&harness.library()?)?)
    } else {
        None
    };
    loaded
        .into_iter()
        .map(|(path, report)| {
            let (kind, judged) =
                judged(harness, path, report, clusters, referable.as_ref(), margin)?;
            Ok((path.clone(), kind, judged))
        })
        .collect()
}

fn judged(
    harness: &Harness,
    path: &Path,
    report: serde_json::Value,
    clusters: &Clusters,
    referable: Option<&Referable>,
    margin: f64,
) -> Result<(ReportKind, Vec<Judged>), String> {
    let parse = |error: serde_json::Error| format!("{}: {error}", path.display());
    if report.get("runs").is_some() {
        let report: shared_mixes::Report = serde_json::from_value(report).map_err(parse)?;
        let judged = report
            .runs
            .iter()
            .filter_map(|run| {
                // Runs on the development mixes have no planned truth.
                let rendered = report
                    .mixes
                    .iter()
                    .find(|rendered| rendered.mix.name == run.name)?;
                let left_out: BTreeSet<String> = run.left_out.iter().cloned().collect();
                Some(Judged {
                    name: format!("{}: {}", run.index, run.name),
                    index: Some(run.index.clone()),
                    truth: seconds::rendered_truth(&rendered.mix, clusters, &left_out),
                    segments: run
                        .detections
                        .iter()
                        .map(|measured| seconds::Segment::from(&measured.found))
                        .collect(),
                    duration_seconds: rendered.mix.seconds,
                    margin: 0.0,
                    audio: None,
                })
            })
            .collect();
        return Ok((ReportKind::Rendered, judged));
    }
    match (report.get("mixes").is_some(), referable) {
        (true, _) => {
            let report: mixes::MixesReport = serde_json::from_value(report).map_err(parse)?;
            let judged = report
                .mixes
                .iter()
                .map(|result| Judged {
                    name: result.mix.name.clone(),
                    index: None,
                    truth: seconds::mix_truth(&result.mix, clusters),
                    segments: result
                        .detections
                        .iter()
                        .map(seconds::Segment::from)
                        .collect(),
                    duration_seconds: result.mix.seconds,
                    margin: 0.0,
                    audio: Some(
                        report
                            .kind
                            .renders(&harness.paths.work, report.seed, report.label().as_deref())
                            .join(format!("{}.mp3", result.mix.name)),
                    ),
                })
                .collect();
            Ok((ReportKind::Mixes, judged))
        }
        (false, Some(referable)) => {
            let report: scan::ScanReport = serde_json::from_value(report).map_err(parse)?;
            let set = manifest::load_set(&harness.sets(), &report.set, referable)
                .map_err(|problems| format!("{}: {}", report.set, problems.join("; ")))?;
            let left_out: BTreeSet<String> = report.left_out_assets.iter().cloned().collect();
            Ok((
                ReportKind::Scan,
                vec![Judged {
                    name: report.set.clone(),
                    index: None,
                    truth: seconds::set_truth(&set, report.duration_seconds, clusters, &left_out),
                    segments: report
                        .detections
                        .iter()
                        .map(seconds::Segment::from)
                        .collect(),
                    duration_seconds: report.duration_seconds,
                    margin,
                    audio: Some(set.audio.clone()),
                }],
            ))
        }
        (false, None) => Err(format!(
            "{} is neither a mixes nor a scan report",
            path.display()
        )),
    }
}

/// The name a report's scores or gaps are written under: its stem, and
/// for a scan the margin.
fn judged_name(path: &Path, kind: ReportKind, margin: f64) -> String {
    let stem = path
        .file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default();
    match kind {
        ReportKind::Scan => format!("{stem}-margin-{margin:.0}.json"),
        ReportKind::Mixes | ReportKind::Rendered => format!("{stem}.json"),
    }
}

/// One gap matched against itself, beside its truth.
#[derive(Serialize)]
struct SelfMatched {
    mix: String,
    start_seconds: f64,
    end_seconds: f64,
    missed_tracks: usize,
    indexed_tracks: usize,
    estimate: Option<u32>,
    blocks: Vec<BlocksAt>,
    /// With `--ratios`: the blocks with repeats at those ratios too.
    #[serde(skip_serializing_if = "Option::is_none")]
    blocks_at_ratios: Option<Vec<BlocksAt>>,
    /// The product's estimate (ADR 0024) from this gap's repeats at speed
    /// 1.0 and the listing's clean intervals, by the code `identify` uses.
    tracks: Option<TracksAt>,
    /// With `--neighbours`: the gap searched against its neighbours'
    /// records, in seconds from the gap's start.
    #[serde(skip_serializing_if = "Option::is_none")]
    neighbours: Option<Vec<NeighbourMatch>>,
}

/// The thresholds compared, in hits per cell and lag.
const MIN_HITS: [u32; 4] = [5, 10, 20, 40];

/// A gap's blocks at one threshold (`repeats::Blocks`).
#[derive(Serialize)]
struct BlocksAt {
    min_hits: u32,
    /// Each block's first and last cell, in seconds from the gap's start.
    spans: Vec<(f64, f64)>,
    /// Cells in a block, of all cells.
    cells_in_blocks: usize,
    cells: usize,
}

impl BlocksAt {
    fn each(repeats: &Repeats) -> Vec<BlocksAt> {
        MIN_HITS
            .iter()
            .map(|&min_hits| {
                let blocks = repeats.blocks(min_hits);
                BlocksAt {
                    min_hits: blocks.min_hits,
                    spans: blocks.spans,
                    cells_in_blocks: blocks.cells_in_blocks,
                    cells: blocks.cells,
                }
            })
            .collect()
    }
}

/// The product's estimate of a gap, as its JSON report writes it.
#[derive(Serialize)]
struct TracksAt {
    low: u32,
    high: u32,
    method: &'static str,
}

impl From<Estimate> for TracksAt {
    fn from(estimate: Estimate) -> TracksAt {
        TracksAt {
            low: estimate.low,
            high: estimate.high,
            method: estimate.method.name(),
        }
    }
}

/// A detection of a neighbour's record inside a gap.
#[derive(Serialize)]
struct NeighbourMatch {
    asset: String,
    start_seconds: f64,
    end_seconds: f64,
    windows: u32,
    hits: u32,
}

struct SelfMatchOptions<'a> {
    margin: f64,
    min_length: f64,
    ratios: &'a [f64],
    neighbours: bool,
}

/// A gap to match: the mix's audio and name, the gap, its neighbours'
/// files and the mix's clean intervals.
type ListedGap<'a> = (&'a Path, String, gaps::Gap, Vec<String>, Vec<f64>);

/// `self-match`: every listed gap of each report, matched against itself,
/// one gap per worker.
fn match_gaps_against_themselves(
    harness: &Harness,
    reports: &[PathBuf],
    options: &SelfMatchOptions,
    jobs: usize,
) -> Result<(), String> {
    let clusters = harness.clusters()?;
    let profile = Profile::CURRENT;
    let min_length = options.min_length;
    let records = if options.neighbours {
        Some(NeighbourRecords::open(harness)?)
    } else {
        None
    };
    let ladder = harness.rungs();
    for (path, kind, judged) in judged_reports(harness, reports, &clusters, options.margin)? {
        let mut gaps: Vec<ListedGap> = Vec::new();
        for mix in &judged {
            let Some(audio) = &mix.audio else { continue };
            let listing = gaps::list(
                &mix.name,
                &mix.truth,
                &mix.segments,
                mix.duration_seconds,
                mix.margin,
                &[min_length],
            );
            for at in listing.at {
                for gap in at.gaps {
                    let neighbours = gaps::neighbours(&mix.segments, &gap);
                    gaps.push((
                        audio,
                        mix.name.clone(),
                        gap,
                        neighbours,
                        at.intervals.clone(),
                    ));
                }
            }
        }
        let matched: Vec<SelfMatched> =
            map_in_order(&gaps, jobs, |(audio, mix, gap, neighbours, intervals)| {
                let excerpt = Excerpt {
                    start: Some(Duration::from_secs_f64(gap.start_seconds)),
                    duration: Some(Duration::from_secs_f64(gap.length_seconds())),
                };
                let samples = decode(audio, profile.sample_rate, excerpt)
                    .map_err(|error| format!("{}: {error}", audio.display()))?
                    .samples;
                let neighbours = match &records {
                    Some(records) => Some(records.search(
                        neighbours,
                        &samples,
                        &harness.search.matching,
                        &ladder,
                    )?),
                    None => None,
                };
                let at_one = Repeats::find(&samples, &profile, &[]);
                let records = repeats::stretches(&at_one.blocks(RECORD_HITS)).len();
                Ok(SelfMatched {
                    mix: mix.clone(),
                    start_seconds: gap.start_seconds,
                    end_seconds: gap.end_seconds,
                    missed_tracks: gap.missed_tracks.len(),
                    indexed_tracks: gap.indexed_tracks.len(),
                    estimate: gap.estimate,
                    blocks: BlocksAt::each(&at_one),
                    blocks_at_ratios: (!options.ratios.is_empty()).then(|| {
                        BlocksAt::each(&Repeats::find(&samples, &profile, options.ratios))
                    }),
                    tracks: estimate(records, length_range(gap.length_seconds(), intervals))
                        .map(TracksAt::from),
                    neighbours,
                })
            })
            .into_iter()
            .collect::<Result<_, String>>()?;
        let name = format!("self-match-{}", judged_name(&path, kind, options.margin));
        write_json(&harness.gaps_dir().join(name), &matched)?;
        println!(
            "{}: {} gaps of at least {min_length:.0} s",
            path.display(),
            matched.len()
        );
        for gap in &matched {
            let counts: Vec<String> = gap
                .blocks
                .iter()
                .map(|blocks| format!("{}@{}", blocks.spans.len(), blocks.min_hits))
                .collect();
            println!(
                "  {} {:.0}-{:.0} s: {} missed, estimate {}, blocks {}",
                gap.mix,
                gap.start_seconds,
                gap.end_seconds,
                gap.missed_tracks,
                gap.estimate
                    .map_or_else(|| String::from("-"), |n| n.to_string()),
                counts.join(" ")
            );
        }
    }
    Ok(())
}

/// The peak records a gap's neighbours are searched with: the corpus
/// library's, and with --other-peaks-dir the other store's.
struct NeighbourRecords {
    corpus: BTreeMap<String, PeakRecord>,
    other: Option<SecondLibrary>,
}

impl NeighbourRecords {
    fn open(harness: &Harness) -> Result<NeighbourRecords, String> {
        let (records, _) = load_records(
            &harness.library()?,
            &harness.store()?,
            &Profile::CURRENT,
            &BTreeSet::new(),
        );
        let other = match harness.other_store()? {
            Some(store) => Some(SecondLibrary::from_store(store, &BTreeSet::new())?),
            None => None,
        };
        Ok(NeighbourRecords {
            corpus: records
                .into_iter()
                .map(|record| (record.header.source.path.clone(), record))
                .collect(),
            other,
        })
    }

    fn record(&self, asset: &str) -> Result<PeakRecord, String> {
        if let Some(record) = self.corpus.get(asset) {
            return Ok(record.clone());
        }
        self.other
            .as_ref()
            .and_then(|other| other.record(asset))
            .unwrap_or_else(|| Err(format!("no peak record of {asset}")))
            .map(|mut record| {
                record.header.source.path = asset.to_owned();
                record
            })
    }

    /// `samples` searched against an index of the `assets`' records alone,
    /// every detection kept.
    fn search(
        &self,
        assets: &[String],
        samples: &[f32],
        matching: &Matching,
        ladder: &[Rung],
    ) -> Result<Vec<NeighbourMatch>, String> {
        if assets.is_empty() {
            return Ok(Vec::new());
        }
        let records: Vec<PeakRecord> = assets
            .iter()
            .map(|asset| self.record(asset))
            .collect::<Result<_, _>>()?;
        let index = matching.index(Index::build(&records).map_err(|error| error.to_string())?);
        Ok(matching
            .search(&index, samples, &Profile::CURRENT, ladder, 1)
            .iter()
            .map(|detection| NeighbourMatch {
                asset: index.asset(detection.asset).path.clone(),
                start_seconds: detection.start_seconds,
                end_seconds: detection.end_seconds,
                windows: detection.evidence.windows,
                hits: detection.evidence.hits,
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gunfinger_core::gaps::Method;

    #[test]
    fn self_match_lists_the_blocks_at_each_threshold_in_order() {
        let profile = Profile::CURRENT;
        let silence = vec![0.0_f32; 40 * profile.sample_rate as usize];

        let blocks = BlocksAt::each(&Repeats::find(&silence, &profile, &[]));

        let thresholds: Vec<u32> = blocks.iter().map(|blocks| blocks.min_hits).collect();
        assert_eq!(thresholds, MIN_HITS);
        assert!(
            blocks
                .iter()
                .all(|blocks| blocks.spans.is_empty() && blocks.cells == 8)
        );
    }

    #[test]
    fn self_match_writes_the_estimate_as_a_report_does() {
        let tracks = TracksAt::from(Estimate {
            low: 1,
            high: 3,
            method: Method::RepeatsWithinLength,
        });

        assert_eq!(
            serde_json::to_value(&tracks).ok(),
            Some(serde_json::json!({ "low": 1, "high": 3, "method": "repeats-within-length" }))
        );
    }
}

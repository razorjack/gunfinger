//! Duplicate clusters of every file of an index (session 86), for a
//! library too large to search each whole file against: the other
//! library's tens of thousands of files.
//!
//! The criterion is the clusters' (`MIN_COVERAGE` of the shorter file
//! by one alignment, the verdicts by ear overriding it); only the
//! candidates come from a cheaper search. Each file's middle
//! `EXCERPT_SECONDS` of stored peaks are searched against the whole
//! index on the turntable ladder, and the strongest other files found
//! are measured as `find_around` measures a candidate: the whole file
//! against the found file alone at the candidate's fitted speed, and,
//! when that covers between `REPORTED_COVERAGE` and `MIN_COVERAGE`,
//! on the whole ladder too, whose chains follow a rip whose speed
//! drifts (experiment 0049). An alignment covering 80% of the shorter
//! file of a pair holds its middle (for files of 33 s or more), so the
//! excerpt of the shorter file lies inside every alignment the
//! criterion accepts.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use gunfinger_core::index::{AssetId, Index};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, SKIPPED_SHARE, search_peaks};
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakRecord;
use serde::Serialize;

use super::{
    Clusters, Contradictions, MIN_COVERAGE, MIN_HITS_TO_MEASURE, Pair, Query, REPORTED_COVERAGE,
    Verdicts, at_fitted_speed, better_of, clustering_ladder, clusters_of, cut, cut_note, pair,
};
use crate::shared_mixes::passage;

/// The length of each file's excerpt that finds its candidates: long
/// enough for a rip to stand far above chance (session 74's passages).
pub const EXCERPT_SECONDS: f64 = 20.0;
/// The strongest other files found by a file's excerpt that are measured.
pub const MEASURED_PER_QUERY: usize = 20;

/// What the search did, beside the pairs it kept.
#[derive(Debug, Default, Serialize)]
pub struct Counts {
    pub queries: usize,
    pub queries_without_record: usize,
    pub candidates_measured: usize,
    /// Candidates also measured on the whole ladder against the found file.
    pub measured_on_ladder: usize,
    /// Candidates whose pair covers less than `REPORTED_COVERAGE`:
    /// counted, not kept.
    pub below_reported: usize,
    pub search_seconds: f64,
}

impl Counts {
    fn add(&mut self, other: &Counts) {
        self.candidates_measured += other.candidates_measured;
        self.measured_on_ladder += other.measured_on_ladder;
        self.below_reported += other.below_reported;
    }
}

/// The pairs of every file of `index` with the files its excerpt finds,
/// those covering `REPORTED_COVERAGE` or more, strongest first. `index`
/// is searched as it is (the caller sets its fullest lists aside);
/// `record` gives a file's stored peaks by the index's name for it.
pub fn pairs<'a, F>(index: &Index, record: F, jobs: usize) -> (Vec<Pair>, Counts)
where
    F: Fn(&str) -> Result<Cow<'a, PeakRecord>, String> + Sync,
{
    let started = Instant::now();
    let ladder = clustering_ladder();
    let ids: Vec<AssetId> = (0..index.assets().len())
        .filter_map(|id| u32::try_from(id).ok().map(AssetId))
        .collect();
    let finished = AtomicUsize::new(0);
    let per_query: Vec<Option<(Vec<Pair>, Counts)>> = map_in_order(&ids, jobs, |&id| {
        let query = &index.asset(id).path;
        let measured = match record(query) {
            Ok(query_record) => Some(query_pairs(index, &query_record, id, &record, &ladder)),
            Err(error) => {
                eprintln!("not searched: {query}: {error}");
                None
            }
        };
        let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
        if count.is_multiple_of(500) {
            eprintln!(
                "[{count}/{}] {:.0} s",
                ids.len(),
                started.elapsed().as_secs_f64()
            );
        }
        measured
    });
    let mut pairs = Vec::new();
    let mut counts = Counts::default();
    for measured in per_query {
        counts.queries += 1;
        let Some((found, query_counts)) = measured else {
            counts.queries_without_record += 1;
            continue;
        };
        counts.add(&query_counts);
        pairs.extend(found);
    }
    counts.search_seconds = started.elapsed().as_secs_f64();
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));
    (pairs, counts)
}

/// One file's pairs: its excerpt's candidates, each measured with the
/// whole file.
fn query_pairs<'a, F>(
    index: &Index,
    query: &PeakRecord,
    id: AssetId,
    record: &F,
    ladder: &[Rung],
) -> (Vec<Pair>, Counts)
where
    F: Fn(&str) -> Result<Cow<'a, PeakRecord>, String> + Sync,
{
    let profile = Profile::CURRENT;
    let name = &query.header.source.path;
    let seconds = query.header.duration_seconds;
    let excerpt = middle(&query.peaks, seconds, &profile);
    let detections = search_peaks(index, &excerpt, &profile, ladder, 1);
    let mut counts = Counts::default();
    let mut pairs = Vec::new();
    for candidate in strongest_others(&detections, id) {
        let found = &index.asset(candidate.asset).path;
        let found_record = match record(found) {
            Ok(found_record) => found_record,
            Err(error) => {
                eprintln!("not measured: {name} ~ {found}: {error}");
                continue;
            }
        };
        counts.candidates_measured += 1;
        let lengths = (seconds, found_record.header.duration_seconds);
        let measured = |detection: &Detection| pair(name, found.clone(), lengths, detection);
        let mut best = at_fitted_speed(Query::Peaks(&query.peaks), &found_record, candidate)
            .map(|refined| measured(&refined));
        let coverage = best.as_ref().map_or(0.0, |pair| pair.coverage);
        if (REPORTED_COVERAGE..MIN_COVERAGE).contains(&coverage) {
            counts.measured_on_ladder += 1;
            if let Some(on_ladder) = on_ladder_alone(&query.peaks, &found_record, ladder)
                .iter()
                .map(measured)
                .max_by(|a, b| a.coverage.total_cmp(&b.coverage))
            {
                best = Some(match best {
                    Some(refined) => better_of(on_ladder, Some(refined)),
                    None => on_ladder,
                });
            }
        }
        match best {
            Some(pair) if pair.coverage >= REPORTED_COVERAGE => pairs.push(pair),
            _ => counts.below_reported += 1,
        }
    }
    (pairs, counts)
}

/// The middle `EXCERPT_SECONDS` of a file's peaks, or all of them when the
/// file is shorter.
fn middle(peaks: &[Peak], seconds: f64, profile: &Profile) -> Vec<Peak> {
    if seconds <= EXCERPT_SECONDS {
        return peaks.to_vec();
    }
    let start = (seconds - EXCERPT_SECONDS) / 2.0;
    passage(peaks, start, start + EXCERPT_SECONDS, profile)
}

/// The other files' strongest detections of `MIN_HITS_TO_MEASURE` hits or
/// more, one per file, the `MEASURED_PER_QUERY` strongest (ties by asset).
fn strongest_others(detections: &[Detection], query: AssetId) -> Vec<&Detection> {
    let mut strongest: BTreeMap<AssetId, &Detection> = BTreeMap::new();
    for detection in detections {
        if detection.asset == query || detection.evidence.hits < MIN_HITS_TO_MEASURE {
            continue;
        }
        strongest
            .entry(detection.asset)
            .and_modify(|kept| {
                if detection.evidence.hits > kept.evidence.hits {
                    *kept = detection;
                }
            })
            .or_insert(detection);
    }
    let mut ranked: Vec<&Detection> = strongest.into_values().collect();
    ranked.sort_by(|a, b| {
        b.evidence
            .hits
            .cmp(&a.evidence.hits)
            .then(a.asset.cmp(&b.asset))
    });
    ranked.truncate(MEASURED_PER_QUERY);
    ranked
}

/// The whole query against `found` alone on the ladder, every list.
fn on_ladder_alone(query: &[Peak], found: &PeakRecord, ladder: &[Rung]) -> Vec<Detection> {
    let Ok(index) = Index::build(std::slice::from_ref(found)) else {
        return Vec::new();
    };
    search_peaks(&index, query, &Profile::CURRENT, ladder, 1)
}

/// The clusters the pairs make, with the verdicts by ear applied.
pub fn clustered(
    mut pairs: Vec<Pair>,
    verdicts: &Verdicts,
    contradictions: Contradictions,
) -> Result<Clusters, String> {
    verdicts.apply(&mut pairs);
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));
    let (duplicates, cut_links) =
        clusters_of(verdicts.links(&pairs), verdicts, &pairs, contradictions)?;
    cut(&mut pairs, &cut_links);
    Ok(Clusters {
        criterion: format!(
            "{}; every file of the index a query, its candidates found by its middle {EXCERPT_SECONDS:.0} s of stored peaks (the {MEASURED_PER_QUERY} strongest other files of {MIN_HITS_TO_MEASURE} hits or more, the fullest {}% of posting lists set aside), each measured with the whole file; when the fitted speed covers {:.0}-{:.0}%, the whole ladder against the found file alone as well{}",
            super::criterion(),
            SKIPPED_SHARE * 100.0,
            REPORTED_COVERAGE * 100.0,
            MIN_COVERAGE * 100.0,
            cut_note(&cut_links)
        ),
        duplicates,
        pairs,
        cut_links,
    })
}

/// Refuses to write the library-wide clusters over the clusters in use:
/// they are a scratch result, written beside it.
pub fn check_out(out: &Path, in_use: &Path) -> Result<(), String> {
    let same_name = out.file_name().is_some() && out.file_name() == in_use.file_name();
    let same_file = match (out.canonicalize(), in_use.canonicalize()) {
        (Ok(out), Ok(in_use)) => out == in_use,
        _ => false,
    };
    if same_name || same_file {
        return Err(format!(
            "{} would replace or shadow the clusters in use ({}); write the library-wide clusters under another name",
            out.display(),
            in_use.display()
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::store::RecordHeader;

    use super::*;

    #[test]
    fn the_clusters_in_use_are_never_the_output() {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-library-wide-{}", std::process::id()));
        let reports = dir.join("reports");
        fs::create_dir_all(&reports).unwrap();
        let in_use = reports.join("duplicate-clusters.json");
        fs::write(&in_use, "{}").unwrap();

        assert!(check_out(&in_use, &in_use).is_err());
        assert!(check_out(&reports.join("./duplicate-clusters.json"), &in_use).is_err());
        assert!(check_out(&dir.join("elsewhere/duplicate-clusters.json"), &in_use).is_err());
        assert!(check_out(&dir.join("library-wide.json"), &in_use).is_ok());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_excerpt_is_the_middle_of_the_file() {
        let profile = Profile::CURRENT;
        let peaks = scattered(1, profile.frames(100.0));
        let excerpt = middle(&peaks, 100.0, &profile);
        let first = excerpt.first().unwrap().frame;
        let last = excerpt.last().unwrap().frame;
        assert!((profile.frames(40.0)..profile.frames(40.0) + 3.0).contains(&first));
        assert!((profile.frames(60.0) - 3.0..=profile.frames(60.0)).contains(&last));

        let short: Vec<Peak> = peaks.iter().take(5).copied().collect();
        assert_eq!(middle(&short, 15.0, &profile), short);
    }

    #[test]
    fn a_rip_joins_its_file_and_an_unrelated_file_does_not() {
        let profile = Profile::CURRENT;
        let frames = profile.frames(240.0);
        let records = [
            record("a.mp3", scattered(1, frames)),
            record("a-rip.mp3", scattered(1, frames)),
            record("b.mp3", scattered(2, frames)),
        ];
        let index = Index::build(&records).unwrap();
        let by_name = |name: &str| -> Result<Cow<'_, PeakRecord>, String> {
            records
                .iter()
                .find(|record| record.header.source.path == name)
                .map(Cow::Borrowed)
                .ok_or_else(|| format!("no record of {name}"))
        };

        let (pairs, counts) = pairs(&index, by_name, 2);

        assert_eq!(counts.queries, 3);
        let joined: Vec<(&str, &str)> = pairs
            .iter()
            .filter(|pair| pair.same_recording)
            .map(|pair| (pair.query.as_str(), pair.found.as_str()))
            .collect();
        assert_eq!(joined.len(), 2);
        assert!(joined.contains(&("a.mp3", "a-rip.mp3")));
        assert!(joined.contains(&("a-rip.mp3", "a.mp3")));
        assert!(
            pairs
                .iter()
                .all(|pair| pair.query != "b.mp3" && pair.found != "b.mp3")
        );

        let clusters = clustered(pairs, &Verdicts::default(), Contradictions::Stop).unwrap();
        assert_eq!(clusters.duplicates, [["a-rip.mp3", "a.mp3"]]);
    }

    fn record(path: &str, peaks: Vec<Peak>) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: Profile::CURRENT
                    .seconds(peaks.last().map_or(0.0, |peak| peak.frame)),
            },
            peaks,
        }
    }

    /// Peaks every 3 frames up to `frames`, at pseudo-random bins of `seed`.
    fn scattered(seed: u64, frames: f64) -> Vec<Peak> {
        let mut state = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
        let mut peaks = Vec::new();
        let mut frame = 0.0;
        while frame < frames {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            peaks.push(Peak {
                frame,
                bin: 5.0 + (state % 490) as f32,
                magnitude: 10.0,
            });
            frame += 3.0;
        }
        peaks
    }
}

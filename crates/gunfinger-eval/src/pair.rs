//! The evidence between two library files, searched alone: why a pair of
//! rips covers less of each other than the clustering rule needs. The
//! query is searched against an index of the found file only, by its
//! stored peaks on several ladders and at the fitted speed, and, for a
//! corpus file, by its decoded audio. Every alignment is listed, so a cut
//! or an edit shows as alignments at one speed with different offsets.

use std::collections::{BTreeMap, BTreeSet};

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::hash::{Point, for_each_pair};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Matcher, search, search_peaks, search_with};
use gunfinger_core::speed::{Playback, Rung, key_lock_ladder, ladder};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::Serialize;

use crate::padding::SECOND_LIBRARY_PREFIX;

#[derive(Debug, Serialize)]
pub struct PairReport {
    pub query: String,
    pub found: String,
    pub query_seconds: f64,
    pub found_seconds: f64,
    pub searches: Vec<Search>,
}

#[derive(Debug, Serialize)]
pub struct Search {
    pub name: String,
    /// The found file's alignments, strongest first.
    pub alignments: Vec<Alignment>,
}

#[derive(Debug, Serialize)]
pub struct Alignment {
    pub query_start_seconds: f64,
    pub query_end_seconds: f64,
    pub found_start_seconds: f64,
    pub found_end_seconds: f64,
    pub speed: f64,
    pub key_locked: bool,
    pub hits: u32,
    pub windows: u32,
    /// The span in the query over the shorter file's length.
    pub coverage: f64,
    /// Where query time 0 falls in the found file: alignments of one
    /// recording at one speed share it, unless the rip is cut or edited.
    pub offset_seconds: f64,
    /// For the search at the fitted speed: the seconds of the shorter file
    /// that hold a hit of the alignment, over its length. Coverage counts
    /// the span from the first hit to the last, gaps included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported: Option<f64>,
}

/// Where the files' records and audio come from.
pub struct Sources<'a> {
    pub library: &'a Library,
    pub store: &'a PeakStore,
    pub other: Option<&'a PeakStore>,
}

/// Each pair of `pairs` (query, found) searched alone.
pub fn run(sources: &Sources, pairs: &[(String, String)]) -> Result<Vec<PairReport>, String> {
    let records = Records::load(sources)?;
    pairs
        .iter()
        .map(|(query, found)| run_pair(sources, &records, query, found))
        .collect()
}

fn run_pair(
    sources: &Sources,
    records: &Records,
    query: &str,
    found: &str,
) -> Result<PairReport, String> {
    let profile = Profile::CURRENT;
    let query_record = records.record(sources, query)?;
    let found_record = records.record(sources, found)?;
    let index =
        Index::build(std::slice::from_ref(&found_record)).map_err(|error| error.to_string())?;
    let shorter = query_record
        .header
        .duration_seconds
        .min(found_record.header.duration_seconds);
    let peaks = |rungs: &[Rung]| search_peaks(&index, &query_record.peaks, &profile, rungs, 1);
    let mut searches = vec![
        named(
            "peaks, ladder within 2%",
            &peaks(&within_two_percent()),
            shorter,
        ),
        named("peaks, turntable ladder", &peaks(&ladder()), shorter),
        named(
            "peaks, key-locked ladder",
            &peaks(&key_lock_ladder()),
            shorter,
        ),
    ];
    if let Some(best) = peaks(&ladder()).first() {
        let fitted = [Rung::Turntable(best.speed)];
        let mut search = named("peaks, at the fitted speed", &peaks(&fitted), shorter);
        let query_is_shorter =
            query_record.header.duration_seconds <= found_record.header.duration_seconds;
        for (alignment, detection) in search.alignments.iter_mut().zip(peaks(&fitted)) {
            let seconds =
                supported_seconds(&index, &query_record.peaks, &detection, query_is_shorter);
            alignment.supported = Some(seconds as f64 / shorter);
        }
        searches.push(search);
    }
    if !query.starts_with(SECOND_LIBRARY_PREFIX) {
        let audio = decode(
            &sources.library.root.join(query),
            profile.sample_rate,
            Excerpt::default(),
        )
        .map_err(|error| error.to_string())?;
        let both: Vec<Rung> = ladder().into_iter().chain(key_lock_ladder()).collect();
        searches.push(named(
            "audio, both ladders",
            &search(&index, &audio.samples, &profile, &both, 1),
            shorter,
        ));
        let fitted = Matcher::Fitted.options();
        searches.push(named(
            "audio, both ladders, second pass",
            &search_with(&index, &audio.samples, &profile, &both, 1, fitted),
            shorter,
        ));
    }
    Ok(PairReport {
        query: query.to_owned(),
        found: found.to_owned(),
        query_seconds: query_record.header.duration_seconds,
        found_seconds: found_record.header.duration_seconds,
        searches,
    })
}

/// The turntable rungs from 0.98 to 1.02: the clustering ladder before
/// session 6.
fn within_two_percent() -> Vec<Rung> {
    ladder()
        .into_iter()
        .filter(|rung| (rung.speed().0 - 1.0).abs() <= 0.0201)
        .collect()
}

/// The corpus records, and the other library's files with a current record.
struct Records {
    corpus: Vec<PeakRecord>,
    other: Vec<Asset>,
}

impl Records {
    fn load(sources: &Sources) -> Result<Records, String> {
        let profile = Profile::CURRENT;
        let (corpus, _) = load_records(sources.library, sources.store, &profile, &BTreeSet::new());
        let other = match sources.other {
            Some(other) => {
                other
                    .current_sources(&profile)
                    .map_err(|error| error.to_string())?
                    .0
            }
            None => Vec::new(),
        };
        Ok(Records { corpus, other })
    }

    /// A corpus file's record, or with the prefix one of the other
    /// library's.
    fn record(&self, sources: &Sources, path: &str) -> Result<PeakRecord, String> {
        let Some(other_path) = path.strip_prefix(SECOND_LIBRARY_PREFIX) else {
            return self
                .corpus
                .iter()
                .find(|record| record.header.source.path == path)
                .cloned()
                .ok_or_else(|| format!("no current record of {path} in the corpus store"));
        };
        let other = sources
            .other
            .ok_or("a second-library/ path needs --other-peaks-dir")?;
        let asset = self
            .other
            .iter()
            .find(|asset| asset.path == other_path)
            .ok_or_else(|| format!("no current record of {other_path} in the other store"))?;
        other
            .load(asset, &Profile::CURRENT)
            .map_err(|error| error.to_string())
    }
}

fn named(name: &str, detections: &[Detection], shorter: f64) -> Search {
    Search {
        name: name.to_owned(),
        alignments: detections
            .iter()
            .map(|detection| Alignment {
                query_start_seconds: detection.start_seconds,
                query_end_seconds: detection.end_seconds,
                found_start_seconds: detection.track_start_seconds,
                found_end_seconds: detection.track_end_seconds,
                speed: detection.speed.0,
                key_locked: detection.playback == Playback::KeyLocked,
                hits: detection.evidence.hits,
                windows: detection.evidence.windows,
                coverage: (detection.end_seconds - detection.start_seconds) / shorter,
                offset_seconds: detection.track_start_seconds
                    - detection.speed.0 * detection.start_seconds,
                supported: None,
            })
            .collect(),
    }
}

/// Supported seconds count hits within this many frames of the line
/// through an alignment's ends...
const SEARCH_FRAMES: f64 = 48.0;
/// ...in the densest run of offsets this wide in each query window this
/// long, when it holds this many hits (the search's lines).
const RUN_FRAMES: f64 = 2.0;
const WINDOW_SECONDS: f64 = 10.0;
const MIN_RUN_HITS: usize = 3;

/// The distinct seconds of the shorter file holding a hit of `detection`
/// (a turntable alignment found in stored peaks): each pair hash of the
/// query's peaks at its speed whose posting lies near the line through the
/// alignment's ends. Only each window's densest run of offsets counts, so
/// that a slow drift is followed and chance postings are not.
fn supported_seconds(
    index: &Index,
    query_peaks: &[Peak],
    detection: &Detection,
    query_is_shorter: bool,
) -> usize {
    let profile = Profile::CURRENT;
    let speed = detection.speed.0;
    let first = profile.frames(detection.start_seconds);
    let last = profile.frames(detection.end_seconds);
    let reference = profile.frames(detection.track_start_seconds);
    let slope = (profile.frames(detection.track_end_seconds) - reference) / (last - first);
    if !(last > first && slope.is_finite()) {
        return 0;
    }
    // Rescaled as the stored-peak search rescales them for a rung.
    let points: Vec<Point> = query_peaks
        .iter()
        .map(|peak| Point {
            frame: peak.frame * speed,
            bin: (f64::from(peak.bin) / speed) as f32,
        })
        .collect();
    let window_frames = profile.frames(WINDOW_SECONDS);
    // Per window: each hit's offset from the line and its second in each file.
    let mut windows: BTreeMap<u64, Vec<(f64, u64, u64)>> = BTreeMap::new();
    for_each_pair(&points, |hash, anchor| {
        let query_frame = points[anchor].frame / speed;
        if query_frame < first || query_frame > last {
            return;
        }
        let expected = reference + slope * (query_frame - first);
        for posting in index.postings(hash) {
            let found_frame = f64::from(posting.frame());
            let offset = found_frame - expected;
            if offset.abs() <= SEARCH_FRAMES {
                windows
                    .entry(((query_frame - first) / window_frames) as u64)
                    .or_default()
                    .push((
                        offset,
                        profile.seconds(query_frame) as u64,
                        profile.seconds(found_frame) as u64,
                    ));
            }
        }
    });
    let mut seconds = BTreeSet::new();
    for hits in windows.values_mut() {
        hits.sort_by(|a, b| a.0.total_cmp(&b.0));
        let mut best = 0..0;
        let mut end = 0;
        for start in 0..hits.len() {
            end = end.max(start);
            while end < hits.len() && hits[end].0 - hits[start].0 <= RUN_FRAMES {
                end += 1;
            }
            if end - start > best.len() {
                best = start..end;
            }
        }
        if best.len() >= MIN_RUN_HITS {
            seconds.extend(hits[best].iter().map(
                |&(_, query, found)| {
                    if query_is_shorter { query } else { found }
                },
            ));
        }
    }
    seconds.len()
}

pub fn print_summary(reports: &[PairReport]) {
    for report in reports {
        print_pair(report);
    }
}

fn print_pair(report: &PairReport) {
    println!(
        "{} ({:.1} s) against {} ({:.1} s)",
        report.query, report.query_seconds, report.found, report.found_seconds
    );
    for search in &report.searches {
        println!("  {}:", search.name);
        for alignment in search.alignments.iter().take(6) {
            println!(
                "    query {:>6.1}-{:>6.1} s  found {:>6.1}-{:>6.1} s  speed {:.4}{}  {:>5} hits {:>3} windows  coverage {:>3.0}%{}  offset {:>7.2} s",
                alignment.query_start_seconds,
                alignment.query_end_seconds,
                alignment.found_start_seconds,
                alignment.found_end_seconds,
                alignment.speed,
                if alignment.key_locked {
                    " key-locked"
                } else {
                    ""
                },
                alignment.hits,
                alignment.windows,
                100.0 * alignment.coverage,
                alignment
                    .supported
                    .map_or_else(String::new, |supported| format!(
                        "  supported {:>3.0}%",
                        100.0 * supported
                    )),
                alignment.offset_seconds
            );
        }
    }
}

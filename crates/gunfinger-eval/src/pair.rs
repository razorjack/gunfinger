//! The evidence between two library files, searched alone: why a pair of
//! rips covers less of each other than the clustering rule needs. The
//! query is searched against an index of the found file only, by its
//! stored peaks on several ladders and at the fitted speed, and, for a
//! corpus file, by its decoded audio. Every alignment is listed, so a cut
//! or an edit shows as alignments at one speed with different offsets.

use std::collections::BTreeSet;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library};
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
        searches.push(named(
            "peaks, at the fitted speed",
            &peaks(&fitted),
            shorter,
        ));
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
            })
            .collect(),
    }
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
                "    query {:>6.1}-{:>6.1} s  found {:>6.1}-{:>6.1} s  speed {:.4}{}  {:>5} hits {:>3} windows  coverage {:>3.0}%  offset {:>7.2} s",
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
                alignment.offset_seconds
            );
        }
    }
}

//! A passage query (session 74): which files of the index hold a passage
//! of one file, such as a break the listener likes, strongest first.
//!
//! The passage is the file's stored peaks within a span, searched as
//! `holders` searches one (`search_peaks`: peaks rescaled to each rung of
//! the product's turntable ladder, one pass) against the whole index the
//! harness builds, twice: with every posting list, and with the fullest
//! lists set aside as the matcher's first pass sets them aside (ADR 0014).
//! Each file is listed once, at its strongest detection; the query's own
//! file and its duplicate cluster are listed apart. For the strongest
//! other files, the whole query file is measured against the found file
//! alone, to tell a file holding the passage only from one that goes on
//! matching beyond it (another rip or version).

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use gunfinger_core::hash::{PairHash, Point, for_each_pair};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, SKIPPED_SHARE, search_peaks};
use gunfinger_core::speed::{Playback, Rung, SpeedRatio};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::harness::Ladder;
use crate::padding::Padding;
use crate::shared_mixes::passage;

/// A file is listed when one of its detections has at least this many
/// hits: below `shared_mixes::LISTED_HOLDER_HITS` (30), because a passage
/// of 20 s holds fewer hits than the passages `holders` looked up.
pub const FLOOR_HITS: u32 = 20;
/// The other files whose reach is measured, strongest first.
pub const REACHED_FILES: usize = 20;
/// The listing counts files at these hits or more.
const COUNTED_AT: [u32; 3] = [20, 30, 60];

/// One passage to search. Unknown fields are refused.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub name: String,
    pub file: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Files the scoring expects to hold the passage: the report lists
    /// those the index does not hold, so that they are not counted as
    /// missed.
    #[serde(default)]
    pub expected: Vec<String>,
}

/// Which posting lists the passage's search scans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lists {
    /// Every list.
    All,
    /// All but the fullest share of lists, as the matcher's first pass.
    Filtered,
}

impl Lists {
    fn name(self) -> &'static str {
        match self {
            Lists::All => "all",
            Lists::Filtered => "filtered",
        }
    }
}

#[derive(Serialize)]
pub struct Report {
    pub criterion: String,
    pub index_files: usize,
    pub index_seconds: f64,
    pub searches: Vec<Searched>,
}

#[derive(Serialize)]
pub struct Searched {
    pub name: String,
    pub file: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub lists: &'static str,
    pub search_seconds: f64,
    pub reach_seconds: f64,
    /// The passage's distinct hashes at speed 1.0, those with postings,
    /// and those whose lists the filtered search sets aside.
    pub passage_hashes: usize,
    pub held_hashes: usize,
    pub set_aside_hashes: usize,
    /// Files listed (own included) at each of `COUNTED_AT`'s hits.
    pub listed_at: Vec<(u32, usize)>,
    pub own: Vec<Holder>,
    pub others: Vec<Holder>,
    /// The query's expected files that the index does not hold.
    pub expected_not_indexed: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Holder {
    pub file: String,
    pub hits: u32,
    pub windows: u32,
    /// Hits over the passage's length: the ranking measure.
    pub hits_per_second: f64,
    pub query_start_seconds: f64,
    pub query_end_seconds: f64,
    pub found_start_seconds: f64,
    pub found_end_seconds: f64,
    /// The found file's speed relative to the query file's.
    pub speed: f64,
    /// The rung kind of the detection.
    pub playback: Played,
    /// The file's detections of `FLOOR_HITS` or more. The search keeps
    /// one detection of a file per moment of the query, so for a passage
    /// of 20 s this is nearly always 1.
    pub detections: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reach: Option<Reach>,
}

/// The whole query file against the found file alone, on the rung of the
/// strongest detection: the strongest alignment overlapping the passage.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Reach {
    pub query_start_seconds: f64,
    pub query_end_seconds: f64,
    pub hits: u32,
}

/// One detection of a file, by the file's name.
#[derive(Clone, Debug, PartialEq)]
pub struct Found {
    pub file: String,
    pub detection: Detection,
}

/// Each file with a detection of `FLOOR_HITS` or more, once, at its
/// strongest detection, strongest first (ties by path); those of `own`
/// apart from the others.
pub fn ranked(
    found: Vec<Found>,
    own: &BTreeSet<String>,
    passage_seconds: f64,
) -> (Vec<Holder>, Vec<Holder>) {
    let mut by_file: BTreeMap<String, Holder> = BTreeMap::new();
    for Found { file, detection } in found {
        if detection.evidence.hits < FLOOR_HITS {
            continue;
        }
        let candidate = holder(&file, &detection, passage_seconds);
        match by_file.get_mut(&file) {
            Some(kept) => {
                kept.detections += 1;
                if candidate.hits > kept.hits {
                    *kept = Holder {
                        detections: kept.detections,
                        ..candidate
                    };
                }
            }
            None => {
                by_file.insert(file, candidate);
            }
        }
    }
    let (mut own_files, mut others): (Vec<Holder>, Vec<Holder>) = by_file
        .into_values()
        .partition(|holder| own.contains(&holder.file));
    for listed in [&mut own_files, &mut others] {
        listed.sort_by(|a, b| b.hits.cmp(&a.hits).then_with(|| a.file.cmp(&b.file)));
    }
    (own_files, others)
}

fn holder(file: &str, detection: &Detection, passage_seconds: f64) -> Holder {
    Holder {
        file: file.to_owned(),
        hits: detection.evidence.hits,
        windows: detection.evidence.windows,
        hits_per_second: f64::from(detection.evidence.hits) / passage_seconds,
        query_start_seconds: detection.start_seconds,
        query_end_seconds: detection.end_seconds,
        found_start_seconds: detection.track_start_seconds,
        found_end_seconds: detection.track_end_seconds,
        speed: detection.speed.0,
        playback: Played::from(detection.playback),
        detections: 1,
        reach: None,
    }
}

/// How a holder's detection was played, as the report names it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Played {
    Turntable,
    KeyLocked,
}

impl From<Playback> for Played {
    fn from(playback: Playback) -> Played {
        match playback {
            Playback::Turntable => Played::Turntable,
            Playback::KeyLocked => Played::KeyLocked,
        }
    }
}

impl Played {
    fn rung(self, speed: SpeedRatio) -> Rung {
        match self {
            Played::Turntable => Rung::Turntable(speed),
            Played::KeyLocked => Rung::KeyLocked(speed),
        }
    }
}

/// The passage's distinct hashes at speed 1.0.
fn hashes(passage: &[Peak]) -> BTreeSet<PairHash> {
    let points: Vec<Point> = passage
        .iter()
        .map(|peak| Point {
            frame: peak.frame,
            bin: peak.bin,
        })
        .collect();
    let mut hashes = BTreeSet::new();
    for_each_pair(&points, |hash, _| {
        hashes.insert(hash);
    });
    hashes
}

/// Of `hashes`, those with postings in `index`, and those of them whose
/// lists the index's search leaves out.
fn held_and_set_aside(index: &Index, hashes: &BTreeSet<PairHash>) -> (usize, usize) {
    let held = hashes
        .iter()
        .filter(|&&hash| !index.postings(hash).is_empty())
        .count();
    let set_aside = hashes
        .iter()
        .filter(|&&hash| {
            !index.postings(hash).is_empty() && index.scanned_postings(hash).is_empty()
        })
        .count();
    (held, set_aside)
}

/// The index the passage is searched against for `lists`, from the index
/// as built.
fn searched_index(index: Index, lists: Lists) -> Index {
    match lists {
        Lists::All => index,
        Lists::Filtered => index.skipping_fullest(SKIPPED_SHARE),
    }
}

/// Where the queries' records come from, and the files of each recording.
pub struct Sources<'a> {
    pub library: &'a Library,
    pub store: &'a PeakStore,
    pub padding: &'a Padding,
    pub clusters: &'a Clusters,
}

/// Every query in both modes, all lists first, `jobs` queries at a time,
/// each on one thread.
pub fn run(
    queries: &[Query],
    sources: &Sources,
    ladder: Ladder,
    jobs: usize,
) -> Result<Report, String> {
    let rungs = ladder.rungs(0);
    let profile = Profile::CURRENT;
    let (corpus, problems) =
        load_records(sources.library, sources.store, &profile, &BTreeSet::new());
    for problem in &problems {
        eprintln!("left out of the index: {problem}");
    }
    let started = Instant::now();
    // In library order, as the saved index's header names the files.
    let built = sources.padding.index(&corpus, &profile, &BTreeSet::new())?;
    let index_seconds = started.elapsed().as_secs_f64();
    let index_files = built.assets().len();
    let records = Records {
        corpus: corpus
            .iter()
            .map(|record| (record.header.source.path.as_str(), record))
            .collect(),
        padding: sources.padding,
    };
    let own_of = own_files(sources.clusters);
    let paths: Vec<String> = built
        .assets()
        .iter()
        .map(|asset| asset.path.clone())
        .collect();
    let indexed: BTreeSet<&str> = paths.iter().map(String::as_str).collect();
    let mut searches = Vec::new();
    let mut index = built;
    for lists in [Lists::All, Lists::Filtered] {
        index = searched_index(index, lists);
        let searched = map_in_order(queries, jobs, |query| {
            search_query(&index, &indexed, &records, &own_of, query, lists, &rungs)
        });
        eprintln!(
            "{} queries searched with {} lists",
            queries.len(),
            lists.name()
        );
        searches.extend(searched);
    }
    Ok(Report {
        criterion: criterion(ladder, rungs.len()),
        index_files,
        index_seconds,
        searches,
    })
}

fn criterion(ladder: Ladder, rungs: usize) -> String {
    format!(
        "the passage's stored peaks searched on the {} ladder ({rungs} rungs), one pass; each file at its strongest detection of {FLOOR_HITS} hits or more, ranked by hits over the passage's length; the query's file and its duplicate cluster apart; reach for the {REACHED_FILES} strongest others; lists: all, then filtered (the fullest {SKIPPED_SHARE} set aside)",
        ladder.name()
    )
}

/// Each clustered file's cluster.
fn own_files(clusters: &Clusters) -> BTreeMap<&str, &[String]> {
    clusters
        .duplicates
        .iter()
        .flat_map(|members| {
            members
                .iter()
                .map(move |member| (member.as_str(), members.as_slice()))
        })
        .collect()
}

/// The records of the corpus in memory, and those of the other library
/// read when asked.
struct Records<'a> {
    corpus: BTreeMap<&'a str, &'a PeakRecord>,
    padding: &'a Padding,
}

impl Records<'_> {
    fn record(&self, file: &str) -> Result<Cow<'_, PeakRecord>, String> {
        if let Some(&record) = self.corpus.get(file) {
            return Ok(Cow::Borrowed(record));
        }
        match self
            .padding
            .other
            .as_ref()
            .and_then(|other| other.record(file))
        {
            Some(record) => record.map(Cow::Owned),
            None => Err(format!("no record of {file} at hand")),
        }
    }
}

fn search_query(
    index: &Index,
    indexed: &BTreeSet<&str>,
    records: &Records,
    own_of: &BTreeMap<&str, &[String]>,
    query: &Query,
    lists: Lists,
    rungs: &[Rung],
) -> Searched {
    let mut searched = Searched {
        name: query.name.clone(),
        file: query.file.clone(),
        start_seconds: query.start_seconds,
        end_seconds: query.end_seconds,
        lists: lists.name(),
        search_seconds: 0.0,
        reach_seconds: 0.0,
        passage_hashes: 0,
        held_hashes: 0,
        set_aside_hashes: 0,
        listed_at: Vec::new(),
        own: Vec::new(),
        others: Vec::new(),
        expected_not_indexed: query
            .expected
            .iter()
            .filter(|file| !indexed.contains(file.as_str()))
            .cloned()
            .collect(),
        error: None,
    };
    let record = match records.record(&query.file) {
        Ok(record) => record,
        Err(error) => {
            searched.error = Some(error);
            return searched;
        }
    };
    let profile = Profile::CURRENT;
    let passage = passage(
        &record.peaks,
        query.start_seconds,
        query.end_seconds,
        &profile,
    );
    let hashes = hashes(&passage);
    (searched.held_hashes, searched.set_aside_hashes) = held_and_set_aside(index, &hashes);
    searched.passage_hashes = hashes.len();
    let started = Instant::now();
    let found: Vec<Found> = search_peaks(index, &passage, &profile, rungs, 1)
        .into_iter()
        .map(|detection| Found {
            file: index.asset(detection.asset).path.clone(),
            detection,
        })
        .collect();
    searched.search_seconds = started.elapsed().as_secs_f64();
    let mut own: BTreeSet<String> = own_of
        .get(query.file.as_str())
        .map(|members| members.iter().cloned().collect())
        .unwrap_or_default();
    own.insert(query.file.clone());
    let (own_listed, mut others) = ranked(found, &own, query.end_seconds - query.start_seconds);
    searched.listed_at = COUNTED_AT
        .iter()
        .map(|&at| {
            let count =
                |listed: &[Holder]| listed.iter().filter(|holder| holder.hits >= at).count();
            (at, count(&own_listed) + count(&others))
        })
        .collect();
    let started = Instant::now();
    for holder in others.iter_mut().take(REACHED_FILES) {
        holder.reach = match records.record(&holder.file) {
            Ok(found) => reach(&record.peaks, &found, holder),
            Err(error) => {
                eprintln!("reach not measured: {error}");
                None
            }
        };
    }
    searched.reach_seconds = started.elapsed().as_secs_f64();
    searched.own = own_listed;
    searched.others = others;
    searched
}

/// The whole query against `found` alone, every list, at the speed and
/// playback of `holder`'s detection (the chain's fitted speed, not
/// necessarily a rung of the ladder): the strongest alignment overlapping it in the
/// query. `None` when no alignment kept overlaps it: the search keeps one
/// alignment per moment of the query, so a stronger one elsewhere in the
/// found file can take the passage's place.
fn reach(query: &[Peak], found: &PeakRecord, holder: &Holder) -> Option<Reach> {
    let index = match Index::build(std::slice::from_ref(found)) {
        Ok(index) => index,
        Err(error) => {
            eprintln!("reach not measured for {}: {error}", holder.file);
            return None;
        }
    };
    let rung = [holder.playback.rung(SpeedRatio(holder.speed))];
    search_peaks(&index, query, &Profile::CURRENT, &rung, 1)
        .into_iter()
        .filter(|aligned| {
            aligned.start_seconds < holder.query_end_seconds
                && holder.query_start_seconds < aligned.end_seconds
        })
        .max_by_key(|aligned| aligned.evidence.hits)
        .map(|aligned| Reach {
            query_start_seconds: aligned.start_seconds,
            query_end_seconds: aligned.end_seconds,
            hits: aligned.evidence.hits,
        })
}

pub fn print_summary(report: &Report) {
    println!(
        "{} files in the index (built or loaded in {:.1} s)",
        report.index_files, report.index_seconds
    );
    for searched in &report.searches {
        let strongest = searched
            .others
            .first()
            .map_or(String::from("none"), |holder| {
                format!("{} ({} hits)", holder.file, holder.hits)
            });
        println!(
            "{:<32} {:<8} {:>5} others, strongest {strongest}",
            searched.name,
            searched.lists,
            searched.others.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::confidence::{Evidence, Pass};
    use gunfinger_core::index::AssetId;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::speed::Playback;
    use gunfinger_core::store::RecordHeader;

    use super::*;

    fn found(file: &str, hits: u32, start_seconds: f64) -> Found {
        Found {
            file: file.to_owned(),
            detection: Detection {
                asset: AssetId(0),
                start_seconds,
                end_seconds: start_seconds + 10.0,
                track_start_seconds: 100.0,
                track_end_seconds: 110.0,
                speed: SpeedRatio(1.0),
                playback: Playback::Turntable,
                evidence: Evidence {
                    hits,
                    windows: 2,
                    pass: Pass::Ladder,
                },
            },
        }
    }

    fn files(listed: &[Holder]) -> Vec<(&str, u32, usize)> {
        listed
            .iter()
            .map(|holder| (holder.file.as_str(), holder.hits, holder.detections))
            .collect()
    }

    #[test]
    fn each_file_is_listed_once_at_its_strongest_detection_strongest_first() {
        let own = BTreeSet::from([String::from("query.mp3"), String::from("rip.mp3")]);
        let (own_listed, others) = ranked(
            vec![
                found("b.mp3", 40, 0.0),
                found("a.mp3", 25, 0.0),
                found("b.mp3", 90, 5.0),
                found("c.mp3", 90, 0.0),
                found("rip.mp3", 300, 0.0),
                found("query.mp3", 500, 0.0),
                found("weak.mp3", FLOOR_HITS - 1, 0.0),
                found("a.mp3", FLOOR_HITS - 1, 9.0),
            ],
            &own,
            20.0,
        );

        assert_eq!(
            files(&others),
            [("b.mp3", 90, 2), ("c.mp3", 90, 1), ("a.mp3", 25, 1)]
        );
        assert_eq!(others[0].query_start_seconds, 5.0);
        assert_eq!(others[0].hits_per_second, 4.5);
        assert_eq!(
            files(&own_listed),
            [("query.mp3", 500, 1), ("rip.mp3", 300, 1)]
        );
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

    fn peak(frame: f64, bin: f32) -> Peak {
        Peak {
            frame,
            bin,
            magnitude: 10.0,
        }
    }

    /// A sequence of pseudo-random numbers, the same in every run.
    fn numbers(seed: u64) -> impl FnMut() -> u64 {
        let mut state = seed;
        move || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        }
    }

    /// A drum loop: one bar of 30 peaks repeated every 200 frames, so its
    /// hashes fill the fullest lists.
    fn looped(frames: f64) -> Vec<Peak> {
        let mut next = numbers(0x2545_f491_4f6c_dd1d);
        let mut bar: Vec<Peak> = (0..30)
            .map(|_| peak((next() % 200) as f64, 50.0 + (next() % 400) as f32))
            .collect();
        bar.sort_by(|a, b| a.frame.total_cmp(&b.frame));
        let mut peaks = Vec::new();
        let mut start = 0.0;
        while start < frames {
            peaks.extend(bar.iter().map(|beat| peak(start + beat.frame, beat.bin)));
            start += 200.0;
        }
        peaks
    }

    /// Unrelated peaks: many hashes, each held a few times at most.
    fn scattered(count: usize) -> Vec<Peak> {
        let mut next = numbers(0x9e37_79b9_7f4a_7c15);
        (0..count)
            .map(|i| peak(i as f64 * 3.0, 5.0 + (next() % 490) as f32))
            .collect()
    }

    #[test]
    fn the_filtered_search_sets_a_loops_full_lists_aside_and_all_lists_keep_them() {
        let loop_peaks = looped(6_000.0);
        let records = [
            record("loop.mp3", loop_peaks.clone()),
            record("other.mp3", scattered(20_000)),
        ];
        let profile = Profile::CURRENT;
        let passage = passage(&loop_peaks, 10.0, 30.0, &profile);
        let hashes = hashes(&passage);
        let found = |index: &Index| -> Vec<String> {
            search_peaks(index, &passage, &profile, &Ladder::Turntable.rungs(0), 1)
                .into_iter()
                .filter(|detection| detection.evidence.hits >= FLOOR_HITS)
                .map(|detection| index.asset(detection.asset).path.clone())
                .collect()
        };

        let all = searched_index(Index::build(&records).unwrap(), Lists::All);
        assert_eq!(held_and_set_aside(&all, &hashes), (hashes.len(), 0));
        assert!(found(&all).contains(&String::from("loop.mp3")));

        let filtered = searched_index(Index::build(&records).unwrap(), Lists::Filtered);
        assert_eq!(
            held_and_set_aside(&filtered, &hashes),
            (hashes.len(), hashes.len())
        );
        assert!(found(&filtered).is_empty());
    }

    #[test]
    fn a_key_locked_passage_is_found_on_both_ladders_and_its_reach_follows_key_lock() {
        let profile = Profile::CURRENT;
        // High bins, which a turntable rung 4% off moves by 10-20 bins.
        let original: Vec<Peak> = scattered(20_000)
            .into_iter()
            .map(|p| peak(p.frame, 250.0 + p.bin / 2.0))
            .collect();
        // The query file plays the original 4% faster with its pitch kept:
        // times shrink, frequencies stay.
        let key_locked: Vec<Peak> = original
            .iter()
            .map(|p| peak(p.frame / 1.04, p.bin))
            .collect();
        let records = [record("original.mp3", original)];
        let index = Index::build(&records).unwrap();
        let passage = passage(&key_locked, 20.0, 40.0, &profile);
        let strongest = |ladder: Ladder| {
            search_peaks(&index, &passage, &profile, &ladder.rungs(0), 1)
                .into_iter()
                .filter(|detection| detection.evidence.hits >= FLOOR_HITS)
                .max_by_key(|detection| detection.evidence.hits)
        };

        assert!(strongest(Ladder::Turntable).is_none());
        let detection = strongest(Ladder::Both).unwrap();
        assert_eq!(detection.playback, Playback::KeyLocked);
        let holder = holder("original.mp3", &detection, 20.0);
        assert_eq!(holder.playback, Played::KeyLocked);
        let reach = reach(&key_locked, &records[0], &holder).unwrap();
        assert!(reach.query_end_seconds - reach.query_start_seconds > 60.0);
    }

    #[test]
    fn a_passage_is_the_files_peaks_within_the_span() {
        let profile = Profile::CURRENT;
        let at = |seconds: f64| peak(profile.frames(seconds), 100.0);
        let peaks = vec![at(1.0), at(10.0), at(15.0), at(30.0), at(31.0)];

        assert_eq!(
            passage(&peaks, 10.0, 30.0, &profile),
            [at(10.0), at(15.0), at(30.0)]
        );
    }
}

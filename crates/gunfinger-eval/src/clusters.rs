//! Duplicate clusters: library files that are the same recording.
//!
//! Each file is searched against the whole library on the matcher's
//! turntable ladder, with the fullest posting lists set aside: rips of one
//! recording usually play within a percent of each other, but uploads sped
//! up by 3-5% are common (experiments 0039, 0048). Each candidate pair is
//! also measured again alone, at the candidate's fitted speed, and the
//! alignment that covers more counts: one rung loses a rip whose speed
//! drifts, which the ladder's chains follow from rung to rung (experiment
//! 0049). Two files are the same recording when that single alignment
//! covers nearly all of the shorter one. The
//! criterion is deliberately strict: a remix or VIP that shares some
//! sections with the original must stay separate. The owner's verdicts on
//! pairs (`Verdicts`) override it. Clusters depend on library audio and
//! the owner's ears alone, never on what a set scan returned.
//!
//! With `Source::Peaks` each file's stored peaks are searched instead of its
//! decoded audio: no decoding, so it scales to a large library, at the cost
//! of the hashes that rescaled peaks lose (`search::search_peaks`).
//!
//! In a larger library the corpus was drawn from, `find_around` finds the
//! clusters of the corpus recordings only: clustering every file of tens of
//! thousands against every other would take days.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::{TrackLength, build_index, load_records};
use gunfinger_core::library::ignore::{Pattern, leaving_out};
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, SKIPPED_SHARE, search, search_peaks};
use gunfinger_core::speed::{self, Playback, Rung};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::padding::SECOND_LIBRARY_PREFIX;

/// The alignment must cover this share of the shorter file.
const MIN_COVERAGE: f64 = 0.8;
/// Pairs above this coverage but below `MIN_COVERAGE` are reported for review.
const REPORTED_COVERAGE: f64 = 0.2;
/// In another library, pairs with this many hits are kept whatever their
/// coverage: the census of related recordings (experiment 0015).
const RELATED_HITS: u32 = 30;
/// Pairs from this coverage up to `MIN_COVERAGE` go to the owner to judge.
const BORDERLINE_COVERAGE: f64 = 0.4;
/// The other library was indexed with its own track length range.
const ANY_LENGTH: TrackLength = TrackLength {
    min: Duration::ZERO,
    max: Duration::from_secs(24 * 3600),
};
/// Candidates with fewer hits and less coverage are not measured again:
/// chance alignments, most of every query's candidates.
const MIN_HITS_TO_MEASURE: u32 = RELATED_HITS;
/// Joins with fewer hits per second of aligned span than this share of
/// the median join are listed for the owner's ear. Coverage counts the
/// span from the first to the last hit, however little evidence lies in
/// it: at NAS scale one join covers 91% with 0.7 hits per second, against
/// 18.6 for the median join. A flag for listening, not part of the
/// criterion.
const SPARSE_SHARE: f64 = 0.1;

/// Every library asset's cluster. Assets without a duplicate form a cluster
/// of their own.
#[derive(Debug, Serialize, Deserialize)]
pub struct Clusters {
    pub criterion: String,
    /// Clusters with more than one member, each sorted, ordered by first member.
    pub duplicates: Vec<Vec<String>>,
    /// The evidence for every pair considered, strongest first.
    pub pairs: Vec<Pair>,
    /// Joins cut because a chain through them linked two files the owner
    /// judged different (`Contradictions::CutSparsest`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cut_links: Vec<CutLink>,
}

/// A join `Contradictions::CutSparsest` removed: the sparsest measured link
/// of a chain between two files judged different. The harness's guess at
/// the wrong link, for the owner to judge.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CutLink {
    pub query: String,
    pub found: String,
    /// The two files judged different that the chain linked.
    pub judged_different: [String; 2],
}

/// What `clusters` does when a chain of joins links two files the owner
/// judged different.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contradictions {
    /// Stop and name the chain, for the owner to judge its wrong link.
    Stop,
    /// Cut the chain's measured join with the fewest hits per second of
    /// aligned span, repeatedly until no chain is left, and list each cut.
    CutSparsest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pair {
    pub query: String,
    pub found: String,
    /// Share of the shorter file covered by the alignment.
    pub coverage: f64,
    pub speed: f64,
    pub hits: u32,
    pub same_recording: bool,
    /// Where the alignment lies in each file, in seconds.
    #[serde(default)]
    pub query_start_seconds: f64,
    #[serde(default)]
    pub query_end_seconds: f64,
    #[serde(default)]
    pub found_start_seconds: f64,
    #[serde(default)]
    pub found_end_seconds: f64,
    /// The owner's verdict on the pair, when there is one; it decides
    /// `same_recording`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_verdict: Option<bool>,
    /// The coverage of the candidate on the ladder, when the pair was
    /// also measured again at its fitted speed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ladder_coverage: Option<f64>,
}

impl Pair {
    /// How densely hits fill the query's aligned span, the span that
    /// coverage counts.
    pub fn hits_per_second(&self) -> f64 {
        let span = self.query_end_seconds - self.query_start_seconds;
        if span > 0.0 {
            f64::from(self.hits) / span
        } else {
            0.0
        }
    }
}

impl Clusters {
    pub fn load(path: &Path) -> Result<Clusters, String> {
        let text = fs::read_to_string(path).map_err(|error| {
            format!(
                "cannot read {} ({error}); run `gunfinger-eval clusters` first",
                path.display()
            )
        })?;
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// The members of the cluster holding `asset`, including itself.
    pub fn cluster_of(&self, asset: &str) -> BTreeSet<String> {
        self.duplicates
            .iter()
            .find(|members| members.iter().any(|member| member == asset))
            .map_or_else(
                || BTreeSet::from([asset.to_owned()]),
                |members| members.iter().cloned().collect(),
            )
    }
}

/// What each library file is searched with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// Its decoded audio.
    Audio,
    /// Its stored peaks, rescaled to each speed.
    Peaks,
}

pub fn find(
    library: &Library,
    store: &PeakStore,
    source: Source,
    verdicts: &Verdicts,
    contradictions: Contradictions,
    jobs: usize,
) -> Result<Clusters, String> {
    let mut pairs = self_match(library, store, source, jobs, |pair| {
        pair.coverage >= REPORTED_COVERAGE
    })?;
    verdicts.apply(&mut pairs);
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));
    let (duplicates, cut_links) =
        clusters_of(verdicts.links(&pairs), verdicts, &pairs, contradictions)?;
    cut(&mut pairs, &cut_links);

    Ok(Clusters {
        criterion: format!(
            "{}{}{}",
            criterion(),
            match source {
                Source::Audio => "",
                Source::Peaks => " (stored peaks searched)",
            },
            cut_note(&cut_links)
        ),
        duplicates,
        pairs,
        cut_links,
    })
}

fn criterion() -> String {
    let ladder = speed::ladder();
    let speed = |rung: Option<&Rung>| rung.map_or(1.0, |rung| rung.speed().0);
    format!(
        "one alignment at the pair's fitted speed covers at least {:.0}% of the shorter file (candidates on the turntable ladder {:.2}..{:.2}, the fullest {}% of posting lists skipped); the owner's verdicts override it",
        MIN_COVERAGE * 100.0,
        speed(ladder.first()),
        speed(ladder.last()),
        SKIPPED_SHARE * 100.0
    )
}

/// The clusters of the corpus recordings in another library. Each query
/// (a corpus record, or one of the other library's named
/// `second-library/<path>`, `other_queries`) is searched by its stored
/// peaks against an index of `other_assets` alone; files of the other library found to be the same
/// recording are searched in turn, until none is new, so that chains of
/// rips are followed. `copies` (corpus files' identical copies) are not
/// searched again. Files of the other library are named
/// `second-library/<path>`, as in an index of both. Pairs are kept from 20%
/// coverage or 30 hits.
pub fn find_around(
    queries: Vec<PeakRecord>,
    other: &PeakStore,
    other_assets: &[Asset],
    copies: &BTreeSet<String>,
    verdicts: &Verdicts,
    rounds: Rounds,
    jobs: usize,
) -> Result<Vec<Pair>, String> {
    let profile = Profile::CURRENT;
    let built = build_index(other_assets, other, &profile, &BTreeSet::new(), ANY_LENGTH)
        .map_err(|error| error.to_string())?;
    for problem in &built.problems {
        eprintln!("left out: {problem}");
    }
    let index = built.index.skipping_fullest(SKIPPED_SHARE);
    let by_path: BTreeMap<&str, &Asset> = other_assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let ladder = clustering_ladder();
    let mut searched: BTreeSet<String> = queries
        .iter()
        .map(|record| record.header.source.path.clone())
        .collect();
    let mut round = queries;
    let mut pairs = Vec::new();
    let mut number = 0;
    while !round.is_empty() {
        number += 1;
        eprintln!("round {number}: {} queries", round.len());
        let finished = AtomicUsize::new(0);
        let found: Vec<Result<Vec<Pair>, String>> = map_in_order(&round, jobs, |record| {
            let query = &record.header.source.path;
            let detections = search_peaks(&index, &record.peaks, &profile, &ladder, 1);
            let mut kept = Vec::new();
            for detection in candidates(&detections, record.header.duration_seconds, |asset| {
                index.asset(asset).duration_seconds
            }) {
                let asset = index.asset(detection.asset);
                let found = format!("{SECOND_LIBRARY_PREFIX}{}", asset.path);
                if found == *query {
                    continue;
                }
                let Some(&source) = by_path.get(asset.path.as_str()) else {
                    continue;
                };
                let found_record = other
                    .load(source, &profile)
                    .map_err(|error| error.to_string())?;
                let lengths = (record.header.duration_seconds, asset.duration_seconds);
                let refined =
                    at_fitted_speed(Query::Peaks(&record.peaks), &found_record, detection)
                        .map(|refined| pair(query, found.clone(), lengths, &refined));
                let pair = better_of(pair(query, found, lengths, detection), refined);
                if pair.coverage >= REPORTED_COVERAGE || pair.hits >= RELATED_HITS {
                    kept.push(pair);
                }
            }
            let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
            eprintln!("[{count}/{}] {query}", round.len());
            Ok(strongest_per_file(kept))
        });
        let mut found: Vec<Vec<Pair>> = found.into_iter().collect::<Result<_, String>>()?;
        for pairs in &mut found {
            verdicts.apply(pairs);
        }
        let new: BTreeSet<String> = found
            .iter()
            .flatten()
            .filter(|pair| pair.same_recording && !searched.contains(&pair.found))
            .map(|pair| pair.found.clone())
            .filter(|found| {
                found
                    .strip_prefix(SECOND_LIBRARY_PREFIX)
                    .is_some_and(|path| !copies.contains(path))
            })
            .collect();
        round = Vec::new();
        if rounds == Rounds::First {
            pairs.extend(found.into_iter().flatten());
            break;
        }
        for found in new {
            let path = &found[SECOND_LIBRARY_PREFIX.len()..];
            let Some(&asset) = by_path.get(path) else {
                continue;
            };
            let mut record = other
                .load(asset, &profile)
                .map_err(|error| error.to_string())?;
            record.header.source.path = found.clone();
            round.push(record);
            searched.insert(found);
        }
        pairs.extend(found.into_iter().flatten());
    }
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage));
    Ok(pairs)
}

/// The records of the other library's `files` (`second-library/<path>`),
/// named so, to search beside the corpus files. Copies of corpus files are
/// left out: their corpus file is searched.
pub fn other_queries(
    other: &PeakStore,
    other_assets: &[Asset],
    files: &BTreeSet<String>,
    copies: &BTreeSet<String>,
) -> Result<Vec<PeakRecord>, String> {
    let profile = Profile::CURRENT;
    let mut records = Vec::new();
    for asset in other_assets {
        let name = format!("{SECOND_LIBRARY_PREFIX}{}", asset.path);
        if !files.contains(&name) || copies.contains(&asset.path) {
            continue;
        }
        let mut record = other
            .load(asset, &profile)
            .map_err(|error| error.to_string())?;
        record.header.source.path = name;
        records.push(record);
    }
    Ok(records)
}

/// The corpus clusters joined by the same-recording pairs found in another
/// library (`find_around`), which are kept as the evidence, with the
/// owner's verdicts applied.
pub fn merged(
    corpus: &Clusters,
    mut pairs: Vec<Pair>,
    verdicts: &Verdicts,
    other_library: &str,
    contradictions: Contradictions,
) -> Result<Clusters, String> {
    verdicts.apply(&mut pairs);
    let corpus_links = corpus.duplicates.iter().flat_map(|members| {
        members
            .iter()
            .map(|member| (members[0].as_str(), member.as_str()))
    });
    let (duplicates, cut_links) = clusters_of(
        corpus_links.chain(verdicts.links(&pairs)),
        verdicts,
        &pairs,
        contradictions,
    )?;
    cut(&mut pairs, &cut_links);
    Ok(Clusters {
        criterion: format!(
            "{}; with the rips of the corpus recordings in {other_library} (stored peaks searched, named {SECOND_LIBRARY_PREFIX}<path>){}",
            corpus.criterion,
            cut_note(&cut_links)
        ),
        duplicates,
        pairs,
        cut_links,
    })
}

fn cut_note(cut_links: &[CutLink]) -> &'static str {
    if cut_links.is_empty() {
        ""
    } else {
        "; where joins chained two files the owner judged different, the chain's sparsest join was cut (`cut_links`)"
    }
}

/// The joins cut to keep files judged different apart, each with its
/// evidence, for the owner to judge.
pub fn print_cut_links(clusters: &Clusters) {
    if clusters.cut_links.is_empty() {
        return;
    }
    println!(
        "{} joins cut, each the sparsest of a chain between files judged different, for the owner:",
        clusters.cut_links.len()
    );
    for link in &clusters.cut_links {
        let measured = clusters
            .pairs
            .iter()
            .find(|pair| ordered(&pair.query, &pair.found) == ordered(&link.query, &link.found))
            .map_or_else(String::new, evidence);
        println!(
            "  {} ~ {} {measured}, chaining {} to {}",
            link.query, link.found, link.judged_different[0], link.judged_different[1]
        );
    }
}

/// An earlier run's pairs as its search measured them, with the current
/// verdicts in place of that run's and none of its cuts: `same_recording`
/// by the coverage rule unless a verdict decides it. Pairs with a file
/// this run does not search (pruned since, or left out by an ignore file)
/// are dropped; returns the pairs kept and the number dropped.
pub fn as_measured(
    pairs: Vec<Pair>,
    searched: &Searched,
    verdicts: &Verdicts,
) -> (Vec<Pair>, usize) {
    let before = pairs.len();
    let mut kept: Vec<Pair> = pairs
        .into_iter()
        .filter(|pair| {
            searched.outside(&pair.query).is_none() && searched.outside(&pair.found).is_none()
        })
        .map(|pair| Pair {
            same_recording: pair.coverage >= MIN_COVERAGE,
            owner_verdict: None,
            ..pair
        })
        .collect();
    verdicts.apply(&mut kept);
    let dropped = before - kept.len();
    (kept, dropped)
}

/// Marks the pairs of the cut links as not the same recording.
fn cut(pairs: &mut [Pair], cut_links: &[CutLink]) {
    for pair in pairs {
        let link = ordered(&pair.query, &pair.found);
        if cut_links
            .iter()
            .any(|cut| ordered(&cut.query, &cut.found) == link)
        {
            pair.same_recording = false;
        }
    }
}

/// What `find_around` found: whether the corpus clusters reappear through
/// the other library, the other library's further rips of the corpus
/// recordings, and borderline pairs for the owner to judge by ear.
pub fn print_around(corpus: &Clusters, pairs: &[Pair], copies: &BTreeSet<String>) {
    let is_corpus = |path: &str| !path.starts_with(SECOND_LIBRARY_PREFIX);
    let found_again: Vec<Vec<String>> = merge(same_recording(pairs))
        .into_iter()
        .map(|members| {
            members
                .into_iter()
                .filter(|member| is_corpus(member))
                .collect::<Vec<_>>()
        })
        .filter(|members| members.len() > 1)
        .collect();
    let projected = Clusters {
        criterion: String::new(),
        duplicates: found_again,
        pairs: Vec::new(),
        cut_links: Vec::new(),
    };
    print_differences("the other library", &projected, "the corpus", corpus);
    let further: Vec<&Pair> = pairs
        .iter()
        .filter(|pair| {
            pair.same_recording
                && pair
                    .found
                    .strip_prefix(SECOND_LIBRARY_PREFIX)
                    .is_some_and(|path| !copies.contains(path))
        })
        .collect();
    let further_files: BTreeSet<&str> = further.iter().map(|pair| pair.found.as_str()).collect();
    println!(
        "{} further rips of the queries' recordings (not copies of corpus files):",
        further_files.len()
    );
    for pair in further {
        println!("  {} = {} {}", pair.query, pair.found, evidence(pair));
    }
    print_sparse_joins(pairs);
    let borderline: Vec<&Pair> = pairs
        .iter()
        .filter(|pair| pair.coverage >= BORDERLINE_COVERAGE && !pair.same_recording)
        .collect();
    println!(
        "{} borderline pairs ({:.0}% to {:.0}% coverage), for the owner:",
        borderline.len(),
        100.0 * BORDERLINE_COVERAGE,
        100.0 * MIN_COVERAGE
    );
    for pair in borderline {
        println!("  {} ~ {} {}", pair.query, pair.found, evidence(pair));
    }
    let related = pairs
        .iter()
        .filter(|pair| pair.coverage < REPORTED_COVERAGE)
        .count();
    println!(
        "{related} pairs below 20% coverage with {RELATED_HITS} hits or more (related recordings)"
    );
}

/// Rips of one recording, uploads sped up by a few percent included: the
/// matcher's turntable ladder.
fn clustering_ladder() -> Vec<Rung> {
    speed::ladder()
}

/// Whether to follow the corpus recordings' rips round after round, or to
/// stop after the corpus files' own searches (for timing on a sample).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Rounds {
    UntilNoneIsNew,
    First,
}

/// What a library file is searched with.
#[derive(Clone, Copy)]
enum Query<'a> {
    Audio(&'a [f32]),
    Peaks(&'a [Peak]),
}

/// The detections of other files worth measuring again: a few dozen hits,
/// or a fifth of the shorter file. `duration` gives each asset's length.
fn candidates<'a>(
    detections: &'a [Detection],
    query_seconds: f64,
    duration: impl Fn(gunfinger_core::index::AssetId) -> f64 + 'a,
) -> impl Iterator<Item = &'a Detection> + 'a {
    detections.iter().filter(move |detection| {
        let shorter = query_seconds.min(duration(detection.asset));
        detection.evidence.hits >= MIN_HITS_TO_MEASURE
            || (detection.end_seconds - detection.start_seconds) >= REPORTED_COVERAGE * shorter
    })
}

/// The pair measured again alone: the query against the found file's
/// record only, on one rung at the candidate's fitted speed and playback,
/// in every posting list. `None` when that finds nothing.
fn at_fitted_speed(query: Query, found: &PeakRecord, candidate: &Detection) -> Option<Detection> {
    let profile = Profile::CURRENT;
    let index = Index::build(std::slice::from_ref(found)).ok()?;
    let rung = [match candidate.playback {
        Playback::Turntable => Rung::Turntable(candidate.speed),
        Playback::KeyLocked => Rung::KeyLocked(candidate.speed),
    }];
    let detections = match query {
        Query::Audio(samples) => search(&index, samples, &profile, &rung, 1),
        Query::Peaks(peaks) => search_peaks(&index, peaks, &profile, &rung, 1),
    };
    detections.into_iter().next()
}

/// The pair a detection makes, its coverage counted in the shorter file's
/// own seconds: an upload 4% fast is 4% shorter, and its span in the
/// longer file would overstate it (experiment 0048). `lengths` are the
/// query's and the found file's.
fn pair(query: &str, found: String, lengths: (f64, f64), detection: &Detection) -> Pair {
    let (query_seconds, found_seconds) = lengths;
    let coverage = if query_seconds <= found_seconds {
        (detection.end_seconds - detection.start_seconds) / query_seconds
    } else {
        (detection.track_end_seconds - detection.track_start_seconds) / found_seconds
    };
    Pair {
        query: query.to_owned(),
        found,
        coverage,
        speed: detection.speed.0,
        hits: detection.evidence.hits,
        same_recording: coverage >= MIN_COVERAGE,
        query_start_seconds: detection.start_seconds,
        query_end_seconds: detection.end_seconds,
        found_start_seconds: detection.track_start_seconds,
        found_end_seconds: detection.track_end_seconds,
        owner_verdict: None,
        ladder_coverage: None,
    }
}

/// The candidate on the ladder or the pair measured again at its fitted
/// speed, whichever covers more, with the ladder's coverage kept.
fn better_of(on_ladder: Pair, refined: Option<Pair>) -> Pair {
    let ladder_coverage = Some(on_ladder.coverage);
    match refined {
        Some(refined) if refined.coverage >= on_ladder.coverage => Pair {
            ladder_coverage,
            ..refined
        },
        Some(_) => Pair {
            ladder_coverage,
            ..on_ladder
        },
        None => on_ladder,
    }
}

/// One query's pairs, the strongest with each file: two candidates of one
/// file (two speeds, or two stretches) measured again give one alignment
/// twice.
fn strongest_per_file(mut pairs: Vec<Pair>) -> Vec<Pair> {
    pairs.sort_by(|a, b| b.coverage.total_cmp(&a.coverage).then(b.hits.cmp(&a.hits)));
    let mut seen = BTreeSet::new();
    pairs.retain(|pair| seen.insert(pair.found.clone()));
    pairs
}

fn same_recording(pairs: &[Pair]) -> impl Iterator<Item = (&str, &str)> {
    pairs
        .iter()
        .filter(|pair| pair.same_recording)
        .map(|pair| (pair.query.as_str(), pair.found.as_str()))
}

/// The owner's verdicts on pairs of files, by ear: the same recording or
/// different ones, whatever the coverage says. Read from a plain text file
/// (`docs/pair-verdicts.txt`): one verdict per line, `same` or `different`
/// and the two paths, separated by tabs; `#` starts a comment line.
#[derive(Debug, Default)]
pub struct Verdicts {
    /// The file they were read from, for messages.
    file: PathBuf,
    /// Pairs in path order, with the owner's verdict.
    verdicts: BTreeMap<(String, String), Verdict>,
}

#[derive(Debug, Clone, Copy)]
struct Verdict {
    same: bool,
    /// Its line in the file, counted from 1.
    line: usize,
}

impl Verdicts {
    /// The verdicts in `path`; none when the file does not exist.
    pub fn load(path: &Path) -> Result<Verdicts, String> {
        match fs::read_to_string(path) {
            Ok(text) => Verdicts::parse(&text)
                .map(|verdicts| Verdicts {
                    file: path.to_owned(),
                    ..verdicts
                })
                .map_err(|error| format!("{}: {error}", path.display())),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Verdicts::default()),
            Err(error) => Err(format!("cannot read {}: {error}", path.display())),
        }
    }

    fn parse(text: &str) -> Result<Verdicts, String> {
        let mut verdicts = BTreeMap::new();
        for (index, line) in text.lines().enumerate() {
            let number = index + 1;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            let fields: Vec<&str> = line.split('\t').collect();
            let same = match fields.first().map(|verdict| verdict.trim()) {
                Some("same") => true,
                Some("different") => false,
                _ => {
                    return Err(format!(
                        "line {number}: a verdict starts with `same` or `different`"
                    ));
                }
            };
            let [_, a, b] = fields[..] else {
                return Err(format!(
                    "line {number}: a verdict and two paths, separated by tabs"
                ));
            };
            let verdict = Verdict { same, line: number };
            if verdicts
                .insert(ordered(a, b), verdict)
                .is_some_and(|earlier| earlier.same != same)
            {
                return Err(format!(
                    "line {number}: {a} and {b} are judged both the same and different; keep one verdict"
                ));
            }
        }
        Ok(Verdicts {
            file: PathBuf::new(),
            verdicts,
        })
    }

    /// Whether a verdict names these two files, in either order.
    pub fn judges(&self, a: &str, b: &str) -> bool {
        self.verdicts.contains_key(&ordered(a, b))
    }

    /// The verdicts on two files the run searches, and a message naming the
    /// line and the file of every other verdict, which adds nothing: a
    /// cluster holds files of the libraries searched, never a file their
    /// ignore files leave out.
    pub fn on(self, searched: &Searched) -> (Verdicts, Vec<String>) {
        let mut messages = Vec::new();
        let mut kept = BTreeMap::new();
        for ((a, b), verdict) in self.verdicts {
            match searched.outside(&a).or_else(|| searched.outside(&b)) {
                Some((file, why)) => messages.push((
                    verdict.line,
                    format!(
                        "{}, line {}: {file} {why}; the verdict adds nothing",
                        self.file.display(),
                        verdict.line
                    ),
                )),
                None => {
                    kept.insert((a, b), verdict);
                }
            }
        }
        messages.sort();
        (
            Verdicts {
                file: self.file,
                verdicts: kept,
            },
            messages.into_iter().map(|(_, message)| message).collect(),
        )
    }

    fn verdict(&self, a: &str, b: &str) -> Option<bool> {
        self.verdicts
            .get(&ordered(a, b))
            .map(|verdict| verdict.same)
    }

    fn judged_different(&self) -> impl Iterator<Item = (&str, &str)> {
        self.verdicts
            .iter()
            .filter(|(_, verdict)| !verdict.same)
            .map(|((a, b), _)| (a.as_str(), b.as_str()))
    }

    /// Sets each judged pair's `same_recording` to the owner's verdict.
    fn apply(&self, pairs: &mut [Pair]) {
        for pair in pairs {
            if let Some(same) = self.verdict(&pair.query, &pair.found) {
                pair.same_recording = same;
                pair.owner_verdict = Some(same);
            }
        }
    }

    /// The same-recording pairs, with the pairs the owner judged the same
    /// whether or not a search found them.
    fn links<'a>(&'a self, pairs: &'a [Pair]) -> impl Iterator<Item = (&'a str, &'a str)> {
        let judged_same = self
            .verdicts
            .iter()
            .filter(|(_, verdict)| verdict.same)
            .map(|((a, b), _)| (a.as_str(), b.as_str()));
        same_recording(pairs).chain(judged_same)
    }
}

/// The files a clusters run searches: the corpus library's assets and,
/// with another library, its files with a current record, named
/// `second-library/<path>`. Their ignore files tell why a file is not
/// among them.
pub struct Searched {
    files: BTreeSet<String>,
    corpus_ignore: Option<IgnoreFile>,
    other_ignore: Option<IgnoreFile>,
}

/// A library's ignore file and its patterns.
pub struct IgnoreFile {
    pub path: PathBuf,
    pub patterns: Vec<Pattern>,
}

impl Searched {
    pub fn corpus(library: &Library) -> Searched {
        Searched {
            files: library
                .assets
                .iter()
                .map(|asset| asset.path.clone())
                .collect(),
            corpus_ignore: library.ignore_file.as_ref().map(|path| IgnoreFile {
                path: path.clone(),
                patterns: library
                    .ignored
                    .iter()
                    .map(|ignored| ignored.pattern.clone())
                    .collect(),
            }),
            other_ignore: None,
        }
    }

    /// With the other library's files that have a current record, less
    /// those its ignore file leaves out, when it could be read.
    pub fn with_other(mut self, assets: &[Asset], ignore: Option<IgnoreFile>) -> Searched {
        self.files.extend(
            assets
                .iter()
                .map(|asset| format!("{SECOND_LIBRARY_PREFIX}{}", asset.path)),
        );
        self.other_ignore = ignore;
        self
    }

    /// `None` when the run searches `file`; otherwise the file and why not.
    fn outside<'a>(&self, file: &'a str) -> Option<(&'a str, String)> {
        let (path, ignore) = match file.strip_prefix(SECOND_LIBRARY_PREFIX) {
            Some(path) => (path, &self.other_ignore),
            None => (file, &self.corpus_ignore),
        };
        let left_out = ignore.as_ref().and_then(|ignore| {
            leaving_out(&ignore.patterns, path).map(|pattern| {
                format!(
                    "is left out by {}, line {} (`{}`)",
                    ignore.path.display(),
                    pattern.line,
                    pattern.text
                )
            })
        });
        match left_out {
            Some(why) => Some((file, why)),
            None if !self.files.contains(file) => Some((
                file,
                String::from("is not a file of the libraries this run searches"),
            )),
            None => None,
        }
    }
}

fn ordered(a: &str, b: &str) -> (String, String) {
    let (first, second) = if a <= b { (a, b) } else { (b, a) };
    (first.to_owned(), second.to_owned())
}

/// Searches every indexed file against the whole library, one file per
/// thread, and returns its detections of other files that `keep` accepts.
pub fn self_match(
    library: &Library,
    store: &PeakStore,
    source: Source,
    jobs: usize,
    keep: impl Fn(&Pair) -> bool + Sync,
) -> Result<Vec<Pair>, String> {
    let profile = Profile::CURRENT;
    let (records, problems) = load_records(library, store, &profile, &BTreeSet::new());
    for problem in &problems {
        eprintln!("left out: {problem}");
    }
    let index = Index::build(&records)
        .map_err(|error| error.to_string())?
        .skipping_fullest(SKIPPED_SHARE);
    let ladder = clustering_ladder();
    let by_path: BTreeMap<&str, &PeakRecord> = records
        .iter()
        .map(|record| (record.header.source.path.as_str(), record))
        .collect();

    let finished = AtomicUsize::new(0);
    let found: Vec<Result<Vec<Pair>, String>> = map_in_order(&records, jobs, |record| {
        let query = &record.header.source.path;
        let audio = match source {
            Source::Audio => Some(
                decode(
                    &library.root.join(query),
                    profile.sample_rate,
                    Excerpt::default(),
                )
                .map_err(|error| error.to_string())?,
            ),
            Source::Peaks => None,
        };
        let searched = match &audio {
            Some(audio) => Query::Audio(&audio.samples),
            None => Query::Peaks(&record.peaks),
        };
        let detections = match searched {
            Query::Audio(samples) => search(&index, samples, &profile, &ladder, 1),
            Query::Peaks(peaks) => search_peaks(&index, peaks, &profile, &ladder, 1),
        };
        let mut pairs = Vec::new();
        for detection in candidates(&detections, record.header.duration_seconds, |asset| {
            index.asset(asset).duration_seconds
        }) {
            let found = &index.asset(detection.asset).path;
            if found == query {
                continue;
            }
            let found_record = by_path[found.as_str()];
            let lengths = (
                record.header.duration_seconds,
                found_record.header.duration_seconds,
            );
            let refined = at_fitted_speed(searched, found_record, detection)
                .map(|refined| pair(query, found.clone(), lengths, &refined));
            let pair = better_of(pair(query, found.clone(), lengths, detection), refined);
            if keep(&pair) {
                pairs.push(pair);
            }
        }
        let count = finished.fetch_add(1, Ordering::Relaxed) + 1;
        eprintln!("[{count}/{}] {query}", records.len());
        Ok(strongest_per_file(pairs))
    });
    let mut pairs = Vec::new();
    for result in found {
        pairs.extend(result?);
    }
    Ok(pairs)
}

pub fn print_summary(clusters: &Clusters) {
    println!("{}", clusters.criterion);
    println!("{} clusters with duplicates:", clusters.duplicates.len());
    for members in &clusters.duplicates {
        println!("  {}", members.join("  |  "));
    }
    print_sparse_joins(&clusters.pairs);
}

/// The joins whose hits fill their aligned span far more thinly than a
/// typical join's, for the owner to check by ear.
fn print_sparse_joins(pairs: &[Pair]) {
    let (median, sparse) = sparse_joins(pairs);
    println!(
        "{} joins with under {:.0}% of the median {median:.1} hits per second of aligned span, for the owner:",
        sparse.len(),
        100.0 * SPARSE_SHARE
    );
    for pair in sparse {
        println!("  {} = {} {}", pair.query, pair.found, evidence(pair));
    }
}

/// The same-recording pairs below `SPARSE_SHARE` of the median
/// same-recording pair's hits per second, and that median. Pairs the
/// owner has judged the same are not listed again.
fn sparse_joins(pairs: &[Pair]) -> (f64, Vec<&Pair>) {
    let mut densities: Vec<f64> = pairs
        .iter()
        .filter(|pair| pair.same_recording)
        .map(Pair::hits_per_second)
        .collect();
    densities.sort_by(f64::total_cmp);
    let median = densities
        .get(densities.len() / 2)
        .copied()
        .unwrap_or_default();
    let sparse = pairs
        .iter()
        .filter(|pair| {
            pair.same_recording
                && pair.owner_verdict.is_none()
                && pair.hits_per_second() < SPARSE_SHARE * median
        })
        .collect();
    (median, sparse)
}

/// A pair's evidence as the reports print it: `(91%, 246 hits, 0.7 hits/s)`.
fn evidence(pair: &Pair) -> String {
    format!(
        "({:.0}%, {} hits, {:.1} hits/s)",
        100.0 * pair.coverage,
        pair.hits,
        pair.hits_per_second()
    )
}

/// Prints the clusters with duplicates that one set has and the other does
/// not.
pub fn print_differences(name: &str, clusters: &Clusters, other_name: &str, other: &Clusters) {
    let only = |a: &Clusters, b: &Clusters| -> Vec<String> {
        a.duplicates
            .iter()
            .filter(|members| !b.duplicates.contains(members))
            .map(|members| members.join("  |  "))
            .collect()
    };
    let (ours, theirs) = (only(clusters, other), only(other, clusters));
    println!(
        "against {other_name}: {} clusters the same, {} only in {name}, {} only in {other_name}",
        clusters.duplicates.len() - ours.len(),
        ours.len(),
        theirs.len()
    );
    for members in ours {
        println!("  only in {name}: {members}");
    }
    for members in theirs {
        println!("  only in {other_name}: {members}");
    }
}

/// Follows parent links up to the representative of `node`'s set.
fn root<'a>(parent: &BTreeMap<&'a str, &'a str>, mut node: &'a str) -> &'a str {
    while let Some(&up) = parent.get(node).filter(|&&up| up != node) {
        node = up;
    }
    node
}

/// The clusters that same-recording links make, with the owner's verdicts
/// checked against them, and the links cut on the way. Clusters are the
/// transitive closure of the links, so a `different` verdict, which removes
/// only its own pair's link, does not keep two files apart when a third
/// file links to both. Such a contradiction is an error naming the chain,
/// for the owner to judge the wrong link, unless `contradictions` is
/// `CutSparsest`: then the chain's measured join with the fewest hits per
/// second is cut, never a link the owner judged the same, and the next
/// chain is looked for.
fn clusters_of<'a>(
    links: impl IntoIterator<Item = (&'a str, &'a str)>,
    verdicts: &Verdicts,
    pairs: &[Pair],
    contradictions: Contradictions,
) -> Result<(Vec<Vec<String>>, Vec<CutLink>), String> {
    let mut links: Vec<(&str, &str)> = links.into_iter().collect();
    let mut cut_links = Vec::new();
    loop {
        let chains: Vec<((&str, &str), Vec<&str>)> = verdicts
            .judged_different()
            .filter_map(|(a, b)| Some(((a, b), chain(&links, a, b)?)))
            .collect();
        let Some(((a, b), first)) = chains.first() else {
            return Ok((merge(links), cut_links));
        };
        let sparsest = (contradictions == Contradictions::CutSparsest)
            .then(|| sparsest_join(first, verdicts, pairs))
            .flatten();
        let Some(pair) = sparsest else {
            let named: Vec<String> = match contradictions {
                Contradictions::Stop => chains,
                Contradictions::CutSparsest => vec![chains[0].clone()],
            }
            .into_iter()
            .map(|((a, b), chain)| {
                format!(
                    "{a} ~ {b} is judged different, but joins link them:\n{}",
                    describe(&chain, verdicts, pairs)
                )
            })
            .collect();
            return Err(format!(
                "{}\nA cluster holds every file a chain of joins reaches, so a `different` verdict alone cannot keep these files apart. Listen to the links of each chain, judge the wrong one `different` in the verdict file (or correct the verdict), and run `clusters` again.",
                named.join("\n")
            ));
        };
        let link = ordered(&pair.query, &pair.found);
        links.retain(|&(x, y)| ordered(x, y) != link);
        cut_links.push(CutLink {
            query: pair.query.clone(),
            found: pair.found.clone(),
            judged_different: [(*a).to_owned(), (*b).to_owned()],
        });
    }
}

/// The measured same-recording pair with the fewest hits per second among
/// a chain's links, leaving out links the owner judged the same.
fn sparsest_join<'a>(chain: &[&str], verdicts: &Verdicts, pairs: &'a [Pair]) -> Option<&'a Pair> {
    chain
        .windows(2)
        .filter(|step| verdicts.verdict(step[0], step[1]) != Some(true))
        .filter_map(|step| {
            pairs.iter().find(|pair| {
                pair.same_recording
                    && ordered(&pair.query, &pair.found) == ordered(step[0], step[1])
            })
        })
        .min_by(|x, y| x.hits_per_second().total_cmp(&y.hits_per_second()))
}

/// The files on a shortest chain of links from `from` to `to`, both ends
/// included; `None` when no chain joins them.
fn chain<'a>(links: &[(&'a str, &'a str)], from: &'a str, to: &'a str) -> Option<Vec<&'a str>> {
    let mut neighbours: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for &(a, b) in links {
        neighbours.entry(a).or_default().push(b);
        neighbours.entry(b).or_default().push(a);
    }
    let mut reached_from: BTreeMap<&str, &str> = BTreeMap::from([(from, from)]);
    let mut queue = VecDeque::from([from]);
    while let Some(file) = queue.pop_front() {
        if file == to {
            let mut chain = vec![to];
            let mut file = to;
            while file != from {
                file = reached_from[file];
                chain.push(file);
            }
            chain.reverse();
            return Some(chain);
        }
        for &next in neighbours.get(file).into_iter().flatten() {
            if !reached_from.contains_key(next) {
                reached_from.insert(next, file);
                queue.push_back(next);
            }
        }
    }
    None
}

/// A chain, one file per line, each link with what made it: the owner's
/// `same` verdict, a measured pair's evidence, or a corpus cluster.
fn describe(chain: &[&str], verdicts: &Verdicts, pairs: &[Pair]) -> String {
    let mut lines = vec![format!("    {}", chain[0])];
    for step in chain.windows(2) {
        let &[previous, file] = step else {
            continue;
        };
        let made_by = if verdicts.verdict(previous, file) == Some(true) {
            String::from("(judged the same)")
        } else {
            pairs
                .iter()
                .find(|pair| {
                    pair.same_recording
                        && ordered(&pair.query, &pair.found) == ordered(previous, file)
                })
                .map_or_else(|| String::from("(one corpus cluster)"), evidence)
        };
        lines.push(format!("  = {file} {made_by}"));
    }
    lines.join("\n")
}

/// Transitive closure of links between paths, such as same-recording pairs
/// (union-find over paths).
fn merge<'a>(links: impl IntoIterator<Item = (&'a str, &'a str)>) -> Vec<Vec<String>> {
    let mut parent: BTreeMap<&str, &str> = BTreeMap::new();
    for (query, found) in links {
        for node in [query, found] {
            parent.entry(node).or_insert(node);
        }
        let (a, b) = (root(&parent, query), root(&parent, found));
        if a != b {
            parent.insert(a.max(b), a.min(b));
        }
    }
    let mut clusters: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for &node in parent.keys() {
        clusters
            .entry(root(&parent, node))
            .or_default()
            .push(node.to_owned());
    }
    clusters.into_values().collect()
}

#[cfg(test)]
mod tests {
    use gunfinger_core::confidence::Evidence;
    use gunfinger_core::index::AssetId;
    use gunfinger_core::library::Timestamp;
    use gunfinger_core::speed::SpeedRatio;
    use gunfinger_core::store::RecordHeader;

    use super::*;

    #[test]
    fn the_alignment_covering_more_counts() {
        let mut drifting = pair("rip.mp3", "upload.m4a", false);
        drifting.coverage = 0.97;
        let mut one_rung = drifting.clone();
        one_rung.coverage = 0.63;

        let kept = better_of(drifting.clone(), Some(one_rung.clone()));
        assert_eq!((kept.coverage, kept.ladder_coverage), (0.97, Some(0.97)));

        let kept = better_of(one_rung, Some(drifting));
        assert_eq!((kept.coverage, kept.ladder_coverage), (0.97, Some(0.63)));
    }

    #[test]
    fn each_file_keeps_its_strongest_pair() {
        let mut weaker = pair("q.mp3", "f.mp3", false);
        weaker.coverage = 0.65;
        weaker.hits = 2_035;
        let stronger = Pair {
            hits: 2_540,
            ..weaker.clone()
        };
        let other = pair("q.mp3", "g.mp3", false);

        let kept = strongest_per_file(vec![weaker, other, stronger]);

        let kept: Vec<(&str, u32)> = kept
            .iter()
            .map(|pair| (pair.found.as_str(), pair.hits))
            .collect();
        assert_eq!(kept, [("f.mp3", 2_540), ("g.mp3", 0)]);
    }

    #[test]
    fn coverage_is_counted_in_the_shorter_files_seconds() {
        // A rip of 400 s against its upload 4% fast, 384.6 s long; the
        // alignment spans both whole, and only the first half of the rip.
        let whole = Detection {
            asset: AssetId(0),
            start_seconds: 0.0,
            end_seconds: 400.0,
            track_start_seconds: 0.0,
            track_end_seconds: 384.6,
            speed: SpeedRatio(0.9615),
            playback: Playback::Turntable,
            evidence: Evidence::new(40, 5_000),
        };
        let half = Detection {
            end_seconds: 200.0,
            track_end_seconds: 192.3,
            ..whole.clone()
        };
        let rip_searched = super::pair("rip", "upload".into(), (400.0, 384.6), &whole);
        assert!((rip_searched.coverage - 1.0).abs() < 1e-9);
        let rip_searched = super::pair("rip", "upload".into(), (400.0, 384.6), &half);
        assert!((rip_searched.coverage - 0.5).abs() < 1e-9);
        let upload_searched = Detection {
            start_seconds: 0.0,
            end_seconds: 384.6,
            track_start_seconds: 0.0,
            track_end_seconds: 400.0,
            speed: SpeedRatio(1.04),
            ..whole
        };
        let upload_searched = super::pair("upload", "rip".into(), (384.6, 400.0), &upload_searched);
        assert!((upload_searched.coverage - 1.0).abs() < 1e-9);
    }

    fn pair(query: &str, found: &str, same_recording: bool) -> Pair {
        Pair {
            query: query.to_owned(),
            found: found.to_owned(),
            coverage: 0.0,
            speed: 1.0,
            hits: 0,
            same_recording,
            query_start_seconds: 0.0,
            query_end_seconds: 0.0,
            found_start_seconds: 0.0,
            found_end_seconds: 0.0,
            owner_verdict: None,
            ladder_coverage: None,
        }
    }

    #[test]
    fn the_owners_verdicts_override_the_coverage_rule() {
        let verdicts = Verdicts::parse(
            "# header\n\nsame\tb.mp3\ta.mp3\ndifferent\tc.mp3\td.mp3\nsame\tx.mp3\ty.mp3\n",
        )
        .unwrap();
        let mut pairs = vec![
            pair("a.mp3", "b.mp3", false),
            pair("c.mp3", "d.mp3", true),
            pair("e.mp3", "f.mp3", true),
        ];

        verdicts.apply(&mut pairs);
        let clusters = clusters_of(
            verdicts.links(&pairs),
            &verdicts,
            &pairs,
            Contradictions::Stop,
        )
        .unwrap()
        .0;

        assert_eq!(pairs[0].owner_verdict, Some(true));
        assert!(!pairs[1].same_recording);
        assert_eq!(pairs[2].owner_verdict, None);
        assert_eq!(
            clusters,
            [
                vec!["a.mp3", "b.mp3"],
                vec!["e.mp3", "f.mp3"],
                vec!["x.mp3", "y.mp3"]
            ]
        );
    }

    fn asset(path: &str) -> Asset {
        Asset {
            path: path.to_owned(),
            size: 1,
            modified: Timestamp {
                seconds: 0,
                nanos: 0,
            },
        }
    }

    fn verdicts(text: &str) -> Verdicts {
        Verdicts {
            file: PathBuf::from("verdicts.txt"),
            ..Verdicts::parse(text).unwrap()
        }
    }

    #[test]
    fn named_files_of_the_other_library_are_queries_unless_they_copy_a_corpus_file() {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-other-queries-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let assets = [
            asset("rips/a.mp3"),
            asset("rips/b.mp3"),
            asset("rips/c.mp3"),
        ];
        for asset in &assets {
            let record = PeakRecord {
                header: RecordHeader {
                    profile: Profile::CURRENT.id(),
                    source: asset.clone(),
                    duration_seconds: 10.0,
                },
                peaks: Vec::new(),
            };
            store.save(&record).unwrap();
        }
        let files = BTreeSet::from([
            String::from("second-library/rips/a.mp3"),
            String::from("second-library/rips/b.mp3"),
        ]);
        let copies = BTreeSet::from([String::from("rips/b.mp3")]);

        let queries = other_queries(&store, &assets, &files, &copies).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        let names: Vec<&str> = queries
            .iter()
            .map(|record| record.header.source.path.as_str())
            .collect();
        assert_eq!(names, ["second-library/rips/a.mp3"]);
    }

    #[test]
    fn a_corpus_run_takes_no_verdict_on_another_librarys_files() {
        let verdicts = verdicts(
            "# header\nsame\ta.mp3\tb.mp3\nsame\ta.mp3\tsecond-library/upload.opus\ndifferent\tc.mp3\tsecond-library/revision.opus\n",
        );
        let corpus = Library {
            assets: vec![asset("a.mp3"), asset("b.mp3"), asset("c.mp3")],
            ..Library::default()
        };

        let (verdicts, outside) = verdicts.on(&Searched::corpus(&corpus));
        let pairs = [pair("c.mp3", "a.mp3", false)];
        let clusters = clusters_of(
            verdicts.links(&pairs),
            &verdicts,
            &pairs,
            Contradictions::Stop,
        )
        .unwrap()
        .0;

        assert_eq!(clusters, [vec!["a.mp3", "b.mp3"]]);
        assert_eq!(
            outside,
            [
                "verdicts.txt, line 3: second-library/upload.opus is not a file of the libraries this run searches; the verdict adds nothing",
                "verdicts.txt, line 4: second-library/revision.opus is not a file of the libraries this run searches; the verdict adds nothing",
            ]
        );
    }

    #[test]
    fn a_verdict_never_links_a_file_the_ignore_file_leaves_out() {
        // The other store still holds a record of the mixed CD's track.
        let verdicts = verdicts(
            "same\tvip.mp3\tsecond-library/Mixed CD/CD1/02-remix.mp3\nsame\tvip.mp3\tsecond-library/upload.m4a\n",
        );
        let corpus = Library {
            assets: vec![asset("vip.mp3")],
            ..Library::default()
        };
        let other = [asset("Mixed CD/CD1/02-remix.mp3"), asset("upload.m4a")];
        let ignore = IgnoreFile {
            path: PathBuf::from("/music/.gunfingerignore"),
            patterns: gunfinger_core::library::ignore::parse("# mixed CDs\n/Mixed CD/\n").unwrap(),
        };
        let searched = Searched::corpus(&corpus).with_other(&other, Some(ignore));

        let (verdicts, outside) = verdicts.on(&searched);
        let clusters = clusters_of(verdicts.links(&[]), &verdicts, &[], Contradictions::Stop)
            .unwrap()
            .0;

        assert_eq!(clusters, [vec!["second-library/upload.m4a", "vip.mp3"]]);
        assert_eq!(
            outside,
            [
                "verdicts.txt, line 1: second-library/Mixed CD/CD1/02-remix.mp3 is left out by /music/.gunfingerignore, line 2 (`/Mixed CD/`); the verdict adds nothing"
            ]
        );
    }

    #[test]
    fn a_verdict_line_needs_a_verdict_and_two_paths() {
        assert!(Verdicts::parse("maybe\ta.mp3\tb.mp3\n").is_err());
        assert!(Verdicts::parse("same\ta.mp3\n").is_err());
        assert!(Verdicts::parse("# only comments\n").is_ok());
    }

    #[test]
    fn a_pair_cannot_be_judged_both_ways() {
        assert!(Verdicts::parse("same\ta.mp3\tb.mp3\ndifferent\tb.mp3\ta.mp3\n").is_err());
        assert!(Verdicts::parse("same\ta.mp3\tb.mp3\nsame\tb.mp3\ta.mp3\n").is_ok());
    }

    #[test]
    fn a_different_verdict_holds_against_a_chain_through_a_third_file() {
        // An original, a rip of it, and a revision the rip's search joined
        // with sparse evidence; the owner hears the revision as different.
        let verdicts = Verdicts::parse("different\toriginal.mp3\trevision.opus\n").unwrap();
        let mut rip = pair("original.mp3", "rip.m4a", true);
        rip.coverage = 0.98;
        rip.hits = 24_774;
        rip.query_end_seconds = 400.0;
        let mut revision = pair("revision.opus", "rip.m4a", true);
        revision.coverage = 0.91;
        revision.hits = 246;
        revision.query_end_seconds = 330.0;
        let pairs = [rip, revision];

        let error = clusters_of(
            same_recording(&pairs),
            &verdicts,
            &pairs,
            Contradictions::Stop,
        )
        .unwrap_err();

        assert!(error.starts_with(
            "original.mp3 ~ revision.opus is judged different, but joins link them:\n    \
             original.mp3\n  = rip.m4a (98%, 24774 hits, 61.9 hits/s)\n  \
             = revision.opus (91%, 246 hits, 0.7 hits/s)\n"
        ));
        let without_verdict = clusters_of(
            same_recording(&pairs),
            &Verdicts::default(),
            &pairs,
            Contradictions::Stop,
        )
        .unwrap()
        .0;
        assert_eq!(
            without_verdict,
            [vec!["original.mp3", "revision.opus", "rip.m4a"]]
        );
    }

    #[test]
    fn cutting_the_sparsest_join_keeps_files_judged_different_apart() {
        let verdicts = Verdicts::parse(
            "different\toriginal.mp3\trevision.opus\nsame\toriginal.mp3\tupload.m4a\n",
        )
        .unwrap();
        let joined = |query: &str, found: &str, hits: u32| Pair {
            hits,
            query_end_seconds: 300.0,
            ..pair(query, found, true)
        };
        let mut pairs = vec![
            joined("original.mp3", "rip.m4a", 6_000),
            joined("rip.m4a", "revision.opus", 400),
            joined("revision.opus", "remaster.opus", 5_000),
            joined("upload.m4a", "revision.opus", 300),
        ];
        verdicts.apply(&mut pairs);

        let (clusters, cut) = clusters_of(
            verdicts.links(&pairs),
            &verdicts,
            &pairs,
            Contradictions::CutSparsest,
        )
        .unwrap();

        assert_eq!(
            clusters,
            [
                vec!["original.mp3", "rip.m4a", "upload.m4a"],
                vec!["remaster.opus", "revision.opus"]
            ]
        );
        let cut: Vec<(&str, &str)> = cut
            .iter()
            .map(|link| (link.query.as_str(), link.found.as_str()))
            .collect();
        assert_eq!(
            cut,
            [
                ("rip.m4a", "revision.opus"),
                ("upload.m4a", "revision.opus")
            ]
        );
    }

    #[test]
    fn an_earlier_runs_pairs_lose_their_verdicts_and_their_pruned_files() {
        let corpus = Library {
            assets: vec![asset("a.mp3")],
            ..Library::default()
        };
        let other = [asset("rip.m4a"), asset("upload.opus")];
        let searched = Searched::corpus(&corpus).with_other(&other, None);
        let judged = Pair {
            coverage: 0.6,
            owner_verdict: Some(true),
            ..pair("a.mp3", "second-library/upload.opus", true)
        };
        let cut = Pair {
            coverage: 0.95,
            ..pair("a.mp3", "second-library/rip.m4a", false)
        };
        let pruned = pair("a.mp3", "second-library/gone.mp3", true);

        let verdicts = Verdicts::parse("different\ta.mp3\tsecond-library/upload.opus\n").unwrap();
        let (pairs, dropped) = as_measured(vec![judged, cut, pruned], &searched, &verdicts);

        assert_eq!(dropped, 1);
        let state: Vec<(&str, bool, Option<bool>)> = pairs
            .iter()
            .map(|pair| (pair.found.as_str(), pair.same_recording, pair.owner_verdict))
            .collect();
        assert_eq!(
            state,
            [
                ("second-library/upload.opus", false, Some(false)),
                ("second-library/rip.m4a", true, None)
            ]
        );
    }

    #[test]
    fn a_chain_of_links_judged_the_same_is_never_cut() {
        let verdicts =
            Verdicts::parse("different\ta.mp3\tc.mp3\nsame\ta.mp3\tb.mp3\nsame\tb.mp3\tc.mp3\n")
                .unwrap();

        let error = clusters_of(
            verdicts.links(&[]),
            &verdicts,
            &[],
            Contradictions::CutSparsest,
        )
        .unwrap_err();

        assert!(error.starts_with("a.mp3 ~ c.mp3 is judged different, but joins link them:"));
    }

    #[test]
    fn a_different_verdict_on_files_in_separate_clusters_passes() {
        let verdicts = Verdicts::parse("different\ta.mp3\tc.mp3\n").unwrap();
        let pairs = [pair("a.mp3", "b.mp3", true), pair("c.mp3", "d.mp3", true)];

        let clusters = clusters_of(
            same_recording(&pairs),
            &verdicts,
            &pairs,
            Contradictions::Stop,
        )
        .unwrap()
        .0;

        assert_eq!(clusters, [vec!["a.mp3", "b.mp3"], vec!["c.mp3", "d.mp3"]]);
    }

    #[test]
    fn joins_far_sparser_than_the_median_are_listed() {
        let joined = |found: &str, hits: u32| Pair {
            hits,
            query_end_seconds: 100.0,
            ..pair("q.mp3", found, true)
        };
        let mut confirmed = joined("confirmed.mp3", 50);
        confirmed.owner_verdict = Some(true);
        let pairs = [
            joined("a.mp3", 1_800),
            joined("b.mp3", 2_000),
            joined("c.mp3", 2_200),
            joined("sparse.mp3", 70),
            confirmed,
            Pair {
                hits: 10,
                ..pair("q.mp3", "apart.mp3", false)
            },
        ];

        let (median, sparse) = sparse_joins(&pairs);

        assert!((median - 18.0).abs() < 1e-9);
        let sparse: Vec<&str> = sparse.iter().map(|pair| pair.found.as_str()).collect();
        assert_eq!(sparse, ["sparse.mp3"]);
    }

    #[test]
    fn same_recording_pairs_merge_transitively() {
        let pairs = [
            pair("c.mp3", "a.mp3", true),
            pair("d.mp3", "e.mp3", false),
            pair("b.mp3", "c.mp3", true),
            pair("x.mp3", "y.mp3", true),
        ];

        let clusters = merge(same_recording(&pairs));

        assert_eq!(
            clusters,
            [vec!["a.mp3", "b.mp3", "c.mp3"], vec!["x.mp3", "y.mp3"]]
        );
    }

    #[test]
    fn rips_in_another_library_join_the_corpus_clusters() {
        let corpus = Clusters {
            criterion: String::from("corpus"),
            duplicates: vec![vec!["a.mp3".to_owned(), "b.mp3".to_owned()]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let pairs = vec![
            pair("a.mp3", "second-library/x/a.mp3", true),
            pair("second-library/x/a.mp3", "second-library/y/a-rip.mp3", true),
            pair("c.mp3", "second-library/x/c.mp3", true),
            pair("c.mp3", "second-library/x/c-remix.mp3", false),
        ];

        let merged = merged(
            &corpus,
            pairs,
            &Verdicts::default(),
            "/music",
            Contradictions::Stop,
        )
        .unwrap();

        assert_eq!(
            merged.duplicates,
            [
                vec![
                    "a.mp3",
                    "b.mp3",
                    "second-library/x/a.mp3",
                    "second-library/y/a-rip.mp3"
                ],
                vec!["c.mp3", "second-library/x/c.mp3"],
            ]
        );
        assert!(merged.criterion.starts_with("corpus; "));
        assert_eq!(merged.pairs.len(), 4);
    }

    #[test]
    fn an_asset_without_duplicates_is_its_own_cluster() {
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec!["a.mp3".to_owned(), "b.mp3".to_owned()]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        assert_eq!(clusters.cluster_of("b.mp3").len(), 2);
        assert_eq!(
            clusters.cluster_of("z.mp3"),
            BTreeSet::from(["z.mp3".to_owned()])
        );
    }
}

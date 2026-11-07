//! Two records of one file (brief 82, the user's idea of 2026-10-31): the
//! record as ripped and a second one, such as the repaired form of a file
//! with an inverted channel (`variant-store --repair-inverted
//! --repaired-only`). Each record is its own asset of the index, so chains
//! and plays never join the two. At the file, the plays of its two records
//! are merged only by keeping the stronger (`strongest`): a file's level is
//! the higher of its records' levels, and their hits are never added.

use std::collections::BTreeSet;
use std::path::Path;
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::index::Index;
use gunfinger_core::library::Asset;
use gunfinger_core::plays::{self, Play};
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::Serialize;

use crate::matching::Matching;
use crate::padding::{OTHER_LIBRARY_PREFIX, OtherLibrary, current_sources};

/// In the index, a file's second record is named `second-record/<path>`,
/// where `<path>` is its first record's name there.
pub const SECOND_RECORD_PREFIX: &str = "second-record/";

pub fn second_record_path(path: &str) -> String {
    format!("{SECOND_RECORD_PREFIX}{path}")
}

/// Which of its file's records an asset of the index is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Record {
    /// Today's record of the file, as ripped.
    First,
    Second,
}

/// The file an asset of the index stands for (its first record's name),
/// and which of the file's records it is.
pub fn file_of(path: &str) -> (&str, Record) {
    match path.strip_prefix(SECOND_RECORD_PREFIX) {
        Some(file) => (file, Record::Second),
        None => (path, Record::First),
    }
}

/// The second records of some of the other library's files, named as the
/// index names them.
pub struct SecondRecords {
    records: Vec<PeakRecord>,
}

impl SecondRecords {
    /// Every current record of the store at `dir`, each the second record
    /// of the other library's file at the same path, which `other` must
    /// hold.
    pub fn load(dir: &Path, other: &OtherLibrary) -> Result<SecondRecords, String> {
        let store = PeakStore::open(dir).map_err(|error| error.to_string())?;
        let assets = current_sources(&store, crate::available_parallelism())?;
        let records = assets
            .iter()
            .map(|asset| {
                let first = format!("{OTHER_LIBRARY_PREFIX}{}", asset.path);
                if !other.holds(&first) {
                    return Err(format!(
                        "{} holds a record of {}, which the other library's index does not hold",
                        dir.display(),
                        asset.path
                    ));
                }
                let mut record = store
                    .load(asset, &Profile::CURRENT)
                    .map_err(|error| error.to_string())?;
                record.header.source.path = second_record_path(&first);
                Ok(record)
            })
            .collect::<Result<Vec<_>, String>>()?;
        Ok(SecondRecords { records })
    }

    pub fn records(&self) -> &[PeakRecord] {
        &self.records
    }

    /// Their sources as the index names them.
    pub fn sources(&self) -> impl Iterator<Item = Asset> + '_ {
        self.records
            .iter()
            .map(|record| record.header.source.clone())
    }

    /// The files they are second records of, as the index names their
    /// first records.
    pub fn files(&self) -> BTreeSet<String> {
        self.records
            .iter()
            .map(|record| file_of(&record.header.source.path).0.to_owned())
            .collect()
    }

    /// `excluded` with the second record of each file it names: leaving a
    /// file out leaves out both its records.
    pub fn with_second_records(&self, excluded: &BTreeSet<String>) -> BTreeSet<String> {
        let mut both = excluded.clone();
        both.extend(
            self.files()
                .into_iter()
                .filter(|file| excluded.contains(file))
                .map(|file| second_record_path(&file)),
        );
        both
    }
}

/// One play of one record, as `second-records` reports it.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RecordPlay {
    /// The asset's name in the index.
    pub asset: String,
    /// The file it is a record of.
    pub file: String,
    pub record: Record,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub confidence: Level,
    /// The hits of its strongest segment.
    pub hits: u32,
    pub windows: u32,
    pub speed: f64,
}

/// `Confidence`, for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Level {
    Weak,
    Possible,
    Confident,
}

impl From<Confidence> for Level {
    fn from(confidence: Confidence) -> Level {
        match confidence {
            Confidence::Weak => Level::Weak,
            Confidence::Possible => Level::Possible,
            Confidence::Confident => Level::Confident,
        }
    }
}

impl RecordPlay {
    pub fn of(index: &Index, play: &Play) -> RecordPlay {
        let asset = index.asset(play.asset).path.clone();
        let (file, record) = file_of(&asset);
        let strongest = play
            .segments()
            .iter()
            .max_by_key(|segment| segment.evidence.hits)
            .map(|segment| segment.evidence);
        RecordPlay {
            file: file.to_owned(),
            record,
            start_seconds: play.start_seconds(),
            end_seconds: play.end_seconds(),
            confidence: play.confidence().into(),
            hits: strongest.map_or(0, |evidence| evidence.hits),
            windows: strongest.map_or(0, |evidence| evidence.windows),
            speed: play.speed().0,
            asset,
        }
    }

    /// The order in which one of a file's plays outweighs another: the
    /// higher level, then more hits, then the first record.
    fn strength(&self) -> (Level, u32, std::cmp::Reverse<Record>) {
        (self.confidence, self.hits, std::cmp::Reverse(self.record))
    }

    fn overlaps(&self, other: &RecordPlay) -> bool {
        self.start_seconds < other.end_seconds && other.start_seconds < self.end_seconds
    }
}

/// The plays at the file: of a file's plays from its two records that
/// overlap in query time, only the stronger is kept. Plays are taken
/// strongest first, and one is dropped only when a kept play of the
/// file's other record overlaps it, so a play beaten by a play that is
/// itself dropped stays. Plays of one record never remove each other, and
/// nothing is added: a file is confident only where one of its records is.
pub fn strongest(plays: &[RecordPlay]) -> Vec<RecordPlay> {
    let mut order: Vec<usize> = (0..plays.len()).collect();
    order.sort_by(|&a, &b| plays[b].strength().cmp(&plays[a].strength()));
    let mut kept = vec![false; plays.len()];
    for &candidate in &order {
        let play = &plays[candidate];
        kept[candidate] = !order.iter().any(|&other| {
            kept[other]
                && plays[other].file == play.file
                && plays[other].record != play.record
                && plays[other].overlaps(play)
        });
    }
    plays
        .iter()
        .zip(kept)
        .filter(|(_, kept)| *kept)
        .map(|(play, _)| play.clone())
        .collect()
}

/// Which records an arm's index holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Arm {
    /// Today's index: no second records.
    First,
    /// Every first record and every second one.
    Both,
    /// The second records in place of their files' first ones.
    Second,
}

impl Arm {
    pub fn name(self) -> &'static str {
        match self {
            Arm::First => "first",
            Arm::Both => "both",
            Arm::Second => "second",
        }
    }
}

/// What `second-records` writes.
#[derive(Serialize)]
pub struct Report {
    pub records: Arm,
    pub assets: usize,
    pub postings: usize,
    /// The files with a second record, and those records' postings.
    pub second_record_files: Vec<String>,
    pub second_record_postings: usize,
    pub index_seconds: f64,
    pub queries: Vec<Query>,
}

#[derive(Serialize)]
pub struct Query {
    pub file: String,
    pub seconds: f64,
    pub wall_seconds: f64,
    /// Every play at least possible, per record.
    pub plays: Vec<RecordPlay>,
    /// The plays at the file (`strongest`).
    pub files: Vec<RecordPlay>,
}

/// Which records `search` indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Records {
    /// Every first record and every second one.
    Both,
    /// The second records in place of their files' first ones.
    Second,
}

/// The index of one arm: `index` (whose second records are `second`) as
/// `records` says. Without second records it is today's index.
pub fn arm(index: Index, second: Option<&SecondRecords>, records: Records) -> (Index, Arm) {
    match (second, records) {
        (None, _) => (index, Arm::First),
        (Some(_), Records::Both) => (index, Arm::Both),
        (Some(second), Records::Second) => (index.without_assets(&second.files()), Arm::Second),
    }
}

/// Searches each of `queries` against `index`, the index of the arm
/// `name` with its lists set aside, and reports every play per record and
/// at the file; the caller fills in the index's time.
pub fn search(
    index: &Index,
    name: Arm,
    second: Option<&SecondRecords>,
    queries: &[std::path::PathBuf],
    matching: &Matching,
    ladder: &[Rung],
    jobs: usize,
) -> Result<Report, String> {
    let files = second.map(SecondRecords::files).unwrap_or_default();
    let second_record_postings = match second {
        Some(second) => Index::build(second.records())
            .map_err(|error| error.to_string())?
            .posting_count(),
        None => 0,
    };
    let profile = matching.profile();
    let queries = queries
        .iter()
        .map(|path| {
            let started = Instant::now();
            let audio = matching.decode_query(path)?;
            let detections = matching.search_audio(index, &audio, ladder, jobs);
            let plays: Vec<RecordPlay> = plays::group(&detections)
                .iter()
                .map(|play| RecordPlay::of(index, play))
                .collect();
            Ok(Query {
                file: path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default(),
                seconds: audio.signal().len() as f64 / f64::from(profile.sample_rate),
                wall_seconds: started.elapsed().as_secs_f64(),
                files: strongest(&plays),
                plays,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Report {
        records: name,
        assets: index.assets().len(),
        postings: index.posting_count(),
        second_record_files: files.into_iter().collect(),
        second_record_postings,
        index_seconds: 0.0,
        queries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(asset: &str, start: f64, end: f64, confidence: Level, hits: u32) -> RecordPlay {
        let (file, record) = file_of(asset);
        RecordPlay {
            asset: asset.to_owned(),
            file: file.to_owned(),
            record,
            start_seconds: start,
            end_seconds: end,
            confidence,
            hits,
            windows: 4,
            speed: 1.0,
        }
    }

    const FILE: &str = "second-library/a.mp3";

    fn record(path: &str) -> PeakRecord {
        PeakRecord {
            header: gunfinger_core::store::RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: gunfinger_core::library::Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 10.0,
            },
            peaks: Vec::new(),
        }
    }

    fn second() -> String {
        second_record_path(FILE)
    }

    #[test]
    fn a_second_record_names_its_file() {
        assert_eq!(
            second(),
            "second-record/second-library/a.mp3",
            "the prefix goes before the first record's name"
        );
        assert_eq!(file_of(&second()), (FILE, Record::Second));
        assert_eq!(file_of(FILE), (FILE, Record::First));
        assert_eq!(crate::padding::original_path(&second()), FILE);
    }

    #[test]
    fn the_stronger_record_is_kept_where_both_play() {
        let plays = [
            play(FILE, 0.0, 30.0, Level::Possible, 150),
            play(&second(), 2.0, 30.0, Level::Confident, 900),
        ];
        assert_eq!(strongest(&plays), vec![plays[1].clone()]);
    }

    #[test]
    fn hits_of_the_two_records_are_never_added() {
        // Neither record is confident, so the file is not, although their
        // hits together would pass the rule's 240.
        let plays = [
            play(FILE, 0.0, 30.0, Level::Possible, 150),
            play(&second(), 0.0, 30.0, Level::Possible, 140),
        ];
        let kept = strongest(&plays);
        assert_eq!(kept, vec![plays[0].clone()]);
        assert_eq!(kept[0].confidence, Level::Possible);
        assert_eq!(kept[0].hits, 150);
    }

    #[test]
    fn at_equal_strength_the_first_record_is_kept() {
        let plays = [
            play(&second(), 0.0, 30.0, Level::Confident, 500),
            play(FILE, 0.0, 30.0, Level::Confident, 500),
        ];
        assert_eq!(strongest(&plays), vec![plays[1].clone()]);
    }

    #[test]
    fn plays_apart_in_time_or_of_other_files_are_all_kept() {
        let plays = [
            play(FILE, 0.0, 30.0, Level::Confident, 500),
            play(&second(), 200.0, 230.0, Level::Possible, 100),
            play("second-library/b.mp3", 0.0, 30.0, Level::Possible, 100),
            play(FILE, 100.0, 130.0, Level::Possible, 120),
        ];
        assert_eq!(strongest(&plays), plays.to_vec());
    }

    #[test]
    fn a_play_beaten_only_by_a_dropped_play_is_kept() {
        // The second record's play beats the first record's play at 0-30 s
        // but loses to its play at 25-60 s, which does not overlap 0-30 s.
        let plays = [
            play(FILE, 0.0, 30.0, Level::Possible, 150),
            play(&second(), 20.0, 40.0, Level::Possible, 200),
            play(FILE, 25.0, 60.0, Level::Confident, 600),
        ];
        assert_eq!(strongest(&plays), vec![plays[0].clone(), plays[2].clone()]);
    }

    #[test]
    fn the_arms_index_both_records_or_the_second_alone() {
        let records = SecondRecords {
            records: vec![record(&second())],
        };
        let index = || {
            Index::build(&[
                record(FILE),
                record("second-library/b.mp3"),
                record(&second()),
            ])
            .unwrap()
        };
        let paths = |index: &Index| -> Vec<String> {
            index
                .assets()
                .iter()
                .map(|asset| asset.path.clone())
                .collect()
        };
        let (both, name) = arm(index(), Some(&records), Records::Both);
        assert_eq!((paths(&both).len(), name), (3, Arm::Both));
        let (alone, name) = arm(index(), Some(&records), Records::Second);
        assert_eq!(name, Arm::Second);
        assert_eq!(paths(&alone), ["second-library/b.mp3", second().as_str()]);
        let (today, name) = arm(index(), None, Records::Both);
        assert_eq!((paths(&today).len(), name), (3, Arm::First));
    }

    #[test]
    fn leaving_a_file_out_leaves_out_its_second_record() {
        let records = SecondRecords {
            records: vec![record(&second())],
        };
        let excluded = BTreeSet::from([FILE.to_owned(), String::from("other.mp3")]);
        assert_eq!(
            records.with_second_records(&excluded),
            BTreeSet::from([FILE.to_owned(), second(), String::from("other.mp3")])
        );
        assert_eq!(
            records.with_second_records(&BTreeSet::from([String::from("other.mp3")])),
            BTreeSet::from([String::from("other.mp3")])
        );
    }
}

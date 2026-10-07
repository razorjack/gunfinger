//! The content map between the corpus and another library it was drawn
//! from: each corpus file with the other library's files whose peak record
//! is identical, so the same audio. It is built from the two peak stores
//! alone, without decoding and without the other library mounted.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LibraryMap {
    /// The other library's root, as its store names it.
    pub other_library: Option<String>,
    pub corpus_records: usize,
    pub other_records: usize,
    /// The corpus files with a copy, by path.
    pub copies: Vec<Copies>,
    /// Corpus files with no identical record in the other store.
    pub without_copy: Vec<String>,
}

/// A corpus file's copies in the other library.
#[derive(Serialize, Deserialize)]
pub struct Copies {
    pub corpus: String,
    /// The copy the corpus file stands for in an index of both libraries:
    /// one of the same size if there is one.
    pub stands_for: String,
    /// Whether `stands_for` has the corpus file's size; a copy that differs
    /// only in its tags has another size and the same audio.
    pub same_size: bool,
    /// Further identical files in the other library.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub also: Vec<String>,
}

impl LibraryMap {
    pub fn load(path: &Path) -> Result<LibraryMap, String> {
        let text = std::fs::read_to_string(path).map_err(|error| {
            format!(
                "cannot read {} ({error}); run `gunfinger-eval --other-peaks-dir <store> map-library` first",
                path.display()
            )
        })?;
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))
    }

    /// The other library's files that corpus files stand for.
    pub fn stood_for(&self) -> BTreeSet<String> {
        self.copies
            .iter()
            .map(|copies| copies.stands_for.clone())
            .collect()
    }

    /// Every copy of a corpus file in the other library.
    pub fn copied(&self) -> BTreeSet<String> {
        self.copies
            .iter()
            .flat_map(|copies| std::iter::once(&copies.stands_for).chain(&copies.also))
            .cloned()
            .collect()
    }
}

/// Pairs every corpus file that has a current record with the files of
/// `other` (the store's current records) whose records are identical:
/// candidates have the same duration, read from the headers, and then the
/// same peaks.
pub fn build(
    library: &Library,
    store: &PeakStore,
    other: &PeakStore,
    other_assets: &[Asset],
) -> Result<LibraryMap, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let mut by_duration: BTreeMap<u64, Vec<&Asset>> = BTreeMap::new();
    for asset in other_assets {
        if let Some(header) = other.current_header(asset, &profile) {
            by_duration
                .entry(header.duration_seconds.to_bits())
                .or_default()
                .push(asset);
        }
    }
    let mut taken = BTreeSet::new();
    let mut copies = Vec::new();
    let mut without_copy = Vec::new();
    for record in &records {
        let source = &record.header.source;
        let candidates = by_duration
            .get(&record.header.duration_seconds.to_bits())
            .map_or(&[][..], Vec::as_slice);
        let mut identical: Vec<&Asset> = Vec::new();
        for &candidate in candidates {
            match other.load(candidate, &profile) {
                Ok(found) if found.peaks == record.peaks => identical.push(candidate),
                Ok(_) => {}
                Err(error) => eprintln!("left out: {error}"),
            }
        }
        // Same size first, then by path; a file already stood for goes last.
        identical.sort_by_key(|asset| {
            (
                taken.contains(&asset.path),
                asset.size != source.size,
                asset.path.clone(),
            )
        });
        let Some((first, rest)) = identical.split_first() else {
            without_copy.push(source.path.clone());
            continue;
        };
        taken.insert(first.path.clone());
        copies.push(Copies {
            corpus: source.path.clone(),
            stands_for: first.path.clone(),
            same_size: first.size == source.size,
            also: rest.iter().map(|asset| asset.path.clone()).collect(),
        });
    }
    Ok(LibraryMap {
        other_library: other.library().map_err(|error| error.to_string())?,
        corpus_records: records.len(),
        other_records: other_assets.len(),
        copies,
        without_copy,
    })
}

pub fn print_summary(map: &LibraryMap) {
    let several: Vec<&Copies> = map
        .copies
        .iter()
        .filter(|copies| !copies.also.is_empty())
        .collect();
    let other_size = map.copies.iter().filter(|copies| !copies.same_size).count();
    println!(
        "{} of {} corpus files have a copy among the {} records of {}",
        map.copies.len(),
        map.corpus_records,
        map.other_records,
        map.other_library.as_deref().unwrap_or("the other library")
    );
    println!(
        "{other_size} copies differ in size (tags), {} corpus files have several",
        several.len()
    );
    for copies in several {
        println!(
            "  several: {} = {} | {}",
            copies.corpus,
            copies.stands_for,
            copies.also.join(" | ")
        );
    }
    for path in &map.without_copy {
        println!("  no copy: {path}");
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use gunfinger_core::library::Timestamp;
    use gunfinger_core::peaks::Peak;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    fn asset(path: &str, size: u64) -> Asset {
        Asset {
            path: path.to_owned(),
            size,
            modified: Timestamp {
                seconds: 1_700_000_000,
                nanos: 0,
            },
        }
    }

    fn record(source: Asset, duration_seconds: f64, bin: f32) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source,
                duration_seconds,
            },
            peaks: vec![Peak {
                frame: 1.0,
                bin,
                magnitude: 10.0,
            }],
        }
    }

    #[test]
    fn copies_are_files_with_identical_records_same_size_first() {
        let dir = std::env::temp_dir().join(format!("gunfinger-map-{}", std::process::id()));
        let root = dir.join("corpus");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("a.mp3"), b"12345").unwrap();
        fs::write(root.join("b.mp3"), b"123").unwrap();
        let library = Library::scan(&root).unwrap();
        let store = PeakStore::open(&dir.join("peaks")).unwrap();
        store
            .save(&record(library.assets[0].clone(), 100.0, 50.0))
            .unwrap();
        store
            .save(&record(library.assets[1].clone(), 100.0, 60.0))
            .unwrap();
        let other = PeakStore::open(&dir.join("other")).unwrap();
        let other_assets = [
            asset("x/a-retagged.mp3", 9),
            asset("y/a.mp3", 5),
            asset("z/same-length.mp3", 3),
        ];
        for asset in &other_assets {
            let bin = if asset.path.contains("same-length") {
                70.0
            } else {
                50.0
            };
            other.save(&record(asset.clone(), 100.0, bin)).unwrap();
        }

        let map = build(&library, &store, &other, &other_assets).unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(map.copies.len(), 1);
        let copies = &map.copies[0];
        assert_eq!(copies.corpus, "a.mp3");
        assert_eq!(copies.stands_for, "y/a.mp3");
        assert!(copies.same_size);
        assert_eq!(copies.also, ["x/a-retagged.mp3"]);
        assert_eq!(map.without_copy, ["b.mp3"]);
    }
}

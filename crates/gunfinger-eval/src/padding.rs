//! A larger index for scale experiments: the library's records, then the
//! current records of a second library (a larger real library, when one
//! exists), then reversed copies of the library's records (`synthetic`).
//! The second library is either scanned (`--second-library`) or known from
//! its store alone (`--other-peaks-dir`, less the copies corpus files
//! stand for).
//!
//! The index is built in two passes over the same records in the same
//! order, reading the second library one record at a time and making each
//! copy as it is taken, so it needs little more memory than the index.
//!
//! With a second library, the index of the library and the whole second
//! library may be saved and loaded instead of built (ADR 0014); a run that
//! leaves files out removes them from it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use gunfinger_core::hash;
use gunfinger_core::index::Index;
use gunfinger_core::index::saved::{self, Provenance, Unusable};
use gunfinger_core::indexing::TrackLength;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakRecord, PeakStore};

use crate::rng::Rng;
use crate::synthetic;

/// What is added to the library's records.
#[derive(Default)]
pub struct Padding {
    /// Reversed copies of every record of the library.
    pub copies: usize,
    pub second: Option<SecondLibrary>,
    /// Where the index of the library and the whole second library is
    /// saved. Used only with a second library and no copies.
    pub saved: Option<SavedIndex>,
}

/// The saved index of the library and the whole second library (ADR
/// 0014), and whether a run may load it.
pub enum SavedIndex {
    /// Load it when it was built from what the run would index; otherwise
    /// build the index and save it over the file.
    Use(PathBuf),
    /// Build the index and save it over the file.
    Rebuild(PathBuf),
}

impl SavedIndex {
    fn path(&self) -> &Path {
        match self {
            SavedIndex::Use(path) | SavedIndex::Rebuild(path) => path,
        }
    }
}

/// The harness indexes records of any length; a saved index records this
/// range.
const ANY_LENGTH: TrackLength = TrackLength {
    min: Duration::ZERO,
    max: Duration::from_secs(24 * 3600),
};

/// In the index, the second library's paths start with this, so reports
/// tell its assets from the library's.
pub const SECOND_LIBRARY_PREFIX: &str = "second-library/";

/// Another library and its peak store, as `gunfinger index` wrote it.
pub struct SecondLibrary {
    store: PeakStore,
    /// The assets with a current peak record, in library order.
    assets: Vec<Asset>,
}

impl SecondLibrary {
    pub fn open(root: &Path, peaks_dir: &Path) -> Result<SecondLibrary, String> {
        let library = Library::scan(root)
            .map_err(|error| error.to_string())
            .and_then(crate::nothing_vanished)
            .map_err(|error| {
                format!(
                    "cannot read the second library at {}: {error}",
                    root.display()
                )
            })?;
        let store = PeakStore::open(peaks_dir).map_err(|error| error.to_string())?;
        store
            .check_library(root)
            .map_err(|error| error.to_string())?;
        let profile = Profile::CURRENT;
        let assets: Vec<Asset> = library
            .assets
            .into_iter()
            .filter(|asset| store.has_current(asset, &profile))
            .collect();
        if assets.is_empty() {
            return Err(format!(
                "{} holds no current peak records of {}; run `gunfinger index {} --peaks-dir {}`",
                peaks_dir.display(),
                root.display(),
                root.display(),
                peaks_dir.display()
            ));
        }
        Ok(SecondLibrary { store, assets })
    }

    /// The current records of `store`, without reading its library, less
    /// the files in `without` (paths in the store, unprefixed).
    pub fn from_store(
        store: PeakStore,
        without: &BTreeSet<String>,
    ) -> Result<SecondLibrary, String> {
        let (assets, problems) = store
            .current_sources(&Profile::CURRENT)
            .map_err(|error| error.to_string())?;
        for problem in &problems {
            eprintln!("left out: {problem}");
        }
        Ok(SecondLibrary {
            store,
            assets: assets
                .into_iter()
                .filter(|asset| !without.contains(&asset.path))
                .collect(),
        })
    }

    /// A seeded random choice of `count` of its assets, kept in library
    /// order: a smaller real library, for measuring against size.
    pub fn sampled(mut self, count: usize, seed: u64) -> SecondLibrary {
        let mut order: Vec<usize> = (0..self.assets.len()).collect();
        Rng::new(seed).shuffle(&mut order);
        let mut chosen = vec![false; self.assets.len()];
        for &index in order.iter().take(count) {
            chosen[index] = true;
        }
        let mut chosen = chosen.into_iter();
        self.assets.retain(|_| chosen.next().unwrap_or(false));
        self
    }

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    /// The record of the asset the index names `path` (prefixed); `None`
    /// when it is not one of this library's.
    pub fn record(&self, path: &str) -> Option<Result<PeakRecord, String>> {
        let path = path.strip_prefix(SECOND_LIBRARY_PREFIX)?;
        let asset = self.assets.iter().find(|asset| asset.path == path)?;
        Some(
            self.store
                .load(asset, &Profile::CURRENT)
                .map_err(|error| error.to_string()),
        )
    }

    /// Its files as the index names them, their paths prefixed.
    fn prefixed_assets(&self) -> impl Iterator<Item = Asset> + '_ {
        self.assets.iter().map(|asset| Asset {
            path: format!("{SECOND_LIBRARY_PREFIX}{}", asset.path),
            ..asset.clone()
        })
    }

    /// Its records one at a time, their paths prefixed, leaving out those
    /// whose prefixed path is in `excluded`.
    fn records<'a>(
        &'a self,
        profile: &'a Profile,
        excluded: &'a BTreeSet<String>,
    ) -> impl Iterator<Item = Result<PeakRecord, String>> + 'a {
        self.assets
            .iter()
            .filter(|asset| !excluded.contains(&format!("{SECOND_LIBRARY_PREFIX}{}", asset.path)))
            .map(move |asset| {
                let mut record = self
                    .store
                    .load(asset, profile)
                    .map_err(|error| error.to_string())?;
                record.header.source.path =
                    format!("{SECOND_LIBRARY_PREFIX}{}", record.header.source.path);
                Ok(record)
            })
    }
}

impl Padding {
    /// The second library's assets.
    pub fn second_assets(&self) -> usize {
        self.second.as_ref().map_or(0, SecondLibrary::len)
    }

    /// The index of `records` followed by the padding, leaving out the
    /// files `excluded` names (the library's paths, and the second
    /// library's with their prefix): asset `i` is the `i`-th record not
    /// left out.
    ///
    /// With a saved index, a second library and no copies, the index of
    /// every record and the whole second library is loaded, or built and
    /// saved, and the files left out are then removed from it, which gives
    /// the same index (`Index::without_assets`).
    pub fn index(
        &self,
        records: &[PeakRecord],
        profile: &Profile,
        excluded: &BTreeSet<String>,
    ) -> Result<Index, String> {
        match (&self.saved, &self.second) {
            (Some(saved), Some(second)) if self.copies == 0 => Ok(self
                .whole_index(saved, second, records, profile)?
                .without_assets(excluded)),
            _ => self.build(records, profile, excluded),
        }
    }

    /// The index of every record and the whole second library: the saved
    /// one when its header names exactly these files, otherwise built and
    /// saved over the file. A file that cannot be written is a warning.
    fn whole_index(
        &self,
        saved: &SavedIndex,
        second: &SecondLibrary,
        records: &[PeakRecord],
        profile: &Profile,
    ) -> Result<Index, String> {
        let path = saved.path();
        let provenance = Provenance {
            profile: profile.id(),
            hash_design: hash::design(),
            track_length: ANY_LENGTH,
            sources: records
                .iter()
                .map(|record| record.header.source.clone())
                .chain(second.prefixed_assets())
                .collect(),
        };
        if let SavedIndex::Use(path) = saved {
            let started = Instant::now();
            match saved::load(path, &provenance) {
                Ok(index) => {
                    eprintln!(
                        "loaded the saved index {} in {:.1} s",
                        path.display(),
                        started.elapsed().as_secs_f64()
                    );
                    saved::remove_unfinished(path);
                    return Ok(index);
                }
                Err(Unusable::Missing) => eprintln!(
                    "building the index; it is saved for the next run in {}",
                    path.display()
                ),
                Err(unusable) => eprintln!(
                    "{unusable}; building the index again and saving it over {}",
                    path.display()
                ),
            }
        }
        let started = Instant::now();
        let index = self.build(records, profile, &BTreeSet::new())?;
        eprintln!(
            "built the index in {:.1} s",
            started.elapsed().as_secs_f64()
        );
        let started = Instant::now();
        let written = path
            .parent()
            .map_or(Ok(()), fs::create_dir_all)
            .and_then(|()| saved::save(path, &provenance, &index));
        match written {
            Ok(()) => eprintln!(
                "saved the index to {} in {:.1} s",
                path.display(),
                started.elapsed().as_secs_f64()
            ),
            Err(error) => eprintln!(
                "could not save the index to {} ({error}); the next run builds it again",
                path.display()
            ),
        }
        Ok(index)
    }

    fn build(
        &self,
        records: &[PeakRecord],
        profile: &Profile,
        excluded: &BTreeSet<String>,
    ) -> Result<Index, String> {
        let indexed: Vec<&PeakRecord> = records
            .iter()
            .filter(|record| !excluded.contains(&record.header.source.path))
            .collect();
        let mut counting = Index::counting();
        for record in &indexed {
            counting.count(record).map_err(|error| error.to_string())?;
        }
        if let Some(second) = &self.second {
            for record in second.records(profile, excluded) {
                counting
                    .count(&record?)
                    .map_err(|error| error.to_string())?;
            }
        }
        for copy in synthetic::copies(&indexed, self.copies, profile) {
            counting.count(&copy).map_err(|error| error.to_string())?;
        }
        let mut filling = counting.into_filling();
        for record in &indexed {
            filling.fill(record).map_err(|error| error.to_string())?;
        }
        if let Some(second) = &self.second {
            for record in second.records(profile, excluded) {
                filling.fill(&record?).map_err(|error| error.to_string())?;
            }
        }
        for copy in synthetic::copies(&indexed, self.copies, profile) {
            filling.fill(&copy).map_err(|error| error.to_string())?;
        }
        filling.finish().map_err(|error| error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use gunfinger_core::index::AssetId;
    use gunfinger_core::peaks::Peak;
    use gunfinger_core::store::RecordHeader;

    use super::*;

    fn record(asset: Asset, bins: &[f32]) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: asset,
                duration_seconds: 10.0,
            },
            peaks: bins
                .iter()
                .enumerate()
                .map(|(frame, &bin)| Peak {
                    frame: 10.0 * frame as f64,
                    bin,
                    magnitude: 10.0,
                })
                .collect(),
        }
    }

    #[test]
    fn the_second_library_follows_the_library_and_precedes_the_copies() {
        let dir = std::env::temp_dir().join(format!("gunfinger-padding-{}", std::process::id()));
        let root = dir.join("second");
        fs::create_dir_all(&root).unwrap();
        fs::write(root.join("b.mp3"), b"not decoded here").unwrap();
        let store = PeakStore::open(&dir.join("peaks")).unwrap();
        let scanned = Library::scan(&root).unwrap();
        store
            .save(&record(scanned.assets[0].clone(), &[100.0, 120.0, 90.0]))
            .unwrap();
        store.claim_library(&root).unwrap();
        let mut first = scanned.assets[0].clone();
        first.path = String::from("a.mp3");

        let padding = Padding {
            copies: 1,
            second: Some(SecondLibrary::open(&root, &dir.join("peaks")).unwrap()),
            saved: None,
        };
        let index = padding
            .index(
                &[record(first, &[200.0, 220.0, 190.0])],
                &Profile::CURRENT,
                &BTreeSet::new(),
            )
            .unwrap();
        fs::remove_dir_all(&dir).unwrap();

        let paths: Vec<&str> = (0..3)
            .map(|asset| index.asset(AssetId(asset)).path.as_str())
            .collect();
        assert_eq!(
            paths,
            ["a.mp3", "second-library/b.mp3", "synthetic/000/a.mp3"]
        );
        assert_eq!(padding.second_assets(), 1);
    }

    /// A second library of `files` (name, peak bins) with their records,
    /// in a fresh folder under the system's temporary folder.
    fn second_library(dir: &Path, files: &[(&str, &[f32])]) -> SecondLibrary {
        let root = dir.join("second");
        fs::create_dir_all(&root).unwrap();
        for (name, _) in files {
            // Written once, so that every call sees the same files.
            if !root.join(name).exists() {
                fs::write(root.join(name), b"not decoded here").unwrap();
            }
        }
        let store = PeakStore::open(&dir.join("peaks")).unwrap();
        for asset in Library::scan(&root).unwrap().assets {
            let (_, bins) = files.iter().find(|(name, _)| *name == asset.path).unwrap();
            store.save(&record(asset, bins)).unwrap();
        }
        store.claim_library(&root).unwrap();
        SecondLibrary::open(&root, &dir.join("peaks")).unwrap()
    }

    fn corpus() -> Vec<PeakRecord> {
        ["a.mp3", "d.mp3"]
            .iter()
            .zip([[200.0, 220.0, 190.0], [100.0, 120.0, 90.0]])
            .map(|(path, bins)| {
                let asset = Asset {
                    path: (*path).to_owned(),
                    size: 1,
                    modified: gunfinger_core::library::Timestamp {
                        seconds: 1,
                        nanos: 0,
                    },
                };
                record(asset, &bins)
            })
            .collect()
    }

    fn same_index(a: &Index, b: &Index) -> bool {
        a.assets() == b.assets() && a.posting_lists().eq(b.posting_lists())
    }

    fn excluded(paths: &[&str]) -> BTreeSet<String> {
        paths.iter().map(|path| (*path).to_owned()).collect()
    }

    #[test]
    fn the_saved_index_gives_the_built_index_also_with_files_left_out() {
        let dir = std::env::temp_dir().join(format!("gunfinger-saved-{}", std::process::id()));
        let files: &[(&str, &[f32])] = &[
            ("b.mp3", &[100.0, 120.0, 90.0]),
            ("c.mp3", &[150.0, 130.0, 160.0]),
        ];
        let built = Padding {
            second: Some(second_library(&dir, files)),
            ..Padding::default()
        };
        let file = dir.join("indexes").join("library-peaks.index");
        let saved = Padding {
            second: Some(second_library(&dir, files)),
            saved: Some(SavedIndex::Use(file.clone())),
            ..Padding::default()
        };
        let records = corpus();
        let profile = Profile::CURRENT;

        for left_out in [
            excluded(&[]),
            excluded(&["a.mp3", "second-library/b.mp3"]),
            excluded(&["d.mp3", "second-library/c.mp3", "not-indexed.mp3"]),
        ] {
            let expected = built.index(&records, &profile, &left_out).unwrap();
            let first = saved.index(&records, &profile, &left_out).unwrap();
            let written = fs::read(&file).unwrap();
            let second = saved.index(&records, &profile, &left_out).unwrap();
            assert!(same_index(&first, &expected), "{left_out:?}");
            assert!(same_index(&second, &expected), "{left_out:?}");
            assert_eq!(fs::read(&file).unwrap(), written);
        }
        // The file holds the whole index, whatever the runs left out.
        let whole = saved.index(&records, &profile, &excluded(&[])).unwrap();
        assert_eq!(whole.assets().len(), 4);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_damaged_or_stale_saved_index_is_built_again_and_replaced() {
        let dir = std::env::temp_dir().join(format!("gunfinger-stale-{}", std::process::id()));
        let file = dir.join("indexes").join("library-peaks.index");
        let records = corpus();
        let profile = Profile::CURRENT;
        let left_out = excluded(&["a.mp3"]);
        let files: &[(&str, &[f32])] = &[("b.mp3", &[100.0, 120.0, 90.0])];
        let saved = |files: &[(&str, &[f32])]| Padding {
            second: Some(second_library(&dir, files)),
            saved: Some(SavedIndex::Use(file.clone())),
            ..Padding::default()
        };
        let built = |files: &[(&str, &[f32])]| Padding {
            second: Some(second_library(&dir, files)),
            ..Padding::default()
        };
        saved(files).index(&records, &profile, &left_out).unwrap();
        let whole = fs::read(&file).unwrap();

        // Damaged: the last byte of the digest changed.
        let mut damaged = whole.clone();
        *damaged.last_mut().unwrap() ^= 1;
        fs::write(&file, &damaged).unwrap();
        let index = saved(files).index(&records, &profile, &left_out).unwrap();
        assert!(same_index(
            &index,
            &built(files).index(&records, &profile, &left_out).unwrap()
        ));
        assert_eq!(fs::read(&file).unwrap(), whole);

        // Truncated.
        fs::write(&file, &whole[..whole.len() / 2]).unwrap();
        saved(files).index(&records, &profile, &left_out).unwrap();
        assert_eq!(fs::read(&file).unwrap(), whole);

        // Stale: the second library's store gained a record.
        let grown: &[(&str, &[f32])] = &[
            ("b.mp3", &[100.0, 120.0, 90.0]),
            ("e.mp3", &[170.0, 140.0, 180.0]),
        ];
        let index = saved(grown).index(&records, &profile, &left_out).unwrap();
        assert!(same_index(
            &index,
            &built(grown).index(&records, &profile, &left_out).unwrap()
        ));
        assert_eq!(index.assets().len(), 3);
        assert_ne!(fs::read(&file).unwrap(), whole);

        // Stale: a corpus record changed (its modification time).
        let mut changed = corpus();
        changed[1].header.source.modified.seconds = 2;
        let before = fs::read(&file).unwrap();
        saved(grown).index(&changed, &profile, &left_out).unwrap();
        assert_ne!(fs::read(&file).unwrap(), before);
        fs::remove_dir_all(&dir).unwrap();
    }
}

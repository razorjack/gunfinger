//! A larger index for scale experiments: the library's records, then the
//! current records of a second library (a larger real library, when one
//! exists), then reversed copies of the library's records (`synthetic`).
//!
//! The index is built in two passes over the same records in the same
//! order, reading the second library one record at a time and making each
//! copy as it is taken, so it needs little more memory than the index.

use std::path::Path;

use gunfinger_core::index::Index;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakRecord, PeakStore};

use crate::synthetic;

/// What is added to the library's records.
#[derive(Default)]
pub struct Padding {
    /// Reversed copies of every record of the library.
    pub copies: usize,
    pub second: Option<SecondLibrary>,
}

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
        let library = Library::scan(root).map_err(|error| {
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

    pub fn len(&self) -> usize {
        self.assets.len()
    }

    /// Its records one at a time, their paths prefixed.
    fn records<'a>(
        &'a self,
        profile: &'a Profile,
    ) -> impl Iterator<Item = Result<PeakRecord, String>> + 'a {
        self.assets.iter().map(move |asset| {
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

    /// The index of `records` followed by the padding; asset `i` is
    /// `records[i]` for `i < records.len()`.
    pub fn index(&self, records: &[PeakRecord], profile: &Profile) -> Result<Index, String> {
        let mut counting = Index::counting();
        for record in records {
            counting.count(record).map_err(|error| error.to_string())?;
        }
        if let Some(second) = &self.second {
            for record in second.records(profile) {
                counting
                    .count(&record?)
                    .map_err(|error| error.to_string())?;
            }
        }
        for copy in synthetic::copies(records, self.copies, profile) {
            counting.count(&copy).map_err(|error| error.to_string())?;
        }
        let mut filling = counting.into_filling();
        for record in records {
            filling.fill(record).map_err(|error| error.to_string())?;
        }
        if let Some(second) = &self.second {
            for record in second.records(profile) {
                filling.fill(&record?).map_err(|error| error.to_string())?;
            }
        }
        for copy in synthetic::copies(records, self.copies, profile) {
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
        };
        let index = padding
            .index(&[record(first, &[200.0, 220.0, 190.0])], &Profile::CURRENT)
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
}

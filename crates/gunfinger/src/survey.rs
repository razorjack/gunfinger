//! The peak store compared with a library: what `doctor` reports and
//! `prune` removes.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, SystemTime};

use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, SkipReason, Stored};
use miette::IntoDiagnostic;

/// A temporary file younger than this may belong to an `index` still
/// running.
const TEMPORARY_GRACE: Duration = Duration::from_secs(3600);

pub struct Survey {
    pub current: usize,
    /// Records of library files that changed since, or made under another
    /// profile: `index` replaces them.
    pub stale: usize,
    /// Library files with neither a current record nor a skip note.
    pub unindexed: usize,
    pub failed: usize,
    pub too_short: usize,
    pub too_long: usize,
    /// Records and notes of files that are not in the library.
    pub orphans: Vec<Orphan>,
    /// Left by writes that never finished, older than an hour.
    pub leftovers: Vec<Stored>,
    pub unreadable: Vec<Stored>,
    pub bytes: u64,
    /// The longest current record, in seconds.
    pub longest_seconds: f64,
}

pub struct Orphan {
    pub stored: Stored,
    /// The library path the record or note was made for.
    pub source: String,
}

impl Survey {
    pub fn records_and_notes(&self) -> usize {
        self.current
            + self.stale
            + self.failed
            + self.too_short
            + self.too_long
            + self.orphans.len()
    }
}

pub fn survey(library: &Library, store: &PeakStore, profile: &Profile) -> miette::Result<Survey> {
    let assets: BTreeMap<&str, &Asset> = library
        .assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let mut covered: BTreeSet<&str> = BTreeSet::new();
    let mut survey = Survey {
        current: 0,
        stale: 0,
        unindexed: 0,
        failed: 0,
        too_short: 0,
        too_long: 0,
        orphans: Vec::new(),
        leftovers: Vec::new(),
        unreadable: Vec::new(),
        bytes: 0,
        longest_seconds: 0.0,
    };
    for stored in store.survey().into_diagnostic()? {
        let metadata = std::fs::metadata(stored.file()).ok();
        survey.bytes += metadata.as_ref().map_or(0, std::fs::Metadata::len);
        let (source, matches) = match &stored {
            Stored::Record { header, .. } => {
                let asset = assets.get(header.source.path.as_str());
                let current = asset.is_some_and(|asset| header.is_current(asset, profile));
                if current {
                    survey.longest_seconds = survey.longest_seconds.max(header.duration_seconds);
                }
                (&header.source.path, current)
            }
            Stored::Skip { note, .. } => {
                let asset = assets.get(note.source.path.as_str());
                let applies = asset.is_some_and(|asset| note.source == **asset);
                if applies {
                    match note.reason {
                        SkipReason::Failed(_) => survey.failed += 1,
                        SkipReason::TooShort { .. } => survey.too_short += 1,
                        SkipReason::TooLong { .. } => survey.too_long += 1,
                    }
                }
                (&note.source.path, applies)
            }
            Stored::Temporary { .. } => {
                let age = metadata
                    .and_then(|metadata| metadata.modified().ok())
                    .and_then(|modified| SystemTime::now().duration_since(modified).ok());
                if age.is_some_and(|age| age > TEMPORARY_GRACE) {
                    survey.leftovers.push(stored);
                }
                continue;
            }
            Stored::Unreadable { .. } => {
                survey.unreadable.push(stored);
                continue;
            }
        };
        match assets.get_key_value(source.as_str()) {
            Some((&path, _)) if matches => {
                if matches!(stored, Stored::Record { .. }) {
                    survey.current += 1;
                }
                covered.insert(path);
            }
            // A stale skip note is ignored by `index` and replaced in time.
            Some(_) => {
                if matches!(stored, Stored::Record { .. }) {
                    survey.stale += 1;
                }
            }
            None => {
                let source = source.clone();
                survey.orphans.push(Orphan { stored, source });
            }
        }
    }
    survey.unindexed = assets.len() - covered.len();
    Ok(survey)
}

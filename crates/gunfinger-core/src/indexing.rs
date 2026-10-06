//! Extracting the peaks of every library asset into the peak store.

use std::collections::BTreeSet;
use std::time::Duration;

use crate::decode::{Excerpt, decode};
use crate::index::{Index, IndexError};
use crate::library::{Asset, Library};
use crate::parallel::map_in_order;
use crate::peaks::extract_peaks;
use crate::profile::Profile;
use crate::store::{PeakRecord, PeakStore, RecordHeader, SkipNote, SkipReason, StoreError, fnv1a};

/// What indexing did with one asset.
#[derive(Debug)]
pub enum Outcome {
    Extracted {
        peaks: usize,
    },
    UpToDate,
    /// Longer than the track limit: a mix or an album rip, not a track.
    TooLong,
    Failed {
        reason: String,
    },
    /// Passed over because an earlier run failed on this exact file or
    /// found it too long.
    Remembered(SkipReason),
}

pub struct IndexingOptions {
    pub jobs: usize,
    pub max_track: Duration,
    /// Try files again that earlier runs failed on or found too long.
    pub retry_skipped: bool,
}

/// Brings the store up to date with the library, one asset per worker
/// thread. `report` is called from the workers as each asset finishes.
/// Returns the outcomes in library order.
pub fn index_library(
    library: &Library,
    store: &PeakStore,
    profile: &Profile,
    options: &IndexingOptions,
    report: impl Fn(&Asset, &Outcome) + Sync,
) -> Vec<Outcome> {
    map_in_order(&library.assets, options.jobs, |asset| {
        let outcome = index_asset(library, asset, store, profile, options);
        report(asset, &outcome);
        outcome
    })
}

fn index_asset(
    library: &Library,
    asset: &Asset,
    store: &PeakStore,
    profile: &Profile,
    options: &IndexingOptions,
) -> Outcome {
    if store.has_current(asset, profile) {
        return Outcome::UpToDate;
    }
    let max_track = options.max_track;
    if !options.retry_skipped
        && let Some(note) = store.skip_note(asset)
    {
        let still_applies = match note.reason {
            SkipReason::Failed(_) => true,
            // A raised limit may admit the file now.
            SkipReason::TooLong { limit_seconds } => max_track.as_secs_f64() <= limit_seconds,
        };
        if still_applies {
            return Outcome::Remembered(note.reason);
        }
    }
    // Decoding stops just past the limit, so a two-hour mix in the library
    // costs little more than a long track.
    let excerpt = Excerpt {
        start: None,
        duration: Some(max_track + Duration::from_secs(1)),
    };
    let path = library.absolute_path(asset);
    let audio = match decode(&path, profile.sample_rate, excerpt) {
        Ok(audio) => audio,
        Err(error) => {
            let reason = error.to_string();
            // A file that vanished or changed during the run, or a network
            // share that went away, is not a damaged file.
            if error.is_about_the_file() && asset.matches_file(&path) {
                remember(store, asset, SkipReason::Failed(reason.clone()));
            }
            return Outcome::Failed { reason };
        }
    };
    if audio.duration() > max_track {
        remember(
            store,
            asset,
            SkipReason::TooLong {
                limit_seconds: max_track.as_secs_f64(),
            },
        );
        return Outcome::TooLong;
    }
    let record = PeakRecord {
        header: RecordHeader {
            profile: profile.id(),
            source: asset.clone(),
            duration_seconds: audio.duration().as_secs_f64(),
        },
        peaks: extract_peaks(&audio.samples, profile),
    };
    match store.save(&record) {
        Ok(()) => Outcome::Extracted {
            peaks: record.peaks.len(),
        },
        Err(error) => Outcome::Failed {
            reason: error.to_string(),
        },
    }
}

/// A note that cannot be written costs only a retry on the next run.
fn remember(store: &PeakStore, asset: &Asset, reason: SkipReason) {
    let _ = store.save_skip(&SkipNote {
        source: asset.clone(),
        reason,
    });
}

/// The current peak records of the library's assets, in library order,
/// leaving out the paths in `excluded`. Assets without a current record are
/// returned as errors rather than failing the whole load.
pub fn load_records(
    library: &Library,
    store: &PeakStore,
    profile: &Profile,
    excluded: &BTreeSet<String>,
) -> (Vec<PeakRecord>, Vec<StoreError>) {
    let mut records = Vec::new();
    let mut problems = Vec::new();
    for asset in &library.assets {
        if excluded.contains(&asset.path) {
            continue;
        }
        match store.load(asset, profile) {
            Ok(record) => records.push(record),
            Err(problem) => problems.push(problem),
        }
    }
    (records, problems)
}

/// An index built from the peak store.
pub struct BuiltIndex {
    pub index: Index,
    /// The library revision of the assets indexed.
    pub revision: String,
    /// Assets left out because they have no current peak record.
    pub problems: Vec<StoreError>,
}

/// The index of the library's current peak records, leaving out the paths
/// in `excluded`. It is built in two passes that read one record at a time
/// from the store (`Index::counting`), so it needs little more memory than
/// the index itself. Assets without a current record are left out and
/// returned as problems rather than failing the build.
pub fn build_index(
    library: &Library,
    store: &PeakStore,
    profile: &Profile,
    excluded: &BTreeSet<String>,
) -> Result<BuiltIndex, IndexError> {
    let mut counting = Index::counting();
    let mut indexed = Vec::new();
    let mut problems = Vec::new();
    for asset in &library.assets {
        if excluded.contains(&asset.path) {
            continue;
        }
        match store.load(asset, profile) {
            Ok(record) => {
                counting.count(&record)?;
                indexed.push(asset);
            }
            Err(problem) => problems.push(problem),
        }
    }
    let mut filling = counting.into_filling();
    for &asset in &indexed {
        let record = store
            .load(asset, profile)
            .map_err(|_| IndexError::Changed)?;
        filling.fill(&record)?;
    }
    Ok(BuiltIndex {
        index: filling.finish()?,
        revision: library_revision(indexed),
        problems,
    })
}

/// The assets an index built now would hold, reading only the headers of
/// their peak records: `library_revision` of these is the revision
/// `build_index` would give, unless a record's peaks turn out unreadable.
pub fn indexable_assets<'a>(
    library: &'a Library,
    store: &'a PeakStore,
    profile: &'a Profile,
    excluded: &'a BTreeSet<String>,
) -> impl Iterator<Item = &'a Asset> {
    library
        .assets
        .iter()
        .filter(|asset| !excluded.contains(&asset.path) && store.has_current(asset, profile))
}

/// A digest of the files an index is made from: each asset's path, size and
/// modification time, in library order. Indexes of the same revision built
/// under the same profile and hash design hold the same postings.
pub fn library_revision<'a>(assets: impl IntoIterator<Item = &'a Asset>) -> String {
    let mut bytes = Vec::new();
    for asset in assets {
        bytes.extend(asset.path.as_bytes());
        bytes.push(0);
        bytes.extend(asset.size.to_le_bytes());
        bytes.extend(asset.modified.seconds.to_le_bytes());
        bytes.extend(asset.modified.nanos.to_le_bytes());
    }
    format!("{:016x}", fnv1a(&bytes))
}

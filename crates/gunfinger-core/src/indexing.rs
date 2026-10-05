//! Extracting the peaks of every library asset into the peak store.

use std::collections::BTreeSet;
use std::time::Duration;

use crate::decode::{Excerpt, decode};
use crate::library::{Asset, Library};
use crate::parallel::map_in_order;
use crate::peaks::extract_peaks;
use crate::profile::Profile;
use crate::store::{PeakRecord, PeakStore, RecordHeader, StoreError};

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
}

pub struct IndexingOptions {
    pub jobs: usize,
    pub max_track: Duration,
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
        let outcome = index_asset(library, asset, store, profile, options.max_track);
        report(asset, &outcome);
        outcome
    })
}

fn index_asset(
    library: &Library,
    asset: &Asset,
    store: &PeakStore,
    profile: &Profile,
    max_track: Duration,
) -> Outcome {
    if store.has_current(asset, profile) {
        return Outcome::UpToDate;
    }
    // Decoding stops just past the limit, so a two-hour mix in the library
    // costs little more than a long track.
    let excerpt = Excerpt {
        start: None,
        duration: Some(max_track + Duration::from_secs(1)),
    };
    let audio = match decode(&library.absolute_path(asset), profile.sample_rate, excerpt) {
        Ok(audio) => audio,
        Err(error) => {
            return Outcome::Failed {
                reason: error.to_string(),
            };
        }
    };
    if audio.duration() > max_track {
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

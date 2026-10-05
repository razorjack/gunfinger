//! Extracting the peaks of every library asset into the peak store.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crate::decode::{Excerpt, decode};
use crate::library::{Asset, Library};
use crate::peaks::extract_peaks;
use crate::profile::Profile;
use crate::store::{PeakRecord, PeakStore, RecordHeader};

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

/// Brings the store up to date with the library. Each worker thread takes
/// the next unprocessed asset; `report` is called from the workers as each
/// asset finishes. Returns the outcomes in library order.
pub fn index_library(
    library: &Library,
    store: &PeakStore,
    profile: &Profile,
    options: &IndexingOptions,
    report: impl Fn(&Asset, &Outcome) + Sync,
) -> Vec<Outcome> {
    let next = AtomicUsize::new(0);
    let mut outcomes: Vec<(usize, Outcome)> = thread::scope(|scope| {
        let workers: Vec<_> = (0..options.jobs.max(1))
            .map(|_| {
                scope.spawn(|| {
                    let mut done = Vec::new();
                    loop {
                        let position = next.fetch_add(1, Ordering::Relaxed);
                        let Some(asset) = library.assets.get(position) else {
                            break;
                        };
                        let outcome =
                            index_asset(library, asset, store, profile, options.max_track);
                        report(asset, &outcome);
                        done.push((position, outcome));
                    }
                    done
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|worker| worker.join().unwrap_or_default())
            .collect()
    });
    outcomes.sort_by_key(|(position, _)| *position);
    outcomes.into_iter().map(|(_, outcome)| outcome).collect()
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

//! Extracting the peaks of every library asset into the peak store.

use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;
use std::time::Duration;

use crate::decode::{Excerpt, decode, probe};
use crate::index::{Index, IndexError};
use crate::library::{Asset, Library};
use crate::parallel::map_in_order;
use crate::peaks::extract_peaks;
use crate::profile::Profile;
use crate::store::{
    PeakRecord, PeakStore, RecordHeader, SkipNote, SkipReason, StoreError, TagNote, fnv1a,
};
use crate::timecode::format_timecode;

/// A declared length decides on its own only when it is this far outside
/// the range, so that a bitrate estimate a little off cannot reject a
/// track; nearer a limit, the decoded length decides.
const DECLARED_LENGTH_MARGIN: f64 = 1.1;

/// What indexing did with one asset.
#[derive(Debug)]
pub enum Outcome {
    Extracted {
        peaks: usize,
    },
    UpToDate,
    /// Up to date, and its tags, which the store lacked, were read from the
    /// file's header.
    Tagged,
    /// Not a track: `TooShort` or `TooLong` for the track length range.
    Rejected(SkipReason),
    Failed {
        reason: String,
    },
    /// Passed over because an earlier run failed on or rejected this exact
    /// file.
    Remembered(SkipReason),
}

/// The lengths of library files that count as tracks. Shorter files are
/// samples and loops; longer ones are mixes and album rips.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrackLength {
    pub min: Duration,
    pub max: Duration,
}

impl fmt::Display for TrackLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.min.is_zero() {
            write!(f, "up to {}", format_timecode(self.max))
        } else {
            write!(
                f,
                "{} to {}",
                format_timecode(self.min),
                format_timecode(self.max)
            )
        }
    }
}

impl TrackLength {
    pub fn admits(&self, length: Duration) -> bool {
        self.rejects(length).is_none()
    }

    /// Why a file of this length is not a track, if it is not.
    pub fn rejects(&self, length: Duration) -> Option<SkipReason> {
        self.rejects_beyond(length, 1.0)
    }

    /// As `rejects`, for a length that may be off by up to `factor`.
    fn rejects_beyond(&self, length: Duration, factor: f64) -> Option<SkipReason> {
        if length.mul_f64(factor) < self.min {
            Some(SkipReason::TooShort { limit: self.min })
        } else if length > self.max.mul_f64(factor) {
            Some(SkipReason::TooLong { limit: self.max })
        } else {
            None
        }
    }

    /// The length of a peak record outside the range, if it is. Records
    /// indexed under a wider range stay in the store, so widening it again
    /// needs no decoding.
    fn leaves_out(&self, header: &RecordHeader) -> Option<Duration> {
        Duration::try_from_secs_f64(header.duration_seconds)
            .ok()
            .filter(|&length| !self.admits(length))
    }
}

pub struct IndexingOptions {
    pub jobs: usize,
    pub length: TrackLength,
    /// Try files again that earlier runs failed on or rejected.
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
    let path = library.absolute_path(asset);
    if store.has_current(asset, profile) {
        return match store.tags(asset) {
            Some(_) => Outcome::UpToDate,
            None => tag(store, asset, &path),
        };
    }
    let length = options.length;
    if !options.retry_skipped
        && let Some(note) = store.skip_note(asset)
    {
        let still_applies = match note.reason {
            SkipReason::Failed(_) => true,
            // A widened range may admit the file now.
            SkipReason::TooLong { limit } => length.max <= limit,
            SkipReason::TooShort { limit } => length.min >= limit,
        };
        if still_applies {
            return Outcome::Remembered(note.reason);
        }
    }
    let probe = probe(&path);
    // A set in a folder of tracks is rejected from its header alone.
    if let Some(reason) = probe
        .as_ref()
        .and_then(|probe| probe.length)
        .and_then(|declared| length.rejects_beyond(declared, DECLARED_LENGTH_MARGIN))
    {
        remember(store, asset, reason.clone());
        return Outcome::Rejected(reason);
    }
    // Decoding stops just past the limit, so a mix whose header declares no
    // length costs little more than a long track.
    let excerpt = Excerpt {
        start: None,
        duration: Some(length.max + Duration::from_secs(1)),
    };
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
    if let Some(reason) = length.rejects(audio.duration()) {
        remember(store, asset, reason.clone());
        return Outcome::Rejected(reason);
    }
    let record = PeakRecord {
        header: RecordHeader {
            profile: profile.id(),
            source: asset.clone(),
            duration_seconds: audio.duration().as_secs_f64(),
        },
        peaks: extract_peaks(&audio.samples, profile),
    };
    if let Err(error) = store.save(&record) {
        return Outcome::Failed {
            reason: error.to_string(),
        };
    }
    // Without tags the record still serves; the next run reads them.
    if let Some(probe) = probe {
        let _ = store.save_tags(&TagNote {
            source: asset.clone(),
            tags: probe.tags,
        });
    }
    Outcome::Extracted {
        peaks: record.peaks.len(),
    }
}

/// Stores the tags of an up-to-date asset, reading only its header. A file
/// without tags gets an empty note, so it is not read again.
fn tag(store: &PeakStore, asset: &Asset, path: &Path) -> Outcome {
    let Some(probe) = probe(path) else {
        return Outcome::UpToDate;
    };
    let saved = store.save_tags(&TagNote {
        source: asset.clone(),
        tags: probe.tags,
    });
    match saved {
        Ok(()) => Outcome::Tagged,
        Err(_) => Outcome::UpToDate,
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
    /// Assets left out because their length is outside the track length
    /// range, with that length.
    pub outside: Vec<(String, Duration)>,
}

/// The index of the library's current peak records within `length`,
/// leaving out the paths in `excluded`. It is built in two passes that read
/// one record at a time from the store (`Index::counting`), so it needs
/// little more memory than the index itself. Assets without a current
/// record are left out and returned as problems rather than failing the
/// build.
pub fn build_index(
    library: &Library,
    store: &PeakStore,
    profile: &Profile,
    excluded: &BTreeSet<String>,
    length: TrackLength,
) -> Result<BuiltIndex, IndexError> {
    let mut counting = Index::counting();
    let mut indexed = Vec::new();
    let mut problems = Vec::new();
    let mut outside = Vec::new();
    for asset in &library.assets {
        if excluded.contains(&asset.path) {
            continue;
        }
        match store.load(asset, profile) {
            Ok(record) => match length.leaves_out(&record.header) {
                Some(duration) => outside.push((asset.path.clone(), duration)),
                None => {
                    counting.count(&record)?;
                    indexed.push(asset);
                }
            },
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
        outside,
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
    length: TrackLength,
) -> impl Iterator<Item = &'a Asset> {
    library.assets.iter().filter(move |asset| {
        !excluded.contains(&asset.path)
            && store
                .current_header(asset, profile)
                .is_some_and(|header| length.leaves_out(&header).is_none())
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    const LENGTH: TrackLength = TrackLength {
        min: Duration::from_secs(90),
        max: Duration::from_secs(900),
    };

    #[test]
    fn a_track_length_rejects_samples_and_sets() {
        assert_eq!(
            LENGTH.rejects(Duration::from_secs(12)),
            Some(SkipReason::TooShort {
                limit: Duration::from_secs(90)
            })
        );
        assert_eq!(
            LENGTH.rejects(Duration::from_secs(3600)),
            Some(SkipReason::TooLong {
                limit: Duration::from_secs(900)
            })
        );
        assert!(LENGTH.admits(Duration::from_secs(90)));
        assert!(LENGTH.admits(Duration::from_secs(900)));
        assert_eq!(LENGTH.to_string(), "1:30 to 15:00");
    }

    #[test]
    fn a_declared_length_near_a_limit_does_not_decide() {
        let near = [Duration::from_secs(85), Duration::from_secs(960)];
        let far = [Duration::from_secs(80), Duration::from_secs(1000)];

        for length in near {
            assert_eq!(
                LENGTH.rejects_beyond(length, DECLARED_LENGTH_MARGIN),
                None,
                "{length:?}"
            );
        }
        for length in far {
            assert!(
                LENGTH
                    .rejects_beyond(length, DECLARED_LENGTH_MARGIN)
                    .is_some(),
                "{length:?}"
            );
        }
    }
}

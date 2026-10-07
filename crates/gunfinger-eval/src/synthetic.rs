//! Synthetic library growth, to measure query cost and the null at library
//! sizes the corpus does not reach (ADR 0007).
//!
//! A copy of a real peak record is reversed in time, so no real audio can
//! align with it, and stretched in time and frequency, so its hashes differ
//! from the original's and from other copies' while still coming from music
//! of the same kind. Reversed copies keep the spectra, drum hits and density
//! of the library but none of its forward structure, so the null they give
//! is a lower bound: a real library of the same size also holds remixes,
//! shared breaks and samples.

use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakRecord, RecordHeader};

/// Stretch factors for time and frequency. Steps of 4% change most hashes:
/// the anchor bin is quantised in 2% steps and Δt in whole frames.
const STRETCHES: [f64; 11] = [
    1.00, 0.96, 1.04, 0.92, 1.08, 0.88, 1.12, 0.84, 1.16, 0.80, 1.20,
];

/// The most copies `copies` can make of each record.
pub const MAX_COPIES: usize = STRETCHES.len() * STRETCHES.len();

/// `count` reversed copies of every record, the least stretched first,
/// named `synthetic/<copy>/<path>`, made as they are taken.
pub fn copies<'a>(
    records: &'a [PeakRecord],
    count: usize,
    profile: &'a Profile,
) -> impl Iterator<Item = PeakRecord> + 'a {
    let mut stretches: Vec<(f64, f64)> = STRETCHES
        .iter()
        .flat_map(|&time| STRETCHES.iter().map(move |&frequency| (time, frequency)))
        .collect();
    stretches.sort_by(|a, b| {
        let distance = |(time, frequency): (f64, f64)| (time - 1.0).abs() + (frequency - 1.0).abs();
        distance(*a).total_cmp(&distance(*b))
    });
    stretches.truncate(count);
    stretches
        .into_iter()
        .enumerate()
        .flat_map(move |(copy, (time, frequency))| {
            records
                .iter()
                .map(move |record| reversed(record, copy, time, frequency, profile))
        })
}

fn reversed(
    record: &PeakRecord,
    copy: usize,
    time: f64,
    frequency: f64,
    profile: &Profile,
) -> PeakRecord {
    let end = profile.frames(record.header.duration_seconds);
    let mut peaks: Vec<Peak> = record
        .peaks
        .iter()
        .map(|peak| Peak {
            frame: (end - peak.frame).max(0.0) * time,
            bin: (f64::from(peak.bin) * frequency) as f32,
            magnitude: peak.magnitude,
        })
        .filter(|peak| (profile.min_bin..profile.max_bin).contains(&(peak.bin as usize)))
        .collect();
    peaks.sort_by(|a, b| a.frame.total_cmp(&b.frame).then(a.bin.total_cmp(&b.bin)));
    let mut header: RecordHeader = record.header.clone();
    header.source.path = format!("synthetic/{copy:03}/{}", header.source.path);
    header.duration_seconds *= time;
    PeakRecord { header, peaks }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};

    use super::*;

    fn record() -> PeakRecord {
        let peak = |frame, bin| Peak {
            frame,
            bin,
            magnitude: 10.0,
        };
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: String::from("a.mp3"),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: Profile::CURRENT.seconds(100.0),
            },
            peaks: vec![peak(10.0, 100.0), peak(20.0, 200.0), peak(90.0, 300.0)],
        }
    }

    #[test]
    fn the_first_copy_is_the_record_reversed_in_time() {
        let copies: Vec<PeakRecord> = copies(&[record()], 1, &Profile::CURRENT).collect();

        let peaks: Vec<(f64, f32)> = copies[0]
            .peaks
            .iter()
            .map(|peak| (peak.frame, peak.bin))
            .collect();
        assert_eq!(peaks, [(10.0, 300.0), (80.0, 200.0), (90.0, 100.0)]);
        assert_eq!(copies[0].header.source.path, "synthetic/000/a.mp3");
    }

    #[test]
    fn later_copies_are_stretched_and_distinct() {
        let copies: Vec<PeakRecord> = copies(&[record()], 5, &Profile::CURRENT).collect();

        let mut shapes: Vec<String> = copies
            .iter()
            .map(|copy| {
                format!(
                    "{:?}",
                    copy.peaks
                        .iter()
                        .map(|peak| (peak.frame, peak.bin))
                        .collect::<Vec<_>>()
                )
            })
            .collect();
        shapes.sort();
        shapes.dedup();
        assert_eq!(shapes.len(), 5);
    }
}

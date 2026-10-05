//! The peak store: one self-describing binary record per asset.
//!
//! Peaks are the source of truth; hashes and the index are rebuilt from them.
//! The store is meant to live on a NAS, so records are read and written
//! sequentially in one pass, and written atomically (temporary file, then
//! rename) so a crash never leaves a half-written record behind.
//!
//! Record layout, little-endian:
//!
//! ```text
//! magic           8 bytes  "GUNFPEAK"
//! format version  u16
//! profile         u16 length + UTF-8
//! source path     u16 length + UTF-8, relative to the library root
//! source size     u64
//! source mtime    i64 seconds + u32 nanoseconds
//! duration        f64 seconds of decoded audio
//! peak count      u32
//! peaks           per peak: frame delta (LEB128 varint), bin (u16, 1/64
//!                 bin), magnitude (u8, 0.5 dB steps from -20 dB)
//! ```

use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::library::{Asset, Timestamp};
use crate::peaks::{BIN_STEPS, MAGNITUDE_STEPS_PER_DB, Peak};
use crate::profile::Profile;

const MAGIC: &[u8; 8] = b"GUNFPEAK";
pub const FORMAT_VERSION: u16 = 1;
/// The quietest magnitude a `u8` field can hold. The peak picker's floor is
/// above it and STFT power of audio in [-1, 1] stays below its ceiling of
/// 107.5 dB, so clamping never happens in practice.
const MAGNITUDE_ORIGIN_DB: f32 = -20.0;

/// The peaks of one asset and the facts that tell whether they are current.
#[derive(Debug, Clone, PartialEq)]
pub struct PeakRecord {
    pub header: RecordHeader,
    pub peaks: Vec<Peak>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordHeader {
    pub profile: String,
    pub source: Asset,
    pub duration_seconds: f64,
}

impl RecordHeader {
    /// A record is current when it describes this exact file (path, size and
    /// mtime) under this front-end profile.
    pub fn is_current(&self, asset: &Asset, profile: &Profile) -> bool {
        self.source == *asset && self.profile == profile.id()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("no peak record for {asset}; run `gunfinger index` on the library")]
    Missing { asset: String },
    #[error("the peak record for {asset} is out of date; run `gunfinger index` on the library")]
    Stale { asset: String },
    #[error("peak record {path} is unreadable ({reason}); delete it and run `gunfinger index`")]
    Corrupt { path: PathBuf, reason: String },
    #[error("could not access the peak store at {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
}

pub struct PeakStore {
    dir: PathBuf,
}

impl PeakStore {
    pub fn open(dir: &Path) -> Result<PeakStore, StoreError> {
        fs::create_dir_all(dir).map_err(|source| StoreError::Io {
            path: dir.to_owned(),
            source,
        })?;
        Ok(PeakStore {
            dir: dir.to_owned(),
        })
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Records are named by a hash of the asset path: library paths are long,
    /// nested and full of characters some file systems reject.
    pub fn record_path(&self, asset_path: &str) -> PathBuf {
        self.dir
            .join(format!("{:016x}.peaks", fnv1a(asset_path.as_bytes())))
    }

    /// Whether a current record exists, reading only its header.
    pub fn has_current(&self, asset: &Asset, profile: &Profile) -> bool {
        let Ok(file) = File::open(self.record_path(&asset.path)) else {
            return false;
        };
        let mut reader = BufReader::new(file);
        read_header(&mut reader).is_ok_and(|header| header.is_current(asset, profile))
    }

    /// Loads the current record of `asset`.
    pub fn load(&self, asset: &Asset, profile: &Profile) -> Result<PeakRecord, StoreError> {
        let path = self.record_path(&asset.path);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(StoreError::Missing {
                    asset: asset.path.clone(),
                });
            }
            Err(source) => return Err(StoreError::Io { path, source }),
        };
        let corrupt = |error: io::Error| StoreError::Corrupt {
            path: path.clone(),
            reason: error.to_string(),
        };
        let mut reader = BufReader::new(file);
        let header = read_header(&mut reader).map_err(corrupt)?;
        if !header.is_current(asset, profile) {
            return Err(StoreError::Stale {
                asset: asset.path.clone(),
            });
        }
        let peaks = read_peaks(&mut reader).map_err(corrupt)?;
        Ok(PeakRecord { header, peaks })
    }

    /// Writes `record` atomically, replacing any previous record.
    pub fn save(&self, record: &PeakRecord) -> Result<(), StoreError> {
        let path = self.record_path(&record.header.source.path);
        let temporary = path.with_extension(format!("tmp{}", std::process::id()));
        let io_error = |source| StoreError::Io {
            path: temporary.clone(),
            source,
        };
        let mut writer = BufWriter::new(File::create(&temporary).map_err(io_error)?);
        write_record(&mut writer, record).map_err(io_error)?;
        let file = writer
            .into_inner()
            .map_err(|error| io_error(error.into_error()))?;
        file.sync_all().map_err(io_error)?;
        fs::rename(&temporary, &path).map_err(io_error)
    }
}

fn write_record(out: &mut impl Write, record: &PeakRecord) -> io::Result<()> {
    let header = &record.header;
    out.write_all(MAGIC)?;
    out.write_all(&FORMAT_VERSION.to_le_bytes())?;
    write_text(out, &header.profile)?;
    write_text(out, &header.source.path)?;
    out.write_all(&header.source.size.to_le_bytes())?;
    out.write_all(&header.source.modified.seconds.to_le_bytes())?;
    out.write_all(&header.source.modified.nanos.to_le_bytes())?;
    out.write_all(&header.duration_seconds.to_le_bytes())?;
    let count = u32::try_from(record.peaks.len()).map_err(io::Error::other)?;
    out.write_all(&count.to_le_bytes())?;

    let mut previous_frame = 0;
    for peak in &record.peaks {
        let delta = peak.frame.checked_sub(previous_frame).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "peaks are not ordered by frame",
            )
        })?;
        write_varint(out, delta)?;
        out.write_all(&((peak.bin * BIN_STEPS) as u16).to_le_bytes())?;
        let magnitude = (peak.magnitude - MAGNITUDE_ORIGIN_DB) * MAGNITUDE_STEPS_PER_DB;
        out.write_all(&[magnitude.clamp(0.0, 255.0) as u8])?;
        previous_frame = peak.frame;
    }
    Ok(())
}

fn read_header(input: &mut impl Read) -> io::Result<RecordHeader> {
    let mut magic = [0; 8];
    input.read_exact(&mut magic)?;
    if &magic != MAGIC {
        return Err(invalid("not a peak record"));
    }
    let version = u16::from_le_bytes(read_array(input)?);
    if version != FORMAT_VERSION {
        return Err(invalid("unsupported format version"));
    }
    let profile = read_text(input)?;
    let path = read_text(input)?;
    let size = u64::from_le_bytes(read_array(input)?);
    let seconds = i64::from_le_bytes(read_array(input)?);
    let nanos = u32::from_le_bytes(read_array(input)?);
    let duration_seconds = f64::from_le_bytes(read_array(input)?);
    Ok(RecordHeader {
        profile,
        source: Asset {
            path,
            size,
            modified: Timestamp { seconds, nanos },
        },
        duration_seconds,
    })
}

fn read_peaks(input: &mut impl Read) -> io::Result<Vec<Peak>> {
    let count = u32::from_le_bytes(read_array(input)?);
    let mut peaks = Vec::with_capacity(count as usize);
    let mut frame = 0_u32;
    for _ in 0..count {
        frame = frame
            .checked_add(read_varint(input)?)
            .ok_or_else(|| invalid("frame index overflows"))?;
        let bin = f32::from(u16::from_le_bytes(read_array(input)?)) / BIN_STEPS;
        let [magnitude] = read_array(input)?;
        peaks.push(Peak {
            frame,
            bin,
            magnitude: f32::from(magnitude) / MAGNITUDE_STEPS_PER_DB + MAGNITUDE_ORIGIN_DB,
        });
    }
    let mut trailing = [0];
    if input.read(&mut trailing)? != 0 {
        return Err(invalid("trailing bytes after the last peak"));
    }
    Ok(peaks)
}

fn write_text(out: &mut impl Write, text: &str) -> io::Result<()> {
    let length = u16::try_from(text.len()).map_err(io::Error::other)?;
    out.write_all(&length.to_le_bytes())?;
    out.write_all(text.as_bytes())
}

fn read_text(input: &mut impl Read) -> io::Result<String> {
    let length = u16::from_le_bytes(read_array(input)?);
    let mut bytes = vec![0; usize::from(length)];
    input.read_exact(&mut bytes)?;
    String::from_utf8(bytes).map_err(|_| invalid("text is not UTF-8"))
}

fn read_array<const N: usize>(input: &mut impl Read) -> io::Result<[u8; N]> {
    let mut bytes = [0; N];
    input.read_exact(&mut bytes)?;
    Ok(bytes)
}

/// LEB128: seven bits per byte, low bits first, high bit set on all but the
/// last byte. Frame deltas between neighbouring peaks are almost always below
/// 128 and take one byte.
fn write_varint(out: &mut impl Write, mut value: u32) -> io::Result<()> {
    while value >= 0x80 {
        out.write_all(&[(value as u8 & 0x7f) | 0x80])?;
        value >>= 7;
    }
    out.write_all(&[value as u8])
}

fn read_varint(input: &mut impl Read) -> io::Result<u32> {
    let mut value = 0_u32;
    for shift in (0..32).step_by(7) {
        let [byte] = read_array(input)?;
        value |= u32::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err(invalid("varint is longer than 32 bits"))
}

fn invalid(reason: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, reason.to_owned())
}

/// 64-bit FNV-1a: a simple, stable hash for naming record files.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    fn asset() -> Asset {
        Asset {
            path: "extra/release/01-artist-title.mp3".to_owned(),
            size: 12_345_678,
            modified: Timestamp {
                seconds: 1_700_000_000,
                nanos: 123_456_789,
            },
        }
    }

    fn record(peaks: Vec<Peak>) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: asset(),
                duration_seconds: 321.5,
            },
            peaks,
        }
    }

    fn round_trip(record: &PeakRecord) -> PeakRecord {
        let mut bytes = Vec::new();
        write_record(&mut bytes, record).unwrap();
        let mut input = bytes.as_slice();
        let header = read_header(&mut input).unwrap();
        let peaks = read_peaks(&mut input).unwrap();
        PeakRecord { header, peaks }
    }

    #[test]
    fn a_record_survives_a_round_trip() {
        let original = record(vec![
            Peak {
                frame: 0,
                bin: 5.0,
                magnitude: -10.0,
            },
            Peak {
                frame: 0,
                bin: 499.0 + 63.0 / 64.0,
                magnitude: 54.5,
            },
            Peak {
                frame: 70_000,
                bin: 123.5,
                magnitude: 12.0,
            },
        ]);

        assert_eq!(round_trip(&original), original);
    }

    #[test]
    fn varints_use_one_byte_below_128() {
        let mut bytes = Vec::new();
        write_varint(&mut bytes, 127).unwrap();
        write_varint(&mut bytes, 128).unwrap();

        assert_eq!(bytes, [0x7f, 0x80, 0x01]);
    }

    #[test]
    fn a_record_for_another_profile_or_file_is_not_current() {
        let header = record(Vec::new()).header;
        let mut touched = asset();
        touched.modified.nanos += 1;
        let other_profile = Profile {
            hop: 256,
            ..Profile::CURRENT
        };

        assert!(header.is_current(&asset(), &Profile::CURRENT));
        assert!(!header.is_current(&touched, &Profile::CURRENT));
        assert!(!header.is_current(&asset(), &other_profile));
    }

    #[test]
    fn saving_and_loading_goes_through_the_file_system() {
        let dir = std::env::temp_dir().join(format!("gunfinger-store-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let original = record(vec![Peak {
            frame: 3,
            bin: 64.25,
            magnitude: 1.5,
        }]);

        store.save(&original).unwrap();
        let loaded = store.load(&asset(), &Profile::CURRENT).unwrap();
        let fresh = store.has_current(&asset(), &Profile::CURRENT);
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(loaded, original);
        assert!(fresh);
    }

    proptest! {
        #[test]
        fn quantised_peaks_round_trip_exactly(
            raw in prop::collection::vec((0_u32..2_000, 0_u16..32_000, 0_u8..=255), 0..200)
        ) {
            let mut frame = 0;
            let peaks: Vec<Peak> = raw
                .into_iter()
                .map(|(delta, bin, magnitude)| {
                    frame += delta;
                    Peak {
                        frame,
                        bin: f32::from(bin) / BIN_STEPS,
                        magnitude: f32::from(magnitude) / MAGNITUDE_STEPS_PER_DB + MAGNITUDE_ORIGIN_DB,
                    }
                })
                .collect();
            let original = record(peaks);

            prop_assert_eq!(round_trip(&original), original);
        }
    }
}

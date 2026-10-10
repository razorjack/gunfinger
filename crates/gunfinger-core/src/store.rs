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
//! peaks           per peak: frame delta (LEB128 varint), offset from the
//!                 frame (i8, 1/64 frame), bin (u16, 1/64 bin), magnitude
//!                 (u8, 0.5 dB steps from -20 dB)
//! ```
//!
//! An asset that has no record because decoding failed or its length is
//! outside the track length range gets a skip note instead, so later runs
//! pass over it until the file changes:
//!
//! ```text
//! magic           8 bytes  "GUNFSKIP"
//! format version  u16
//! source path, size and mtime, as in a record
//! reason          u8: 0 decoding failed, then the message (u16 length +
//!                 UTF-8); 1 too long, then the limit exceeded (f64 seconds);
//!                 2 too short, then the limit not reached (f64 seconds)
//! ```
//!
//! The tags of an indexed file, which name its track when the library
//! itself cannot be read, sit beside its record:
//!
//! ```text
//! magic           8 bytes  "GUNFTAGS"
//! format version  u16
//! source path, size and mtime, as in a record
//! artist, title, album: each u8 0 when absent, or 1 then u16 length +
//!                 UTF-8
//! ```
//!
//! Records are named by their path relative to the library root, so two
//! libraries sharing a store would overwrite each other's records. The
//! store therefore names its library in `library.txt`: the absolute path
//! of the library root, after `#` comment lines. `index` writes it the
//! first time it indexes a library into the store (a store made before
//! this keeps its records), and every command refuses a store that names
//! another library.

use std::fmt;
use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::library::{Asset, Timestamp};
use crate::peaks::{BIN_STEPS, FRAME_STEPS, MAGNITUDE_STEPS_PER_DB, Peak};
use crate::profile::Profile;
use crate::tags::Tags;
use crate::timecode::format_timecode;

const MAGIC: &[u8; 8] = b"GUNFPEAK";
const FORMAT_VERSION: u16 = 2;
const SKIP_MAGIC: &[u8; 8] = b"GUNFSKIP";
/// Names the library whose records the store holds.
const LIBRARY_FILE: &str = "library.txt";
const SKIP_FORMAT_VERSION: u16 = 1;
const TAGS_MAGIC: &[u8; 8] = b"GUNFTAGS";
const TAGS_FORMAT_VERSION: u16 = 1;
/// Longer decoder messages are cut; the start says what went wrong.
const MAX_REASON_BYTES: usize = 2000;
/// Longer tags are cut; no artist or title comes near it.
const MAX_TAG_BYTES: usize = 1000;
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

/// Why an asset has no peak record.
#[derive(Debug, Clone, PartialEq)]
pub struct SkipNote {
    pub source: Asset,
    pub reason: SkipReason,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SkipReason {
    /// Decoding failed with this message.
    Failed(String),
    /// Longer than the longest track allowed when it was indexed.
    TooLong { limit: Duration },
    /// Shorter than the shortest track allowed when it was indexed.
    TooShort { limit: Duration },
}

impl fmt::Display for SkipReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SkipReason::Failed(_) => write!(f, "it failed to decode"),
            SkipReason::TooLong { limit } => write!(f, "longer than {}", format_timecode(*limit)),
            SkipReason::TooShort { limit } => {
                write!(f, "shorter than {}", format_timecode(*limit))
            }
        }
    }
}

/// The tags of an asset's file as `index` read them, possibly none.
#[derive(Debug, Clone, PartialEq)]
pub struct TagNote {
    pub source: Asset,
    pub tags: Tags,
}

/// A file in the store directory, as `PeakStore::survey` finds it.
#[derive(Debug)]
pub enum Stored {
    Record {
        file: PathBuf,
        header: RecordHeader,
    },
    Skip {
        file: PathBuf,
        note: SkipNote,
    },
    Tags {
        file: PathBuf,
        note: TagNote,
    },
    /// Not a record or note this version can read.
    Unreadable {
        file: PathBuf,
        reason: String,
    },
    /// Left behind by a write that never finished.
    Temporary {
        file: PathBuf,
    },
}

impl Stored {
    pub fn file(&self) -> &Path {
        match self {
            Stored::Record { file, .. }
            | Stored::Skip { file, .. }
            | Stored::Tags { file, .. }
            | Stored::Unreadable { file, .. }
            | Stored::Temporary { file } => file,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("no peak record for {asset}; run `gunfinger index` on the library")]
    Missing { asset: String },
    #[error(
        "{asset} was passed over when the library was indexed ({reason}); `gunfinger index --retry-skipped` tries it again"
    )]
    Skipped { asset: String, reason: SkipReason },
    #[error("the peak record for {asset} is out of date; run `gunfinger index` on the library")]
    Stale { asset: String },
    #[error("peak record {path} is unreadable ({reason}); delete it and run `gunfinger index`")]
    Corrupt { path: PathBuf, reason: String },
    #[error("could not access the peak store at {path}: {source}")]
    Io { path: PathBuf, source: io::Error },
    #[error(
        "the peak store at {store} holds the records of the library at {owner}, not {library}; give each library its own store (--peaks-dir), or, if the library moved there, put its new path in {file}"
    )]
    OtherLibrary {
        store: PathBuf,
        owner: String,
        library: String,
        file: PathBuf,
    },
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

    /// The library this store holds the records of, if it names one.
    pub fn library(&self) -> Result<Option<String>, StoreError> {
        let path = self.dir.join(LIBRARY_FILE);
        match fs::read_to_string(&path) {
            Ok(text) => Ok(text
                .lines()
                .map(str::trim)
                .find(|line| !line.is_empty() && !line.starts_with('#'))
                .map(str::to_owned)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(StoreError::Io { path, source }),
        }
    }

    /// Fails when the store names a library other than the one at `root`.
    /// A store that names none passes: it predates the name, or nothing has
    /// been indexed into it.
    pub fn check_library(&self, root: &Path) -> Result<(), StoreError> {
        let library = library_name(root);
        match self.library()? {
            Some(owner) if owner != library => Err(StoreError::OtherLibrary {
                store: self.dir.clone(),
                owner,
                library,
                file: self.dir.join(LIBRARY_FILE),
            }),
            _ => Ok(()),
        }
    }

    /// Makes the library at `root` this store's library, unless the store
    /// names another (an error). A store that names none adopts it, keeping
    /// its records.
    pub fn claim_library(&self, root: &Path) -> Result<(), StoreError> {
        self.check_library(root)?;
        if self.library()?.is_some() {
            return Ok(());
        }
        write_atomically(&self.dir.join(LIBRARY_FILE), |out| {
            writeln!(
                out,
                "# The peak records here describe the files of this library."
            )?;
            writeln!(
                out,
                "# If the library moves, put its new path here: the records stay valid."
            )?;
            writeln!(out, "{}", library_name(root))
        })
    }

    /// Records are named by a hash of the asset path: library paths are long,
    /// nested and full of characters some file systems reject.
    pub fn record_path(&self, asset_path: &str) -> PathBuf {
        self.dir
            .join(format!("{:016x}.peaks", fnv1a(asset_path.as_bytes())))
    }

    /// Size of an asset's record on disk.
    pub fn record_bytes(&self, asset_path: &str) -> Result<u64, StoreError> {
        let path = self.record_path(asset_path);
        match fs::metadata(&path) {
            Ok(metadata) => Ok(metadata.len()),
            Err(source) => Err(StoreError::Io { path, source }),
        }
    }

    /// Whether a current record exists, reading only its header.
    pub fn has_current(&self, asset: &Asset, profile: &Profile) -> bool {
        self.current_header(asset, profile).is_some()
    }

    /// The header of `asset`'s current record, read without its peaks.
    pub fn current_header(&self, asset: &Asset, profile: &Profile) -> Option<RecordHeader> {
        let file = File::open(self.record_path(&asset.path)).ok()?;
        read_header(&mut BufReader::new(file))
            .ok()
            .filter(|header| header.is_current(asset, profile))
    }

    /// Loads the current record of `asset`.
    pub fn load(&self, asset: &Asset, profile: &Profile) -> Result<PeakRecord, StoreError> {
        let (mut reader, header) = self.open_current(asset, profile)?;
        let peaks = read_peaks(&mut reader).map_err(|error| StoreError::Corrupt {
            path: self.record_path(&asset.path),
            reason: error.to_string(),
        })?;
        Ok(PeakRecord { header, peaks })
    }

    /// The header of `asset`'s current record, with the errors `load` gives
    /// when there is none, without reading the peaks.
    pub fn load_header(
        &self,
        asset: &Asset,
        profile: &Profile,
    ) -> Result<RecordHeader, StoreError> {
        self.open_current(asset, profile).map(|(_, header)| header)
    }

    /// The record of `asset`, positioned after its header, when that header
    /// is current.
    fn open_current(
        &self,
        asset: &Asset,
        profile: &Profile,
    ) -> Result<(BufReader<File>, RecordHeader), StoreError> {
        let path = self.record_path(&asset.path);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(self.passed_over(asset).unwrap_or(StoreError::Missing {
                    asset: asset.path.clone(),
                }));
            }
            Err(source) => return Err(StoreError::Io { path, source }),
        };
        let mut reader = BufReader::new(file);
        let header = read_header(&mut reader).map_err(|error| StoreError::Corrupt {
            path: path.clone(),
            reason: error.to_string(),
        })?;
        if !header.is_current(asset, profile) {
            return Err(self.passed_over(asset).unwrap_or(StoreError::Stale {
                asset: asset.path.clone(),
            }));
        }
        Ok((reader, header))
    }

    /// `Skipped` when `index` remembered why this exact file has no current
    /// record: running `index` again would not change that.
    fn passed_over(&self, asset: &Asset) -> Option<StoreError> {
        self.skip_note(asset).map(|note| StoreError::Skipped {
            asset: asset.path.clone(),
            reason: note.reason,
        })
    }

    /// Writes `record` atomically, replacing any previous record, and
    /// drops the asset's skip note.
    pub fn save(&self, record: &PeakRecord) -> Result<(), StoreError> {
        let path = self.record_path(&record.header.source.path);
        write_atomically(&path, |out| write_record(out, record))?;
        self.forget_skip(&record.header.source.path)
    }

    fn skip_path(&self, asset_path: &str) -> PathBuf {
        self.record_path(asset_path).with_extension("skip")
    }

    /// The skip note of `asset`, if it has one for this exact file.
    pub fn skip_note(&self, asset: &Asset) -> Option<SkipNote> {
        let file = File::open(self.skip_path(&asset.path)).ok()?;
        let note = read_skip(&mut BufReader::new(file)).ok()?;
        (note.source == *asset).then_some(note)
    }

    pub fn save_skip(&self, note: &SkipNote) -> Result<(), StoreError> {
        write_atomically(&self.skip_path(&note.source.path), |out| {
            write_skip(out, note)
        })
    }

    fn tags_path(&self, asset_path: &str) -> PathBuf {
        self.record_path(asset_path).with_extension("tags")
    }

    /// The tags `index` read from this exact file, if it read them.
    pub fn tags(&self, asset: &Asset) -> Option<Tags> {
        let file = File::open(self.tags_path(&asset.path)).ok()?;
        let note = read_tags(&mut BufReader::new(file)).ok()?;
        (note.source == *asset).then_some(note.tags)
    }

    pub fn save_tags(&self, note: &TagNote) -> Result<(), StoreError> {
        write_atomically(&self.tags_path(&note.source.path), |out| {
            write_tags(out, note)
        })
    }

    fn forget_skip(&self, asset_path: &str) -> Result<(), StoreError> {
        let path = self.skip_path(asset_path);
        match fs::remove_file(&path) {
            Err(source) if source.kind() != io::ErrorKind::NotFound => {
                Err(StoreError::Io { path, source })
            }
            _ => Ok(()),
        }
    }

    /// Every file in the store, reading only headers, sorted by name.
    pub fn survey(&self) -> Result<Vec<Stored>, StoreError> {
        let mut files = self.files()?;
        files.sort();
        Ok(files.into_iter().map(survey_file).collect())
    }

    /// The files this store holds a current record of under `profile`,
    /// sorted by path as a library lists them, reading only headers. Without
    /// the library there is no telling whether a file has since changed or
    /// been deleted, so their records are among them. Records whose header
    /// cannot be read are returned as errors.
    pub fn current_sources(
        &self,
        profile: &Profile,
    ) -> Result<(Vec<Asset>, Vec<StoreError>), StoreError> {
        let mut sources = Vec::new();
        let mut problems = Vec::new();
        for file in self.files()? {
            if file
                .extension()
                .is_none_or(|extension| extension != "peaks")
            {
                continue;
            }
            match survey_file(file) {
                Stored::Record { file, header }
                    if header.profile == profile.id()
                        && file == self.record_path(&header.source.path) =>
                {
                    sources.push(header.source);
                }
                Stored::Unreadable { file, reason } => {
                    problems.push(StoreError::Corrupt { path: file, reason });
                }
                _ => {}
            }
        }
        sources.sort_by(|a, b| a.path.cmp(&b.path));
        Ok((sources, problems))
    }

    /// The paths of the store's records and notes, unsorted.
    fn files(&self) -> Result<Vec<PathBuf>, StoreError> {
        let io_error = |source| StoreError::Io {
            path: self.dir.clone(),
            source,
        };
        fs::read_dir(&self.dir)
            .map_err(io_error)?
            .map(|entry| entry.map(|entry| entry.path()))
            .filter(|path| {
                path.as_ref()
                    .map_or(true, |path| !path.ends_with(LIBRARY_FILE))
            })
            .collect::<Result<_, _>>()
            .map_err(io_error)
    }

    /// Deletes a file `survey` listed.
    pub fn remove(&self, stored: &Stored) -> Result<(), StoreError> {
        let path = stored.file();
        fs::remove_file(path).map_err(|source| StoreError::Io {
            path: path.to_owned(),
            source,
        })
    }
}

/// A library's name in the store: its root as an absolute path, so that the
/// same library reached from another directory has the same name. A root
/// that cannot be read, such as an unmounted share, keeps its name.
fn library_name(root: &Path) -> String {
    root.canonicalize()
        .or_else(|_| std::path::absolute(root))
        .unwrap_or_else(|_| root.to_path_buf())
        .to_string_lossy()
        .into_owned()
}

fn survey_file(file: PathBuf) -> Stored {
    let extension = file
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default();
    if extension.starts_with("tmp") {
        return Stored::Temporary { file };
    }
    let mut reader = match File::open(&file) {
        Ok(opened) => BufReader::new(opened),
        Err(error) => {
            return Stored::Unreadable {
                file,
                reason: error.to_string(),
            };
        }
    };
    let read = match extension {
        "peaks" => read_header(&mut reader).map(|header| Stored::Record {
            file: file.clone(),
            header,
        }),
        "skip" => read_skip(&mut reader).map(|note| Stored::Skip {
            file: file.clone(),
            note,
        }),
        "tags" => read_tags(&mut reader).map(|note| Stored::Tags {
            file: file.clone(),
            note,
        }),
        _ => Err(invalid("not a gunfinger file")),
    };
    read.unwrap_or_else(|error| Stored::Unreadable {
        file,
        reason: error.to_string(),
    })
}

/// Writes through a temporary file and a rename, so a crash never leaves a
/// half-written file under the final name.
fn write_atomically(
    path: &Path,
    write: impl FnOnce(&mut BufWriter<File>) -> io::Result<()>,
) -> Result<(), StoreError> {
    let temporary = path.with_extension(format!("tmp{}", std::process::id()));
    let io_error = |source| StoreError::Io {
        path: temporary.clone(),
        source,
    };
    let mut writer = BufWriter::new(File::create(&temporary).map_err(io_error)?);
    write(&mut writer).map_err(io_error)?;
    let file = writer
        .into_inner()
        .map_err(|error| io_error(error.into_error()))?;
    file.sync_all().map_err(io_error)?;
    fs::rename(&temporary, path).map_err(io_error)
}

pub(crate) fn write_source(out: &mut impl Write, source: &Asset) -> io::Result<()> {
    write_text(out, &source.path)?;
    out.write_all(&source.size.to_le_bytes())?;
    out.write_all(&source.modified.seconds.to_le_bytes())?;
    out.write_all(&source.modified.nanos.to_le_bytes())
}

pub(crate) fn read_source(input: &mut impl Read) -> io::Result<Asset> {
    let path = read_text(input)?;
    let size = u64::from_le_bytes(read_array(input)?);
    let seconds = i64::from_le_bytes(read_array(input)?);
    let nanos = u32::from_le_bytes(read_array(input)?);
    Ok(Asset {
        path,
        size,
        modified: Timestamp { seconds, nanos },
    })
}

fn write_skip(out: &mut impl Write, note: &SkipNote) -> io::Result<()> {
    out.write_all(SKIP_MAGIC)?;
    out.write_all(&SKIP_FORMAT_VERSION.to_le_bytes())?;
    write_source(out, &note.source)?;
    match &note.reason {
        SkipReason::Failed(message) => {
            out.write_all(&[0])?;
            write_text(out, clipped(message, MAX_REASON_BYTES))
        }
        SkipReason::TooLong { limit } => {
            out.write_all(&[1])?;
            out.write_all(&limit.as_secs_f64().to_le_bytes())
        }
        SkipReason::TooShort { limit } => {
            out.write_all(&[2])?;
            out.write_all(&limit.as_secs_f64().to_le_bytes())
        }
    }
}

fn read_skip(input: &mut impl Read) -> io::Result<SkipNote> {
    let mut magic = [0; 8];
    input.read_exact(&mut magic)?;
    if &magic != SKIP_MAGIC {
        return Err(invalid("not a skip note"));
    }
    if u16::from_le_bytes(read_array(input)?) != SKIP_FORMAT_VERSION {
        return Err(invalid("unsupported skip note version"));
    }
    let source = read_source(input)?;
    let reason = match read_array(input)? {
        [0] => SkipReason::Failed(read_text(input)?),
        [1] => SkipReason::TooLong {
            limit: read_limit(input)?,
        },
        [2] => SkipReason::TooShort {
            limit: read_limit(input)?,
        },
        _ => return Err(invalid("unknown skip reason")),
    };
    Ok(SkipNote { source, reason })
}

fn read_limit(input: &mut impl Read) -> io::Result<Duration> {
    Duration::try_from_secs_f64(f64::from_le_bytes(read_array(input)?))
        .map_err(|_| invalid("invalid length limit"))
}

fn write_record(out: &mut impl Write, record: &PeakRecord) -> io::Result<()> {
    let header = &record.header;
    out.write_all(MAGIC)?;
    out.write_all(&FORMAT_VERSION.to_le_bytes())?;
    write_text(out, &header.profile)?;
    write_source(out, &header.source)?;
    out.write_all(&header.duration_seconds.to_le_bytes())?;
    let count = u32::try_from(record.peaks.len()).map_err(io::Error::other)?;
    out.write_all(&count.to_le_bytes())?;

    let mut previous_frame = 0;
    for peak in &record.peaks {
        let frame = peak.frame.round() as u32;
        let delta = frame.checked_sub(previous_frame).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "peaks are not ordered by frame",
            )
        })?;
        write_varint(out, delta)?;
        let offset = ((peak.frame - f64::from(frame)) * f64::from(FRAME_STEPS)).round() as i8;
        out.write_all(&offset.to_le_bytes())?;
        out.write_all(&((peak.bin * BIN_STEPS) as u16).to_le_bytes())?;
        let magnitude = (peak.magnitude - MAGNITUDE_ORIGIN_DB) * MAGNITUDE_STEPS_PER_DB;
        out.write_all(&[magnitude.clamp(0.0, 255.0) as u8])?;
        previous_frame = frame;
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
    let source = read_source(input)?;
    let duration_seconds = f64::from_le_bytes(read_array(input)?);
    Ok(RecordHeader {
        profile,
        source,
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
        let offset = f64::from(i8::from_le_bytes(read_array(input)?)) / f64::from(FRAME_STEPS);
        let bin = f32::from(u16::from_le_bytes(read_array(input)?)) / BIN_STEPS;
        let [magnitude] = read_array(input)?;
        peaks.push(Peak {
            frame: f64::from(frame) + offset,
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

fn write_tags(out: &mut impl Write, note: &TagNote) -> io::Result<()> {
    out.write_all(TAGS_MAGIC)?;
    out.write_all(&TAGS_FORMAT_VERSION.to_le_bytes())?;
    write_source(out, &note.source)?;
    let Tags {
        artist,
        title,
        album,
    } = &note.tags;
    for tag in [artist, title, album] {
        match tag {
            Some(text) => {
                out.write_all(&[1])?;
                write_text(out, clipped(text, MAX_TAG_BYTES))?;
            }
            None => out.write_all(&[0])?,
        }
    }
    Ok(())
}

fn read_tags(input: &mut impl Read) -> io::Result<TagNote> {
    let mut magic = [0; 8];
    input.read_exact(&mut magic)?;
    if &magic != TAGS_MAGIC {
        return Err(invalid("not a tag note"));
    }
    if u16::from_le_bytes(read_array(input)?) != TAGS_FORMAT_VERSION {
        return Err(invalid("unsupported tag note version"));
    }
    let source = read_source(input)?;
    let mut tag = || -> io::Result<Option<String>> {
        match read_array(input)? {
            [0] => Ok(None),
            [1] => read_text(input).map(Some),
            _ => Err(invalid("unknown tag marker")),
        }
    };
    let tags = Tags {
        artist: tag()?,
        title: tag()?,
        album: tag()?,
    };
    Ok(TagNote { source, tags })
}

/// The start of `text`, at most `max` bytes, cut at a character boundary.
fn clipped(text: &str, max: usize) -> &str {
    let mut end = text.len().min(max);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

pub(crate) fn write_text(out: &mut impl Write, text: &str) -> io::Result<()> {
    let length = u16::try_from(text.len()).map_err(io::Error::other)?;
    out.write_all(&length.to_le_bytes())?;
    out.write_all(text.as_bytes())
}

pub(crate) fn read_text(input: &mut impl Read) -> io::Result<String> {
    let length = u16::from_le_bytes(read_array(input)?);
    let mut bytes = vec![0; usize::from(length)];
    input.read_exact(&mut bytes)?;
    String::from_utf8(bytes).map_err(|_| invalid("text is not UTF-8"))
}

pub(crate) fn read_array<const N: usize>(input: &mut impl Read) -> io::Result<[u8; N]> {
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
pub(crate) fn fnv1a(bytes: &[u8]) -> u64 {
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
                frame: 0.0,
                bin: 5.0,
                magnitude: -10.0,
            },
            Peak {
                frame: 0.0,
                bin: 499.0 + 63.0 / 64.0,
                magnitude: 54.5,
            },
            Peak {
                frame: 1.0 - 31.0 / 64.0,
                bin: 6.0,
                magnitude: 0.0,
            },
            Peak {
                frame: 70_000.0 + 31.0 / 64.0,
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
            frame: 3.25,
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

    #[test]
    fn a_skip_note_lasts_until_the_file_changes_or_peaks_are_saved() {
        let dir = std::env::temp_dir().join(format!("gunfinger-skip-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let failed = SkipNote {
            source: asset(),
            reason: SkipReason::Failed(String::from("ffmpeg failed: invalid data")),
        };
        let mut touched = asset();
        touched.size += 1;

        store.save_skip(&failed).unwrap();
        let remembered = store.skip_note(&asset());
        let for_changed_file = store.skip_note(&touched);
        let loaded = store.load(&asset(), &Profile::CURRENT);
        let loaded_changed = store.load(&touched, &Profile::CURRENT);
        let surveyed = store.survey().unwrap().len();
        store.save(&record(Vec::new())).unwrap();
        let after_save = store.skip_note(&asset());
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(remembered, Some(failed));
        assert_eq!(for_changed_file, None);
        assert!(
            matches!(loaded, Err(StoreError::Skipped { .. })),
            "{loaded:?}"
        );
        assert!(
            matches!(loaded_changed, Err(StoreError::Missing { .. })),
            "{loaded_changed:?}"
        );
        assert_eq!(surveyed, 1);
        assert_eq!(after_save, None);
    }

    #[test]
    fn tags_are_kept_for_the_exact_file_and_surveyed() {
        let dir = std::env::temp_dir().join(format!("gunfinger-tags-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let note = TagNote {
            source: asset(),
            tags: Tags {
                artist: Some(String::from("Bad Company")),
                title: Some(String::from("The Nine")),
                album: None,
            },
        };
        let mut touched = asset();
        touched.size += 1;

        store.save_tags(&note).unwrap();
        let read = store.tags(&asset());
        let for_changed_file = store.tags(&touched);
        let surveyed = store.survey().unwrap();
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(read, Some(note.tags));
        assert_eq!(for_changed_file, None);
        assert!(
            matches!(&surveyed[..], [Stored::Tags { note: found, .. }] if found.source == asset()),
            "{surveyed:?}"
        );
    }

    #[test]
    fn length_notes_keep_their_limits() {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-length-notes-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let mut sample = asset();
        sample.path = String::from("loops/amen.wav");
        let short = SkipNote {
            source: sample.clone(),
            reason: SkipReason::TooShort {
                limit: Duration::from_secs(90),
            },
        };
        let long = SkipNote {
            source: asset(),
            reason: SkipReason::TooLong {
                limit: Duration::from_secs(900),
            },
        };

        store.save_skip(&short).unwrap();
        store.save_skip(&long).unwrap();
        let remembered_short = store.skip_note(&sample);
        let remembered_long = store.skip_note(&asset());
        fs::remove_dir_all(&dir).unwrap();

        assert_eq!(short.reason.to_string(), "shorter than 1:30");
        assert_eq!(long.reason.to_string(), "longer than 15:00");
        assert_eq!(remembered_short, Some(short));
        assert_eq!(remembered_long, Some(long));
    }

    #[test]
    fn a_store_lists_the_files_of_its_current_records_by_path() {
        let dir = std::env::temp_dir().join(format!("gunfinger-sources-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        let mut first = record(Vec::new());
        first.header.source.path = String::from("a/first.wav");
        let mut old = record(Vec::new());
        old.header.source.path = String::from("b/old.wav");
        old.header.profile = Profile {
            hop: 256,
            ..Profile::CURRENT
        }
        .id();

        store.save(&record(Vec::new())).unwrap();
        store.save(&first).unwrap();
        store.save(&old).unwrap();
        store
            .save_tags(&TagNote {
                source: asset(),
                tags: Tags::default(),
            })
            .unwrap();
        fs::write(dir.join("0000000000000000.peaks"), "not a record").unwrap();
        let listed = store.current_sources(&Profile::CURRENT);
        fs::remove_dir_all(&dir).unwrap();

        let (sources, problems) = listed.unwrap();
        assert_eq!(sources, [first.header.source, asset()]);
        assert!(
            matches!(&problems[..], [StoreError::Corrupt { .. }]),
            "{problems:?}"
        );
    }

    #[test]
    fn the_survey_tells_records_notes_and_leftovers_apart() {
        let dir = std::env::temp_dir().join(format!("gunfinger-survey-{}", std::process::id()));
        let store = PeakStore::open(&dir).unwrap();
        store.save(&record(Vec::new())).unwrap();
        let mut long = asset();
        long.path = String::from("mixes/two-hour-mix.mp3");
        store
            .save_skip(&SkipNote {
                source: long,
                reason: SkipReason::TooLong {
                    limit: Duration::from_secs(1200),
                },
            })
            .unwrap();
        fs::write(dir.join("0123456789abcdef.tmp4242"), b"half").unwrap();
        fs::write(dir.join("notes.txt"), b"hello").unwrap();

        let kinds: Vec<&str> = store
            .survey()
            .unwrap()
            .iter()
            .map(|stored| match stored {
                Stored::Record { .. } => "record",
                Stored::Skip { .. } => "skip",
                Stored::Tags { .. } => "tags",
                Stored::Unreadable { .. } => "unreadable",
                Stored::Temporary { .. } => "temporary",
            })
            .collect();
        fs::remove_dir_all(&dir).unwrap();

        let mut sorted = kinds.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, ["record", "skip", "temporary", "unreadable"]);
    }

    #[test]
    fn a_store_adopts_the_first_library_indexed_into_it_and_refuses_others() {
        let dir = std::env::temp_dir().join(format!("gunfinger-claim-{}", std::process::id()));
        let (first, second) = (dir.join("first"), dir.join("second"));
        fs::create_dir_all(&first).unwrap();
        fs::create_dir_all(&second).unwrap();
        let store = PeakStore::open(&dir.join("peaks")).unwrap();
        store.save(&record(Vec::new())).unwrap();

        let before = store.check_library(&second);
        store.claim_library(&first).unwrap();
        let again = store.claim_library(&first.join("..").join("first"));
        let other = store.check_library(&second);
        let records = store.survey().unwrap().len();
        fs::remove_dir_all(&dir).unwrap();

        assert!(before.is_ok(), "a store that names no library passes");
        assert!(again.is_ok(), "the same library by another path");
        assert!(matches!(other, Err(StoreError::OtherLibrary { .. })));
        assert_eq!(records, 1, "the survey passes over library.txt");
    }

    proptest! {
        #[test]
        fn quantised_peaks_round_trip_exactly(
            raw in prop::collection::vec((0_u32..2_000, -31_i8..=31, 0_u16..32_000, 0_u8..=255), 0..200)
        ) {
            let mut frame = 0;
            let peaks: Vec<Peak> = raw
                .into_iter()
                .map(|(delta, offset, bin, magnitude)| {
                    frame += delta;
                    Peak {
                        frame: f64::from(frame) + f64::from(offset) / f64::from(FRAME_STEPS),
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

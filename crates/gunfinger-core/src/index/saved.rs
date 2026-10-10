//! The index saved to a file, so that a search loads it instead of building
//! it from the peak store again (ADR 0011).
//!
//! The file holds the in-memory layout unchanged, little-endian: the offsets
//! table (`2^21 + 1` posting positions) and the 4-byte postings, after a
//! header naming what the index was built from: the file format, the
//! front-end profile, the hash design, the track length range and the asset
//! table (each file's path, size and modification time, as peak records name
//! them, and its duration), in asset order. A digest of everything before it
//! ends the file. `load` returns an index only when the header names exactly
//! what a build now would use and the file reads back whole; otherwise the
//! caller builds the index and saves it over the file.
//!
//! As for the peak store: sequential reads and writes, no mmap, and writes
//! through a temporary file and a rename.

use std::fs::{self, File};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;

use super::{FRAME_BITS, Index, IndexedAsset, Posting};
use crate::hash::HASH_BITS;
use crate::indexing::TrackLength;
use crate::library::Asset;
use crate::store::{fnv1a, read_array, read_source, read_text, write_source, write_text};

const MAGIC: &[u8; 8] = b"GFINDEX\0";
/// Changes with the layout of the file or of a posting.
const FORMAT: u32 = 1;
const OFFSETS: usize = (1 << HASH_BITS) + 1;
/// Tables are read and written this many 4-byte words at a time.
const CHUNK_WORDS: usize = 1 << 20;

/// What an index is built from. Indexes built from the same provenance hold
/// the same postings.
#[derive(Debug, Clone, PartialEq)]
pub struct Provenance {
    /// The front-end profile (`Profile::id`).
    pub profile: String,
    /// The hash design (`hash::design`).
    pub hash_design: String,
    pub track_length: TrackLength,
    /// The files indexed, in the order of their asset ids.
    pub sources: Vec<Asset>,
}

/// Why a saved index is not used.
#[derive(Debug, thiserror::Error)]
pub enum Unusable {
    #[error("there is no saved index")]
    Missing,
    #[error("the saved index is out of date: {0}")]
    Stale(String),
    #[error("the saved index cannot be read: {0}")]
    Unreadable(String),
}

/// The file name of the saved index of the peak store at `store`, an
/// absolute path: the store folder's name, for people, and a digest of the
/// whole path, so that each store has one file and no two share it.
pub fn file_name(store: &Path) -> String {
    let name = store.file_name().map_or_else(
        || String::from("store"),
        |name| name.to_string_lossy().into_owned(),
    );
    format!(
        "{name}-{:016x}.index",
        fnv1a(store.as_os_str().as_encoded_bytes())
    )
}

/// Writes `index`, built from `provenance`, to `path` through a temporary
/// file in the same folder and a rename, so that a crash or a full disk
/// never leaves part of an index under the final name. Temporary files of
/// earlier writes that did not finish are deleted first.
pub fn save(path: &Path, provenance: &Provenance, index: &Index) -> io::Result<()> {
    if provenance.sources.len() != index.assets.len()
        || provenance
            .sources
            .iter()
            .zip(&index.assets)
            .any(|(source, asset)| source.path != asset.path)
    {
        return Err(io::Error::other(
            "the provenance does not name the index's assets",
        ));
    }
    remove_unfinished(path);
    let temporary = temporary_path(path, std::process::id());
    let written =
        write_file(&temporary, provenance, index).and_then(|()| fs::rename(&temporary, path));
    if written.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    written
}

/// The index saved at `path`, when it was built from `expected` and reads
/// back whole.
pub fn load(path: &Path, expected: &Provenance) -> Result<Index, Unusable> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Err(Unusable::Missing),
        Err(error) => return Err(Unusable::Unreadable(error.to_string())),
    };
    let file_bytes = file
        .metadata()
        .map_err(|error| Unusable::Unreadable(error.to_string()))?
        .len();
    let mut input = Digesting {
        inner: BufReader::with_capacity(4 * CHUNK_WORDS, file),
        digest: Digest::new(),
        read: 0,
    };
    let assets = read_header(&mut input, expected).map_err(HeaderError::into_unusable)?;
    let offsets = read_words(&mut input, OFFSETS).map_err(unreadable)?;
    check_offsets(&offsets)?;
    let posting_count = u64::from_le_bytes(input.array().map_err(unreadable)?);
    let expected_bytes = input.read + 4 * posting_count + 8;
    if file_bytes != expected_bytes {
        return Err(Unusable::Unreadable(format!(
            "it holds {file_bytes} bytes where its header needs {expected_bytes}"
        )));
    }
    if posting_count != u64::from(offsets[OFFSETS - 1]) {
        return Err(Unusable::Unreadable(String::from(
            "its posting count disagrees with its offsets table",
        )));
    }
    let postings = read_postings(&mut input, offsets[OFFSETS - 1] as usize, assets.len())?;
    let computed = input.digest.0;
    let stored = u64::from_le_bytes(read_array(&mut input.inner).map_err(unreadable)?);
    if stored != computed {
        return Err(Unusable::Unreadable(String::from(
            "its digest does not match its contents",
        )));
    }
    Ok(Index {
        assets,
        offsets,
        postings,
        longest_scanned: u32::MAX,
    })
}

fn write_file(path: &Path, provenance: &Provenance, index: &Index) -> io::Result<()> {
    let mut header = Vec::new();
    header.write_all(MAGIC)?;
    header.write_all(&FORMAT.to_le_bytes())?;
    write_text(&mut header, &provenance.profile)?;
    write_text(&mut header, &provenance.hash_design)?;
    header.write_all(&FRAME_BITS.to_le_bytes())?;
    write_length(&mut header, provenance.track_length.min)?;
    write_length(&mut header, provenance.track_length.max)?;
    let count = u32::try_from(index.assets.len()).map_err(io::Error::other)?;
    header.write_all(&count.to_le_bytes())?;
    for (source, asset) in provenance.sources.iter().zip(&index.assets) {
        write_source(&mut header, source)?;
        header.write_all(&asset.duration_seconds.to_le_bytes())?;
    }
    let mut digest = Digest::new();
    digest.bytes(&header);
    let mut out = BufWriter::with_capacity(4 * CHUNK_WORDS, File::create(path)?);
    out.write_all(&header)?;
    write_words(&mut out, &mut digest, &index.offsets)?;
    let posting_count = index.postings.len() as u64;
    out.write_all(&posting_count.to_le_bytes())?;
    digest.bytes(&posting_count.to_le_bytes());
    for chunk in index.postings.chunks(CHUNK_WORDS) {
        let words: Vec<u32> = chunk.iter().map(|posting| posting.0).collect();
        write_words(&mut out, &mut digest, &words)?;
    }
    out.write_all(&digest.0.to_le_bytes())?;
    let file = out.into_inner().map_err(io::IntoInnerError::into_error)?;
    file.sync_all()
}

fn write_words(out: &mut impl Write, digest: &mut Digest, words: &[u32]) -> io::Result<()> {
    let mut bytes = Vec::with_capacity(4 * CHUNK_WORDS.min(words.len()));
    for chunk in words.chunks(CHUNK_WORDS) {
        bytes.clear();
        for &word in chunk {
            digest.word(word);
            bytes.extend_from_slice(&word.to_le_bytes());
        }
        out.write_all(&bytes)?;
    }
    Ok(())
}

fn write_length(out: &mut impl Write, length: Duration) -> io::Result<()> {
    let nanos = u64::try_from(length.as_nanos()).map_err(io::Error::other)?;
    out.write_all(&nanos.to_le_bytes())
}

/// The header's asset table, when the header names `expected`.
fn read_header(
    input: &mut Digesting<impl Read>,
    expected: &Provenance,
) -> Result<Vec<IndexedAsset>, HeaderError> {
    if &input.array::<8>()? != MAGIC {
        return Err(HeaderError::Unreadable(String::from(
            "it is not a saved index",
        )));
    }
    let format = u32::from_le_bytes(input.array()?);
    if format != FORMAT {
        return Err(HeaderError::Stale(format!(
            "it has format {format}, this version reads {FORMAT}"
        )));
    }
    if read_text(input)? != expected.profile {
        return Err(stale("it was built under another front-end profile"));
    }
    if read_text(input)? != expected.hash_design {
        return Err(stale("it was built under another hash design"));
    }
    if u32::from_le_bytes(input.array()?) != FRAME_BITS {
        return Err(stale("its postings split frame and asset bits differently"));
    }
    let track_length = TrackLength {
        min: read_length(input)?,
        max: read_length(input)?,
    };
    if track_length != expected.track_length {
        return Err(HeaderError::Stale(format!(
            "it holds the tracks {track_length} long, this search those {} long",
            expected.track_length
        )));
    }
    let count = u32::from_le_bytes(input.array()?) as usize;
    if count != expected.sources.len() {
        return Err(HeaderError::Stale(format!(
            "it holds {count} files, the peak store {} now",
            expected.sources.len()
        )));
    }
    let mut assets = Vec::with_capacity(count);
    for wanted in &expected.sources {
        let source = read_source(input)?;
        let duration_seconds = f64::from_le_bytes(input.array()?);
        if source != *wanted {
            return Err(HeaderError::Stale(format!(
                "{} is not the file it holds in its place",
                wanted.path
            )));
        }
        assets.push(IndexedAsset {
            path: source.path,
            duration_seconds,
        });
    }
    Ok(assets)
}

fn read_length(input: &mut impl Read) -> io::Result<Duration> {
    Ok(Duration::from_nanos(u64::from_le_bytes(read_array(input)?)))
}

fn read_words(input: &mut Digesting<impl Read>, count: usize) -> io::Result<Vec<u32>> {
    let mut words = Vec::with_capacity(count);
    let mut bytes = vec![0; 4 * CHUNK_WORDS.min(count)];
    while words.len() < count {
        let chunk = &mut bytes[..4 * CHUNK_WORDS.min(count - words.len())];
        input.inner.read_exact(chunk)?;
        input.read += chunk.len() as u64;
        for word in chunk.chunks_exact(4) {
            let word = u32::from_le_bytes([word[0], word[1], word[2], word[3]]);
            input.digest.word(word);
            words.push(word);
        }
    }
    Ok(words)
}

/// The postings, each checked to name an asset of the table, so that a
/// damaged file cannot name an asset the index does not hold.
fn read_postings(
    input: &mut Digesting<impl Read>,
    count: usize,
    assets: usize,
) -> Result<Vec<Posting>, Unusable> {
    let mut postings = Vec::with_capacity(count);
    let mut bytes = vec![0; 4 * CHUNK_WORDS.min(count)];
    let mut damaged = false;
    while postings.len() < count {
        let chunk = &mut bytes[..4 * CHUNK_WORDS.min(count - postings.len())];
        input.inner.read_exact(chunk).map_err(unreadable)?;
        for word in chunk.chunks_exact(4) {
            let posting = Posting(u32::from_le_bytes([word[0], word[1], word[2], word[3]]));
            input.digest.word(posting.0);
            damaged |= posting.asset().0 as usize >= assets;
            postings.push(posting);
        }
    }
    if damaged {
        return Err(Unusable::Unreadable(String::from(
            "a posting names an asset it does not hold",
        )));
    }
    Ok(postings)
}

fn check_offsets(offsets: &[u32]) -> Result<(), Unusable> {
    if offsets[0] != 0 || offsets.windows(2).any(|pair| pair[1] < pair[0]) {
        return Err(Unusable::Unreadable(String::from(
            "its offsets table is out of order",
        )));
    }
    Ok(())
}

/// `<file name>.<process id>.partial`, beside the index.
fn temporary_path(path: &Path, process: u32) -> PathBuf {
    let mut name = path.file_name().unwrap_or_default().to_os_string();
    name.push(format!(".{process}.partial"));
    path.with_file_name(name)
}

/// Deletes what writes of this index left behind when they stopped before
/// the rename.
fn remove_unfinished(path: &Path) {
    let (Some(folder), Some(name)) = (path.parent(), path.file_name()) else {
        return;
    };
    let prefix = format!("{}.", name.to_string_lossy());
    let Ok(entries) = fs::read_dir(if folder.as_os_str().is_empty() {
        Path::new(".")
    } else {
        folder
    }) else {
        return;
    };
    for entry in entries.flatten() {
        let entry_name = entry.file_name();
        let entry_name = entry_name.to_string_lossy();
        if entry_name.starts_with(&prefix) && entry_name.ends_with(".partial") {
            let _ = fs::remove_file(entry.path());
        }
    }
}

/// How a header that cannot be used fails: out of date, or not readable.
enum HeaderError {
    Stale(String),
    Unreadable(String),
}

impl HeaderError {
    fn into_unusable(self) -> Unusable {
        match self {
            HeaderError::Stale(why) => Unusable::Stale(why),
            HeaderError::Unreadable(why) => Unusable::Unreadable(why),
        }
    }
}

impl From<io::Error> for HeaderError {
    fn from(error: io::Error) -> HeaderError {
        HeaderError::Unreadable(describe(&error))
    }
}

fn stale(why: &str) -> HeaderError {
    HeaderError::Stale(why.to_owned())
}

#[allow(
    clippy::needless_pass_by_value,
    reason = "`map_err` passes the error by value"
)]
fn unreadable(error: io::Error) -> Unusable {
    Unusable::Unreadable(describe(&error))
}

fn describe(error: &io::Error) -> String {
    if error.kind() == io::ErrorKind::UnexpectedEof {
        String::from("it ends early")
    } else {
        error.to_string()
    }
}

/// A reader that keeps the digest of the header bytes it reads, and their
/// count. The tables are read from `inner` directly, a word at a time.
struct Digesting<R> {
    inner: R,
    digest: Digest,
    read: u64,
}

impl<R: Read> Digesting<R> {
    fn array<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        read_array(self)
    }
}

impl<R: Read> Read for Digesting<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        let count = self.inner.read(buffer)?;
        self.digest.bytes(&buffer[..count]);
        self.read += count as u64;
        Ok(count)
    }
}

/// FNV-1a, over the header byte by byte and over the tables a 4-byte word
/// at a time: one multiplication per posting keeps the digest cheap beside
/// reading 4 bytes.
struct Digest(u64);

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

impl Digest {
    fn new() -> Digest {
        Digest(FNV_OFFSET)
    }

    fn bytes(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.0 = (self.0 ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
        }
    }

    fn word(&mut self, word: u32) {
        self.0 = (self.0 ^ u64::from(word)).wrapping_mul(FNV_PRIME);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::Timestamp;
    use crate::peaks::Peak;
    use crate::store::{PeakRecord, RecordHeader};

    const LENGTH: TrackLength = TrackLength {
        min: Duration::from_secs(90),
        max: Duration::from_secs(900),
    };

    fn source(path: &str, seconds: i64) -> Asset {
        Asset {
            path: path.to_owned(),
            size: 1000,
            modified: Timestamp { seconds, nanos: 0 },
        }
    }

    fn record(source: &Asset, offset: f64) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: String::from("profile"),
                source: source.clone(),
                duration_seconds: 200.0,
            },
            peaks: (0..40)
                .map(|step| Peak {
                    frame: offset + f64::from(step) * 7.0,
                    bin: 20.0 + (step * 13 % 90) as f32,
                    magnitude: 0.0,
                })
                .collect(),
        }
    }

    fn library() -> (Provenance, Index) {
        let sources = vec![source("a.mp3", 1), source("b/c.mp3", 2)];
        let records = [record(&sources[0], 0.0), record(&sources[1], 300.0)];
        let provenance = Provenance {
            profile: String::from("profile"),
            hash_design: String::from("design"),
            track_length: LENGTH,
            sources,
        };
        (provenance, Index::build(&records).unwrap())
    }

    fn scratch(test: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-saved-{test}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn same(a: &Index, b: &Index) -> bool {
        a.assets == b.assets && a.offsets == b.offsets && a.postings == b.postings
    }

    #[test]
    fn a_saved_index_loads_as_it_was_built() {
        let dir = scratch("round-trip");
        let path = dir.join("library.index");
        let (provenance, index) = library();

        save(&path, &provenance, &index).unwrap();
        let loaded = load(&path, &provenance).unwrap();

        assert!(same(&loaded, &index));
        assert!(index.posting_count() > 0);
        assert_eq!(
            fs::read_dir(&dir).unwrap().count(),
            1,
            "no temporary file left"
        );
    }

    #[test]
    fn each_store_has_its_own_file_name() {
        let name = file_name(Path::new("/Users/me/.local/share/gunfinger/nas-dnb-peaks"));
        let other = file_name(Path::new("/Volumes/copy/nas-dnb-peaks"));

        assert!(
            name.starts_with("nas-dnb-peaks-") && name.ends_with(".index"),
            "{name}"
        );
        assert_eq!(name.len(), "nas-dnb-peaks-".len() + 16 + ".index".len());
        assert_ne!(name, other);
    }

    #[test]
    fn a_missing_file_is_missing() {
        let dir = scratch("missing");
        let (provenance, _) = library();

        let loaded = load(&dir.join("library.index"), &provenance);

        assert!(matches!(loaded, Err(Unusable::Missing)));
    }

    #[test]
    fn a_header_naming_another_build_is_stale() {
        let dir = scratch("stale");
        let path = dir.join("library.index");
        let (provenance, index) = library();
        save(&path, &provenance, &index).unwrap();

        let profile = Provenance {
            profile: String::from("another profile"),
            ..provenance.clone()
        };
        let design = Provenance {
            hash_design: String::from("another design"),
            ..provenance.clone()
        };
        let length = Provenance {
            track_length: TrackLength {
                min: Duration::ZERO,
                max: LENGTH.max,
            },
            ..provenance.clone()
        };

        for expected in [profile, design, length] {
            let loaded = load(&path, &expected);
            assert!(
                matches!(loaded, Err(Unusable::Stale(_))),
                "{:?}",
                loaded.err()
            );
        }
    }

    #[test]
    fn a_changed_store_makes_the_saved_index_stale() {
        let dir = scratch("changed");
        let path = dir.join("library.index");
        let (provenance, index) = library();
        save(&path, &provenance, &index).unwrap();

        let mut retouched = provenance.clone();
        retouched.sources[1].modified.seconds += 1;
        let mut added = provenance.clone();
        added.sources.push(source("d.mp3", 3));
        let mut removed = provenance.clone();
        removed.sources.pop();

        let message = load(&path, &retouched).err().unwrap().to_string();
        assert!(message.contains("b/c.mp3"), "{message}");
        for expected in [added, removed] {
            assert!(matches!(load(&path, &expected), Err(Unusable::Stale(_))));
        }
    }

    #[test]
    fn a_truncated_file_is_unreadable_wherever_it_ends() {
        let dir = scratch("truncated");
        let path = dir.join("library.index");
        let (provenance, index) = library();
        save(&path, &provenance, &index).unwrap();
        let whole = fs::read(&path).unwrap();
        let header = 8 + 4 + 40;
        let offsets_end = whole.len() - 8 - 4 * index.posting_count() - 8;

        for cut in [
            0,
            5,
            header,
            offsets_end - 3,
            offsets_end + 4,
            whole.len() - 9,
            whole.len() - 1,
        ] {
            fs::write(&path, &whole[..cut]).unwrap();
            let loaded = load(&path, &provenance);
            assert!(
                matches!(loaded, Err(Unusable::Unreadable(_))),
                "cut at {cut} of {}: {:?}",
                whole.len(),
                loaded.err()
            );
        }
    }

    #[test]
    fn a_damaged_posting_is_unreadable() {
        let dir = scratch("damaged");
        let path = dir.join("library.index");
        let (provenance, index) = library();
        save(&path, &provenance, &index).unwrap();
        let whole = fs::read(&path).unwrap();
        let last_posting = whole.len() - 8 - 4;

        let mut frame = whole.clone();
        frame[last_posting] ^= 1;
        fs::write(&path, &frame).unwrap();
        let flipped = load(&path, &provenance).err().unwrap().to_string();

        let mut asset = whole;
        asset[last_posting + 3] = 0xff;
        fs::write(&path, &asset).unwrap();
        let foreign = load(&path, &provenance).err().unwrap().to_string();

        assert!(flipped.contains("digest"), "{flipped}");
        assert!(foreign.contains("an asset it does not hold"), "{foreign}");
    }

    #[test]
    fn saving_again_replaces_the_file_and_what_unfinished_writes_left() {
        let dir = scratch("replace");
        let path = dir.join("library.index");
        let (provenance, index) = library();
        fs::write(&path, b"old").unwrap();
        fs::write(temporary_path(&path, 1), b"left by a crash").unwrap();
        fs::write(dir.join("other.index"), b"another library").unwrap();

        save(&path, &provenance, &index).unwrap();

        assert!(same(&load(&path, &provenance).unwrap(), &index));
        let mut names: Vec<String> = fs::read_dir(&dir)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        names.sort();
        assert_eq!(names, ["library.index", "other.index"]);
    }
}

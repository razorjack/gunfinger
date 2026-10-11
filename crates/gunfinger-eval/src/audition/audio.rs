//! Audio for the page, decoded by the `ffmpeg` executable (a child process
//! with explicit arguments, never a shell): windows of a file as WAV, which
//! every browser decodes, and waveform envelopes, cached under the work
//! directory.
//!
//! A window is 30 s of one file, numbered from its start: window `k` holds
//! file time `30k` to `30k + 30`. MP3 and Opus decoders need a few frames
//! to settle after a seek, so each window is decoded from 0.5 s earlier and
//! those frames dropped: consecutive windows then join without a click
//! (bit-exact for MP3 and Opus, measured in session 14).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::UNIX_EPOCH;

use gunfinger_core::decode::{Container, Excerpt, decode};
use serde::{Deserialize, Serialize};

use crate::audition::schema::Fnv1a;

pub const WINDOW_SECONDS: f64 = 30.0;
pub const SAMPLE_RATE: u32 = 44_100;
const CHANNELS: u16 = 2;
const BYTES_PER_FRAME: usize = 2 * CHANNELS as usize;
const PRE_ROLL_SECONDS: f64 = 0.5;

/// Envelopes show the shape of the music, not its treble: a mono decode
/// at 4 kHz, the minimum, maximum and RMS level of each 50 ms.
const ENVELOPE_RATE: u32 = 4_000;
const BUCKET_SECONDS: f64 = 0.05;

/// Window `window` of `path` as a 16-bit stereo WAV at 44.1 kHz; shorter at
/// the file's end, `None` past it. An error means the file could not be
/// decoded now (FFmpeg failed, the NAS is gone), not that it ends.
pub fn window_wav(path: &Path, window: u32) -> Result<Option<Vec<u8>>, String> {
    let start = f64::from(window) * WINDOW_SECONDS;
    let pre_roll = start.min(PRE_ROLL_SECONDS);
    let pcm = decode_pcm(path, start - pre_roll, WINDOW_SECONDS + pre_roll)?;
    let dropped = (pre_roll * f64::from(SAMPLE_RATE)).round() as usize * BYTES_PER_FRAME;
    let pcm = pcm.get(dropped..).unwrap_or_default();
    Ok((!pcm.is_empty()).then(|| wav(pcm)))
}

/// Raw 16-bit little-endian stereo PCM from `start`, `seconds` long.
fn decode_pcm(path: &Path, start: f64, seconds: f64) -> Result<Vec<u8>, String> {
    let output = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-threads", "1"])
        .arg("-ss")
        .arg(format!("{start:.6}"))
        .arg("-t")
        .arg(format!("{seconds:.6}"))
        .args(Container::of(path).input_args())
        .arg("-i")
        .arg(path)
        .args(["-map", "0:a:0", "-vn", "-sn", "-dn", "-ac"])
        .arg(CHANNELS.to_string())
        .arg("-ar")
        .arg(SAMPLE_RATE.to_string())
        .args(["-f", "s16le", "pipe:1"])
        .output()
        .map_err(|error| match error.kind() {
            std::io::ErrorKind::NotFound => String::from(
                "`ffmpeg` was not found on PATH; install FFmpeg (for example `brew install ffmpeg`)",
            ),
            _ => format!("could not run `ffmpeg` for {}: {error}", path.display()),
        })?;
    if !output.status.success() {
        let diagnostics = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "`ffmpeg` failed on {} ({}): {}",
            path.display(),
            output.status,
            diagnostics.trim()
        ));
    }
    let mut pcm = output.stdout;
    pcm.truncate(pcm.len() - pcm.len() % BYTES_PER_FRAME);
    Ok(pcm)
}

/// A canonical 44-byte RIFF header, then the samples.
fn wav(pcm: &[u8]) -> Vec<u8> {
    let data_bytes = pcm.len() as u32;
    let byte_rate = SAMPLE_RATE * BYTES_PER_FRAME as u32;
    let mut wav = Vec::with_capacity(44 + pcm.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + data_bytes).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16_u32.to_le_bytes());
    wav.extend_from_slice(&1_u16.to_le_bytes()); // PCM
    wav.extend_from_slice(&CHANNELS.to_le_bytes());
    wav.extend_from_slice(&SAMPLE_RATE.to_le_bytes());
    wav.extend_from_slice(&byte_rate.to_le_bytes());
    wav.extend_from_slice(&(BYTES_PER_FRAME as u16).to_le_bytes());
    wav.extend_from_slice(&16_u16.to_le_bytes()); // bits per sample
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&data_bytes.to_le_bytes());
    wav.extend_from_slice(pcm);
    wav
}

/// A file's waveform: the minimum and maximum of each bucket, from -127 to
/// 127, its RMS level from 0 to 127 (loud masters reach full scale in
/// every bucket; the RMS still shows their shape), and the file's decoded
/// length.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub bucket_seconds: f64,
    pub duration_seconds: f64,
    pub min: Vec<i8>,
    pub max: Vec<i8>,
    pub rms: Vec<i8>,
}

/// Envelopes cached in `dir`, one JSON file per audio file, named by a
/// digest of its path, size and modification time: a changed file gets a
/// new envelope.
#[derive(Debug, Clone)]
pub struct Envelopes {
    pub dir: PathBuf,
}

impl Envelopes {
    pub fn below(work: &Path) -> Envelopes {
        Envelopes {
            dir: work.join("audition").join("envelopes"),
        }
    }

    /// The envelope of `path`, from the cache or decoded and cached.
    pub fn get(&self, path: &Path) -> Result<Envelope, String> {
        let cached = self.dir.join(format!("{:016x}.json", cache_key(path)?));
        if let Ok(text) = fs::read_to_string(&cached)
            && let Ok(envelope) = serde_json::from_str(&text)
        {
            return Ok(envelope);
        }
        let audio =
            decode(path, ENVELOPE_RATE, Excerpt::default()).map_err(|error| error.to_string())?;
        let envelope = envelope(&audio.samples, ENVELOPE_RATE);
        write_atomically(
            &cached,
            &serde_json::to_string(&envelope).map_err(|error| error.to_string())?,
        )?;
        Ok(envelope)
    }
}

fn cache_key(path: &Path) -> Result<u64, String> {
    let metadata =
        fs::metadata(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let modified = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .unwrap_or_default();
    let mut digest = Fnv1a::new();
    digest.write(path.as_os_str().as_encoded_bytes());
    digest.write(&[0]);
    digest.write(&metadata.len().to_le_bytes());
    digest.write(&modified.as_secs().to_le_bytes());
    digest.write(&modified.subsec_nanos().to_le_bytes());
    Ok(digest.finish())
}

fn write_atomically(path: &Path, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    }
    let partial = path.with_extension(format!("{}.partial", std::process::id()));
    fs::write(&partial, text)
        .and_then(|()| fs::rename(&partial, path))
        .map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// The minimum and maximum of each bucket of `samples`.
fn envelope(samples: &[f32], rate: u32) -> Envelope {
    let bucket = (BUCKET_SECONDS * f64::from(rate)).round() as usize;
    let level = |sample: f32| (sample.clamp(-1.0, 1.0) * 127.0).round() as i8;
    let (mut min, mut max, mut rms) = (Vec::new(), Vec::new(), Vec::new());
    for chunk in samples.chunks(bucket.max(1)) {
        let (low, high) = chunk
            .iter()
            .fold((f32::MAX, f32::MIN), |(low, high), &sample| {
                (low.min(sample), high.max(sample))
            });
        let power = chunk.iter().map(|&sample| sample * sample).sum::<f32>() / chunk.len() as f32;
        min.push(level(low));
        max.push(level(high));
        rms.push(level(power.sqrt()));
    }
    Envelope {
        bucket_seconds: BUCKET_SECONDS,
        duration_seconds: samples.len() as f64 / f64::from(rate),
        min,
        max,
        rms,
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A generated sine (mp3, mono, 44.1 kHz), or `None` without FFmpeg.
    pub(crate) fn tone(path: &Path, seconds: f64, frequency: u32) -> Option<()> {
        let status = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg(format!(
                "sine=frequency={frequency}:sample_rate=44100:duration={seconds}"
            ))
            .args(["-c:a", "libmp3lame", "-b:a", "128k"])
            .arg(path)
            .status()
            .ok()?;
        status.success().then_some(())
    }

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-audition-audio-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_envelope_holds_each_buckets_extremes() {
        let samples: Vec<f32> = (0..400)
            .map(|index| if index == 250 { -1.0 } else { 0.5 })
            .collect();

        let envelope = envelope(&samples, 4_000);

        assert_eq!(envelope.min, [64, -127]);
        assert_eq!(envelope.max, [64, 64]);
        // sqrt((199 x 0.25 + 1) / 200) = 0.5012
        assert_eq!(envelope.rms, [64, 64]);
        assert_eq!(envelope.duration_seconds, 0.1);
    }

    #[test]
    fn a_wav_header_describes_its_samples() {
        let wav = wav(&[0; 8]);

        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(wav[4..8].try_into().unwrap()), 44);
        assert_eq!(u16::from_le_bytes(wav[22..24].try_into().unwrap()), 2);
        assert_eq!(u32::from_le_bytes(wav[24..28].try_into().unwrap()), 44_100);
        assert_eq!(u32::from_le_bytes(wav[40..44].try_into().unwrap()), 8);
        assert_eq!(wav.len(), 52);
    }

    #[test]
    fn windows_join_and_envelopes_are_cached_by_file() {
        let dir = scratch("windows");
        let file = dir.join("tone.mp3");
        if tone(&file, 33.0, 440).is_none() {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }

        let first = window_wav(&file, 0).unwrap().unwrap();
        let second = window_wav(&file, 1).unwrap().unwrap();

        let frames = |wav: &[u8]| (wav.len() - 44) / BYTES_PER_FRAME;
        assert_eq!(frames(&first), 30 * 44_100);
        // The mp3 holds about 3 s more (and the encoder's padding).
        assert!(
            (3 * 44_100..4 * 44_100).contains(&frames(&second)),
            "{}",
            frames(&second)
        );
        assert_eq!(window_wav(&file, 5), Ok(None));
        assert!(window_wav(&dir.join("gone.mp3"), 0).is_err());

        let envelopes = Envelopes {
            dir: dir.join("envelopes"),
        };
        let envelope = envelopes.get(&file).unwrap();
        assert!(
            (envelope.duration_seconds - 33.0).abs() < 0.1,
            "{}",
            envelope.duration_seconds
        );
        assert_eq!(fs::read_dir(&envelopes.dir).unwrap().count(), 1);
        assert_eq!(envelopes.get(&file).unwrap(), envelope);
    }
}

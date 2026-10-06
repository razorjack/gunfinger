//! Rendering test audio with FFmpeg: excerpts played at another speed, the
//! way a turntable plays them (pitch and tempo together), or key-locked
//! (tempo only), optionally filtered, and encoded.

use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/// Sample rate of rendered audio between FFmpeg and the encoder.
pub const RENDER_RATE: u32 = 44_100;

/// How a rendered excerpt is encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// 16-bit WAV: isolates the effect of the speed change.
    Lossless,
    /// MP3 at this many kbit/s.
    Mp3(u32),
    Aac(u32),
    Opus(u32),
}

impl Encoding {
    pub fn extension(self) -> &'static str {
        match self {
            Encoding::Lossless => "wav",
            Encoding::Mp3(_) => "mp3",
            Encoding::Aac(_) => "m4a",
            Encoding::Opus(_) => "opus",
        }
    }

    fn codec_args(self) -> Vec<String> {
        let (codec, kbps) = match self {
            Encoding::Lossless => return vec!["-c:a".into(), "pcm_s16le".into()],
            Encoding::Mp3(kbps) => ("libmp3lame", kbps),
            Encoding::Aac(kbps) => ("aac", kbps),
            Encoding::Opus(kbps) => ("libopus", kbps),
        };
        vec![
            "-c:a".into(),
            codec.into(),
            "-b:a".into(),
            format!("{kbps}k"),
        ]
    }
}

/// How the source is played back.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Playback {
    /// A turntable at this speed ratio: pitch and tempo together.
    Turntable(f64),
    /// Key lock at this tempo ratio: tempo changes, pitch does not.
    KeyLocked(f64),
}

impl Playback {
    /// Resample to a known rate, then either relabel it as faster or slower
    /// and resample back (the sample stream is unchanged, so pitch and tempo
    /// move together) or stretch time only.
    fn filter(self) -> String {
        match self {
            Playback::Turntable(speed) => format!(
                "aresample={RENDER_RATE},asetrate={:.0},aresample={RENDER_RATE}",
                f64::from(RENDER_RATE) * speed
            ),
            Playback::KeyLocked(tempo) => format!("aresample={RENDER_RATE},atempo={tempo:.4}"),
        }
    }
}

/// Renders `seconds` of output starting at `start` seconds into `source`,
/// played at `speed` times its native speed. Output time `t` therefore holds
/// source time `start + speed * t`.
pub fn render_excerpt(
    source: &Path,
    start: f64,
    seconds: f64,
    speed: f64,
    encoding: Encoding,
    output: &Path,
) -> Result<(), String> {
    let status = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-y", "-ss"])
        .arg(format!("{start:.3}"))
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac", "2", "-af"])
        .arg(Playback::Turntable(speed).filter())
        .arg("-t")
        .arg(format!("{seconds:.3}"))
        .args(encoding.codec_args())
        .arg(output)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "ffmpeg could not render {} ({status})",
            output.display()
        ))
    }
}

/// Like `render_excerpt`, but returns mono samples at `RENDER_RATE` for
/// further processing, with `filter` (an FFmpeg filter chain) applied after
/// the playback.
pub fn render_samples(
    source: &Path,
    start: f64,
    seconds: f64,
    playback: Playback,
    filter: Option<&str>,
) -> Result<Vec<f32>, String> {
    let mut chain = playback.filter();
    if let Some(filter) = filter {
        chain.push(',');
        chain.push_str(filter);
    }
    let output = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-ss"])
        .arg(format!("{start:.3}"))
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac", "1", "-af", &chain, "-t"])
        .arg(format!("{seconds:.3}"))
        .args(["-f", "f32le", "pipe:1"])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "ffmpeg could not render {}: {}",
            source.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(output
        .stdout
        .chunks_exact(4)
        .map(|bytes| f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        .collect())
}

/// Encodes mono samples at `RENDER_RATE` into `output`.
pub fn encode(samples: &[f32], encoding: Encoding, output: &Path) -> Result<(), String> {
    let wav = output.with_extension("render.wav");
    write_wav(&wav, samples)?;
    let status = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-y", "-i"])
        .arg(&wav)
        .args(encoding.codec_args())
        .arg(output)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    // The temporary file only matters while FFmpeg reads it.
    let _ = fs::remove_file(&wav);
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "ffmpeg could not encode {} ({status})",
            output.display()
        ))
    }
}

/// 16-bit mono PCM WAV at `RENDER_RATE`.
fn write_wav(path: &Path, samples: &[f32]) -> Result<(), String> {
    let data_bytes = u32::try_from(samples.len() * 2).map_err(|error| error.to_string())?;
    let mut bytes = Vec::with_capacity(44 + samples.len() * 2);
    bytes.extend(b"RIFF");
    bytes.extend((36 + data_bytes).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes()); // PCM
    bytes.extend(1_u16.to_le_bytes()); // mono
    bytes.extend(RENDER_RATE.to_le_bytes());
    bytes.extend((RENDER_RATE * 2).to_le_bytes());
    bytes.extend(2_u16.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(data_bytes.to_le_bytes());
    for &sample in samples {
        let value = (sample.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16;
        bytes.extend(value.to_le_bytes());
    }
    fs::write(path, bytes).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

pub fn rms(samples: &[f32]) -> f64 {
    let sum: f64 = samples.iter().map(|&x| f64::from(x) * f64::from(x)).sum();
    (sum / samples.len().max(1) as f64).sqrt()
}

/// Keeps the 16-bit render from clipping where no clipping is intended.
pub fn limited(samples: Vec<f32>) -> Vec<f32> {
    let peak = samples.iter().fold(0.0_f32, |peak, &x| peak.max(x.abs()));
    if peak <= 0.99 {
        return samples;
    }
    samples.iter().map(|&x| x * 0.99 / peak).collect()
}

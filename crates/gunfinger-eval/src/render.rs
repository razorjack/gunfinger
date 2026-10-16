//! Rendering test audio with FFmpeg: excerpts played at another speed, the
//! way a turntable plays them (pitch and tempo together), or key-locked
//! (tempo only), optionally filtered, and encoded.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};

use gunfinger_core::decode::Container;

/// A cache entry is reusable only after its encoder completed and the
/// final file was installed. Older caches without a receipt render once.
pub fn is_complete(path: &Path) -> bool {
    fs::metadata(path).is_ok_and(|metadata| {
        metadata.len() > 0
            && fs::read_to_string(receipt(path))
                .is_ok_and(|saved| saved == metadata.len().to_string())
    })
}

fn receipt(path: &Path) -> PathBuf {
    path.with_extension(format!(
        "{}.complete",
        path.extension().unwrap_or_default().to_string_lossy()
    ))
}

struct RenderFile {
    temporary: PathBuf,
}

impl RenderFile {
    fn new(output: &Path) -> RenderFile {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let extension = output.extension().unwrap_or_default().to_string_lossy();
        RenderFile {
            temporary: output.with_extension(format!(
                "{}.{}.partial.{extension}",
                std::process::id(),
                sequence
            )),
        }
    }

    fn finish(&self, output: &Path) -> Result<(), String> {
        let install = || -> std::io::Result<()> {
            let size = fs::metadata(&self.temporary)?.len();
            if size == 0 {
                return Err(std::io::Error::other("encoder produced an empty file"));
            }
            fs::rename(&self.temporary, output)?;
            fs::write(receipt(output), size.to_string())
        };
        install().map_err(|error| format!("cannot finish {}: {error}", output.display()))
    }
}

impl Drop for RenderFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.temporary);
    }
}

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
    let rendered = RenderFile::new(output);
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
        .arg(&rendered.temporary)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    if status.success() {
        rendered.finish(output)
    } else {
        Err(format!(
            "ffmpeg could not render {} ({status})",
            output.display()
        ))
    }
}

/// Like `render_excerpt`, but returns mono samples at `RENDER_RATE` for
/// further processing, with `filter` (an FFmpeg filter chain) applied after
/// the playback. FFmpeg seeks to `start` by the source's timestamps.
pub fn render_samples(
    source: &Path,
    start: f64,
    seconds: f64,
    playback: Playback,
    filter: Option<&str>,
) -> Result<Vec<f32>, String> {
    render_mono(source, Some(start), &chain(playback, filter), seconds)
}

/// Like `render_samples`, but `start` counts the samples decoded from the
/// source's beginning, as indexing decodes it: the timeline of the file's
/// peak record, on which a detection's line lies. A seek goes by
/// timestamps instead, and where damaged frames are dropped the two part
/// (experiment 0127: Do Drop In's decoded audio runs 235 ms ahead of its
/// timestamps). Decodes everything before `start`.
pub fn render_samples_as_indexed(
    source: &Path,
    start: f64,
    seconds: f64,
    playback: Playback,
    filter: Option<&str>,
) -> Result<Vec<f32>, String> {
    let from_start = format!(
        "asetpts=N/SR/TB,atrim=start={start:.6},asetpts=PTS-STARTPTS,{}",
        chain(playback, filter)
    );
    render_mono(source, None, &from_start, seconds)
}

fn chain(playback: Playback, filter: Option<&str>) -> String {
    let mut chain = playback.filter();
    if let Some(filter) = filter {
        chain.push(',');
        chain.push_str(filter);
    }
    chain
}

/// `seconds` of `source` through the filter `chain`, mono, after FFmpeg's
/// seek to `seek` when there is one.
fn render_mono(
    source: &Path,
    seek: Option<f64>,
    chain: &str,
    seconds: f64,
) -> Result<Vec<f32>, String> {
    let mut command = Command::new("ffmpeg");
    command.args(["-nostdin", "-v", "error"]);
    if let Some(start) = seek {
        command.arg("-ss").arg(format!("{start:.3}"));
    }
    let output = command
        // A WAV file behind an ID3 tag opens as the core's decoder opens it.
        .args(Container::of(source).input_args())
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac", "1", "-af", chain, "-t"])
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
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&bytes| f32::from_le_bytes(bytes))
        .collect())
}

/// Encodes mono samples at `RENDER_RATE` into `output`, through `filter`
/// (an FFmpeg filter chain) when one is given.
pub fn encode(
    samples: &[f32],
    filter: Option<&str>,
    encoding: Encoding,
    output: &Path,
) -> Result<(), String> {
    let rendered = RenderFile::new(output);
    let input = RenderFile::new(&output.with_extension("wav"));
    write_wav(&input.temporary, samples)?;
    let mut command = Command::new("ffmpeg");
    command
        .args(["-nostdin", "-v", "error", "-y", "-i"])
        .arg(&input.temporary);
    if let Some(filter) = filter {
        command.args(["-af", filter]);
    }
    let status = command
        .args(encoding.codec_args())
        .arg(&rendered.temporary)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    if status.success() {
        rendered.finish(output)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_wav_file_behind_an_id3_tag_renders() {
        let dir = std::env::temp_dir().join(format!("gunfinger-render-id3-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let wav = dir.join("tone.wav");
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("sine=frequency=440:sample_rate=8000:duration=3")
            .arg(&wav)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        // An ID3v2.3 header announcing 10 bytes of padding, then the WAV.
        let mut tagged = b"ID3\x03\x00\x00\x00\x00\x00\x0a".to_vec();
        tagged.extend([0; 10]);
        tagged.extend(fs::read(&wav).unwrap());
        let file = dir.join("tagged.mp3");
        fs::write(&file, tagged).unwrap();

        let samples = render_samples(
            &file,
            1.0,
            1.0,
            Playback::Turntable(1.0),
            Some("aresample=8000"),
        )
        .unwrap();

        assert!(
            (7_900..=8_100).contains(&samples.len()),
            "{}",
            samples.len()
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_render_as_indexed_follows_the_decoded_samples_where_timestamps_jump() {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-render-indexed-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        // Each sample holds its own time in seconds; the timestamps jump
        // 0.5 s at 2 s, as where a decoder drops damaged frames.
        let file = dir.join("jump.mka");
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("aevalsrc=t:s=8000:d=6")
            .args(["-af", "asetpts='if(gte(T,2),PTS+0.5/TB,PTS)'"])
            .args(["-c:a", "pcm_f32le"])
            .arg(&file)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        let indexed = gunfinger_core::decode::decode(
            &file,
            8_000,
            gunfinger_core::decode::Excerpt::default(),
        )
        .unwrap();
        let as_indexed = render_samples_as_indexed(
            &file,
            4.0,
            0.5,
            Playback::Turntable(1.0),
            Some("aresample=8000"),
        )
        .unwrap();
        let sought = render_samples(
            &file,
            4.0,
            0.5,
            Playback::Turntable(1.0),
            Some("aresample=8000"),
        )
        .unwrap();

        // Indexing decodes the samples one after another, with no gap.
        assert_eq!(indexed.samples.len(), 48_000);
        assert!((indexed.samples[32_100] - 4.0125).abs() < 1e-4);
        assert!(
            (3_990..=4_010).contains(&as_indexed.len()),
            "{}",
            as_indexed.len()
        );
        assert!(
            (as_indexed[100] - 4.0125).abs() < 1e-3,
            "{}",
            as_indexed[100]
        );
        assert!((sought[100] - 3.5125).abs() < 1e-3, "{}", sought[100]);
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unfinished_and_truncated_outputs_are_not_cache_entries() {
        let dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/tests/render-completion");
        fs::create_dir_all(&dir).unwrap();
        let output = dir.join("query.mp3");
        let _ = fs::remove_file(receipt(&output));
        fs::write(&output, b"legacy partial").unwrap();
        assert!(!is_complete(&output));
        let temporary;
        {
            let render = RenderFile::new(&output);
            temporary = render.temporary.clone();
            fs::write(&temporary, b"interrupted").unwrap();
        }
        assert!(!temporary.exists());
        assert!(!is_complete(&output));
        let render = RenderFile::new(&output);
        fs::write(&render.temporary, b"complete render").unwrap();
        render.finish(&output).unwrap();
        assert!(is_complete(&output));
        fs::write(&output, b"truncated").unwrap();
        assert!(!is_complete(&output));
        fs::remove_dir_all(dir).unwrap();
    }
}

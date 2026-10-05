//! Rendering test audio with FFmpeg: excerpts played at another speed, the
//! way a turntable plays them (pitch and tempo together).

use std::path::Path;
use std::process::{Command, Stdio};

/// How a rendered excerpt is encoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    /// 16-bit WAV: isolates the effect of the speed change.
    Lossless,
    /// 128 kbit/s MP3: a speed change followed by lossy coding.
    Mp3,
}

impl Encoding {
    pub fn extension(self) -> &'static str {
        match self {
            Encoding::Lossless => "wav",
            Encoding::Mp3 => "mp3",
        }
    }

    fn codec_args(self) -> &'static [&'static str] {
        match self {
            Encoding::Lossless => &["-c:a", "pcm_s16le"],
            Encoding::Mp3 => &["-c:a", "libmp3lame", "-b:a", "128k"],
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
    // Resample to a known rate, relabel it as faster or slower, then resample
    // back: the sample stream is unchanged, so pitch and tempo move together.
    let filter = format!(
        "aresample=44100,asetrate={:.0},aresample=44100",
        44100.0 * speed
    );
    let status = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-y", "-ss"])
        .arg(format!("{start:.3}"))
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac", "2", "-af", &filter, "-t"])
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

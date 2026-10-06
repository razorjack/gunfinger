//! Audio decoding through the `ffmpeg` executable.
//!
//! FFmpeg reads every container and codec the collection uses and resamples to
//! the analysis rate. Gunfinger runs it as a child process (never through a
//! shell) and streams raw 32-bit float samples from its stdout.

use std::ffi::OsString;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, Command, ExitStatus, Stdio};
use std::thread;
use std::time::Duration;

/// The part of a file to decode. The default is the whole file.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Excerpt {
    pub start: Option<Duration>,
    pub duration: Option<Duration>,
}

/// Mono samples at a known rate.
#[derive(Debug, Clone, PartialEq)]
pub struct Audio {
    pub sample_rate: u32,
    pub samples: Vec<f32>,
}

impl Audio {
    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.samples.len() as f64 / f64::from(self.sample_rate))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    #[error("`ffmpeg` was not found on PATH; install FFmpeg (for example `brew install ffmpeg`)")]
    FfmpegMissing,
    #[error("could not run `ffmpeg` for {path}: {source}")]
    Spawn { path: PathBuf, source: io::Error },
    #[error("could not read decoded audio of {path} from `ffmpeg`: {source}")]
    Pipe { path: PathBuf, source: io::Error },
    #[error("`ffmpeg` failed on {path} ({status}): {diagnostics}")]
    Failed {
        path: PathBuf,
        status: ExitStatus,
        diagnostics: String,
    },
    #[error(
        "{path} decoded only {decoded:.1} s of {expected:.1} s; the file is probably damaged: {diagnostics}"
    )]
    Truncated {
        path: PathBuf,
        decoded: f64,
        expected: f64,
        diagnostics: String,
    },
    #[error("{path} contains no decodable audio")]
    NoAudio { path: PathBuf },
}

impl DecodeError {
    /// Whether the file's content is at fault, rather than FFmpeg or the
    /// system: only then is the failure worth remembering.
    pub fn is_about_the_file(&self) -> bool {
        match self {
            DecodeError::Failed { .. }
            | DecodeError::Truncated { .. }
            | DecodeError::NoAudio { .. } => true,
            DecodeError::FfmpegMissing | DecodeError::Spawn { .. } | DecodeError::Pipe { .. } => {
                false
            }
        }
    }
}

/// Decodes the first audio stream of `path` to mono at `sample_rate`.
///
/// Embedded cover art, subtitles and data streams are ignored. A failure is
/// reported for the whole file: no partial audio is returned.
pub fn decode(path: &Path, sample_rate: u32, excerpt: Excerpt) -> Result<Audio, DecodeError> {
    let mut child = spawn_ffmpeg(path, sample_rate, excerpt)?;
    let (samples, diagnostics) = read_output(&mut child);
    let status = child.wait();
    let samples = samples.map_err(|source| DecodeError::Pipe {
        path: path.to_owned(),
        source,
    })?;
    let status = status.map_err(|source| DecodeError::Pipe {
        path: path.to_owned(),
        source,
    })?;

    if !status.success() {
        return Err(DecodeError::Failed {
            path: path.to_owned(),
            status,
            diagnostics: summarise(&diagnostics),
        });
    }
    if samples.is_empty() {
        return Err(DecodeError::NoAudio {
            path: path.to_owned(),
        });
    }
    let audio = Audio {
        sample_rate,
        samples,
    };
    if !diagnostics.trim().is_empty() {
        reject_if_truncated(path, &audio, excerpt, &diagnostics)?;
    }
    Ok(audio)
}

fn spawn_ffmpeg(path: &Path, sample_rate: u32, excerpt: Excerpt) -> Result<Child, DecodeError> {
    let mut command = Command::new("ffmpeg");
    command.args(["-nostdin", "-v", "error", "-threads", "1"]);
    if let Some(start) = excerpt.start {
        command.arg("-ss").arg(seconds_arg(start));
    }
    if let Some(duration) = excerpt.duration {
        command.arg("-t").arg(seconds_arg(duration));
    }
    command
        .arg("-i")
        .arg(path)
        .args(["-map", "0:a:0", "-vn", "-sn", "-dn", "-ac", "1"])
        .arg("-ar")
        .arg(sample_rate.to_string())
        .args(["-f", "f32le", "pipe:1"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    command.spawn().map_err(|source| match source.kind() {
        io::ErrorKind::NotFound => DecodeError::FfmpegMissing,
        _ => DecodeError::Spawn {
            path: path.to_owned(),
            source,
        },
    })
}

fn seconds_arg(duration: Duration) -> OsString {
    format!("{:.6}", duration.as_secs_f64()).into()
}

/// Reads samples from stdout while a second thread drains stderr, so that
/// neither pipe can fill up and stall FFmpeg. On a read error the child is
/// killed; the caller always reaps it.
fn read_output(child: &mut Child) -> (io::Result<Vec<f32>>, String) {
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    thread::scope(|scope| {
        let drainer = scope.spawn(move || stderr.map(drain).unwrap_or_default());
        let samples = match stdout {
            Some(stdout) => read_samples(stdout),
            None => Err(io::Error::other("ffmpeg stdout was not captured")),
        };
        if samples.is_err() {
            // Ignoring the result: the child may already have exited.
            let _ = child.kill();
        }
        let diagnostics = drainer.join().unwrap_or_default();
        (samples, diagnostics)
    })
}

fn drain(mut stderr: ChildStderr) -> String {
    let mut bytes = Vec::new();
    // A read error only loses diagnostics; the exit status still decides.
    let _ = stderr.read_to_end(&mut bytes);
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Converts a little-endian `f32` byte stream into samples, carrying an
/// incomplete trailing sample over to the next read.
pub(crate) fn read_samples(mut source: impl Read) -> io::Result<Vec<f32>> {
    const SAMPLE_BYTES: usize = 4;
    let mut samples = Vec::new();
    let mut buffer = vec![0_u8; 64 * 1024];
    let mut carried = 0;
    loop {
        let read = match source.read(&mut buffer[carried..]) {
            Ok(0) => break,
            Ok(read) => read,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) => return Err(error),
        };
        let available = carried + read;
        let whole = available - available % SAMPLE_BYTES;
        for bytes in buffer[..whole].chunks_exact(SAMPLE_BYTES) {
            samples.push(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]));
        }
        buffer.copy_within(whole..available, 0);
        carried = available - whole;
    }
    Ok(samples)
}

/// FFmpeg reports recoverable glitches (a damaged frame in an otherwise good
/// MP3) on stderr and still exits successfully. Such a file is accepted unless
/// the audio stops well short of the length the container declares.
fn reject_if_truncated(
    path: &Path,
    audio: &Audio,
    excerpt: Excerpt,
    diagnostics: &str,
) -> Result<(), DecodeError> {
    const TOLERANCE_SECONDS: f64 = 1.0;
    let Some(container) = probe_duration(path) else {
        return Ok(());
    };
    let start = excerpt.start.map_or(0.0, |start| start.as_secs_f64());
    let mut expected = (container - start).max(0.0);
    if let Some(duration) = excerpt.duration {
        expected = expected.min(duration.as_secs_f64());
    }
    let decoded = audio.duration().as_secs_f64();
    if decoded + TOLERANCE_SECONDS < expected {
        return Err(DecodeError::Truncated {
            path: path.to_owned(),
            decoded,
            expected,
            diagnostics: summarise(diagnostics),
        });
    }
    Ok(())
}

/// The container's declared duration in seconds, if `ffprobe` can tell.
fn probe_duration(path: &Path) -> Option<f64> {
    let output = Command::new("ffprobe")
        .args(["-v", "error", "-show_entries", "format=duration"])
        .args(["-of", "default=noprint_wrappers=1:nokey=1"])
        .arg(path)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

/// The last few lines of FFmpeg's stderr: enough to see what went wrong
/// without flooding the terminal when every frame of a file is damaged.
fn summarise(diagnostics: &str) -> String {
    const LINES: usize = 3;
    let lines: Vec<&str> = diagnostics
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    let tail = &lines[lines.len().saturating_sub(LINES)..];
    if tail.is_empty() {
        "no diagnostics".to_owned()
    } else {
        tail.join(" | ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bytes_of(samples: &[f32]) -> Vec<u8> {
        samples
            .iter()
            .flat_map(|sample| sample.to_le_bytes())
            .collect()
    }

    /// Delivers its data in reads of a fixed, awkward size so that samples
    /// straddle read boundaries.
    struct Trickle {
        data: Vec<u8>,
        position: usize,
        step: usize,
    }

    impl Read for Trickle {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let end = (self.position + self.step)
                .min(self.data.len())
                .min(self.position + buffer.len());
            let count = end - self.position;
            buffer[..count].copy_from_slice(&self.data[self.position..end]);
            self.position = end;
            Ok(count)
        }
    }

    #[test]
    fn samples_survive_reads_that_split_them() {
        let samples = [0.5, -1.0, 0.25, 3.0e-7, -0.125];
        let source = Trickle {
            data: bytes_of(&samples),
            position: 0,
            step: 3,
        };

        assert_eq!(read_samples(source).unwrap(), samples);
    }

    #[test]
    fn a_trailing_partial_sample_is_dropped() {
        let mut data = bytes_of(&[1.0, 2.0]);
        data.extend([0, 0]);

        assert_eq!(read_samples(data.as_slice()).unwrap(), [1.0, 2.0]);
    }

    #[test]
    fn diagnostics_are_summarised_to_the_last_lines() {
        let diagnostics = "one\n\ntwo\nthree\nfour\n";

        assert_eq!(summarise(diagnostics), "two | three | four");
        assert_eq!(summarise(""), "no diagnostics");
    }
}

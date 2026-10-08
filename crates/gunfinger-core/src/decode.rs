//! Audio decoding through the `ffmpeg` executable.
//!
//! FFmpeg reads every container and codec the collection uses and resamples to
//! the analysis rate. Gunfinger runs it as a child process (never through a
//! shell) and streams raw 32-bit float samples from its stdout.

use std::ffi::OsString;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, Command, ExitStatus, Stdio};
use std::thread;
use std::time::Duration;

use crate::tags::Tags;

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
        .args(Container::of(path).input_args())
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

/// How FFmpeg must open a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Container {
    /// FFmpeg recognises the file as it is: every file but the one below.
    AsIs,
    /// An MP3 stream in a WAV (RIFF) container, with an ID3v2 tag written
    /// in front of the container. FFmpeg's format probe looks past the tag,
    /// recognises the RIFF header and picks the WAV reader, which then
    /// starts at byte 0 and rejects the tag ("Invalid data found when
    /// processing input"). Skipping the tag opens the container.
    RiffBehindId3 { tag_bytes: u64 },
}

impl Container {
    /// Reads the first bytes of `path`. A file that cannot be read opens
    /// as it is, so FFmpeg reports the problem.
    pub fn of(path: &Path) -> Container {
        File::open(path)
            .ok()
            .and_then(|mut file| riff_behind_id3(&mut file))
            .map_or(Container::AsIs, |tag_bytes| Container::RiffBehindId3 {
                tag_bytes,
            })
    }

    /// FFmpeg input options that go before `-i`.
    pub fn input_args(self) -> Vec<String> {
        match self {
            Container::AsIs => Vec::new(),
            Container::RiffBehindId3 { tag_bytes } => {
                vec![String::from("-skip_initial_bytes"), tag_bytes.to_string()]
            }
        }
    }
}

/// The length of an ID3v2 tag at the start of `file` when "RIFF" follows
/// it. The tag's 10-byte header ends with its size as a 28-bit syncsafe
/// integer (7 bits per byte), which leaves out the header itself and the
/// 10-byte footer that flag bit 4 announces (ID3v2.4).
fn riff_behind_id3(file: &mut (impl Read + Seek)) -> Option<u64> {
    const FOOTER_PRESENT: u8 = 0x10;
    let mut header = [0_u8; 10];
    file.read_exact(&mut header).ok()?;
    if &header[..3] != b"ID3" {
        return None;
    }
    let size = header[6..]
        .iter()
        .fold(0_u64, |size, byte| (size << 7) | u64::from(byte & 0x7F));
    let footer = if header[5] & FOOTER_PRESENT == 0 {
        0
    } else {
        10
    };
    let tag_bytes = 10 + size + footer;
    let mut next = [0_u8; 4];
    file.seek(SeekFrom::Start(tag_bytes)).ok()?;
    file.read_exact(&mut next).ok()?;
    (&next == b"RIFF").then_some(tag_bytes)
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
    let Some(container) = probe(path).and_then(|probe| probe.length) else {
        return Ok(());
    };
    let start = excerpt.start.map_or(0.0, |start| start.as_secs_f64());
    let mut expected = (container.as_secs_f64() - start).max(0.0);
    if let Some(duration) = excerpt.duration {
        expected = expected.min(duration.as_secs_f64());
    }
    let decoded = audio.duration().as_secs_f64();
    if decoded + truncation_tolerance(expected) < expected {
        return Err(DecodeError::Truncated {
            path: path.to_owned(),
            decoded,
            expected,
            diagnostics: summarise(diagnostics),
        });
    }
    Ok(())
}

/// How far decoded audio may fall short of the expected length: the larger
/// of 1 s and 1%. Rips damaged only in their last frames lose 1.3-2.4 s of
/// 5-8 minutes (0.3-0.5%) and are worth keeping; a file cut off by a
/// failed copy or download loses far more.
fn truncation_tolerance(expected_seconds: f64) -> f64 {
    const MINIMUM_SECONDS: f64 = 1.0;
    const SHARE: f64 = 0.01;
    (expected_seconds * SHARE).max(MINIMUM_SECONDS)
}

/// What `ffprobe` reads from a file's header, without decoding the audio.
#[derive(Debug, Default, PartialEq)]
pub struct Probe {
    /// The length the container declares. For an MP3 without a length
    /// header FFmpeg estimates it from the bitrate: within 0.3% on 60
    /// constant-bitrate library files, but a variable bitrate can throw it
    /// off.
    pub length: Option<Duration>,
    /// Container tags first, then those of the streams (Ogg, Opus).
    pub tags: Tags,
}

/// Reads the declared length and the tags of `path`, at a fraction of the
/// cost of decoding. `None` when `ffprobe` cannot run or cannot read the
/// file.
pub fn probe(path: &Path) -> Option<Probe> {
    match Container::of(path) {
        Container::AsIs => run_ffprobe(path, &[]),
        container @ Container::RiffBehindId3 { .. } => {
            let mut probe = run_ffprobe(path, &container.input_args())?;
            // The tags are in the ID3 tag the WAV reader skips; FFmpeg's
            // MP3 reader parses it.
            if let Some(tagged) = run_ffprobe(path, &["-f".into(), "mp3".into()]) {
                probe.tags = tagged.tags.or(probe.tags);
            }
            Some(probe)
        }
    }
}

fn run_ffprobe(path: &Path, input_args: &[String]) -> Option<Probe> {
    let output = Command::new("ffprobe")
        .args(["-v", "error", "-of", "flat", "-show_entries"])
        .arg("format=duration:format_tags:stream_tags")
        .args(input_args)
        .arg(path)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| parse_probe(&String::from_utf8_lossy(&output.stdout)))
}

/// Parses `ffprobe -of flat`: one `key="value"` per line, such as
/// `format.tags.title="China Cup"` or `streams.stream.0.tags.TITLE="..."`,
/// with newlines and quotes inside values escaped.
fn parse_probe(output: &str) -> Probe {
    let mut length = None;
    let mut format_tags = Tags::default();
    let mut stream_tags = Tags::default();
    for line in output.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = unescape(value);
        if key == "format.duration" {
            length = value
                .parse()
                .ok()
                .and_then(|seconds| Duration::try_from_secs_f64(seconds).ok());
        } else if let Some(name) = key.strip_prefix("format.tags.") {
            format_tags.offer(name, &value);
        } else if let Some((_, name)) = key
            .strip_prefix("streams.stream.")
            .and_then(|rest| rest.split_once(".tags."))
        {
            stream_tags.offer(name, &value);
        }
    }
    Probe {
        length,
        tags: format_tags.or(stream_tags),
    }
}

/// A flat value without its quotes and backslash escapes.
fn unescape(quoted: &str) -> String {
    let inner = quoted
        .strip_prefix('"')
        .and_then(|rest| rest.strip_suffix('"'))
        .unwrap_or(quoted);
    let mut text = String::with_capacity(inner.len());
    let mut characters = inner.chars();
    while let Some(character) = characters.next() {
        if character != '\\' {
            text.push(character);
            continue;
        }
        match characters.next() {
            Some('n') => text.push('\n'),
            Some('t') => text.push('\t'),
            Some('r') => text.push('\r'),
            Some(escaped) => text.push(escaped),
            None => {}
        }
    }
    text
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
    fn a_probe_reads_the_length_and_prefers_container_tags() {
        let output = [
            r#"streams.stream.0.tags.TITLE="dominion""#,
            r#"streams.stream.0.tags.ARTIST="KRAKEN""#,
            r#"streams.stream.1.tags.comment="Cover (front)""#,
            r#"format.duration="489.440000""#,
            r#"format.tags.title="Dominion""#,
        ]
        .join("\n");

        let probe = parse_probe(&output);

        assert_eq!(probe.length, Some(Duration::from_secs_f64(489.44)));
        assert_eq!(
            probe.tags,
            Tags {
                artist: Some(String::from("KRAKEN")),
                title: Some(String::from("Dominion")),
                album: None,
            }
        );
    }

    #[test]
    fn flat_values_lose_their_quotes_and_escapes() {
        assert_eq!(
            unescape(r#""one\ntwo \"quoted\" \\ end""#),
            "one\ntwo \"quoted\" \\ end"
        );
        assert_eq!(unescape("bare"), "bare");
    }

    fn id3_header(version: u8, flags: u8, size: u32) -> Vec<u8> {
        let syncsafe = [
            (size >> 21) as u8 & 0x7F,
            (size >> 14) as u8 & 0x7F,
            (size >> 7) as u8 & 0x7F,
            size as u8 & 0x7F,
        ];
        let mut header = b"ID3".to_vec();
        header.extend([version, 0, flags]);
        header.extend(syncsafe);
        header
    }

    #[test]
    fn riff_right_after_an_id3_tag_is_found_past_the_tag() {
        let mut file = id3_header(3, 0, 300);
        file.extend([0; 300]);
        file.extend(b"RIFF\0\0\0\0WAVE");

        assert_eq!(riff_behind_id3(&mut io::Cursor::new(file)), Some(310));
    }

    #[test]
    fn a_flagged_id3_footer_counts_towards_the_tag() {
        let mut file = id3_header(4, 0x10, 200);
        file.extend([0; 200]);
        file.extend(b"3DI\x04\0\x10\0\0\x01\x48");
        file.extend(b"RIFF\0\0\0\0WAVE");

        assert_eq!(riff_behind_id3(&mut io::Cursor::new(file)), Some(220));
    }

    #[test]
    fn an_id3_tag_before_mp3_frames_or_no_tag_opens_as_it_is() {
        let mut tagged_mp3 = id3_header(3, 0, 20);
        tagged_mp3.extend([0; 20]);
        tagged_mp3.extend([0xFF, 0xFB, 0x90, 0x64]);
        let plain_wav = b"RIFF\0\0\0\0WAVEfmt ".to_vec();

        assert_eq!(riff_behind_id3(&mut io::Cursor::new(tagged_mp3)), None);
        assert_eq!(riff_behind_id3(&mut io::Cursor::new(plain_wav)), None);
        assert_eq!(riff_behind_id3(&mut io::Cursor::new(b"ID3".to_vec())), None);
    }

    #[test]
    fn the_truncation_tolerance_is_a_second_or_a_hundredth() {
        assert!((truncation_tolerance(60.0) - 1.0).abs() < 1e-9);
        assert!((truncation_tolerance(100.0) - 1.0).abs() < 1e-9);
        assert!((truncation_tolerance(490.9) - 4.909).abs() < 1e-9);
    }

    #[test]
    fn the_tolerance_keeps_damaged_last_frames_and_rejects_cut_files() {
        // Lengths decoded and declared for rips in the owner's collection.
        let damaged_at_the_end = [(488.5, 490.9), (398.3, 399.8), (291.0, 292.4)];
        let cut_off = [(311.3, 332.5), (393.2, 409.4)];

        for (decoded, expected) in damaged_at_the_end {
            assert!(decoded + truncation_tolerance(expected) >= expected);
        }
        for (decoded, expected) in cut_off {
            assert!(decoded + truncation_tolerance(expected) < expected);
        }
    }

    #[test]
    fn diagnostics_are_summarised_to_the_last_lines() {
        let diagnostics = "one\n\ntwo\nthree\nfour\n";

        assert_eq!(summarise(diagnostics), "two | three | four");
        assert_eq!(summarise(""), "no diagnostics");
    }
}

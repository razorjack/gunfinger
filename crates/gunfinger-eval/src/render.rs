//! Rendering test audio with FFmpeg: excerpts played at another speed, the
//! way a turntable plays them (pitch and tempo together), or key-locked
//! (tempo only), optionally filtered, and encoded.
//!
//! Every FFmpeg run here is watched by the core's stall timeout
//! (`decode::watch`, ADR 0018), and a cached render is reused only when its
//! receipt holds the recipe of the render asked for (brief 78).

use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use gunfinger_core::decode::watch::{self, Heartbeat, Run};
use gunfinger_core::decode::{Container, STALL, read_all};
use serde::{Deserialize, Serialize};

/// The version of the harness's rendering code. Bump it whenever a change
/// to how the harness renders (this module, or what `mixes`, `robust`,
/// `shared`, `recall`, `loss` or `sweep` do with the samples) changes the
/// audio that the same recipe gives, so that every receipt refuses its
/// render.
pub const RENDERER: &str = "gunfinger-eval renders 1";

/// What a render was made from and how: its source files as they were,
/// the steps that decide its audio, and the renderer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    sources: Vec<SourceFile>,
    steps: String,
    renderer: String,
}

/// A source file as a render read it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct SourceFile {
    path: PathBuf,
    bytes: u64,
    modified_nanos: u64,
}

impl Recipe {
    /// The recipe of a render of `sources`, as they are now. `steps` holds
    /// everything else the caller decides that changes the audio (excerpt,
    /// playback, filters, partners, faults, seeds, encoding), written with
    /// `{:?}` so that numbers round-trip.
    pub fn new(sources: &[&Path], steps: String) -> Result<Recipe, String> {
        Ok(Recipe {
            sources: sources
                .iter()
                .map(|source| SourceFile::now(source))
                .collect::<Result<_, _>>()?,
            steps,
            renderer: renderer()?,
        })
    }
}

impl SourceFile {
    fn now(path: &Path) -> Result<SourceFile, String> {
        let file = path.to_path_buf();
        let metadata = watch::within(STALL, move || fs::metadata(file))
            .map_err(|error| format!("cannot start a thread: {error}"))?
            .ok_or_else(|| stalled(&format!("read {}", path.display()), STALL))?
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let modified = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .unwrap_or_default();
        Ok(SourceFile {
            path: path.to_path_buf(),
            bytes: metadata.len(),
            modified_nanos: u64::try_from(modified.as_nanos()).unwrap_or(u64::MAX),
        })
    }
}

/// `RENDERER` and FFmpeg's version, asked once per run.
fn renderer() -> Result<String, String> {
    static RENDERER_NOW: OnceLock<Result<String, String>> = OnceLock::new();
    RENDERER_NOW
        .get_or_init(|| {
            let mut command = Command::new("ffmpeg");
            command.arg("-version");
            let printed = pipe_watched(&mut command, STALL, "print its version")?;
            let version = String::from_utf8_lossy(&printed)
                .lines()
                .next()
                .map(str::to_owned)
                .ok_or("ffmpeg -version printed nothing")?;
            Ok(format!("{RENDERER}; {version}"))
        })
        .clone()
}

/// A completed render's receipt.
#[derive(Deserialize)]
struct Receipt {
    bytes: u64,
    recipe: Recipe,
}

/// Renders `output` with `render`, unless a render completed from the same
/// `recipe` is there already. A receipt that is missing, unreadable, of
/// another recipe or of the older form (a size alone), or a file of
/// another size, means the render is made afresh.
pub fn once(
    output: &Path,
    recipe: &Recipe,
    render: impl FnOnce() -> Result<(), String>,
) -> Result<(), String> {
    if is_complete(output, recipe) {
        return Ok(());
    }
    let receipt = receipt(output);
    match fs::remove_file(&receipt) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => {
            return Err(format!("cannot remove {}: {error}", receipt.display()));
        }
        _ => {}
    }
    render()?;
    let bytes = fs::metadata(output)
        .map_err(|error| format!("cannot read {}: {error}", output.display()))?
        .len();
    let text = serde_json::json!({ "bytes": bytes, "recipe": recipe }).to_string();
    fs::write(&receipt, text)
        .map_err(|error| format!("cannot write {}: {error}", receipt.display()))
}

/// Whether `path` holds a whole render made from `recipe`.
fn is_complete(path: &Path, recipe: &Recipe) -> bool {
    let Ok(metadata) = fs::metadata(path) else {
        return false;
    };
    let saved = fs::read_to_string(receipt(path))
        .ok()
        .and_then(|text| serde_json::from_str::<Receipt>(&text).ok());
    metadata.len() > 0
        && saved.is_some_and(|saved| saved.bytes == metadata.len() && saved.recipe == *recipe)
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
            fs::rename(&self.temporary, output)
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
    let mut command = encoder();
    command
        .args(["-y", "-ss"])
        .arg(format!("{start:.3}"))
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac", "2", "-af"])
        .arg(Playback::Turntable(speed).filter())
        .arg("-t")
        .arg(format!("{seconds:.3}"))
        .args(encoding.codec_args())
        .arg(&rendered.temporary);
    encode_watched(&mut command, STALL, &format!("render {}", output.display()))?;
    rendered.finish(output)
}

/// `render_excerpt`, unless the same render is at `output` already
/// (`once`).
pub fn excerpt_once(
    source: &Path,
    start: f64,
    seconds: f64,
    speed: f64,
    encoding: Encoding,
    output: &Path,
) -> Result<(), String> {
    let recipe = excerpt_recipe(source, start, seconds, speed, encoding)?;
    once(output, &recipe, || {
        render_excerpt(source, start, seconds, speed, encoding, output)
    })
}

/// The recipe of `render_excerpt` with these arguments.
pub fn excerpt_recipe(
    source: &Path,
    start: f64,
    seconds: f64,
    speed: f64,
    encoding: Encoding,
) -> Result<Recipe, String> {
    let steps = format!(
        "render_excerpt from {start:?} s for {seconds:?} s at speed {speed:?}, {encoding:?}"
    );
    Recipe::new(&[source], steps)
}

/// FFmpeg asked to report its progress on stdout, for an encode to a file,
/// which otherwise writes nothing while it works.
fn encoder() -> Command {
    let mut command = Command::new("ffmpeg");
    command.args(["-nostdin", "-v", "error", "-nostats", "-progress", "pipe:1"]);
    command
}

/// Runs `command`, an `encoder()` writing a file, under the stall
/// timeout `stall`. FFmpeg repeats its last progress report while its
/// input stalls (brief 78, measured on FFmpeg 8.1.3), so only a report
/// whose position or size has advanced counts as a sign of life.
fn encode_watched(command: &mut Command, stall: Duration, doing: &str) -> Result<(), String> {
    match watch::run_reporting(command, stall, follow_progress) {
        Err(error) => Err(format!("cannot run ffmpeg: {error}")),
        Ok(Run::Stalled) => Err(stalled(doing, stall)),
        Ok(Run::Finished {
            status,
            diagnostics,
            ..
        }) => {
            if status.success() {
                Ok(())
            } else {
                Err(format!(
                    "ffmpeg could not {doing} ({status}): {}",
                    diagnostics.trim()
                ))
            }
        }
    }
}

/// The keys of FFmpeg's progress report that advance while it encodes.
const PROGRESS_KEYS: [&str; 2] = ["out_time_us", "total_size"];

/// Reads FFmpeg's `-progress` report, beating whenever one of
/// `PROGRESS_KEYS` differs from its last value.
fn follow_progress(heartbeat: Heartbeat<()>) -> io::Result<()> {
    let mut report = BufReader::new(heartbeat);
    let mut last: [Option<String>; 2] = [None, None];
    let mut line = String::new();
    while report.read_line(&mut line)? > 0 {
        if let Some((key, value)) = line.trim_end().split_once('=')
            && let Some(slot) = PROGRESS_KEYS.iter().position(|&known| known == key)
            && last[slot].as_deref() != Some(value)
        {
            last[slot] = Some(value.to_owned());
            report.get_ref().beat();
        }
        line.clear();
    }
    Ok(())
}

/// Runs `command`, FFmpeg writing samples to stdout, under the stall
/// timeout `stall`, and returns what it wrote: the samples are the
/// heartbeat, as in the core's decode.
pub(crate) fn pipe_watched(
    command: &mut Command,
    stall: Duration,
    doing: &str,
) -> Result<Vec<u8>, String> {
    match watch::run(command, stall, read_all) {
        Err(error) => Err(format!("cannot run ffmpeg: {error}")),
        Ok(Run::Stalled) => Err(stalled(doing, stall)),
        Ok(Run::Finished {
            output,
            status,
            diagnostics,
        }) => {
            if !status.success() {
                return Err(format!("ffmpeg could not {doing}: {}", diagnostics.trim()));
            }
            output.map_err(|error| format!("cannot read what ffmpeg wrote to {doing}: {error}"))
        }
    }
}

/// How FFmpeg must open `source`, from its first bytes, read under the
/// stall timeout.
pub(crate) fn container(source: &Path) -> Result<Container, String> {
    let file = source.to_path_buf();
    watch::within(STALL, move || Container::of(&file))
        .map_err(|error| format!("cannot start a thread: {error}"))?
        .ok_or_else(|| stalled(&format!("read {}", source.display()), STALL))
}

fn stalled(doing: &str, stall: Duration) -> String {
    format!(
        "could not {doing}: no progress for {} s, so it was stopped; the storage holding the source may be unresponsive, such as a NAS that hangs",
        stall.as_secs()
    )
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

/// Like `render_samples` without a filter, but with both channels kept:
/// a one-channel source gives two identical channels.
pub fn render_stereo_samples(
    source: &Path,
    start: f64,
    seconds: f64,
    playback: Playback,
) -> Result<Stereo, String> {
    render_pcm(source, Some(start), &playback.filter(), seconds, 2)
        .map(|samples| Stereo::split(&samples))
}

/// Two channels of equal length.
#[derive(Debug, Clone, PartialEq)]
pub struct Stereo {
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}

impl Stereo {
    /// From samples interleaved left, right; a trailing half frame is
    /// dropped.
    pub fn split(interleaved: &[f32]) -> Stereo {
        let (frames, _) = interleaved.as_chunks::<2>();
        Stereo {
            left: frames.iter().map(|frame| frame[0]).collect(),
            right: frames.iter().map(|frame| frame[1]).collect(),
        }
    }

    fn interleaved(&self) -> Vec<f32> {
        self.left
            .iter()
            .zip(&self.right)
            .flat_map(|(&left, &right)| [left, right])
            .collect()
    }

    /// `limited` over both channels: one gain for the two.
    pub fn limited(self) -> Stereo {
        let peak = self
            .left
            .iter()
            .chain(&self.right)
            .fold(0.0_f32, |peak, &x| peak.max(x.abs()));
        if peak <= 0.99 {
            return self;
        }
        let scale = |channel: Vec<f32>| channel.iter().map(|&x| x * 0.99 / peak).collect();
        Stereo {
            left: scale(self.left),
            right: scale(self.right),
        }
    }
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
    render_pcm(source, seek, chain, seconds, 1)
}

/// As `render_mono`, with this many channels, interleaved.
fn render_pcm(
    source: &Path,
    seek: Option<f64>,
    chain: &str,
    seconds: f64,
    channels: u8,
) -> Result<Vec<f32>, String> {
    let mut command = Command::new("ffmpeg");
    command.args(["-nostdin", "-v", "error"]);
    if let Some(start) = seek {
        command.arg("-ss").arg(format!("{start:.3}"));
    }
    command
        // A WAV file behind an ID3 tag opens as the core's decoder opens it.
        .args(container(source)?.input_args())
        .arg("-i")
        .arg(source)
        .args(["-map", "0:a:0", "-ac"])
        .arg(channels.to_string())
        .args(["-af", chain, "-t"])
        .arg(format!("{seconds:.3}"))
        .args(["-f", "f32le", "pipe:1"]);
    let output = pipe_watched(&mut command, STALL, &format!("render {}", source.display()))?;
    Ok(output
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
    encode_pcm(samples, 1, filter, encoding, output)
}

/// Encodes two channels at `RENDER_RATE` into `output`.
pub fn encode_stereo(stereo: &Stereo, encoding: Encoding, output: &Path) -> Result<(), String> {
    encode_pcm(&stereo.interleaved(), 2, None, encoding, output)
}

fn encode_pcm(
    samples: &[f32],
    channels: u16,
    filter: Option<&str>,
    encoding: Encoding,
    output: &Path,
) -> Result<(), String> {
    let rendered = RenderFile::new(output);
    let input = RenderFile::new(&output.with_extension("wav"));
    write_wav(&input.temporary, samples, channels)?;
    let mut command = encoder();
    command.args(["-y", "-i"]).arg(&input.temporary);
    if let Some(filter) = filter {
        command.args(["-af", filter]);
    }
    command.args(encoding.codec_args()).arg(&rendered.temporary);
    encode_watched(&mut command, STALL, &format!("encode {}", output.display()))?;
    rendered.finish(output)
}

/// 16-bit PCM WAV at `RENDER_RATE`, `samples` interleaved over
/// `channels`.
fn write_wav(path: &Path, samples: &[f32], channels: u16) -> Result<(), String> {
    let data_bytes = u32::try_from(samples.len() * 2).map_err(|error| error.to_string())?;
    let mut bytes = Vec::with_capacity(44 + samples.len() * 2);
    bytes.extend(b"RIFF");
    bytes.extend((36 + data_bytes).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes()); // PCM
    bytes.extend(channels.to_le_bytes());
    bytes.extend(RENDER_RATE.to_le_bytes());
    bytes.extend((RENDER_RATE * 2 * u32::from(channels)).to_le_bytes());
    bytes.extend((2 * channels).to_le_bytes());
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
    use std::cell::Cell;
    use std::time::Instant;

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

    fn scratch(test: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("gunfinger-render-{test}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// A render that writes `content` the way the encoders do, counting
    /// its calls.
    fn rendering<'a>(
        output: &'a Path,
        content: &'a [u8],
        calls: &'a Cell<u32>,
    ) -> impl FnOnce() -> Result<(), String> + 'a {
        move || {
            calls.set(calls.get() + 1);
            let render = RenderFile::new(output);
            fs::write(&render.temporary, content).unwrap();
            render.finish(output)
        }
    }

    fn recipe_of(source: &Path, steps: &str) -> Recipe {
        Recipe {
            sources: vec![SourceFile::now(source).unwrap()],
            steps: steps.to_owned(),
            renderer: String::from("renders 1; ffmpeg version 8.1.3"),
        }
    }

    #[test]
    fn a_changed_source_fault_or_renderer_refuses_the_cached_render() {
        let dir = scratch("recipe");
        let source = dir.join("track.mp3");
        fs::write(&source, b"the track").unwrap();
        let output = dir.join("query.mp3");
        let calls = Cell::new(0);
        let unfaulted = "excerpt from 30.0 s at speed 1.03, fault None";
        let first = recipe_of(&source, unfaulted);

        once(&output, &first, rendering(&output, b"render", &calls)).unwrap();
        once(&output, &first, rendering(&output, b"render", &calls)).unwrap();
        assert_eq!(calls.get(), 1);

        // Other audio at the same path.
        fs::write(&source, b"another track").unwrap();
        let replaced = recipe_of(&source, unfaulted);
        assert!(!is_complete(&output, &replaced));
        once(&output, &replaced, rendering(&output, b"render", &calls)).unwrap();
        assert_eq!(calls.get(), 2);

        // The same size, modified at another time.
        fs::write(&source, b"other track!!").unwrap();
        fs::File::options()
            .write(true)
            .open(&source)
            .unwrap()
            .set_modified(UNIX_EPOCH + Duration::from_secs(1_000_000_000))
            .unwrap();
        let retagged = recipe_of(&source, unfaulted);
        assert_eq!(retagged.sources[0].bytes, replaced.sources[0].bytes);
        assert!(!is_complete(&output, &retagged));
        once(&output, &retagged, rendering(&output, b"render", &calls)).unwrap();
        assert_eq!(calls.get(), 3);

        // Another fault.
        let faulted = recipe_of(
            &source,
            "excerpt from 30.0 s at speed 1.03, fault Some(Inverted)",
        );
        assert!(!is_complete(&output, &faulted));
        once(&output, &faulted, rendering(&output, b"render", &calls)).unwrap();
        assert_eq!(calls.get(), 4);

        // Another renderer.
        let newer = Recipe {
            renderer: String::from("renders 1; ffmpeg version 8.1.4"),
            ..faulted.clone()
        };
        assert!(!is_complete(&output, &newer));
        once(&output, &newer, rendering(&output, b"render", &calls)).unwrap();
        assert_eq!(calls.get(), 5);
        assert!(is_complete(&output, &newer));
        assert!(!is_complete(&output, &faulted));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_receipt_of_a_size_alone_is_refused_and_replaced() {
        let dir = scratch("old-receipt");
        let source = dir.join("track.mp3");
        fs::write(&source, b"the track").unwrap();
        let output = dir.join("query.mp3");
        fs::write(&output, b"old render").unwrap();
        fs::write(receipt(&output), "10").unwrap();
        let recipe = recipe_of(&source, "excerpt");
        let calls = Cell::new(0);

        assert!(!is_complete(&output, &recipe));
        once(&output, &recipe, rendering(&output, b"new render", &calls)).unwrap();

        assert_eq!(calls.get(), 1);
        assert_eq!(fs::read(&output).unwrap(), b"new render");
        assert!(is_complete(&output, &recipe));
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn unfinished_failed_and_truncated_renders_are_not_reused() {
        let dir = scratch("completion");
        let source = dir.join("track.mp3");
        fs::write(&source, b"the track").unwrap();
        let output = dir.join("query.mp3");
        let recipe = recipe_of(&source, "excerpt");
        let calls = Cell::new(0);
        let temporary;
        {
            let render = RenderFile::new(&output);
            temporary = render.temporary.clone();
            fs::write(&temporary, b"interrupted").unwrap();
        }
        assert!(!temporary.exists());
        assert!(!is_complete(&output, &recipe));

        once(
            &output,
            &recipe,
            rendering(&output, b"complete render", &calls),
        )
        .unwrap();
        assert!(is_complete(&output, &recipe));
        fs::write(&output, b"truncated").unwrap();
        assert!(!is_complete(&output, &recipe));
        let failed = once(&output, &recipe, || Err(String::from("ffmpeg failed")));

        assert!(failed.is_err());
        assert!(!receipt(&output).exists());
        assert!(!is_complete(&output, &recipe));
        fs::remove_dir_all(&dir).ok();
    }

    /// The stall timeout of the stand-ins below, as in `decode::watch`'s
    /// tests: long enough for a loaded machine to start a child.
    const STALL_IN_TESTS: Duration = Duration::from_secs(2);

    /// An executable shell script standing in for FFmpeg.
    fn stand_in(dir: &Path, script: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;
        let path = dir.join("ffmpeg");
        fs::write(&path, format!("#!/bin/sh\n{script}\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
        path
    }

    #[test]
    fn an_encode_that_writes_nothing_is_stopped_after_the_stall_timeout() {
        let dir = scratch("encode-hangs");
        let hangs = stand_in(&dir, "exec sleep 60");
        let started = Instant::now();

        let error = encode_watched(&mut Command::new(hangs), STALL_IN_TESTS, "render query.mp3")
            .unwrap_err();

        assert!(
            started.elapsed() >= STALL_IN_TESTS,
            "{:?}",
            started.elapsed()
        );
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "{:?}",
            started.elapsed()
        );
        assert!(
            error.starts_with("could not render query.mp3: no progress for 2 s"),
            "{error}"
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_encode_whose_progress_advances_finishes_without_writing_diagnostics() {
        let dir = scratch("encode-progresses");
        // A report every 0.1 s whose position advances, 40 in all: at least
        // 4 s, longer than the stall timeout, with nothing on stderr.
        let progresses = stand_in(
            &dir,
            "i=0; while [ $i -lt 40 ]; do echo \"out_time_us=$((i * 100000))\"; echo 'total_size=1024'; echo 'progress=continue'; sleep 0.1; i=$((i + 1)); done; echo 'progress=end'",
        );
        let stall = Duration::from_secs(3);
        let started = Instant::now();

        encode_watched(&mut Command::new(progresses), stall, "render query.mp3").unwrap();

        assert!(started.elapsed() > stall, "{:?}", started.elapsed());
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_encode_whose_progress_report_repeats_is_stopped() {
        let dir = scratch("encode-repeats");
        // What FFmpeg 8.1.3 writes while its input stalls: the same report
        // every period (brief 78), here every 0.1 s for up to 30 s.
        let repeats = stand_in(
            &dir,
            "i=0; while [ $i -lt 300 ]; do echo 'out_time_us=2822290'; echo 'total_size=1024'; echo 'speed=0.5x'; echo 'progress=continue'; sleep 0.1; i=$((i + 1)); done",
        );
        let started = Instant::now();

        let error = encode_watched(
            &mut Command::new(repeats),
            STALL_IN_TESTS,
            "render query.mp3",
        )
        .unwrap_err();

        assert!(
            started.elapsed() < Duration::from_secs(25),
            "{:?}",
            started.elapsed()
        );
        assert!(error.contains("no progress for 2 s"), "{error}");
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_render_to_a_pipe_that_writes_nothing_is_stopped() {
        let dir = scratch("pipe-hangs");
        let hangs = stand_in(&dir, "exec sleep 60");
        let started = Instant::now();

        let error =
            pipe_watched(&mut Command::new(hangs), STALL_IN_TESTS, "render track.mp3").unwrap_err();

        assert!(
            started.elapsed() < Duration::from_secs(30),
            "{:?}",
            started.elapsed()
        );
        assert!(
            error.starts_with("could not render track.mp3: no progress for 2 s"),
            "{error}"
        );
        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn an_excerpt_encodes_with_its_progress_watched() {
        let dir = scratch("excerpt");
        let tone = dir.join("tone.wav");
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("sine=frequency=440:sample_rate=44100:duration=4")
            .arg(&tone)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        let output = dir.join("excerpt.mp3");

        excerpt_once(&tone, 1.0, 2.0, 1.05, Encoding::Mp3(128), &output).unwrap();

        let recipe = Recipe::new(
            &[&tone],
            String::from("render_excerpt from 1.0 s for 2.0 s at speed 1.05, Mp3(128)"),
        )
        .unwrap();
        assert!(is_complete(&output, &recipe));
        assert!(
            recipe
                .renderer
                .starts_with("gunfinger-eval renders 1; ffmpeg version ")
        );
        fs::remove_dir_all(&dir).ok();
    }
}

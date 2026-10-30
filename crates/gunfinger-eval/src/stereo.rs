//! Two channels of a query, for the stereo faults of `robust` (experiment
//! 0194 and after): which channel or mix of the two is searched, and how
//! wide a source is. The core always searches FFmpeg's downmix (`-ac 1`
//! in `decode.rs`); the other choices here are a diagnostic, and nothing
//! outside the harness uses them. `FrontEnd` is the variant that adds the
//! two channels' powers instead (experiment 0245 and after), for queries
//! alone or for queries and records.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use gunfinger_core::decode::{Container, Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::extract_signal_peaks;
use gunfinger_core::profile::Profile;
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::store::{PeakRecord, PeakStore, RecordHeader};
use serde::Serialize;

use crate::render::Stereo;
use crate::rng::Rng;

/// What `robust` searches of each query.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum QueryChannels {
    /// The core's decoding, unchanged: FFmpeg's downmix.
    Downmix,
    Left,
    Right,
    /// The channel with the higher RMS over the query.
    Louder,
    /// (L - R) / 2: the mid of the two channels again, (L + R) / 2, when
    /// one of them is inverted.
    Side,
}

impl QueryChannels {
    pub fn name(self) -> &'static str {
        match self {
            QueryChannels::Downmix => "downmix",
            QueryChannels::Left => "left",
            QueryChannels::Right => "right",
            QueryChannels::Louder => "louder",
            QueryChannels::Side => "side",
        }
    }

    /// The samples searched for the file at `path`, at `sample_rate`.
    pub fn decode(self, path: &Path, sample_rate: u32) -> Result<Vec<f32>, String> {
        let stereo = || decode_stereo(path, sample_rate);
        Ok(match self {
            QueryChannels::Downmix => {
                decode(path, sample_rate, Excerpt::default())
                    .map_err(|error| error.to_string())?
                    .samples
            }
            QueryChannels::Left => stereo()?.left,
            QueryChannels::Right => stereo()?.right,
            QueryChannels::Louder => louder(stereo()?),
            QueryChannels::Side => side(&stereo()?),
        })
    }
}

/// How the harness analyses audio: today's downmix, or both channels with
/// their powers added (`Signal::PowerSum`).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, clap::ValueEnum)]
pub enum FrontEnd {
    /// Today's: queries and records from FFmpeg's downmix.
    #[default]
    Downmix,
    /// Queries by their channels' powers, against today's records.
    PowerSumQueries,
    /// Queries by their channels' powers, against records made the same
    /// way (`Profile::POWER_SUM`; a store `power-store` builds, named with
    /// `--peaks-dir`).
    PowerSum,
}

impl FrontEnd {
    pub fn name(self) -> &'static str {
        match self {
            FrontEnd::Downmix => "downmix",
            FrontEnd::PowerSumQueries => "power-sum-queries",
            FrontEnd::PowerSum => "power-sum",
        }
    }

    /// The profile of the records searched, and of the search's analysis.
    pub fn profile(self) -> Profile {
        match self {
            FrontEnd::Downmix | FrontEnd::PowerSumQueries => Profile::CURRENT,
            FrontEnd::PowerSum => Profile::POWER_SUM,
        }
    }

    /// The file at `path` as this front end searches it, at `sample_rate`.
    pub fn decode_query(self, path: &Path, sample_rate: u32) -> Result<QueryAudio, String> {
        match self {
            FrontEnd::Downmix => Ok(QueryAudio::Mono(
                decode(path, sample_rate, Excerpt::default())
                    .map_err(|error| error.to_string())?
                    .samples,
            )),
            FrontEnd::PowerSumQueries | FrontEnd::PowerSum => {
                Ok(QueryAudio::PowerSum(decode_stereo(path, sample_rate)?))
            }
        }
    }
}

/// A query's decoded audio: one channel, or two whose powers are added.
pub enum QueryAudio {
    Mono(Vec<f32>),
    PowerSum(Stereo),
}

impl QueryAudio {
    pub fn signal(&self) -> Signal<'_> {
        match self {
            QueryAudio::Mono(samples) => Signal::Mono(samples),
            QueryAudio::PowerSum(stereo) => Signal::PowerSum {
                left: &stereo.left,
                right: &stereo.right,
            },
        }
    }

    /// The samples of one channel, which the peak verifier needs.
    pub fn mono(&self) -> Option<&[f32]> {
        match self {
            QueryAudio::Mono(samples) => Some(samples),
            QueryAudio::PowerSum(_) => None,
        }
    }
}

/// What `build_store` wrote.
#[derive(Serialize)]
pub struct StoreBuilt {
    pub front_end: &'static str,
    pub profile: String,
    pub records: usize,
    /// Files decoded from a replacement instead of their library file.
    pub replaced: usize,
    pub seconds_of_audio: f64,
    pub peaks: usize,
    pub postings: usize,
    pub wall_seconds: f64,
}

/// Writes to `out` a record under `front_end`'s profile for each library
/// file with a current record in `store` (today's profile), decoding it, or
/// its replacement in `replacements` (by library path), whole at the
/// profile's rate. `out` must not be `store`: the records of a variant or of
/// a replaced file never go to the library's own store.
pub fn build_store(
    library: &Library,
    store: &PeakStore,
    out: &PeakStore,
    front_end: FrontEnd,
    replacements: &BTreeMap<String, PathBuf>,
    jobs: usize,
) -> Result<StoreBuilt, String> {
    if front_end == FrontEnd::PowerSumQueries {
        return Err(String::from(
            "a store is built for --front-end downmix or power-sum",
        ));
    }
    let same_dir = |a: &Path, b: &Path| a.canonicalize().ok() == b.canonicalize().ok();
    if same_dir(out.dir(), store.dir()) {
        return Err(format!(
            "{} is the library's store; build into another directory",
            out.dir().display()
        ));
    }
    out.claim_library(&library.root)
        .map_err(|error| error.to_string())?;
    let started = std::time::Instant::now();
    let profile = front_end.profile();
    let (current, _) = load_records(library, store, &Profile::CURRENT, &BTreeSet::new());
    let sources: Vec<_> = current
        .into_iter()
        .map(|record| record.header.source)
        .collect();
    let records = map_in_order(&sources, jobs, |source| {
        let file = replacements
            .get(&source.path)
            .cloned()
            .unwrap_or_else(|| library.root.join(&source.path));
        let audio = front_end.decode_query(&file, profile.sample_rate)?;
        let signal = audio.signal();
        let record = PeakRecord {
            header: RecordHeader {
                profile: profile.id(),
                source: source.clone(),
                duration_seconds: signal.len() as f64 / f64::from(profile.sample_rate),
            },
            peaks: extract_signal_peaks(signal, &profile),
        };
        out.save(&record).map_err(|error| error.to_string())?;
        Ok(record)
    })
    .into_iter()
    .collect::<Result<Vec<PeakRecord>, String>>()?;
    let index = Index::build(&records).map_err(|error| error.to_string())?;
    Ok(StoreBuilt {
        front_end: front_end.name(),
        profile: profile.id(),
        records: records.len(),
        replaced: sources
            .iter()
            .filter(|source| replacements.contains_key(&source.path))
            .count(),
        seconds_of_audio: records
            .iter()
            .map(|record| record.header.duration_seconds)
            .sum(),
        peaks: records.iter().map(|record| record.peaks.len()).sum(),
        postings: index.posting_count(),
        wall_seconds: started.elapsed().as_secs_f64(),
    })
}

/// The channel with the higher RMS.
fn louder(stereo: Stereo) -> Vec<f32> {
    if mean_square(&stereo.right) > mean_square(&stereo.left) {
        stereo.right
    } else {
        stereo.left
    }
}

/// (L - R) / 2.
fn side(stereo: &Stereo) -> Vec<f32> {
    stereo
        .left
        .iter()
        .zip(&stereo.right)
        .map(|(&left, &right)| (left - right) / 2.0)
        .collect()
}

/// Both channels of `path` at `sample_rate`, as the core decodes one (the
/// first audio stream, the same container handling) but with `-ac 2`: a
/// one-channel file gives two identical channels.
pub fn decode_stereo(path: &Path, sample_rate: u32) -> Result<Stereo, String> {
    let output = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-threads", "1"])
        .args(Container::of(path).input_args())
        .arg("-i")
        .arg(path)
        .args(["-map", "0:a:0", "-vn", "-sn", "-dn", "-ac", "2", "-ar"])
        .arg(sample_rate.to_string())
        .args(["-f", "f32le", "pipe:1"])
        .stdin(Stdio::null())
        .output()
        .map_err(|error| format!("cannot run ffmpeg: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "ffmpeg could not decode {} as two channels: {}",
            path.display(),
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let samples: Vec<f32> = output
        .stdout
        .as_chunks::<4>()
        .0
        .iter()
        .map(|&bytes| f32::from_le_bytes(bytes))
        .collect();
    Ok(Stereo::split(&samples))
}

/// Mean squares below this (-120 dB of full scale) count as this, so that
/// identical channels (no side) and a channel inverted against an
/// identical one (no mid) give finite ratios of about -120 and +120 dB.
const SILENCE: f64 = 1e-12;

/// The side's energy against the mid's, in dB: S = (L - R) / 2 and
/// M = (L + R) / 2 over the whole of both channels. Near minus infinity for
/// a mono source; above 0 dB when the side is louder, as when one channel
/// of a narrow source is inverted.
pub fn side_to_mid_db(stereo: &Stereo) -> f64 {
    let (mut side, mut mid) = (0.0_f64, 0.0_f64);
    for (&left, &right) in stereo.left.iter().zip(&stereo.right) {
        let (left, right) = (f64::from(left), f64::from(right));
        side += ((left - right) / 2.0).powi(2);
        mid += ((left + right) / 2.0).powi(2);
    }
    let frames = stereo.left.len().max(1) as f64;
    10.0 * ((side / frames).max(SILENCE) / (mid / frames).max(SILENCE)).log10()
}

fn mean_square(samples: &[f32]) -> f64 {
    samples.iter().map(|&x| f64::from(x).powi(2)).sum::<f64>() / samples.len().max(1) as f64
}

/// How wide a source is, by its side-to-mid ratio (brief 42's
/// pre-registration).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Width {
    /// -40 dB or less.
    Mono,
    /// Above -40 dB, up to -20 dB.
    NearlyMono,
    /// Above -20 dB, up to -10 dB.
    Narrow,
    /// Above -10 dB.
    Wide,
}

impl Width {
    pub fn of(side_to_mid_db: f64) -> Width {
        if side_to_mid_db <= -40.0 {
            Width::Mono
        } else if side_to_mid_db <= -20.0 {
            Width::NearlyMono
        } else if side_to_mid_db <= -10.0 {
            Width::Narrow
        } else {
            Width::Wide
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Width::Mono => "mono",
            Width::NearlyMono => "nearly-mono",
            Width::Narrow => "narrow",
            Width::Wide => "wide",
        }
    }
}

#[derive(Serialize)]
pub struct WidthReport {
    pub seed: u64,
    /// Library files with a current peak record, the population sampled.
    pub population: usize,
    pub files: Vec<FileWidth>,
}

#[derive(Serialize)]
pub struct FileWidth {
    pub asset: String,
    pub seconds: f64,
    pub side_to_mid_db: f64,
    pub width: &'static str,
}

/// The width of `sample` library files with a current peak record, drawn
/// with `seed`, each decoded whole as two channels at the profile's rate.
pub fn sample_widths(
    library: &Library,
    store: &PeakStore,
    seed: u64,
    sample: usize,
    jobs: usize,
) -> Result<WidthReport, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let mut assets: Vec<String> = records
        .into_iter()
        .map(|record| record.header.source.path)
        .collect();
    assets.sort();
    let population = assets.len();
    Rng::new(seed).shuffle(&mut assets);
    assets.truncate(sample);
    assets.sort();
    let files = map_in_order(&assets, jobs, |asset| {
        let stereo = decode_stereo(&library.root.join(asset), profile.sample_rate)?;
        let ratio = side_to_mid_db(&stereo);
        Ok(FileWidth {
            asset: asset.clone(),
            seconds: stereo.left.len() as f64 / f64::from(profile.sample_rate),
            side_to_mid_db: ratio,
            width: Width::of(ratio).name(),
        })
    })
    .into_iter()
    .collect::<Result<Vec<FileWidth>, String>>()?;
    Ok(WidthReport {
        seed,
        population,
        files,
    })
}

pub fn print_widths(report: &WidthReport) {
    println!(
        "stereo width, seed {}: {} of {} files",
        report.seed,
        report.files.len(),
        report.population
    );
    for width in [Width::Mono, Width::NearlyMono, Width::Narrow, Width::Wide] {
        let count = report
            .files
            .iter()
            .filter(|file| file.width == width.name())
            .count();
        println!("{:<12} {count:>4}", width.name());
    }
    let louder_side = report
        .files
        .iter()
        .filter(|file| file.side_to_mid_db > 0.0)
        .count();
    println!("side louder than mid: {louder_side}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stereo(left: &[f32], right: &[f32]) -> Stereo {
        Stereo {
            left: left.to_vec(),
            right: right.to_vec(),
        }
    }

    #[test]
    fn the_louder_channel_is_the_one_with_more_energy() {
        assert_eq!(louder(stereo(&[0.5, 0.2], &[0.1, -0.2])), [0.5, 0.2]);
        assert_eq!(
            louder(stereo(&[0.1, -0.1, 0.1], &[0.0, 0.0, 0.4])),
            [0.0, 0.0, 0.4]
        );
    }

    #[test]
    fn the_side_is_half_the_difference_of_the_channels() {
        assert_eq!(side(&stereo(&[0.5, 0.2], &[0.1, -0.2])), [0.2, 0.2]);
    }

    #[test]
    fn the_side_of_an_inverted_channel_is_the_original_mid() {
        let left = [0.3, -0.1, 0.25];
        let right = [0.1, 0.2, -0.05];
        let inverted: Vec<f32> = right.iter().map(|x| -x).collect();

        let side = side(&stereo(&left, &inverted));

        for ((a, l), r) in side.iter().zip(&left).zip(&right) {
            assert!((a - (l + r) / 2.0).abs() < 1e-7);
        }
    }

    #[test]
    fn width_runs_from_mono_to_an_inverted_channel() {
        let tone: Vec<f32> = (0..8_000)
            .map(|index| (index as f32 * 0.05).sin())
            .collect();
        let quieter: Vec<f32> = tone.iter().map(|x| x * 0.9).collect();
        let inverted: Vec<f32> = tone.iter().map(|x| -x).collect();

        let mono = side_to_mid_db(&stereo(&tone, &tone));
        let narrow = side_to_mid_db(&stereo(&tone, &quieter));
        let flipped = side_to_mid_db(&stereo(&tone, &inverted));

        assert!(mono < -100.0, "{mono}");
        // S = 0.05 x, M = 0.95 x: 20 log10(0.05 / 0.95).
        assert!((narrow - (-25.575)).abs() < 0.01, "{narrow}");
        assert!(flipped > 100.0, "{flipped}");
        assert_eq!(Width::of(mono), Width::Mono);
        assert_eq!(Width::of(narrow), Width::NearlyMono);
        assert_eq!(Width::of(-20.0), Width::NearlyMono);
        assert_eq!(Width::of(-15.0), Width::Narrow);
        assert_eq!(Width::of(-3.0), Width::Wide);
    }

    /// A one-channel file: FFmpeg's two channels are each the sample at
    /// 1/sqrt(2), so their powers add up to the downmix's (the sample
    /// itself), apart from rounding.
    #[test]
    fn a_one_channel_file_gives_the_downmix_peaks_under_the_power_sum() {
        let dir = std::env::temp_dir().join(format!("gunfinger-power-sum-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("mono.wav");
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("anoisesrc=d=8:c=pink:r=44100:a=0.3:seed=7")
            .args(["-ac", "1"])
            .arg(&file)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }
        let profile = Profile::CURRENT;
        let peaks = |front_end: FrontEnd| {
            let audio = front_end.decode_query(&file, profile.sample_rate).unwrap();
            extract_signal_peaks(audio.signal(), &profile)
        };

        let downmix = peaks(FrontEnd::Downmix);
        let summed = peaks(FrontEnd::PowerSum);

        assert!(downmix.len() > 200, "{}", downmix.len());
        let same = summed.iter().filter(|peak| downmix.contains(peak)).count();
        assert!(
            same as f64 >= 0.99 * downmix.len() as f64,
            "{same} of {} the same",
            downmix.len()
        );
        assert!(summed.len().abs_diff(downmix.len()) <= downmix.len() / 100);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn a_stereo_file_decodes_to_its_two_channels() {
        let dir = std::env::temp_dir().join(format!("gunfinger-stereo-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("two-tones.wav");
        // 440 Hz on the left, silence on the right.
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("aevalsrc=sin(440*2*PI*t)|0:s=44100:d=2")
            .arg(&file)
            .status()
            .is_ok_and(|status| status.success());
        if !generated {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        }

        let decoded = decode_stereo(&file, 8_000).unwrap();
        let left = QueryChannels::Left.decode(&file, 8_000).unwrap();
        let right = QueryChannels::Right.decode(&file, 8_000).unwrap();
        let downmix = QueryChannels::Downmix.decode(&file, 8_000).unwrap();

        assert!((15_900..=16_100).contains(&decoded.left.len()));
        assert_eq!(decoded.left.len(), decoded.right.len());
        assert!(mean_square(&left) > 0.4, "{}", mean_square(&left));
        assert!(mean_square(&right) < 1e-9);
        // FFmpeg's downmix adds each channel at 1/sqrt(2): half the
        // energy of a channel the other lacks.
        let ratio = mean_square(&downmix) / mean_square(&left);
        assert!((ratio - 0.5).abs() < 0.02, "{ratio}");
        std::fs::remove_dir_all(&dir).ok();
    }
}

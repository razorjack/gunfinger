//! Two channels of a query, for the stereo faults of `robust` (experiment
//! 0194 and after): which channel or mix of the two is searched, and how
//! wide a source is. The core always searches FFmpeg's downmix (`-ac 1`
//! in `decode.rs`); the other choices here are a diagnostic, and nothing
//! outside the harness uses them. `FrontEnd` is the variant that adds the
//! two channels' powers instead (experiment 0245 and after), for queries
//! alone or for queries and records. `Polarity` is the check at indexing
//! of brief 67: a record picked from the downmix with one channel's
//! polarity reversed, only for a file whose channels are inverted.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use gunfinger_core::decode::{Container, Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::ignore::{Pattern, leaving_out};
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::extract_signal_peaks;
use gunfinger_core::profile::Profile;
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::store::{PeakRecord, PeakStore, RecordHeader};
use serde::Serialize;

use crate::recall::{KINDS, Kind};
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
    /// way (`Profile::POWER_SUM`; a store `variant-store` builds, named with
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
    /// Files whose record was picked from the downmix with one channel's
    /// polarity reversed (`Polarity::RepairInverted`).
    pub repaired: Vec<String>,
    pub seconds_of_audio: f64,
    pub peaks: usize,
    pub postings: usize,
    pub wall_seconds: f64,
}

/// Writes to `out` a record under `front_end`'s profile for each library
/// file with a current record in `store` (today's profile), decoding it, or
/// its replacement in `replacements` (by library path), whole at the
/// profile's rate, with its polarity checked as `polarity` says. `out` must
/// not be `store`: the records of a variant or of a replaced file never go
/// to the library's own store.
pub fn build_store(
    library: &Library,
    store: &PeakStore,
    out: &PeakStore,
    front_end: FrontEnd,
    polarity: Polarity,
    replacements: &BTreeMap<String, PathBuf>,
    jobs: usize,
) -> Result<StoreBuilt, String> {
    if front_end == FrontEnd::PowerSumQueries {
        return Err(String::from(
            "a store is built for --front-end downmix or power-sum",
        ));
    }
    if polarity == Polarity::RepairInverted && front_end != FrontEnd::Downmix {
        return Err(String::from(
            "inverted channels are repaired only under --front-end downmix",
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
        let (audio, checked) = match polarity {
            Polarity::AsDecoded => (
                front_end.decode_query(&file, profile.sample_rate)?,
                Checked::Unchanged,
            ),
            Polarity::RepairInverted => {
                let (samples, checked) = decode_repairing(&file, profile.sample_rate)?;
                (QueryAudio::Mono(samples), checked)
            }
        };
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
        Ok((record, checked))
    })
    .into_iter()
    .collect::<Result<Vec<(PeakRecord, Checked)>, String>>()?;
    let repaired = records
        .iter()
        .filter(|(_, checked)| *checked == Checked::Repaired)
        .map(|(record, _)| record.header.source.path.clone())
        .collect();
    let records: Vec<PeakRecord> = records.into_iter().map(|(record, _)| record).collect();
    let index = Index::build(&records).map_err(|error| error.to_string())?;
    Ok(StoreBuilt {
        front_end: front_end.name(),
        profile: profile.id(),
        records: records.len(),
        replaced: sources
            .iter()
            .filter(|source| replacements.contains_key(&source.path))
            .count(),
        repaired,
        seconds_of_audio: records
            .iter()
            .map(|record| record.header.duration_seconds)
            .sum(),
        peaks: records.iter().map(|record| record.peaks.len()).sum(),
        postings: index.posting_count(),
        wall_seconds: started.elapsed().as_secs_f64(),
    })
}

/// Whether `build_store` checks each file's channels for an inverted one
/// (brief 67's pre-registration).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Polarity {
    /// Today's: every file as the front end decodes it.
    AsDecoded,
    /// A file whose channels are inverted (`inverted`) gets the downmix with
    /// one channel's polarity reversed; every other file today's downmix.
    RepairInverted,
}

/// What the check did to one file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Checked {
    Unchanged,
    Repaired,
}

/// The file at `path` at `sample_rate` as `Polarity::RepairInverted`
/// indexes it. Only an inverted file is mixed down here; every other file
/// goes through the core's `decode`, so its record stays byte-identical to
/// today's (FFmpeg's own downmix need not equal one made from its two
/// channels to the bit).
pub fn decode_repairing(path: &Path, sample_rate: u32) -> Result<(Vec<f32>, Checked), String> {
    let stereo = decode_stereo(path, sample_rate)?;
    if inverted(&channels(&stereo)) {
        return Ok((reversed_downmix(&stereo), Checked::Repaired));
    }
    let samples = decode(path, sample_rate, Excerpt::default())
        .map_err(|error| error.to_string())?
        .samples;
    Ok((samples, Checked::Unchanged))
}

/// The detector of brief 67: both channels present (within
/// `ONE_CHANNEL_DB`), correlated at `INVERTED_CORRELATION` or below at lag
/// 0, and most strongly there within `MAX_LAG`, so that a delayed channel
/// is not taken for an inverted one. A negative correlation already makes
/// the side louder than the mid, so the side-to-mid ratio adds nothing.
pub fn inverted(channels: &Channels) -> bool {
    channels.balance_db.abs() < ONE_CHANNEL_DB
        && channels.correlation <= INVERTED_CORRELATION
        && channels.strongest_lag == 0
}

/// (L - R) / sqrt(2): FFmpeg's downmix, (L + R) / sqrt(2), with the right
/// channel's polarity reversed. Reversing the left instead negates every
/// sample, which leaves the spectrogram's power, and so the peaks, as
/// they are.
fn reversed_downmix(stereo: &Stereo) -> Vec<f32> {
    stereo
        .left
        .iter()
        .zip(&stereo.right)
        .map(|(&left, &right)| (left - right) * std::f32::consts::FRAC_1_SQRT_2)
        .collect()
}

/// The census's windows for a file inverted only partway: 30 s, the last
/// kept from 10 s.
const WINDOW_SECONDS: usize = 30;
const LAST_WINDOW_SECONDS: usize = 10;

/// The correlation at lag 0 in each window of `stereo` at `sample_rate`.
pub fn window_correlations(stereo: &Stereo, sample_rate: u32) -> Vec<f64> {
    let rate = sample_rate as usize;
    let frames = stereo.left.len().min(stereo.right.len());
    (0..frames)
        .step_by(WINDOW_SECONDS * rate)
        .map(|start| start..frames.min(start + WINDOW_SECONDS * rate))
        .filter(|window| window.len() >= LAST_WINDOW_SECONDS * rate)
        .map(|window| {
            let (left, right) = (&stereo.left[window.clone()], &stereo.right[window]);
            let scale = (mean_square(left).max(SILENCE) * mean_square(right).max(SILENCE)).sqrt()
                * left.len() as f64;
            dot(left, right) / scale
        })
        .collect()
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

/// How the two channels of a file relate (experiment 0248): the left
/// channel's power against the right's, their correlation at lag 0, and the
/// lag within `MAX_LAG` samples where the correlation is strongest (a
/// positive lag: the right channel late). The correlation is not centred
/// (audio has next to no DC) and is normalised by the whole channels'
/// energies, which over a file differs from the overlap's by at most
/// `MAX_LAG` samples.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Channels {
    pub balance_db: f64,
    pub correlation: f64,
    pub strongest_lag: i32,
    pub strongest_correlation: f64,
}

/// 10 ms at 8 kHz: twice session 42's longest delay (5 ms).
pub const MAX_LAG: usize = 80;

/// The census's classes (brief 62's pre-registration), from session 42's
/// synthetic faults: one channel 20 dB below the other; an inverted channel
/// (a correlation of about -0.8 to -1 on narrow sources); a delay of 0.5 ms
/// or more (1 and 5 ms are 8 and 40 samples at 8 kHz; 0.2 ms cost nothing
/// in experiment 0194).
const ONE_CHANNEL_DB: f64 = 20.0;
const INVERTED_CORRELATION: f64 = -0.5;
const DELAYED_MIN_LAG: u32 = 4;
const DELAYED_CORRELATION: f64 = 0.5;
const DELAYED_GAIN: f64 = 0.05;

pub fn channels(stereo: &Stereo) -> Channels {
    let (left, right) = (&stereo.left, &stereo.right);
    let frames = left.len().min(right.len());
    let (left, right) = (&left[..frames], &right[..frames]);
    let (left_power, right_power) = (mean_square(left), mean_square(right));
    let scale = (left_power.max(SILENCE) * right_power.max(SILENCE)).sqrt() * frames.max(1) as f64;
    let at = |lag: isize| {
        let shift = lag.unsigned_abs().min(frames);
        let product = if lag >= 0 {
            dot(&left[..frames - shift], &right[shift..])
        } else {
            dot(&left[shift..], &right[..frames - shift])
        };
        product / scale
    };
    let correlation = at(0);
    let (mut strongest_lag, mut strongest_correlation) = (0, correlation);
    let reach = MAX_LAG as isize;
    for lag in -reach..=reach {
        let value = at(lag);
        if value.abs() > strongest_correlation.abs() {
            (strongest_lag, strongest_correlation) = (lag, value);
        }
    }
    Channels {
        balance_db: 10.0 * (left_power.max(SILENCE) / right_power.max(SILENCE)).log10(),
        correlation,
        strongest_lag: strongest_lag as i32,
        strongest_correlation,
    }
}

fn dot(a: &[f32], b: &[f32]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(&x, &y)| f64::from(x) * f64::from(y))
        .sum()
}

/// A file's class in the census, the first that applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    OneChannel,
    Inverted,
    Delayed,
    Width(Width),
}

impl Class {
    pub fn of(channels: &Channels, side_to_mid_db: f64) -> Class {
        if channels.balance_db.abs() >= ONE_CHANNEL_DB {
            Class::OneChannel
        } else if channels.strongest_correlation <= INVERTED_CORRELATION {
            Class::Inverted
        } else if channels.strongest_lag.unsigned_abs() >= DELAYED_MIN_LAG
            && channels.strongest_correlation >= DELAYED_CORRELATION
            && channels.strongest_correlation - channels.correlation >= DELAYED_GAIN
        {
            Class::Delayed
        } else {
            Class::Width(Width::of(side_to_mid_db))
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Class::OneChannel => "one-channel",
            Class::Inverted => "inverted",
            Class::Delayed => "delayed",
            Class::Width(width) => width.name(),
        }
    }
}

const CLASSES: [Class; 7] = [
    Class::OneChannel,
    Class::Inverted,
    Class::Delayed,
    Class::Width(Width::Mono),
    Class::Width(Width::NearlyMono),
    Class::Width(Width::Narrow),
    Class::Width(Width::Wide),
];

#[derive(Serialize)]
pub struct WidthReport {
    /// Absent for `--files`, which measures every file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
    /// The files sampled from: the library's with a current peak record,
    /// or the other library's left after `--exclude` and its ignore file.
    pub population: usize,
    /// The other library's population by kind of source.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub population_by_kind: Vec<KindCount>,
    pub files: Vec<FileWidth>,
}

#[derive(Serialize)]
pub struct KindCount {
    pub kind: Kind,
    pub files: usize,
}

#[derive(Serialize)]
pub struct FileWidth {
    pub asset: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    pub seconds: f64,
    pub side_to_mid_db: f64,
    pub width: &'static str,
    pub channels: Channels,
    pub class: &'static str,
    /// Whether the check at indexing (`inverted`) would repair the record.
    pub repaired: bool,
    /// The 30 s windows, and those correlated at `INVERTED_CORRELATION` or
    /// below at lag 0: a file inverted only partway.
    pub windows: usize,
    pub inverted_windows: usize,
}

/// The file at `path` decoded whole as two channels at the profile's rate,
/// and measured.
fn measure(path: &Path, asset: &str, kind: Option<Kind>) -> Result<FileWidth, String> {
    let sample_rate = Profile::CURRENT.sample_rate;
    let stereo = decode_stereo(path, sample_rate)?;
    let ratio = side_to_mid_db(&stereo);
    let channels = channels(&stereo);
    let windows = window_correlations(&stereo, sample_rate);
    Ok(FileWidth {
        asset: asset.to_owned(),
        kind,
        seconds: stereo.left.len() as f64 / f64::from(sample_rate),
        side_to_mid_db: ratio,
        width: Width::of(ratio).name(),
        channels,
        class: Class::of(&channels, ratio).name(),
        repaired: inverted(&channels),
        windows: windows.len(),
        inverted_windows: windows
            .iter()
            .filter(|&&correlation| correlation <= INVERTED_CORRELATION)
            .count(),
    })
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
    let (records, _) = load_records(library, store, &Profile::CURRENT, &BTreeSet::new());
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
        measure(&library.root.join(asset), asset, None)
    })
    .into_iter()
    .collect::<Result<Vec<FileWidth>, String>>()?;
    Ok(WidthReport {
        seed: Some(seed),
        population,
        population_by_kind: Vec::new(),
        files,
    })
}

/// Files and folders of the other library left out of its sample: a file
/// by its path, a folder by its path and a trailing `/`, one per line
/// (`#` starts a comment).
pub struct LeftOut {
    files: BTreeSet<String>,
    folders: Vec<String>,
}

impl LeftOut {
    pub fn parse(text: &str) -> LeftOut {
        let lines = text
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'));
        let (folders, files): (Vec<&str>, Vec<&str>) = lines.partition(|line| line.ends_with('/'));
        LeftOut {
            files: files.into_iter().map(str::to_owned).collect(),
            folders: folders.into_iter().map(str::to_owned).collect(),
        }
    }

    pub fn contains(&self, path: &str) -> bool {
        self.files.contains(path) || self.folders.iter().any(|folder| path.starts_with(folder))
    }
}

/// The census of the other library (experiment 0248): `sample` of the
/// files with a current record there, an equal share from each kind of
/// source (the first kinds take what does not divide), drawn with `seed`
/// from those that neither `ignore` (its ignore file's patterns) nor
/// `left_out` leaves out, each decoded whole from `root` and measured.
pub fn sample_other_widths(
    assets: &[Asset],
    root: &Path,
    ignore: &[Pattern],
    left_out: &LeftOut,
    seed: u64,
    sample: usize,
    jobs: usize,
) -> Result<WidthReport, String> {
    let paths: Vec<&str> = assets
        .iter()
        .map(|asset| asset.path.as_str())
        .filter(|path| leaving_out(ignore, path).is_none() && !left_out.contains(path))
        .collect();
    let population_by_kind = KINDS
        .iter()
        .map(|&kind| KindCount {
            kind,
            files: paths.iter().filter(|path| Kind::of(path) == kind).count(),
        })
        .collect();
    let drawn = draw_by_kind(&paths, seed, sample);
    let files = map_in_order(&drawn, jobs, |(kind, path)| {
        measure(&root.join(path), path, Some(*kind))
    })
    .into_iter()
    .collect::<Result<Vec<FileWidth>, String>>()?;
    Ok(WidthReport {
        seed: Some(seed),
        population: paths.len(),
        population_by_kind,
        files,
    })
}

/// `sample` of `paths`, an equal share of each kind, each kind's shuffled
/// in turn by one generator; sorted by kind, then path.
fn draw_by_kind(paths: &[&str], seed: u64, sample: usize) -> Vec<(Kind, String)> {
    let mut rng = Rng::new(seed);
    let mut drawn = Vec::new();
    for (position, &kind) in KINDS.iter().enumerate() {
        let share = sample / KINDS.len() + usize::from(position < sample % KINDS.len());
        let mut of_kind: Vec<&str> = paths
            .iter()
            .copied()
            .filter(|path| Kind::of(path) == kind)
            .collect();
        of_kind.sort_unstable();
        rng.shuffle(&mut of_kind);
        of_kind.truncate(share);
        of_kind.sort_unstable();
        drawn.extend(of_kind.into_iter().map(|path| (kind, path.to_owned())));
    }
    drawn
}

/// Every MP3 under `dir`, measured alike: session 42's faulted renders,
/// against which the census's classes were checked.
pub fn file_widths(dir: &Path, jobs: usize) -> Result<WidthReport, String> {
    let mut paths = Vec::new();
    mp3s_under(dir, &mut paths)?;
    paths.sort();
    let files = map_in_order(&paths, jobs, |path| {
        let name = path.strip_prefix(dir).unwrap_or(path).to_string_lossy();
        measure(path, &name, None)
    })
    .into_iter()
    .collect::<Result<Vec<FileWidth>, String>>()?;
    Ok(WidthReport {
        seed: None,
        population: files.len(),
        population_by_kind: Vec::new(),
        files,
    })
}

fn mp3s_under(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|error| format!("cannot list {}: {error}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            mp3s_under(&path, paths)?;
        } else if path.extension().is_some_and(|extension| extension == "mp3") {
            paths.push(path);
        }
    }
    Ok(())
}

pub fn print_widths(report: &WidthReport) {
    match report.seed {
        Some(seed) => println!(
            "stereo width, seed {seed}: {} of {} files",
            report.files.len(),
            report.population
        ),
        None => println!("stereo width: {} files", report.files.len()),
    }
    let kinds: Vec<Option<Kind>> = if report.population_by_kind.is_empty() {
        vec![None]
    } else {
        KINDS.iter().copied().map(Some).collect()
    };
    for kind in kinds {
        if let Some(kind) = kind {
            println!("{kind:?}");
        }
        for class in CLASSES {
            let count = report
                .files
                .iter()
                .filter(|file| file.class == class.name() && (kind.is_none() || file.kind == kind))
                .count();
            println!("  {:<12} {count:>4}", class.name());
        }
    }
    let louder_side = report
        .files
        .iter()
        .filter(|file| file.side_to_mid_db > 0.0)
        .count();
    println!("side louder than mid: {louder_side}");
    let repaired = report.files.iter().filter(|file| file.repaired).count();
    let partly = report
        .files
        .iter()
        .filter(|file| !file.repaired && file.inverted_windows > 0)
        .count();
    println!("repaired by the check: {repaired}; partly inverted, not repaired: {partly}");
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

    /// Seeded white noise, as a stand-in for broadband music.
    fn noise(frames: usize, seed: u64) -> Vec<f32> {
        let mut state = seed;
        (0..frames)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                ((state >> 40) as f32 / (1u64 << 24) as f32) - 0.5
            })
            .collect()
    }

    fn class_of(left: &[f32], right: &[f32]) -> (Channels, Class) {
        let pair = stereo(left, right);
        let channels = channels(&pair);
        (channels, Class::of(&channels, side_to_mid_db(&pair)))
    }

    #[test]
    fn the_census_tells_each_of_session_42s_faults_apart() {
        let mid = noise(16_000, 1);
        let other = noise(16_000, 2);
        // A narrow source: the right channel the mid plus a little of its own.
        let right: Vec<f32> = mid.iter().zip(&other).map(|(m, o)| m + 0.5 * o).collect();

        let (clean, class) = class_of(&mid, &right);
        assert_eq!(class, Class::Width(Width::Narrow), "{clean:?}");
        assert_eq!(clean.strongest_lag, 0);

        let inverted: Vec<f32> = right.iter().map(|x| -x).collect();
        let (channels, class) = class_of(&mid, &inverted);
        assert_eq!(class, Class::Inverted, "{channels:?}");
        assert!((channels.correlation + clean.correlation).abs() < 1e-9);

        let silent = vec![0.0; right.len()];
        assert_eq!(class_of(&mid, &silent).1, Class::OneChannel);

        // 5 ms at 8 kHz: the right channel 40 samples late.
        let late: Vec<f32> = [vec![0.0; 40], right[..right.len() - 40].to_vec()].concat();
        let (channels, class) = class_of(&mid, &late);
        assert_eq!(class, Class::Delayed, "{channels:?}");
        assert_eq!(channels.strongest_lag, 40);

        let early: Vec<f32> = [right[8..].to_vec(), vec![0.0; 8]].concat();
        assert_eq!(class_of(&mid, &early).0.strongest_lag, -8);

        // 0.2 ms is below the resolution: one sample late is not a delay.
        let one: Vec<f32> = [vec![0.0; 1], mid[..mid.len() - 1].to_vec()].concat();
        let (channels, class) = class_of(&mid, &one);
        assert_eq!(channels.strongest_lag, 1);
        assert_eq!(
            class,
            Class::Width(Width::Wide),
            "white noise is wide one sample apart"
        );

        assert_eq!(class_of(&mid, &mid).1, Class::Width(Width::Mono));
    }

    #[test]
    fn the_check_repairs_only_a_channel_inverted_at_lag_0() {
        let mid = noise(16_000, 1);
        let other = noise(16_000, 2);
        let right: Vec<f32> = mid.iter().zip(&other).map(|(m, o)| m + 0.5 * o).collect();
        let negated = |samples: &[f32]| samples.iter().map(|x| -x).collect::<Vec<f32>>();
        let check = |left: &[f32], right: &[f32]| inverted(&channels(&stereo(left, right)));

        assert!(!check(&mid, &right), "an ordinary narrow source");
        assert!(!check(&mid, &mid), "a mono source");
        assert!(check(&mid, &negated(&right)), "the right channel inverted");
        assert!(check(&negated(&mid), &right), "the left channel inverted");
        assert!(!check(&mid, &vec![0.0; mid.len()]), "a silent channel");
        // Quieter by more than 20 dB: one channel, not an inverted pair.
        let faint: Vec<f32> = right.iter().map(|x| -0.05 * x).collect();
        assert!(!check(&mid, &faint));
        // A wide source correlated at about -0.3: the side louder than the
        // mid, but not an inverted channel by the rule.
        let wide: Vec<f32> = mid
            .iter()
            .zip(&other)
            .map(|(m, o)| -0.3 * m + 0.95 * o)
            .collect();
        let wide_channels = channels(&stereo(&mid, &wide));
        assert!(
            (-0.5..-0.1).contains(&wide_channels.correlation),
            "{wide_channels:?}"
        );
        assert!(!inverted(&wide_channels));
        // Inverted and 5 ms late: correlated most strongly at 40 samples,
        // which reversing the polarity at lag 0 would not repair.
        let late: Vec<f32> = [vec![0.0; 40], negated(&mid[..mid.len() - 40])].concat();
        let late_channels = channels(&stereo(&mid, &late));
        assert_eq!(late_channels.strongest_lag, 40);
        assert!(!inverted(&late_channels));
        // At lag 0 below -0.5 too (about -0.55), but strongest at 40
        // samples (about -0.83), like session 42's one 5 ms render the
        // census counted inverted: only the lag condition leaves it.
        let echoed: Vec<f32> = mid
            .iter()
            .zip(&late)
            .map(|(m, l)| -0.6 * m + 0.9 * l)
            .collect();
        let echoed_channels = channels(&stereo(&mid, &echoed));
        assert!(echoed_channels.correlation <= -0.5, "{echoed_channels:?}");
        assert_eq!(echoed_channels.strongest_lag, 40);
        assert!(!inverted(&echoed_channels));
    }

    #[test]
    fn the_reversed_downmix_of_an_inverted_channel_is_the_original_downmix() {
        let left = [0.3, -0.1, 0.25];
        let right = [0.1, 0.2, -0.05];
        let inverted: Vec<f32> = right.iter().map(|x| -x).collect();

        let repaired = reversed_downmix(&stereo(&left, &inverted));

        for ((a, l), r) in repaired.iter().zip(&left).zip(&right) {
            assert!((a - (l + r) * std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-7);
        }
    }

    #[test]
    fn windows_find_a_channel_inverted_partway() {
        let rate = 8_000;
        let mid = noise(75 * rate, 3);
        // Inverted from 30 s: windows 0-30 s, 30-60 s and 60-75 s.
        let right: Vec<f32> = mid
            .iter()
            .enumerate()
            .map(|(index, x)| if index < 30 * rate { *x } else { -x })
            .collect();

        let windows = window_correlations(&stereo(&mid, &right), rate as u32);

        assert_eq!(windows.len(), 3);
        assert!(windows[0] > 0.99, "{windows:?}");
        assert!(windows[1] < -0.99 && windows[2] < -0.99, "{windows:?}");
        // The whole file is correlated at about (30 - 45) / 75 = -0.2, so
        // the check leaves it.
        assert!(!inverted(&channels(&stereo(&mid, &right))));
        // A last window shorter than 10 s is dropped.
        let short = stereo(&mid[..65 * rate], &right[..65 * rate]);
        assert_eq!(window_correlations(&short, rate as u32).len(), 2);
    }

    #[test]
    fn the_census_draws_an_equal_share_of_each_kind() {
        let paths: Vec<String> = (0..30)
            .map(|index| format!("__full_scene/release-{index}/track.mp3"))
            .chain((0..5).map(|index| format!("__youtube_archivists/channel/{index}.m4a")))
            .chain((0..30).map(|index| format!("vinyl/{index}.mp3")))
            .collect();
        let paths: Vec<&str> = paths.iter().map(String::as_str).collect();

        let drawn = draw_by_kind(&paths, 2026, 10);

        let of = |kind: Kind| drawn.iter().filter(|(k, _)| *k == kind).count();
        assert_eq!(of(Kind::SceneRelease), 4);
        assert_eq!(of(Kind::YoutubeChannel), 3);
        assert_eq!(of(Kind::Elsewhere), 3);
        assert!(drawn.iter().all(|(kind, path)| Kind::of(path) == *kind));
        assert_eq!(drawn, draw_by_kind(&paths, 2026, 10));
        assert_ne!(drawn, draw_by_kind(&paths, 2027, 10));
        // A kind with fewer files than its share gives them all.
        assert_eq!(
            draw_by_kind(&paths, 2026, 30)
                .iter()
                .filter(|(kind, _)| *kind == Kind::YoutubeChannel)
                .count(),
            5
        );
    }

    #[test]
    fn the_census_leaves_out_listed_files_and_folders() {
        let left_out = LeftOut::parse(
            "# 0065's lists\nEM_1999.mp3\nva-mixed_cd-2000/\n\n__full_scene/a/cd2/\n",
        );

        assert!(left_out.contains("EM_1999.mp3"));
        assert!(left_out.contains("va-mixed_cd-2000/01-track.mp3"));
        assert!(left_out.contains("__full_scene/a/cd2/03.mp3"));
        assert!(!left_out.contains("__full_scene/a/cd1/03.mp3"));
        assert!(!left_out.contains("sety/EM_1999.mp3"));
        assert!(!left_out.contains("# 0065's lists"));
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

    /// Writes 8 s of two pink noises mixed into two channels by `pan`
    /// (FFmpeg's channel gains), or into one with `channels` 1.
    fn generated(dir: &Path, name: &str, pan: &str, channels: u32) -> Option<PathBuf> {
        let file = dir.join(name);
        let generated = Command::new("ffmpeg")
            .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
            .arg("anoisesrc=d=8:c=pink:r=44100:a=0.3:seed=7")
            .args(["-f", "lavfi", "-i"])
            .arg("anoisesrc=d=8:c=pink:r=44100:a=0.3:seed=8")
            .arg("-filter_complex")
            .arg(format!("[0:a][1:a]amerge=inputs=2,{pan}"))
            .args(["-ac", &channels.to_string()])
            .arg(&file)
            .status()
            .is_ok_and(|status| status.success());
        generated.then_some(file)
    }

    #[test]
    fn the_check_leaves_ordinary_files_to_the_core_decoder_and_repairs_an_inverted_one() {
        let dir = std::env::temp_dir().join(format!("gunfinger-polarity-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let narrow = "pan=stereo|c0=c0|c1=0.8*c0+0.3*c1";
        let (Some(ordinary), Some(flipped), Some(mono)) = (
            generated(&dir, "narrow.wav", narrow, 2),
            generated(
                &dir,
                "inverted.wav",
                "pan=stereo|c0=c0|c1=-0.8*c0-0.3*c1",
                2,
            ),
            generated(&dir, "mono.wav", "pan=mono|c0=c0", 1),
        ) else {
            eprintln!("skipped: ffmpeg cannot generate audio here");
            return;
        };
        let profile = Profile::CURRENT;
        let core = |file: &Path| {
            decode(file, profile.sample_rate, Excerpt::default())
                .unwrap()
                .samples
        };
        let peaks = |samples: &[f32]| extract_signal_peaks(Signal::Mono(samples), &profile);

        for file in [&ordinary, &mono] {
            let (samples, checked) = decode_repairing(file, profile.sample_rate).unwrap();
            assert_eq!(checked, Checked::Unchanged, "{}", file.display());
            assert_eq!(samples, core(file), "{}", file.display());
        }

        let (repaired, checked) = decode_repairing(&flipped, profile.sample_rate).unwrap();
        assert_eq!(checked, Checked::Repaired);
        let clean = peaks(&core(&ordinary));
        let shared_with_clean = |found: &[gunfinger_core::peaks::Peak]| {
            found.iter().filter(|peak| clean.contains(peak)).count() as f64 / clean.len() as f64
        };
        let after = shared_with_clean(&peaks(&repaired));
        let before = shared_with_clean(&peaks(&core(&flipped)));
        assert!(clean.len() > 200, "{}", clean.len());
        assert!(
            after >= 0.9,
            "{after} of the clean file's peaks after the repair"
        );
        assert!(
            before < 0.5,
            "{before} of them in today's record of the inverted file"
        );
        // Off, the store's decoding is today's for the inverted file too.
        let QueryAudio::Mono(today) = FrontEnd::Downmix
            .decode_query(&flipped, profile.sample_rate)
            .unwrap()
        else {
            panic!("the downmix front end decodes one channel");
        };
        assert_eq!(today, core(&flipped));
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

//! Robustness: the sweep's excerpts under what clubs, DJs and broadcasts do
//! to a record (EQ, noise, low bitrates, blends, speech, skips, pitch rides,
//! wow, broadcast processing, speeds outside the ladder, key lock). Every condition runs on the same
//! excerpts, so each can be compared with the untransformed control.
//!
//! The index and the excerpts are the sweep's (same seed): held-out clusters
//! stay out of the index, and held-out excerpts measure the null.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::padding::Padding;
use crate::render::{Encoding, Playback, RENDER_RATE, encode, limited, render_samples, rms};
use crate::rng::Rng;
use crate::sweep::{Draw, EXCERPT_SECONDS, Plan};
use crate::tempo::{ENVELOPE_RATE, beat_period, best_lag, onset_envelope};

/// Excerpts per condition and speed: the sweep's first indexed and
/// held-out draws.
const INDEXED: usize = 40;
const HELD_OUT: usize = 10;
/// Speeds for conditions that do not set their own: one below and one above
/// native speed, both between rungs.
const BASE_SPEEDS: [f64; 2] = [0.95, 1.03];
/// Seconds of source skipped by the needle at mid-excerpt.
const SKIP_SECONDS: f64 = 2.0;
/// The pitch ride: speed rises by this much over the excerpt.
const RAMP: f64 = 0.02;
/// Wow: the speed swings by this much either way once per revolution. A
/// worn deck's; a well-kept Technics 1200 stays near 0.01%.
const WOW_DEPTH: f64 = 0.002;
/// FM broadcast processing: slow gain riding, fast compression, then a hard
/// limiter.
const BROADCAST: &str = "acompressor=threshold=0.05:ratio=6:attack=20:release=250:makeup=4,acompressor=threshold=0.2:ratio=20:attack=0.5:release=40:makeup=2,alimiter=limit=0.7:attack=0.5:release=20:level=false";
/// A beatmatched partner plays within this share of its native speed.
const PARTNER_RANGE: f64 = 0.1;
/// Ratios between a found beat period and the true one.
const METRICAL_FACTORS: [f64; 7] = [1.0, 2.0, 0.5, 1.5, 2.0 / 3.0, 4.0 / 3.0, 0.75];
/// Combined damage: wow at 33 rpm, a beatmatched partner this far below,
/// broadcast processing and a low-bitrate stream.
const COMBINED_PARTNER_DB: i32 = -6;
const COMBINED_ENCODING: Encoding = Encoding::Aac(64);
/// Speech replaces the music for this span in `Insert`, and is laid over it
/// in `VoiceOver`.
const INSERT_SPAN: (f64, f64) = (11.0, 19.0);
const VOICE_OVER_SPAN: (f64, f64) = (10.0, 20.0);
const SPEECH: &str = "You're listening to the essential mix. Lots more to come in the next hour, so stay locked, and don't touch that dial. Big shout to everyone out there tonight.";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Condition {
    Control,
    BassBoost,
    BassCut,
    LowPass2k,
    LowPass800,
    HighPass1k,
    Telephone,
    Echo,
    Clipping,
    PinkNoise {
        snr_db: i32,
    },
    Codec(Encoding),
    /// A second library track mixed in for the whole excerpt, at this level
    /// relative to the first.
    Blend {
        partner_db: i32,
    },
    VoiceOver,
    Insert,
    Skip,
    Ramp,
    /// Speed modulated once per revolution, at this many revolutions per
    /// second: 0.55 at 33 rpm, 0.75 at 45 rpm.
    Wow(f64),
    Broadcast,
    /// A second library track at the first one's tempo, its beats on the
    /// first one's, at this level relative to it.
    Beatmatched {
        partner_db: i32,
    },
    /// Wow, a beatmatched partner, broadcast processing and a low bitrate
    /// at once.
    Combined,
    Speed(f64),
    KeyLock(f64),
}

const CONDITIONS: [Condition; 38] = [
    Condition::Control,
    Condition::BassBoost,
    Condition::BassCut,
    Condition::LowPass2k,
    Condition::LowPass800,
    Condition::HighPass1k,
    Condition::Telephone,
    Condition::Echo,
    Condition::Clipping,
    Condition::PinkNoise { snr_db: 10 },
    Condition::PinkNoise { snr_db: 0 },
    Condition::Codec(Encoding::Mp3(64)),
    Condition::Codec(Encoding::Mp3(32)),
    Condition::Codec(Encoding::Aac(48)),
    Condition::Codec(Encoding::Opus(24)),
    Condition::Blend { partner_db: -6 },
    Condition::Blend { partner_db: 0 },
    Condition::VoiceOver,
    Condition::Insert,
    Condition::Skip,
    Condition::Ramp,
    Condition::Wow(0.55),
    Condition::Wow(0.75),
    Condition::Broadcast,
    Condition::Beatmatched { partner_db: -6 },
    Condition::Beatmatched { partner_db: 0 },
    Condition::Combined,
    Condition::Speed(0.90),
    Condition::Speed(0.88),
    Condition::Speed(0.84),
    Condition::Speed(1.10),
    Condition::Speed(1.12),
    Condition::Speed(1.16),
    Condition::KeyLock(0.98),
    Condition::KeyLock(1.02),
    Condition::KeyLock(0.95),
    Condition::KeyLock(1.05),
    Condition::KeyLock(1.08),
];

/// Speeds just past the ladder's ends, run only when named in `--only`.
const LADDER_EDGE: [Condition; 10] = [
    Condition::Speed(0.918),
    Condition::Speed(0.916),
    Condition::Speed(0.914),
    Condition::Speed(0.912),
    Condition::Speed(0.91),
    Condition::Speed(1.082),
    Condition::Speed(1.084),
    Condition::Speed(1.086),
    Condition::Speed(1.088),
    Condition::Speed(1.09),
];

impl Condition {
    fn name(self) -> String {
        match self {
            Condition::Control => "control".into(),
            Condition::BassBoost => "bass-boost-12db".into(),
            Condition::BassCut => "bass-cut-250hz".into(),
            Condition::LowPass2k => "low-pass-2khz".into(),
            Condition::LowPass800 => "low-pass-800hz".into(),
            Condition::HighPass1k => "high-pass-1khz".into(),
            Condition::Telephone => "telephone".into(),
            Condition::Echo => "echo".into(),
            Condition::Clipping => "clipping-12db".into(),
            Condition::PinkNoise { snr_db } => format!("pink-noise-snr-{snr_db}db"),
            Condition::Codec(encoding) => match encoding {
                Encoding::Mp3(kbps) => format!("mp3-{kbps}k"),
                Encoding::Aac(kbps) => format!("aac-{kbps}k"),
                Encoding::Opus(kbps) => format!("opus-{kbps}k"),
                Encoding::Lossless => "wav".into(),
            },
            Condition::Blend { partner_db } => format!("blend-{partner_db}db"),
            Condition::VoiceOver => "voice-over".into(),
            Condition::Insert => "speech-insert-8s".into(),
            Condition::Skip => "needle-skip-2s".into(),
            Condition::Ramp => "pitch-ride-2pct".into(),
            Condition::Wow(hz) => format!("wow-{hz:.2}hz"),
            Condition::Broadcast => "broadcast".into(),
            Condition::Beatmatched { partner_db } => format!("beatmatched-{partner_db}db"),
            Condition::Combined => "combined".into(),
            Condition::Speed(speed) => {
                let percent = (speed - 1.0) * 100.0;
                if (percent - percent.round()).abs() < 1e-6 {
                    format!("speed{percent:+.0}pct")
                } else {
                    format!("speed{percent:+.1}pct")
                }
            }
            Condition::KeyLock(tempo) => format!("key-lock{:+.0}pct", (tempo - 1.0) * 100.0),
        }
    }

    fn speeds(self) -> Vec<f64> {
        match self {
            Condition::Speed(speed) | Condition::KeyLock(speed) => vec![speed],
            _ => BASE_SPEEDS.to_vec(),
        }
    }

    fn playback(self, speed: f64) -> Playback {
        match self {
            Condition::KeyLock(tempo) => Playback::KeyLocked(tempo),
            _ => Playback::Turntable(speed),
        }
    }

    /// Two-pole filters are applied twice for a steep, DJ-mixer-like slope.
    fn filter(self) -> Option<&'static str> {
        match self {
            Condition::BassBoost => Some("bass=g=12:f=100"),
            Condition::BassCut => Some("highpass=f=250,highpass=f=250"),
            Condition::LowPass2k => Some("lowpass=f=2000,lowpass=f=2000"),
            Condition::LowPass800 => Some("lowpass=f=800,lowpass=f=800"),
            Condition::HighPass1k => Some("highpass=f=1000,highpass=f=1000"),
            Condition::Telephone => {
                Some("highpass=f=300,highpass=f=300,lowpass=f=3400,lowpass=f=3400")
            }
            Condition::Echo => Some("aecho=0.8:0.6:120|250:0.4|0.25"),
            Condition::Broadcast => Some(BROADCAST),
            _ => None,
        }
    }

    /// Applied to the finished query, after any blend.
    fn final_filter(self) -> Option<&'static str> {
        match self {
            Condition::Combined => Some(BROADCAST),
            _ => None,
        }
    }

    fn encoding(self) -> Encoding {
        match self {
            Condition::Codec(encoding) => encoding,
            Condition::Combined => COMBINED_ENCODING,
            _ => Encoding::Mp3(128),
        }
    }

    /// The second track of a blend counts as correct too.
    fn has_partner(self) -> bool {
        matches!(
            self,
            Condition::Blend { .. } | Condition::Beatmatched { .. } | Condition::Combined
        )
    }
}

/// One rendered query: an excerpt under a condition at a speed.
struct Job<'a> {
    condition: Condition,
    speed: f64,
    number: usize,
    draw: &'a Draw,
    partner: &'a Draw,
    path: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct RobustReport {
    pub seed: u64,
    /// The rungs searched: `turntable`, `key-lock` or `both`.
    #[serde(default)]
    pub ladder: String,
    #[serde(default)]
    pub synthetic_copies: usize,
    /// Assets of a second library in the index.
    #[serde(default)]
    pub second_library_assets: usize,
    /// Assets in the index, synthetic copies and a second library included.
    #[serde(default)]
    pub indexed_assets: usize,
    pub rows: Vec<Row>,
    pub queries: Vec<QueryResult>,
}

#[derive(Serialize, Deserialize)]
pub struct QueryResult {
    pub condition: String,
    pub speed: f64,
    pub number: usize,
    pub asset: String,
    pub held_out: bool,
    /// The strongest detection of the excerpt's own cluster.
    pub best_hits: u32,
    pub best_windows: u32,
    pub best_speed: f64,
    pub confident: bool,
    pub possible: bool,
    /// For blends: the second track was found confidently.
    pub partner_confident: bool,
    /// Detections of anything else at the confident and possible levels.
    pub wrong_confident: usize,
    pub wrong_possible: usize,
    pub strongest_wrong_hits: u32,
    /// Wall time of the search on one thread, other queries running
    /// alongside.
    #[serde(default)]
    pub search_seconds: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub condition: String,
    pub speed: f64,
    pub indexed: usize,
    pub confident: usize,
    pub possible_or_better: usize,
    pub partner_confident: usize,
    pub wrong_confident: usize,
    pub wrong_possible: usize,
    pub strongest_wrong_hits: u32,
    /// Median over indexed excerpts of the best hits against the control's
    /// best hits for the same excerpt (mean of the two base speeds).
    pub median_hit_retention: f64,
    #[serde(default)]
    pub median_search_seconds: f64,
}

/// What to run: the draw's seed, the conditions (all when empty) and the
/// rungs searched.
pub struct Options<'a> {
    pub seed: u64,
    /// Where the seed's sweep panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    pub only: &'a [String],
    pub ladder_name: &'a str,
    pub ladder: &'a [Rung],
    /// What is added to the index to measure a larger library.
    pub padding: &'a Padding,
    pub matching: &'a Matching,
    pub jobs: usize,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<RobustReport, String> {
    let Options {
        seed,
        panels,
        only,
        ladder_name,
        ladder,
        padding,
        matching,
        jobs,
    } = *options;
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let plan = Plan::for_seed(&records, clusters, seed, panels)?;
    let indexed: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !plan.held_out.contains(&record.header.source.path))
        .cloned()
        .collect();
    let index = matching.index(padding.index(&indexed, &profile, &plan.held_out)?);
    drop(indexed);
    drop(records);
    let draws: Vec<&Draw> = plan
        .draws
        .iter()
        .filter(|draw| !draw.held_out)
        .take(INDEXED)
        .chain(
            plan.draws
                .iter()
                .filter(|draw| draw.held_out)
                .take(HELD_OUT),
        )
        .collect();

    let dir = work.join("robust").join(format!("seed-{seed}"));
    let speech = speech_samples(&dir)?;
    let conditions: Vec<Condition> = CONDITIONS
        .into_iter()
        .filter(|condition| {
            *condition == Condition::Control || only.is_empty() || only.contains(&condition.name())
        })
        .chain(
            LADDER_EDGE
                .into_iter()
                .filter(|condition| only.contains(&condition.name())),
        )
        .collect();
    let mut work_items = Vec::new();
    for &condition in &conditions {
        for speed in condition.speeds() {
            for (number, draw) in draws.iter().enumerate() {
                let path = dir.join(condition.name()).join(format!(
                    "{number:03}-{speed:.3}.{}",
                    condition.encoding().extension()
                ));
                work_items.push(Job {
                    condition,
                    speed,
                    number,
                    draw,
                    partner: draws[(number + 1) % INDEXED],
                    path,
                });
            }
        }
    }

    let results: Vec<Result<QueryResult, String>> = map_in_order(&work_items, jobs, |job| {
        if !job.path.exists() {
            render_query(job, library, &speech, seed)?;
        }
        let audio = decode(&job.path, profile.sample_rate, Excerpt::default())
            .map_err(|error| error.to_string())?;
        let started = Instant::now();
        let detections = matching.search(&index, &audio.samples, &profile, ladder, 1);
        let search_seconds = started.elapsed().as_secs_f64();
        let own = clusters.cluster_of(&job.draw.asset);
        let partner = if job.condition.has_partner() {
            clusters.cluster_of(&job.partner.asset)
        } else {
            BTreeSet::new()
        };
        let mut result = QueryResult {
            condition: job.condition.name(),
            speed: job.speed,
            number: job.number,
            asset: job.draw.asset.clone(),
            held_out: job.draw.held_out,
            best_hits: 0,
            best_windows: 0,
            best_speed: 0.0,
            confident: false,
            possible: false,
            partner_confident: false,
            wrong_confident: 0,
            wrong_possible: 0,
            strongest_wrong_hits: 0,
            search_seconds,
        };
        for detection in &detections {
            let asset = &index.asset(detection.asset).path;
            let level = detection.evidence.confidence();
            if own.contains(asset) {
                if detection.evidence.hits > result.best_hits {
                    result.best_hits = detection.evidence.hits;
                    result.best_windows = detection.evidence.windows;
                    result.best_speed = detection.speed.0;
                }
                result.confident |= level == Confidence::Confident;
                result.possible |= level >= Confidence::Possible;
            } else if partner.contains(asset) {
                result.partner_confident |= level == Confidence::Confident;
            } else {
                result.strongest_wrong_hits =
                    result.strongest_wrong_hits.max(detection.evidence.hits);
                match level {
                    Confidence::Confident => result.wrong_confident += 1,
                    Confidence::Possible => result.wrong_possible += 1,
                    Confidence::Weak => {}
                }
            }
        }
        Ok(result)
    });
    let queries = results
        .into_iter()
        .collect::<Result<Vec<QueryResult>, String>>()?;
    let rows = summarise(&conditions, &queries);
    Ok(RobustReport {
        seed,
        ladder: ladder_name.to_owned(),
        synthetic_copies: padding.copies,
        second_library_assets: padding.second_assets(),
        indexed_assets: index.assets().len(),
        rows,
        queries,
    })
}

fn render_query(job: &Job, library: &Library, speech: &[f32], seed: u64) -> Result<(), String> {
    let source = library.root.join(&job.draw.asset);
    let start = job.draw.start_seconds;
    let condition = job.condition;
    let samples = match condition {
        Condition::Ramp => {
            let needed = EXCERPT_SECONDS * (job.speed + RAMP / 2.0) + 1.0;
            let native = render_samples(&source, start, needed, Playback::Turntable(1.0), None)?;
            ramp(&native, job.speed, job.speed + RAMP)
        }
        Condition::Wow(hz) => wow(&source, start, job.speed, hz)?,
        Condition::Combined => wow(&source, start, job.speed, 0.55)?,
        Condition::Skip => {
            let longer = render_samples(
                &source,
                start,
                EXCERPT_SECONDS + SKIP_SECONDS,
                condition.playback(job.speed),
                None,
            )?;
            skip(&longer)
        }
        _ => render_samples(
            &source,
            start,
            EXCERPT_SECONDS,
            condition.playback(job.speed),
            condition.filter(),
        )?,
    };
    let samples = match condition {
        Condition::PinkNoise { snr_db } => {
            let mut rng = Rng::new(seed ^ ((job.number as u64) << 8));
            with_noise(&samples, f64::from(snr_db), &mut rng)
        }
        Condition::Blend { partner_db } => {
            let partner = render_samples(
                &library.root.join(&job.partner.asset),
                job.partner.start_seconds,
                EXCERPT_SECONDS,
                Playback::Turntable(job.speed),
                None,
            )?;
            mixed(&samples, &partner, f64::from(partner_db))
        }
        Condition::Beatmatched { partner_db } => {
            let partner = beatmatched_partner(&samples, job, library)?;
            mixed(&samples, &partner, f64::from(partner_db))
        }
        Condition::Combined => {
            let partner = beatmatched_partner(&samples, job, library)?;
            mixed(&samples, &partner, f64::from(COMBINED_PARTNER_DB))
        }
        Condition::VoiceOver => laid_over(&samples, speech, VOICE_OVER_SPAN),
        Condition::Insert => replaced(&samples, speech, INSERT_SPAN),
        Condition::Clipping => clipped(&samples),
        _ => samples,
    };
    if let Some(dir) = job.path.parent() {
        fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    }
    encode(
        &limited(samples),
        condition.final_filter(),
        condition.encoding(),
        &job.path,
    )
}

/// The excerpt played at `speed` with wow at `hz`: output time `t` holds
/// source time `speed * (t + d * (1 - cos(2 pi hz t)) / (2 pi hz))`, the
/// integral of a speed `speed * (1 + d * sin(2 pi hz t))`.
fn wow(source: &Path, start: f64, speed: f64, hz: f64) -> Result<Vec<f32>, String> {
    let needed = EXCERPT_SECONDS * speed * (1.0 + WOW_DEPTH) + 1.0;
    let native = render_samples(source, start, needed, Playback::Turntable(1.0), None)?;
    let angular = 2.0 * std::f64::consts::PI * hz;
    Ok(resampled(&native, |t| {
        speed * (t + WOW_DEPTH * (1.0 - (angular * t).cos()) / angular)
    }))
}

/// The partner track at the excerpt's tempo, its beats on the excerpt's,
/// as a DJ beatmatches the next record. Tempos come from onset envelopes.
/// When a tempo cannot be found, or matching would take the partner more
/// than `PARTNER_RANGE` from its native speed, it plays at the excerpt's
/// speed unmatched, as in `Blend`; stderr says so.
fn beatmatched_partner(samples: &[f32], job: &Job, library: &Library) -> Result<Vec<f32>, String> {
    let source = library.root.join(&job.partner.asset);
    let start = job.partner.start_seconds;
    let render =
        |seconds, speed| render_samples(&source, start, seconds, Playback::Turntable(speed), None);
    let plain = render(EXCERPT_SECONDS, job.speed)?;
    let ours = onset_envelope(samples, RENDER_RATE);
    let periods = beat_period(&ours).zip(beat_period(&onset_envelope(&plain, RENDER_RATE)));
    // A period found at a half, two thirds or three quarters of the beat
    // (or the inverse) is still the right tempo, up to that factor.
    let partner_speed = periods.and_then(|(ours, theirs)| {
        METRICAL_FACTORS
            .iter()
            .map(|factor| job.speed * theirs / ours * factor)
            .filter(|speed| (speed - 1.0).abs() <= PARTNER_RANGE)
            .min_by(|a, b| (a - 1.0).abs().total_cmp(&(b - 1.0).abs()))
    });
    let (Some(partner_speed), Some((period, _))) = (partner_speed, periods) else {
        let tempos = periods.map_or_else(
            || String::from("no tempo found"),
            |(ours, theirs)| format!("{:.1} and {:.1} bpm", 60.0 / ours, 60.0 / theirs),
        );
        eprintln!(
            "{} {:03}: partner not beatmatched ({tempos})",
            job.condition.name(),
            job.number
        );
        return Ok(plain);
    };
    let longer = render(EXCERPT_SECONDS + period + 0.5, partner_speed)?;
    let lag = best_lag(
        &ours,
        &onset_envelope(&longer, RENDER_RATE),
        (period * ENVELOPE_RATE).ceil() as usize,
    );
    let skip = seconds_to_index(lag as f64 / ENVELOPE_RATE).min(longer.len());
    Ok(longer[skip..].to_vec())
}

/// Speech from macOS `say`, once per seed directory.
fn speech_samples(dir: &Path) -> Result<Vec<f32>, String> {
    fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    let path = dir.join("speech.aiff");
    if !path.exists() {
        let status = Command::new("say")
            .arg("-o")
            .arg(&path)
            .arg(SPEECH)
            .stdin(Stdio::null())
            .status()
            .map_err(|error| {
                format!("cannot run `say` (macOS) for the speech conditions: {error}")
            })?;
        if !status.success() {
            return Err(format!("`say` failed ({status})"));
        }
    }
    render_samples(&path, 0.0, 20.0, Playback::Turntable(1.0), None)
}

fn seconds_to_index(seconds: f64) -> usize {
    (seconds * f64::from(RENDER_RATE)) as usize
}

/// Pink noise (Paul Kellet's economy filter over white noise) at `snr_db`
/// below the music's RMS level.
fn with_noise(samples: &[f32], snr_db: f64, rng: &mut Rng) -> Vec<f32> {
    let (mut b0, mut b1, mut b2) = (0.0, 0.0, 0.0);
    let noise: Vec<f64> = (0..samples.len())
        .map(|_| {
            let white = 2.0 * rng.unit() - 1.0;
            b0 = 0.997_65 * b0 + white * 0.099_046;
            b1 = 0.963 * b1 + white * 0.296_516_4;
            b2 = 0.57 * b2 + white * 1.052_691_3;
            b0 + b1 + b2 + white * 0.1848
        })
        .collect();
    let noise_rms = (noise.iter().map(|x| x * x).sum::<f64>() / noise.len().max(1) as f64).sqrt();
    let gain = rms(samples) / 10_f64.powf(snr_db / 20.0) / noise_rms.max(1e-12);
    samples
        .iter()
        .zip(&noise)
        .map(|(&x, &n)| (f64::from(x) + gain * n) as f32)
        .collect()
}

/// `partner` mixed under `samples` at `partner_db` relative RMS level.
fn mixed(samples: &[f32], partner: &[f32], partner_db: f64) -> Vec<f32> {
    let gain = rms(samples) / rms(partner).max(1e-12) * 10_f64.powf(partner_db / 20.0);
    samples
        .iter()
        .enumerate()
        .map(|(index, &x)| {
            let other = partner.get(index).copied().unwrap_or(0.0);
            (f64::from(x) + gain * f64::from(other)) as f32
        })
        .collect()
}

/// Speech at the music's RMS level laid over `span` (seconds).
fn laid_over(samples: &[f32], speech: &[f32], span: (f64, f64)) -> Vec<f32> {
    let gain = rms(samples) / rms(speech).max(1e-12);
    let (from, to) = (seconds_to_index(span.0), seconds_to_index(span.1));
    let mut out = samples.to_vec();
    for (offset, slot) in out.iter_mut().enumerate().take(to).skip(from) {
        if let Some(&voice) = speech.get(offset - from) {
            *slot += (gain * f64::from(voice)) as f32;
        }
    }
    out
}

/// The music in `span` replaced by speech, as when a broadcast cuts away.
fn replaced(samples: &[f32], speech: &[f32], span: (f64, f64)) -> Vec<f32> {
    let gain = rms(samples) / rms(speech).max(1e-12);
    let (from, to) = (seconds_to_index(span.0), seconds_to_index(span.1));
    let mut out = samples.to_vec();
    for (offset, slot) in out.iter_mut().enumerate().take(to).skip(from) {
        *slot = speech
            .get(offset - from)
            .map_or(0.0, |&voice| (gain * f64::from(voice)) as f32);
    }
    out
}

/// The needle jumps forward `SKIP_SECONDS` of output at mid-excerpt.
fn skip(longer: &[f32]) -> Vec<f32> {
    let middle = seconds_to_index(EXCERPT_SECONDS / 2.0);
    let jump = seconds_to_index(SKIP_SECONDS);
    longer[..middle]
        .iter()
        .chain(&longer[(middle + jump).min(longer.len())..])
        .copied()
        .collect()
}

/// Native-speed audio played with a speed rising linearly from `from` to
/// `to` over the excerpt: output time `t` holds source time
/// `from * t + (to - from) * t^2 / (2 T)`.
fn ramp(native: &[f32], from: f64, to: f64) -> Vec<f32> {
    resampled(native, |t| {
        from * t + (to - from) * t * t / (2.0 * EXCERPT_SECONDS)
    })
}

/// An excerpt of native-speed audio whose output time `t` (seconds) holds
/// source time `source_time(t)`. Linear interpolation is enough here: the
/// analysis band ends at 4 kHz, far below the 44.1 kHz rate.
fn resampled(native: &[f32], source_time: impl Fn(f64) -> f64) -> Vec<f32> {
    let rate = f64::from(RENDER_RATE);
    (0..seconds_to_index(EXCERPT_SECONDS))
        .map(|index| {
            let source = source_time(index as f64 / rate) * rate;
            let before = source.floor() as usize;
            let fraction = (source - source.floor()) as f32;
            match (native.get(before), native.get(before + 1)) {
                (Some(&a), Some(&b)) => a + (b - a) * fraction,
                _ => 0.0,
            }
        })
        .collect()
}

/// 12 dB of hard clipping, as from an overdriven mixer.
fn clipped(samples: &[f32]) -> Vec<f32> {
    let peak = samples.iter().fold(0.0_f32, |peak, &x| peak.max(x.abs()));
    let scale = 1.0 / peak.max(1e-9);
    samples
        .iter()
        .map(|&x| (x * scale).clamp(-0.25, 0.25) * 2.0)
        .collect()
}

fn summarise(conditions: &[Condition], queries: &[QueryResult]) -> Vec<Row> {
    let control_hits = |number: usize| -> f64 {
        let hits: Vec<f64> = queries
            .iter()
            .filter(|query| query.condition == Condition::Control.name() && query.number == number)
            .map(|query| f64::from(query.best_hits))
            .collect();
        hits.iter().sum::<f64>() / hits.len().max(1) as f64
    };
    let mut rows = Vec::new();
    for condition in conditions {
        for speed in condition.speeds() {
            let here: Vec<&QueryResult> = queries
                .iter()
                .filter(|query| query.condition == condition.name() && query.speed == speed)
                .collect();
            let indexed: Vec<&&QueryResult> = here.iter().filter(|query| !query.held_out).collect();
            let mut retention: Vec<f64> = indexed
                .iter()
                .map(|query| f64::from(query.best_hits) / control_hits(query.number).max(1.0))
                .collect();
            retention.sort_by(f64::total_cmp);
            let mut seconds: Vec<f64> = here.iter().map(|query| query.search_seconds).collect();
            seconds.sort_by(f64::total_cmp);
            rows.push(Row {
                condition: condition.name(),
                speed,
                indexed: indexed.len(),
                confident: indexed.iter().filter(|query| query.confident).count(),
                possible_or_better: indexed.iter().filter(|query| query.possible).count(),
                partner_confident: indexed
                    .iter()
                    .filter(|query| query.partner_confident)
                    .count(),
                wrong_confident: here.iter().map(|query| query.wrong_confident).sum(),
                wrong_possible: here.iter().map(|query| query.wrong_possible).sum(),
                strongest_wrong_hits: here
                    .iter()
                    .map(|query| query.strongest_wrong_hits)
                    .max()
                    .unwrap_or(0),
                median_hit_retention: retention.get(retention.len() / 2).copied().unwrap_or(0.0),
                median_search_seconds: seconds.get(seconds.len() / 2).copied().unwrap_or(0.0),
            });
        }
    }
    rows
}

pub fn print_summary(report: &RobustReport) {
    println!(
        "robustness, seed {}, {} ladder: {INDEXED} indexed and {HELD_OUT} held-out excerpts per row",
        report.seed, report.ladder
    );
    println!(
        "{:<22} {:>6} {:>9} {:>9} {:>8} {:>6} {:>6} {:>7} {:>9} {:>7}",
        "condition",
        "speed",
        "confident",
        "possible",
        "partner",
        "wrong",
        "w.poss",
        "w.max",
        "retention",
        "search"
    );
    for row in &report.rows {
        println!(
            "{:<22} {:>+5.0}% {:>4}/{:<4} {:>4}/{:<4} {:>8} {:>6} {:>6} {:>7} {:>8.0}% {:>6.2}s",
            row.condition,
            (row.speed - 1.0) * 100.0,
            row.confident,
            row.indexed,
            row.possible_or_better,
            row.indexed,
            row.partner_confident,
            row.wrong_confident,
            row.wrong_possible,
            row.strongest_wrong_hits,
            row.median_hit_retention * 100.0,
            row.median_search_seconds
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_skip_drops_the_skipped_span() {
        let rate = f64::from(RENDER_RATE);
        let longer: Vec<f32> = (0..seconds_to_index(EXCERPT_SECONDS + SKIP_SECONDS))
            .map(|index| (index as f64 / rate) as f32)
            .collect();

        let skipped = skip(&longer);

        let middle = seconds_to_index(EXCERPT_SECONDS / 2.0);
        assert_eq!(skipped.len(), seconds_to_index(EXCERPT_SECONDS));
        assert!((f64::from(skipped[middle]) - (EXCERPT_SECONDS / 2.0 + SKIP_SECONDS)).abs() < 1e-3);
    }

    #[test]
    fn a_ramp_at_constant_speed_reads_the_source_at_that_speed() {
        let rate = f64::from(RENDER_RATE);
        let native: Vec<f32> = (0..seconds_to_index(40.0))
            .map(|index| (index as f64 / rate) as f32)
            .collect();

        let played = ramp(&native, 1.03, 1.03);

        let at_ten = seconds_to_index(10.0);
        assert!((f64::from(played[at_ten]) - 10.3).abs() < 1e-3);
    }

    #[test]
    fn noise_is_added_at_the_requested_level() {
        let tone: Vec<f32> = (0..44_100)
            .map(|index| (index as f32 * 0.05).sin() * 0.5)
            .collect();

        let noisy = with_noise(&tone, 10.0, &mut Rng::new(1));

        let noise: Vec<f32> = noisy.iter().zip(&tone).map(|(a, b)| a - b).collect();
        let snr = 20.0 * (rms(&tone) / rms(&noise)).log10();
        assert!((snr - 10.0).abs() < 0.01);
    }
}

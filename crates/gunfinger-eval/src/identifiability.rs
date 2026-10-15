//! Proposal 5's identifiability diagnostic (experiment 0112;
//! `docs/research/2026-10-14-matching-proposals.md`): over the shared
//! passage of each direction of a panel half, how closely the related
//! recording reproduces the played one in the audio, in the peaks and in
//! the pair hashes, next to the played recording's own rips, the noise
//! floor (two rips of one record are one recording, however they sound).
//!
//! For each file X (the related file, and each other file of the played
//! recording), the played file's passage is decoded and searched against
//! an index of X alone with the fitted matcher on both ladders, as `pair`
//! searches audio. X's strongest detection gives
//! - pairs: its hits, and hits per second of the passage;
//! - peaks: the peak verifier's share of X's reference peaks found in the
//!   passage, and its highest share with X shifted away (chance);
//! - audio: X rendered along the detection's line and compared with the
//!   passage: the median over frames of the correlation of the two
//!   log-power spectra across bins, after each bin's mean over the
//!   passage is taken out (a fixed EQ or transfer curve), each 10 s window
//!   at its best lag within a few frames; and the highest such value with
//!   X shifted away (chance).
//!
//! It searches no large index: the panel's queries against the whole
//! library are `shared`'s.

use std::collections::BTreeSet;
use std::time::Duration;

use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Matcher, search_with};
use gunfinger_core::spectrogram::for_each_frame;
use gunfinger_core::speed::{self, Rung, key_lock_ladder, ladder};
use gunfinger_core::store::PeakRecord;
use serde::{Deserialize, Serialize};

use crate::pair::{Records, Sources};
use crate::render::{Playback, render_samples};
use crate::shared::Roots;
use crate::verifier::{CHANCE_SHIFTS_SECONDS, Verifier};

/// Each window of the passage this long is aligned on its own...
const WINDOW_SECONDS: f64 = 10.0;
/// ...at the lag within this many frames that correlates best.
const MAX_LAG_FRAMES: usize = 4;

/// The panel as `scripts/analysis/shared_panel.py` writes it; only what
/// the diagnostic reads.
#[derive(Deserialize)]
pub struct Panel {
    pub families: Vec<Family>,
}

#[derive(Deserialize)]
pub struct Family {
    pub name: String,
    pub half: String,
    pub recordings: Vec<Vec<String>>,
    pub directions: Vec<Direction>,
}

#[derive(Deserialize)]
pub struct Direction {
    pub name: String,
    pub played: String,
    pub related: String,
    pub passage: Option<Passage>,
}

#[derive(Deserialize)]
pub struct Passage {
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub control: bool,
}

#[derive(Serialize)]
pub struct Report {
    pub half: String,
    pub directions: Vec<Measured>,
}

#[derive(Serialize)]
pub struct Measured {
    pub family: String,
    pub direction: String,
    pub played: String,
    pub passage_start_seconds: f64,
    pub passage_end_seconds: f64,
    pub control: bool,
    pub files: Vec<Compared>,
}

#[derive(Serialize)]
pub struct Compared {
    pub file: String,
    /// `related`, or `rip` for another file of the played recording.
    pub role: String,
    /// X's strongest detection in the passage, if any.
    pub found: Option<Found>,
}

#[derive(Serialize)]
pub struct Found {
    pub hits: u32,
    pub windows: u32,
    pub level: String,
    pub hits_per_second: f64,
    pub speed: f64,
    pub key_locked: bool,
    pub query_start_seconds: f64,
    pub query_end_seconds: f64,
    pub file_start_seconds: f64,
    pub file_end_seconds: f64,
    pub peaks_share: Option<f64>,
    pub peaks_chance: Option<f64>,
    pub audio_similarity: f64,
    pub audio_chance: f64,
}

pub fn run(panel: &Panel, half: &str, sources: &Sources, roots: &Roots) -> Result<Report, String> {
    let records = Records::load(sources)?;
    let mut directions = Vec::new();
    for family in panel.families.iter().filter(|family| family.half == half) {
        for direction in &family.directions {
            let Some(passage) = &direction.passage else {
                continue;
            };
            eprintln!("{}", direction.name);
            let played = family
                .recordings
                .iter()
                .find(|recording| recording.contains(&direction.played))
                .ok_or_else(|| format!("{} is in no recording of its family", direction.played))?;
            let mut files = vec![(direction.related.clone(), "related")];
            files.extend(
                played
                    .iter()
                    .filter(|file| **file != direction.played)
                    .map(|file| (file.clone(), "rip")),
            );
            let passage_audio = decode(
                &roots.path(&direction.played)?,
                Profile::CURRENT.sample_rate,
                Excerpt {
                    start: Some(Duration::from_secs_f64(passage.start_seconds)),
                    duration: Some(Duration::from_secs_f64(
                        passage.end_seconds - passage.start_seconds,
                    )),
                },
            )
            .map_err(|error| error.to_string())?;
            let compared = files
                .into_iter()
                .map(|(file, role)| {
                    let record = records.record(sources, &file)?;
                    Ok(Compared {
                        found: measure(&record, &file, &passage_audio.samples, roots)?,
                        file,
                        role: role.to_owned(),
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            directions.push(Measured {
                family: family.name.clone(),
                direction: direction.name.clone(),
                played: direction.played.clone(),
                passage_start_seconds: passage.start_seconds,
                passage_end_seconds: passage.end_seconds,
                control: passage.control,
                files: compared,
            });
        }
    }
    Ok(Report {
        half: half.to_owned(),
        directions,
    })
}

/// X's strongest detection in the passage, with its peaks and audio
/// compared.
fn measure(
    record: &PeakRecord,
    file: &str,
    passage: &[f32],
    roots: &Roots,
) -> Result<Option<Found>, String> {
    let profile = Profile::CURRENT;
    let index = Index::build(std::slice::from_ref(record)).map_err(|error| error.to_string())?;
    let both: Vec<Rung> = ladder().into_iter().chain(key_lock_ladder()).collect();
    let detections = search_with(
        &index,
        passage,
        &profile,
        &both,
        1,
        Matcher::Fitted.options(),
    );
    let Some(best) = detections
        .iter()
        .max_by_key(|detection| detection.evidence.hits)
    else {
        return Ok(None);
    };
    let verified =
        Verifier::new(std::slice::from_ref(record), None).verify(&index, best, passage, &profile);
    let seconds = passage.len() as f64 / f64::from(profile.sample_rate);
    let path = roots.path(file)?;
    let aligned = |shift: f64| aligned_audio(&path, best, shift, passage.len());
    let audio_similarity = similarity(passage, &aligned(0.0)?, &profile);
    let mut audio_chance = f64::NEG_INFINITY;
    for shift in CHANCE_SHIFTS_SECONDS {
        audio_chance = audio_chance.max(similarity(passage, &aligned(shift)?, &profile));
    }
    Ok(Some(Found {
        hits: best.evidence.hits,
        windows: best.evidence.windows,
        level: format!("{:?}", best.evidence.confidence()).to_lowercase(),
        hits_per_second: f64::from(best.evidence.hits) / seconds,
        speed: best.speed.0,
        key_locked: best.playback == speed::Playback::KeyLocked,
        query_start_seconds: best.start_seconds,
        query_end_seconds: best.end_seconds,
        file_start_seconds: best.track_start_seconds,
        file_end_seconds: best.track_end_seconds,
        peaks_share: verified.as_ref().map(|verified| verified.aligned.share),
        peaks_chance: verified.as_ref().map(|verified| {
            verified
                .shifted
                .iter()
                .map(|count| count.share)
                .fold(0.0, f64::max)
        }),
        audio_similarity,
        audio_chance,
    }))
}

/// X's audio along `detection`'s line, `shift` seconds of X later, as
/// `length` samples at the profile's rate in the passage's time: sample
/// `t` holds X at `track_start + speed × (t - start)` (plus the shift),
/// silent where X has no audio.
fn aligned_audio(
    path: &std::path::Path,
    detection: &Detection,
    shift: f64,
    length: usize,
) -> Result<Vec<f32>, String> {
    let rate = f64::from(Profile::CURRENT.sample_rate);
    let speed = detection.speed.0;
    let at_zero = detection.track_start_seconds - speed * detection.start_seconds + shift;
    let playback = match detection.playback {
        speed::Playback::KeyLocked => Playback::KeyLocked(speed),
        speed::Playback::Turntable => Playback::Turntable(speed),
    };
    let lead = if at_zero < 0.0 {
        (-at_zero / speed * rate) as usize
    } else {
        0
    };
    let seconds = length as f64 / rate;
    let resample = format!("aresample={}", Profile::CURRENT.sample_rate);
    let rendered = render_samples(path, at_zero.max(0.0), seconds, playback, Some(&resample))?;
    let mut samples = vec![0.0; lead.min(length)];
    samples.extend(rendered);
    samples.resize(length, 0.0);
    Ok(samples)
}

/// The median over frames of the correlation of `a`'s and `b`'s log-power
/// spectra across the profile's bins, each bin's mean over the excerpt
/// taken out; each window of `WINDOW_SECONDS` at its best lag within
/// `MAX_LAG_FRAMES`.
fn similarity(a: &[f32], b: &[f32], profile: &Profile) -> f64 {
    let a = centred_spectra(a, profile);
    let b = centred_spectra(b, profile);
    let frames = a.len().min(b.len());
    let window = profile.frames(WINDOW_SECONDS).round() as usize;
    let mut kept = Vec::new();
    let mut start = 0;
    while start < frames {
        let end = (start + window).min(frames);
        let best = lags()
            .map(|lag| {
                (start..end)
                    .filter_map(|frame| {
                        let other = frame.checked_add_signed(lag).filter(|&f| f < frames)?;
                        Some(correlation(&a[frame], &b[other]))
                    })
                    .collect::<Vec<f64>>()
            })
            .max_by(|x, y| median(x).total_cmp(&median(y)))
            .unwrap_or_default();
        kept.extend(best);
        start = end;
    }
    median(&kept)
}

fn lags() -> impl Iterator<Item = isize> {
    let max = MAX_LAG_FRAMES as isize;
    -max..=max
}

/// Each frame's log power over the profile's bins, less each bin's mean.
fn centred_spectra(samples: &[f32], profile: &Profile) -> Vec<Vec<f64>> {
    let mut spectra: Vec<Vec<f64>> = Vec::new();
    for_each_frame(samples, profile, |power| {
        spectra.push(
            power[profile.min_bin..profile.max_bin]
                .iter()
                .map(|&db| f64::from(db))
                .collect(),
        );
    });
    let bins = profile.max_bin - profile.min_bin;
    for bin in 0..bins {
        let mean =
            spectra.iter().map(|frame| frame[bin]).sum::<f64>() / spectra.len().max(1) as f64;
        for frame in &mut spectra {
            frame[bin] -= mean;
        }
    }
    spectra
}

/// Pearson's correlation; 0 when either side is constant.
fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len() as f64;
    let (mean_a, mean_b) = (a.iter().sum::<f64>() / n, b.iter().sum::<f64>() / n);
    let (mut ab, mut aa, mut bb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        ab += (x - mean_a) * (y - mean_b);
        aa += (x - mean_a) * (x - mean_a);
        bb += (y - mean_b) * (y - mean_b);
    }
    if aa == 0.0 || bb == 0.0 {
        0.0
    } else {
        ab / (aa * bb).sqrt()
    }
}

fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NEG_INFINITY;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len() % 2 == 1 {
        sorted[middle]
    } else {
        f64::midpoint(sorted[middle - 1], sorted[middle])
    }
}

pub fn print_summary(report: &Report) {
    for direction in &report.directions {
        println!(
            "{}{} ({:.1}-{:.1} s)",
            direction.direction,
            if direction.control { " control" } else { "" },
            direction.passage_start_seconds,
            direction.passage_end_seconds
        );
        let roles: BTreeSet<&str> = direction
            .files
            .iter()
            .map(|file| file.role.as_str())
            .collect();
        for role in roles {
            for compared in direction.files.iter().filter(|file| file.role == role) {
                match &compared.found {
                    None => println!("  {role:<8} nothing  {}", compared.file),
                    Some(found) => println!(
                        "  {role:<8} {:>5} hits {:>2} windows {:>5.2}/s  peaks {:.2} (chance {:.2})  audio {:.2} (chance {:.2})  {}",
                        found.hits,
                        found.windows,
                        found.hits_per_second,
                        found.peaks_share.unwrap_or(f64::NAN),
                        found.peaks_chance.unwrap_or(f64::NAN),
                        found.audio_similarity,
                        found.audio_chance,
                        compared.file
                    ),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_median_of_an_even_count_is_the_middle_pairs_mean() {
        assert_eq!(median(&[3.0, 1.0, 2.0]), 2.0);
        assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), 2.5);
    }

    #[test]
    fn correlation_ignores_level_and_offset() {
        let a = [1.0, 2.0, 3.0, 2.0];
        let louder: Vec<f64> = a.iter().map(|x| 3.0 * x + 10.0).collect();
        let flat = [5.0; 4];

        assert!((correlation(&a, &louder) - 1.0).abs() < 1e-12);
        assert_eq!(correlation(&a, &flat), 0.0);
    }

    fn noise(seconds: f64, seed: u64) -> Vec<f32> {
        let mut state = seed;
        (0..(seconds * 8000.0) as usize)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                ((state >> 33) as f32 / (1u64 << 31) as f32) - 0.5
            })
            .collect()
    }

    /// Noise whose level moves from frame to frame, so that its spectra
    /// differ between frames.
    fn music_like(seconds: f64, seed: u64) -> Vec<f32> {
        noise(seconds, seed)
            .iter()
            .enumerate()
            .map(|(n, &x)| x * (1.0 + (n as f32 / 700.0).sin() * (n as f32 / 3100.0).cos()))
            .collect()
    }

    #[test]
    fn the_same_audio_is_more_similar_than_other_audio() {
        let profile = Profile::CURRENT;
        let a = music_like(12.0, 1);
        let quieter: Vec<f32> = a.iter().map(|x| 0.3 * x).collect();
        let other = music_like(12.0, 2);

        let same = similarity(&a, &quieter, &profile);
        let different = similarity(&a, &other, &profile);

        assert!(same > 0.99, "{same}");
        assert!(different < 0.3, "{different}");
    }

    #[test]
    fn a_lag_of_a_few_frames_is_followed() {
        let profile = Profile::CURRENT;
        let a = music_like(12.0, 3);
        let late: Vec<f32> = std::iter::repeat_n(0.0, 3 * profile.hop)
            .chain(a.iter().copied())
            .take(a.len())
            .collect();

        assert!(similarity(&a, &late, &profile) > 0.95);
    }
}

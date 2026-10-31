//! The harmonic view of proposal 5 (brief 66): does a candidate found
//! through shared drums also match the query's sustained material at the
//! alignment the search found?
//!
//! The query and the candidate's reference, rendered onto the query's
//! clock along the detection's line, are each separated into a harmonic
//! and a percussive part by median filtering (FitzGerald 2010, §2): a
//! held note is steady across time and narrow across frequency, a drum hit
//! the reverse. Each part is turned back into audio and its peaks picked
//! by the core's picker in a smaller neighbourhood than the profile's, and
//! the reference's peaks are looked for among the query's of the same
//! part, as the peak verifier does (`verifier.rs`). The same count on the
//! whole spectrum, with the same picker, is the comparison arm: denser
//! peaks without separation. The view never searches: no offset, speed or
//! line is fitted, so a related version that the search placed on the
//! shared drums is measured where it was placed.

use std::fs;
use std::path::Path;

use gunfinger_core::peaks::{Peak, extract_peaks};
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::speed;
use serde::Serialize;

use crate::matching::Matching;
use crate::render::{Encoding, Playback, encode, limited, render_samples_as_indexed};
use crate::residual::{overlap_add, spectra};
use crate::verifier::{CHANCE_SHIFTS_SECONDS, TOLERANCE};

/// The median across time spans this many frames (400 ms at the
/// profile's hop): more than a beat at 160-180 BPM (21-23 frames), while a
/// drum hit, smeared by the 128 ms window, covers about 4-8, well under
/// the 13 a median needs to pass it.
pub const TIME_MEDIAN_FRAMES: usize = 25;
/// The median across frequency spans this many bins (133 Hz): a partial's
/// main lobe covers about 4, a snare or hats all of them.
pub const FREQUENCY_MEDIAN_BINS: usize = 17;
/// Peaks of every view are picked in this neighbourhood (half-sizes in
/// frames and bins; the profile's is 12), about twice as many as the
/// profile's: session 65's denser arm.
pub const NEIGHBOURHOOD: usize = 8;
/// Detections with fewer hits are not measured: below this they are
/// chance lines.
pub const MIN_HITS: u32 = 20;
/// Why a detection is not measured under another front end.
pub const DOWNMIX_ONLY: &str = "--harmonic analyses the downmix; leave --front-end at its default";
/// The reference is rendered from this many of its seconds before the
/// detection's span to as many after: room for the chance shifts, the
/// medians' and the picker's context.
const MARGIN_SECONDS: f64 = 8.0;

/// The peaks of each view of a stretch of audio.
pub struct Analysed {
    denser: Vec<Peak>,
    harmonic: Vec<Peak>,
    percussive: Vec<Peak>,
}

impl Analysed {
    /// `samples` at the profile's rate, separated and picked.
    pub fn of(samples: &[f32], profile: &Profile) -> Analysed {
        let picker = picker(profile);
        let parts = separate(samples, profile);
        Analysed {
            denser: extract_peaks(samples, &picker),
            harmonic: extract_peaks(&parts.harmonic, &picker),
            percussive: extract_peaks(&parts.percussive, &picker),
        }
    }
}

/// The profile with the views' smaller neighbourhood.
fn picker(profile: &Profile) -> Profile {
    Profile {
        neighbourhood_frames: NEIGHBOURHOOD,
        neighbourhood_bins: NEIGHBOURHOOD,
        ..*profile
    }
}

/// A detection measured in each view.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Measured {
    /// The whole spectrum: denser peaks, no separation.
    pub denser: View,
    pub harmonic: View,
    pub percussive: View,
}

/// The reference's peaks in the detection's span and those found in the
/// query, at the alignment and with the reference shifted.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct View {
    pub reference_peaks: u32,
    pub found: u32,
    /// `found` over `reference_peaks` (0 without reference peaks).
    pub share: f64,
    /// The mean share at `CHANCE_SHIFTS_SECONDS`, over the shifts that
    /// hold reference peaks.
    pub chance_share: f64,
    /// `share` minus `chance_share`.
    pub excess_share: f64,
}

/// The harmonic and percussive parts of `samples`, each as long.
pub struct Parts {
    pub harmonic: Vec<f32>,
    pub percussive: Vec<f32>,
}

/// FitzGerald's separation: each bin's magnitude median-filtered across
/// time gives the harmonic estimate `H`, each frame's across frequency the
/// percussive `P`; the soft mask `H^2 / (H^2 + P^2)` (his power 2) and its
/// complement weight the complex STFT, which overlap-add turns back into
/// audio.
pub fn separate(samples: &[f32], profile: &Profile) -> Parts {
    let mut harmonic = spectra(samples, profile);
    let mut percussive = harmonic.clone();
    let magnitudes: Vec<Vec<f32>> = harmonic
        .iter()
        .map(|frame| frame.iter().map(|bin| bin.norm()).collect())
        .collect();
    let steady = median_across_time(&magnitudes, TIME_MEDIAN_FRAMES);
    let broad = median_across_frequency(&magnitudes, FREQUENCY_MEDIAN_BINS);
    for (frame, (harmonic, percussive)) in harmonic.iter_mut().zip(&mut percussive).enumerate() {
        for bin in 0..harmonic.len() {
            let h = steady[frame][bin] * steady[frame][bin];
            let p = broad[frame][bin] * broad[frame][bin];
            let weight = if h + p > 0.0 { h / (h + p) } else { 0.5 };
            harmonic[bin] *= weight;
            percussive[bin] *= 1.0 - weight;
        }
    }
    Parts {
        harmonic: overlap_add(&harmonic, samples, profile),
        percussive: overlap_add(&percussive, samples, profile),
    }
}

/// Each bin's median over the `length` frames centred on each frame,
/// fewer at the edges.
fn median_across_time(magnitudes: &[Vec<f32>], length: usize) -> Vec<Vec<f32>> {
    let half = length / 2;
    let bins = magnitudes.first().map_or(0, Vec::len);
    let mut medians = vec![vec![0.0; bins]; magnitudes.len()];
    let mut window = Vec::with_capacity(length);
    for bin in 0..bins {
        for frame in 0..magnitudes.len() {
            let from = frame.saturating_sub(half);
            let to = (frame + half + 1).min(magnitudes.len());
            window.clear();
            window.extend(magnitudes[from..to].iter().map(|row| row[bin]));
            medians[frame][bin] = median(&mut window);
        }
    }
    medians
}

/// Each frame's median over the `length` bins centred on each bin, fewer
/// at the edges.
fn median_across_frequency(magnitudes: &[Vec<f32>], length: usize) -> Vec<Vec<f32>> {
    let half = length / 2;
    let mut window = Vec::with_capacity(length);
    magnitudes
        .iter()
        .map(|row| {
            (0..row.len())
                .map(|bin| {
                    let from = bin.saturating_sub(half);
                    let to = (bin + half + 1).min(row.len());
                    window.clear();
                    window.extend_from_slice(&row[from..to]);
                    median(&mut window)
                })
                .collect()
        })
        .collect()
}

/// The middle value (the upper of the two middle ones for an even count).
fn median(values: &mut [f32]) -> f32 {
    let middle = values.len() / 2;
    *values.select_nth_unstable_by(middle, f32::total_cmp).1
}

/// Where a detection's reference is rendered: from `from_seconds` of the
/// file, played at the line's `slope`, so that query time `t` holds render
/// time `lead_seconds + t - start_seconds`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placement {
    pub from_seconds: f64,
    pub slope: f64,
    pub lead_seconds: f64,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub render_seconds: f64,
}

impl Placement {
    /// The line through the detection's ends, as the peak verifier takes
    /// it (file time = `track_start + slope x (t - start)`), with
    /// `MARGIN_SECONDS` of the file on either side; `None` for an empty or
    /// reversed span.
    pub fn of(detection: &Detection) -> Option<Placement> {
        let span = detection.end_seconds - detection.start_seconds;
        let slope = (detection.track_end_seconds - detection.track_start_seconds) / span;
        if !(span > 0.0 && slope.is_finite() && slope > 0.0) {
            return None;
        }
        let from_seconds = (detection.track_start_seconds - MARGIN_SECONDS).max(0.0);
        Some(Placement {
            from_seconds,
            slope,
            lead_seconds: (detection.track_start_seconds - from_seconds) / slope,
            start_seconds: detection.start_seconds,
            end_seconds: detection.end_seconds,
            render_seconds: span
                + (detection.track_start_seconds - from_seconds + MARGIN_SECONDS) / slope,
        })
    }

    /// Render frames minus query frames, with the reference shifted
    /// `shift_seconds` of its own time.
    fn frame_offset(&self, shift_seconds: f64, profile: &Profile) -> f64 {
        profile.frames(self.lead_seconds - self.start_seconds + shift_seconds / self.slope)
    }
}

/// The detection's file rendered along `placement` and decoded at the
/// profile's rate as the query was (through a WAV in `scratch`, removed
/// once decoded). The file is decoded as indexed: the timeline of its peak
/// record, on which the detection's line lies (experiment 0127).
pub fn reference(
    source: &Path,
    placement: &Placement,
    playback: speed::Playback,
    matching: &Matching,
    scratch: &Path,
) -> Result<Vec<f32>, String> {
    let playback = match playback {
        speed::Playback::Turntable => Playback::Turntable(placement.slope),
        speed::Playback::KeyLocked => Playback::KeyLocked(placement.slope),
    };
    let samples = render_samples_as_indexed(
        source,
        placement.from_seconds,
        placement.render_seconds,
        playback,
        None,
    )?;
    fs::create_dir_all(scratch).map_err(|error| format!("{}: {error}", scratch.display()))?;
    let path = scratch.join(format!("reference-{}.wav", unique()));
    encode(&limited(samples), None, Encoding::Lossless, &path)?;
    let decoded = matching.decode_query(&path);
    for file in [path.clone(), path.with_extension("wav.complete")] {
        fs::remove_file(&file).map_err(|error| format!("{}: {error}", file.display()))?;
    }
    decoded?
        .mono()
        .map(<[f32]>::to_vec)
        .ok_or_else(|| String::from(DOWNMIX_ONLY))
}

/// A name no other reference of this process takes.
fn unique() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    format!(
        "{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    )
}

/// The reference's views counted against the query's on `placement`.
pub fn measure(
    query: &Analysed,
    reference: &Analysed,
    placement: &Placement,
    profile: &Profile,
) -> Measured {
    let view = |query: &[Peak], reference: &[Peak]| counted(query, reference, placement, profile);
    Measured {
        denser: view(&query.denser, &reference.denser),
        harmonic: view(&query.harmonic, &reference.harmonic),
        percussive: view(&query.percussive, &reference.percussive),
    }
}

fn counted(query: &[Peak], reference: &[Peak], placement: &Placement, profile: &Profile) -> View {
    let (reference_peaks, found) = found_in_span(query, reference, placement, 0.0, profile);
    let aligned = share(reference_peaks, found);
    let shifted: Vec<f64> = CHANCE_SHIFTS_SECONDS
        .iter()
        .map(|&shift| found_in_span(query, reference, placement, shift, profile))
        .filter(|&(peaks, _)| peaks > 0)
        .map(|(peaks, found)| share(peaks, found))
        .collect();
    let chance_share = if shifted.is_empty() {
        0.0
    } else {
        shifted.iter().sum::<f64>() / shifted.len() as f64
    };
    View {
        reference_peaks,
        found,
        share: aligned,
        chance_share,
        excess_share: aligned - chance_share,
    }
}

fn share(peaks: u32, found: u32) -> f64 {
    if peaks == 0 {
        0.0
    } else {
        f64::from(found) / f64::from(peaks)
    }
}

/// The reference's peaks that map into the detection's span with the
/// reference shifted `shift_seconds`, and how many of them have a query
/// peak within the verifier's tolerance. Both are ordered by frame.
fn found_in_span(
    query: &[Peak],
    reference: &[Peak],
    placement: &Placement,
    shift_seconds: f64,
    profile: &Profile,
) -> (u32, u32) {
    let offset = placement.frame_offset(shift_seconds, profile);
    let first = profile.frames(placement.start_seconds);
    let last = profile.frames(placement.end_seconds);
    let mut peaks = 0;
    let mut found = 0;
    for peak in reference {
        let frame = peak.frame - offset;
        if frame < first || frame > last {
            continue;
        }
        peaks += 1;
        let from = query.partition_point(|candidate| candidate.frame < frame - TOLERANCE.frames);
        let near = query[from..]
            .iter()
            .take_while(|candidate| candidate.frame <= frame + TOLERANCE.frames)
            .any(|candidate| (candidate.bin - peak.bin).abs() <= TOLERANCE.bins);
        found += u32::from(near);
    }
    (peaks, found)
}

#[cfg(test)]
mod tests {
    use gunfinger_core::confidence::Evidence;
    use gunfinger_core::index::AssetId;
    use gunfinger_core::speed::SpeedRatio;

    use super::*;

    const RATE: f32 = 8000.0;

    fn tone(hz: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
        (0..(seconds * RATE) as usize)
            .map(|n| amplitude * (2.0 * std::f32::consts::PI * hz * n as f32 / RATE).sin())
            .collect()
    }

    /// A click (one sample) every `period` seconds from 0.25 s.
    fn clicks(period: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
        let mut samples = vec![0.0; (seconds * RATE) as usize];
        let mut at = 0.25;
        while ((at * RATE) as usize) < samples.len() {
            samples[(at * RATE) as usize] = amplitude;
            at += period;
        }
        samples
    }

    /// A drum hit every `period` seconds from 0.25 s: a fixed 6 ms burst
    /// of pseudo-random samples under a decay, so that its spectrum has a
    /// shape of its own (a one-sample click's is flat, and its local
    /// maxima would be set by whatever else plays).
    fn hits(period: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
        let mut state: u32 = 12_345;
        let burst: Vec<f32> = (0..48)
            .map(|n| {
                state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                let noise = (state >> 16) as f32 / 32_768.0 - 1.0;
                amplitude * noise * (-(n as f32) / 12.0).exp()
            })
            .collect();
        let mut samples = vec![0.0; (seconds * RATE) as usize];
        let mut at = 0.25;
        while ((at * RATE) as usize) + burst.len() < samples.len() {
            let start = (at * RATE) as usize;
            for (slot, value) in samples[start..].iter_mut().zip(&burst) {
                *slot += value;
            }
            at += period;
        }
        samples
    }

    fn added(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter().zip(b).map(|(a, b)| a + b).collect()
    }

    /// The power of `samples` in the bin of `hz`, summed over the frames.
    fn power_at(samples: &[f32], hz: f32) -> f64 {
        let bin = (hz * Profile::CURRENT.fft_size as f32 / RATE).round() as usize;
        spectra(samples, &Profile::CURRENT)
            .iter()
            .map(|frame| f64::from(frame[bin].norm_sqr()))
            .sum()
    }

    /// Power far from the tone (bins 300-500), in the frames around each
    /// click: where only the clicks put energy.
    fn click_power(samples: &[f32]) -> f64 {
        spectra(samples, &Profile::CURRENT)
            .iter()
            .map(|frame| {
                frame[300..500]
                    .iter()
                    .map(|bin| f64::from(bin.norm_sqr()))
                    .sum::<f64>()
            })
            .sum()
    }

    #[test]
    fn the_harmonic_part_keeps_a_sustained_tone_and_drops_clicks() {
        let tone = tone(440.0, 8.0, 0.3);
        let clicks = clicks(0.35, 8.0, 0.9);
        let mixture = added(&tone, &clicks);
        let parts = separate(&mixture, &Profile::CURRENT);
        let tone_power = power_at(&mixture, 440.0);
        let click_power_before = click_power(&mixture);
        assert!(power_at(&parts.harmonic, 440.0) > 0.8 * tone_power);
        assert!(click_power(&parts.harmonic) < 0.1 * click_power_before);
        assert!(power_at(&parts.percussive, 440.0) < 0.1 * tone_power);
        assert!(click_power(&parts.percussive) > 0.6 * click_power_before);
    }

    #[test]
    fn the_parts_add_up_to_the_audio() {
        let mixture = added(&tone(300.0, 4.0, 0.3), &clicks(0.35, 4.0, 0.9));
        let parts = separate(&mixture, &Profile::CURRENT);
        let size = Profile::CURRENT.fft_size;
        for n in size..mixture.len() - 2 * size {
            let sum = parts.harmonic[n] + parts.percussive[n];
            assert!((sum - mixture[n]).abs() < 1e-3, "sample {n}");
        }
    }

    fn placement(start: f64, end: f64, lead: f64) -> Placement {
        Placement {
            from_seconds: 0.0,
            slope: 1.0,
            lead_seconds: lead,
            start_seconds: start,
            end_seconds: end,
            render_seconds: end - start + 2.0 * lead,
        }
    }

    #[test]
    fn the_same_audio_finds_every_peak_at_the_alignment_and_few_shifted() {
        // A tone that changes every 0.4 s over clicks: no two places alike.
        let melody: Vec<f32> = (0..60)
            .flat_map(|step| tone(300.0 + 37.0 * ((step * 7) % 23) as f32, 0.4, 0.3))
            .collect();
        let audio = added(&melody, &clicks(0.35, 24.0, 0.9));
        let profile = Profile::CURRENT;
        let analysed = Analysed::of(&audio, &profile);
        // The query is the render from 8 s on: render time = query time + 8.
        let query = Analysed::of(&audio[(8.0 * RATE) as usize..], &profile);
        let measured = measure(&query, &analysed, &placement(1.0, 8.0, 9.0), &profile);
        for view in [&measured.denser, &measured.harmonic, &measured.percussive] {
            assert!(view.reference_peaks >= 20, "{view:?}");
            assert!(view.share > 0.9, "{view:?}");
            assert!(view.chance_share < 0.5, "{view:?}");
        }
    }

    #[test]
    fn a_shared_drum_track_with_other_notes_matches_in_the_percussive_part_only() {
        let drums = hits(0.35, 24.0, 0.9);
        let ours: Vec<f32> = (0..60)
            .flat_map(|step| tone(300.0 + 37.0 * ((step * 7) % 23) as f32, 0.4, 0.3))
            .collect();
        let theirs: Vec<f32> = (0..60)
            .flat_map(|step| tone(1500.0 + 41.0 * ((step * 5) % 19) as f32, 0.4, 0.3))
            .collect();
        let profile = Profile::CURRENT;
        let query = Analysed::of(&added(&drums, &ours), &profile);
        let related = Analysed::of(&added(&drums, &theirs), &profile);
        let measured = measure(&query, &related, &placement(9.0, 16.0, 9.0), &profile);
        // The notes' changes are transients too, and they differ, so the
        // percussive part does not match whole.
        assert!(measured.harmonic.share < 0.1, "{:?}", measured.harmonic);
        assert!(
            measured.percussive.excess_share > 0.3,
            "{:?}",
            measured.percussive
        );
    }

    #[test]
    fn a_placement_follows_the_line_through_the_detections_ends() {
        let detection = Detection {
            asset: AssetId(0),
            start_seconds: 10.0,
            end_seconds: 20.0,
            track_start_seconds: 100.0,
            track_end_seconds: 110.2,
            speed: SpeedRatio(1.02),
            playback: speed::Playback::Turntable,
            evidence: Evidence::new(3, 300),
        };
        let placement = Placement::of(&detection).unwrap();
        let profile = Profile::CURRENT;
        assert!((placement.slope - 1.02).abs() < 1e-9);
        assert!((placement.from_seconds - 92.0).abs() < 1e-9);
        // Query time 10 s holds file time 100 s: 8 file seconds into the
        // render, 8 / 1.02 s of render time.
        let offset = placement.frame_offset(0.0, &profile);
        assert!((offset - profile.frames(8.0 / 1.02 - 10.0)).abs() < 1e-9);
        // Shifted 5 s of the file: 5 / 1.02 s of render time further.
        let shifted = placement.frame_offset(5.0, &profile);
        assert!((shifted - offset - profile.frames(5.0 / 1.02)).abs() < 1e-9);
        // The render reaches 8 file seconds past the span's end.
        let end_in_file = placement.from_seconds + placement.slope * placement.render_seconds;
        assert!((end_in_file - 118.2).abs() < 1e-9);
    }

    #[test]
    fn a_placement_near_the_files_start_begins_at_zero() {
        let detection = Detection {
            asset: AssetId(0),
            start_seconds: 30.0,
            end_seconds: 40.0,
            track_start_seconds: 3.0,
            track_end_seconds: 13.0,
            speed: SpeedRatio(1.0),
            playback: speed::Playback::Turntable,
            evidence: Evidence::new(3, 300),
        };
        let placement = Placement::of(&detection).unwrap();
        assert_eq!(placement.from_seconds, 0.0);
        assert!((placement.lead_seconds - 3.0).abs() < 1e-9);
        assert!((placement.render_seconds - 21.0).abs() < 1e-9);
    }
}

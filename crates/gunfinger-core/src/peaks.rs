//! Spectral peaks: local maxima of the log-power spectrogram.
//!
//! A peak is a time-frequency point louder than everything around it. Peaks
//! survive equalisation, compression and added noise far better than the
//! spectrum as a whole (Wang 2003), and they are the stored source of truth
//! from which hashes and the index are derived.

use std::ops::Range;

use crate::profile::Profile;
use crate::spectrogram::for_each_frame;

/// Positions are refined between frames and bins and kept to 1/64: finer
/// than the interpolation is accurate, coarse enough for small fixed-point
/// fields in the peak store.
pub const FRAME_STEPS: f32 = 64.0;
pub const BIN_STEPS: f32 = 64.0;
/// A refined frame stays within this distance of its STFT frame, so the
/// frame is recovered by rounding.
const MAX_FRAME_OFFSET: f32 = 31.0 / FRAME_STEPS;
/// Magnitudes are kept to half a decibel, enough to rank peaks by strength.
pub const MAGNITUDE_STEPS_PER_DB: f32 = 2.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Peak {
    /// Time in STFT frames, refined between frames and quantised to
    /// `FRAME_STEPS`. Double precision keeps 1/64 frame exact in a mix of
    /// several hours.
    pub frame: f64,
    /// Frequency in bins, refined between bins and quantised to `BIN_STEPS`.
    pub bin: f32,
    /// Log power in dB, quantised to `MAGNITUDE_STEPS_PER_DB`.
    pub magnitude: f32,
}

/// Extracts the peaks of `samples`, ordered by frame and then by bin.
pub fn extract_peaks(samples: &[f32], profile: &Profile) -> Vec<Peak> {
    extract_peaks_in(samples, profile, 0..usize::MAX)
}

/// The peaks `extract_peaks` finds in the STFT frames `frames`, the same to
/// the bit. Only the frames their neighbourhoods reach are transformed, so
/// a long query can be analysed a stretch at a time.
pub fn extract_peaks_in(samples: &[f32], profile: &Profile, frames: Range<usize>) -> Vec<Peak> {
    let first = frames.start.saturating_sub(profile.neighbourhood_frames);
    let start = first.saturating_mul(profile.hop);
    if start >= samples.len() {
        return Vec::new();
    }
    // Frame `n` ends at sample `n * hop + fft_size`.
    let end = frames
        .end
        .saturating_add(profile.neighbourhood_frames)
        .saturating_sub(1)
        .saturating_mul(profile.hop)
        .saturating_add(profile.fft_size)
        .min(samples.len());
    let mut picker = PeakPicker::new(profile, first, frames);
    for_each_frame(&samples[start..end], profile, |frame| picker.push(frame));
    picker.finish()
}

/// Streams frames through a ring buffer just tall enough to hold one
/// neighbourhood, so memory stays constant however long the audio is.
struct PeakPicker<'p> {
    profile: &'p Profile,
    /// Frame `n` lives at `rows[n % rows.len()]`.
    rows: Vec<Vec<f32>>,
    /// The number of the next frame to be pushed.
    received: usize,
    /// Peaks are picked in these frames only.
    wanted: Range<usize>,
    peaks: Vec<Peak>,
}

impl<'p> PeakPicker<'p> {
    /// A picker whose first frame pushed is frame `first`. Frames from
    /// `wanted.start - neighbourhood_frames` on must be pushed, or frames
    /// from 0 (the start of the audio).
    fn new(profile: &'p Profile, first: usize, wanted: Range<usize>) -> Self {
        let height = 2 * profile.neighbourhood_frames + 1;
        let bins = profile.fft_size / 2 + 1;
        PeakPicker {
            profile,
            rows: vec![vec![f32::NEG_INFINITY; bins]; height],
            received: first,
            wanted,
            peaks: Vec::new(),
        }
    }

    fn push(&mut self, frame: &[f32]) {
        let slot = self.received % self.rows.len();
        self.rows[slot].copy_from_slice(frame);
        self.received += 1;
        // The frame `neighbourhood_frames` back now has its whole neighbourhood.
        if let Some(centre) = self
            .received
            .checked_sub(1 + self.profile.neighbourhood_frames)
        {
            self.pick(centre);
        }
    }

    /// Picks the last frames, whose neighbourhoods the end of the audio
    /// cuts short. Pushing must have stopped at the end of the audio or
    /// `neighbourhood_frames` past the wanted frames.
    fn finish(mut self) -> Vec<Peak> {
        let first_unpicked = self
            .received
            .saturating_sub(self.profile.neighbourhood_frames);
        for centre in first_unpicked..self.received {
            self.pick(centre);
        }
        self.peaks
    }

    fn pick(&mut self, centre: usize) {
        if !self.wanted.contains(&centre) {
            return;
        }
        let profile = self.profile;
        let height = self.rows.len();
        let row = &self.rows[centre % height];
        let before = (centre > 0).then(|| &self.rows[(centre - 1) % height]);
        let after = (centre + 1 < self.received).then(|| &self.rows[(centre + 1) % height]);
        for bin in profile.min_bin..profile.max_bin {
            let value = row[bin];
            // The four nearest neighbours reject most bins cheaply, before
            // the whole neighbourhood is scanned.
            if value < profile.floor_db
                || value <= row[bin - 1]
                || value <= row[bin + 1]
                || before.is_some_and(|before| value <= before[bin])
                || after.is_some_and(|after| value <= after[bin])
                || !self.dominates_neighbourhood(centre, bin)
            {
                continue;
            }
            // Without refinement, the time difference of two peaks jitters by
            // a whole frame whenever the query's frame grid is offset from
            // the reference's (experiment 0001).
            let frame_offset = match (before, after) {
                (Some(before), Some(after)) => vertex_offset(before[bin], value, after[bin]),
                _ => 0.0,
            }
            .clamp(-MAX_FRAME_OFFSET, MAX_FRAME_OFFSET);
            let bin_offset = vertex_offset(row[bin - 1], value, row[bin + 1]);
            self.peaks.push(Peak {
                frame: centre as f64 + f64::from(quantise(frame_offset, FRAME_STEPS)),
                bin: bin as f32 + quantise(bin_offset, BIN_STEPS),
                magnitude: quantise(value, MAGNITUDE_STEPS_PER_DB),
            });
        }
    }

    /// True when `(centre, bin)` is strictly louder than every other point in
    /// its neighbourhood that exists (frames before the start or after the end
    /// do not).
    fn dominates_neighbourhood(&self, centre: usize, bin: usize) -> bool {
        let profile = self.profile;
        let value = self.rows[centre % self.rows.len()][bin];
        let first_frame = centre.saturating_sub(profile.neighbourhood_frames);
        let last_frame = (centre + profile.neighbourhood_frames).min(self.received - 1);
        let half = profile.neighbourhood_bins_at(bin);
        let first_bin = bin.saturating_sub(half);
        let last_bin = (bin + half).min(self.rows[0].len() - 1);
        for frame in first_frame..=last_frame {
            let row = &self.rows[frame % self.rows.len()];
            for other in first_bin..=last_bin {
                if (frame, other) != (centre, bin) && row[other] >= value {
                    return false;
                }
            }
        }
        true
    }
}

/// Offset of the vertex of the parabola through three equally spaced points,
/// relative to the middle one. For a local maximum it lies in (-0.5, 0.5).
/// On a log-power Hann spectrum this recovers a sinusoid's frequency to a
/// small fraction of a bin (Smith & Serra 1987, quadratic interpolation).
fn vertex_offset(left: f32, middle: f32, right: f32) -> f32 {
    let curvature = left - 2.0 * middle + right;
    if curvature == 0.0 {
        return 0.0;
    }
    0.5 * (left - right) / curvature
}

fn quantise(value: f32, steps_per_unit: f32) -> f32 {
    (value * steps_per_unit).round() / steps_per_unit
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    fn tone(frequency: f32, seconds: f32, profile: &Profile) -> Vec<f32> {
        let rate = profile.sample_rate as f32;
        (0..(seconds * rate) as usize)
            .map(|n| 0.5 * (2.0 * PI * frequency * n as f32 / rate).sin())
            .collect()
    }

    #[test]
    fn a_steady_tone_yields_peaks_at_its_frequency() {
        let profile = Profile::CURRENT;
        let frequency = 1000.0;

        let peaks = extract_peaks(&tone(frequency, 2.0, &profile), &profile);

        assert!(!peaks.is_empty());
        let expected_bin = frequency / profile.bin_hz() as f32;
        for peak in &peaks {
            assert!(
                (peak.bin - expected_bin).abs() < 0.1,
                "peak at bin {} for a tone at bin {expected_bin}",
                peak.bin
            );
        }
    }

    #[test]
    fn silence_has_no_peaks() {
        let profile = Profile::CURRENT;

        assert!(extract_peaks(&vec![0.0; 16_000], &profile).is_empty());
    }

    #[test]
    fn peaks_are_ordered_by_frame_then_bin() {
        let profile = Profile::CURRENT;
        let chord: Vec<f32> = tone(440.0, 3.0, &profile)
            .iter()
            .zip(tone(1900.0, 3.0, &profile))
            .map(|(a, b)| a + b)
            .collect();

        let peaks = extract_peaks(&chord, &profile);

        assert!(peaks.len() >= 2);
        assert!(peaks.windows(2).all(
            |pair| (pair[0].frame.round(), pair[0].bin) < (pair[1].frame.round(), pair[1].bin)
        ));
    }

    #[test]
    fn peaks_of_a_stretch_of_frames_are_those_of_the_whole() {
        let profile = Profile::CURRENT;
        let mut state: u32 = 7;
        let noise: Vec<f32> = (0..20 * profile.sample_rate)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (state >> 8) as f32 / (1 << 24) as f32 - 0.5
            })
            .collect();
        let whole = extract_peaks(&noise, &profile);
        let frames = whole.last().unwrap().frame.round() as usize + 1;

        for stretch in [
            0..5,
            3..40,
            100..700,
            600..frames + 50,
            frames - 3..usize::MAX,
        ] {
            let expected: Vec<Peak> = whole
                .iter()
                .filter(|peak| stretch.contains(&(peak.frame.round() as usize)))
                .copied()
                .collect();
            assert_eq!(
                extract_peaks_in(&noise, &profile, stretch.clone()),
                expected,
                "{stretch:?}"
            );
        }
    }

    #[test]
    fn the_vertex_of_a_symmetric_triple_is_the_middle() {
        assert_eq!(vertex_offset(1.0, 2.0, 1.0), 0.0);
        assert!((vertex_offset(1.0, 2.0, 1.5) - 0.166_666_67).abs() < 1e-6);
    }
}

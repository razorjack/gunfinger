//! Spectral peaks: local maxima of the log-power spectrogram.
//!
//! A peak is a time-frequency point louder than everything around it. Peaks
//! survive equalisation, compression and added noise far better than the
//! spectrum as a whole (Wang 2003), and they are the stored source of truth
//! from which hashes and the index are derived.

use std::cmp::Ordering;
use std::ops::Range;

use crate::profile::Profile;
use crate::spectrogram::{decibels, for_each_power_frame};

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
    for_each_power_frame(&samples[start..end], profile, |frame| picker.push(frame));
    picker.finish()
}

/// Streams frames through a ring buffer just tall enough to hold one
/// neighbourhood, so memory stays constant however long the audio is.
///
/// Peaks are defined on levels in decibels, but the ring holds powers: since
/// `decibels` never decreases as power grows, a point with no more power than
/// another has no higher level, and the loudest power of a set has its
/// loudest level. Comparing powers settles every test but one: for a bin
/// that passes them all (about 0.6 a frame), the level of the loudest other
/// point may still round to the bin's, which its level decides. Levels are
/// otherwise taken only to refine the peaks, so the peaks are those picked
/// on levels, to the bit (experiment 0213).
struct PeakPicker<'p> {
    profile: &'p Profile,
    /// The lowest power whose level reaches the profile's floor.
    floor_power: f32,
    /// Frame `n`'s powers live at `rows[n % rows.len()]`.
    rows: Vec<Vec<f32>>,
    /// The number of the next frame to be pushed.
    received: usize,
    /// Peaks are picked in these frames only.
    wanted: Range<usize>,
    /// Whether each bin from `min_bin` of the frame being picked reaches the
    /// floor and has more power than its four nearest neighbours (see
    /// `pick`).
    passes: Vec<bool>,
    /// The bins that pass, in the first places.
    candidates: Vec<usize>,
    /// Stands for the frame before the first and after the last.
    silence: Vec<f32>,
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
            floor_power: floor_power(profile.floor_db),
            rows: vec![vec![0.0; bins]; height],
            received: first,
            wanted,
            passes: vec![false; bins],
            candidates: vec![0; bins],
            silence: vec![0.0; bins],
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
        // The four nearest neighbours reject most bins cheaply, before the
        // whole neighbourhood is scanned. Their outcome changes from bin to
        // bin, so they are evaluated without branches (experiment 0097); a
        // missing frame counts as silence (no power, a level of -inf), which
        // rejects nothing a finite floor does not. A bin with no more power
        // than a neighbour has no higher level; one below `floor_power` is
        // below the floor. The test of every bin is one loop over slices of
        // the same length with no running count, which the compiler can turn
        // into tests of several bins at once; the bins that pass are
        // gathered after it.
        let (above, below) = (
            before.unwrap_or(&self.silence),
            after.unwrap_or(&self.silence),
        );
        let (first, end) = (profile.min_bin, profile.max_bin);
        let values = &row[first..end];
        let count = values.len();
        let (lower, higher) = (&row[first - 1..][..count], &row[first + 1..][..count]);
        let (above, below) = (&above[first..][..count], &below[first..][..count]);
        let passes = &mut self.passes[..count];
        for offset in 0..count {
            let value = values[offset];
            passes[offset] = !((value < self.floor_power)
                | (value <= lower[offset])
                | (value <= higher[offset])
                | (value <= above[offset])
                | (value <= below[offset]));
        }
        let mut candidates = 0;
        for (offset, &passed) in passes.iter().enumerate() {
            self.candidates[candidates] = first + offset;
            candidates += usize::from(passed);
        }
        for &bin in &self.candidates[..candidates] {
            let Some(level) = self.peak_level(centre, bin) else {
                continue;
            };
            // Without refinement, the time difference of two peaks jitters by
            // a whole frame whenever the query's frame grid is offset from
            // the reference's (experiment 0001).
            let frame_offset = match (before, after) {
                (Some(before), Some(after)) => {
                    vertex_offset(decibels(before[bin]), level, decibels(after[bin]))
                }
                _ => 0.0,
            }
            .clamp(-MAX_FRAME_OFFSET, MAX_FRAME_OFFSET);
            let bin_offset = vertex_offset(decibels(row[bin - 1]), level, decibels(row[bin + 1]));
            self.peaks.push(Peak {
                frame: centre as f64 + f64::from(quantise(frame_offset, FRAME_STEPS)),
                bin: bin as f32 + quantise(bin_offset, BIN_STEPS),
                magnitude: quantise(level, MAGNITUDE_STEPS_PER_DB),
            });
        }
    }

    /// The level of `(centre, bin)` when it is strictly louder than every
    /// other point in its neighbourhood that exists (frames before the start
    /// or after the end do not) and than its four nearest neighbours.
    ///
    /// Frames are tried nearest first, where a louder point is likeliest, and
    /// each frame's bins are compared without branches; the answer does not
    /// depend on the order. A point with at least the power is at least as
    /// loud; when none has, only the loudest of the others can match the
    /// level.
    fn peak_level(&self, centre: usize, bin: usize) -> Option<f32> {
        let profile = self.profile;
        let row = &self.rows[centre % self.rows.len()];
        let value = row[bin];
        let half = profile.neighbourhood_bins_at(bin);
        let first_bin = bin.saturating_sub(half);
        let last_bin = (bin + half).min(row.len() - 1);
        let louder_or_equal = |others: &[f32]| {
            others
                .iter()
                .fold(false, |found, &other| found | (other >= value))
        };
        if louder_or_equal(&row[first_bin..bin]) || louder_or_equal(&row[bin + 1..=last_bin]) {
            return None;
        }
        let first_frame = centre.saturating_sub(profile.neighbourhood_frames);
        let last_frame = (centre + profile.neighbourhood_frames).min(self.received - 1);
        for distance in 1..=profile.neighbourhood_frames {
            let nearby = [centre.checked_sub(distance), Some(centre + distance)];
            for frame in nearby.into_iter().flatten() {
                if (first_frame..=last_frame).contains(&frame)
                    && louder_or_equal(&self.rows[frame % self.rows.len()][first_bin..=last_bin])
                {
                    return None;
                }
            }
        }
        // `max` passes over NaN as the comparisons on levels did.
        let loudest_of = |others: &[f32]| {
            others
                .iter()
                .fold(0.0_f32, |loudest, &other| loudest.max(other))
        };
        let mut loudest =
            loudest_of(&row[first_bin..bin]).max(loudest_of(&row[bin + 1..=last_bin]));
        for frame in first_frame..=last_frame {
            if frame != centre {
                loudest = loudest.max(loudest_of(
                    &self.rows[frame % self.rows.len()][first_bin..=last_bin],
                ));
            }
        }
        // The four nearest neighbours, which a neighbourhood narrower than a
        // bin or a frame either way would leave out.
        loudest = loudest.max(row[bin - 1]).max(row[bin + 1]);
        let nearest_frames = [centre.checked_sub(1), Some(centre + 1)];
        for frame in nearest_frames.into_iter().flatten() {
            if frame < self.received {
                loudest = loudest.max(self.rows[frame % self.rows.len()][bin]);
            }
        }
        // Strictly louder: the loudest other level is lower, or the two do
        // not compare (a NaN level, which nothing reaches).
        let level = decibels(value);
        decibels(loudest)
            .partial_cmp(&level)
            .is_none_or(Ordering::is_lt)
            .then_some(level)
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

/// The lowest power whose level in decibels reaches `floor_db`. Since
/// `decibels` never decreases as power grows, a power is below the floor
/// exactly when it is below this one. Non-negative floats are ordered as
/// their bit patterns, so the search halves a range of patterns.
fn floor_power(floor_db: f32) -> f32 {
    let mut below = 0_u32;
    let mut reaches = f32::INFINITY.to_bits();
    while reaches - below > 1 {
        let middle = below + (reaches - below) / 2;
        if decibels(f32::from_bits(middle)) < floor_db {
            below = middle;
        } else {
            reaches = middle;
        }
    }
    f32::from_bits(reaches)
}

fn quantise(value: f32, steps_per_unit: f32) -> f32 {
    (value * steps_per_unit).round() / steps_per_unit
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use crate::spectrogram::for_each_frame;

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

    /// The peaks' STFT frames, bins and levels by the definition on levels in
    /// decibels: louder than the floor and strictly louder than every other
    /// point of the neighbourhood that exists, over the whole spectrogram at
    /// once.
    fn peaks_by_definition(samples: &[f32], profile: &Profile) -> Vec<(usize, usize, f32)> {
        let mut spectrogram: Vec<Vec<f32>> = Vec::new();
        for_each_frame(samples, profile, |frame| spectrogram.push(frame.to_vec()));
        let mut peaks = Vec::new();
        for (centre, row) in spectrogram.iter().enumerate() {
            for bin in profile.min_bin..profile.max_bin {
                let value = row[bin];
                let half = profile.neighbourhood_bins_at(bin);
                let frames = centre.saturating_sub(profile.neighbourhood_frames)
                    ..=(centre + profile.neighbourhood_frames).min(spectrogram.len() - 1);
                let bins = bin.saturating_sub(half)..=(bin + half).min(row.len() - 1);
                let mut others_quieter = true;
                for frame in frames {
                    for other in bins.clone() {
                        if (frame, other) != (centre, bin) && spectrogram[frame][other] >= value {
                            others_quieter = false;
                        }
                    }
                }
                if value >= profile.floor_db && others_quieter {
                    peaks.push((centre, bin, value));
                }
            }
        }
        peaks
    }

    #[test]
    fn peaks_are_the_points_louder_than_their_whole_neighbourhood() {
        let profile = Profile::CURRENT;
        let mut state: u32 = 11;
        let mut noise = |count: usize| -> Vec<f32> {
            (0..count)
                .map(|_| {
                    state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    (state >> 8) as f32 / (1 << 24) as f32 - 0.5
                })
                .collect()
        };
        let chord: Vec<f32> = tone(440.0, 6.0, &profile)
            .iter()
            .zip(tone(1900.0, 6.0, &profile))
            .zip(noise(6 * profile.sample_rate as usize))
            .map(|((a, b), n)| a + b + 0.3 * n)
            .collect();
        // Noise repeating every 4 hops: frames 4 apart are equal, so every
        // local maximum ties with points of its neighbourhood and none is a
        // peak.
        let period = noise(4 * profile.hop);
        let repeating: Vec<f32> = period
            .iter()
            .cycle()
            .take(5 * profile.sample_rate as usize)
            .copied()
            .collect();
        let spread = Profile {
            spread: Some(crate::profile::Spread {
                share: 0.094,
                min_bins: 4,
                max_bins: 24,
            }),
            ..profile
        };

        for (samples, profile) in [
            (&chord, &profile),
            (&repeating, &profile),
            (&chord, &spread),
        ] {
            let found = extract_peaks(samples, profile);
            let expected = peaks_by_definition(samples, profile);
            assert_eq!(found.len(), expected.len());
            // A refined frame rounds to its STFT frame; a refined bin may lie
            // half a bin from its STFT bin. The magnitude is the level, to the
            // bit once quantised.
            for (peak, &(frame, bin, level)) in found.iter().zip(&expected) {
                assert_eq!(peak.frame.round() as usize, frame);
                assert!((peak.bin - bin as f32).abs() <= 0.5, "{peak:?} {bin}");
                assert_eq!(
                    peak.magnitude.to_bits(),
                    quantise(level, MAGNITUDE_STEPS_PER_DB).to_bits()
                );
            }
        }
        assert!(peaks_by_definition(&chord, &profile).len() > 100);
        assert!(peaks_by_definition(&chord, &spread).len() > 100);
        assert!(peaks_by_definition(&repeating, &profile).is_empty());
    }

    /// Two powers whose levels round to the same decibels: the louder power
    /// is no peak, as on levels, whether the other is a nearest neighbour or
    /// farther away in the neighbourhood.
    #[test]
    fn powers_with_the_same_level_tie_as_their_levels_do() {
        let profile = Profile::CURRENT;
        let bins = profile.fft_size / 2 + 1;
        let quiet = 1.0_f32;
        let loud = 1000.0_f32;
        let louder = f32::from_bits(loud.to_bits() + 1);
        assert!(louder > loud);
        assert_eq!(decibels(louder), decibels(loud));
        let frames = 2 * profile.neighbourhood_frames + 1;
        let centre = profile.neighbourhood_frames;
        let bin = 100;

        // A neighbourhood one bin wide leaves the nearest bins either side to
        // the four-neighbour terms of the final comparison.
        let narrow = Profile {
            neighbourhood_bins: 0,
            ..profile
        };
        for (frame, other_bin, profile) in [
            (centre, bin + 1, &profile),
            (centre + 1, bin, &profile),
            (centre + 5, bin - 7, &profile),
            (centre, bin - 1, &narrow),
            (centre, bin, &profile),
        ] {
            let mut picker = PeakPicker::new(profile, 0, centre..centre + 1);
            for row_frame in 0..frames {
                let mut row = vec![quiet; bins];
                if row_frame == centre {
                    row[bin] = louder;
                }
                if row_frame == frame && (frame, other_bin) != (centre, bin) {
                    row[other_bin] = loud;
                }
                picker.push(&row);
            }
            let peaks = picker.finish();

            let alone = (frame, other_bin) == (centre, bin);
            assert_eq!(peaks.len(), usize::from(alone), "{frame} {other_bin}");
        }
    }

    #[test]
    fn the_floor_power_is_the_lowest_power_whose_level_reaches_the_floor() {
        for floor_db in [-10.0, 0.0, -200.0, 35.5] {
            let power = floor_power(floor_db);

            assert!(decibels(power) >= floor_db);
            assert!(decibels(f32::from_bits(power.to_bits() - 1)) < floor_db);
        }
    }

    #[test]
    fn the_vertex_of_a_symmetric_triple_is_the_middle() {
        assert_eq!(vertex_offset(1.0, 2.0, 1.0), 0.0);
        assert!((vertex_offset(1.0, 2.0, 1.5) - 0.166_666_67).abs() < 1e-6);
    }
}

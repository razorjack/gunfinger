//! The masked residual of proposal 4 (brief 65): what is left of an
//! excerpt of a mix once a record known to play there is taken out, so
//! that a quieter record under it can be searched for.
//!
//! The residual is a spectral mask, not a subtraction of waveforms, which
//! would need the record's timing and transfer to the sample. Given a
//! render of the record placed where it plays (the reference), its gain
//! in the mixture is estimated per band and block of about a second, and
//! each bin keeps the share of its power that the record does not explain
//! (a power residual: powers of unrelated signals add only on average).
//! The masked spectrum is turned back into audio by overlap-add, because
//! the search analyses its query again on every rung of the ladder.

use std::ops::Range;

use gunfinger_core::profile::Profile;
use realfft::RealFftPlanner;
use realfft::num_complex::Complex;
use serde::{Deserialize, Serialize};

/// How much of the reference's estimated power each bin of the mixture
/// loses: `over` times the estimate (over-subtraction; above 1 removes
/// more of the record where the estimate falls short, and more of what
/// plays with it). A bin the record explains entirely falls to silence,
/// below the peak picker's floor: a share of it kept would keep the
/// record's pattern of peaks wherever nothing else plays near it.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub over: f64,
}

/// Gains are fitted per block of this many frames (about 1 s at the
/// profile's hop): slow enough to hold many points, fast enough to
/// follow a fade of 2 s or more.
const BLOCK_FRAMES: usize = 63;
/// The gain is this quantile of the mixture's power over the reference's
/// at the reference's stronger points: where another record adds power
/// the ratio rises, so a low quantile follows the points the reference
/// dominates.
const GAIN_QUANTILE: f64 = 0.3;
/// A band and block with fewer points of the reference than this gets no
/// gain: nothing is taken out there.
const MIN_POINTS: usize = 16;
/// A band and block whose reference power is below this share of the
/// band's mean over the whole excerpt is silence of the record (before it
/// fades in, in a break): no gain, since the ratio there measures what
/// else plays, not the record.
const SILENT_SHARE: f32 = 1e-6;
/// The first band ends here (39 Hz at the profile's rate); the bands above
/// are a third of an octave wide.
const FIRST_BAND_END: usize = 5;
/// Samples whose overlapping windows add up to less than this (the first
/// and last few) keep the mixture's value.
const MIN_WINDOW_WEIGHT: f32 = 0.1;

/// The residual of `mixture` once `reference` (the same span, aligned
/// sample for sample, at any level) is taken out under `rule`. Both are
/// mono at the profile's rate; the residual has the mixture's length.
pub fn residual(mixture: &[f32], reference: &[f32], profile: &Profile, rule: Rule) -> Vec<f32> {
    let mut reference = reference.to_vec();
    reference.resize(mixture.len(), 0.0);
    let mut spectra = spectra(mixture, profile);
    let mixture_powers: Vec<Vec<f32>> = spectra.iter().map(|frame| powers(frame)).collect();
    let reference_powers: Vec<Vec<f32>> = spectra_of_reference(&reference, profile);
    let bands = bands(profile.fft_size / 2 + 1);
    let gains = band_gains(&mixture_powers, &reference_powers, &bands);
    let over = rule.over as f32;
    for (frame, spectrum) in spectra.iter_mut().enumerate() {
        let block = &gains[frame / BLOCK_FRAMES];
        for (band, &gain) in bands.iter().zip(block) {
            for bin in band.clone() {
                let explained = over * gain * reference_powers[frame][bin];
                let power = mixture_powers[frame][bin];
                if power > 0.0 {
                    let kept = (1.0 - explained / power).max(0.0);
                    spectrum[bin] *= kept.sqrt();
                }
            }
        }
    }
    overlap_add(&spectra, mixture, profile)
}

/// The reference's gain in the mixture, per block of `BLOCK_FRAMES`
/// frames (outer) and band (inner), as a ratio of powers.
fn band_gains(
    mixture: &[Vec<f32>],
    reference: &[Vec<f32>],
    bands: &[Range<usize>],
) -> Vec<Vec<f32>> {
    let means: Vec<f32> = bands
        .iter()
        .map(|band| mean_power(reference, band.clone()))
        .collect();
    mixture
        .chunks(BLOCK_FRAMES)
        .zip(reference.chunks(BLOCK_FRAMES))
        .map(|(mixture, reference)| {
            bands
                .iter()
                .zip(&means)
                .map(|(band, &mean)| {
                    if mean_power(reference, band.clone()) < SILENT_SHARE * mean {
                        0.0
                    } else {
                        block_gain(mixture, reference, band.clone())
                    }
                })
                .collect()
        })
        .collect()
}

/// `GAIN_QUANTILE` of the mixture's power over the reference's, at the
/// points of the band and block where the reference's power is at least
/// its median there.
fn block_gain(mixture: &[Vec<f32>], reference: &[Vec<f32>], band: Range<usize>) -> f32 {
    let points: Vec<(f32, f32)> = mixture
        .iter()
        .zip(reference)
        .flat_map(|(mixture, reference)| {
            band.clone().map(move |bin| (mixture[bin], reference[bin]))
        })
        .filter(|&(_, reference)| reference > 0.0)
        .collect();
    if points.len() < MIN_POINTS {
        return 0.0;
    }
    let median = quantile(
        points.iter().map(|&(_, reference)| reference).collect(),
        0.5,
    );
    let ratios: Vec<f32> = points
        .iter()
        .filter(|&&(_, reference)| reference >= median)
        .map(|&(mixture, reference)| mixture / reference)
        .collect();
    if ratios.len() < MIN_POINTS {
        return 0.0;
    }
    quantile(ratios, GAIN_QUANTILE)
}

/// The value at `share` of the way through `values` sorted (nearest rank).
fn quantile(mut values: Vec<f32>, share: f64) -> f32 {
    values.sort_by(f32::total_cmp);
    let rank = ((values.len() - 1) as f64 * share).round() as usize;
    values[rank]
}

fn mean_power(frames: &[Vec<f32>], band: Range<usize>) -> f32 {
    let count = frames.len() * band.len();
    let total: f64 = frames
        .iter()
        .flat_map(|frame| &frame[band.clone()])
        .map(|&power| f64::from(power))
        .sum();
    (total / count.max(1) as f64) as f32
}

/// The bands gains are fitted in: below `FIRST_BAND_END`, then a third of
/// an octave each, up to `bins`.
fn bands(bins: usize) -> Vec<Range<usize>> {
    let mut edges = vec![0, FIRST_BAND_END];
    let mut step = 1;
    while edges[edges.len() - 1] < bins {
        let edge = (FIRST_BAND_END as f64 * 2_f64.powf(f64::from(step) / 3.0)).round() as usize;
        step += 1;
        if edge > edges[edges.len() - 1] {
            edges.push(edge.min(bins));
        }
    }
    edges.windows(2).map(|pair| pair[0]..pair[1]).collect()
}

/// The STFT of `samples` at the profile's window and hop: frame `i`
/// covers samples `i * hop .. i * hop + fft_size`, as the core's
/// spectrogram does.
pub(crate) fn spectra(samples: &[f32], profile: &Profile) -> Vec<Vec<Complex<f32>>> {
    let size = profile.fft_size;
    let window = hann(size);
    let fft = RealFftPlanner::<f32>::new().plan_fft_forward(size);
    let mut input = fft.make_input_vec();
    let mut scratch = fft.make_scratch_vec();
    let mut frames = Vec::new();
    let mut start = 0;
    while start + size <= samples.len() {
        for (n, value) in input.iter_mut().enumerate() {
            *value = samples[start + n] * window[n];
        }
        let mut spectrum = fft.make_output_vec();
        let transformed = fft.process_with_scratch(&mut input, &mut spectrum, &mut scratch);
        debug_assert!(transformed.is_ok(), "buffers come from the plan");
        frames.push(spectrum);
        start += profile.hop;
    }
    frames
}

fn spectra_of_reference(reference: &[f32], profile: &Profile) -> Vec<Vec<f32>> {
    spectra(reference, profile)
        .iter()
        .map(|frame| powers(frame))
        .collect()
}

fn powers(spectrum: &[Complex<f32>]) -> Vec<f32> {
    spectrum.iter().map(Complex::norm_sqr).collect()
}

/// Weighted overlap-add: each frame's inverse transform, windowed again,
/// summed and divided by the sum of the squared windows, which gives back
/// the original samples when no frame was changed.
pub(crate) fn overlap_add(
    spectra: &[Vec<Complex<f32>>],
    mixture: &[f32],
    profile: &Profile,
) -> Vec<f32> {
    let size = profile.fft_size;
    let window = hann(size);
    let ifft = RealFftPlanner::<f32>::new().plan_fft_inverse(size);
    let mut frame = ifft.make_output_vec();
    let mut scratch = ifft.make_scratch_vec();
    let mut sum = vec![0.0_f32; mixture.len()];
    let mut weight = vec![0.0_f32; mixture.len()];
    for (index, spectrum) in spectra.iter().enumerate() {
        let mut input = spectrum.clone();
        // A real signal's spectrum has no imaginary part at 0 and at the
        // Nyquist frequency; the inverse transform requires it.
        let last = input.len() - 1;
        input[0].im = 0.0;
        input[last].im = 0.0;
        let transformed = ifft.process_with_scratch(&mut input, &mut frame, &mut scratch);
        debug_assert!(transformed.is_ok(), "buffers come from the plan");
        let start = index * profile.hop;
        for n in 0..size {
            // realfft's inverse is not normalised: it scales by `size`.
            sum[start + n] += window[n] * frame[n] / size as f32;
            weight[start + n] += window[n] * window[n];
        }
    }
    mixture
        .iter()
        .zip(sum.iter().zip(&weight))
        .map(|(&original, (&sum, &weight))| {
            if weight >= MIN_WINDOW_WEIGHT {
                sum / weight
            } else {
                original
            }
        })
        .collect()
}

/// The periodic Hann window, as the core's spectrogram uses.
fn hann(size: usize) -> Vec<f32> {
    (0..size)
        .map(|n| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * n as f32 / size as f32).cos())
        .collect()
}

#[cfg(test)]
mod tests {
    use gunfinger_core::peaks::{Peak, extract_peaks};

    use super::*;

    const RATE: f32 = 8000.0;

    /// Bursts of a tone at `hz`, 80 ms long under a Hann envelope, every
    /// 500 ms from `first` seconds, for `seconds`.
    fn bursts(hz: f32, first: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
        let length = (seconds * RATE) as usize;
        let burst = (0.08 * RATE) as usize;
        let mut samples = vec![0.0; length];
        let mut start = (first * RATE) as usize;
        while start + burst < length {
            for n in 0..burst {
                let envelope =
                    0.5 - 0.5 * (2.0 * std::f32::consts::PI * n as f32 / burst as f32).cos();
                let t = (start + n) as f32 / RATE;
                samples[start + n] +=
                    amplitude * envelope * (2.0 * std::f32::consts::PI * hz * t).sin();
            }
            start += (0.5 * RATE) as usize;
        }
        samples
    }

    fn added(a: &[f32], b: &[f32]) -> Vec<f32> {
        a.iter().zip(b).map(|(a, b)| a + b).collect()
    }

    fn scaled(samples: &[f32], gain: f32) -> Vec<f32> {
        samples.iter().map(|sample| sample * gain).collect()
    }

    /// Peaks within two bins of `hz`.
    fn near(peaks: &[Peak], hz: f32) -> usize {
        let bin = hz * Profile::CURRENT.fft_size as f32 / RATE;
        peaks
            .iter()
            .filter(|peak| (peak.bin - bin).abs() <= 2.0)
            .count()
    }

    /// Record A: two tones; record B: two other tones, 9 dB down, a
    /// quarter of a second later.
    fn records() -> (Vec<f32>, Vec<f32>) {
        let a = added(
            &bursts(300.0, 0.5, 20.0, 0.3),
            &bursts(1200.0, 0.5, 20.0, 0.3),
        );
        let quiet = 0.3 * 10_f32.powf(-9.0 / 20.0);
        let b = added(
            &bursts(700.0, 0.75, 20.0, quiet),
            &bursts(2000.0, 0.75, 20.0, quiet),
        );
        (a, b)
    }

    #[test]
    fn the_residual_loses_the_peaks_of_the_record_taken_out_and_keeps_the_others() {
        let (a, b) = records();
        let mixture = added(&scaled(&a, 0.5), &b);
        let profile = Profile::CURRENT;
        let before = extract_peaks(&mixture, &profile);
        // The reference is A at another level: the gain is fitted.
        let left = residual(&mixture, &a, &profile, Rule { over: 2.0 });
        let after = extract_peaks(&left, &profile);
        for hz in [300.0, 1200.0] {
            assert!(
                near(&before, hz) >= 30,
                "A's peaks at {hz} Hz in the mixture"
            );
            assert_eq!(near(&after, hz), 0, "A's peaks at {hz} Hz in the residual");
        }
        for hz in [700.0, 2000.0] {
            assert!(
                near(&before, hz) >= 30,
                "B's peaks at {hz} Hz in the mixture"
            );
            assert_eq!(
                near(&after, hz),
                near(&before, hz),
                "B's peaks at {hz} Hz kept"
            );
        }
    }

    #[test]
    fn a_silent_reference_leaves_the_mixture_as_it_was() {
        let (a, b) = records();
        let mixture = added(&a, &b);
        let profile = Profile::CURRENT;
        let left = residual(
            &mixture,
            &vec![0.0; mixture.len()],
            &profile,
            Rule { over: 2.0 },
        );
        let largest_error = mixture
            .iter()
            .zip(&left)
            .map(|(before, after)| (before - after).abs())
            .fold(0.0_f32, f32::max);
        assert!(
            largest_error < 1e-4,
            "overlap-add gives the samples back: {largest_error}"
        );
    }

    #[test]
    fn the_gain_is_the_references_level_in_the_mixture() {
        let (a, _) = records();
        let profile = Profile::CURRENT;
        let mixture: Vec<Vec<f32>> = spectra(&scaled(&a, 0.5), &profile)
            .iter()
            .map(|frame| powers(frame))
            .collect();
        let reference = spectra_of_reference(&a, &profile);
        let bands = bands(profile.fft_size / 2 + 1);
        let gains = band_gains(&mixture, &reference, &bands);
        // The band holding 300 Hz (bin 38.4), in a block in the middle.
        let band = bands.iter().position(|band| band.contains(&38)).unwrap();
        let gain = gains[gains.len() / 2][band];
        assert!(
            (gain - 0.25).abs() < 0.01,
            "an amplitude of 0.5 is a power of 0.25: {gain}"
        );
    }

    #[test]
    fn the_bands_cover_every_bin_once_a_third_of_an_octave_wide() {
        let bands = bands(513);
        assert_eq!(bands[0], 0..5);
        assert_eq!(bands[1].start, 5);
        assert_eq!(bands[bands.len() - 1].end, 513);
        for pair in bands.windows(2) {
            assert_eq!(pair[0].end, pair[1].start);
        }
        assert_eq!(bands.len(), 22);
    }
}

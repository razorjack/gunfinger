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
//!
//! Brief 70 adds a measure of the timing the reference is off by (`lags`)
//! and two fixes (`Fix`): the reference shifted onto the mixture by those
//! lags, and the same with gains fitted over shorter blocks.

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

/// What is done before the mask (brief 70's fixes, fixed before their
/// runs).
#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Fix {
    /// The reference as placed by its time map (session 65).
    Off,
    /// The reference shifted onto the mixture by the lags `lags` measures.
    Align,
    /// `Align`, and gains fitted per block of `FINE_BLOCK_FRAMES`.
    AlignFine,
}

impl Fix {
    pub fn name(self) -> &'static str {
        match self {
            Fix::Off => "off",
            Fix::Align => "align",
            Fix::AlignFine => "align-fine",
        }
    }

    fn block_frames(self) -> usize {
        match self {
            Fix::Off | Fix::Align => BLOCK_FRAMES,
            Fix::AlignFine => FINE_BLOCK_FRAMES,
        }
    }
}

/// The residual's samples, and how well the gains fitted.
pub struct Residual {
    pub samples: Vec<f32>,
    /// Per band, the median over blocks of the spread of the mixture's
    /// power over the reference's at the points the gain is read from:
    /// the 70th over the 30th percentile, in dB. A record the reference
    /// matches up to a gain spreads little; one it is off from in time or
    /// in sound spreads more. `None` where no block has a gain.
    pub gain_spread_db: Vec<Option<f32>>,
}

/// Gains are fitted per block of this many frames (about 1 s at the
/// profile's hop): slow enough to hold many points, fast enough to
/// follow a fade of 2 s or more.
const BLOCK_FRAMES: usize = 63;
/// `Fix::AlignFine`'s blocks: about 0.5 s, still 16 points above the
/// median in a band one bin wide.
const FINE_BLOCK_FRAMES: usize = 32;
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

/// The residual of `mixture` once `reference` (the same span, placed by
/// its time map, at any level) is taken out under `rule`, after `fix`.
/// Both are mono at the profile's rate; the residual has the mixture's
/// length.
pub fn residual(
    mixture: &[f32],
    reference: &[f32],
    profile: &Profile,
    rule: Rule,
    fix: Fix,
) -> Residual {
    let mut reference = match fix {
        Fix::Off => reference.to_vec(),
        Fix::Align | Fix::AlignFine => {
            shifted(reference, &lags(mixture, reference, profile), profile)
        }
    };
    reference.resize(mixture.len(), 0.0);
    let mut spectra = spectra(mixture, profile);
    let mixture_powers: Vec<Vec<f32>> = spectra.iter().map(|frame| powers(frame)).collect();
    let reference_powers: Vec<Vec<f32>> = power_spectra(&reference, profile);
    let bands = bands(profile.fft_size / 2 + 1);
    let block_frames = fix.block_frames();
    let fit = band_gains(&mixture_powers, &reference_powers, &bands, block_frames);
    let over = rule.over as f32;
    for (frame, spectrum) in spectra.iter_mut().enumerate() {
        let block = &fit.gains[frame / block_frames];
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
    Residual {
        samples: overlap_add(&spectra, mixture, profile),
        gain_spread_db: fit.spread_db,
    }
}

struct GainFit {
    /// Per block (outer) and band (inner), as a ratio of powers.
    gains: Vec<Vec<f32>>,
    /// Per band: `Residual::gain_spread_db`.
    spread_db: Vec<Option<f32>>,
}

/// The reference's gain in the mixture, per block of `block_frames` frames
/// and band.
fn band_gains(
    mixture: &[Vec<f32>],
    reference: &[Vec<f32>],
    bands: &[Range<usize>],
    block_frames: usize,
) -> GainFit {
    let means: Vec<f32> = bands
        .iter()
        .map(|band| mean_power(reference, band.clone()))
        .collect();
    let mut spreads: Vec<Vec<f32>> = vec![Vec::new(); bands.len()];
    let gains = mixture
        .chunks(block_frames)
        .zip(reference.chunks(block_frames))
        .map(|(mixture, reference)| {
            bands
                .iter()
                .zip(&means)
                .zip(&mut spreads)
                .map(|((band, &mean), spreads)| {
                    if mean_power(reference, band.clone()) < SILENT_SHARE * mean {
                        return 0.0;
                    }
                    let Some(ratios) = strong_ratios(mixture, reference, band.clone()) else {
                        return 0.0;
                    };
                    let gain = quantile(ratios.clone(), GAIN_QUANTILE);
                    if gain > 0.0 {
                        let high = quantile(ratios, 1.0 - GAIN_QUANTILE);
                        spreads.push(10.0 * (high / gain).log10());
                    }
                    gain
                })
                .collect()
        })
        .collect();
    GainFit {
        gains,
        spread_db: spreads
            .into_iter()
            .map(|spreads| (!spreads.is_empty()).then(|| quantile(spreads, 0.5)))
            .collect(),
    }
}

/// The mixture's power over the reference's at the points of the band and
/// block where the reference's power is at least its median there; `None`
/// when there are fewer than `MIN_POINTS`. The gain is their
/// `GAIN_QUANTILE`.
fn strong_ratios(
    mixture: &[Vec<f32>],
    reference: &[Vec<f32>],
    band: Range<usize>,
) -> Option<Vec<f32>> {
    let points: Vec<(f32, f32)> = mixture
        .iter()
        .zip(reference)
        .flat_map(|(mixture, reference)| {
            band.clone().map(move |bin| (mixture[bin], reference[bin]))
        })
        .filter(|&(_, reference)| reference > 0.0)
        .collect();
    if points.len() < MIN_POINTS {
        return None;
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
    (ratios.len() >= MIN_POINTS).then_some(ratios)
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

/// The lags are measured over blocks of this many frames (8.2 s at the
/// profile's hop) every `LAG_STEP` frames.
const LAG_BLOCK: usize = 512;
const LAG_STEP: usize = 256;
/// Lags of up to this many frames either way (400 ms) are tried: more
/// than the lines' errors of 0127 and the seek's offset of Do Drop In.
const MAX_LAG: isize = 25;
/// Each block's lag is refined within this many frames of the line
/// through the blocks' first estimates...
const REFINE_LAG: isize = 3;
/// ...in steps of a hop over this many: the reference's spectra are also
/// taken from each such step into its first frame (2 ms at the profile's
/// hop).
const PHASES: usize = 8;
/// A block whose best correlation is under this is not used: the
/// reference does not follow the mixture there (a break, another record).
const MIN_CORRELATION: f64 = 0.3;
/// A block whose reference power is under this share of the excerpt's
/// mean is the record's silence.
const SILENT_BLOCK_SHARE: f64 = 1e-3;

/// How far the reference is from the mixture in time (brief 70): the
/// reference sample at mixture time `t` should be the one at `t - lag`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Lags {
    /// The blocks used, in order.
    pub blocks: Vec<BlockLag>,
    /// The blocks measured, used or not.
    pub measured: usize,
    /// The Theil-Sen line through the blocks' first estimates: its lag at
    /// the excerpt's start and its slope in parts per million.
    pub offset_ms: f64,
    pub drift_ppm: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockLag {
    /// The block's middle, from the excerpt's start.
    pub seconds: f64,
    pub lag_ms: f64,
    /// The mean over bands of the correlation of band levels (`levels`) at the
    /// lag found, and at no lag.
    pub correlation: f64,
    pub at_zero: f64,
}

impl Lags {
    /// The lag at `seconds`: linear between the blocks' lags, along the
    /// line's slope beyond the first and the last; 0 without blocks.
    pub fn at(&self, seconds: f64) -> f64 {
        let (Some(first), Some(last)) = (self.blocks.first(), self.blocks.last()) else {
            return 0.0;
        };
        let slope = self.drift_ppm * 1e-6;
        if seconds <= first.seconds {
            return first.lag_ms - slope * (first.seconds - seconds) * 1000.0;
        }
        if seconds >= last.seconds {
            return last.lag_ms + slope * (seconds - last.seconds) * 1000.0;
        }
        let after = self
            .blocks
            .iter()
            .position(|block| block.seconds > seconds)
            .unwrap_or(self.blocks.len() - 1)
            .max(1);
        let (before, after) = (&self.blocks[after - 1], &self.blocks[after]);
        let share = (seconds - before.seconds) / (after.seconds - before.seconds);
        before.lag_ms + share * (after.lag_ms - before.lag_ms)
    }
}

/// The lags between `reference` and `mixture` (same rate, the reference
/// placed by its time map): per block, the lag of the reference's band
/// levels (`levels`) that best correlates with the mixture's within `MAX_LAG`
/// frames; a Theil-Sen line through those; each block's lag refined
/// within `REFINE_LAG` frames of the line, in steps of a `PHASES`th of a
/// hop.
pub fn lags(mixture: &[f32], reference: &[f32], profile: &Profile) -> Lags {
    let mut reference = reference.to_vec();
    reference.resize(mixture.len(), 0.0);
    let mixture = power_spectra(mixture, profile);
    let bands = bands(profile.fft_size / 2 + 1);
    let mixture_levels = levels(&mixture, &bands);
    let step = profile.hop / PHASES;
    // Phase `p`'s frame `f` starts at the reference's sample
    // `p * step + f * hop`. Matched with the mixture's frame `f + k`, the
    // mixture holds that audio `k * hop - p * step` samples later: the
    // reference is early by that lag, as `Lags` counts it.
    let phases: Vec<Vec<Vec<f64>>> = (0..PHASES)
        .map(|phase| levels(&power_spectra(&reference[phase * step..], profile), &bands))
        .collect();
    let loudness: Vec<f64> = power_spectra(&reference, profile)
        .iter()
        .map(|frame| frame.iter().map(|&power| f64::from(power)).sum())
        .collect();
    let mean_loudness = loudness.iter().sum::<f64>() / loudness.len().max(1) as f64;
    let frames = mixture.len();
    let starts: Vec<usize> = if frames >= LAG_BLOCK {
        (0..=(frames - LAG_BLOCK) / LAG_STEP)
            .map(|block| block * LAG_STEP)
            .collect()
    } else if frames > 2 * MAX_LAG as usize {
        vec![0]
    } else {
        Vec::new()
    };
    let correlation = |start: usize, lag: isize, phase: usize| {
        let block = start..(start + LAG_BLOCK).min(frames);
        mean_correlation(&mixture_levels, &phases[phase], block, lag)
    };
    let first: Vec<(usize, f64)> = starts
        .iter()
        .filter(|&&start| {
            let end = (start + LAG_BLOCK).min(frames);
            let mean = loudness[start..end].iter().sum::<f64>() / (end - start) as f64;
            mean >= SILENT_BLOCK_SHARE * mean_loudness
        })
        .filter_map(|&start| {
            let (lag, best) = best_lag(-MAX_LAG..=MAX_LAG, 0, |lag| correlation(start, lag, 0));
            (best >= MIN_CORRELATION).then_some((start, lag as f64))
        })
        .collect();
    let middle = |start: usize| start as f64 + (LAG_BLOCK.min(frames) as f64) / 2.0;
    let (offset, slope) = theil_sen(
        &first
            .iter()
            .map(|&(start, lag)| (middle(start), lag))
            .collect::<Vec<_>>(),
    );
    let rate = f64::from(profile.sample_rate);
    let frame_seconds = profile.hop as f64 / rate;
    let blocks = first
        .iter()
        .map(|&(start, _)| {
            let centre = (offset + slope * middle(start)).round() as isize;
            let candidates = (centre - REFINE_LAG..=centre + REFINE_LAG)
                .flat_map(|lag| (0..PHASES).map(move |phase| (lag, phase)));
            let ((lag, phase), best) = best_lag(candidates, (centre, 0), |(lag, phase)| {
                correlation(start, lag, phase)
            });
            let samples = lag * profile.hop as isize - (phase * step) as isize;
            BlockLag {
                seconds: middle(start) * frame_seconds,
                lag_ms: samples as f64 / rate * 1000.0,
                correlation: best,
                at_zero: correlation(start, 0, 0),
            }
        })
        .collect();
    Lags {
        blocks,
        measured: starts.len(),
        offset_ms: offset * frame_seconds * 1000.0,
        drift_ppm: slope * 1e6,
    }
}

/// The candidate with the highest correlation, the first on a tie (and
/// `fallback` with none).
fn best_lag<T: Copy>(
    candidates: impl Iterator<Item = T>,
    fallback: T,
    correlation: impl Fn(T) -> f64,
) -> (T, f64) {
    candidates
        .map(|candidate| (candidate, correlation(candidate)))
        .fold((fallback, f64::NEG_INFINITY), |best, candidate| {
            if candidate.1 > best.1 {
                candidate
            } else {
                best
            }
        })
}

/// Per frame (outer) and band above the first (inner), the cube root of
/// the band's power: compressed so that loud frames do not decide alone,
/// but not so far as the logarithm, under which the window's leakage
/// around short sounds pulled the lag off by 5 ms in the tests. A gain
/// per band does not change a correlation.
fn levels(frames: &[Vec<f32>], bands: &[Range<usize>]) -> Vec<Vec<f64>> {
    frames
        .iter()
        .map(|frame| {
            bands[1..]
                .iter()
                .map(|band| {
                    let power: f64 = frame[band.clone()].iter().map(|&p| f64::from(p)).sum();
                    power.cbrt()
                })
                .collect()
        })
        .collect()
}

/// The mean over bands of Pearson's correlation between the mixture's
/// levels over `block` and the reference's `lag` frames earlier, over the
/// frames both have; bands that do not vary are left out.
fn mean_correlation(
    mixture: &[Vec<f64>],
    reference: &[Vec<f64>],
    block: Range<usize>,
    lag: isize,
) -> f64 {
    let pairs: Vec<(&Vec<f64>, &Vec<f64>)> = block
        .filter_map(|frame| {
            let other = usize::try_from(frame as isize - lag).ok()?;
            Some((&mixture[frame], reference.get(other)?))
        })
        .collect();
    let bands = mixture.first().map_or(0, Vec::len);
    let correlations: Vec<f64> = (0..bands)
        .filter_map(|band| {
            let n = pairs.len() as f64;
            let (x, y): (Vec<f64>, Vec<f64>) =
                pairs.iter().map(|(m, r)| (m[band], r[band])).unzip();
            let (mean_x, mean_y) = (x.iter().sum::<f64>() / n, y.iter().sum::<f64>() / n);
            let mut products = 0.0;
            let mut squares_x = 0.0;
            let mut squares_y = 0.0;
            for (x, y) in x.iter().zip(&y) {
                products += (x - mean_x) * (y - mean_y);
                squares_x += (x - mean_x) * (x - mean_x);
                squares_y += (y - mean_y) * (y - mean_y);
            }
            let scale = (squares_x * squares_y).sqrt();
            (scale > 1e-12).then(|| products / scale)
        })
        .collect();
    if correlations.is_empty() {
        return f64::NEG_INFINITY;
    }
    correlations.iter().sum::<f64>() / correlations.len() as f64
}

/// The Theil-Sen line through `points`: the median of the pairs' slopes,
/// and the median of the intercepts it leaves (robust to a block locked
/// on the wrong beat). One point gives a flat line; none, zero.
fn theil_sen(points: &[(f64, f64)]) -> (f64, f64) {
    let median = |mut values: Vec<f64>| {
        values.sort_by(f64::total_cmp);
        let middle = values.len() / 2;
        if values.len() % 2 == 1 {
            values[middle]
        } else {
            0.5 * (values[middle - 1] + values[middle])
        }
    };
    if points.is_empty() {
        return (0.0, 0.0);
    }
    let slopes: Vec<f64> = points
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            points[i + 1..]
                .iter()
                .map(move |b| (b.1 - a.1) / (b.0 - a.0))
        })
        .collect();
    let slope = if slopes.is_empty() {
        0.0
    } else {
        median(slopes)
    };
    let offset = median(points.iter().map(|(x, y)| y - slope * x).collect());
    (offset, slope)
}

/// `reference` moved by `lags`: the sample at mixture time `t` is the
/// reference's at `t - lag(t)`, the nearest sample (silence outside it).
fn shifted(reference: &[f32], lags: &Lags, profile: &Profile) -> Vec<f32> {
    let rate = f64::from(profile.sample_rate);
    (0..reference.len())
        .map(|n| {
            let seconds = n as f64 / rate;
            let source = n as f64 - lags.at(seconds) / 1000.0 * rate;
            let source = source.round();
            if source >= 0.0 && (source as usize) < reference.len() {
                reference[source as usize]
            } else {
                0.0
            }
        })
        .collect()
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

fn power_spectra(reference: &[f32], profile: &Profile) -> Vec<Vec<f32>> {
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
        let left = residual(&mixture, &a, &profile, Rule { over: 2.0 }, Fix::Off);
        let after = extract_peaks(&left.samples, &profile);
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
            Fix::Off,
        );
        let largest_error = mixture
            .iter()
            .zip(&left.samples)
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
        let reference = power_spectra(&a, &profile);
        let bands = bands(profile.fft_size / 2 + 1);
        let fit = band_gains(&mixture, &reference, &bands, BLOCK_FRAMES);
        // The band holding 300 Hz (bin 38.4), in a block in the middle.
        let band = bands.iter().position(|band| band.contains(&38)).unwrap();
        let gain = fit.gains[fit.gains.len() / 2][band];
        assert!(
            (gain - 0.25).abs() < 0.01,
            "an amplitude of 0.5 is a power of 0.25: {gain}"
        );
        let spread = fit.spread_db[band].unwrap();
        assert!(
            spread < 0.1,
            "a copy at another level spreads by {spread} dB"
        );
    }

    /// Tone bursts at `hz` as `bursts` makes them, starting at irregular
    /// times (0.3 to 0.9 s apart, from a fixed generator), so that no
    /// shift of a few hundred milliseconds looks like another.
    fn irregular(hz: f32, seconds: f32, amplitude: f32, seed: u64) -> Vec<f32> {
        let length = (seconds * RATE) as usize;
        let burst = (0.08 * RATE) as usize;
        let mut samples = vec![0.0; length];
        let mut state = seed;
        let mut start = (0.5 * RATE) as usize;
        while start + burst < length {
            for n in 0..burst {
                let envelope =
                    0.5 - 0.5 * (2.0 * std::f32::consts::PI * n as f32 / burst as f32).cos();
                let t = (start + n) as f32 / RATE;
                samples[start + n] +=
                    amplitude * envelope * (2.0 * std::f32::consts::PI * hz * t).sin();
            }
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            let gap = 0.3 + 0.6 * ((state >> 33) as f32 / (1_u64 << 31) as f32);
            start += (gap * RATE) as usize;
        }
        samples
    }

    /// `samples` later by `delay` samples (silence before), same length.
    fn delayed(samples: &[f32], delay: usize) -> Vec<f32> {
        let mut later = vec![0.0; delay];
        later.extend_from_slice(&samples[..samples.len() - delay]);
        later
    }

    fn irregular_records() -> (Vec<f32>, Vec<f32>) {
        let a = added(
            &irregular(300.0, 40.0, 0.3, 1),
            &irregular(1200.0, 40.0, 0.3, 2),
        );
        let quiet = 0.3 * 10_f32.powf(-9.0 / 20.0);
        let b = added(
            &irregular(700.0, 40.0, quiet, 3),
            &irregular(2000.0, 40.0, quiet, 4),
        );
        (a, b)
    }

    #[test]
    fn the_lags_find_a_reference_placed_125_ms_early() {
        let (a, b) = irregular_records();
        // The mixture holds A 1,000 samples (125 ms) later than the
        // reference has it.
        let mixture = added(&scaled(&delayed(&a, 1000), 0.5), &b);
        let lags = lags(&mixture, &a, &Profile::CURRENT);
        assert!(lags.measured >= 8, "{} blocks", lags.measured);
        assert!(!lags.blocks.is_empty());
        for block in &lags.blocks {
            assert!(
                (block.lag_ms - 125.0).abs() < 4.0,
                "at {} s: {} ms",
                block.seconds,
                block.lag_ms
            );
            assert!(block.correlation > block.at_zero);
        }
        assert!(lags.drift_ppm.abs() < 100.0, "{} ppm", lags.drift_ppm);
    }

    #[test]
    fn aligning_takes_out_a_record_whose_reference_is_off_in_time() {
        let (a, b) = irregular_records();
        let mixture = added(&scaled(&delayed(&a, 1000), 0.5), &b);
        let profile = Profile::CURRENT;
        let count = |samples: &[f32], hz: f32| near(&extract_peaks(samples, &profile), hz);
        let off = residual(&mixture, &a, &profile, Rule { over: 2.0 }, Fix::Off);
        let aligned = residual(&mixture, &a, &profile, Rule { over: 2.0 }, Fix::Align);
        for hz in [300.0, 1200.0] {
            assert!(
                count(&off.samples, hz) >= 20,
                "unaligned, A stays at {hz} Hz"
            );
            // A millisecond off still leaves a peak or two at a burst's
            // edges (with the exact delay, at most one).
            assert!(
                count(&aligned.samples, hz) <= 5,
                "aligned, A goes at {hz} Hz"
            );
        }
        for hz in [700.0, 2000.0] {
            assert!(
                count(&aligned.samples, hz) + 2 >= count(&mixture, hz),
                "B's peaks at {hz} Hz kept"
            );
        }
    }

    #[test]
    fn the_lag_between_blocks_is_linear_and_follows_the_drift_beyond_them() {
        let block = |seconds: f64, lag_ms: f64| BlockLag {
            seconds,
            lag_ms,
            correlation: 0.9,
            at_zero: 0.1,
        };
        let lags = Lags {
            blocks: vec![block(10.0, 20.0), block(20.0, 40.0)],
            measured: 2,
            offset_ms: 0.0,
            drift_ppm: 2000.0,
        };
        assert!((lags.at(15.0) - 30.0).abs() < 1e-9);
        // 2,000 ppm is 2 ms a second.
        assert!((lags.at(5.0) - 10.0).abs() < 1e-9);
        assert!((lags.at(25.0) - 50.0).abs() < 1e-9);
        assert_eq!(Lags::default().at(12.0), 0.0);
    }

    #[test]
    fn the_theil_sen_line_ignores_a_block_on_the_wrong_beat() {
        let points = [(0.0, 1.0), (1.0, 2.0), (2.0, 3.0), (3.0, 40.0), (4.0, 5.0)];
        let (offset, slope) = theil_sen(&points);
        assert!((slope - 1.0).abs() < 1e-9 && (offset - 1.0).abs() < 1e-9);
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

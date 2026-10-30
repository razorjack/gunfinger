//! Short-time Fourier transform producing log-power frames.

use std::f32::consts::PI;
use std::ops::Range;

use realfft::RealFftPlanner;
use realfft::num_complex::Complex;

use crate::profile::Profile;

/// Calls `visit` with the log power spectrum (in dB) of each STFT frame, in
/// order. Frame `i` covers samples `i * hop .. i * hop + fft_size`; a trailing
/// partial frame is dropped.
///
/// Frames are produced one at a time so that a two-hour mix never needs its
/// whole spectrogram in memory.
pub fn for_each_frame(samples: &[f32], profile: &Profile, mut visit: impl FnMut(&[f32])) {
    let mut power_db = Vec::new();
    for_each_power_frame(Signal::Mono(samples), profile, |power| {
        power_db.clear();
        power_db.extend(power.iter().map(|&power| decibels(power)));
        visit(&power_db);
    });
}

/// What the STFT analyses: one channel, or two whose powers are added.
#[derive(Debug, Clone, Copy)]
pub enum Signal<'a> {
    /// One channel: FFmpeg's downmix, today's front end.
    Mono(&'a [f32]),
    /// Two channels of the same length, analysed apart; each bin's power is
    /// `|X_L|^2 + |X_R|^2` (a variant under evaluation, experiment 0245 and
    /// after). With the downmix `M = (L + R) / sqrt(2)` and the side
    /// `S = (L - R) / sqrt(2)`, an orthogonal change of channels, that is
    /// `|X_M|^2 + |X_S|^2`: the downmix's power plus the side's, so equal
    /// channels give the downmix's power, and inverting either channel
    /// (swapping `M` and `S`) changes nothing.
    PowerSum { left: &'a [f32], right: &'a [f32] },
}

impl<'a> Signal<'a> {
    /// Samples per channel.
    pub fn len(self) -> usize {
        match self {
            Signal::Mono(samples) => samples.len(),
            Signal::PowerSum { left, right } => left.len().min(right.len()),
        }
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    /// The samples `range` of each channel.
    pub fn slice(self, range: Range<usize>) -> Signal<'a> {
        match self {
            Signal::Mono(samples) => Signal::Mono(&samples[range]),
            Signal::PowerSum { left, right } => Signal::PowerSum {
                left: &left[range.clone()],
                right: &right[range],
            },
        }
    }
}

/// Like `for_each_frame`, with each bin's power before it is turned into
/// decibels: `decibels` of each value is what `for_each_frame` gives.
pub fn for_each_power_frame(signal: Signal, profile: &Profile, mut visit: impl FnMut(&[f32])) {
    let size = profile.fft_size;
    let window = hann(size);
    let fft = RealFftPlanner::<f32>::new().plan_fft_forward(size);
    let mut frame = fft.make_input_vec();
    let mut spectrum = fft.make_output_vec();
    let mut scratch = fft.make_scratch_vec();
    let mut powers = vec![0.0; spectrum.len()];
    let mut transform = |samples: &[f32], spectrum: &mut [Complex<f32>]| {
        for (n, value) in frame.iter_mut().enumerate() {
            *value = samples[n] * window[n];
        }
        let transformed = fft.process_with_scratch(&mut frame, spectrum, &mut scratch);
        debug_assert!(transformed.is_ok(), "buffers come from the plan");
    };

    let mut start = 0;
    while start + size <= signal.len() {
        let frames = start..start + size;
        match signal {
            Signal::Mono(samples) => {
                transform(&samples[frames], &mut spectrum);
                for (power, bin) in powers.iter_mut().zip(&spectrum) {
                    // The tiny offset keeps digital silence finite (-200 dB).
                    *power = bin.norm_sqr() + 1e-20;
                }
            }
            Signal::PowerSum { left, right } => {
                transform(&left[frames.clone()], &mut spectrum);
                for (power, bin) in powers.iter_mut().zip(&spectrum) {
                    *power = bin.norm_sqr();
                }
                transform(&right[frames], &mut spectrum);
                for (power, bin) in powers.iter_mut().zip(&spectrum) {
                    *power = *power + bin.norm_sqr() + 1e-20;
                }
            }
        }
        visit(&powers);
        start += profile.hop;
    }
}

/// A bin's power in decibels. It never decreases as power grows
/// (`decibels_never_decrease_as_power_grows`), so comparing powers gives
/// the order of their levels, apart from powers whose levels round to the
/// same decibels.
pub fn decibels(power: f32) -> f32 {
    10.0 * power.log10()
}

/// The periodic Hann window, `w[n] = 0.5 - 0.5 cos(2πn / N)`.
fn hann(size: usize) -> Vec<f32> {
    (0..size)
        .map(|n| 0.5 - 0.5 * (2.0 * PI * n as f32 / size as f32).cos())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(frequency: f32, profile: &Profile, seconds: f32) -> Vec<f32> {
        let rate = profile.sample_rate as f32;
        (0..(seconds * rate) as usize)
            .map(|n| (2.0 * PI * frequency * n as f32 / rate).sin())
            .collect()
    }

    #[test]
    fn a_sine_concentrates_power_in_its_bin() {
        let profile = Profile::CURRENT;
        let bin = 100;
        let frequency = bin as f32 * profile.bin_hz() as f32;
        let mut loudest = Vec::new();

        for_each_frame(&sine(frequency, &profile, 1.0), &profile, |frame| {
            let (index, _) = frame
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.total_cmp(b.1))
                .unwrap();
            loudest.push(index);
        });

        assert!(!loudest.is_empty());
        assert!(loudest.iter().all(|&index| index == bin));
    }

    /// Every non-negative power, infinity included, checked on the machine
    /// running the tests, since the logarithm comes from the platform's
    /// maths library. The peak picker's exactness rests on this.
    #[test]
    fn decibels_never_decrease_as_power_grows() {
        let last = f32::INFINITY.to_bits();
        let threads = std::thread::available_parallelism().map_or(4, std::num::NonZero::get) as u32;
        let share = last / threads + 1;
        let decreases: u32 = std::thread::scope(|scope| {
            let workers: Vec<_> = (0..threads)
                .map(|thread| {
                    scope.spawn(move || {
                        let first = thread * share;
                        let end = (first + share).min(last);
                        let mut decreases = 0;
                        let mut previous = decibels(f32::from_bits(first));
                        for bits in first + 1..=end {
                            let level = decibels(f32::from_bits(bits));
                            decreases += u32::from(level < previous);
                            previous = level;
                        }
                        decreases
                    })
                })
                .collect();
            workers
                .into_iter()
                .map(|worker| worker.join().unwrap())
                .sum()
        });

        assert_eq!(decreases, 0);
        assert_eq!(decibels(0.0), f32::NEG_INFINITY);
        assert_eq!(decibels(f32::INFINITY), f32::INFINITY);
    }

    #[test]
    fn frames_step_by_the_hop_and_drop_the_partial_tail() {
        let profile = Profile::CURRENT;
        let samples = vec![0.0; profile.fft_size + 3 * profile.hop + 1];
        let mut frames = 0;

        for_each_frame(&samples, &profile, |_| frames += 1);

        assert_eq!(frames, 4);
    }
}

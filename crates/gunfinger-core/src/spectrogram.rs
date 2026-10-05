//! Short-time Fourier transform producing log-power frames.

use std::f32::consts::PI;

use realfft::RealFftPlanner;

use crate::profile::Profile;

/// Calls `visit` with the log power spectrum (in dB) of each STFT frame, in
/// order. Frame `i` covers samples `i * hop .. i * hop + fft_size`; a trailing
/// partial frame is dropped.
///
/// Frames are produced one at a time so that a two-hour mix never needs its
/// whole spectrogram in memory.
pub fn for_each_frame(samples: &[f32], profile: &Profile, mut visit: impl FnMut(&[f32])) {
    let size = profile.fft_size;
    let window = hann(size);
    let fft = RealFftPlanner::<f32>::new().plan_fft_forward(size);
    let mut frame = fft.make_input_vec();
    let mut spectrum = fft.make_output_vec();
    let mut power_db = vec![0.0; spectrum.len()];

    let mut start = 0;
    while start + size <= samples.len() {
        for (n, value) in frame.iter_mut().enumerate() {
            *value = samples[start + n] * window[n];
        }
        let transformed = fft.process(&mut frame, &mut spectrum);
        debug_assert!(transformed.is_ok(), "buffers come from the plan");
        for (power, bin) in power_db.iter_mut().zip(&spectrum) {
            // The tiny offset keeps digital silence finite (-200 dB).
            *power = 10.0 * (bin.norm_sqr() + 1e-20).log10();
        }
        visit(&power_db);
        start += profile.hop;
    }
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

    #[test]
    fn frames_step_by_the_hop_and_drop_the_partial_tail() {
        let profile = Profile::CURRENT;
        let samples = vec![0.0; profile.fft_size + 3 * profile.hop + 1];
        let mut frames = 0;

        for_each_frame(&samples, &profile, |_| frames += 1);

        assert_eq!(frames, 4);
    }
}

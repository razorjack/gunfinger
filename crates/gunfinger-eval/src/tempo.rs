//! Tempo and beat phase of rendered audio, for beatmatched blends.
//!
//! The onset envelope is the positive spectral flux (the summed rise of
//! log power across bins from one frame to the next) at 100 frames per
//! second. Its autocorrelation peaks at the beat period; the peak near
//! eight beats gives the period to a fraction of a percent. Two envelopes
//! are lined up by the lag that maximises their correlation.

use gunfinger_core::profile::Profile;
use gunfinger_core::spectrogram::for_each_frame;

/// Envelope frames per second.
pub const ENVELOPE_RATE: f64 = 100.0;
/// Tempos looked for, in beats per minute: breakbeat to fast drum & bass.
const SLOWEST_BPM: f64 = 120.0;
const FASTEST_BPM: f64 = 190.0;
/// The period is refined on the autocorrelation peak this many beats out.
const REFINE_BEATS: f64 = 8.0;

/// The onset envelope of mono `samples` at `rate` Hz.
pub fn onset_envelope(samples: &[f32], rate: u32) -> Vec<f32> {
    let hop = (f64::from(rate) / ENVELOPE_RATE).round() as usize;
    let profile = Profile {
        sample_rate: rate,
        fft_size: hop.next_power_of_two() * 2,
        hop,
        ..Profile::CURRENT
    };
    let mut previous: Vec<f32> = Vec::new();
    let mut envelope = Vec::new();
    for_each_frame(samples, &profile, |power_db| {
        let flux = if previous.is_empty() {
            0.0
        } else {
            power_db
                .iter()
                .zip(&previous)
                .map(|(now, before)| (now - before).max(0.0))
                .sum()
        };
        envelope.push(flux);
        previous.clear();
        previous.extend_from_slice(power_db);
    });
    let mean = envelope.iter().sum::<f32>() / envelope.len().max(1) as f32;
    envelope.iter().map(|value| value - mean).collect()
}

/// The beat period in seconds, or `None` when the envelope is too short.
pub fn beat_period(envelope: &[f32]) -> Option<f64> {
    let shortest = (60.0 / FASTEST_BPM * ENVELOPE_RATE).floor() as usize;
    let longest = (60.0 / SLOWEST_BPM * ENVELOPE_RATE).ceil() as usize;
    let beat = (shortest..=longest).max_by(|&a, &b| {
        correlation(envelope, envelope, a).total_cmp(&correlation(envelope, envelope, b))
    })?;
    let around = (REFINE_BEATS * beat as f64).round() as usize;
    let spread = around / 25;
    if around + spread + 1 >= envelope.len() {
        return Some(beat as f64 / ENVELOPE_RATE);
    }
    let peak = (around - spread..=around + spread).max_by(|&a, &b| {
        correlation(envelope, envelope, a).total_cmp(&correlation(envelope, envelope, b))
    })?;
    // A parabola through the peak and its neighbours places it between
    // frames.
    let (left, middle, right) = (
        correlation(envelope, envelope, peak - 1),
        correlation(envelope, envelope, peak),
        correlation(envelope, envelope, peak + 1),
    );
    let curvature = left - 2.0 * middle + right;
    let refined = if curvature < 0.0 {
        peak as f64 + 0.5 * (left - right) / curvature
    } else {
        peak as f64
    };
    Some(refined / REFINE_BEATS / ENVELOPE_RATE)
}

/// The delay, in envelope frames below `longest`, by which `later` best
/// follows `earlier`: `later[i + lag]` lines up with `earlier[i]`.
pub fn best_lag(earlier: &[f32], later: &[f32], longest: usize) -> usize {
    (0..longest)
        .max_by(|&a, &b| correlation(earlier, later, a).total_cmp(&correlation(earlier, later, b)))
        .unwrap_or(0)
}

/// The mean of `a[i] * b[i + lag]` over the overlap.
fn correlation(a: &[f32], b: &[f32], lag: usize) -> f64 {
    let overlap = a.len().min(b.len().saturating_sub(lag));
    if overlap == 0 {
        return f64::NEG_INFINITY;
    }
    let sum: f64 = (0..overlap)
        .map(|i| f64::from(a[i]) * f64::from(b[i + lag]))
        .sum();
    sum / overlap as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Clicks at `bpm`, starting `delay` seconds in.
    fn clicks(bpm: f64, delay: f64, seconds: f64, rate: u32) -> Vec<f32> {
        let mut samples = vec![0.0; (seconds * f64::from(rate)) as usize];
        let mut at = delay;
        while at < seconds {
            let start = (at * f64::from(rate)) as usize;
            for (offset, sample) in samples.iter_mut().skip(start).take(200).enumerate() {
                *sample = (offset as f32 * 0.7).sin() * (1.0 - offset as f32 / 200.0);
            }
            at += 60.0 / bpm;
        }
        samples
    }

    #[test]
    fn a_click_track_gives_its_tempo() {
        let rate = 44_100;
        let envelope = onset_envelope(&clicks(172.0, 0.1, 30.0, rate), rate);

        let period = beat_period(&envelope).unwrap();

        assert!((60.0 / period - 172.0).abs() < 0.5, "{} bpm", 60.0 / period);
    }

    #[test]
    fn the_lag_lines_up_two_click_tracks() {
        let rate = 44_100;
        let first = onset_envelope(&clicks(170.0, 0.0, 20.0, rate), rate);
        let second = onset_envelope(&clicks(170.0, 0.15, 20.0, rate), rate);

        let lag = best_lag(&first, &second, 35);

        assert!((lag as f64 - 15.0).abs() <= 1.0, "lag {lag}");
    }
}

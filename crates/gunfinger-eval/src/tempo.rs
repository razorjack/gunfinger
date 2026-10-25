//! Tempo and beat phase of rendered audio, for beatmatched blends.
//!
//! The onset envelope is the positive spectral flux (the summed rise of
//! log power across bins from one frame to the next) at 100 frames per
//! second. Its autocorrelation peaks at the beat period; the peak near
//! eight beats gives the period to a fraction of a percent. Two envelopes
//! are lined up by the lag that maximises their correlation.
//!
//! A drop is where the music comes back in full after a breakdown: the
//! largest rise of loudness, moved to the beat (`Structure`).

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

/// Loudness frames per second, for finding drops.
const LOUDNESS_RATE: f64 = 2.0;
/// The rise at a moment: the mean loudness over this many seconds after it
/// less the mean over as many before it.
const RISE_SECONDS: f64 = 8.0;
/// A drop moves to the beat within half a period whose next beats hold the
/// most onsets, counted over this many beats.
const DROP_BEATS: usize = 16;

/// What finds a record's drops: the rise of loudness every half second and
/// the onset envelope, at native speed, with the beat period.
pub struct Structure {
    /// `rise[i]` belongs to file time `i / LOUDNESS_RATE`; `None` within
    /// `RISE_SECONDS` of either end.
    rise: Vec<Option<f64>>,
    envelope: Vec<f32>,
    period: f64,
}

impl Structure {
    /// The structure of mono `samples` at `rate` Hz with beat `period`
    /// seconds.
    pub fn of(samples: &[f32], rate: u32, period: f64) -> Structure {
        let frame = (f64::from(rate) / LOUDNESS_RATE).round() as usize;
        let loudness: Vec<f64> = samples
            .chunks_exact(frame.max(1))
            .map(|chunk| {
                let power =
                    chunk.iter().map(|x| f64::from(*x).powi(2)).sum::<f64>() / chunk.len() as f64;
                10.0 * (power + 1e-10).log10()
            })
            .collect();
        Structure {
            rise: rises(&loudness),
            envelope: onset_envelope(samples, rate),
            period,
        }
    }

    /// The drop between file times `from` and `to`: the moment of the
    /// largest rise, moved to the beat, or `None` when the window holds no
    /// rise.
    pub fn drop_within(&self, from: f64, to: f64) -> Option<f64> {
        let first = (from * LOUDNESS_RATE).ceil().max(0.0) as usize;
        let last = (to * LOUDNESS_RATE).floor();
        if last < first as f64 {
            return None;
        }
        let rise = (first..=last as usize)
            .filter_map(|i| Some((i, (*self.rise.get(i)?)?)))
            .max_by(|a, b| a.1.total_cmp(&b.1))?
            .0 as f64
            / LOUDNESS_RATE;
        Some(self.on_the_beat(rise, from, to))
    }

    /// The position within half a beat of `at`, and within `from..=to`,
    /// whose next `DROP_BEATS` beats hold the most onsets.
    fn on_the_beat(&self, at: f64, from: f64, to: f64) -> f64 {
        let half = self.period / 2.0;
        let first = ((at - half).max(from) * ENVELOPE_RATE).ceil().max(0.0) as usize;
        let last = ((at + half).min(to) * ENVELOPE_RATE).floor() as usize;
        let onsets = |frame: usize| -> f64 {
            (0..DROP_BEATS)
                .filter_map(|beat| {
                    let at = frame + (beat as f64 * self.period * ENVELOPE_RATE).round() as usize;
                    self.envelope.get(at).map(|value| f64::from(*value))
                })
                .sum()
        };
        (first..=last)
            .max_by(|&a, &b| onsets(a).total_cmp(&onsets(b)))
            .map_or(at, |frame| frame as f64 / ENVELOPE_RATE)
    }
}

#[cfg(test)]
impl Structure {
    /// A record of `seconds` whose only rises are at `drops`, with a beat
    /// every `period` from its start.
    pub fn with_drops(drops: &[f64], seconds: f64, period: f64) -> Structure {
        let frames = (seconds * LOUDNESS_RATE) as usize;
        let rise = (0..frames)
            .map(|i| {
                let at = i as f64 / LOUDNESS_RATE;
                Some(if drops.iter().any(|drop| (drop - at).abs() < 0.25) {
                    10.0
                } else {
                    0.0
                })
            })
            .collect();
        let mut envelope = vec![0.0; (seconds * ENVELOPE_RATE) as usize];
        let mut beat = 0.0;
        while beat < seconds {
            if let Some(value) = envelope.get_mut((beat * ENVELOPE_RATE).round() as usize) {
                *value = 1.0;
            }
            beat += period;
        }
        Structure {
            rise,
            envelope,
            period,
        }
    }
}

/// The rise of `loudness` at each frame, where both sides of it fit.
fn rises(loudness: &[f64]) -> Vec<Option<f64>> {
    let side = (RISE_SECONDS * LOUDNESS_RATE) as usize;
    let mean = |frames: &[f64]| frames.iter().sum::<f64>() / frames.len() as f64;
    (0..loudness.len())
        .map(|i| {
            (i >= side && i + side <= loudness.len())
                .then(|| mean(&loudness[i..i + side]) - mean(&loudness[i - side..i]))
        })
        .collect()
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
    fn a_drop_is_the_rise_of_loudness_on_the_beat() {
        let rate = 8_000;
        let period: f64 = 60.0 / 172.0;
        let mut samples = clicks(172.0, 0.1, 80.0, rate);
        // Quiet until the beat at or after 40.5 s, then loud.
        let drop_beat = ((40.5 - 0.1) / period).ceil();
        let drop = 0.1 + drop_beat * period;
        for (index, sample) in samples.iter_mut().enumerate() {
            if (index as f64) < drop * f64::from(rate) - 2.0 {
                *sample *= 0.05;
            }
        }
        let structure = Structure::of(&samples, rate, period);

        let found = structure.drop_within(10.0, 70.0).unwrap();

        // An onset enters the envelope's frame up to a window early (32 ms
        // here), alike for every record, so drops still line up.
        assert!((found - drop).abs() <= 0.04, "{found} for {drop}");
        // Without the rise in its window, the strongest rise there, still
        // inside the window and on a beat.
        let elsewhere = structure.drop_within(50.0, 60.0).unwrap();
        assert!((50.0..=60.0).contains(&elsewhere), "{elsewhere}");
        let beats = (elsewhere - 0.1) / period;
        assert!(
            (beats - beats.round()).abs() * period <= 0.04,
            "{elsewhere}"
        );
        assert_eq!(structure.drop_within(30.0, 29.0), None);
        assert_eq!(structure.drop_within(0.0, 7.0), None);
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

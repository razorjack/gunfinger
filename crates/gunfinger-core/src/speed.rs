//! Playback speed: the ladder of assumed speeds and analysis at a given
//! speed.
//!
//! A turntable resamples the record: at speed `s` every frequency is
//! multiplied by `s` and every duration divided by it. Hashes keep absolute
//! frequency, so the query is analysed once per rung of a ladder of assumed
//! speeds; on the rung nearest the true speed its hashes meet the reference
//! hashes.
//!
//! Key lock (a CDJ's master tempo, a digital DJ's time stretch) changes
//! tempo only: durations are divided by the tempo, frequencies stay. The
//! turntable ladder cannot meet such audio past about 1% (experiment 0009);
//! key-locked rungs rescale time alone.

use std::ops::Range;

use crate::hash::Point;
use crate::peaks::extract_peaks_in;
use crate::profile::Profile;

/// Playback speed relative to the recording's native speed: a record pitched
/// up 3% plays at 1.03.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SpeedRatio(pub f64);

/// Turntables reach ±8%.
const SLOWEST: f64 = 0.92;
const FASTEST: f64 = 1.08;
/// Hash survival falls quickly with residual speed error; at ±0.2% (half
/// this step) about 45% of clean hashes survive (experiment 0001).
pub(crate) const STEP: f64 = 0.004;

/// How a record was played: on a turntable, pitch and tempo move together;
/// under key lock, tempo moves alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Playback {
    Turntable,
    KeyLocked,
}

/// One assumed way the query was played.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rung {
    /// Pitch and tempo together, at this speed.
    Turntable(SpeedRatio),
    /// Tempo only, at this ratio; pitch unchanged.
    KeyLocked(SpeedRatio),
}

impl Rung {
    /// The ratio of time: reference duration over query duration.
    pub fn speed(self) -> SpeedRatio {
        match self {
            Rung::Turntable(speed) | Rung::KeyLocked(speed) => speed,
        }
    }

    pub fn playback(self) -> Playback {
        match self {
            Rung::Turntable(_) => Playback::Turntable,
            Rung::KeyLocked(_) => Playback::KeyLocked,
        }
    }

    /// The query's peaks in reference coordinates under this assumption.
    pub fn points(self, samples: &[f32], profile: &Profile) -> Vec<Point> {
        self.points_in(samples, profile, 0..usize::MAX)
    }

    /// Like `points`, for the peaks in the STFT frames `frames` of this
    /// rung's analysis only.
    pub fn points_in(self, samples: &[f32], profile: &Profile, frames: Range<usize>) -> Vec<Point> {
        match self {
            Rung::Turntable(speed) => points_at_speed_in(samples, profile, speed, frames),
            Rung::KeyLocked(tempo) => points_at_tempo_in(samples, profile, tempo, frames),
        }
    }

    /// The hop of this rung's analysis in samples: the profile's, shrunk by
    /// the speed or tempo. One STFT frame of it spans `hop / profile.hop`
    /// frames of the query.
    pub fn hop(self, profile: &Profile) -> usize {
        (profile.hop as f64 / self.speed().0).round() as usize
    }
}

/// The ladder's speeds as reports record them; which ladders were searched
/// is the playback.
pub fn design() -> String {
    format!("ladder={SLOWEST}..{FASTEST}/{STEP}")
}

/// The assumed speeds searched, slowest first.
pub fn ladder() -> Vec<Rung> {
    speeds().map(Rung::Turntable).collect()
}

/// The same speeds as tempo ratios under key lock.
pub fn key_lock_ladder() -> Vec<Rung> {
    speeds().map(Rung::KeyLocked).collect()
}

fn speeds() -> impl Iterator<Item = SpeedRatio> {
    let rungs = ((FASTEST - SLOWEST) / STEP).round() as u32;
    (0..=rungs).map(|rung| SpeedRatio(SLOWEST + f64::from(rung) * STEP))
}

/// The peaks of `samples` in reference coordinates, assuming the audio plays
/// at `speed`.
///
/// The STFT window and hop shrink by the speed, so each frame covers the
/// stretch of music a reference frame covers at native speed and each bin
/// the same musical frequency. Peaks are then picked on the same grid as the
/// reference's. Transforming the coordinates of peaks picked on the query's
/// own grid instead loses about half the surviving hashes (experiment 0001).
pub fn points_at_speed(samples: &[f32], profile: &Profile, speed: SpeedRatio) -> Vec<Point> {
    points_at_speed_in(samples, profile, speed, 0..usize::MAX)
}

fn points_at_speed_in(
    samples: &[f32],
    profile: &Profile,
    speed: SpeedRatio,
    frames: Range<usize>,
) -> Vec<Point> {
    let fft_size = (profile.fft_size as f64 / speed.0).round() as usize;
    let hop = Rung::Turntable(speed).hop(profile);
    let scaled = Profile {
        fft_size,
        hop,
        // Above the query's Nyquist frequency there is nothing to see.
        max_bin: profile.max_bin.min(fft_size / 2),
        ..*profile
    };
    // Rounding the window and hop to whole samples is corrected here.
    let bin_scale = profile.fft_size as f64 / (fft_size as f64 * speed.0);
    let frame_scale = hop as f64 * speed.0 / profile.hop as f64;
    extract_peaks_in(samples, &scaled, frames)
        .iter()
        .map(|peak| Point {
            frame: peak.frame * frame_scale,
            bin: (f64::from(peak.bin) * bin_scale) as f32,
        })
        .collect()
}

/// The peaks of `samples` in reference coordinates, assuming key-locked
/// playback at `tempo`: only the hop shrinks, so frames follow the music
/// while the window and the bins keep their native frequencies.
pub fn points_at_tempo(samples: &[f32], profile: &Profile, tempo: SpeedRatio) -> Vec<Point> {
    points_at_tempo_in(samples, profile, tempo, 0..usize::MAX)
}

fn points_at_tempo_in(
    samples: &[f32],
    profile: &Profile,
    tempo: SpeedRatio,
    frames: Range<usize>,
) -> Vec<Point> {
    let hop = Rung::KeyLocked(tempo).hop(profile);
    let scaled = Profile { hop, ..*profile };
    let frame_scale = hop as f64 * tempo.0 / profile.hop as f64;
    extract_peaks_in(samples, &scaled, frames)
        .iter()
        .map(|peak| Point {
            frame: peak.frame * frame_scale,
            bin: peak.bin,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn the_ladder_spans_eight_percent_either_way() {
        let ladder = ladder();

        assert_eq!(ladder.len(), 41);
        assert!((ladder[0].speed().0 - 0.92).abs() < 1e-9);
        assert!((ladder[20].speed().0 - 1.0).abs() < 1e-9);
        assert!((ladder[40].speed().0 - 1.08).abs() < 1e-9);
    }

    #[test]
    fn a_tone_played_fast_is_found_at_its_native_frequency() {
        let profile = Profile::CURRENT;
        let speed = 1.05;
        let native_bin = 200.0;
        let rate = profile.sample_rate as f32;
        let played = native_bin * profile.bin_hz() as f32 * speed as f32;
        let samples: Vec<f32> = (0..2 * profile.sample_rate as usize)
            .map(|n| 0.5 * (2.0 * PI * played * n as f32 / rate).sin())
            .collect();

        let points = points_at_speed(&samples, &profile, SpeedRatio(speed));

        assert!(!points.is_empty());
        for point in points {
            assert!(
                (point.bin - native_bin).abs() < 0.1,
                "found bin {}",
                point.bin
            );
        }
    }
}

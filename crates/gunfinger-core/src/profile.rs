//! The front-end profile: every parameter that shapes the stored peaks.
//!
//! Peaks extracted under different profiles are not comparable, so each peak
//! record carries the profile's identifier and is re-extracted when it
//! changes.

/// Parameters of the spectrogram and the peak picker.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Profile {
    /// Analysis sample rate in Hz.
    pub sample_rate: u32,
    /// STFT window length in samples (Hann window).
    pub fft_size: usize,
    /// Samples between successive STFT frames.
    pub hop: usize,
    /// Lowest and one-past-highest frequency bin searched for peaks.
    pub min_bin: usize,
    pub max_bin: usize,
    /// Half-sizes of the local-maximum neighbourhood: a peak is the largest
    /// value within `±neighbourhood_frames` frames and `±neighbourhood_bins`
    /// bins.
    pub neighbourhood_frames: usize,
    pub neighbourhood_bins: usize,
    /// Peaks quieter than this (in dB of STFT power) are noise floor.
    pub floor_db: f32,
}

impl Profile {
    pub const CURRENT: Profile = Profile {
        sample_rate: 8000,
        fft_size: 1024,
        hop: 128,
        // 39 Hz to 3.9 kHz: below the lowest bass notes is rumble, above is
        // the resampler's transition band.
        min_bin: 5,
        max_bin: 500,
        neighbourhood_frames: 12,
        neighbourhood_bins: 12,
        floor_db: -10.0,
    };

    /// A stable textual identifier, stored in every peak record.
    pub fn id(&self) -> String {
        format!(
            "peaks-v1 rate={} fft={} hop={} bins={}..{} nbhd={}x{} floor={}",
            self.sample_rate,
            self.fft_size,
            self.hop,
            self.min_bin,
            self.max_bin,
            self.neighbourhood_frames,
            self.neighbourhood_bins,
            self.floor_db,
        )
    }

    pub fn frames_per_second(&self) -> f64 {
        f64::from(self.sample_rate) / self.hop as f64
    }

    pub fn seconds(&self, frames: f64) -> f64 {
        frames / self.frames_per_second()
    }

    pub fn frames(&self, seconds: f64) -> f64 {
        seconds * self.frames_per_second()
    }

    pub fn bin_hz(&self) -> f64 {
        f64::from(self.sample_rate) / self.fft_size as f64
    }
}

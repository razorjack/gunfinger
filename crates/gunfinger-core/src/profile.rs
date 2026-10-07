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
    /// With `Some`, the neighbourhood's half-size in bins grows with
    /// frequency instead of `neighbourhood_bins` (a variant under
    /// evaluation, spreading peaks more evenly across octaves).
    pub spread: Option<Spread>,
}

/// A neighbourhood half-size in bins of `share` times the bin, within
/// `min_bins..=max_bins`: the same width in octaves at every frequency
/// between the limits.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spread {
    pub share: f32,
    pub min_bins: usize,
    pub max_bins: usize,
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
        spread: None,
    };

    /// A stable textual identifier, stored in every peak record.
    pub fn id(&self) -> String {
        let spread = self.spread.map_or_else(String::new, |spread| {
            format!(
                " spread={}:{}..{}",
                spread.share, spread.min_bins, spread.max_bins
            )
        });
        format!(
            "peaks-v2 rate={} fft={} hop={} bins={}..{} nbhd={}x{} floor={}{spread}",
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

    /// The neighbourhood's half-size in bins around `bin`.
    pub fn neighbourhood_bins_at(&self, bin: usize) -> usize {
        match self.spread {
            None => self.neighbourhood_bins,
            Some(spread) => ((bin as f32 * spread.share).round() as usize)
                .clamp(spread.min_bins, spread.max_bins),
        }
    }

    fn frames_per_second(&self) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_current_profile_keeps_the_identifier_its_records_carry() {
        assert_eq!(
            Profile::CURRENT.id(),
            "peaks-v2 rate=8000 fft=1024 hop=128 bins=5..500 nbhd=12x12 floor=-10"
        );
    }

    #[test]
    fn a_spread_neighbourhood_widens_with_frequency_within_its_limits() {
        let spread = Profile {
            spread: Some(Spread {
                share: 0.094,
                min_bins: 4,
                max_bins: 24,
            }),
            ..Profile::CURRENT
        };

        assert_eq!(spread.neighbourhood_bins_at(10), 4);
        assert_eq!(spread.neighbourhood_bins_at(128), 12);
        assert_eq!(spread.neighbourhood_bins_at(400), 24);
        assert!(spread.id().ends_with(" spread=0.094:4..24"));
    }
}

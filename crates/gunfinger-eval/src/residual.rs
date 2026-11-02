//! The harness's names for the masked residual of proposal 4 (brief 65),
//! whose code is the core's (`gunfinger_core::residual`, ADR 0025): the
//! `over` of a run (`Rule`), the fixes brief 70 compared (`Fix`), and the
//! lags as reports store them.

use gunfinger_core::profile::Profile;
use gunfinger_core::residual::{self, Alignment, BLOCK_FRAMES, Mask};
use serde::{Deserialize, Serialize};

/// How much of the reference's estimated power each bin of the mixture
/// loses (`Mask::over`).
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

/// `Fix::AlignFine`'s blocks: about 0.5 s, still 16 points above the
/// median in a band one bin wide.
const FINE_BLOCK_FRAMES: usize = 32;

impl Fix {
    pub fn name(self) -> &'static str {
        match self {
            Fix::Off => "off",
            Fix::Align => "align",
            Fix::AlignFine => "align-fine",
        }
    }

    /// The core's mask for this fix under `rule`.
    fn mask(self, rule: Rule) -> Mask {
        let (alignment, block_frames) = match self {
            Fix::Off => (Alignment::AsPlaced, BLOCK_FRAMES),
            Fix::Align => (Alignment::Aligned, BLOCK_FRAMES),
            Fix::AlignFine => (Alignment::Aligned, FINE_BLOCK_FRAMES),
        };
        Mask {
            over: rule.over,
            alignment,
            block_frames,
        }
    }
}

/// The core's residual of `mixture` once `reference` is taken out under
/// `rule`, after `fix`.
pub fn residual(
    mixture: &[f32],
    reference: &[f32],
    profile: &Profile,
    rule: Rule,
    fix: Fix,
) -> residual::Residual {
    residual::residual(mixture, reference, profile, fix.mask(rule))
}

/// The core's `lags`, as a report stores them.
pub fn lags(mixture: &[f32], reference: &[f32], profile: &Profile) -> Lags {
    Lags::from(residual::lags(mixture, reference, profile))
}

/// `residual::Lags`, field for field.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Lags {
    pub blocks: Vec<BlockLag>,
    pub measured: usize,
    pub offset_ms: f64,
    pub drift_ppm: f64,
}

/// `residual::BlockLag`, field for field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockLag {
    pub seconds: f64,
    pub lag_ms: f64,
    pub correlation: f64,
    pub at_zero: f64,
}

impl From<residual::Lags> for Lags {
    fn from(lags: residual::Lags) -> Lags {
        Lags {
            blocks: lags
                .blocks
                .into_iter()
                .map(|block| BlockLag {
                    seconds: block.seconds,
                    lag_ms: block.lag_ms,
                    correlation: block.correlation,
                    at_zero: block.at_zero,
                })
                .collect(),
            measured: lags.measured,
            offset_ms: lags.offset_ms,
            drift_ppm: lags.drift_ppm,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_fix_is_the_core_mask_it_was_measured_as() {
        let rule = Rule { over: 2.0 };
        assert_eq!(
            Fix::Off.mask(rule),
            Mask {
                over: 2.0,
                alignment: Alignment::AsPlaced,
                block_frames: 63,
            }
        );
        assert_eq!(Fix::Align.mask(rule), Mask::UNDER);
        assert_eq!(Fix::AlignFine.mask(rule).block_frames, 32);
        assert_eq!(Fix::AlignFine.mask(rule).alignment, Alignment::Aligned);
    }
}

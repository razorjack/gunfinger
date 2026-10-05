//! The confidence rule: one frozen rule for every input.
//!
//! Only absolute, sustained, aligned evidence counts: hash hits lying on one
//! line through query and reference time, in several windows of query time.
//! Fractions of query hashes are not used (a blend dilutes them), nor is the
//! margin over the runner-up (duplicate rips of one recording tie).
//!
//! Calibration (experiment 0004, fan-out 2): the strongest detection that
//! matched no played track was 95 hits in 2 windows, a remix sharing a
//! section with the original. The weakest identifying detection was 501
//! hits in 3 windows (a 30 s excerpt between two rungs).

/// How much aligned evidence supports a detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evidence {
    /// Windows of query time in which the line has hits.
    pub windows: u32,
    /// Hash hits on the line.
    pub hits: u32,
}

/// About twice the strongest false candidate measured and two and a half
/// times below the weakest identifying detection.
pub const MIN_HITS: u32 = 200;
/// At least 20 seconds of continuous alignment (windows are 10 s); a 30 s
/// excerpt spans 3.
pub const MIN_WINDOWS: u32 = 3;

impl Evidence {
    pub fn is_confident(&self) -> bool {
        self.hits >= MIN_HITS && self.windows >= MIN_WINDOWS
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_strength_and_persistence_are_required() {
        let confident = Evidence {
            windows: MIN_WINDOWS,
            hits: MIN_HITS,
        };
        let brief = Evidence {
            windows: MIN_WINDOWS - 1,
            hits: 10 * MIN_HITS,
        };
        let faint = Evidence {
            windows: 10 * MIN_WINDOWS,
            hits: MIN_HITS - 1,
        };

        assert!(confident.is_confident());
        assert!(!brief.is_confident());
        assert!(!faint.is_confident());
    }
}

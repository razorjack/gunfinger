//! The confidence rule: one frozen rule for every input.
//!
//! Only absolute, sustained, aligned evidence counts: hash hits lying on one
//! line through query and reference time, in several windows of query time.
//! Fractions of query hashes are not used (a blend dilutes them), nor is the
//! margin over the runner-up (duplicate rips of one recording tie).
//!
//! Calibration (experiment 0003): the strongest detection that matched no
//! played track was 243 hits in 2 windows, a remix sharing a section with the
//! original; unrelated audio reached at most 71 hits. The weakest correct
//! detection was 1,274 hits in 3 windows (a 30 s excerpt).

/// How much aligned evidence supports a detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evidence {
    /// Windows of query time in which the line has hits.
    pub windows: u32,
    /// Hash hits on the line.
    pub hits: u32,
}

/// About twice the strongest false candidate measured.
pub const MIN_HITS: u32 = 500;
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
            windows: 3,
            hits: 500,
        };
        let brief = Evidence {
            windows: 2,
            hits: 5000,
        };
        let faint = Evidence {
            windows: 30,
            hits: 499,
        };

        assert!(confident.is_confident());
        assert!(!brief.is_confident());
        assert!(!faint.is_confident());
    }
}

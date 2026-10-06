//! Confidence: how far the evidence for a detection goes.
//!
//! Only absolute, sustained, aligned evidence counts: hash hits lying on one
//! line through query and reference time, in several windows of query time.
//! Fractions of query hashes are not used (a blend dilutes them), nor is the
//! margin over the runner-up (duplicate rips of one recording tie).
//!
//! Both thresholds were calibrated on the sweep and the development set at
//! 262 library tracks (`docs/calibration.md`). Measure them again when the
//! library grows: chance alignments get stronger with more postings.

/// How much aligned evidence supports a detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Evidence {
    /// Windows of query time in which the line has hits.
    pub windows: u32,
    /// Hash hits on the line.
    pub hits: u32,
}

/// What a detection's evidence supports, weakest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Confidence {
    /// At the level of chance alignments with unrelated recordings.
    Weak,
    /// Stronger than any unrelated recording produced, but short of the
    /// frozen rule: this recording, or one sharing material with it (a
    /// remix, a VIP), probably plays here. Shown to help, never counted as
    /// an identification (ADR 0006).
    Possible,
    /// The frozen rule: an identification.
    Confident,
}

/// The frozen rule (experiment 0004): about twice the strongest false
/// candidate measured (95 hits, a remix sharing a section with the played
/// original) and two and a half times below the weakest identifying
/// detection (501 hits, a 30 s excerpt between two rungs).
pub const MIN_HITS: u32 = 200;
/// Hits in three 10 s windows of one chain span more than 10 s of query
/// time; a 30 s excerpt always covers 3, a 15-20 s play only at some places
/// on the window grid (experiment 0020).
pub const MIN_WINDOWS: u32 = 3;
/// About twice the strongest detection of an unrelated recording measured
/// (28 hits); only remixes of the played recording went higher (experiment
/// 0006).
pub const MIN_POSSIBLE_HITS: u32 = 60;

/// The rule as reports record it.
pub fn rule() -> String {
    format!(
        "confident: {MIN_HITS} hits in {MIN_WINDOWS} windows; possible: {MIN_POSSIBLE_HITS} hits"
    )
}

impl Evidence {
    pub fn confidence(&self) -> Confidence {
        if self.hits >= MIN_HITS && self.windows >= MIN_WINDOWS {
            Confidence::Confident
        } else if self.hits >= MIN_POSSIBLE_HITS {
            Confidence::Possible
        } else {
            Confidence::Weak
        }
    }

    pub fn is_confident(&self) -> bool {
        self.confidence() == Confidence::Confident
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

    #[test]
    fn evidence_short_of_the_rule_can_still_be_possible() {
        let possible = Evidence {
            windows: 2,
            hits: MIN_POSSIBLE_HITS,
        };
        let weak = Evidence {
            windows: 10,
            hits: MIN_POSSIBLE_HITS - 1,
        };
        let brief_but_strong = Evidence {
            windows: MIN_WINDOWS - 1,
            hits: 10 * MIN_HITS,
        };

        assert_eq!(possible.confidence(), Confidence::Possible);
        assert_eq!(weak.confidence(), Confidence::Weak);
        assert_eq!(brief_but_strong.confidence(), Confidence::Possible);
    }
}

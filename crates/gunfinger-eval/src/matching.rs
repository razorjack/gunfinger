//! The matcher the harness searches with, and opt-in changes to matching
//! under evaluation. They apply to every search the harness runs, and the
//! reports of anything but the default go to `reports/variant-<name>/`, so
//! that calibrate and regress read one matcher at a time.

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Matcher, Options, SecondOverlap, search_with};
use gunfinger_core::speed::Rung;

#[derive(clap::Args, Clone, Copy, Debug, PartialEq)]
pub struct Matching {
    /// Search with the single-pass matcher, the default before session 6:
    /// every posting list, one pass over the ladder, 200 hits in 3 windows.
    #[arg(long, global = true)]
    pub single_pass: bool,
    /// Empty this share of the fullest posting lists (0.01 is the fullest
    /// 1%), to measure what common hashes cost and contribute. With the
    /// default matcher the lists are emptied instead of set aside.
    #[arg(long, global = true, default_value_t = 0.0)]
    pub drop_fullest: f64,
    /// The second pass measures each stretch of a few windows again at its
    /// own speed when it drifts from the fitted one.
    #[arg(long, global = true, conflicts_with = "single_pass")]
    pub speed_per_stretch: bool,
    /// Detections' boundaries leave out weak windows at either end.
    #[arg(long, global = true)]
    pub trim_ends: bool,
    /// After the second pass, detections of one asset overlap only where
    /// their first-pass chains do, so that both sides of a pitch ride are
    /// kept (experiment 0175).
    #[arg(long, global = true, conflicts_with = "single_pass")]
    pub overlap_by_chains: bool,
    /// After the second pass, detections of one asset whose fitted speeds
    /// differ by more than this share (0.006 is 0.6%) do not overlap, so
    /// that both sides of a pitch ride are kept (experiment 0179).
    #[arg(long, global = true, value_name = "SHARE", conflicts_with_all = ["single_pass", "overlap_by_chains"])]
    pub apart_in_speed: Option<f64>,
}

impl Matching {
    pub fn check(&self) -> Result<(), String> {
        if !(0.0..1.0).contains(&self.drop_fullest) {
            return Err(String::from("--drop-fullest is a share below 1"));
        }
        if self
            .apart_in_speed
            .is_some_and(|share| !(share > 0.0 && share < 1.0))
        {
            return Err(String::from("--apart-in-speed is a share between 0 and 1"));
        }
        Ok(())
    }

    pub fn matcher(&self) -> Matcher {
        if self.single_pass {
            Matcher::SinglePass
        } else {
            Matcher::Fitted
        }
    }

    /// The reports directory's name; `None` for the default matcher.
    pub fn name(&self) -> Option<String> {
        let mut parts = Vec::new();
        if self.single_pass {
            parts.push(String::from("single-pass"));
        }
        if self.drop_fullest > 0.0 {
            parts.push(format!("drop-{}", self.drop_fullest));
        }
        if self.speed_per_stretch {
            parts.push(String::from("stretch"));
        }
        if self.trim_ends {
            parts.push(String::from("trim"));
        }
        if self.overlap_by_chains {
            parts.push(String::from("chains"));
        }
        if let Some(share) = self.apart_in_speed {
            parts.push(format!("apart-{share}"));
        }
        (!parts.is_empty()).then(|| parts.join("-"))
    }

    pub fn options(&self) -> Options {
        Options {
            speed_per_stretch: self.speed_per_stretch,
            trim_weak_ends: self.trim_ends,
            second_overlap: self.second_overlap(),
            ..self.matcher().options()
        }
    }

    fn second_overlap(&self) -> SecondOverlap {
        match (self.overlap_by_chains, self.apart_in_speed) {
            (true, _) => SecondOverlap::Chains,
            (false, Some(share)) => SecondOverlap::SpansAtSpeed(share),
            (false, None) => SecondOverlap::Spans,
        }
    }

    pub fn index(&self, index: Index) -> Index {
        if self.drop_fullest > 0.0 {
            index.without_fullest(self.drop_fullest)
        } else {
            self.matcher().index(index)
        }
    }

    pub fn search(
        &self,
        index: &Index,
        samples: &[f32],
        profile: &Profile,
        ladder: &[Rung],
        jobs: usize,
    ) -> Vec<Detection> {
        search_with(index, samples, profile, ladder, jobs, self.options())
    }

    /// The thresholds its confident detections meet.
    pub fn rule(&self) -> Rule {
        self.pass().rule()
    }

    pub fn pass(&self) -> Pass {
        self.matcher().pass()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT: Matching = Matching {
        single_pass: false,
        drop_fullest: 0.0,
        speed_per_stretch: false,
        trim_ends: false,
        overlap_by_chains: false,
        apart_in_speed: None,
    };

    #[test]
    fn the_default_judges_overlap_on_spans_and_writes_the_default_reports() {
        assert_eq!(DEFAULT.options(), Matcher::Fitted.options());
        assert_eq!(DEFAULT.options().second_overlap, SecondOverlap::Spans);
        assert_eq!(DEFAULT.name(), None);
    }

    #[test]
    fn apart_in_speed_judges_overlap_at_one_speed_in_its_own_reports() {
        let apart = Matching {
            apart_in_speed: Some(0.006),
            ..DEFAULT
        };

        assert_eq!(apart.check(), Ok(()));
        assert_eq!(
            apart.options().second_overlap,
            SecondOverlap::SpansAtSpeed(0.006)
        );
        assert_eq!(apart.name().as_deref(), Some("apart-0.006"));
    }

    #[test]
    fn apart_in_speed_is_a_share_between_0_and_1() {
        for share in [0.0, 1.0, -0.006, 6.0] {
            let apart = Matching {
                apart_in_speed: Some(share),
                ..DEFAULT
            };
            assert!(apart.check().is_err(), "{share}");
        }
    }

    #[test]
    fn overlap_by_chains_skips_the_second_suppression() {
        let chains = Matching {
            overlap_by_chains: true,
            ..DEFAULT
        };

        assert_eq!(chains.options().second_overlap, SecondOverlap::Chains);
        assert_eq!(chains.name().as_deref(), Some("chains"));
    }
}

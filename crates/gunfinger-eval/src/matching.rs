//! The matcher the harness searches with, and opt-in changes to matching
//! under evaluation. They apply to every search the harness runs, and the
//! reports of anything but the default go to `reports/variant-<name>/`, so
//! that calibrate and regress read one matcher at a time.

use std::path::Path;

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{
    Detection, Matcher, Options, SAME_SPEED_SHARE, SecondOverlap, Trace, search_signal_with,
    search_with, trace_with,
};
use gunfinger_core::speed::Rung;

use crate::stereo::{FrontEnd, QueryAudio};

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
    /// After the second pass, a detection that overlaps a stronger one of
    /// the same asset is dropped whatever their speeds: the default before
    /// ADR 0021, which drops one side of a pitch ride (experiment 0175).
    #[arg(long, global = true, conflicts_with = "single_pass")]
    pub overlap_by_spans: bool,
    /// After the second pass, detections of one asset overlap only where
    /// their first-pass chains do, so that both sides of a pitch ride are
    /// kept (experiment 0178).
    #[arg(long, global = true, conflicts_with_all = ["single_pass", "overlap_by_spans"])]
    pub overlap_by_chains: bool,
    /// After the second pass, detections of one asset whose fitted speeds
    /// differ by more than this share do not overlap. The default is 0.006
    /// (0.6%, ADR 0021); this measures another.
    #[arg(long, global = true, value_name = "SHARE", conflicts_with_all = ["single_pass", "overlap_by_spans", "overlap_by_chains"])]
    pub apart_in_speed: Option<f64>,
    /// How audio is analysed (`stereo::FrontEnd`): the downmix, or both
    /// channels' powers added, for queries alone or for queries and
    /// records (a store built by `variant-store`). `robust`, `scan`,
    /// `mixes` and `variant-store` take it.
    #[arg(long, global = true, value_enum, default_value_t = FrontEnd::Downmix)]
    pub front_end: FrontEnd,
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
        if self.apart_in_speed == Some(SAME_SPEED_SHARE) {
            return Err(format!(
                "--apart-in-speed {SAME_SPEED_SHARE} is the default; leave the option out"
            ));
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
        if self.overlap_by_spans {
            parts.push(String::from("spans"));
        }
        if self.overlap_by_chains {
            parts.push(String::from("chains"));
        }
        if let Some(share) = self.apart_in_speed {
            parts.push(format!("apart-{share}"));
        }
        if self.front_end != FrontEnd::Downmix {
            parts.push(self.front_end.name().to_owned());
        }
        (!parts.is_empty()).then(|| parts.join("-"))
    }

    pub fn options(&self) -> Options {
        let matcher = self.matcher().options();
        Options {
            speed_per_stretch: self.speed_per_stretch,
            trim_weak_ends: self.trim_ends,
            second_overlap: self.second_overlap().unwrap_or(matcher.second_overlap),
            ..matcher
        }
    }

    /// The opt-in second suppression, if one is chosen.
    fn second_overlap(&self) -> Option<SecondOverlap> {
        if self.overlap_by_spans {
            Some(SecondOverlap::Spans)
        } else if self.overlap_by_chains {
            Some(SecondOverlap::Chains)
        } else {
            self.apart_in_speed.map(SecondOverlap::SpansAtSpeed)
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

    /// The profile of the records searched and of the search's analysis.
    pub fn profile(&self) -> Profile {
        self.front_end.profile()
    }

    /// The file at `path` as the front end searches it.
    pub fn decode_query(&self, path: &Path) -> Result<QueryAudio, String> {
        self.front_end
            .decode_query(path, self.profile().sample_rate)
    }

    /// Like `search`, for audio of the front end's.
    pub fn search_audio(
        &self,
        index: &Index,
        audio: &QueryAudio,
        ladder: &[Rung],
        jobs: usize,
    ) -> Vec<Detection> {
        let profile = self.profile();
        search_signal_with(
            index,
            audio.signal(),
            &profile,
            ladder,
            jobs,
            self.options(),
        )
    }

    /// The first pass alone: the detections the second pass would measure
    /// again, with their first-pass evidence and each one's chain of lines.
    pub fn first_pass(
        &self,
        index: &Index,
        samples: &[f32],
        profile: &Profile,
        ladder: &[Rung],
        jobs: usize,
    ) -> Trace {
        let options = Options {
            second_pass: false,
            ..self.options()
        };
        trace_with(index, samples, profile, ladder, jobs, options, |_, _| {})
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
        overlap_by_spans: false,
        overlap_by_chains: false,
        apart_in_speed: None,
        front_end: FrontEnd::Downmix,
    };

    #[test]
    fn the_default_judges_overlap_at_one_speed_and_writes_the_default_reports() {
        assert_eq!(DEFAULT.options(), Matcher::Fitted.options());
        assert_eq!(
            DEFAULT.options().second_overlap,
            SecondOverlap::SpansAtSpeed(0.006)
        );
        assert_eq!(DEFAULT.name(), None);
    }

    #[test]
    fn overlap_by_spans_measures_the_rule_before_adr_0021_in_its_own_reports() {
        let spans = Matching {
            overlap_by_spans: true,
            ..DEFAULT
        };

        assert_eq!(spans.check(), Ok(()));
        assert_eq!(spans.options().second_overlap, SecondOverlap::Spans);
        assert_eq!(spans.name().as_deref(), Some("spans"));
    }

    #[test]
    fn apart_in_speed_judges_overlap_at_another_share_in_its_own_reports() {
        let apart = Matching {
            apart_in_speed: Some(0.004),
            ..DEFAULT
        };

        assert_eq!(apart.check(), Ok(()));
        assert_eq!(
            apart.options().second_overlap,
            SecondOverlap::SpansAtSpeed(0.004)
        );
        assert_eq!(apart.name().as_deref(), Some("apart-0.004"));
    }

    #[test]
    fn apart_in_speed_is_a_share_between_0_and_1_other_than_the_default() {
        for share in [0.0, 1.0, -0.006, 6.0, 0.006] {
            let apart = Matching {
                apart_in_speed: Some(share),
                ..DEFAULT
            };
            assert!(apart.check().is_err(), "{share}");
        }
    }

    #[test]
    fn the_power_sum_front_ends_write_their_own_reports_and_records() {
        let queries = Matching {
            front_end: FrontEnd::PowerSumQueries,
            ..DEFAULT
        };
        let both = Matching {
            front_end: FrontEnd::PowerSum,
            ..DEFAULT
        };

        assert_eq!(DEFAULT.profile(), Profile::CURRENT);
        assert_eq!(queries.name().as_deref(), Some("power-sum-queries"));
        assert_eq!(queries.profile(), Profile::CURRENT);
        assert_eq!(both.name().as_deref(), Some("power-sum"));
        assert_eq!(both.profile(), Profile::POWER_SUM);
        assert_eq!(both.options(), DEFAULT.options());
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

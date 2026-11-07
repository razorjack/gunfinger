//! The matcher the harness searches with, and opt-in changes to matching
//! under evaluation. They apply to every search the harness runs, and the
//! reports of anything but the default go to `reports/variant-<name>/`, so
//! that calibrate and regress read one matcher at a time.

use std::fmt;
use std::path::Path;

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{
    Detection, Matcher, Options, SAME_SPEED_SHARE, SKIPPED_SHARE, SecondOverlap, SecondPassFit,
    Trace, search_signal_with, search_with, trace_with,
};
use gunfinger_core::speed::Rung;

use crate::stereo::{FrontEnd, QueryAudio};

#[derive(clap::Args, Clone, Copy, PartialEq)]
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
    /// Set aside this share of the fullest posting lists from the first
    /// pass instead of the default 1% (0.01), so that only the cut moves:
    /// lists cross it as a change of the library makes them (ADR 0014,
    /// experiment 0242). Session 81's perturbation.
    #[arg(long, global = true, value_name = "SHARE", conflicts_with_all = ["single_pass", "drop_fullest"])]
    pub set_aside_share: Option<f64>,
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
    /// `mixes`, `recall`, `shared` and `variant-store` take it.
    #[arg(long, global = true, value_enum, default_value_t = FrontEnd::Downmix)]
    pub front_end: FrontEnd,
    /// The alignment the second pass measures each candidate of 80 hits or
    /// more at (`SecondPassFit`): the chain's speed (the default), the best
    /// of a grid within 0.4% (`grid`), the grid chosen on alternate windows
    /// and counted on the others (`cross`), or a fit iterated on the hits
    /// (`iterated`), or that fit only where the chain's speed is the mean
    /// of its rungs (`iterated-short`). Experiments 0251-0253 and
    /// session 81's.
    #[arg(long, global = true, value_enum, default_value_t = Fit::ChainSpeed, conflicts_with_all = ["single_pass", "speed_per_stretch"])]
    pub second_pass_fit: Fit,
}

/// `--second-pass-fit`'s values, one per `SecondPassFit`.
#[derive(clap::ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fit {
    ChainSpeed,
    Grid,
    Cross,
    Iterated,
    IteratedShort,
}

impl Fit {
    fn second_pass_fit(self) -> SecondPassFit {
        match self {
            Fit::ChainSpeed => SecondPassFit::ChainSpeed,
            Fit::Grid => SecondPassFit::BestOfGrid,
            Fit::Cross => SecondPassFit::CrossFitted,
            Fit::Iterated => SecondPassFit::Iterated,
            Fit::IteratedShort => SecondPassFit::IteratedShortChains,
        }
    }

    /// The reports directory's part; `None` for the default.
    fn name(self) -> Option<&'static str> {
        match self {
            Fit::ChainSpeed => None,
            Fit::Grid => Some("fit-grid"),
            Fit::Cross => Some("fit-cross"),
            Fit::Iterated => Some("fit-iterated"),
            Fit::IteratedShort => Some("fit-iterated-short"),
        }
    }
}

/// Baselines' provenance records this (`provenance.json`, "matching"):
/// the derived form, with `set_aside_share` only when it is given, so that
/// baselines written before it existed still compare.
impl fmt::Debug for Matching {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fields = formatter.debug_struct("Matching");
        fields
            .field("single_pass", &self.single_pass)
            .field("drop_fullest", &self.drop_fullest);
        if let Some(share) = self.set_aside_share {
            fields.field("set_aside_share", &share);
        }
        fields
            .field("speed_per_stretch", &self.speed_per_stretch)
            .field("trim_ends", &self.trim_ends)
            .field("overlap_by_spans", &self.overlap_by_spans)
            .field("overlap_by_chains", &self.overlap_by_chains)
            .field("apart_in_speed", &self.apart_in_speed)
            .field("front_end", &self.front_end)
            .field("second_pass_fit", &self.second_pass_fit)
            .finish()
    }
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
        if self
            .set_aside_share
            .is_some_and(|share| !(share > 0.0 && share < 1.0))
        {
            return Err(String::from("--set-aside-share is a share between 0 and 1"));
        }
        if self.set_aside_share == Some(SKIPPED_SHARE) {
            return Err(format!(
                "--set-aside-share {SKIPPED_SHARE} is the default; leave the option out"
            ));
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
        if let Some(share) = self.set_aside_share {
            parts.push(format!("set-aside-{share}"));
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
        if let Some(fit) = self.second_pass_fit.name() {
            parts.push(fit.to_owned());
        }
        (!parts.is_empty()).then(|| parts.join("-"))
    }

    pub fn options(&self) -> Options {
        let matcher = self.matcher().options();
        Options {
            speed_per_stretch: self.speed_per_stretch,
            trim_weak_ends: self.trim_ends,
            second_overlap: self.second_overlap().unwrap_or(matcher.second_overlap),
            second_pass_fit: self.second_pass_fit.second_pass_fit(),
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
        } else if let Some(share) = self.set_aside_share {
            index.skipping_fullest(share)
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
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::peaks::Peak;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;
    use crate::fullest::set_aside_hashes;

    const DEFAULT: Matching = Matching {
        single_pass: false,
        drop_fullest: 0.0,
        set_aside_share: None,
        speed_per_stretch: false,
        trim_ends: false,
        overlap_by_spans: false,
        overlap_by_chains: false,
        apart_in_speed: None,
        front_end: FrontEnd::Downmix,
        second_pass_fit: Fit::ChainSpeed,
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
    fn each_second_pass_fit_writes_its_own_reports_and_the_default_measures_at_the_chains_speed() {
        assert_eq!(DEFAULT.options().second_pass_fit, SecondPassFit::ChainSpeed);
        let fits = [
            (Fit::Grid, SecondPassFit::BestOfGrid, "fit-grid"),
            (Fit::Cross, SecondPassFit::CrossFitted, "fit-cross"),
            (Fit::Iterated, SecondPassFit::Iterated, "fit-iterated"),
            (
                Fit::IteratedShort,
                SecondPassFit::IteratedShortChains,
                "fit-iterated-short",
            ),
        ];
        for (fit, second_pass_fit, name) in fits {
            let variant = Matching {
                second_pass_fit: fit,
                ..DEFAULT
            };
            assert_eq!(variant.check(), Ok(()));
            assert_eq!(variant.options().second_pass_fit, second_pass_fit);
            assert_eq!(variant.name().as_deref(), Some(name));
            assert_eq!(
                Options {
                    second_pass_fit: SecondPassFit::ChainSpeed,
                    ..variant.options()
                },
                DEFAULT.options()
            );
        }
    }

    fn record(path: &str, bins: &[f32]) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_string(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: 10.0,
            },
            peaks: bins
                .iter()
                .enumerate()
                .map(|(frame, &bin)| Peak {
                    frame: 10.0 * frame as f64,
                    bin,
                    magnitude: 10.0,
                })
                .collect(),
        }
    }

    #[test]
    fn another_set_aside_share_moves_only_the_cut_in_its_own_reports() {
        // One pair of peaks in six files, the others in one each.
        let mut records = vec![
            record("a.mp3", &[100.0, 120.0, 0.0, 300.0, 340.0]),
            record("b.mp3", &[200.0, 230.0]),
        ];
        records.extend((0..4).map(|copy| record(&format!("common-{copy}.mp3"), &[100.0, 120.0])));
        let index = || Index::build(&records).unwrap();
        let lists = index()
            .posting_lists()
            .filter(|list| !list.is_empty())
            .count();
        let share = 1.0 / lists as f64;
        let moved = Matching {
            set_aside_share: Some(share),
            ..DEFAULT
        };

        assert_eq!(moved.check(), Ok(()));
        assert_eq!(moved.name(), Some(format!("set-aside-{share}")));
        assert_eq!(moved.options(), DEFAULT.options());
        assert_eq!(set_aside_hashes(&moved.index(index())).len(), 1);
        assert!(set_aside_hashes(&DEFAULT.index(index())).is_empty());
        for share in [0.0, 1.0, SKIPPED_SHARE] {
            let invalid = Matching {
                set_aside_share: Some(share),
                ..DEFAULT
            };
            assert!(invalid.check().is_err(), "{share}");
        }
    }

    #[test]
    fn the_provenance_of_the_default_is_as_before_the_set_aside_share() {
        assert_eq!(
            format!("{DEFAULT:?}"),
            "Matching { single_pass: false, drop_fullest: 0.0, speed_per_stretch: false, \
             trim_ends: false, overlap_by_spans: false, overlap_by_chains: false, \
             apart_in_speed: None, front_end: Downmix, second_pass_fit: ChainSpeed }"
        );
        let moved = Matching {
            set_aside_share: Some(0.0101),
            ..DEFAULT
        };
        assert!(format!("{moved:?}").contains("drop_fullest: 0.0, set_aside_share: 0.0101, "));
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

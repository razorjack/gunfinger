//! The matcher the harness searches with, and opt-in changes to matching
//! under evaluation. They apply to every search the harness runs, and the
//! reports of anything but the default go to `reports/variant-<name>/`, so
//! that calibrate and regress read one matcher at a time.

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Matcher, Options, search_with};
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
}

impl Matching {
    pub fn check(&self) -> Result<(), String> {
        if (0.0..1.0).contains(&self.drop_fullest) {
            Ok(())
        } else {
            Err(String::from("--drop-fullest is a share below 1"))
        }
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
        (!parts.is_empty()).then(|| parts.join("-"))
    }

    pub fn options(&self) -> Options {
        Options {
            speed_per_stretch: self.speed_per_stretch,
            trim_weak_ends: self.trim_ends,
            ..self.matcher().options()
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

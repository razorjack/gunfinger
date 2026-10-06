//! Opt-in changes to matching under evaluation. They apply to every search
//! the harness runs, and their reports go to `reports/variant-<name>/`, so
//! that calibrate and regress read one matcher at a time.

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search, search_twice};
use gunfinger_core::speed::Rung;

#[derive(clap::Args, Clone, Copy, Debug, PartialEq)]
pub struct Matching {
    /// Measure each candidate again at its fitted speed (opt-in second
    /// pass), with that pass's thresholds.
    #[arg(long, global = true)]
    pub second_pass: bool,
    /// Empty this share of the fullest posting lists (0.01 is the fullest
    /// 1%), to measure what common hashes cost and contribute.
    #[arg(long, global = true, default_value_t = 0.0)]
    pub drop_fullest: f64,
}

impl Matching {
    pub fn check(&self) -> Result<(), String> {
        if (0.0..1.0).contains(&self.drop_fullest) {
            Ok(())
        } else {
            Err(String::from("--drop-fullest is a share below 1"))
        }
    }

    /// The reports directory's name; `None` for the default matcher.
    pub fn name(&self) -> Option<String> {
        let mut parts = Vec::new();
        if self.second_pass {
            parts.push(String::from("second-pass"));
        }
        if self.drop_fullest > 0.0 {
            parts.push(format!("drop-{}", self.drop_fullest));
        }
        (!parts.is_empty()).then(|| parts.join("-"))
    }

    pub fn index(&self, index: Index) -> Index {
        if self.drop_fullest > 0.0 {
            index.without_fullest(self.drop_fullest)
        } else {
            index
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
        if self.second_pass {
            search_twice(index, samples, profile, ladder, jobs)
        } else {
            search(index, samples, profile, ladder, jobs)
        }
    }

    /// The thresholds its confident detections meet.
    pub fn rule(&self) -> Rule {
        self.pass().rule()
    }

    pub fn pass(&self) -> Pass {
        if self.second_pass {
            Pass::Fitted
        } else {
            Pass::Ladder
        }
    }
}

//! Per-second scoring (Sonnleitner et al. 2016, §5): every second of a mix
//! judged against the truth, beside the track counts of `scoring` and
//! `mixes`, which stay as they are. It reads saved reports and searches
//! nothing.
//!
//! Detections are grouped into plays as `identify` groups them
//! (`plays::group`); each play takes the highest level its segments reach
//! under the rule scored (ADR 0013), and labels a second with its file
//! when the second lies inside one of its segments (first to last aligned
//! hit), not in a gap between them. A label is right when its file is in
//! the cluster of a track playing there. During a transition both tracks
//! play, so either is right; where borders are approximate (a manifest's
//! starts), a margin either side of each border is a transition.

use std::collections::{BTreeMap, BTreeSet};

use gunfinger_core::confidence::{Confidence, Evidence};
use gunfinger_core::index::AssetId;
use gunfinger_core::plays;
use gunfinger_core::search::Detection;
use gunfinger_core::speed::{Playback, SpeedRatio};
use serde::Serialize;

use crate::clusters::Clusters;
use crate::manifest::Set;
use crate::mixes::{MixFound, PlannedMix};
use crate::scoring::{Found, evidence};

/// The confident rule detections are levelled by.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rule {
    /// Today's: the hits in 3 windows (`confidence.rs`).
    Windows,
    /// The hits over at least this many seconds from the first to the last
    /// aligned hit, in place of 3 windows (session 27, as `grid` compares).
    MinSpan(f64),
}

impl Rule {
    pub fn name(self) -> String {
        match self {
            Rule::Windows => String::from("3 windows"),
            Rule::MinSpan(seconds) => format!("span {seconds:.0} s"),
        }
    }

    /// The level of evidence whose first and last hit lie `span_seconds`
    /// apart.
    pub fn level(self, evidence: Evidence, span_seconds: f64) -> Confidence {
        let rule = evidence.pass.rule();
        match self {
            Rule::Windows => evidence.confidence(),
            Rule::MinSpan(seconds) if evidence.hits >= rule.min_hits && span_seconds >= seconds => {
                Confidence::Confident
            }
            Rule::MinSpan(_) if evidence.hits >= rule.min_possible_hits => Confidence::Possible,
            Rule::MinSpan(_) => Confidence::Weak,
        }
    }
}

/// A detection as reports keep it.
#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub windows: u32,
    pub hits: u32,
    pub fitted: bool,
}

impl Segment {
    fn evidence(&self) -> Evidence {
        evidence(self.windows, self.hits, self.fitted)
    }
}

/// A track of the truth.
#[derive(Debug, Clone)]
pub struct Truth {
    /// The files whose label is right for it: its cluster's.
    pub accepted: BTreeSet<String>,
    /// Where it plays.
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Whether the index holds it; seconds where no indexed track plays
    /// measure specificity.
    pub in_library: bool,
}

/// How each second of a mix was labelled, while an indexed track plays
/// (or one of its margins holds) and while none does.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Seconds {
    pub correct: u32,
    pub possible: u32,
    pub unlabelled: u32,
    /// Unlabelled inside a border's margin, where the truth is uncertain.
    pub unlabelled_in_margin: u32,
    /// A confident label that is not right, while an indexed track plays.
    pub mislabelled: u32,
    /// No indexed track plays and no confident label.
    pub silent: u32,
    /// A confident label while no indexed track plays.
    pub mislabelled_unindexed: u32,
}

impl Seconds {
    pub fn add(&mut self, other: Seconds) {
        self.correct += other.correct;
        self.possible += other.possible;
        self.unlabelled += other.unlabelled;
        self.unlabelled_in_margin += other.unlabelled_in_margin;
        self.mislabelled += other.mislabelled;
        self.silent += other.silent;
        self.mislabelled_unindexed += other.mislabelled_unindexed;
    }

    pub fn indexed(&self) -> u32 {
        self.correct
            + self.possible
            + self.unlabelled
            + self.unlabelled_in_margin
            + self.mislabelled
    }

    pub fn unindexed(&self) -> u32 {
        self.silent + self.mislabelled_unindexed
    }
}

/// Where a confidently identified track's labels start and end against
/// its truth (negative: early).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Boundary {
    pub start_seconds: f64,
    pub start_error_seconds: f64,
    pub end_error_seconds: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Score {
    pub seconds: Seconds,
    pub boundaries: Vec<Boundary>,
}

/// The truth of a generated mix: each play from its first to its last
/// audible sample, indexed unless its cluster was held out.
pub fn mix_truth(mix: &PlannedMix, clusters: &Clusters) -> Vec<Truth> {
    mix.plays
        .iter()
        .map(|play| Truth {
            accepted: clusters.cluster_of(&play.asset),
            start_seconds: play.start_seconds,
            end_seconds: play.end_seconds,
            in_library: !play.held_out,
        })
        .collect()
}

/// The truth of a mix rendered by `shared-mixes`, searched with `left_out`
/// out of the index: a play is indexed unless its cluster was held out or
/// every file of it left out.
pub fn rendered_truth(
    mix: &PlannedMix,
    clusters: &Clusters,
    left_out: &BTreeSet<String>,
) -> Vec<Truth> {
    mix_truth(mix, clusters)
        .into_iter()
        .map(|truth| Truth {
            in_library: truth.in_library
                && truth.accepted.iter().any(|asset| !left_out.contains(asset)),
            ..truth
        })
        .collect()
}

/// The truth of a set as its manifest gives it: each track from its start
/// to the next track's, the last to the end of the mix; indexed when it is
/// referenced and its files were not left out.
pub fn set_truth(
    set: &Set,
    duration_seconds: f64,
    clusters: &Clusters,
    left_out: &BTreeSet<String>,
) -> Vec<Truth> {
    set.tracks
        .iter()
        .enumerate()
        .map(|(position, track)| {
            let accepted: BTreeSet<String> = track
                .references
                .iter()
                .flat_map(|reference| clusters.cluster_of(reference))
                .collect();
            Truth {
                in_library: accepted.iter().any(|asset| !left_out.contains(asset)),
                accepted,
                start_seconds: track.start.as_secs_f64(),
                end_seconds: set
                    .tracks
                    .get(position + 1)
                    .map_or(duration_seconds, |next| next.start.as_secs_f64()),
            }
        })
        .collect()
}

impl From<&MixFound> for Segment {
    fn from(found: &MixFound) -> Segment {
        Segment {
            asset: found.asset.clone(),
            start_seconds: found.start_seconds,
            end_seconds: found.end_seconds,
            windows: found.windows,
            hits: found.hits,
            fitted: found.fitted,
        }
    }
}

impl From<&Found> for Segment {
    fn from(found: &Found) -> Segment {
        Segment {
            asset: found.asset.clone(),
            start_seconds: found.start_seconds,
            end_seconds: found.end_seconds,
            windows: found.windows,
            hits: found.hits,
            fitted: found.fitted,
        }
    }
}

/// One mix or set scored under one rule.
#[derive(Debug, Serialize)]
pub struct Scored {
    pub name: String,
    pub rule: String,
    pub score: Score,
}

/// Prints each rule's seconds over `scored`, and its boundary errors.
pub fn print_summary(title: &str, scored: &[&Scored]) {
    println!("{title}");
    let mut rules: Vec<&str> = Vec::new();
    for scored in scored {
        if !rules.contains(&scored.rule.as_str()) {
            rules.push(&scored.rule);
        }
    }
    for rule in rules {
        let mut total = Seconds::default();
        let mut boundaries = Vec::new();
        for scored in scored.iter().filter(|scored| scored.rule == rule) {
            total.add(scored.score.seconds);
            boundaries.extend(scored.score.boundaries.iter().cloned());
        }
        let share = |count: u32, of: u32| 100.0 * f64::from(count) / f64::from(of.max(1));
        let indexed = total.indexed();
        println!(
            "  {rule:>10}: indexed {indexed} s: correct {:.1}%, possible {:.1}%, unlabelled {:.1}%, in a margin {:.1}%, mislabelled {}; not indexed {} s: specificity {:.1}%, mislabelled {}",
            share(total.correct, indexed),
            share(total.possible, indexed),
            share(total.unlabelled, indexed),
            share(total.unlabelled_in_margin, indexed),
            total.mislabelled,
            total.unindexed(),
            share(total.silent, total.unindexed()),
            total.mislabelled_unindexed,
        );
        if !boundaries.is_empty() {
            let starts: Vec<f64> = boundaries.iter().map(|b| b.start_error_seconds).collect();
            let ends: Vec<f64> = boundaries.iter().map(|b| b.end_error_seconds).collect();
            println!(
                "  {:>10}  {} tracks: start error median {:+.1} s, |error| p90 {:.1} s; end error median {:+.1} s, |error| p90 {:.1} s",
                "",
                boundaries.len(),
                median(&starts),
                percentile_abs(&starts, 0.9),
                median(&ends),
                percentile_abs(&ends, 0.9),
            );
        }
    }
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    sorted[sorted.len() / 2]
}

fn percentile_abs(values: &[f64], share: f64) -> f64 {
    let mut sorted: Vec<f64> = values.iter().map(|value| value.abs()).collect();
    sorted.sort_by(f64::total_cmp);
    let at = ((share * sorted.len() as f64).ceil() as usize).clamp(1, sorted.len());
    sorted[at - 1]
}

/// A play as the scorer labels with it.
pub struct Label<'a> {
    pub asset: &'a str,
    pub level: Confidence,
    /// Its segments' spans, from the first to the last aligned hit.
    pub spans: Vec<(f64, f64)>,
}

impl Label<'_> {
    pub fn covers(&self, t: f64) -> bool {
        self.spans
            .iter()
            .any(|&(start, end)| start <= t && t <= end)
    }
}

/// Scores `segments` against `truth` over `duration_seconds`, judging each
/// whole second at its middle. A label is also right within `margin`
/// seconds of a track's start or end, where an unlabelled second is
/// counted apart.
pub fn score(
    truth: &[Truth],
    segments: &[Segment],
    duration_seconds: f64,
    margin: f64,
    rule: Rule,
) -> Score {
    let labels = labels(segments, rule);
    let mut seconds = Seconds::default();
    let mut second = 0.5;
    while second < duration_seconds {
        let right = |truth: &Truth| {
            truth.start_seconds - margin <= second && second < truth.end_seconds + margin
        };
        let is_right = |asset: &str| {
            truth
                .iter()
                .any(|truth| right(truth) && truth.accepted.contains(asset))
        };
        let indexed = truth.iter().any(|truth| truth.in_library && right(truth));
        let in_margin = truth.iter().any(|truth| {
            right(truth) && !(truth.start_seconds <= second && second < truth.end_seconds)
        });
        let at = |level: Confidence| {
            labels
                .iter()
                .filter(move |label| label.level == level && label.covers(second))
        };
        let wrong = at(Confidence::Confident).any(|label| !is_right(label.asset));
        let count = match (indexed, wrong) {
            (true, true) => &mut seconds.mislabelled,
            (false, true) => &mut seconds.mislabelled_unindexed,
            (false, false) => &mut seconds.silent,
            (true, false) => {
                if at(Confidence::Confident).any(|label| is_right(label.asset)) {
                    &mut seconds.correct
                } else if at(Confidence::Possible).any(|label| is_right(label.asset)) {
                    &mut seconds.possible
                } else if in_margin {
                    &mut seconds.unlabelled_in_margin
                } else {
                    &mut seconds.unlabelled
                }
            }
        };
        *count += 1;
        second += 1.0;
    }
    Score {
        seconds,
        boundaries: truth
            .iter()
            .filter(|truth| truth.in_library)
            .filter_map(|truth| boundary(truth, &labels))
            .collect(),
    }
}

/// The span of the confident labels of `truth`'s files that overlap it.
fn boundary(truth: &Truth, labels: &[Label]) -> Option<Boundary> {
    let spans: Vec<(f64, f64)> = labels
        .iter()
        .filter(|label| label.level == Confidence::Confident)
        .filter(|label| truth.accepted.contains(label.asset))
        .flat_map(|label| label.spans.iter().copied())
        .filter(|&(start, end)| start < truth.end_seconds && truth.start_seconds < end)
        .collect();
    let start = spans.iter().map(|span| span.0).reduce(f64::min)?;
    let end = spans.iter().map(|span| span.1).reduce(f64::max)?;
    Some(Boundary {
        start_seconds: truth.start_seconds,
        start_error_seconds: start - truth.start_seconds,
        end_error_seconds: end - truth.end_seconds,
    })
}

/// The plays of `segments`, grouped by `plays::group`, each at the highest
/// level its segments reach under `rule`.
pub fn labels(segments: &[Segment], rule: Rule) -> Vec<Label<'_>> {
    let files: Vec<&str> = segments
        .iter()
        .map(|segment| segment.asset.as_str())
        .collect::<BTreeSet<&str>>()
        .into_iter()
        .collect();
    let ids: BTreeMap<&str, u32> = files
        .iter()
        .zip(0..)
        .map(|(&file, id)| (file, id))
        .collect();
    let detections: Vec<Detection> = segments
        .iter()
        .map(|segment| Detection {
            asset: AssetId(ids[segment.asset.as_str()]),
            start_seconds: segment.start_seconds,
            end_seconds: segment.end_seconds,
            track_start_seconds: 0.0,
            track_end_seconds: 0.0,
            speed: SpeedRatio(1.0),
            playback: Playback::Turntable,
            evidence: segment.evidence(),
        })
        .collect();
    plays::group(&detections)
        .iter()
        .map(|play| Label {
            asset: files[play.asset.0 as usize],
            level: play
                .segments()
                .iter()
                .map(|detection| {
                    rule.level(
                        detection.evidence,
                        detection.end_seconds - detection.start_seconds,
                    )
                })
                .max()
                .unwrap_or(Confidence::Weak),
            spans: play
                .segments()
                .iter()
                .map(|detection| (detection.start_seconds, detection.end_seconds))
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn truth(asset: &str, start_seconds: f64, end_seconds: f64, in_library: bool) -> Truth {
        Truth {
            accepted: BTreeSet::from([asset.to_owned()]),
            start_seconds,
            end_seconds,
            in_library,
        }
    }

    fn segment(
        asset: &str,
        start_seconds: f64,
        end_seconds: f64,
        windows: u32,
        hits: u32,
    ) -> Segment {
        Segment {
            asset: asset.to_owned(),
            start_seconds,
            end_seconds,
            windows,
            hits,
            fitted: true,
        }
    }

    fn seconds(truth: &[Truth], segments: &[Segment], duration: f64) -> Seconds {
        score(truth, segments, duration, 0.0, Rule::Windows).seconds
    }

    #[test]
    fn a_labelled_play_is_correct_and_the_rest_is_unlabelled_or_silent() {
        let truth = [truth("a", 0.0, 60.0, true), truth("x", 60.0, 90.0, false)];
        let found = [segment("a", 10.0, 50.0, 4, 900)];

        assert_eq!(
            seconds(&truth, &found, 90.0),
            Seconds {
                correct: 40,
                unlabelled: 20,
                silent: 30,
                ..Seconds::default()
            }
        );
    }

    #[test]
    fn during_a_crossfade_either_track_is_right() {
        let truth = [truth("a", 0.0, 40.0, true), truth("b", 30.0, 70.0, true)];
        let found = [
            segment("a", 0.0, 40.0, 4, 900),
            segment("b", 35.0, 70.0, 4, 900),
        ];

        let counted = seconds(&truth, &found, 70.0);

        assert_eq!(counted.correct, 70);
        assert_eq!(counted.mislabelled, 0);
    }

    #[test]
    fn a_wrong_confident_label_beside_a_right_one_is_mislabelled() {
        let truth = [truth("a", 0.0, 60.0, true)];
        let found = [
            segment("a", 0.0, 60.0, 6, 1_500),
            segment("remix", 20.0, 50.0, 3, 300),
        ];

        let counted = seconds(&truth, &found, 60.0);

        assert_eq!(counted.mislabelled, 30);
        assert_eq!(counted.correct, 30);
    }

    #[test]
    fn a_confident_label_where_nothing_indexed_plays_costs_specificity() {
        let truth = [truth("held out", 0.0, 60.0, false)];
        let found = [segment("other", 10.0, 40.0, 3, 300)];

        let counted = seconds(&truth, &found, 60.0);

        assert_eq!(counted.mislabelled_unindexed, 30);
        assert_eq!(counted.silent, 30);
        assert_eq!(counted.indexed(), 0);
    }

    #[test]
    fn a_possible_play_labels_possibly_and_never_wrongly() {
        let truth = [truth("a", 0.0, 60.0, true)];
        let found = [
            segment("a", 10.0, 30.0, 2, 500),
            segment("remix", 30.0, 50.0, 2, 120),
        ];

        let counted = seconds(&truth, &found, 60.0);

        assert_eq!(counted.possible, 20);
        assert_eq!(counted.mislabelled, 0);
        assert_eq!(counted.unlabelled, 40);
    }

    #[test]
    fn a_confident_segment_makes_its_whole_play_confident_but_not_its_gaps() {
        // ADR 0013: one play of two segments 20 s apart.
        let truth = [truth("a", 0.0, 100.0, true)];
        let found = [
            segment("a", 0.0, 40.0, 4, 900),
            segment("a", 60.0, 80.0, 2, 400),
        ];

        let counted = seconds(&truth, &found, 100.0);

        assert_eq!(counted.correct, 60);
        assert_eq!(counted.unlabelled, 40);
    }

    #[test]
    fn inside_a_margin_the_neighbour_is_right_and_silence_is_set_apart() {
        let truth = [truth("a", 0.0, 60.0, true), truth("b", 60.0, 120.0, true)];
        // `a` runs 10 s past the approximate border; `b` starts 20 s late.
        let found = [
            segment("a", 0.0, 70.0, 7, 1_500),
            segment("b", 80.0, 120.0, 4, 900),
        ];

        let with_margin = score(&truth, &found, 120.0, 30.0, Rule::Windows).seconds;
        let without = score(&truth, &found, 120.0, 0.0, Rule::Windows).seconds;

        assert_eq!(with_margin.correct, 110);
        assert_eq!(with_margin.unlabelled_in_margin, 10);
        assert_eq!(with_margin.mislabelled, 0);
        assert_eq!(without.mislabelled, 10);
        assert_eq!(without.unlabelled, 10);
    }

    #[test]
    fn a_minimum_span_promotes_a_brief_play_of_two_windows() {
        let truth = [truth("a", 0.0, 30.0, true)];
        let found = [segment("a", 5.0, 21.0, 2, 450)];

        let today = score(&truth, &found, 30.0, 0.0, Rule::Windows).seconds;
        let span = score(&truth, &found, 30.0, 0.0, Rule::MinSpan(15.0)).seconds;
        let longer = score(&truth, &found, 30.0, 0.0, Rule::MinSpan(20.0)).seconds;

        assert_eq!(today.possible, 16);
        assert_eq!(span.correct, 16);
        assert_eq!(longer.possible, 16);
    }

    #[test]
    fn the_rule_keeps_the_possible_tier() {
        let evidence = |windows, hits| evidence(windows, hits, true);

        assert_eq!(
            Rule::MinSpan(10.0).level(evidence(1, 239), 30.0),
            Confidence::Possible
        );
        assert_eq!(
            Rule::MinSpan(10.0).level(evidence(1, 59), 30.0),
            Confidence::Weak
        );
        assert_eq!(
            Rule::MinSpan(10.0).level(evidence(1, 240), 9.9),
            Confidence::Possible
        );
        assert_eq!(
            Rule::MinSpan(10.0).level(evidence(1, 240), 10.0),
            Confidence::Confident
        );
    }

    #[test]
    fn a_rendered_play_whose_files_are_left_out_is_not_indexed() {
        let play = |asset: &str, held_out| crate::mixes::PlannedPlay {
            asset: asset.to_owned(),
            held_out,
            source_start_seconds: 0.0,
            speed: 1.0,
            start_seconds: 0.0,
            end_seconds: 60.0,
            fade_in_seconds: 0.0,
            fade_out_seconds: 0.0,
            bass_cut_until: None,
            bass_cut_from: None,
        };
        let mix = PlannedMix {
            name: String::from("m"),
            seconds: 60.0,
            plays: vec![
                play("played", false),
                play("drawn", false),
                play("held", true),
            ],
        };
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("played"), String::from("rip")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        let both_out = BTreeSet::from([String::from("played"), String::from("rip")]);
        let source_out = BTreeSet::from([String::from("played")]);
        let indexed = |left_out| {
            rendered_truth(&mix, &clusters, left_out)
                .iter()
                .map(|truth| truth.in_library)
                .collect::<Vec<bool>>()
        };

        assert_eq!(indexed(&both_out), [false, true, false]);
        assert_eq!(indexed(&source_out), [true, true, false]);
    }

    #[test]
    fn a_margin_beside_an_unindexed_track_lets_the_indexed_one_run_on_and_the_rest_is_silent() {
        let truth = [truth("a", 0.0, 60.0, true), truth("x", 60.0, 120.0, false)];
        let found = [segment("a", 0.0, 75.0, 8, 1_500)];

        let counted = score(&truth, &found, 120.0, 30.0, Rule::Windows).seconds;

        assert_eq!(
            counted,
            Seconds {
                correct: 75,
                unlabelled_in_margin: 15,
                silent: 30,
                ..Seconds::default()
            }
        );
    }

    #[test]
    fn a_wrong_label_in_the_margin_beside_an_unindexed_track_counts_while_the_indexed_one_may_play()
    {
        let truth = [truth("a", 0.0, 60.0, true), truth("x", 60.0, 120.0, false)];
        let found = [
            segment("a", 0.0, 60.0, 6, 1_500),
            segment("other", 80.0, 100.0, 3, 300),
        ];

        let counted = score(&truth, &found, 120.0, 30.0, Rule::Windows).seconds;

        assert_eq!(
            counted,
            Seconds {
                correct: 60,
                unlabelled_in_margin: 20,
                mislabelled: 10,
                mislabelled_unindexed: 10,
                silent: 20,
                ..Seconds::default()
            }
        );
    }

    #[test]
    fn a_margin_before_an_indexed_track_that_follows_an_unindexed_one() {
        let truth = [truth("x", 0.0, 60.0, false), truth("a", 60.0, 120.0, true)];
        let found = [segment("a", 40.0, 120.0, 8, 1_500)];

        let counted = score(&truth, &found, 120.0, 30.0, Rule::Windows).seconds;

        assert_eq!(
            counted,
            Seconds {
                correct: 80,
                unlabelled_in_margin: 10,
                silent: 30,
                ..Seconds::default()
            }
        );
    }

    #[test]
    fn a_manifest_track_plays_until_the_next_one_starts_and_the_last_until_the_end() {
        let track = |position: u32, start: u64, references: &[&str]| crate::manifest::Track {
            position,
            artist: String::from("Artist"),
            title: format!("Track {position}"),
            start: std::time::Duration::from_secs(start),
            references: references.iter().map(|&file| file.to_owned()).collect(),
            set_aside: Vec::new(),
        };
        let set = Set {
            title: String::from("A mix"),
            audio: std::path::PathBuf::from("mix.mp3"),
            tracks: vec![
                track(1, 0, &["a.mp3"]),
                track(2, 150, &[]),
                track(3, 300, &["c.mp3", "c vinyl.mp3"]),
            ],
        };
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("a rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        let truth = set_truth(&set, 420.0, &clusters, &BTreeSet::new());

        let spans: Vec<(f64, f64, bool)> = truth
            .iter()
            .map(|truth| (truth.start_seconds, truth.end_seconds, truth.in_library))
            .collect();
        assert_eq!(
            spans,
            [
                (0.0, 150.0, true),
                (150.0, 300.0, false),
                (300.0, 420.0, true)
            ]
        );
        assert_eq!(
            truth[0].accepted,
            BTreeSet::from([String::from("a rip.mp3"), String::from("a.mp3")])
        );
        assert!(truth[1].accepted.is_empty());
        assert_eq!(
            truth[2].accepted,
            BTreeSet::from([String::from("c vinyl.mp3"), String::from("c.mp3")])
        );
    }

    #[test]
    fn a_manifest_track_is_indexed_while_any_file_of_its_cluster_is() {
        let set = Set {
            title: String::from("A mix"),
            audio: std::path::PathBuf::from("mix.mp3"),
            tracks: vec![crate::manifest::Track {
                position: 1,
                artist: String::from("Artist"),
                title: String::from("Track"),
                start: std::time::Duration::from_secs(0),
                references: vec![String::from("a.mp3")],
                set_aside: Vec::new(),
            }],
        };
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("a rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let indexed = |left_out: &[&str]| {
            let left_out: BTreeSet<String> = left_out.iter().map(|&file| file.to_owned()).collect();
            set_truth(&set, 60.0, &clusters, &left_out)[0].in_library
        };

        assert!(indexed(&[]));
        assert!(indexed(&["a.mp3"]));
        assert!(!indexed(&["a.mp3", "a rip.mp3"]));
    }

    #[test]
    fn a_generated_mix_is_its_plays_and_a_held_out_play_is_not_indexed() {
        let play = |asset: &str, held_out, start_seconds, end_seconds| crate::mixes::PlannedPlay {
            asset: asset.to_owned(),
            held_out,
            source_start_seconds: 30.0,
            speed: 1.02,
            start_seconds,
            end_seconds,
            fade_in_seconds: 4.0,
            fade_out_seconds: 4.0,
            bass_cut_until: None,
            bass_cut_from: None,
        };
        let mix = PlannedMix {
            name: String::from("m"),
            seconds: 120.0,
            plays: vec![
                play("a.mp3", false, 0.0, 64.0),
                play("held.mp3", true, 60.0, 120.0),
            ],
        };
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("a rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };

        let truth = mix_truth(&mix, &clusters);

        let spans: Vec<(f64, f64, bool)> = truth
            .iter()
            .map(|truth| (truth.start_seconds, truth.end_seconds, truth.in_library))
            .collect();
        assert_eq!(spans, [(0.0, 64.0, true), (60.0, 120.0, false)]);
        assert_eq!(
            truth[0].accepted,
            BTreeSet::from([String::from("a rip.mp3"), String::from("a.mp3")])
        );
        assert_eq!(
            truth[1].accepted,
            BTreeSet::from([String::from("held.mp3")])
        );
    }

    #[test]
    fn boundaries_measure_the_confident_labels_against_the_truth() {
        let truth = [truth("a", 10.0, 70.0, true), truth("b", 70.0, 90.0, true)];
        let found = [
            segment("a", 12.5, 40.0, 3, 600),
            segment("a", 50.0, 71.0, 3, 600),
            segment("b", 75.0, 85.0, 2, 300),
        ];

        let boundaries = score(&truth, &found, 90.0, 0.0, Rule::Windows).boundaries;

        assert_eq!(
            boundaries,
            [Boundary {
                start_seconds: 10.0,
                start_error_seconds: 2.5,
                end_error_seconds: 1.0,
            }]
        );
    }
}

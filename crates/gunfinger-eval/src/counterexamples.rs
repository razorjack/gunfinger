//! Automated counterexample search (brief 68): small generated mixes under
//! transformations sampled within fixed realistic bounds, searched and
//! compared with the truth. Each failure (a true play lost, a long gap in
//! a found one, or a wrong answer) is then minimized while it persists:
//! transformations are dropped one at a time, in the order they were
//! drawn, then each left is halved. The minimal recipe is saved; it
//! renders the same mix again.
//!
//! A sample plays a held-out lead-in, cut into the target play, then a
//! partner after the target. The target is indexed or held out; the
//! partner too. The transformations act on the target (speed, a pitch
//! ride, a bass kill), on the partner's entry (a blend over the target's
//! end) or on the whole render (a channel fault over the mix's second
//! half, `mixes::render`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::files::write_json;
use crate::matching::Matching;
use crate::mixes::{
    MixFound, MixResult, PlannedMix, PlannedPlay, Pools, Ride, Searcher, SweepIndex, draw_unused,
    search_mix, sweep_index,
};
use crate::padding::Padding;
use crate::rng::Rng;
use crate::robust::{Fault, stereo_fault};
use crate::scoring::Level;
use crate::shared::Roots;
use crate::titles::SameTitles;

/// The held-out lead-in, and the partner's play after the target ends.
const LEAD_SECONDS: f64 = 30.0;
const PARTNER_SECONDS: f64 = 30.0;
/// The target play's length in the mix: at least this long, so that a
/// play lost is one the frozen rule could find untransformed.
const TARGET_SECONDS: (f64, f64) = (40.0, 120.0);
/// Share of targets and of partners drawn from held-out clusters (as the
/// generated mixes' `HELD_OUT_SHARE`).
const HELD_OUT_SHARE: f64 = 0.25;
/// Each axis joins a sample with this probability.
const AXIS_SHARE: f64 = 0.5;
/// The generated mixes' turntable speeds (`mixes::SPEED_RANGE`).
const SPEED_RANGE: f64 = 0.06;
/// The rides kind's default draw (`mixes::RideDraw`): the change, and the
/// ramp's length; the ramp starts 30-70% into the play.
const RIDE_CHANGE: (f64, f64) = (0.01, 0.03);
const RIDE_SECONDS: (f64, f64) = (5.0, 15.0);
/// A ride turns back towards native speed if it would end further than
/// this from it: the ladder covers 0.92-1.08.
const SPEED_LIMIT: f64 = 0.06;
/// A bass kill from the target's entry, and a blend of the partner over
/// the target's end, last as long as the long mixes' crossfades
/// (`mixes::LONG_CROSSFADE_SECONDS`), each at most the target's length.
const KILL_SECONDS: (f64, f64) = (10.0, 60.0);
const BLEND_SECONDS: (f64, f64) = (10.0, 60.0);
/// The partner's level against the target's during a blend (`robust`'s
/// blends: -6 and 0 dB).
const BLEND_DB: (f64, f64) = (-6.0, 0.0);
/// Brief 42's channel faults, by their `robust` names.
const FAULTS: [&str; 8] = [
    "stereo-inverted",
    "stereo-silent",
    "stereo-noise-snr-10db",
    "stereo-noise-snr-0db",
    "stereo-noise-snr--10db",
    "stereo-delay-0.2ms",
    "stereo-delay-1ms",
    "stereo-delay-5ms",
];
/// A found target fails when a stretch this long of its play has no
/// confident detection of it (a side of a ride lost, experiment 0176).
const GAP_SECONDS: f64 = 30.0;
/// Halvings tried per transformation while minimizing.
const MAX_SHRINKS: usize = 3;

/// A file played in a sample, from this far into the track.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
}

/// What a sample plays before any transformation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Base {
    pub lead: Source,
    pub target: Source,
    pub target_seconds: f64,
    pub partner: Source,
}

/// One change to the base, within the bounds above.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Transformation {
    /// The target at this turntable speed instead of native.
    Speed { speed: f64 },
    /// From `at` (a share of the target's length) the target's speed
    /// moves by `change` over `seconds`, then holds.
    Ride { change: f64, at: f64, seconds: f64 },
    /// The target's bass cut (the generator's bass swap) for its first
    /// `seconds`.
    BassKill { seconds: f64 },
    /// The partner comes in `seconds` before the target ends, at `gain_db`
    /// against it.
    Blend { seconds: f64, gain_db: f64 },
    /// The right channel under this `robust` stereo fault over the mix's
    /// second half.
    ChannelFault { fault: String },
}

impl Transformation {
    pub fn kind(&self) -> &'static str {
        match self {
            Transformation::Speed { .. } => "speed",
            Transformation::Ride { .. } => "ride",
            Transformation::BassKill { .. } => "bass-kill",
            Transformation::Blend { .. } => "blend",
            Transformation::ChannelFault { .. } => "channel-fault",
        }
    }

    /// The same transformation at half its size; `None` for a channel
    /// fault, which has no size.
    pub fn halved(&self) -> Option<Transformation> {
        match self {
            Transformation::Speed { speed } => Some(Transformation::Speed {
                speed: 1.0 + (speed - 1.0) / 2.0,
            }),
            Transformation::Ride {
                change,
                at,
                seconds,
            } => Some(Transformation::Ride {
                change: change / 2.0,
                at: *at,
                seconds: *seconds,
            }),
            Transformation::BassKill { seconds } => Some(Transformation::BassKill {
                seconds: seconds / 2.0,
            }),
            Transformation::Blend { seconds, gain_db } => Some(Transformation::Blend {
                seconds: seconds / 2.0,
                gain_db: *gain_db,
            }),
            Transformation::ChannelFault { .. } => None,
        }
    }
}

/// Everything a render needs: the base and its transformations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub base: Base,
    pub transformations: Vec<Transformation>,
}

impl Recipe {
    fn with(&self, transformations: &[Transformation]) -> Recipe {
        Recipe {
            base: self.base.clone(),
            transformations: transformations.to_vec(),
        }
    }

    /// The mix this recipe plays, and its channel fault. The target is
    /// the second play.
    pub fn planned(&self, name: &str) -> Result<(PlannedMix, Option<Fault>), String> {
        let Base {
            lead,
            target,
            target_seconds,
            partner,
        } = &self.base;
        let mut speed = 1.0;
        let mut ride = None;
        let mut kill = None;
        let mut blend = None;
        let mut fault = None;
        for transformation in &self.transformations {
            match transformation {
                Transformation::Speed { speed: value } => speed = *value,
                Transformation::Ride {
                    change,
                    at,
                    seconds,
                } => ride = Some((*change, *at, *seconds)),
                Transformation::BassKill { seconds } => kill = Some(*seconds),
                Transformation::Blend { seconds, gain_db } => blend = Some((*seconds, *gain_db)),
                Transformation::ChannelFault { fault: name } => fault = Some(stereo_fault(name)?),
            }
        }
        let target_start = LEAD_SECONDS;
        let target_end = target_start + target_seconds;
        let (overlap, partner_db) = blend.map_or((0.0, 0.0), |(seconds, db)| {
            (seconds.min(*target_seconds), db)
        });
        let play = |source: &Source, start_seconds: f64, end_seconds: f64| PlannedPlay {
            asset: source.asset.clone(),
            held_out: source.held_out,
            source_start_seconds: source.start_seconds,
            speed: 1.0,
            start_seconds,
            end_seconds,
            fade_in_seconds: 0.0,
            fade_out_seconds: 0.0,
            bass_cut_until: None,
            bass_cut_from: None,
            ride: None,
            gain_db: 0.0,
            scenario: None,
            side: None,
            drop_seconds: None,
        };
        let target_play = PlannedPlay {
            speed,
            ride: ride.map(|(change, at, seconds)| Ride {
                start_seconds: target_start + at * target_seconds,
                seconds,
                to_speed: speed + change,
            }),
            bass_cut_until: kill.map(|seconds| target_start + seconds.min(*target_seconds)),
            ..play(target, target_start, target_end)
        };
        let partner_play = PlannedPlay {
            gain_db: partner_db,
            ..play(partner, target_end - overlap, target_end + PARTNER_SECONDS)
        };
        let mix = PlannedMix {
            name: name.to_owned(),
            seconds: target_end + PARTNER_SECONDS,
            plays: vec![play(lead, 0.0, target_start), target_play, partner_play],
        };
        Ok((mix, fault))
    }
}

/// Draws a sample: the base from `pools` (lead, target and partner from
/// distinct clusters), then each axis with probability `AXIS_SHARE`.
/// Each axis's values are drawn whether or not it joins, so that a
/// sample's values do not depend on which axes joined.
pub fn draw(pools: &Pools, clusters: &Clusters, rng: &mut Rng) -> Option<Recipe> {
    let mut used = BTreeSet::new();
    let lead = draw_unused(&pools.held_out, &mut used, clusters, rng)?;
    let pool = |held_out: bool| {
        if held_out {
            &pools.held_out
        } else {
            &pools.indexed
        }
    };
    let target_held_out = rng.unit() < HELD_OUT_SHARE;
    let target = draw_unused(pool(target_held_out), &mut used, clusters, rng)?;
    let partner_held_out = rng.unit() < HELD_OUT_SHARE;
    let partner = draw_unused(pool(partner_held_out), &mut used, clusters, rng)?;
    let target_duration = target.header.duration_seconds;
    let fastest = 1.0 + SPEED_LIMIT + RIDE_CHANGE.1;
    let target_seconds = between(TARGET_SECONDS, rng.unit()).min(0.85 * target_duration / fastest);
    let base = Base {
        lead: placed(lead, true, LEAD_SECONDS, rng),
        target: placed(target, target_held_out, target_seconds * fastest, rng),
        target_seconds,
        partner: placed(
            partner,
            partner_held_out,
            BLEND_SECONDS.1 + PARTNER_SECONDS,
            rng,
        ),
    };
    let mut transformations = Vec::new();
    let speed = 1.0 + (2.0 * rng.unit() - 1.0) * SPEED_RANGE;
    let sped = rng.unit() < AXIS_SHARE;
    if sped {
        transformations.push(Transformation::Speed { speed });
    }
    let (change, at, seconds) = (
        between(RIDE_CHANGE, rng.unit()) * if rng.unit() < 0.5 { 1.0 } else { -1.0 },
        between((0.3, 0.7), rng.unit()),
        between(RIDE_SECONDS, rng.unit()),
    );
    if rng.unit() < AXIS_SHARE {
        let from = if sped { speed } else { 1.0 };
        let change = if (from + change - 1.0).abs() > SPEED_LIMIT {
            -change
        } else {
            change
        };
        transformations.push(Transformation::Ride {
            change,
            at,
            seconds,
        });
    }
    let kill = between(KILL_SECONDS, rng.unit());
    if rng.unit() < AXIS_SHARE {
        transformations.push(Transformation::BassKill { seconds: kill });
    }
    let (blend, gain_db) = (
        between(BLEND_SECONDS, rng.unit()),
        between(BLEND_DB, rng.unit()),
    );
    if rng.unit() < AXIS_SHARE {
        transformations.push(Transformation::Blend {
            seconds: blend,
            gain_db,
        });
    }
    let fault = FAULTS[rng.below(FAULTS.len())];
    if rng.unit() < AXIS_SHARE {
        transformations.push(Transformation::ChannelFault {
            fault: fault.to_owned(),
        });
    }
    Some(Recipe {
        base,
        transformations,
    })
}

fn between((low, high): (f64, f64), unit: f64) -> f64 {
    low + unit * (high - low)
}

/// `record` played from a place clear of its first tenth (often a sparse
/// intro) with `needed` seconds of track left before 95% of its length.
fn placed(record: &PeakRecord, held_out: bool, needed: f64, rng: &mut Rng) -> Source {
    let duration = record.header.duration_seconds;
    let earliest = 0.1 * duration;
    let latest = 0.95 * duration - needed;
    Source {
        asset: record.header.source.path.clone(),
        held_out,
        start_seconds: earliest + rng.unit() * (latest - earliest).max(0.0),
    }
}

/// What the search made of a render: the target's level, the longest
/// stretch of its play that no confident detection of it covers, and the
/// files confidently found where no play of theirs is.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Outcome {
    pub target: Level,
    pub uncovered_seconds: f64,
    pub wrong: Vec<WrongAnswer>,
}

/// A confident detection matching no play, and the play it overlaps most.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WrongAnswer {
    pub asset: String,
    pub hits: u32,
    pub over: Option<String>,
}

impl Outcome {
    /// The same level and wrong answers, and the same uncovered stretch
    /// to a microsecond: serde_json's default parsing can read a float of
    /// the recipe one unit in the last place off, and the play's ends,
    /// and the stretch with them, move by as much.
    fn agrees(&self, other: &Outcome) -> bool {
        self.target == other.target
            && self.wrong == other.wrong
            && (self.uncovered_seconds - other.uncovered_seconds).abs() < 1e-6
    }

    /// `accepted`: the files a detection of the target may name (its
    /// cluster and the files judged to carry its title).
    fn of(result: &MixResult, accepted: &BTreeSet<String>) -> Outcome {
        let plays = &result.mix.plays;
        let target = &plays[1];
        let covering: Vec<(f64, f64)> = result
            .detections
            .iter()
            .filter(|found| {
                found.confidence() == Confidence::Confident && accepted.contains(&found.asset)
            })
            .map(|found| (found.start_seconds, found.end_seconds))
            .collect();
        let wrong = result
            .score
            .wrong_confident
            .iter()
            .map(|found| {
                let overlap = |play: &PlannedPlay| {
                    found.end_seconds.min(play.end_seconds)
                        - found.start_seconds.max(play.start_seconds)
                };
                WrongAnswer {
                    asset: found.asset.clone(),
                    hits: found.hits,
                    over: plays
                        .iter()
                        .filter(|play| overlap(play) > 0.0)
                        .max_by(|a, b| overlap(a).total_cmp(&overlap(b)))
                        .map(|play| play.asset.clone()),
                }
            })
            .collect();
        // The scorer's level is its strongest detection's by hits, which
        // can fall short of the rule while a weaker one is confident.
        let found = covering
            .iter()
            .any(|&(start, end)| start < target.end_seconds && target.start_seconds < end);
        Outcome {
            target: if found {
                Level::Confident
            } else {
                result.score.plays[1].level
            },
            uncovered_seconds: longest_uncovered(
                target.start_seconds,
                target.end_seconds,
                covering,
            ),
            wrong,
        }
    }

    /// The failures this outcome holds for `base`: the target lost or a
    /// gap in it (when indexed), and each file found wrongly.
    fn failures(&self, base: &Base) -> Vec<Failure> {
        let target = base.target.asset.clone();
        let lost = if base.target.held_out {
            None
        } else if self.target != Level::Confident {
            Some(Failure::Lost { target })
        } else if self.uncovered_seconds >= GAP_SECONDS {
            Some(Failure::Gap { target })
        } else {
            None
        };
        let wrong: BTreeSet<&String> = self.wrong.iter().map(|answer| &answer.asset).collect();
        lost.into_iter()
            .chain(wrong.into_iter().map(|asset| Failure::Wrong {
                asset: asset.clone(),
            }))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Failure {
    /// The indexed target has no confident detection.
    Lost { target: String },
    /// The indexed target is found, but `GAP_SECONDS` or more of its play
    /// has no confident detection of it.
    Gap { target: String },
    /// This file is confidently found where no play of its cluster, or of
    /// a file judged to carry the title, is.
    Wrong { asset: String },
}

impl Failure {
    fn name(&self) -> &'static str {
        match self {
            Failure::Lost { .. } => "lost",
            Failure::Gap { .. } => "gap",
            Failure::Wrong { .. } => "wrong",
        }
    }

    fn holds(&self, outcome: &Outcome) -> bool {
        match self {
            Failure::Lost { .. } => outcome.target != Level::Confident,
            Failure::Gap { .. } => {
                outcome.target == Level::Confident && outcome.uncovered_seconds >= GAP_SECONDS
            }
            Failure::Wrong { asset } => outcome.wrong.iter().any(|answer| &answer.asset == asset),
        }
    }
}

/// The longest stretch of `start..end` that none of `spans` covers.
fn longest_uncovered(start: f64, end: f64, mut spans: Vec<(f64, f64)>) -> f64 {
    spans.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut longest: f64 = 0.0;
    let mut covered_to = start;
    for (from, to) in spans {
        longest = longest.max(from.min(end) - covered_to);
        covered_to = covered_to.max(to);
    }
    longest.max(end - covered_to)
}

/// The smallest list of transformations, by greedy steps, for which
/// `fails` still holds: none if it holds without any; else each one
/// dropped while it holds without it, then each left halved, up to
/// `MAX_SHRINKS` times, while it holds halved. `fails` is assumed to hold
/// for `transformations`.
pub fn minimized(
    transformations: Vec<Transformation>,
    mut fails: impl FnMut(&[Transformation]) -> bool,
) -> Vec<Transformation> {
    if fails(&[]) {
        return Vec::new();
    }
    let mut kept = transformations;
    let mut dropped_one = true;
    while dropped_one && kept.len() > 1 {
        dropped_one = false;
        for position in 0..kept.len() {
            let mut fewer = kept.clone();
            fewer.remove(position);
            if fails(&fewer) {
                kept = fewer;
                dropped_one = true;
                break;
            }
        }
    }
    for position in 0..kept.len() {
        for _ in 0..MAX_SHRINKS {
            let Some(smaller) = kept[position].halved() else {
                break;
            };
            let mut trial = kept.clone();
            trial[position] = smaller;
            if !fails(&trial) {
                break;
            }
            kept = trial;
        }
    }
    kept
}

/// A play of the mix as the truth has it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expected {
    pub asset: String,
    pub held_out: bool,
    pub start_seconds: f64,
    pub end_seconds: f64,
}

/// A failure found in a sample, its minimal recipe and what the minimal
/// recipe's render gave.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterexample {
    pub number: usize,
    pub sample: usize,
    pub failure: Failure,
    /// `failure`'s kind and the minimal recipe's kinds of transformation,
    /// such as `lost: bass-kill+blend` (`none`: the base alone fails).
    pub class: String,
    /// The recording family: the first file of the target's cluster for a
    /// play lost; for a wrong answer, the wrong file's and the one of the
    /// play it overlaps.
    pub family: String,
    pub sampled: Recipe,
    pub minimal: Recipe,
    pub expected: Vec<Expected>,
    pub minimal_outcome: Outcome,
    pub evaluations: usize,
}

/// One recipe file under `recipes/`: the counterexample, and how to
/// render and search it again.
#[derive(Serialize, Deserialize)]
pub struct SavedRecipe {
    pub seed: u64,
    pub build: String,
    pub ladder: String,
    pub indexed_assets: usize,
    pub counterexample: Counterexample,
}

#[derive(Serialize, Deserialize)]
pub struct Sampled {
    pub sample: usize,
    pub recipe: Recipe,
    pub outcome: Outcome,
}

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: String,
    pub failures: usize,
    pub families: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CounterexampleReport {
    pub seed: u64,
    pub build: String,
    pub ladder: String,
    pub indexed_assets: usize,
    pub samples: usize,
    /// Renders searched: one per sample, then the minimizer's.
    pub evaluations: usize,
    pub sampling_seconds: f64,
    pub minimizing_seconds: f64,
    pub sampled: Vec<Sampled>,
    pub counterexamples: Vec<Counterexample>,
    pub classes: Vec<Class>,
}

pub struct Options<'a> {
    pub seed: u64,
    pub samples: usize,
    pub same_titles: &'a SameTitles,
    pub panels: &'a Path,
    pub ladder_name: &'a str,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// The index's records beside the corpus's; the samples play corpus
    /// files only.
    pub padding: &'a Padding,
    /// Where the report, the recipes and the renders, while they are
    /// searched, go.
    pub out: &'a Path,
    pub jobs: usize,
}

/// Renders a recipe, searches it on one thread, scores it and deletes
/// the render.
struct Evaluator<'a> {
    roots: Roots<'a>,
    clusters: &'a Clusters,
    titles: &'a SameTitles,
    index: &'a Index,
    ladder: &'a [Rung],
    matching: &'a Matching,
    renders: PathBuf,
}

impl Evaluator<'_> {
    fn outcome(&self, recipe: &Recipe, name: &str) -> Result<Outcome, String> {
        Ok(self.searched(recipe, name)?.0)
    }

    /// The outcome, and every detection of two windows or more.
    fn searched(&self, recipe: &Recipe, name: &str) -> Result<(Outcome, Vec<MixFound>), String> {
        let (mix, fault) = recipe.planned(name)?;
        let path = self.renders.join(format!("{name}.mp3"));
        let searcher = Searcher {
            index: self.index,
            ladder: self.ladder,
            matching: self.matching,
            verifier: None,
            fault,
        };
        let result = search_mix(
            &mix,
            &path,
            &self.roots,
            self.clusters,
            self.titles,
            &searcher,
        );
        let leftovers = [
            path.with_extension("mp3.complete"),
            path.with_extension("truth.json"),
            path,
        ];
        for leftover in leftovers {
            // A render that failed part-way may have left any of these.
            let _ = fs::remove_file(leftover);
        }
        let cluster = self.clusters.cluster_of(&recipe.base.target.asset);
        let accepted = cluster
            .union(&self.titles.credited(self.clusters, &cluster))
            .cloned()
            .collect();
        let result = result?;
        Ok((Outcome::of(&result, &accepted), result.detections))
    }
}

/// The sweep index of `options.seed`, as the generated mixes search.
fn sweep(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    options: &Options,
) -> Result<SweepIndex, String> {
    let sweep = sweep_index(
        load_records(
            library,
            store,
            &options.matching.profile(),
            &BTreeSet::new(),
        )
        .0,
        clusters,
        options.seed,
        options.panels,
        options.matching,
        options.padding,
        &BTreeSet::new(),
    )?;
    options
        .same_titles
        .warn_outside(&sweep.index, &sweep.held_out);
    Ok(sweep)
}

impl<'a> Evaluator<'a> {
    fn new(
        library: &'a Library,
        clusters: &'a Clusters,
        index: &'a Index,
        options: &'a Options,
    ) -> Evaluator<'a> {
        Evaluator {
            roots: Roots {
                library: &library.root,
                other: None,
            },
            clusters,
            titles: options.same_titles,
            index,
            ladder: options.ladder,
            matching: options.matching,
            renders: options.out.join("renders"),
        }
    }
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    options: &Options,
) -> Result<CounterexampleReport, String> {
    let SweepIndex {
        records,
        held_out,
        index,
    } = sweep(library, store, clusters, options)?;
    let pools = Pools::new(&records, &held_out);
    let mut rng = Rng::new(options.seed);
    let recipes: Vec<Recipe> = (0..options.samples)
        .map(|_| draw(&pools, clusters, &mut rng))
        .collect::<Option<_>>()
        .ok_or("the library has too few long tracks for the samples")?;
    let evaluator = Evaluator::new(library, clusters, &index, options);
    let numbered: Vec<(usize, &Recipe)> = recipes.iter().enumerate().collect();
    let started = Instant::now();
    let sampled: Vec<Sampled> = map_in_order(&numbered, options.jobs, |&(sample, recipe)| {
        Ok(Sampled {
            sample,
            recipe: recipe.clone(),
            outcome: evaluator.outcome(recipe, &format!("sample-{sample:05}"))?,
        })
    })
    .into_iter()
    .collect::<Result<_, String>>()?;
    let sampling_seconds = started.elapsed().as_secs_f64();
    let found: Vec<(usize, usize, Failure)> = sampled
        .iter()
        .flat_map(|sampled| {
            sampled
                .outcome
                .failures(&sampled.recipe.base)
                .into_iter()
                .map(move |failure| (sampled.sample, failure))
        })
        .enumerate()
        .map(|(number, (sample, failure))| (number, sample, failure))
        .collect();
    let started = Instant::now();
    let counterexamples: Vec<Counterexample> =
        map_in_order(&found, options.jobs, |(number, sample, failure)| {
            minimize(
                *number,
                *sample,
                failure,
                &recipes[*sample],
                &evaluator,
                clusters,
            )
        })
        .into_iter()
        .collect::<Result<_, String>>()?;
    let minimizing_seconds = started.elapsed().as_secs_f64();
    let build = std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or_default();
    let recipes_dir = options.out.join("recipes");
    for counterexample in &counterexamples {
        write_json(
            &recipes_dir.join(format!("{:04}.json", counterexample.number)),
            &SavedRecipe {
                seed: options.seed,
                build: build.clone(),
                ladder: options.ladder_name.to_owned(),
                indexed_assets: index.assets().len(),
                counterexample: counterexample.clone(),
            },
        )?;
    }
    Ok(CounterexampleReport {
        seed: options.seed,
        build,
        ladder: options.ladder_name.to_owned(),
        indexed_assets: index.assets().len(),
        samples: options.samples,
        evaluations: options.samples
            + counterexamples
                .iter()
                .map(|counterexample| counterexample.evaluations)
                .sum::<usize>(),
        sampling_seconds,
        minimizing_seconds,
        classes: classes(&counterexamples),
        sampled,
        counterexamples,
    })
}

/// A saved recipe rendered and searched again.
#[derive(Serialize, Deserialize)]
pub struct Replay {
    pub failure: Failure,
    pub class: String,
    pub holds: bool,
    /// The outcome agrees with the one saved with the recipe
    /// (`Outcome::agrees`).
    pub same_outcome: bool,
    pub outcome: Outcome,
    /// Every detection of two windows or more in the render.
    pub detections: Vec<MixFound>,
}

/// Renders and searches `saved`'s minimal recipe again, under the index
/// of `options.seed`.
pub fn replay(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    options: &Options,
    saved: &SavedRecipe,
) -> Result<Replay, String> {
    if saved.seed != options.seed || saved.ladder != options.ladder_name {
        return Err(format!(
            "the recipe was found with seed {} and the {} ladder: replay it with --seed {} and that ladder",
            saved.seed, saved.ladder, saved.seed
        ));
    }
    let SweepIndex { index, .. } = sweep(library, store, clusters, options)?;
    if index.assets().len() != saved.indexed_assets {
        return Err(format!(
            "the recipe was found with {} files indexed, this index holds {}: replay it with the other library it was found with (--other-peaks-dir), or without",
            saved.indexed_assets,
            index.assets().len()
        ));
    }
    let evaluator = Evaluator::new(library, clusters, &index, options);
    let counterexample = &saved.counterexample;
    let (outcome, detections) = evaluator.searched(
        &counterexample.minimal,
        &format!("replay-{:05}", counterexample.number),
    )?;
    Ok(Replay {
        failure: counterexample.failure.clone(),
        class: counterexample.class.clone(),
        holds: counterexample.failure.holds(&outcome),
        same_outcome: outcome.agrees(&counterexample.minimal_outcome),
        outcome,
        detections,
    })
}

/// Minimizes one failure of sample `sample` and renders its minimal
/// recipe once more for the outcome it keeps.
fn minimize(
    number: usize,
    sample: usize,
    failure: &Failure,
    sampled: &Recipe,
    evaluator: &Evaluator,
    clusters: &Clusters,
) -> Result<Counterexample, String> {
    let mut evaluations = 0;
    let mut error = None;
    let minimal = minimized(sampled.transformations.clone(), |trial| {
        evaluations += 1;
        let name = format!("failure-{number:05}-{evaluations:03}");
        match evaluator.outcome(&sampled.with(trial), &name) {
            Ok(outcome) => failure.holds(&outcome),
            Err(message) => {
                error.get_or_insert(message);
                false
            }
        }
    });
    if let Some(message) = error {
        return Err(message);
    }
    let minimal = sampled.with(&minimal);
    evaluations += 1;
    let minimal_outcome = evaluator.outcome(&minimal, &format!("failure-{number:05}-minimal"))?;
    let kinds: Vec<&str> = minimal
        .transformations
        .iter()
        .map(Transformation::kind)
        .collect();
    let first_of = |asset: &str| {
        clusters
            .cluster_of(asset)
            .into_iter()
            .next()
            .unwrap_or_else(|| asset.to_owned())
    };
    let family = match failure {
        Failure::Lost { target } | Failure::Gap { target } => first_of(target),
        Failure::Wrong { asset } => {
            let over = minimal_outcome
                .wrong
                .iter()
                .find(|answer| &answer.asset == asset)
                .and_then(|answer| answer.over.as_deref())
                .map_or_else(|| String::from("nothing"), first_of);
            format!("{} over {over}", first_of(asset))
        }
    };
    let (mix, _) = minimal.planned("expected")?;
    Ok(Counterexample {
        number,
        sample,
        class: format!(
            "{}: {}",
            failure.name(),
            if kinds.is_empty() {
                String::from("none")
            } else {
                kinds.join("+")
            }
        ),
        family,
        failure: failure.clone(),
        sampled: sampled.clone(),
        expected: mix
            .plays
            .iter()
            .map(|play| Expected {
                asset: play.asset.clone(),
                held_out: play.held_out,
                start_seconds: play.start_seconds,
                end_seconds: play.end_seconds,
            })
            .collect(),
        minimal,
        minimal_outcome,
        evaluations,
    })
}

/// Failures grouped by class, each with its distinct families.
fn classes(counterexamples: &[Counterexample]) -> Vec<Class> {
    let mut grouped: BTreeMap<&str, (usize, BTreeSet<&str>)> = BTreeMap::new();
    for counterexample in counterexamples {
        let entry = grouped.entry(&counterexample.class).or_default();
        entry.0 += 1;
        entry.1.insert(&counterexample.family);
    }
    grouped
        .into_iter()
        .map(|(class, (failures, families))| Class {
            class: class.to_owned(),
            failures,
            families: families.into_iter().map(str::to_owned).collect(),
        })
        .collect()
}

pub fn print_summary(report: &CounterexampleReport) {
    let lost = report
        .sampled
        .iter()
        .filter(|sampled| {
            !sampled.recipe.base.target.held_out && sampled.outcome.target != Level::Confident
        })
        .count();
    let wrong = report
        .sampled
        .iter()
        .filter(|sampled| !sampled.outcome.wrong.is_empty())
        .count();
    println!(
        "counterexamples, seed {}, {} ladder, {} indexed: {} samples, {lost} with the target lost, {wrong} with a wrong answer",
        report.seed, report.ladder, report.indexed_assets, report.samples
    );
    println!(
        "{} renders searched; sampling {:.0} s, minimizing {:.0} s",
        report.evaluations, report.sampling_seconds, report.minimizing_seconds
    );
    for class in &report.classes {
        println!(
            "  {:<40} {:>4} failures, {:>3} families",
            class.class,
            class.failures,
            class.families.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audition::audio::tests::tone;
    use crate::render::RENDER_RATE;

    fn speed(speed: f64) -> Transformation {
        Transformation::Speed { speed }
    }

    fn fault() -> Transformation {
        Transformation::ChannelFault {
            fault: String::from("stereo-inverted"),
        }
    }

    fn kill(seconds: f64) -> Transformation {
        Transformation::BassKill { seconds }
    }

    /// Fails when a speed 2% or more from native and a channel fault play
    /// together.
    fn fast_and_faulted(trial: &[Transformation]) -> bool {
        let fast = trial.iter().any(|transformation| {
            matches!(transformation, Transformation::Speed { speed } if (speed - 1.0).abs() >= 0.02)
        });
        fast && trial.contains(&fault())
    }

    #[test]
    fn the_minimizer_keeps_only_what_the_failure_needs_at_the_smallest_size_that_fails() {
        let mut calls = 0;
        let minimal = minimized(
            vec![
                kill(40.0),
                speed(1.06),
                Transformation::Blend {
                    seconds: 30.0,
                    gain_db: -3.0,
                },
                fault(),
            ],
            |trial| {
                calls += 1;
                fast_and_faulted(trial)
            },
        );
        // 6% halves to 3%, which still fails; 1.5% does not.
        assert_eq!(minimal, vec![speed(1.03), fault()]);
        assert!(fast_and_faulted(&minimal));
        for position in 0..minimal.len() {
            let mut fewer = minimal.clone();
            fewer.remove(position);
            assert!(!fast_and_faulted(&fewer));
        }
        assert!(calls <= 12, "{calls} calls");
    }

    #[test]
    fn the_longest_uncovered_stretch_counts_both_ends_and_overlapping_spans() {
        assert_eq!(longest_uncovered(30.0, 100.0, Vec::new()), 70.0);
        assert_eq!(
            longest_uncovered(30.0, 100.0, vec![(60.0, 80.0), (20.0, 40.0), (35.0, 50.0)]),
            20.0
        );
        assert_eq!(longest_uncovered(30.0, 100.0, vec![(32.0, 64.0)]), 36.0);
        assert_eq!(longest_uncovered(30.0, 100.0, vec![(0.0, 120.0)]), 0.0);
    }

    #[test]
    fn outcomes_agree_across_a_float_read_one_unit_off() {
        let outcome = Outcome {
            target: Level::Confident,
            uncovered_seconds: 44.580_687_133_200_925,
            wrong: Vec::new(),
        };
        let read = Outcome {
            uncovered_seconds: 44.580_687_133_200_91,
            ..outcome.clone()
        };
        assert!(outcome.agrees(&read));
        let possible = Outcome {
            target: Level::Possible,
            ..read
        };
        assert!(!outcome.agrees(&possible));
    }

    fn found(
        asset: &str,
        start_seconds: f64,
        end_seconds: f64,
        windows: u32,
        hits: u32,
    ) -> MixFound {
        MixFound {
            asset: asset.to_owned(),
            start_seconds,
            end_seconds,
            track_start_seconds: 0.0,
            speed: 1.0,
            windows,
            hits,
            fitted: true,
            verified: None,
        }
    }

    #[test]
    fn a_target_is_found_when_any_detection_of_it_is_confident() {
        let (mix, _) = recipe(Vec::new()).planned("mix").unwrap();
        // The scorer's best by hits spans 2 windows, short of the rule;
        // the weaker one is confident.
        let strong_but_short = found("target.mp3", 30.0, 33.0, 2, 3000);
        let confident = found("target.mp3", 31.0, 35.0, 5, 1000);
        assert_eq!(strong_but_short.confidence(), Confidence::Possible);
        assert_eq!(confident.confidence(), Confidence::Confident);
        let result = MixResult {
            score: crate::mixes::MixScore {
                plays: mix
                    .plays
                    .iter()
                    .map(|play| crate::mixes::PlayScore {
                        truth: play.clone(),
                        level: Level::Possible,
                        best: None,
                        start_late_seconds: None,
                        end_early_seconds: None,
                        position_error_seconds: None,
                    })
                    .collect(),
                wrong_confident: Vec::new(),
                wrong_possible: Vec::new(),
                false_candidates: Vec::new(),
                same_title: 0,
            },
            mix,
            detections: vec![strong_but_short, confident],
        };
        let accepted = BTreeSet::from([String::from("target.mp3")]);
        let outcome = Outcome::of(&result, &accepted);
        assert_eq!(outcome.target, Level::Confident);
        assert!((outcome.uncovered_seconds - 1.0).abs() < 1e-9);
        let elsewhere = BTreeSet::from([String::from("partner.mp3")]);
        assert_eq!(Outcome::of(&result, &elsewhere).target, Level::Possible);
    }

    #[test]
    fn the_minimizer_returns_nothing_when_the_base_alone_fails() {
        let minimal = minimized(vec![kill(40.0), fault()], |_| true);
        assert!(minimal.is_empty());
    }

    #[test]
    fn the_minimizer_keeps_a_single_transformation_it_needs() {
        let minimal = minimized(vec![fault()], |trial| trial.contains(&fault()));
        assert_eq!(minimal, vec![fault()]);
    }

    fn recipe(transformations: Vec<Transformation>) -> Recipe {
        let source = |asset: &str, held_out: bool, start_seconds: f64| Source {
            asset: asset.to_owned(),
            held_out,
            start_seconds,
        };
        Recipe {
            base: Base {
                lead: source("lead.mp3", true, 1.0),
                target: source("target.mp3", false, 2.0),
                target_seconds: 6.0,
                partner: source("partner.mp3", false, 0.5),
            },
            transformations,
        }
    }

    #[test]
    fn a_recipe_plans_the_target_second_with_its_transformations() {
        let (mix, fault) = recipe(vec![
            speed(1.04),
            Transformation::Ride {
                change: -0.02,
                at: 0.5,
                seconds: 2.0,
            },
            kill(10.0),
            Transformation::Blend {
                seconds: 10.0,
                gain_db: -6.0,
            },
            fault(),
        ])
        .planned("mix")
        .unwrap();
        assert!(matches!(fault, Some(Fault::Inverted)));
        let [lead, target, partner] = &mix.plays[..] else {
            panic!("three plays")
        };
        assert_eq!((lead.start_seconds, lead.end_seconds), (0.0, 30.0));
        assert_eq!((target.start_seconds, target.end_seconds), (30.0, 36.0));
        assert_eq!(target.speed, 1.04);
        let ride = target.ride.unwrap();
        assert_eq!(ride.start_seconds, 33.0);
        assert!((ride.to_speed - 1.02).abs() < 1e-12);
        // The kill and the blend last at most the target's length.
        assert_eq!(target.bass_cut_until, Some(36.0));
        assert_eq!((partner.start_seconds, partner.end_seconds), (30.0, 66.0));
        assert_eq!(partner.gain_db, -6.0);
        assert_eq!(mix.seconds, 66.0);
        let (plain, no_fault) = recipe(Vec::new()).planned("mix").unwrap();
        assert!(no_fault.is_none());
        assert_eq!(plain.plays[1].speed, 1.0);
        assert_eq!(plain.plays[2].start_seconds, 36.0);
    }

    fn scratch(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "gunfinger-counterexamples-{test}-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn decoded(path: &Path) -> Vec<f32> {
        crate::render::render_samples(
            path,
            0.0,
            1000.0,
            crate::render::Playback::Turntable(1.0),
            None,
        )
        .unwrap()
    }

    #[test]
    fn a_saved_recipe_renders_the_same_audio_again() {
        let dir = scratch("round-trip");
        let library = dir.join("library");
        fs::create_dir_all(&library).unwrap();
        for (file, frequency) in [("lead.mp3", 300), ("target.mp3", 500), ("partner.mp3", 700)] {
            if tone(&library.join(file), 50.0, frequency).is_none() {
                eprintln!("skipped: FFmpeg not found");
                return;
            }
        }
        // Values as `draw` gives them, with every digit, which serde_json
        // may read one unit in the last place off.
        let mut original = recipe(vec![
            speed(1.031_740_897_331_802_7),
            Transformation::Ride {
                change: 0.010_179_344_104_904_602,
                at: 0.350_503_553_466_155_64,
                seconds: 1.461_024_892_425_437,
            },
            kill(3.077_418_713_320_091),
            Transformation::Blend {
                seconds: 2.674_214_316_824_21,
                gain_db: -3.419_597_847_919_715_3,
            },
            Transformation::ChannelFault {
                fault: String::from("stereo-noise-snr-0db"),
            },
        ]);
        original.base.target_seconds = 6.774_187_133_200_91;
        original.base.target.start_seconds = 2.664_248_054_966_72;
        let saved = dir.join("recipe.json");
        write_json(&saved, &original).unwrap();
        let loaded: Recipe = crate::files::read_json(&saved).unwrap();
        assert_eq!(loaded.transformations.len(), original.transformations.len());
        assert!((loaded.base.target_seconds - original.base.target_seconds).abs() < 1e-12);
        let roots = Roots {
            library: &library,
            other: None,
        };
        let mut renders = Vec::new();
        for (name, recipe) in [("first", &original), ("again", &loaded)] {
            let (mix, fault) = recipe.planned(name).unwrap();
            let path = dir.join(format!("{name}.mp3"));
            crate::mixes::render(&mix, &roots, fault, &path).unwrap();
            renders.push((fs::read(&path).unwrap(), decoded(&path)));
        }
        assert_eq!(renders[0].0, renders[1].0);
        let samples = &renders[0].1;
        // 66.8 s of mix at the render rate, give or take the encoder's padding.
        let expected = 66.774 * f64::from(RENDER_RATE);
        assert!((samples.len() as f64 - expected).abs() < 0.05 * expected);
        assert!(samples.iter().any(|&sample| sample.abs() > 0.01));
        fs::remove_dir_all(&dir).unwrap();
    }
}

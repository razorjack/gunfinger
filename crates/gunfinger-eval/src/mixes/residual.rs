//! `mixes --kind drops --residual`: proposal 4's pilot (brief 65). Every
//! play of an indexed record that the search of the whole mix finds
//! confidently is a record A; its span of the mix is searched again in
//! four ways (`ArmName`), among them with what A explains taken out
//! (`crate::residual`), and each way is scored against the truth.
//!
//! A's contribution is rendered as `render` mixed it, from a time map: the
//! generator's (`Timing::Truth`), from another rip of A's recording where
//! the index holds one, or the line of A's confident detection
//! (`Timing::Detected`).

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::index::Index;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search_peaks, search_signal_with};
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::store::PeakRecord;
use serde::{Deserialize, Serialize};

use super::{
    MixFound, MixResult, PlannedMix, PlannedPlay, PlayScore, RENDER_RATE, Scenario, Searcher, Side,
    play_audio, score,
};
use crate::clusters::Clusters;
use crate::render::{Encoding, encode};
use crate::residual::{Rule, residual};
use crate::scoring::Level;
use crate::shared::Roots;
use crate::stereo::QueryAudio;
use crate::titles::SameTitles;

/// The query's peaks of the denser arm are picked in a neighbourhood of
/// this half-size in frames and bins instead of the profile's 12: about
/// twice as many peaks. The index is unchanged.
const DENSER_NEIGHBOURHOOD: usize = 8;
/// Threads each excerpt's search takes; mixes are searched side by side.
const SEARCH_JOBS: usize = 2;

#[derive(Debug, Clone, Copy)]
pub struct Pilot {
    pub rule: Rule,
    pub timing: Timing,
}

/// Where A's time map comes from.
#[derive(clap::ValueEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Timing {
    /// The generator's: source start, speed, fades, level and bass cuts.
    Truth,
    /// The line of A's strongest confident detection, in the file it
    /// names, without fades or bass cuts (the gains follow them).
    Detected,
}

impl Timing {
    pub fn name(self) -> &'static str {
        match self {
            Timing::Truth => "truth",
            Timing::Detected => "detected",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub rule: Rule,
    pub timing: Timing,
    pub cases: Vec<Case>,
}

/// One record A found confidently, and its span searched each way.
#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub mix: String,
    /// A's place among the mix's plays.
    pub play: usize,
    /// The file A's contribution was rendered from.
    pub reference: String,
    pub reference_kind: ReferenceKind,
    /// The excerpt: A's audible span.
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub arms: Vec<Arm>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceKind {
    /// The file the mix plays: the mask leans on identical audio.
    Played,
    /// Another indexed file of A's recording, placed by its alignment with
    /// the played file's stored peaks.
    OtherRip,
    /// The file of A's confident detection, placed by its line.
    Detected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArmName {
    /// The excerpt as the mix holds it.
    Plain,
    /// The excerpt with denser query peaks (`DENSER_NEIGHBOURHOOD`).
    Denser,
    /// The masked residual.
    Masked,
    /// The masked residual searched without the recordings of the quiet
    /// and brief records of the seed's mixes (A of such a block only).
    MaskedWithoutPartners,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Arm {
    pub name: ArmName,
    /// The plays overlapping the excerpt, A's among them.
    pub plays: Vec<ArmPlay>,
    pub wrong_confident: Vec<MixFound>,
    pub wrong_possible: Vec<MixFound>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmPlay {
    pub play: usize,
    pub level: Level,
    pub best: Option<MixFound>,
}

/// What the cases are searched and scored with.
pub struct Context<'a> {
    pub searcher: &'a Searcher<'a>,
    /// The index less the quiet and brief records' recordings.
    pub without_partners: &'a Index,
    /// Every record loaded, for other rips and their peaks.
    pub records: &'a [PeakRecord],
    pub roots: &'a Roots<'a>,
    pub clusters: &'a Clusters,
    pub titles: &'a SameTitles,
    /// Where references are written as WAV for a moment, to be decoded at
    /// the profile's rate as the mix is.
    pub scratch: &'a Path,
    pub pilot: Pilot,
    pub jobs: usize,
}

/// Whether a play is a quiet or brief record inside a stronger one, whose
/// recording `Context::without_partners` leaves out.
pub fn is_partner(play: &PlannedPlay) -> bool {
    play.side == Some(Side::Second)
        && matches!(play.scenario, Some(Scenario::Quiet | Scenario::Brief))
}

pub fn run(
    planned: &[(PlannedMix, PathBuf)],
    results: &[MixResult],
    context: &Context,
) -> Result<Report, String> {
    fs::create_dir_all(context.scratch).map_err(|error| error.to_string())?;
    let mixes: Vec<(&(PlannedMix, PathBuf), &MixResult)> = planned.iter().zip(results).collect();
    let cases = map_in_order(&mixes, context.jobs, |((mix, path), result)| {
        mix_cases(mix, path, result, context)
    });
    Ok(Report {
        rule: context.pilot.rule,
        timing: context.pilot.timing,
        cases: cases
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .flatten()
            .collect(),
    })
}

fn mix_cases(
    mix: &PlannedMix,
    path: &Path,
    result: &MixResult,
    context: &Context,
) -> Result<Vec<Case>, String> {
    let audio = mono(context.searcher.matching.decode_query(path)?)?;
    result
        .score
        .plays
        .iter()
        .enumerate()
        .filter(|(_, score)| !score.truth.held_out && score.level == Level::Confident)
        .map(|(play, score)| case(mix, play, score, &audio, context))
        .collect()
}

fn case(
    mix: &PlannedMix,
    play: usize,
    score: &PlayScore,
    audio: &[f32],
    context: &Context,
) -> Result<Case, String> {
    let a = &mix.plays[play];
    let (placed, reference_kind) = match context.pilot.timing {
        Timing::Truth => truth_reference(a, context)?,
        Timing::Detected => detected_reference(a, score)?,
    };
    let reference = reference_audio(&placed, context, &format!("{}-{play:02}", mix.name))?;
    let profile = context.searcher.matching.profile();
    let rate = f64::from(profile.sample_rate);
    // The render starts at this sample of the mix (`summed`).
    let start = (a.start_seconds * f64::from(RENDER_RATE)).round() / f64::from(RENDER_RATE);
    let first = ((start * rate).round() as usize).min(audio.len());
    let excerpt = &audio[first..(first + reference.len()).min(audio.len())];
    let offset = first as f64 / rate;
    let masked = residual(excerpt, &reference, &profile, context.pilot.rule);
    let denser = Profile {
        neighbourhood_frames: DENSER_NEIGHBOURHOOD,
        neighbourhood_bins: DENSER_NEIGHBOURHOOD,
        ..profile
    };
    let index = context.searcher.index;
    let span = (offset, offset + excerpt.len() as f64 / rate);
    let arm = |name, index: &Index, samples: &[f32], profile: &Profile| {
        let found = searched(index, samples, profile, offset, context);
        scored(name, mix, &found, span, context)
    };
    let mut arms = vec![
        arm(ArmName::Plain, index, excerpt, &profile),
        arm(ArmName::Denser, index, excerpt, &denser),
        arm(ArmName::Masked, index, &masked, &profile),
    ];
    if mix.plays.get(play + 1).is_some_and(is_partner) && a.side == Some(Side::First) {
        arms.push(arm(
            ArmName::MaskedWithoutPartners,
            context.without_partners,
            &masked,
            &profile,
        ));
    }
    Ok(Case {
        mix: mix.name.clone(),
        play,
        reference: placed.asset,
        reference_kind,
        start_seconds: span.0,
        end_seconds: span.1,
        arms,
    })
}

/// A as the generator placed it, rendered from another indexed file of its
/// recording when there is one that its stored peaks align with.
fn truth_reference(
    a: &PlannedPlay,
    context: &Context,
) -> Result<(PlannedPlay, ReferenceKind), String> {
    let indexed: BTreeSet<&str> = context
        .searcher
        .index
        .assets()
        .iter()
        .map(|asset| asset.path.as_str())
        .collect();
    let record_of = |path: &str| {
        context
            .records
            .iter()
            .find(|record| record.header.source.path == path)
    };
    let rip = context
        .clusters
        .cluster_of(&a.asset)
        .into_iter()
        .filter(|path| *path != a.asset && indexed.contains(path.as_str()))
        .find_map(|path| record_of(&path));
    let line = rip
        .zip(record_of(&a.asset))
        .and_then(|(rip, played)| rip_line(played, rip, context).map(|line| (rip, line)));
    let Some((rip, line)) = line else {
        return Ok((a.clone(), ReferenceKind::Played));
    };
    // The played file's time `f` is the rip's `line.start_seconds + (f -
    // line.track_start_seconds) / speed`.
    let speed = line.speed.0;
    let placed = PlannedPlay {
        asset: rip.header.source.path.clone(),
        source_start_seconds: line.start_seconds
            + (a.source_start_seconds - line.track_start_seconds) / speed,
        speed: a.speed / speed,
        ..a.clone()
    };
    Ok((placed, ReferenceKind::OtherRip))
}

/// The strongest line of `rip`'s stored peaks in `played`'s: the rip's
/// time as query, the played file's as reference.
fn rip_line(played: &PeakRecord, rip: &PeakRecord, context: &Context) -> Option<Detection> {
    let index = Index::build(std::slice::from_ref(played)).ok()?;
    let profile = context.searcher.matching.profile();
    search_peaks(&index, &rip.peaks, &profile, context.searcher.ladder, 1)
        .into_iter()
        .max_by_key(|detection| detection.evidence.hits)
}

/// A as its strongest matching detection places it: that file, from where
/// its line puts the play's start, at its speed.
fn detected_reference(
    a: &PlannedPlay,
    score: &PlayScore,
) -> Result<(PlannedPlay, ReferenceKind), String> {
    let best = score
        .best
        .as_ref()
        .ok_or_else(|| format!("{} is confident without a detection", a.asset))?;
    let placed = PlannedPlay {
        asset: best.asset.clone(),
        source_start_seconds: (best.track_start_seconds
            + best.speed * (a.start_seconds - best.start_seconds))
            .max(0.0),
        speed: best.speed,
        fade_in_seconds: 0.0,
        fade_out_seconds: 0.0,
        bass_cut_until: None,
        bass_cut_from: None,
        ride: None,
        gain_db: 0.0,
        ..a.clone()
    };
    Ok((placed, ReferenceKind::Detected))
}

/// `placed` rendered as `render` mixes a play, at the profile's rate
/// through the decoder the mix is searched with.
fn reference_audio(
    placed: &PlannedPlay,
    context: &Context,
    name: &str,
) -> Result<Vec<f32>, String> {
    let [samples] = play_audio(placed, context.roots, [None])?;
    // The level is fitted from the mixture; this only keeps WAV from
    // clipping.
    let peak = samples
        .iter()
        .fold(0.0_f32, |peak, sample| peak.max(sample.abs()))
        .max(1e-9);
    let scaled: Vec<f32> = samples.iter().map(|sample| sample * 0.5 / peak).collect();
    let path = context.scratch.join(format!("{name}.wav"));
    encode(&scaled, None, Encoding::Lossless, &path)?;
    let decoded = context.searcher.matching.decode_query(&path);
    fs::remove_file(&path).map_err(|error| format!("{}: {error}", path.display()))?;
    mono(decoded?)
}

fn mono(audio: QueryAudio) -> Result<Vec<f32>, String> {
    match audio {
        QueryAudio::Mono(samples) => Ok(samples),
        QueryAudio::PowerSum(_) => Err(String::from(
            "--residual analyses the downmix; leave --front-end at its default",
        )),
    }
}

/// The detections in `samples`, an excerpt from `offset` seconds of the
/// mix, in mix time.
fn searched(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    offset: f64,
    context: &Context,
) -> Vec<MixFound> {
    let matching = context.searcher.matching;
    search_signal_with(
        index,
        Signal::Mono(samples),
        profile,
        context.searcher.ladder,
        SEARCH_JOBS,
        matching.options(),
    )
    .iter()
    .map(|detection| {
        let mut found = MixFound::new(index, detection, None);
        found.start_seconds += offset;
        found.end_seconds += offset;
        found
    })
    .collect()
}

/// `found` scored against the mix's truth, kept to the plays overlapping
/// the excerpt `span`.
fn scored(
    name: ArmName,
    mix: &PlannedMix,
    found: &[MixFound],
    span: (f64, f64),
    context: &Context,
) -> Arm {
    let score = score(mix, found, context.clusters, context.titles);
    let plays = score
        .plays
        .into_iter()
        .enumerate()
        .filter(|(_, play)| play.truth.start_seconds < span.1 && span.0 < play.truth.end_seconds)
        .map(|(play, score)| ArmPlay {
            play,
            level: score.level,
            best: score.best,
        })
        .collect();
    Arm {
        name,
        plays,
        wrong_confident: score.wrong_confident,
        wrong_possible: score.wrong_possible,
    }
}

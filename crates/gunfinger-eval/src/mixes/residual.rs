//! `mixes --kind drops --residual`: proposal 4's pilot (brief 65). Every
//! play of an indexed record that the search of the whole mix finds
//! confidently is a record A; its span of the mix is searched again in
//! four ways (`ArmName`), among them with what A explains taken out
//! (`crate::residual`), and each way is scored against the truth.
//!
//! A's contribution is rendered as `render` mixed it, from a time map: the
//! generator's (`Timing::Truth`) or the line of A's confident detection
//! (`Timing::Detected`); from another rip of A's recording where the index
//! holds one. Brief 70 runs several timings and fixes (`Fix`) in one run,
//! measures what the reference is off by, and stores for each masked
//! detection the evidence the plain excerpt holds of it.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Confidence;
use gunfinger_core::index::Index;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, search_peaks, search_signal_with};
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakRecord;
use serde::{Deserialize, Serialize};

use super::{
    MixFound, MixResult, PlannedMix, PlannedPlay, PlayScore, RENDER_RATE, Scenario, Searcher, Side,
    play_audio, score,
};
use crate::clusters::Clusters;
use crate::padding::OtherLibrary;
use crate::render::{Encoding, Playback, encode, render_samples, render_samples_as_indexed};
use crate::residual::{Fix, Lags, Rule, lags, residual};
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
/// The plain evidence of a masked detection (brief 70, step 3): the plain
/// excerpt over the detection's span and this much either way...
const EVIDENCE_MARGIN_SECONDS: f64 = 5.0;
/// ...searched on the rungs within this share of the detection's speed;
/// a detection there agrees when its speed is as close and its file time
/// at the middle of their overlap is within `EVIDENCE_SECONDS`.
const EVIDENCE_SPEED: f64 = 0.006;
const EVIDENCE_SECONDS: f64 = 1.0;
/// The seek is measured on this much audio, against the decoded samples
/// up to `SEEK_REACH_SECONDS` either way, at the render's rate over
/// `SEEK_DECIMATION`.
const SEEK_SECONDS: f64 = 2.0;
const SEEK_REACH_SECONDS: f64 = 0.5;
const SEEK_DECIMATION: usize = 10;
/// A seek's offset is reported only where the two renders correlate at
/// least this well.
const SEEK_MIN_CORRELATION: f64 = 0.5;

#[derive(Debug, Clone)]
pub struct Pilot {
    pub rule: Rule,
    pub timings: Vec<Timing>,
    pub fixes: Vec<Fix>,
}

impl Pilot {
    /// `residual-<timings>-over-<over>`, then `-fix-<fixes>` unless the
    /// only fix is `off` (session 65's names).
    pub fn name(&self) -> String {
        let timings: Vec<&str> = self.timings.iter().map(|timing| timing.name()).collect();
        let fixes: Vec<&str> = self.fixes.iter().map(|fix| fix.name()).collect();
        let name = format!("residual-{}-over-{}", timings.join("-"), self.rule.over);
        if self.fixes == [Fix::Off] {
            name
        } else {
            format!("{name}-fix-{}", fixes.join("-"))
        }
    }
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
    /// Absent from session 65's reports, which had one `timing` and no
    /// fix; they still load, with their cases' masked arms left out.
    #[serde(default)]
    pub timings: Vec<Timing>,
    #[serde(default)]
    pub fixes: Vec<Fix>,
    pub cases: Vec<Case>,
}

/// One record A found confidently, and its span searched each way.
#[derive(Debug, Serialize, Deserialize)]
pub struct Case {
    pub mix: String,
    /// A's place among the mix's plays.
    pub play: usize,
    /// The excerpt: A's audible span.
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// The played file's seek at A's start against its decoded samples
    /// (`seek_offset_ms`): what the mix holds against the stored peaks'
    /// timeline.
    pub played_seek_ms: Option<f64>,
    /// The plain and denser arms.
    pub arms: Vec<Arm>,
    /// For each timing, A's reference and the masked arms.
    #[serde(default)]
    pub maskings: Vec<Masking>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Masking {
    pub timing: Timing,
    /// The file A's contribution was rendered from.
    pub reference: String,
    pub reference_kind: ReferenceKind,
    /// The rip's alignment with the file the time map is on, by their
    /// stored peaks, when the reference was placed through one.
    pub rip_line: Option<RipLine>,
    /// The reference file's seek at the reference's start.
    pub reference_seek_ms: Option<f64>,
    /// What the reference is off by in the excerpt, before any fix.
    pub lags: Lags,
    pub fits: Vec<Fit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReferenceKind {
    /// The file the mix plays: the mask leans on identical audio.
    Played,
    /// Another indexed file of A's recording.
    OtherRip,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RipLine {
    pub hits: u32,
    pub windows: u32,
    pub speed: f64,
}

/// The masked arms under one fix.
#[derive(Debug, Serialize, Deserialize)]
pub struct Fit {
    pub fix: Fix,
    pub gain_spread_db: Vec<Option<f32>>,
    pub arms: Vec<Arm>,
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
    /// The masked arms: every detection at possible or above, with the
    /// evidence the plain excerpt holds of it.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<PlainEvidence>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArmPlay {
    pub play: usize,
    pub level: Level,
    pub best: Option<MixFound>,
}

/// A masked detection, and the hits of the strongest detection that
/// agrees with it in the plain excerpt searched against its file alone
/// (0 when none does).
#[derive(Debug, Serialize, Deserialize)]
pub struct PlainEvidence {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub hits: u32,
    pub plain_hits: u32,
}

/// What the cases are searched and scored with.
pub struct Context<'a> {
    pub searcher: &'a Searcher<'a>,
    /// The index less the quiet and brief records' recordings.
    pub without_partners: &'a Index,
    /// Every record loaded, for other rips and their peaks...
    pub records: &'a [PeakRecord],
    /// ...and the other library's, at its scale.
    pub other: Option<&'a OtherLibrary>,
    pub roots: &'a Roots<'a>,
    pub clusters: &'a Clusters,
    pub titles: &'a SameTitles,
    /// Where references are written as WAV for a moment, to be decoded at
    /// the profile's rate as the mix is.
    pub scratch: &'a Path,
    pub pilot: &'a Pilot,
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
    // Each WAV is deleted once decoded; their render receipts go with the
    // directory.
    fs::remove_dir_all(context.scratch)
        .map_err(|error| format!("{}: {error}", context.scratch.display()))?;
    Ok(Report {
        rule: context.pilot.rule,
        timings: context.pilot.timings.clone(),
        fixes: context.pilot.fixes.clone(),
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
    let references = context
        .pilot
        .timings
        .iter()
        .map(|&timing| {
            let reference = reference(a, score, timing, context)?;
            let name = format!("{}-{play:02}-{}", mix.name, timing.name());
            let samples = reference_audio(&reference.placed, context, &name)?;
            Ok((timing, reference, samples))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let length = references
        .first()
        .map_or(0, |(_, _, samples)| samples.len());
    let profile = context.searcher.matching.profile();
    let rate = f64::from(profile.sample_rate);
    // The render starts at this sample of the mix (`summed`).
    let start = (a.start_seconds * f64::from(RENDER_RATE)).round() / f64::from(RENDER_RATE);
    let first = ((start * rate).round() as usize).min(audio.len());
    let excerpt = &audio[first..(first + length).min(audio.len())];
    let offset = first as f64 / rate;
    let denser = Profile {
        neighbourhood_frames: DENSER_NEIGHBOURHOOD,
        neighbourhood_bins: DENSER_NEIGHBOURHOOD,
        ..profile
    };
    let index = context.searcher.index;
    let span = (offset, offset + excerpt.len() as f64 / rate);
    let arm = |name, index: &Index, samples: &[f32], profile: &Profile| {
        let found = searched(index, samples, profile, offset, context);
        let mut arm = scored(name, mix, &found, span, context);
        if matches!(name, ArmName::Masked | ArmName::MaskedWithoutPartners) {
            arm.evidence = found
                .iter()
                .filter(|detection| detection.confidence() >= Confidence::Possible)
                .map(|detection| PlainEvidence {
                    asset: detection.asset.clone(),
                    start_seconds: detection.start_seconds,
                    end_seconds: detection.end_seconds,
                    hits: detection.hits,
                    plain_hits: plain_evidence(detection, excerpt, offset, context),
                })
                .collect();
        }
        arm
    };
    let with_partners_out =
        mix.plays.get(play + 1).is_some_and(is_partner) && a.side == Some(Side::First);
    let maskings = references
        .into_iter()
        .map(|(timing, reference, samples)| {
            let fits = context
                .pilot
                .fixes
                .iter()
                .map(|&fix| {
                    let masked = residual(excerpt, &samples, &profile, context.pilot.rule, fix);
                    let mut arms = vec![arm(ArmName::Masked, index, &masked.samples, &profile)];
                    if with_partners_out {
                        arms.push(arm(
                            ArmName::MaskedWithoutPartners,
                            context.without_partners,
                            &masked.samples,
                            &profile,
                        ));
                    }
                    Fit {
                        fix,
                        gain_spread_db: masked.gain_spread_db,
                        arms,
                    }
                })
                .collect();
            Masking {
                timing,
                reference_seek_ms: seek_offset_ms(
                    &reference.placed.asset,
                    reference.placed.source_start_seconds,
                    context.roots,
                ),
                reference: reference.placed.asset,
                reference_kind: reference.kind,
                rip_line: reference.rip_line,
                lags: lags(excerpt, &samples, &profile),
                fits,
            }
        })
        .collect();
    Ok(Case {
        mix: mix.name.clone(),
        play,
        start_seconds: span.0,
        end_seconds: span.1,
        played_seek_ms: seek_offset_ms(&a.asset, a.source_start_seconds, context.roots),
        arms: vec![
            arm(ArmName::Plain, index, excerpt, &profile),
            arm(ArmName::Denser, index, excerpt, &denser),
        ],
        maskings,
    })
}

/// A's reference under one timing.
struct Reference {
    placed: PlannedPlay,
    kind: ReferenceKind,
    rip_line: Option<RipLine>,
}

/// A placed by `timing`'s time map, rendered from another indexed file of
/// its recording when there is one whose stored peaks align with the file
/// the time map is on (0257's rule), else from that file.
fn reference(
    a: &PlannedPlay,
    score: &PlayScore,
    timing: Timing,
    context: &Context,
) -> Result<Reference, String> {
    let placed = match timing {
        Timing::Truth => a.clone(),
        Timing::Detected => detected_reference(a, score)?,
    };
    let kind = |asset: &str| {
        if asset == a.asset {
            ReferenceKind::Played
        } else {
            ReferenceKind::OtherRip
        }
    };
    let indexed: BTreeSet<&str> = context
        .searcher
        .index
        .assets()
        .iter()
        .map(|asset| asset.path.as_str())
        .collect();
    let rip = context
        .clusters
        .cluster_of(&a.asset)
        .into_iter()
        .filter(|path| *path != a.asset && indexed.contains(path.as_str()))
        .find_map(|path| record_of(&path, context).map(|record| (path, record)));
    let Some((rip, rip_record)) = rip else {
        return Ok(Reference {
            kind: kind(&placed.asset),
            placed,
            rip_line: None,
        });
    };
    if rip == placed.asset {
        return Ok(Reference {
            placed,
            kind: ReferenceKind::OtherRip,
            rip_line: None,
        });
    }
    // The other library's records name their files without the prefix the
    // index and the roots use, so the rip keeps the cluster's name.
    let line = record_of(&placed.asset, context).and_then(|on| rip_line(&on, &rip_record, context));
    let Some(line) = line else {
        return Ok(Reference {
            kind: kind(&placed.asset),
            placed,
            rip_line: None,
        });
    };
    let alignment = Alignment {
        rip_seconds: line.start_seconds,
        played_seconds: line.track_start_seconds,
        speed: line.speed.0,
    };
    Ok(Reference {
        placed: on_rip(&placed, rip, alignment),
        kind: ReferenceKind::OtherRip,
        rip_line: Some(RipLine {
            hits: line.evidence.hits,
            windows: line.evidence.windows,
            speed: line.speed.0,
        }),
    })
}

/// The stored peaks of `path` (a name as the index has it), from the
/// records loaded or the other library's store.
fn record_of(path: &str, context: &Context) -> Option<PeakRecord> {
    context
        .records
        .iter()
        .find(|record| record.header.source.path == path)
        .cloned()
        .or_else(|| match context.other?.record(path)? {
            Ok(record) => Some(record),
            Err(error) => {
                eprintln!("residual: {error}; the record is left out");
                None
            }
        })
}

/// Where a rip lines up with the played file: the rip's time
/// `rip_seconds` is the played file's `played_seconds`, and the played
/// file runs `speed` times as fast as the rip.
#[derive(Debug, Clone, Copy)]
struct Alignment {
    rip_seconds: f64,
    played_seconds: f64,
    speed: f64,
}

/// `a` played from `rip` instead: the played file's time `f` is the rip's
/// `rip_seconds + (f - played_seconds) / speed`, so the rip starts there
/// and runs `speed` times slower than the played file did.
fn on_rip(a: &PlannedPlay, rip: String, alignment: Alignment) -> PlannedPlay {
    PlannedPlay {
        asset: rip,
        source_start_seconds: alignment.rip_seconds
            + (a.source_start_seconds - alignment.played_seconds) / alignment.speed,
        speed: a.speed / alignment.speed,
        ..a.clone()
    }
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
fn detected_reference(a: &PlannedPlay, score: &PlayScore) -> Result<PlannedPlay, String> {
    let best = score
        .best
        .as_ref()
        .ok_or_else(|| format!("{} is confident without a detection", a.asset))?;
    Ok(PlannedPlay {
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
    })
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
    searched_on(
        index,
        samples,
        profile,
        context.searcher.ladder,
        offset,
        context,
    )
}

fn searched_on(
    index: &Index,
    samples: &[f32],
    profile: &Profile,
    ladder: &[Rung],
    offset: f64,
    context: &Context,
) -> Vec<MixFound> {
    let matching = context.searcher.matching;
    search_signal_with(
        index,
        Signal::Mono(samples),
        profile,
        ladder,
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

/// The hits of the strongest detection in the plain excerpt (from
/// `offset` seconds of the mix) that agrees with `masked`: the excerpt
/// over `masked`'s span and `EVIDENCE_MARGIN_SECONDS` either way, searched
/// against `masked`'s file alone on the rungs near its speed.
fn plain_evidence(masked: &MixFound, excerpt: &[f32], offset: f64, context: &Context) -> u32 {
    let Some(record) = record_of(&masked.asset, context) else {
        return 0;
    };
    let Ok(index) = Index::build(std::slice::from_ref(&record)) else {
        return 0;
    };
    let rungs: Vec<Rung> = context
        .searcher
        .ladder
        .iter()
        .copied()
        .filter(|rung| (rung.speed().0 / masked.speed - 1.0).abs() <= EVIDENCE_SPEED)
        .collect();
    let rate = f64::from(context.searcher.matching.profile().sample_rate);
    let at = |seconds: f64| (((seconds - offset) * rate).max(0.0) as usize).min(excerpt.len());
    let from = at(masked.start_seconds - EVIDENCE_MARGIN_SECONDS);
    let to = at(masked.end_seconds + EVIDENCE_MARGIN_SECONDS);
    if rungs.is_empty() || to <= from {
        return 0;
    }
    let profile = context.searcher.matching.profile();
    let start = offset + from as f64 / rate;
    searched_on(&index, &excerpt[from..to], &profile, &rungs, start, context)
        .iter()
        .filter(|plain| agrees(plain, masked))
        .map(|plain| plain.hits)
        .max()
        .unwrap_or(0)
}

/// Whether two detections of one file overlap in mix time at the same
/// alignment: speeds within `EVIDENCE_SPEED`, file times at the middle of
/// their overlap within `EVIDENCE_SECONDS`.
fn agrees(plain: &MixFound, masked: &MixFound) -> bool {
    let start = plain.start_seconds.max(masked.start_seconds);
    let end = plain.end_seconds.min(masked.end_seconds);
    if end <= start {
        return false;
    }
    let middle = 0.5 * (start + end);
    let file_time =
        |found: &MixFound| found.track_start_seconds + found.speed * (middle - found.start_seconds);
    (plain.speed / masked.speed - 1.0).abs() <= EVIDENCE_SPEED
        && (file_time(plain) - file_time(masked)).abs() <= EVIDENCE_SECONDS
}

/// How far a render of `asset` by seek at `start` seconds (as the mix and
/// the references are rendered) is from the samples decoded from the
/// file's beginning, the stored peaks' timeline (0127): positive when the
/// seek lands later in the audio than `start` on that timeline. `None`
/// when the file cannot be rendered or the two do not correlate.
fn seek_offset_ms(asset: &str, start: f64, roots: &Roots) -> Option<f64> {
    let source = roots.path(asset).ok()?;
    let seek = render_samples(&source, start, SEEK_SECONDS, Playback::Turntable(1.0), None).ok()?;
    let earlier = (start - SEEK_REACH_SECONDS).max(0.0);
    let decoded = render_samples_as_indexed(
        &source,
        earlier,
        SEEK_SECONDS + 2.0 * SEEK_REACH_SECONDS,
        Playback::Turntable(1.0),
        None,
    )
    .ok()?;
    let (seek, decoded) = (decimated(&seek), decimated(&decoded));
    let (shift, correlation) = best_shift(&seek, &decoded)?;
    let rate = f64::from(RENDER_RATE) / SEEK_DECIMATION as f64;
    (correlation >= SEEK_MIN_CORRELATION).then(|| (earlier + shift as f64 / rate - start) * 1000.0)
}

/// Means of `SEEK_DECIMATION` samples: a crude low-pass, enough to line up
/// two renders of one file.
fn decimated(samples: &[f32]) -> Vec<f64> {
    samples
        .as_chunks::<SEEK_DECIMATION>()
        .0
        .iter()
        .map(|chunk| {
            chunk.iter().map(|&sample| f64::from(sample)).sum::<f64>() / SEEK_DECIMATION as f64
        })
        .collect()
}

/// Where `part` best matches inside `whole`: the shift with the highest
/// normalised correlation, and that correlation.
fn best_shift(part: &[f64], whole: &[f64]) -> Option<(usize, f64)> {
    if part.is_empty() || whole.len() < part.len() {
        return None;
    }
    let energy = |values: &[f64]| values.iter().map(|value| value * value).sum::<f64>();
    let part_energy = energy(part);
    (0..=whole.len() - part.len())
        .map(|shift| {
            let window = &whole[shift..shift + part.len()];
            let product: f64 = part.iter().zip(window).map(|(a, b)| a * b).sum();
            let scale = (part_energy * energy(window)).sqrt();
            (shift, if scale > 0.0 { product / scale } else { 0.0 })
        })
        .max_by(|a, b| a.1.total_cmp(&b.1))
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
        evidence: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(scenario: Scenario, side: Option<Side>) -> PlannedPlay {
        PlannedPlay {
            asset: String::from("a.mp3"),
            held_out: false,
            source_start_seconds: 100.0,
            speed: 1.02,
            start_seconds: 600.0,
            end_seconds: 750.0,
            fade_in_seconds: 8.0,
            fade_out_seconds: 6.0,
            bass_cut_until: Some(610.0),
            bass_cut_from: None,
            ride: None,
            gain_db: 0.0,
            scenario: Some(scenario),
            side,
            drop_seconds: None,
        }
    }

    #[test]
    fn a_play_moved_to_a_rip_reaches_the_same_audio_at_every_mix_time() {
        let a = play(Scenario::Quiet, Some(Side::First));
        // The rip starts 2.5 s later in its file and runs 0.1% slower.
        let alignment = Alignment {
            rip_seconds: 42.5,
            played_seconds: 40.0,
            speed: 1.001,
        };
        let placed = on_rip(&a, String::from("rip.mp3"), alignment);
        for t in [600.0, 650.0, 749.0] {
            let played = a.source_start_seconds + a.speed * (t - a.start_seconds);
            let rip = alignment.rip_seconds + (played - alignment.played_seconds) / alignment.speed;
            let rendered = placed.source_start_seconds + placed.speed * (t - placed.start_seconds);
            assert!(
                (rendered - rip).abs() < 1e-9,
                "at {t} s: {rendered} against {rip}"
            );
        }
        assert_eq!(placed.asset, "rip.mp3");
        assert_eq!(placed.fade_in_seconds, a.fade_in_seconds);
        assert_eq!(placed.bass_cut_until, a.bass_cut_until);
    }

    #[test]
    fn a_detected_play_follows_its_detections_line_without_fades() {
        let a = play(Scenario::Quiet, Some(Side::First));
        let best = MixFound {
            asset: String::from("other.mp3"),
            start_seconds: 620.0,
            end_seconds: 740.0,
            track_start_seconds: 130.0,
            speed: 1.03,
            windows: 20,
            hits: 900,
            fitted: true,
            verified: None,
        };
        let score = PlayScore {
            truth: a.clone(),
            level: Level::Confident,
            best: Some(best),
            start_late_seconds: None,
            end_early_seconds: None,
            position_error_seconds: None,
        };
        let placed = detected_reference(&a, &score).unwrap();
        assert_eq!(placed.asset, "other.mp3");
        // 20 s before the detection's start, at its speed.
        assert!((placed.source_start_seconds - (130.0 - 1.03 * 20.0)).abs() < 1e-9);
        assert_eq!(placed.speed, 1.03);
        assert_eq!(placed.fade_in_seconds, 0.0);
        assert_eq!(placed.bass_cut_until, None);
    }

    fn found(start: f64, end: f64, track_start: f64, speed: f64) -> MixFound {
        MixFound {
            asset: String::from("b.mp3"),
            start_seconds: start,
            end_seconds: end,
            track_start_seconds: track_start,
            speed,
            windows: 5,
            hits: 120,
            fitted: false,
            verified: None,
        }
    }

    #[test]
    fn plain_evidence_agrees_only_at_the_same_alignment() {
        let masked = found(600.0, 660.0, 100.0, 1.03);
        // The same line, found over a shorter span.
        assert!(agrees(
            &found(620.0, 640.0, 100.0 + 1.03 * 20.0, 1.03),
            &masked
        ));
        // Half a second off in the file: still the same alignment.
        assert!(agrees(&found(620.0, 640.0, 121.1, 1.031), &masked));
        // Two seconds off: another place in the record.
        assert!(!agrees(&found(620.0, 640.0, 122.6, 1.03), &masked));
        // Another speed.
        assert!(!agrees(&found(620.0, 640.0, 120.6, 1.04), &masked));
        // No overlap in the mix.
        assert!(!agrees(&found(670.0, 700.0, 172.1, 1.03), &masked));
    }

    #[test]
    fn the_best_shift_finds_where_a_part_sits() {
        let mut state: u64 = 7;
        let whole: Vec<f64> = (0..400)
            .map(|_| {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                (state >> 40) as f64 - 8_388_608.0
            })
            .collect();
        let part = whole[123..223].to_vec();
        let (shift, correlation) = best_shift(&part, &whole).unwrap();
        assert_eq!(shift, 123);
        assert!((correlation - 1.0).abs() < 1e-9);
        assert_eq!(best_shift(&whole, &part), None);
    }

    #[test]
    fn the_report_is_named_as_in_session_65_with_no_fix() {
        let pilot = |timings: Vec<Timing>, fixes: Vec<Fix>| Pilot {
            rule: Rule { over: 2.0 },
            timings,
            fixes,
        };
        assert_eq!(
            pilot(vec![Timing::Truth], vec![Fix::Off]).name(),
            "residual-truth-over-2"
        );
        assert_eq!(
            pilot(
                vec![Timing::Truth, Timing::Detected],
                vec![Fix::Off, Fix::Align, Fix::AlignFine]
            )
            .name(),
            "residual-truth-detected-over-2-fix-off-align-align-fine"
        );
    }

    #[test]
    fn a_report_of_session_65_still_loads() {
        let old = r#"{"rule": {"over": 2.0}, "timing": "truth", "cases": [{"mix": "mix-00",
            "play": 3, "reference": "a.mp3", "reference_kind": "played",
            "start_seconds": 600.0, "end_seconds": 750.0, "arms": []}]}"#;
        let report: Report = serde_json::from_str(old).unwrap();
        assert_eq!(report.cases.len(), 1);
        assert!(report.timings.is_empty() && report.cases[0].maskings.is_empty());
    }

    #[test]
    fn only_the_second_record_of_a_quiet_or_brief_block_is_a_partner() {
        assert!(is_partner(&play(Scenario::Quiet, Some(Side::Second))));
        assert!(is_partner(&play(Scenario::Brief, Some(Side::Second))));
        assert!(!is_partner(&play(Scenario::Quiet, Some(Side::First))));
        assert!(!is_partner(&play(
            Scenario::UnrelatedDrop,
            Some(Side::Second)
        )));
        assert!(!is_partner(&play(Scenario::Returning, None)));
    }
}

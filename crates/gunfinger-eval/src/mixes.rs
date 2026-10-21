//! Generated mixes: seeded mixes of library tracks with exact truth, for
//! what the real mixes cannot measure: brief plays, detection boundaries,
//! cuts, bass swaps, a returning track. Rendered with the robustness
//! excerpts' tools, they complement real mixes rather than replace them.
//!
//! The index and the held-out clusters are the sweep's for the same seed;
//! held-out tracks play in the mixes too and must produce nothing
//! confident.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::{Confidence, Pass};
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::render::{Encoding, Playback, RENDER_RATE, encode, limited, render_samples, rms};
use crate::rng::Rng;
use crate::scoring::evidence;
use crate::shared::Roots;
use crate::sweep::Plan;
use crate::tempo::{beat_period, onset_envelope};
use crate::verifier::{Verification, Verifier};

/// Plays per generated mix, and the length of each, fades included.
const PLAYS_PER_MIX: usize = 12;
const PLAY_SECONDS: (f64, f64) = (20.0, 60.0);
/// Turntable speeds are drawn within this much of native speed.
const SPEED_RANGE: f64 = 0.06;
/// Share of plays drawn from held-out clusters.
const HELD_OUT_SHARE: f64 = 0.25;
/// The play that returns: it repeats the track of the play this many
/// places before it, from anywhere in the track.
const RETURNING_PLAY: usize = 7;
const RETURN_DISTANCE: usize = 3;
/// A DJ mixer's low cut during a bass swap: two two-pole high-passes at
/// 200 Hz, about 24 dB per octave.
const BASS_CUT: &str = "highpass=f=200,highpass=f=200";
/// Each play is brought to this RMS level before mixing, as a DJ matches
/// the gain of the next record.
const LEVEL: f64 = 0.1;
/// Cuts and the switch to and from a bass cut ramp over this long, so they
/// do not click.
const RAMP_SECONDS: f64 = 0.02;
/// Long mixes: plays of a DJ's length, joined by crossfades of this long
/// (half of them bass swaps), with a run of each of these many held-out
/// tracks in a drawn order, after 2 indexed plays and each followed by 2
/// or 3, so that a mix has at least 5 start-to-start intervals between
/// consecutive indexed plays for method 1's median.
const LONG_PLAY_SECONDS: (f64, f64) = (120.0, 360.0);
const LONG_CROSSFADE_SECONDS: (f64, f64) = (10.0, 60.0);
const HELD_OUT_RUNS: [usize; 4] = [1, 2, 3, 4];
const INDEXED_FIRST: usize = 2;
const INDEXED_AFTER_RUN: (usize, usize) = (2, 3);

/// What a generated mix plays.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum MixKind {
    /// 12 plays of 20-60 s, a returning track, a quarter held out at
    /// random, joined by bass swaps, crossfades and cuts.
    #[default]
    Short,
    /// Plays of 2-6 minutes joined by crossfades of 10-60 s, with runs of
    /// 1, 2, 3 and 4 held-out tracks between indexed ones.
    Long,
    /// Long mixes whose every play is beatmatched to the one before it.
    /// The files of the shared-material panel's families are out of the
    /// index and the draws, so that this is the control of `shared`.
    Beatmatched,
    /// Beatmatched long mixes whose held-out runs of 2 or more hold pairs
    /// of the shared-material panel's records, each played over the
    /// passage the two share, one after the other.
    Shared,
    /// Long mixes whose every play rides the pitch by 1-3% once.
    Rides,
}

impl MixKind {
    // serde's `skip_serializing_if` passes a reference.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_short(&self) -> bool {
        *self == MixKind::Short
    }

    /// Where the mixes of `seed` are rendered under the work directory.
    pub fn renders(self, work: &Path, seed: u64) -> PathBuf {
        work.join("mixes").join(self.name(seed))
    }

    /// The report's and the renders' name: `seed-<seed>` for short mixes,
    /// `<kind>-seed-<seed>` for the others.
    pub fn name(self, seed: u64) -> String {
        match self {
            MixKind::Short => format!("seed-{seed}"),
            MixKind::Long => format!("long-seed-{seed}"),
            MixKind::Beatmatched => format!("beatmatched-seed-{seed}"),
            MixKind::Shared => format!("shared-seed-{seed}"),
            MixKind::Rides => format!("rides-seed-{seed}"),
        }
    }

    fn beatmatches(self) -> bool {
        matches!(self, MixKind::Beatmatched | MixKind::Shared)
    }

    /// The lengths of play `print_summary` counts confident plays by.
    fn play_lengths(self) -> [(f64, f64); 4] {
        match self {
            MixKind::Short => [(20.0, 30.0), (30.0, 40.0), (40.0, 50.0), (50.0, 61.0)],
            MixKind::Long | MixKind::Beatmatched | MixKind::Shared | MixKind::Rides => {
                [(0.0, 120.0), (120.0, 200.0), (200.0, 280.0), (280.0, 361.0)]
            }
        }
    }
}

/// A track's appearance in a planned mix: which audio plays where, at what
/// speed, and how it enters and leaves.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlannedPlay {
    pub asset: String,
    pub held_out: bool,
    /// Where the play starts in the track.
    pub source_start_seconds: f64,
    pub speed: f64,
    /// Mix time of the first and last audible sample, fades included.
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub fade_in_seconds: f64,
    pub fade_out_seconds: f64,
    /// Bass swap: the low end is cut before this mix time (coming in) and
    /// after `bass_cut_from` (going out).
    pub bass_cut_until: Option<f64>,
    pub bass_cut_from: Option<f64>,
    /// A change of speed within the play.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ride: Option<Ride>,
}

/// A pitch ride: from mix time `start_seconds` the speed moves linearly
/// from the play's `speed` to `to_speed` over `seconds`, and stays there.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Ride {
    pub start_seconds: f64,
    pub seconds: f64,
    pub to_speed: f64,
}

impl PlannedPlay {
    /// The position in the track heard at mix time `seconds`: the speed
    /// integrated from the play's start.
    pub fn source_at(&self, seconds: f64) -> f64 {
        let elapsed = seconds - self.start_seconds;
        let Some(ride) = self.ride else {
            return self.source_start_seconds + elapsed * self.speed;
        };
        let before = (ride.start_seconds - self.start_seconds).max(0.0);
        let rising = (elapsed - before).clamp(0.0, ride.seconds);
        let after = (elapsed - before - ride.seconds).max(0.0);
        self.source_start_seconds
            + self.speed * elapsed.min(before)
            + self.speed * rising
            + (ride.to_speed - self.speed) * rising * rising / (2.0 * ride.seconds)
            + ride.to_speed * after
    }

    fn is_bass_cut(&self, seconds: f64) -> bool {
        self.bass_cut_until.is_some_and(|until| seconds < until)
            || self.bass_cut_from.is_some_and(|from| seconds >= from)
    }

    /// Gain at mix time `seconds`: linear fades in and out.
    fn fade(&self, seconds: f64) -> f64 {
        let fade_in = self.fade_in_seconds.max(RAMP_SECONDS);
        let fade_out = self.fade_out_seconds.max(RAMP_SECONDS);
        let rising = (seconds - self.start_seconds) / fade_in;
        let falling = (self.end_seconds - seconds) / fade_out;
        rising.min(falling).clamp(0.0, 1.0)
    }
}

/// A mix to render, with its truth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedMix {
    pub name: String,
    pub seconds: f64,
    pub plays: Vec<PlannedPlay>,
}

/// Renders `mix` as a 128 kbit/s MP3: each play at its speed and level,
/// faded and bass-cut as planned, summed.
pub fn render(mix: &PlannedMix, roots: &Roots, output: &Path) -> Result<(), String> {
    let mut samples = vec![0.0_f32; to_index(mix.seconds)];
    for play in &mix.plays {
        let first = to_index(play.start_seconds);
        for (offset, sample) in play_audio(play, roots)?.into_iter().enumerate() {
            if let Some(slot) = samples.get_mut(first + offset) {
                *slot += sample;
            }
        }
    }
    if let Some(dir) = output.parent() {
        fs::create_dir_all(dir).map_err(|error| error.to_string())?;
    }
    encode(&limited(samples), None, Encoding::Mp3(128), output)
}

/// One play's audio as it sounds in the mix. Where the bass is cut, the
/// high-passed render takes over through a short ramp.
fn play_audio(play: &PlannedPlay, roots: &Roots) -> Result<Vec<f32>, String> {
    let source = roots.path(&play.asset)?;
    let seconds = play.end_seconds - play.start_seconds;
    let render = |filter| match play.ride {
        None => render_samples(
            &source,
            play.source_start_seconds,
            seconds,
            Playback::Turntable(play.speed),
            filter,
        ),
        Some(_) => ridden(play, &source, filter),
    };
    let full = render(None)?;
    let gain = LEVEL / rms(&full).max(1e-9);
    let swapped = play.bass_cut_until.is_some() || play.bass_cut_from.is_some();
    let cut = if swapped {
        render(Some(BASS_CUT))?
    } else {
        Vec::new()
    };
    let rate = f64::from(RENDER_RATE);
    Ok((0..full.len())
        .map(|index| {
            let at = play.start_seconds + index as f64 / rate;
            let cut_share = if swapped { cut_share(play, at) } else { 0.0 };
            let sample = (1.0 - cut_share) * f64::from(full[index])
                + cut_share * f64::from(cut.get(index).copied().unwrap_or(0.0));
            (sample * gain * play.fade(at)) as f32
        })
        .collect())
}

/// A play with a pitch ride: the track at native speed, read at the
/// position `source_at` gives for each output sample, between samples by
/// linear interpolation.
fn ridden(play: &PlannedPlay, source: &Path, filter: Option<&str>) -> Result<Vec<f32>, String> {
    let rate = f64::from(RENDER_RATE);
    let needed = play.source_at(play.end_seconds) - play.source_start_seconds + 1.0;
    let native = render_samples(
        source,
        play.source_start_seconds,
        needed,
        Playback::Turntable(1.0),
        filter,
    )?;
    let length = to_index(play.end_seconds - play.start_seconds);
    Ok((0..length)
        .map(|index| {
            let at = play.start_seconds + index as f64 / rate;
            let position = (play.source_at(at) - play.source_start_seconds) * rate;
            let before = position.floor() as usize;
            let fraction = (position - position.floor()) as f32;
            match (native.get(before), native.get(before + 1)) {
                (Some(&a), Some(&b)) => a + (b - a) * fraction,
                _ => 0.0,
            }
        })
        .collect())
}

/// How much of the bass-cut render plays at `seconds`: 1 inside a cut, 0
/// outside, ramping across each boundary.
fn cut_share(play: &PlannedPlay, seconds: f64) -> f64 {
    let half = RAMP_SECONDS / 2.0;
    let before = play.is_bass_cut(seconds - half);
    let after = play.is_bass_cut(seconds + half);
    match (before, after) {
        (true, true) => 1.0,
        (false, false) => 0.0,
        _ => {
            let boundary = [play.bass_cut_until, play.bass_cut_from]
                .into_iter()
                .flatten()
                .find(|&boundary| (boundary - seconds).abs() <= half)
                .unwrap_or(seconds);
            let into_cut = if after { 1.0 } else { -1.0 };
            (0.5 + into_cut * (seconds - boundary) / RAMP_SECONDS).clamp(0.0, 1.0)
        }
    }
}

fn to_index(seconds: f64) -> usize {
    (seconds * f64::from(RENDER_RATE)).round() as usize
}

/// A detection as the truth scorer sees it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixFound {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub track_start_seconds: f64,
    pub speed: f64,
    pub windows: u32,
    pub hits: u32,
    /// Counted by the second pass (`Pass::Fitted`).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub fitted: bool,
    /// The peak verifier's measures, with `--verify`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verified: Option<Verification>,
}

impl MixFound {
    pub fn new(index: &Index, detection: &Detection, verified: Option<Verification>) -> MixFound {
        MixFound {
            asset: index.asset(detection.asset).path.clone(),
            start_seconds: detection.start_seconds,
            end_seconds: detection.end_seconds,
            track_start_seconds: detection.track_start_seconds,
            speed: detection.speed.0,
            windows: detection.evidence.windows,
            hits: detection.evidence.hits,
            fitted: detection.evidence.pass == Pass::Fitted,
            verified,
        }
    }

    pub fn confidence(&self) -> Confidence {
        evidence(self.windows, self.hits, self.fitted).confidence()
    }

    fn overlaps(&self, play: &PlannedPlay) -> bool {
        self.start_seconds < play.end_seconds && play.start_seconds < self.end_seconds
    }
}

/// What the search made of one planned play.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayScore {
    pub truth: PlannedPlay,
    /// `confident`, `possible`, `weak` or `none`: the strongest matching
    /// detection's level.
    pub level: String,
    pub best: Option<MixFound>,
    /// Over the matching detections that are at least possible: how much
    /// later than the first audible sample they start, and how much earlier
    /// than the last they end (negative: outside the play).
    pub start_late_seconds: Option<f64>,
    pub end_early_seconds: Option<f64>,
    /// The best detection's position in the track at its start, minus the
    /// true position there.
    pub position_error_seconds: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MixScore {
    pub plays: Vec<PlayScore>,
    /// Confident detections that match no play of their cluster.
    pub wrong_confident: Vec<MixFound>,
    /// Possible detections that match no play of their cluster.
    pub wrong_possible: Vec<MixFound>,
    /// The strongest detections that match no play, at any level.
    pub false_candidates: Vec<MixFound>,
}

/// False candidates kept per mix.
const FALSE_CANDIDATES_KEPT: usize = 5;

/// Scores detections (two windows or more) against the truth. A detection
/// matches a play when its asset shares the play's cluster and it overlaps
/// the play's audible span.
pub fn score(mix: &PlannedMix, found: &[MixFound], clusters: &Clusters) -> MixScore {
    let accepted: Vec<BTreeSet<String>> = mix
        .plays
        .iter()
        .map(|play| clusters.cluster_of(&play.asset))
        .collect();
    let matches = |detection: &MixFound, play: usize| {
        accepted[play].contains(&detection.asset) && detection.overlaps(&mix.plays[play])
    };
    let plays = (0..mix.plays.len())
        .map(|play| {
            let matching: Vec<&MixFound> = found
                .iter()
                .filter(|detection| matches(detection, play))
                .collect();
            score_play(&mix.plays[play], &matching)
        })
        .collect();
    let mut unmatched: Vec<&MixFound> = found
        .iter()
        .filter(|detection| !(0..mix.plays.len()).any(|play| matches(detection, play)))
        .collect();
    unmatched.sort_by_key(|detection| std::cmp::Reverse(detection.hits));
    let at_level = |level: Confidence| -> Vec<MixFound> {
        unmatched
            .iter()
            .filter(|detection| detection.confidence() == level)
            .map(|detection| (*detection).clone())
            .collect()
    };
    MixScore {
        plays,
        wrong_confident: at_level(Confidence::Confident),
        wrong_possible: at_level(Confidence::Possible),
        false_candidates: unmatched
            .iter()
            .take(FALSE_CANDIDATES_KEPT)
            .map(|detection| (*detection).clone())
            .collect(),
    }
}

fn score_play(play: &PlannedPlay, matching: &[&MixFound]) -> PlayScore {
    let best = matching
        .iter()
        .max_by_key(|detection| detection.hits)
        .map(|detection| (*detection).clone());
    let level = match best.as_ref().map(MixFound::confidence) {
        Some(Confidence::Confident) => "confident",
        Some(Confidence::Possible) => "possible",
        Some(Confidence::Weak) => "weak",
        None => "none",
    };
    let shown: Vec<&&MixFound> = matching
        .iter()
        .filter(|detection| detection.confidence() >= Confidence::Possible)
        .collect();
    let start = shown
        .iter()
        .map(|detection| detection.start_seconds)
        .reduce(f64::min);
    let end = shown
        .iter()
        .map(|detection| detection.end_seconds)
        .reduce(f64::max);
    PlayScore {
        truth: play.clone(),
        level: level.to_owned(),
        position_error_seconds: best.as_ref().map(|detection| {
            detection.track_start_seconds - play.source_at(detection.start_seconds)
        }),
        best,
        start_late_seconds: start.map(|start| start - play.start_seconds),
        end_early_seconds: end.map(|end| play.end_seconds - end),
    }
}

/// The tracks a generated mix draws from: indexed ones and held-out ones,
/// long enough for any play.
pub struct Pools<'a> {
    pub indexed: Vec<&'a PeakRecord>,
    pub held_out: Vec<&'a PeakRecord>,
}

impl<'a> Pools<'a> {
    pub fn new(records: &'a [PeakRecord], held_out: &BTreeSet<String>) -> Pools<'a> {
        let long_enough = |record: &&PeakRecord| record.header.duration_seconds >= 150.0;
        let (held_out, indexed) = records
            .iter()
            .filter(long_enough)
            .partition(|record| held_out.contains(&record.header.source.path));
        Pools { indexed, held_out }
    }
}

/// A record from `pool` whose cluster is not in `used` yet, which it then
/// joins; `None` when every cluster of the pool is used.
pub fn draw_unused<'a>(
    pool: &[&'a PeakRecord],
    used: &mut BTreeSet<BTreeSet<String>>,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Option<&'a PeakRecord> {
    let fresh: Vec<&'a PeakRecord> = pool
        .iter()
        .copied()
        .filter(|record| !used.contains(&clusters.cluster_of(&record.header.source.path)))
        .collect();
    if fresh.is_empty() {
        return None;
    }
    let chosen = fresh[rng.below(fresh.len())];
    used.insert(clusters.cluster_of(&chosen.header.source.path));
    Some(chosen)
}

/// Draws a mix of `PLAYS_PER_MIX` plays from distinct clusters, one of them
/// returning, a quarter held out, joined by bass swaps, crossfades and cuts.
pub fn draw_mix(
    name: &str,
    pools: &Pools,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<PlannedMix, String> {
    let too_few = || String::from("the library has too few long tracks for a generated mix");
    let mut used: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    let mut chosen: Vec<(&PeakRecord, bool)> = Vec::new();
    for position in 0..PLAYS_PER_MIX {
        if position == RETURNING_PLAY {
            chosen.push(chosen[position - RETURN_DISTANCE]);
            continue;
        }
        let held_out = rng.unit() < HELD_OUT_SHARE;
        let pool = if held_out {
            &pools.held_out
        } else {
            &pools.indexed
        };
        let record = draw_unused(pool, &mut used, clusters, rng).ok_or_else(too_few)?;
        chosen.push((record, held_out));
    }
    // The returning play's first appearance must be indexed: a held-out
    // track returning tests nothing new.
    if chosen[RETURNING_PLAY].1 {
        let record = draw_unused(&pools.indexed, &mut used, clusters, rng).ok_or_else(too_few)?;
        chosen[RETURNING_PLAY - RETURN_DISTANCE] = (record, false);
        chosen[RETURNING_PLAY] = (record, false);
    }

    let mut plays: Vec<PlannedPlay> = Vec::new();
    let mut start = 0.0;
    for &(record, held_out) in &chosen {
        let mut play = drawn_play(record, held_out, start, rng);
        if let Some(previous) = plays.last_mut() {
            join(previous, &mut play, rng);
        }
        start = play.end_seconds;
        plays.push(play);
    }
    Ok(PlannedMix {
        name: name.to_owned(),
        seconds: start,
        plays,
    })
}

/// A play of `record` from mix time `start`: its length, speed and
/// position in the track drawn in that order.
fn drawn_play(record: &PeakRecord, held_out: bool, start: f64, rng: &mut Rng) -> PlannedPlay {
    let seconds = PLAY_SECONDS.0 + rng.unit() * (PLAY_SECONDS.1 - PLAY_SECONDS.0);
    let speed = drawn_speed(rng);
    // Plays stay clear of the first tenth of a track, often a sparse
    // intro; tracks in the pools are long enough for any play.
    let duration = record.header.duration_seconds;
    let earliest = 0.1 * duration;
    let latest = 0.95 * duration - seconds * speed;
    PlannedPlay {
        asset: record.header.source.path.clone(),
        held_out,
        source_start_seconds: earliest + rng.unit() * (latest - earliest),
        speed,
        start_seconds: start,
        end_seconds: start + seconds,
        fade_in_seconds: 0.0,
        fade_out_seconds: 0.0,
        bass_cut_until: None,
        bass_cut_from: None,
        ride: None,
    }
}

fn drawn_speed(rng: &mut Rng) -> f64 {
    1.0 + (2.0 * rng.unit() - 1.0) * SPEED_RANGE
}

/// What the beatmatched, shared and ridden long mixes draw on besides the
/// pools: each file's beat period at native speed, and the pairs of
/// records that share a passage.
#[derive(Default)]
pub struct LongExtras {
    pub periods: BTreeMap<String, f64>,
    pub pairs: Vec<SharedPair>,
}

/// Two records of one family of the shared-material panel, in the order
/// they play, each with its length and the passage the two share.
#[derive(Debug, Clone)]
pub struct SharedPair {
    pub family: String,
    pub records: [PairRecord; 2],
}

#[derive(Debug, Clone)]
pub struct PairRecord {
    pub asset: String,
    pub duration_seconds: f64,
    pub passage: (f64, f64),
}

/// A track picked for a long mix, before its play is drawn.
struct Pick<'a> {
    asset: &'a str,
    duration_seconds: f64,
    held_out: bool,
    /// Where the play must be heard, in the track: a shared passage.
    passage: Option<(f64, f64)>,
}

impl<'a> Pick<'a> {
    fn of(record: &'a PeakRecord, held_out: bool) -> Pick<'a> {
        Pick {
            asset: &record.header.source.path,
            duration_seconds: record.header.duration_seconds,
            held_out,
            passage: None,
        }
    }
}

/// Draws a long mix: `INDEXED_FIRST` indexed plays, then a run of each
/// length of `HELD_OUT_RUNS` in a drawn order, each followed by 2 or 3
/// indexed plays, every track from a cluster of its own, joined by
/// crossfades. `kind` adds beatmatching, shared pairs in the runs or pitch
/// rides; `MixKind::Long` draws exactly what it drew before there were
/// other long kinds.
pub fn draw_long_mix(
    name: &str,
    kind: MixKind,
    pools: &Pools,
    extras: &LongExtras,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<PlannedMix, String> {
    let too_few = || String::from("the library has too few long tracks for a long generated mix");
    let mut used: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    let mut families: BTreeSet<&str> = BTreeSet::new();
    let draw = |held_out: bool, used: &mut BTreeSet<BTreeSet<String>>, rng: &mut Rng| {
        let pool = if held_out {
            &pools.held_out
        } else {
            &pools.indexed
        };
        draw_unused(pool, used, clusters, rng)
            .map(|record| Pick::of(record, held_out))
            .ok_or_else(too_few)
    };
    let mut runs = HELD_OUT_RUNS;
    rng.shuffle(&mut runs);
    let mut chosen: Vec<Pick> = Vec::new();
    for _ in 0..INDEXED_FIRST {
        chosen.push(draw(false, &mut used, rng)?);
    }
    for run in runs {
        if kind == MixKind::Shared {
            let pairs = run / 2;
            // A single track of an odd run plays before or after its pair.
            let single_first = run % 2 == 1 && rng.unit() < 0.5;
            if single_first {
                chosen.push(draw(true, &mut used, rng)?);
            }
            for _ in 0..pairs {
                let pair = draw_pair(&extras.pairs, &mut families, rng).ok_or_else(too_few)?;
                for record in &pair.records {
                    used.insert(clusters.cluster_of(&record.asset));
                    chosen.push(Pick {
                        asset: &record.asset,
                        duration_seconds: record.duration_seconds,
                        held_out: true,
                        passage: Some(record.passage),
                    });
                }
            }
            if run % 2 == 1 && !single_first {
                chosen.push(draw(true, &mut used, rng)?);
            }
        } else {
            for _ in 0..run {
                chosen.push(draw(true, &mut used, rng)?);
            }
        }
        let (fewest, most) = INDEXED_AFTER_RUN;
        for _ in 0..fewest + rng.below(most - fewest + 1) {
            chosen.push(draw(false, &mut used, rng)?);
        }
    }

    let mut plays: Vec<PlannedPlay> = Vec::new();
    let mut start = 0.0;
    for pick in &chosen {
        let drawn = drawn_speed(rng);
        let speed = match plays.last() {
            Some(previous) if kind.beatmatches() => {
                beatmatched_speed(previous, pick.asset, &extras.periods).unwrap_or(drawn)
            }
            _ => drawn,
        };
        let mut play = long_play(pick, start, speed, rng);
        if kind == MixKind::Rides {
            ride(&mut play, pick.duration_seconds, rng);
        }
        if let Some(previous) = plays.last_mut() {
            crossfade(previous, &mut play, rng);
        }
        start = play.end_seconds;
        plays.push(play);
    }
    Ok(PlannedMix {
        name: name.to_owned(),
        seconds: start,
        plays,
    })
}

/// A pair of a family not yet in the mix, which it then joins.
fn draw_pair<'a>(
    pairs: &'a [SharedPair],
    families: &mut BTreeSet<&'a str>,
    rng: &mut Rng,
) -> Option<&'a SharedPair> {
    let fresh: Vec<&SharedPair> = pairs
        .iter()
        .filter(|pair| !families.contains(pair.family.as_str()))
        .collect();
    if fresh.is_empty() {
        return None;
    }
    let chosen = fresh[rng.below(fresh.len())];
    families.insert(&chosen.family);
    Some(chosen)
}

/// A beatmatched play is at most this far from its native speed.
const BEATMATCH_RANGE: f64 = 0.08;
/// Ratios between a found beat period and the true one (as `robust`'s).
const METRICAL_FACTORS: [f64; 7] = [1.0, 2.0, 0.5, 1.5, 2.0 / 3.0, 4.0 / 3.0, 0.75];

/// The speed at which `asset` plays at the tempo `previous` plays at, as
/// a DJ beatmatches the next record: of the metrical readings of the two
/// beat periods, the one nearest native speed, if within
/// `BEATMATCH_RANGE`. `None` when a period is unknown or none is in range;
/// the play keeps its drawn speed then.
fn beatmatched_speed(
    previous: &PlannedPlay,
    asset: &str,
    periods: &BTreeMap<String, f64>,
) -> Option<f64> {
    let ours = periods.get(asset)?;
    let theirs = periods.get(&previous.asset)?;
    let playing = previous.ride.map_or(previous.speed, |ride| ride.to_speed);
    METRICAL_FACTORS
        .iter()
        .map(|factor| playing * ours / theirs * factor)
        .filter(|speed| (speed - 1.0).abs() <= BEATMATCH_RANGE)
        .min_by(|a, b| (a - 1.0).abs().total_cmp(&(b - 1.0).abs()))
}

/// A play of a DJ's length, shortened to fit the track, at `speed`: from a
/// drawn place between 5% of the track and where the play would end at
/// 95% of it, or, for a pick with a passage, from a drawn place at which
/// the play holds the whole passage where it can.
fn long_play(pick: &Pick, start: f64, speed: f64, rng: &mut Rng) -> PlannedPlay {
    let duration = pick.duration_seconds;
    let drawn = LONG_PLAY_SECONDS.0 + rng.unit() * (LONG_PLAY_SECONDS.1 - LONG_PLAY_SECONDS.0);
    let seconds = drawn.min(0.85 * duration / speed);
    let heard = seconds * speed;
    let (earliest, latest) = match pick.passage {
        None => (0.05 * duration, 0.95 * duration - heard),
        Some((from, to)) => {
            let latest = from.min(duration - heard);
            ((to - heard).max(0.0).min(latest), latest)
        }
    };
    let latest = latest.max(earliest);
    PlannedPlay {
        asset: pick.asset.to_owned(),
        held_out: pick.held_out,
        source_start_seconds: earliest + rng.unit() * (latest - earliest),
        speed,
        start_seconds: start,
        end_seconds: start + seconds,
        fade_in_seconds: 0.0,
        fade_out_seconds: 0.0,
        bass_cut_until: None,
        bass_cut_from: None,
        ride: None,
    }
}

/// Pitch rides: a change of `RIDE_CHANGE` of the speed, up or down, over
/// `RIDE_SECONDS`, starting between 30% and 70% into the play. A change
/// that would take the play more than `RIDE_LIMIT` from native speed goes
/// the other way.
const RIDE_CHANGE: (f64, f64) = (0.01, 0.03);
const RIDE_SECONDS: (f64, f64) = (5.0, 15.0);
const RIDE_LIMIT: f64 = 0.07;

/// Gives `play` a drawn pitch ride, moving its start in the track back if
/// the faster end would run past 97% of a track of `duration_seconds`.
fn ride(play: &mut PlannedPlay, duration_seconds: f64, rng: &mut Rng) {
    let seconds = play.end_seconds - play.start_seconds;
    let at = play.start_seconds + (0.3 + 0.4 * rng.unit()) * seconds;
    let over = RIDE_SECONDS.0 + rng.unit() * (RIDE_SECONDS.1 - RIDE_SECONDS.0);
    let size = RIDE_CHANGE.0 + rng.unit() * (RIDE_CHANGE.1 - RIDE_CHANGE.0);
    let up = rng.unit() < 0.5;
    let mut to_speed = play.speed * if up { 1.0 + size } else { 1.0 - size };
    if (to_speed - 1.0).abs() > RIDE_LIMIT {
        to_speed = play.speed * if up { 1.0 - size } else { 1.0 + size };
    }
    play.ride = Some(Ride {
        start_seconds: at,
        seconds: over.min(play.end_seconds - at),
        to_speed,
    });
    let overrun = play.source_at(play.end_seconds) - 0.97 * duration_seconds;
    if overrun > 0.0 {
        play.source_start_seconds = (play.source_start_seconds - overrun).max(0.0);
    }
}

/// Moves `next` back over the end of `previous` by a crossfade of
/// `LONG_CROSSFADE_SECONDS`, at most half the shorter play; half of them
/// hand the low end over halfway.
fn crossfade(previous: &mut PlannedPlay, next: &mut PlannedPlay, rng: &mut Rng) {
    let longest = (previous.end_seconds - previous.start_seconds)
        .min(next.end_seconds - next.start_seconds)
        / 2.0;
    let overlap = (LONG_CROSSFADE_SECONDS.0
        + rng.unit() * (LONG_CROSSFADE_SECONDS.1 - LONG_CROSSFADE_SECONDS.0))
        .min(longest);
    next.start_seconds -= overlap;
    next.end_seconds -= overlap;
    if let Some(ride) = &mut next.ride {
        ride.start_seconds -= overlap;
    }
    previous.fade_out_seconds = overlap;
    next.fade_in_seconds = overlap;
    if rng.unit() < 0.5 {
        let swap = next.start_seconds + overlap / 2.0;
        next.bass_cut_until = Some(swap);
        previous.bass_cut_from = Some(swap);
    }
}

/// A file to play whole in a mix (`draw_around`), and how long it is.
pub struct Whole<'a> {
    pub asset: &'a str,
    pub duration_seconds: f64,
}

/// Plays stop this far before a whole file's end, so a play never asks for
/// audio the file does not have.
const WHOLE_END_GUARD_SECONDS: f64 = 0.1;

/// Draws a mix around `whole`, played from its start to its end at a drawn
/// speed: `around` drawn plays of `pool` before it and as many after, from
/// distinct clusters other than its own, all indexed, joined as
/// `draw_mix` joins its plays.
pub fn draw_around(
    name: &str,
    whole: &Whole,
    around: usize,
    pool: &[&PeakRecord],
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<PlannedMix, String> {
    let too_few = || String::from("the pool has too few long tracks for a mix around a file");
    let mut used = BTreeSet::from([clusters.cluster_of(whole.asset)]);
    let drawn: Vec<&PeakRecord> = (0..2 * around)
        .map(|_| draw_unused(pool, &mut used, clusters, rng).ok_or_else(too_few))
        .collect::<Result<_, _>>()?;
    let speed = drawn_speed(rng);
    let mut plays: Vec<PlannedPlay> = Vec::new();
    let mut start = 0.0;
    for position in 0..=2 * around {
        let mut play = match position.cmp(&around) {
            std::cmp::Ordering::Equal => PlannedPlay {
                asset: whole.asset.to_owned(),
                held_out: false,
                source_start_seconds: 0.0,
                speed,
                start_seconds: start,
                end_seconds: start
                    + (whole.duration_seconds - WHOLE_END_GUARD_SECONDS).max(0.0) / speed,
                fade_in_seconds: 0.0,
                fade_out_seconds: 0.0,
                bass_cut_until: None,
                bass_cut_from: None,
                ride: None,
            },
            std::cmp::Ordering::Less => drawn_play(drawn[position], false, start, rng),
            std::cmp::Ordering::Greater => drawn_play(drawn[position - 1], false, start, rng),
        };
        if let Some(previous) = plays.last_mut() {
            join(previous, &mut play, rng);
        }
        start = play.end_seconds;
        plays.push(play);
    }
    Ok(PlannedMix {
        name: name.to_owned(),
        seconds: start,
        plays,
    })
}

/// Moves `next` back over the end of `previous` by a drawn transition: a
/// bass swap (a long crossfade whose low end changes hands halfway), a
/// plain crossfade or a cut.
fn join(previous: &mut PlannedPlay, next: &mut PlannedPlay, rng: &mut Rng) {
    let longest = (previous.end_seconds - previous.start_seconds)
        .min(next.end_seconds - next.start_seconds)
        / 2.0;
    let choice = rng.unit();
    let overlap = if choice < 0.4 {
        (8.0 + 8.0 * rng.unit()).min(longest)
    } else if choice < 0.7 {
        (4.0 + 6.0 * rng.unit()).min(longest)
    } else {
        0.0
    };
    next.start_seconds -= overlap;
    next.end_seconds -= overlap;
    previous.fade_out_seconds = overlap;
    next.fade_in_seconds = overlap;
    if choice < 0.4 {
        let swap = next.start_seconds + overlap / 2.0;
        next.bass_cut_until = Some(swap);
        previous.bass_cut_from = Some(swap);
    }
}

#[derive(Serialize, Deserialize)]
pub struct MixesReport {
    pub seed: u64,
    /// Reports of short mixes, from before there were kinds, have none.
    #[serde(default, skip_serializing_if = "MixKind::is_short")]
    pub kind: MixKind,
    pub ladder: String,
    pub indexed_assets: usize,
    pub mixes: Vec<MixResult>,
}

#[derive(Serialize, Deserialize)]
pub struct MixResult {
    pub mix: PlannedMix,
    pub score: MixScore,
    /// Every detection with at least two windows, strongest first.
    pub detections: Vec<MixFound>,
}

pub struct Options<'a> {
    pub seed: u64,
    pub kind: MixKind,
    /// The other library's root, for the shared-material panel's files
    /// under `second-library/` (beatmatched and shared kinds).
    pub other_root: Option<&'a Path>,
    /// Where the seed's sweep panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    pub count: usize,
    pub ladder_name: &'a str,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// Measure every detection with the peak verifier.
    pub verify: bool,
    pub jobs: usize,
}

/// The library's records, the sweep's held-out assets for `seed`, and the
/// index of the rest less `left_out`.
pub fn sweep_index(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    seed: u64,
    panels: &Path,
    matching: &Matching,
    left_out: &BTreeSet<String>,
) -> Result<(Vec<PeakRecord>, BTreeSet<String>, Index), String> {
    let (records, _) = load_records(library, store, &Profile::CURRENT, &BTreeSet::new());
    let held_out = Plan::for_seed(&records, clusters, seed, panels)?.held_out;
    let indexed: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !held_out.contains(&record.header.source.path))
        .filter(|record| !left_out.contains(&record.header.source.path))
        .cloned()
        .collect();
    let index = Index::build(&indexed).map_err(|error| error.to_string())?;
    Ok((records, held_out, matching.index(index)))
}

/// The shared-material panel the shared kind draws its pairs from, in the
/// panels directory.
const SHARED_PANEL: &str = "shared-material-seed-2026.json";
/// Each record's tempo is read from this long a native render...
const TEMPO_SECONDS: f64 = 30.0;
/// ...from this far into the track, or from a shared passage's start.
const TEMPO_FROM: f64 = 0.4;

/// The files of every family of the shared-material panel (left out of
/// the index and the draws of both beatmatched kinds), and for the shared
/// kind each direction with a shared passage as a pair, the played file
/// first. A pair's lengths come from FFprobe.
fn shared_pairs(
    panels: &Path,
    roots: &Roots,
    kind: MixKind,
) -> Result<(BTreeSet<String>, Vec<SharedPair>), String> {
    let path = panels.join(SHARED_PANEL);
    let text = fs::read_to_string(&path)
        .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let panel: crate::identifiability::Panel =
        serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
    let files: BTreeSet<String> = panel
        .families
        .iter()
        .flat_map(|family| family.recordings.iter().flatten().cloned())
        .collect();
    if kind != MixKind::Shared {
        return Ok((files, Vec::new()));
    }
    let length = |asset: &str| -> Result<f64, String> {
        let source = roots.path(asset)?;
        gunfinger_core::decode::probe(&source)
            .and_then(|probe| probe.length)
            .map(|length| length.as_secs_f64())
            .ok_or_else(|| format!("cannot read the length of {}", source.display()))
    };
    let mut pairs = Vec::new();
    for family in &panel.families {
        for direction in &family.directions {
            let Some(passage) = &direction.passage else {
                continue;
            };
            let (Some(related_start), Some(related_end)) =
                (passage.related_start_seconds, passage.related_end_seconds)
            else {
                continue;
            };
            if passage.control {
                continue;
            }
            pairs.push(SharedPair {
                family: family.name.clone(),
                records: [
                    PairRecord {
                        asset: direction.played.clone(),
                        duration_seconds: length(&direction.played)?,
                        passage: (passage.start_seconds, passage.end_seconds),
                    },
                    PairRecord {
                        asset: direction.related.clone(),
                        duration_seconds: length(&direction.related)?,
                        passage: (related_start, related_end),
                    },
                ],
            });
        }
    }
    Ok((files, pairs))
}

/// The beat period at native speed of every record of the pools and every
/// pair's files (from its shared passage), where `tempo` finds one.
fn beat_periods(
    pools: &Pools,
    pairs: &[SharedPair],
    roots: &Roots,
    jobs: usize,
) -> BTreeMap<String, f64> {
    let mut wanted: BTreeMap<String, f64> = pools
        .indexed
        .iter()
        .chain(&pools.held_out)
        .map(|record| {
            (
                record.header.source.path.clone(),
                TEMPO_FROM * record.header.duration_seconds,
            )
        })
        .collect();
    for record in pairs.iter().flat_map(|pair| &pair.records) {
        wanted.insert(record.asset.clone(), record.passage.0);
    }
    let wanted: Vec<(String, f64)> = wanted.into_iter().collect();
    let periods = map_in_order(&wanted, jobs, |(asset, from)| {
        let samples = render_samples(
            &roots.path(asset).ok()?,
            *from,
            TEMPO_SECONDS,
            Playback::Turntable(1.0),
            None,
        )
        .ok()?;
        beat_period(&onset_envelope(&samples, RENDER_RATE))
    });
    wanted
        .into_iter()
        .zip(periods)
        .filter_map(|((asset, _), period)| period.map(|period| (asset, period)))
        .collect()
}

/// Draws, renders, searches and scores `count` mixes, one per thread.
pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<MixesReport, String> {
    let roots = Roots {
        library: &library.root,
        second: options.other_root,
    };
    let (families, pairs) = if options.kind.beatmatches() {
        shared_pairs(options.panels, &roots, options.kind)?
    } else {
        (BTreeSet::new(), Vec::new())
    };
    let (records, held_out, index) = sweep_index(
        library,
        store,
        clusters,
        options.seed,
        options.panels,
        options.matching,
        &families,
    )?;
    let index = &index;
    let verifier = options.verify.then(|| Verifier::new(&records, None));
    let without_families: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !families.contains(&record.header.source.path))
        .cloned()
        .collect();
    let pools = Pools::new(&without_families, &held_out);
    let extras = LongExtras {
        periods: if options.kind.beatmatches() {
            beat_periods(&pools, &pairs, &roots, options.jobs)
        } else {
            BTreeMap::new()
        },
        pairs,
    };
    let mut rng = Rng::new(options.seed);
    let dir = options.kind.renders(work, options.seed);
    let planned: Vec<(PlannedMix, PathBuf)> = (0..options.count)
        .map(|number| {
            let name = format!("mix-{number:02}");
            let path = dir.join(format!("{name}.mp3"));
            let mix = match options.kind {
                MixKind::Short => draw_mix(&name, &pools, clusters, &mut rng),
                kind => draw_long_mix(&name, kind, &pools, &extras, clusters, &mut rng),
            };
            mix.map(|mix| (mix, path))
        })
        .collect::<Result<_, _>>()?;
    let searcher = Searcher {
        index,
        ladder: options.ladder,
        matching: options.matching,
        verifier: verifier.as_ref(),
    };
    let results = map_in_order(&planned, options.jobs, |(mix, path)| {
        search_mix(mix, path, &roots, clusters, &searcher)
    });
    Ok(MixesReport {
        seed: options.seed,
        kind: options.kind,
        ladder: options.ladder_name.to_owned(),
        indexed_assets: index.assets().len(),
        mixes: results.into_iter().collect::<Result<_, _>>()?,
    })
}

/// What a mix is searched with: the index, the ladder and the matcher, and
/// the peak verifier when one measures the detections.
pub struct Searcher<'a> {
    pub index: &'a Index,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub verifier: Option<&'a Verifier<'a>>,
}

/// Renders `mix` to `path` unless an earlier run left it there with the
/// same plan (`<path stem>.truth.json`).
pub fn render_once(mix: &PlannedMix, roots: &Roots, path: &Path) -> Result<(), String> {
    let truth_path = path.with_extension("truth.json");
    let plan = serde_json::to_string_pretty(mix).map_err(|error| error.to_string())?;
    let rendered = crate::render::is_complete(path)
        && fs::read_to_string(&truth_path).is_ok_and(|saved| saved == plan);
    if !rendered {
        render(mix, roots, path)?;
        fs::write(&truth_path, &plan).map_err(|error| error.to_string())?;
    }
    Ok(())
}

/// Renders `mix` unless an earlier run left it at `path` with the same
/// plan, searches it on one thread and scores it.
pub fn search_mix(
    mix: &PlannedMix,
    path: &Path,
    roots: &Roots,
    clusters: &Clusters,
    searcher: &Searcher,
) -> Result<MixResult, String> {
    let Searcher {
        index,
        ladder,
        matching,
        verifier,
    } = *searcher;
    render_once(mix, roots, path)?;
    let profile = Profile::CURRENT;
    let audio =
        decode(path, profile.sample_rate, Excerpt::default()).map_err(|error| error.to_string())?;
    let detections: Vec<MixFound> = matching
        .search(index, &audio.samples, &profile, ladder, 1)
        .iter()
        .map(|detection| {
            let verified = verifier
                .and_then(|verifier| verifier.verify(index, detection, &audio.samples, &profile));
            MixFound::new(index, detection, verified)
        })
        .collect();
    Ok(MixResult {
        mix: mix.clone(),
        score: score(mix, &detections, clusters),
        detections,
    })
}

pub fn print_summary(report: &MixesReport, kind: MixKind) {
    let plays: Vec<&PlayScore> = report
        .mixes
        .iter()
        .flat_map(|result| &result.score.plays)
        .collect();
    let indexed: Vec<&&PlayScore> = plays.iter().filter(|play| !play.truth.held_out).collect();
    let count = |level: &str| indexed.iter().filter(|play| play.level == level).count();
    println!(
        "generated mixes, seed {}, {} ladder: {} mixes, {} plays of indexed tracks, {} of held-out ones",
        report.seed,
        report.ladder,
        report.mixes.len(),
        indexed.len(),
        plays.len() - indexed.len()
    );
    println!(
        "indexed plays: {} confident, {} possible, {} weak, {} not found",
        count("confident"),
        count("possible"),
        count("weak"),
        count("none")
    );
    let wrong: usize = report
        .mixes
        .iter()
        .map(|result| result.score.wrong_confident.len())
        .sum();
    let wrong_possible: usize = report
        .mixes
        .iter()
        .map(|result| result.score.wrong_possible.len())
        .sum();
    let strongest_false = report
        .mixes
        .iter()
        .flat_map(|result| &result.score.false_candidates)
        .map(|found| found.hits)
        .max()
        .unwrap_or(0);
    println!(
        "wrong: {wrong} confident, {wrong_possible} possible; strongest false candidate {strongest_false} hits"
    );
    let lengths = kind.play_lengths();
    let longest = lengths[3].1 - 1.0;
    for (low, high) in lengths {
        let here: Vec<&&&PlayScore> = indexed
            .iter()
            .filter(|play| {
                let seconds = play.truth.end_seconds - play.truth.start_seconds;
                (low..high).contains(&seconds)
            })
            .collect();
        let confident = here.iter().filter(|play| play.level == "confident").count();
        println!(
            "  plays of {low:.0}-{:.0} s: {confident}/{} confident",
            high.min(longest),
            here.len()
        );
    }
    let mut late: Vec<f64> = indexed
        .iter()
        .filter_map(|play| play.start_late_seconds)
        .collect();
    let mut early: Vec<f64> = indexed
        .iter()
        .filter_map(|play| play.end_early_seconds)
        .collect();
    late.sort_by(f64::total_cmp);
    early.sort_by(f64::total_cmp);
    let quantile = |values: &[f64], share: f64| {
        values
            .get(((values.len() as f64 - 1.0) * share).round() as usize)
            .copied()
            .unwrap_or(f64::NAN)
    };
    println!(
        "boundaries (plays found as possible or better): start late by median {:.1} s (p10 {:.1}, p90 {:.1}), end early by median {:.1} s (p10 {:.1}, p90 {:.1})",
        quantile(&late, 0.5),
        quantile(&late, 0.1),
        quantile(&late, 0.9),
        quantile(&early, 0.5),
        quantile(&early, 0.1),
        quantile(&early, 0.9)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn play(start_seconds: f64, end_seconds: f64) -> PlannedPlay {
        PlannedPlay {
            asset: String::from("a.mp3"),
            held_out: false,
            source_start_seconds: 100.0,
            speed: 1.02,
            start_seconds,
            end_seconds,
            fade_in_seconds: 0.0,
            fade_out_seconds: 0.0,
            bass_cut_until: None,
            bass_cut_from: None,
            ride: None,
        }
    }

    fn found(asset: &str, start_seconds: f64, end_seconds: f64, hits: u32) -> MixFound {
        MixFound {
            asset: asset.to_owned(),
            start_seconds,
            end_seconds,
            track_start_seconds: 100.0 + (start_seconds - 10.0) * 1.02,
            speed: 1.02,
            windows: 4,
            hits,
            fitted: false,
            verified: None,
        }
    }

    fn record(path: &str, duration_seconds: f64) -> PeakRecord {
        use gunfinger_core::library::{Asset, Timestamp};
        use gunfinger_core::store::RecordHeader;
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds,
            },
            peaks: Vec::new(),
        }
    }

    fn no_clusters() -> Clusters {
        Clusters {
            criterion: String::new(),
            duplicates: Vec::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        }
    }

    #[test]
    fn a_seed_draws_the_mix_it_drew_before_session_21() {
        // Drawn by the code at 8537226, before `drawn_play` was split out
        // of `draw_mix`: earlier sessions' seeds must give the same mixes.
        // Each row: asset, held out, source start, speed, start, end, fade
        // in, fade out, bass cut until, bass cut from.
        #[rustfmt::skip]
        let expected = [
            ("t02.mp3", false, 81.25297769561449, 0.9582444206030898, 0.0, 47.464822857918456, 0.0, 0.0, None, None),
            ("t06.mp3", false, 306.5496789897331, 1.0182256284085305, 47.464822857918456, 85.16827375316791, 0.0, 7.047611857784965, None, None),
            ("t15.mp3", false, 323.2950208166506, 0.9993199194195205, 78.12066189538294, 128.21470352770163, 7.047611857784965, 9.464024758350671, None, None),
            ("t04.mp3", false, 188.93877512744695, 0.9550532364801309, 118.75067876935096, 166.32394994156178, 9.464024758350671, 10.97825708408916, None, Some(160.83482139951718)),
            ("t08.mp3", false, 206.22390807477996, 0.991239056932078, 155.3456928574726, 208.51624878745596, 10.97825708408916, 9.763308655405218, Some(160.83482139951718), None),
            ("t14.mp3", false, 563.2184281621959, 1.028660792758029, 198.75294013205075, 229.72642043185417, 9.763308655405218, 5.083857215958616, None, None),
            ("t05.mp3", false, 72.79518362040584, 1.034152287157502, 224.64256321589556, 261.89667702558495, 5.083857215958616, 0.0, None, None),
            ("t08.mp3", false, 84.7042925961234, 1.0169253432194165, 261.89667702558495, 302.37425035205814, 0.0, 4.384424356336915, None, None),
            ("t00.mp3", false, 64.62033250417596, 1.0577616269144685, 297.9898259957212, 326.2354745022363, 4.384424356336915, 0.0, None, None),
            ("t03.mp3", true, 52.325341838368935, 0.9848553131731824, 326.2354745022363, 352.365826955674, 0.0, 10.770798286386697, None, Some(346.9804278124806)),
            ("t10.mp3", false, 413.03941213357837, 0.9512320587310388, 341.5950286692873, 387.6552882007378, 10.770798286386697, 13.61669515412574, Some(346.9804278124806), Some(380.8469406236749)),
            ("t12.mp3", false, 320.5756099922738, 1.02070382538944, 374.03859304661205, 423.6283468830277, 13.61669515412574, 0.0, Some(380.8469406236749), None),
        ];
        let records: Vec<PeakRecord> = (0..16)
            .map(|n| record(&format!("t{n:02}.mp3"), 150.0 + 37.0 * f64::from(n)))
            .collect();
        let held_out = BTreeSet::from([
            String::from("t03.mp3"),
            String::from("t07.mp3"),
            String::from("t11.mp3"),
        ]);
        let clusters = Clusters {
            duplicates: vec![vec![String::from("t01.mp3"), String::from("t02.mp3")]],
            ..no_clusters()
        };

        let mix = draw_mix(
            "pinned",
            &Pools::new(&records, &held_out),
            &clusters,
            &mut Rng::new(2026),
        )
        .unwrap();

        let close = |a: f64, b: f64| (a - b).abs() < 1e-9;
        let close_option = |a: Option<f64>, b: Option<f64>| match (a, b) {
            (Some(a), Some(b)) => close(a, b),
            (a, b) => a == b,
        };
        assert_eq!(mix.plays.len(), expected.len());
        for (row, (play, expected)) in mix.plays.iter().zip(expected).enumerate() {
            let (asset, held_out, source_start, speed, start, end, fade_in, fade_out, until, from) =
                expected;
            assert_eq!(
                (play.asset.as_str(), play.held_out),
                (asset, held_out),
                "row {row}"
            );
            assert!(
                close(play.source_start_seconds, source_start) && close(play.speed, speed),
                "row {row}: {play:?}"
            );
            assert!(
                close(play.start_seconds, start) && close(play.end_seconds, end),
                "row {row}: {play:?}"
            );
            assert!(
                close(play.fade_in_seconds, fade_in) && close(play.fade_out_seconds, fade_out),
                "row {row}: {play:?}"
            );
            assert!(
                close_option(play.bass_cut_until, until) && close_option(play.bass_cut_from, from),
                "row {row}: {play:?}"
            );
        }
        assert!(close(mix.seconds, 423.6283468830277));
    }

    #[test]
    fn a_long_mix_has_one_run_of_each_length_of_held_out_tracks_between_indexed_ones() {
        let records: Vec<PeakRecord> = (0..40)
            .map(|n| record(&format!("t{n:02}.mp3"), 200.0 + 10.0 * f64::from(n)))
            .collect();
        let held_out: BTreeSet<String> = (0..20).map(|n| format!("t{n:02}.mp3")).collect();

        let mix = draw_long_mix(
            "long",
            MixKind::Long,
            &Pools::new(&records, &held_out),
            &LongExtras::default(),
            &no_clusters(),
            &mut Rng::new(2026),
        )
        .unwrap();

        let mut runs: Vec<usize> = Vec::new();
        let mut run = 0;
        for play in &mix.plays {
            if play.held_out {
                run += 1;
            } else if run > 0 {
                runs.push(run);
                run = 0;
            }
        }
        runs.sort_unstable();
        assert_eq!(runs, [1, 2, 3, 4]);
        assert!(!mix.plays[0].held_out && !mix.plays[1].held_out);
        let indexed_pairs = mix
            .plays
            .windows(2)
            .filter(|pair| !pair[0].held_out && !pair[1].held_out)
            .count();
        assert!(indexed_pairs >= 5, "{indexed_pairs}");
        assert!(!mix.plays[mix.plays.len() - 2].held_out && !mix.plays.last().unwrap().held_out);
        let assets: BTreeSet<&str> = mix.plays.iter().map(|play| play.asset.as_str()).collect();
        assert_eq!(assets.len(), mix.plays.len());
        for play in &mix.plays {
            let seconds = play.end_seconds - play.start_seconds;
            let duration = 200.0 + 10.0 * play.asset[1..3].parse::<f64>().unwrap();
            assert!(
                seconds <= 360.0 && seconds <= 0.85 * duration / play.speed + 1e-9,
                "{play:?}"
            );
            assert!(
                play.source_at(play.end_seconds) <= 0.95 * duration + 1e-9,
                "{play:?}"
            );
        }
        for pair in mix.plays.windows(2) {
            let overlap = pair[0].end_seconds - pair[1].start_seconds;
            assert!((10.0..=60.0).contains(&overlap), "{overlap}");
            assert!((pair[1].fade_in_seconds - overlap).abs() < 1e-9);
        }
    }

    fn pair(family: &str, first: &str, second: &str) -> SharedPair {
        SharedPair {
            family: family.to_owned(),
            records: [
                PairRecord {
                    asset: first.to_owned(),
                    duration_seconds: 400.0,
                    passage: (300.0, 340.0),
                },
                PairRecord {
                    asset: second.to_owned(),
                    duration_seconds: 300.0,
                    passage: (20.0, 60.0),
                },
            ],
        }
    }

    #[test]
    fn a_shared_mix_plays_each_pair_back_to_back_over_its_passage_in_the_runs() {
        let records: Vec<PeakRecord> = (0..40)
            .map(|n| record(&format!("t{n:02}.mp3"), 200.0 + 10.0 * f64::from(n)))
            .collect();
        let held_out: BTreeSet<String> = (0..20).map(|n| format!("t{n:02}.mp3")).collect();
        let extras = LongExtras {
            periods: BTreeMap::new(),
            pairs: vec![
                pair("a", "a1.mp3", "a2.mp3"),
                pair("b", "b1.mp3", "b2.mp3"),
                pair("b", "b1.mp3", "b3.mp3"),
                pair("c", "c1.mp3", "c2.mp3"),
                pair("d", "d1.mp3", "d2.mp3"),
            ],
        };

        let mix = draw_long_mix(
            "shared",
            MixKind::Shared,
            &Pools::new(&records, &held_out),
            &extras,
            &no_clusters(),
            &mut Rng::new(7),
        )
        .unwrap();

        let paired: Vec<&PlannedPlay> = mix
            .plays
            .iter()
            .filter(|play| !play.asset.starts_with('t'))
            .collect();
        // Runs of 2 and 3 hold one pair, the run of 4 two.
        assert_eq!(paired.len(), 8);
        let families: BTreeSet<char> = paired
            .iter()
            .map(|play| play.asset.chars().next().unwrap())
            .collect();
        assert_eq!(families.len(), 4);
        for play in &paired {
            assert!(play.held_out);
            let (from, to) = if play.asset.ends_with("1.mp3") {
                (300.0, 340.0)
            } else {
                (20.0, 60.0)
            };
            assert!(
                play.source_start_seconds <= from + 1e-9
                    && play.source_at(play.end_seconds) >= to - 1e-9,
                "{play:?}"
            );
        }
        for pair in mix.plays.windows(2) {
            if pair[0].asset.ends_with("1.mp3") {
                assert_eq!(pair[1].asset[..1], pair[0].asset[..1]);
            }
        }
        let mut runs: Vec<usize> = Vec::new();
        let mut run = 0;
        for play in &mix.plays {
            if play.held_out {
                run += 1;
            } else if run > 0 {
                runs.push(run);
                run = 0;
            }
        }
        runs.sort_unstable();
        assert_eq!(runs, [1, 2, 3, 4]);
    }

    #[test]
    fn a_beatmatched_play_takes_the_tempo_of_the_one_before() {
        let mut previous = play(0.0, 200.0);
        previous.asset = String::from("a.mp3");
        previous.speed = 1.02;
        let periods = BTreeMap::from([
            (String::from("a.mp3"), 0.35),
            (String::from("b.mp3"), 0.36),
            // Twice b's period: read at half the tempo, still b's up to that.
            (String::from("c.mp3"), 0.72),
            (String::from("d.mp3"), 0.395),
        ]);

        let b = beatmatched_speed(&previous, "b.mp3", &periods).unwrap();
        let c = beatmatched_speed(&previous, "c.mp3", &periods).unwrap();

        assert!((1.02 / 0.35 - b / 0.36).abs() < 1e-9, "{b}");
        assert!((b - c).abs() < 1e-9, "{c}");
        assert_eq!(beatmatched_speed(&previous, "d.mp3", &periods), None);
        assert_eq!(beatmatched_speed(&previous, "e.mp3", &periods), None);
    }

    #[test]
    fn a_ride_changes_the_speed_linearly_then_keeps_it() {
        let mut ridden = play(10.0, 110.0);
        ridden.speed = 1.0;
        ridden.source_start_seconds = 0.0;
        ridden.ride = Some(Ride {
            start_seconds: 50.0,
            seconds: 10.0,
            to_speed: 1.02,
        });

        assert!((ridden.source_at(50.0) - 40.0).abs() < 1e-9);
        // Halfway up the ramp: 5 s at an average speed of 1.005.
        assert!((ridden.source_at(55.0) - 45.025).abs() < 1e-9);
        assert!((ridden.source_at(60.0) - 50.1).abs() < 1e-9);
        assert!((ridden.source_at(110.0) - (50.1 + 50.0 * 1.02)).abs() < 1e-9);
    }

    #[test]
    fn a_ridden_mix_rides_every_play_by_one_to_three_percent_within_it() {
        let records: Vec<PeakRecord> = (0..40)
            .map(|n| record(&format!("t{n:02}.mp3"), 200.0 + 10.0 * f64::from(n)))
            .collect();
        let held_out: BTreeSet<String> = (0..20).map(|n| format!("t{n:02}.mp3")).collect();

        let mix = draw_long_mix(
            "rides",
            MixKind::Rides,
            &Pools::new(&records, &held_out),
            &LongExtras::default(),
            &no_clusters(),
            &mut Rng::new(3),
        )
        .unwrap();

        for play in &mix.plays {
            let ride = play.ride.unwrap();
            let change = (ride.to_speed / play.speed - 1.0).abs();
            assert!((0.01 - 1e-9..=0.03 + 1e-9).contains(&change), "{play:?}");
            assert!((ride.to_speed - 1.0).abs() <= RIDE_LIMIT + 1e-9, "{play:?}");
            let seconds = play.end_seconds - play.start_seconds;
            let into = ride.start_seconds - play.start_seconds;
            assert!(
                (0.3 * seconds - 1e-9..=0.7 * seconds + 1e-9).contains(&into)
                    && ride.start_seconds + ride.seconds <= play.end_seconds + 1e-9,
                "{play:?}"
            );
            let duration = 200.0 + 10.0 * play.asset[1..3].parse::<f64>().unwrap();
            assert!(
                play.source_at(play.end_seconds) <= 0.97 * duration + 1e-9,
                "{play:?}"
            );
        }
    }

    #[test]
    fn a_mix_around_a_file_plays_it_whole_between_drawn_plays() {
        let records: Vec<PeakRecord> = (0..8)
            .map(|n| record(&format!("t{n}.mp3"), 300.0))
            .collect();
        let pool: Vec<&PeakRecord> = records.iter().collect();
        let whole = Whole {
            asset: "w.mp3",
            duration_seconds: 200.0,
        };

        let mix = draw_around("m", &whole, 3, &pool, &no_clusters(), &mut Rng::new(5)).unwrap();

        assert_eq!(mix.plays.len(), 7);
        let centre = &mix.plays[3];
        assert_eq!(centre.asset, "w.mp3");
        assert_eq!(centre.source_start_seconds, 0.0);
        assert!((centre.source_at(centre.end_seconds) - 199.9).abs() < 1e-9);
        assert!((centre.speed - 1.0).abs() <= SPEED_RANGE);
        let drawn: BTreeSet<&str> = mix
            .plays
            .iter()
            .filter(|play| play.asset != "w.mp3")
            .map(|play| play.asset.as_str())
            .collect();
        assert_eq!(drawn.len(), 6);
        for pair in mix.plays.windows(2) {
            let overlap = pair[0].end_seconds - pair[1].start_seconds;
            assert!((pair[0].fade_out_seconds - overlap).abs() < 1e-9);
            assert_eq!(pair[1].fade_in_seconds, pair[0].fade_out_seconds);
        }
        assert_eq!(mix.seconds, mix.plays[6].end_seconds);
    }

    #[test]
    fn a_mix_around_a_file_needs_enough_other_tracks() {
        let records: Vec<PeakRecord> = (0..5)
            .map(|n| record(&format!("t{n}.mp3"), 300.0))
            .collect();
        let pool: Vec<&PeakRecord> = records.iter().collect();
        let whole = Whole {
            asset: "w.mp3",
            duration_seconds: 200.0,
        };

        assert!(draw_around("m", &whole, 3, &pool, &no_clusters(), &mut Rng::new(5)).is_err());
    }

    #[test]
    fn a_bass_swap_hands_the_low_end_over_halfway_through_the_overlap() {
        let mut rng = Rng::new(3);
        let mut first = play(0.0, 40.0);
        let mut second = play(40.0, 80.0);
        // Draw until the transition is a bass swap.
        loop {
            let (mut a, mut b) = (first.clone(), second.clone());
            join(&mut a, &mut b, &mut rng);
            if a.bass_cut_from.is_some() {
                (first, second) = (a, b);
                break;
            }
        }

        let overlap = first.end_seconds - second.start_seconds;
        let swap = second.start_seconds + overlap / 2.0;
        assert!(overlap >= 8.0);
        assert_eq!(first.fade_out_seconds, overlap);
        assert_eq!(second.bass_cut_until, Some(swap));
        assert!(second.is_bass_cut(swap - 0.1) && !second.is_bass_cut(swap + 0.1));
        assert!(!first.is_bass_cut(swap - 0.1) && first.is_bass_cut(swap + 0.1));
        assert_eq!(cut_share(&second, swap - 1.0), 1.0);
        assert_eq!(cut_share(&second, swap + 1.0), 0.0);
        assert!((cut_share(&second, swap) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn detections_are_matched_to_the_play_they_overlap() {
        let mix = PlannedMix {
            name: String::from("mix"),
            seconds: 100.0,
            plays: vec![
                play(10.0, 50.0),
                PlannedPlay {
                    asset: String::from("b.mp3"),
                    held_out: true,
                    ..play(45.0, 100.0)
                },
            ],
        };
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: Vec::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let found = [
            found("a.mp3", 12.0, 48.0, 400),
            found("a.mp3", 70.0, 90.0, 30),
            found("c.mp3", 60.0, 95.0, 250),
        ];

        let score = score(&mix, &found, &clusters);

        assert_eq!(score.plays[0].level, "confident");
        assert_eq!(score.plays[0].start_late_seconds, Some(2.0));
        assert_eq!(score.plays[0].end_early_seconds, Some(2.0));
        assert!(score.plays[0].position_error_seconds.unwrap().abs() < 1e-9);
        assert_eq!(score.plays[1].level, "none");
        assert_eq!(score.wrong_confident, [found[2].clone()]);
        assert_eq!(score.false_candidates.len(), 2);
    }
}

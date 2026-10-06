//! Generated mixes: seeded mixes of library tracks with exact truth, for
//! what the real mixes cannot measure: brief plays, detection boundaries,
//! cuts, bass swaps, a returning track. Rendered with the robustness
//! excerpts' tools, they complement real mixes rather than replace them.
//!
//! The index and the held-out clusters are the sweep's for the same seed;
//! held-out tracks play in the mixes too and must produce nothing
//! confident.

use std::collections::BTreeSet;
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
use crate::sweep::Plan;

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
}

impl PlannedPlay {
    /// The position in the track heard at mix time `seconds`.
    pub fn source_at(&self, seconds: f64) -> f64 {
        self.source_start_seconds + (seconds - self.start_seconds) * self.speed
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
pub fn render(mix: &PlannedMix, library: &Library, output: &Path) -> Result<(), String> {
    let mut samples = vec![0.0_f32; to_index(mix.seconds)];
    for play in &mix.plays {
        let first = to_index(play.start_seconds);
        for (offset, sample) in play_audio(play, library)?.into_iter().enumerate() {
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
fn play_audio(play: &PlannedPlay, library: &Library) -> Result<Vec<f32>, String> {
    let source = library.root.join(&play.asset);
    let seconds = play.end_seconds - play.start_seconds;
    let render = |filter| {
        render_samples(
            &source,
            play.source_start_seconds,
            seconds,
            Playback::Turntable(play.speed),
            filter,
        )
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
}

impl MixFound {
    pub fn new(index: &Index, detection: &Detection) -> MixFound {
        MixFound {
            asset: index.asset(detection.asset).path.clone(),
            start_seconds: detection.start_seconds,
            end_seconds: detection.end_seconds,
            track_start_seconds: detection.track_start_seconds,
            speed: detection.speed.0,
            windows: detection.evidence.windows,
            hits: detection.evidence.hits,
            fitted: detection.evidence.pass == Pass::Fitted,
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
        let seconds = PLAY_SECONDS.0 + rng.unit() * (PLAY_SECONDS.1 - PLAY_SECONDS.0);
        let speed = 1.0 + (2.0 * rng.unit() - 1.0) * SPEED_RANGE;
        // Plays stay clear of the first tenth of a track, often a sparse
        // intro; tracks in the pools are long enough for any play.
        let duration = record.header.duration_seconds;
        let earliest = 0.1 * duration;
        let latest = 0.95 * duration - seconds * speed;
        let mut play = PlannedPlay {
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
    /// Where the seed's sweep panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    pub count: usize,
    pub ladder_name: &'a str,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub jobs: usize,
}

/// The library's records, the sweep's held-out assets for `seed`, and the
/// index of the rest.
pub fn sweep_index(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    seed: u64,
    panels: &Path,
    matching: &Matching,
) -> Result<(Vec<PeakRecord>, BTreeSet<String>, Index), String> {
    let (records, _) = load_records(library, store, &Profile::CURRENT, &BTreeSet::new());
    let held_out = Plan::for_seed(&records, clusters, seed, panels)?.held_out;
    let indexed: Vec<PeakRecord> = records
        .iter()
        .filter(|record| !held_out.contains(&record.header.source.path))
        .cloned()
        .collect();
    let index = Index::build(&indexed).map_err(|error| error.to_string())?;
    Ok((records, held_out, matching.index(index)))
}

/// Draws, renders, searches and scores `count` mixes, one per thread.
pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<MixesReport, String> {
    let (records, held_out, index) = sweep_index(
        library,
        store,
        clusters,
        options.seed,
        options.panels,
        options.matching,
    )?;
    let index = &index;
    let pools = Pools::new(&records, &held_out);
    let mut rng = Rng::new(options.seed);
    let dir = work.join("mixes").join(format!("seed-{}", options.seed));
    let planned: Vec<(PlannedMix, PathBuf)> = (0..options.count)
        .map(|number| {
            let name = format!("mix-{number:02}");
            let path = dir.join(format!("{name}.mp3"));
            draw_mix(&name, &pools, clusters, &mut rng).map(|mix| (mix, path))
        })
        .collect::<Result<_, _>>()?;
    let results = map_in_order(&planned, options.jobs, |(mix, path)| {
        search_mix(
            mix,
            path,
            library,
            index,
            clusters,
            options.ladder,
            options.matching,
        )
    });
    Ok(MixesReport {
        seed: options.seed,
        ladder: options.ladder_name.to_owned(),
        indexed_assets: index.assets().len(),
        mixes: results.into_iter().collect::<Result<_, _>>()?,
    })
}

/// Renders `mix` unless an earlier run left it at `path` with the same
/// plan, searches it on one thread and scores it.
pub fn search_mix(
    mix: &PlannedMix,
    path: &Path,
    library: &Library,
    index: &Index,
    clusters: &Clusters,
    ladder: &[Rung],
    matching: &Matching,
) -> Result<MixResult, String> {
    let truth_path = path.with_extension("truth.json");
    let plan = serde_json::to_string_pretty(mix).map_err(|error| error.to_string())?;
    let rendered =
        path.exists() && fs::read_to_string(&truth_path).is_ok_and(|saved| saved == plan);
    if !rendered {
        render(mix, library, path)?;
        fs::write(&truth_path, &plan).map_err(|error| error.to_string())?;
    }
    let profile = Profile::CURRENT;
    let audio =
        decode(path, profile.sample_rate, Excerpt::default()).map_err(|error| error.to_string())?;
    let detections: Vec<MixFound> = matching
        .search(index, &audio.samples, &profile, ladder, 1)
        .iter()
        .map(|detection| MixFound::new(index, detection))
        .collect();
    Ok(MixResult {
        mix: mix.clone(),
        score: score(mix, &detections, clusters),
        detections,
    })
}

pub fn print_summary(report: &MixesReport) {
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
    for (low, high) in [(20.0, 30.0), (30.0, 40.0), (40.0, 50.0), (50.0, 61.0)] {
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
            high.min(60.0),
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
        }
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

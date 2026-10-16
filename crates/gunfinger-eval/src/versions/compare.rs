//! A reference against a mix along one fitted line, both on the mix's
//! clock (`docs/brief-24.md`, step 3): band levels, 2 s patches every
//! 0.256 s, the reference's band dynamics against the mix's, a gain per
//! DJ EQ group fitted on the anchor bands of one fold of a patch and
//! tested on the other fold, and a state for each band and patch. Pure:
//! the caller decodes the mix and renders the reference.
//!
//! The model is the external report's (research note 2026-10-16): the mix
//! is the reference through a fader and a three-band EQ, plus other
//! music, which only adds power. So a band where the reference predicts
//! much more power than the mix holds is either cut by the EQ, when the
//! cut has the shape of a DJ EQ's shelf or mid cut (unobservable), or a
//! selective absence no DJ EQ makes (contradicted); a band where the mix
//! holds at least twice the power the reference predicts is masked
//! (unobservable).
//!
//! Version 2 (`docs/brief-26.md`, step 3): a band is contradicted only
//! where its EQ group demonstrably passes the reference, with an anchor
//! in the patch. A group without one has a borrowed gain, so nothing
//! shows it open: under a cut, another record's material fills it at any
//! level (experiment 0127's Astro-Sine plays).

use gunfinger_core::profile::Profile;
use gunfinger_core::spectrogram::for_each_frame;
use serde::Serialize;

pub const BANDS: usize = 20;
pub const GROUPS: usize = 3;
/// The first band of the mixer's low (39-246 Hz), mid (246-2,465 Hz) and
/// high (2,465-3,906 Hz) groups.
const GROUP_STARTS: [usize; GROUPS] = [0, 8, 18];
/// Band edges are log-spaced over the profile's bins 5-500.
const LOWEST_BIN: f64 = 5.0;
const BIN_RATIO: f64 = 100.0;

/// A patch is 2.0 s of 16 ms frames...
pub const PATCH_FRAMES: usize = 125;
/// ...and one starts every 0.256 s.
pub const STEP_FRAMES: usize = 16;
/// Dynamics are level differences 48 ms apart.
const DYNAMICS_FRAMES: usize = 3;
/// The one lag a patch may take, shared by all bands (±48 ms).
const MAX_LAG_FRAMES: isize = 3;
/// The two folds alternate in blocks of 0.256 s.
const BLOCK_FRAMES: usize = 16;

/// A band is informative when the reference's dynamics there vary by at
/// least this much (standard deviation, dB)...
const MIN_DYNAMICS_DB: f64 = 1.0;
/// ...and its mean level is within this much of the reference's loudest
/// band in the patch.
const LEVEL_RANGE_DB: f64 = 40.0;
/// A cell is salient within this much of its band's loudest frame.
const SALIENT_DB: f64 = 15.0;
const ANCHOR_CORRELATION: f64 = 0.6;
const SUPPORTED_CORRELATION: f64 = 0.5;
const CONTRADICTED_CORRELATION: f64 = 0.2;
/// The reference predicts ten times the power the mix holds.
const DEFICIT_DB: f64 = -10.0;
/// The mix holds twice the power the reference predicts, so the
/// reference is at most half of the band.
const MASKED_DB: f64 = 3.0;
/// A group's gain stays within these bounds of the play's typical gain.
const GAIN_BELOW_DB: f64 = 40.0;
const GAIN_ABOVE_DB: f64 = 12.0;
/// Digital silence (`for_each_frame` keeps it finite at -200 dB a bin).
const SILENT_DB: f64 = -150.0;

/// Band levels in dB, frame by frame.
pub type Levels = Vec<[f64; BANDS]>;

/// The first bin of each band, and the end of the last.
fn band_edges() -> [usize; BANDS + 1] {
    let mut edges = [0; BANDS + 1];
    for (i, edge) in edges.iter_mut().enumerate() {
        let exponent = i as f64 / BANDS as f64;
        *edge = (LOWEST_BIN * BIN_RATIO.powf(exponent)).round() as usize;
    }
    edges
}

pub fn group_of(band: usize) -> usize {
    GROUP_STARTS
        .iter()
        .rposition(|&start| band >= start)
        .unwrap_or(0)
}

/// Each frame's level in each band: 10 log10 of the summed power of its
/// bins.
pub fn levels(samples: &[f32], profile: &Profile) -> Levels {
    let edges = band_edges();
    let mut levels = Vec::new();
    for_each_frame(samples, profile, |power_db| {
        let mut frame = [0.0; BANDS];
        for (band, level) in frame.iter_mut().enumerate() {
            let power: f64 = power_db[edges[band]..edges[band + 1]]
                .iter()
                .map(|&db| 10f64.powf(f64::from(db) / 10.0))
                .sum();
            *level = 10.0 * power.log10();
        }
        levels.push(frame);
    });
    levels
}

/// What one patch measures before any gain is fitted.
pub struct Measured {
    /// The patch's first frame in the mix; the reference's frame paired
    /// with mix frame `i` is `i + lag`.
    pub first: usize,
    pub lag: isize,
    pub bands: [Band; BANDS],
}

#[derive(Clone, Default)]
pub struct Band {
    pub informative: bool,
    /// The correlation of the two band dynamics, for an informative band.
    pub correlation: Option<f64>,
    /// `mix - reference` in dB at the salient cells of each fold.
    pub offsets: [Vec<f64>; 2],
}

impl Band {
    fn is_anchor(&self) -> bool {
        self.correlation
            .is_some_and(|correlation| correlation >= ANCHOR_CORRELATION)
    }
}

/// Every patch of mix frames `first..end` (all its frames inside), with
/// the reference on the same frames. Patches where either side is
/// digitally silent are left out: there is nothing to compare.
pub fn measure(mix: &Levels, reference: &Levels, first: usize, end: usize) -> Vec<Measured> {
    let end = end.min(mix.len());
    let mut patches = Vec::new();
    let mut start = first.max(DYNAMICS_FRAMES);
    while start + PATCH_FRAMES <= end {
        if let Some(patch) = measure_patch(mix, reference, start) {
            patches.push(patch);
        }
        start += STEP_FRAMES;
    }
    patches
}

fn measure_patch(mix: &Levels, reference: &Levels, first: usize) -> Option<Measured> {
    let frames = first - DYNAMICS_FRAMES..first + PATCH_FRAMES;
    if frames.clone().any(|frame| silent(&mix[frame])) {
        return None;
    }
    let mut best: Option<(f64, Measured)> = None;
    for lag in -MAX_LAG_FRAMES..=MAX_LAG_FRAMES {
        let Some(paired) = paired(reference, first, lag) else {
            continue;
        };
        let loudest = loudest_band(&paired[DYNAMICS_FRAMES..]);
        let bands = std::array::from_fn(|band| measure_band(mix, paired, first, band, loudest));
        let correlations: Vec<f64> = bands.iter().filter_map(|band| band.correlation).collect();
        let score = if correlations.is_empty() {
            f64::NEG_INFINITY
        } else {
            correlations.iter().sum::<f64>() / correlations.len() as f64
        };
        // Ties keep the earlier lag, so the order of the loop decides.
        if best.as_ref().is_none_or(|(highest, _)| score > *highest) {
            best = Some((score, Measured { first, lag, bands }));
        }
    }
    best.map(|(_, measured)| measured)
}

/// The reference's frames paired with mix frames `first - DYNAMICS_FRAMES
/// .. first + PATCH_FRAMES` at `lag`, if they all exist and none is
/// digital silence.
fn paired(reference: &Levels, first: usize, lag: isize) -> Option<&[[f64; BANDS]]> {
    let start = (first - DYNAMICS_FRAMES).checked_add_signed(lag)?;
    let frames = reference.get(start..start + DYNAMICS_FRAMES + PATCH_FRAMES)?;
    (!frames.iter().any(silent)).then_some(frames)
}

fn silent(frame: &[f64; BANDS]) -> bool {
    frame.iter().all(|&level| level < SILENT_DB)
}

/// The mean level of the reference's loudest band over `frames`.
fn loudest_band(frames: &[[f64; BANDS]]) -> f64 {
    (0..BANDS)
        .map(|band| mean(frames.iter().map(|frame| frame[band])))
        .fold(f64::NEG_INFINITY, f64::max)
}

/// One band of a patch: `paired` holds the reference's frames, the
/// first `DYNAMICS_FRAMES` of them before the patch.
fn measure_band(
    mix: &Levels,
    paired: &[[f64; BANDS]],
    first: usize,
    band: usize,
    loudest_band: f64,
) -> Band {
    let reference: Vec<f64> = paired.iter().map(|frame| frame[band]).collect();
    let heard: Vec<f64> = mix[first - DYNAMICS_FRAMES..first + PATCH_FRAMES]
        .iter()
        .map(|frame| frame[band])
        .collect();
    let reference_dynamics = differences(&reference);
    let mix_dynamics = differences(&heard);
    let own = &reference[DYNAMICS_FRAMES..];
    let informative = standard_deviation(&reference_dynamics) >= MIN_DYNAMICS_DB
        && mean(own.iter().copied()) >= loudest_band - LEVEL_RANGE_DB;
    let loudest_frame = own.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let mut offsets = [Vec::new(), Vec::new()];
    for (frame, (&level, &mix_level)) in own.iter().zip(&heard[DYNAMICS_FRAMES..]).enumerate() {
        if level >= loudest_frame - SALIENT_DB {
            offsets[(frame / BLOCK_FRAMES) % 2].push(mix_level - level);
        }
    }
    Band {
        informative,
        correlation: informative.then(|| correlation(&reference_dynamics, &mix_dynamics)),
        offsets,
    }
}

/// `L(i) - L(i - DYNAMICS_FRAMES)` for every frame after the first
/// `DYNAMICS_FRAMES`.
fn differences(levels: &[f64]) -> Vec<f64> {
    levels
        .windows(DYNAMICS_FRAMES + 1)
        .map(|pair| pair[DYNAMICS_FRAMES] - pair[0])
        .collect()
}

/// The play's typical gains, from every patch with an anchor: the median
/// broadband gain, and each group's median offset from it (0 for a group
/// that never has an anchor).
pub struct Typical {
    pub broadband: Option<f64>,
    pub group_offsets: [f64; GROUPS],
}

pub fn typical<'a>(patches: impl Iterator<Item = &'a Measured>) -> Typical {
    let mut broadband = Vec::new();
    let mut group_offsets: [Vec<f64>; GROUPS] = Default::default();
    for patch in patches {
        let Some(overall) = median(anchor_offsets(patch, None, Folds::Both)) else {
            continue;
        };
        broadband.push(overall);
        for (group, offsets) in group_offsets.iter_mut().enumerate() {
            if let Some(gain) = median(anchor_offsets(patch, Some(group), Folds::Both)) {
                offsets.push(gain - overall);
            }
        }
    }
    Typical {
        broadband: median(broadband),
        group_offsets: group_offsets.map(|offsets| median(offsets).unwrap_or(0.0)),
    }
}

#[derive(Clone, Copy)]
enum Folds {
    One(usize),
    Both,
}

/// The offsets of the anchor bands' salient cells, of one group or all.
fn anchor_offsets(patch: &Measured, group: Option<usize>, folds: Folds) -> Vec<f64> {
    patch
        .bands
        .iter()
        .enumerate()
        .filter(|(band, state)| state.is_anchor() && group.is_none_or(|g| group_of(*band) == g))
        .flat_map(|(_, state)| match folds {
            Folds::One(fold) => state.offsets[fold].clone(),
            Folds::Both => [state.offsets[0].clone(), state.offsets[1].clone()].concat(),
        })
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BandState {
    NotInformative,
    Killed,
    Supported,
    Contradicted,
    Masked,
    Undecided,
}

impl BandState {
    pub fn letter(self) -> char {
        match self {
            BandState::NotInformative => '.',
            BandState::Killed => 'K',
            BandState::Supported => 'S',
            BandState::Contradicted => 'C',
            BandState::Masked => 'M',
            BandState::Undecided => '?',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PatchState {
    Supported,
    Contradicted,
    Unobservable,
}

pub struct Judged {
    pub first: usize,
    pub lag: isize,
    /// Each group's gain, the mean of the two folds' fits where both
    /// exist.
    pub gains: [Option<f64>; GROUPS],
    pub correlations: [Option<f64>; BANDS],
    pub residuals: [Option<f64>; BANDS],
    pub bands: [BandState; BANDS],
    pub state: PatchState,
    /// The state version 1's band rules give, without the group rule;
    /// reported beside, deciding nothing.
    pub version_1_state: PatchState,
}

/// The patch's band and patch states under gains fitted on each fold
/// and tested on the other.
pub fn judge(patch: &Measured, typical: &Typical) -> Judged {
    let fitted = [0, 1].map(|fold| fold_gains(patch, typical, fold));
    let residuals: [Option<f64>; BANDS] = std::array::from_fn(|band| {
        let group = group_of(band);
        let state = &patch.bands[band];
        if !state.informative {
            return None;
        }
        // Each fold's cells against the gain fitted on the other fold.
        let tested: Vec<f64> = (0..2)
            .filter_map(|fold| fitted[1 - fold][group].map(|gain| (fold, gain)))
            .flat_map(|(fold, gain)| state.offsets[fold].iter().map(move |offset| offset - gain))
            .collect();
        median(tested)
    });
    let version_1_bands: [BandState; BANDS] = std::array::from_fn(|band| {
        let Some(correlation) = patch.bands[band].correlation else {
            return BandState::NotInformative;
        };
        band_state(correlation, residuals[band], || cut_by_eq(&residuals, band))
    });
    let anchored: [bool; GROUPS] = std::array::from_fn(|group| {
        (0..BANDS).any(|band| group_of(band) == group && patch.bands[band].is_anchor())
    });
    let bands = std::array::from_fn(|band| {
        if anchored[group_of(band)] {
            version_1_bands[band]
        } else {
            without_anchor(version_1_bands[band], residuals[band])
        }
    });
    let any_anchor = anchored.contains(&true);
    Judged {
        first: patch.first,
        lag: patch.lag,
        gains: std::array::from_fn(|group| mean_of(fitted[0][group], fitted[1][group])),
        correlations: std::array::from_fn(|band| patch.bands[band].correlation),
        residuals,
        bands,
        state: patch_state(&bands, any_anchor),
        version_1_state: patch_state(&version_1_bands, any_anchor),
    }
}

/// Each group's gain fitted on one fold: the median offset of the
/// group's anchor bands there, or without them the fold's broadband gain
/// plus the play's typical offset of the group; within the bounds around
/// the play's typical gain.
fn fold_gains(patch: &Measured, typical: &Typical, fold: usize) -> [Option<f64>; GROUPS] {
    let broadband = median(anchor_offsets(patch, None, Folds::One(fold)));
    std::array::from_fn(|group| {
        let gain = median(anchor_offsets(patch, Some(group), Folds::One(fold)))
            .or_else(|| broadband.map(|gain| gain + typical.group_offsets[group]))?;
        Some(match typical.broadband {
            Some(centre) => gain.clamp(centre - GAIN_BELOW_DB, centre + GAIN_ABOVE_DB),
            None => gain,
        })
    })
}

/// The band's state, tested in the order the brief gives (as amended
/// after the synthetic tests).
fn band_state(
    correlation: f64,
    residual: Option<f64>,
    cut_by_eq: impl FnOnce() -> bool,
) -> BandState {
    if correlation >= SUPPORTED_CORRELATION {
        return BandState::Supported;
    }
    match residual {
        Some(e) if e < DEFICIT_DB => {
            if cut_by_eq() {
                BandState::Killed
            } else {
                BandState::Contradicted
            }
        }
        Some(e) if e > MASKED_DB => BandState::Masked,
        Some(_) if correlation < CONTRADICTED_CORRELATION => BandState::Contradicted,
        _ => BandState::Undecided,
    }
}

/// A band of a group with no anchor in the patch, whose gain is
/// borrowed: unless supported, killed when short of the prediction and
/// masked otherwise, never contradicted.
fn without_anchor(state: BandState, residual: Option<f64>) -> BandState {
    match state {
        BandState::NotInformative | BandState::Supported => state,
        _ if residual.is_some_and(|e| e < DEFICIT_DB) => BandState::Killed,
        _ => BandState::Masked,
    }
}

/// Whether the deficit of `band` has the shape of a DJ EQ's cut of its
/// group: a low shelf (every informative band below it in the low group
/// short too), a high shelf (every one above it in the high group), or a
/// mid cut (every informative mid band short, except perhaps the
/// group's lowest and highest, where a cut leaks past the crossovers).
/// Bands without a residual are not informative.
fn cut_by_eq(residuals: &[Option<f64>; BANDS], band: usize) -> bool {
    let group = group_of(band);
    let informative: Vec<usize> = (0..BANDS)
        .filter(|&other| group_of(other) == group && residuals[other].is_some())
        .collect();
    let short = |other: &usize| residuals[*other].is_some_and(|e| e < DEFICIT_DB);
    match group {
        0 => informative.iter().filter(|&&other| other < band).all(short),
        2 => informative.iter().filter(|&&other| other > band).all(short),
        _ => match informative.as_slice() {
            [_, inner @ .., _] => inner.iter().all(short),
            _ => informative.iter().all(short),
        },
    }
}

fn patch_state(bands: &[BandState; BANDS], any_anchor: bool) -> PatchState {
    if !any_anchor {
        return PatchState::Unobservable;
    }
    let supported = bands.iter().filter(|&&s| s == BandState::Supported).count();
    let contradicted = bands
        .iter()
        .filter(|&&s| s == BandState::Contradicted)
        .count();
    if contradicted >= 2 && 2 * contradicted >= supported {
        PatchState::Contradicted
    } else if supported >= 3 {
        PatchState::Supported
    } else {
        PatchState::Unobservable
    }
}

fn mean_of(a: Option<f64>, b: Option<f64>) -> Option<f64> {
    match (a, b) {
        (Some(a), Some(b)) => Some(f64::midpoint(a, b)),
        (one, other) => one.or(other),
    }
}

fn mean(values: impl Iterator<Item = f64>) -> f64 {
    let (sum, count) = values.fold((0.0, 0usize), |(sum, count), v| (sum + v, count + 1));
    if count == 0 {
        f64::NEG_INFINITY
    } else {
        sum / count as f64
    }
}

fn standard_deviation(values: &[f64]) -> f64 {
    let centre = mean(values.iter().copied());
    (values
        .iter()
        .map(|v| (v - centre) * (v - centre))
        .sum::<f64>()
        / values.len().max(1) as f64)
        .sqrt()
}

/// Pearson's correlation; 0 when either side is constant.
fn correlation(a: &[f64], b: &[f64]) -> f64 {
    let (mean_a, mean_b) = (mean(a.iter().copied()), mean(b.iter().copied()));
    let (mut ab, mut aa, mut bb) = (0.0, 0.0, 0.0);
    for (x, y) in a.iter().zip(b) {
        ab += (x - mean_a) * (y - mean_b);
        aa += (x - mean_a) * (x - mean_a);
        bb += (y - mean_b) * (y - mean_b);
    }
    if aa == 0.0 || bb == 0.0 {
        0.0
    } else {
        ab / (aa * bb).sqrt()
    }
}

pub fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    let middle = values.len() / 2;
    Some(if values.len() % 2 == 1 {
        values[middle]
    } else {
        f64::midpoint(values[middle - 1], values[middle])
    })
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    const RATE: f64 = 8000.0;
    const SECONDS: f64 = 24.0;

    /// A pseudo-random sequence, so that tests need no crate.
    struct Draws(u64);

    impl Draws {
        fn next(&mut self) -> f64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            (self.0 >> 11) as f64 / (1u64 << 53) as f64
        }
    }

    /// One part of a synthetic track: partials at fixed frequencies,
    /// sounding in decaying hits at the given times.
    struct Part {
        partials: Vec<(f64, f64)>,
        hits: Vec<f64>,
        decay_seconds: f64,
    }

    impl Part {
        /// `count` partials spread over `low..high` Hz with random phases.
        fn band(low: f64, high: f64, count: usize, seed: u64) -> Vec<(f64, f64)> {
            let mut draws = Draws(seed);
            (0..count)
                .map(|_| (low + (high - low) * draws.next(), 2.0 * PI * draws.next()))
                .collect()
        }

        fn every(period: f64, offset: f64) -> Vec<f64> {
            let mut hits = Vec::new();
            let mut t = offset;
            while t < SECONDS {
                hits.push(t);
                t += period;
            }
            hits
        }

        fn add_to(&self, samples: &mut [f32], level: f64) {
            for &hit in &self.hits {
                let first = (hit * RATE) as usize;
                let last = (((hit + 6.0 * self.decay_seconds) * RATE) as usize).min(samples.len());
                for (n, sample) in samples.iter_mut().enumerate().take(last).skip(first) {
                    let t = n as f64 / RATE;
                    let envelope = (-(t - hit) / self.decay_seconds).exp();
                    let tone: f64 = self
                        .partials
                        .iter()
                        .map(|&(frequency, phase)| (2.0 * PI * frequency * t + phase).sin())
                        .sum::<f64>()
                        / (self.partials.len() as f64).sqrt();
                    *sample += (level * envelope * tone) as f32;
                }
            }
        }
    }

    /// Notes of one partial each, a note every `period` seconds, cycling
    /// through `frequencies`.
    fn line(frequencies: &[f64], period: f64, decay_seconds: f64) -> Vec<Part> {
        frequencies
            .iter()
            .enumerate()
            .map(|(i, &frequency)| Part {
                partials: vec![(frequency, 0.0), (2.0 * frequency, 0.5)],
                hits: Part::every(period * frequencies.len() as f64, period * i as f64),
                decay_seconds,
            })
            .collect()
    }

    /// The drums both arrangements share.
    fn drums() -> Vec<(Part, Group)> {
        vec![
            (
                Part {
                    partials: vec![(55.0, 0.0), (110.0, 0.3)],
                    hits: Part::every(0.35, 0.0),
                    decay_seconds: 0.06,
                },
                Group::Low,
            ),
            (
                Part {
                    partials: Part::band(300.0, 2000.0, 40, 1),
                    hits: Part::every(0.7, 0.35),
                    decay_seconds: 0.05,
                },
                Group::Mid,
            ),
            (
                Part {
                    partials: Part::band(2600.0, 3800.0, 30, 2),
                    hits: Part::every(0.175, 0.0),
                    decay_seconds: 0.02,
                },
                Group::High,
            ),
        ]
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Group {
        Low,
        Mid,
        High,
    }

    /// Drums, a bassline and a lead; arrangement `b` has its own bassline
    /// and lead over the same drums.
    fn track(arrangement: char) -> Vec<(Part, Group)> {
        let mut parts = drums();
        let (bass, lead): (&[f64], &[f64]) = match arrangement {
            'a' => (&[45.0, 60.0, 50.0, 70.0], &[620.0, 930.0, 1240.0, 780.0]),
            _ => (&[65.0, 41.0, 75.0, 52.0], &[700.0, 1100.0, 560.0, 1500.0]),
        };
        let lead_period = if arrangement == 'a' { 0.525 } else { 0.4 };
        parts.extend(
            line(bass, 0.7, 0.4)
                .into_iter()
                .map(|part| (part, Group::Low)),
        );
        parts.extend(
            line(lead, lead_period, 0.15)
                .into_iter()
                .map(|part| (part, Group::Mid)),
        );
        parts
    }

    /// The parts mixed, each group at its gain.
    fn render(parts: &[(Part, Group)], gains: [f64; 3]) -> Vec<f32> {
        let mut samples = vec![0.0; (SECONDS * RATE) as usize];
        for (part, group) in parts {
            let gain = match group {
                Group::Low => gains[0],
                Group::Mid => gains[1],
                Group::High => gains[2],
            };
            if gain > 0.0 {
                part.add_to(&mut samples, 0.3 * gain);
            }
        }
        samples
    }

    fn flat(parts: &[(Part, Group)]) -> Vec<f32> {
        render(parts, [1.0, 1.0, 1.0])
    }

    /// Every patch of the whole excerpt, judged.
    fn judged(mix: &[f32], reference: &[f32]) -> Vec<Judged> {
        let profile = Profile::CURRENT;
        let mix = levels(mix, &profile);
        let reference = levels(reference, &profile);
        let patches = measure(&mix, &reference, 0, mix.len());
        let typical = typical(patches.iter());
        patches.iter().map(|patch| judge(patch, &typical)).collect()
    }

    fn count(patches: &[Judged], state: PatchState) -> usize {
        patches.iter().filter(|patch| patch.state == state).count()
    }

    fn contradicted_share(patches: &[Judged]) -> f64 {
        let contradicted = count(patches, PatchState::Contradicted);
        let supported = count(patches, PatchState::Supported);
        contradicted as f64 / (contradicted + supported).max(1) as f64
    }

    fn band_count(patches: &[Judged], state: BandState, bands: std::ops::Range<usize>) -> usize {
        patches
            .iter()
            .flat_map(|patch| patch.bands[bands.clone()].iter())
            .filter(|&&s| s == state)
            .count()
    }

    #[test]
    fn band_edges_rise_from_bin_5_to_bin_500() {
        let edges = band_edges();

        assert_eq!(edges[0], 5);
        assert_eq!(edges[BANDS], 500);
        assert!(edges.windows(2).all(|pair| pair[0] < pair[1]));
        // The low group ends near 250 Hz and the mid group near 2.5 kHz.
        assert_eq!(edges[GROUP_STARTS[1]], 32);
        assert_eq!(edges[GROUP_STARTS[2]], 315);
    }

    #[test]
    fn bands_belong_to_the_three_groups() {
        assert_eq!(group_of(0), 0);
        assert_eq!(group_of(7), 0);
        assert_eq!(group_of(8), 1);
        assert_eq!(group_of(17), 1);
        assert_eq!(group_of(18), 2);
        assert_eq!(group_of(19), 2);
    }

    #[test]
    fn the_same_recording_is_supported_throughout() {
        let a = flat(&track('a'));

        let patches = judged(&a, &a);

        assert!(patches.len() > 60, "{}", patches.len());
        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert_eq!(count(&patches, PatchState::Supported), patches.len());
        assert!(patches.iter().all(|patch| patch.lag == 0));
    }

    #[test]
    fn an_eqd_copy_is_still_supported() {
        let parts = track('a');
        let reference = flat(&parts);
        // Lows down 6 dB, highs up 3 dB, everything 10 dB quieter.
        let mix: Vec<f32> = render(&parts, [0.5, 1.0, 1.41])
            .iter()
            .map(|x| x * 0.316)
            .collect();

        let patches = judged(&mix, &reference);

        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert!(count(&patches, PatchState::Supported) as f64 >= 0.95 * patches.len() as f64);
        // The fitted gains are the EQ's: -16, -10 and -7 dB.
        for (group, expected) in [-16.0, -10.0, -7.0].into_iter().enumerate() {
            let gains = patches
                .iter()
                .filter_map(|patch| patch.gains[group])
                .collect();
            let gain = median(gains).unwrap();
            assert!((gain - expected).abs() < 1.0, "group {group}: {gain}");
        }
    }

    #[test]
    fn a_bass_kill_leaves_the_low_bands_unobservable_not_contradicted() {
        let parts = track('a');
        let reference = flat(&parts);
        let mix = render(&parts, [0.0, 1.0, 1.0]);

        let patches = judged(&mix, &reference);

        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert_eq!(band_count(&patches, BandState::Contradicted, 0..8), 0);
        assert!(band_count(&patches, BandState::Killed, 0..8) > 0);
        assert!(count(&patches, PatchState::Supported) as f64 >= 0.9 * patches.len() as f64);
    }

    /// The hi-hats go, but the high bands keep the other drums' leakage
    /// on the same beat, so they stay anchors and the high group's gain
    /// takes the cut, down to its bound 40 dB under the play's typical
    /// gain; the other groups keep theirs.
    #[test]
    fn a_high_cut_is_taken_up_by_the_high_groups_gain() {
        let parts = track('a');
        let reference = flat(&parts);
        let mix = render(&parts, [1.0, 1.0, 0.0]);

        let patches = judged(&mix, &reference);

        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert_eq!(band_count(&patches, BandState::Contradicted, 18..20), 0);
        for patch in &patches {
            let [low, mid, high] = patch.gains.map(Option::unwrap);
            assert!(low.abs() < 1.0 && mid.abs() < 1.0, "{low} {mid}");
            assert!((-40.5..-20.0).contains(&high), "{high}");
        }
    }

    #[test]
    fn a_mid_cut_leaves_the_mid_bands_unobservable_not_contradicted() {
        let parts = track('a');
        let reference = flat(&parts);
        let mix = render(&parts, [1.0, 0.0, 1.0]);

        let patches = judged(&mix, &reference);

        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert!(band_count(&patches, BandState::Killed, 8..18) > 0);
    }

    /// Experiment 0127's Astro-Sine plays: with the incoming record's bass
    /// cut, the outgoing record's bass fills the low bands at about the
    /// level the borrowed gain predicts, with its own dynamics. Version 1
    /// read those bands as contradicted; with no anchor in the low group
    /// they are masked or killed, and the patches stay supported by the
    /// mid and high groups.
    #[test]
    fn another_records_bass_under_a_bass_cut_is_masked_not_contradicted() {
        let parts = track('a');
        let reference = flat(&parts);
        let mut other_bass: Vec<(Part, Group)> = track('b')
            .into_iter()
            .filter(|(_, group)| *group == Group::Low)
            .collect();
        for (part, _) in &mut other_bass {
            for hit in &mut part.hits {
                *hit += 0.11;
            }
        }
        let cut = render(&parts, [0.0, 1.0, 1.0]);
        let mix: Vec<f32> = cut
            .iter()
            .zip(&flat(&other_bass))
            .map(|(a, b)| a + b)
            .collect();

        let patches = judged(&mix, &reference);

        assert_eq!(band_count(&patches, BandState::Contradicted, 0..8), 0);
        assert_eq!(count(&patches, PatchState::Contradicted), 0);
        assert!(
            band_count(&patches, BandState::Masked, 0..8)
                + band_count(&patches, BandState::Killed, 0..8)
                > 0
        );
        assert!(count(&patches, PatchState::Supported) as f64 >= 0.9 * patches.len() as f64);
        // Version 1's band rules on the same patches contradicted some.
        assert!(
            patches
                .iter()
                .any(|patch| patch.version_1_state == PatchState::Contradicted)
        );
    }

    #[test]
    fn a_missing_bassline_under_the_same_kick_is_contradicted() {
        let parts = track('a');
        let reference = flat(&parts);
        // The kick plays, so the low group is open; the bassline is gone,
        // which no EQ does.
        let without_bass: Vec<(Part, Group)> = track('a')
            .into_iter()
            .enumerate()
            .filter(|(i, _)| !(3..7).contains(i))
            .map(|(_, part)| part)
            .collect();
        let mix = flat(&without_bass);

        let patches = judged(&mix, &reference);

        // The lowest band's deficit has a shelf's shape and is taken for a
        // cut; the ones above the kick's band are notches.
        assert!(
            band_count(&patches, BandState::Contradicted, 0..8)
                > band_count(&patches, BandState::Killed, 0..8)
        );
    }

    /// Measured when the tests were written: 0.25, against 0.40 for
    /// another arrangement and 0 for the same recording. The partner's
    /// hi-hats, the same sound 110 ms off the beat, leave the high bands
    /// with no dynamics the 128 ms window can follow.
    #[test]
    fn a_blend_with_other_music_is_contradicted_less_than_another_arrangement() {
        let reference = flat(&track('a'));
        let mut other = track('b');
        // Another record: its own drums at other times, its own lines.
        for (part, _) in &mut other {
            for hit in &mut part.hits {
                *hit += 0.11;
            }
        }
        let partner = flat(&other);
        let mix: Vec<f32> = reference.iter().zip(&partner).map(|(a, b)| a + b).collect();

        let patches = judged(&mix, &reference);

        assert!(
            contradicted_share(&patches) <= 0.3,
            "{}",
            contradicted_share(&patches)
        );
        assert!(count(&patches, PatchState::Supported) > 0);
    }

    #[test]
    fn another_arrangement_over_the_same_drums_is_contradicted() {
        let played = flat(&track('a'));
        let related = flat(&track('b'));

        let patches = judged(&played, &related);

        assert!(
            contradicted_share(&patches) >= 0.35,
            "{}",
            contradicted_share(&patches)
        );
    }

    #[test]
    fn a_lag_of_two_frames_is_followed() {
        let reference = flat(&track('a'));
        let hop = Profile::CURRENT.hop;
        // The mix plays the reference two frames late.
        let mix: Vec<f32> = std::iter::repeat_n(0.0, 2 * hop)
            .chain(reference.iter().copied())
            .take(reference.len())
            .collect();

        let patches = judged(&mix, &reference);

        // The first patch cannot look two frames back before the start.
        let after_the_first = &patches[1..];
        assert!(after_the_first.iter().all(|patch| patch.lag == -2));
        assert_eq!(
            count(after_the_first, PatchState::Supported),
            after_the_first.len()
        );
    }

    #[test]
    fn patches_where_the_mix_is_silent_are_left_out() {
        let reference = flat(&track('a'));
        let mut mix = reference.clone();
        let half = mix.len() / 2;
        mix[half..].fill(0.0);

        let all = judged(&reference, &reference).len();
        let heard = judged(&mix, &reference);

        assert!(heard.len() < all / 2 + 2, "{} of {all}", heard.len());
        assert_eq!(count(&heard, PatchState::Supported), heard.len());
    }

    #[test]
    fn a_median_of_an_even_count_is_the_middle_pairs_mean() {
        assert_eq!(median(vec![3.0, 1.0, 2.0]), Some(2.0));
        assert_eq!(median(vec![4.0, 1.0, 3.0, 2.0]), Some(2.5));
        assert_eq!(median(Vec::new()), None);
    }
}

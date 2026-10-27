//! A gap matched against itself: method 2 of
//! `docs/research/2026-10-20-unidentified-gaps.md`, measured in experiments
//! 0162-0166, and the count of records it gives the CLI's gap lines (ADR
//! 0024). A drum & bass record repeats its own material, so the gap's pair
//! hashes, matched against the gap's own at speed 1.0, line up at lags
//! where one record plays again what it played before. Stretches of the
//! gap joined by such repeats form blocks; the long ones, merged where
//! they overlap, are counted as records.
//!
//! The gap is cut into cells of `CELL_SECONDS`. A repeat is a lag (in
//! frames, `LAG_STEP_FRAMES` wide) at which at least `min_hits` hashes of
//! one cell recur `MIN_LAG_SECONDS` or more later; it joins the two
//! cells. Blocks are the groups of joined cells (union-find); cells that
//! repeat nothing belong to no block. Hashes that occur more than
//! `MOST_OCCURRENCES` times in the gap are left out, as stop words.

use std::collections::BTreeMap;

use crate::hash::{PairHash, Point, for_each_pair};
use crate::parallel::map_in_order;
use crate::profile::Profile;
use crate::speed::{SpeedRatio, points_at_speed};

pub const CELL_SECONDS: f64 = 5.0;
/// Shorter lags are the record's own beat and the diagonal.
const MIN_LAG_SECONDS: f64 = 8.0;
const LAG_STEP_FRAMES: f64 = 2.0;
const MOST_OCCURRENCES: usize = 40;
/// The hits per cell and lag that join cells into a record's blocks, and
/// the shortest block taken as a record: the reading chosen on the
/// development gaps and fixed before experiment 0162's test.
pub const RECORD_HITS: u32 = 10;
pub const MIN_STRETCH_SECONDS: f64 = 30.0;

/// The blocks of one gap at one threshold.
#[derive(Debug, PartialEq)]
pub struct Blocks {
    pub min_hits: u32,
    /// Each block's first and last cell, in seconds from the gap's start.
    pub spans: Vec<(f64, f64)>,
    /// Cells in a block, of all cells.
    pub cells_in_blocks: usize,
    pub cells: usize,
}

/// The repeats found in one gap, from which blocks are read at any
/// threshold.
pub struct Repeats {
    cells: usize,
    cell_frames: f64,
    /// Hits per (cell, lag step), for each speed ratio of the repeat to its
    /// first hearing.
    by_ratio: Vec<(f64, Hits)>,
}

/// The records `samples` (one gap) holds by its repeats: the stretches of
/// its blocks at `RECORD_HITS`, at speed 1.0 only, in seconds from its
/// start.
pub fn records(samples: &[f32], profile: &Profile) -> Vec<(f64, f64)> {
    stretches(&Repeats::find(samples, profile, &[]).blocks(RECORD_HITS))
}

/// The records of each of `gaps` (start and end in seconds of `samples`,
/// which start at `start_seconds`), one gap per worker, as `records`
/// finds them, in the samples' time and in order.
pub fn records_in_gaps(
    samples: &[f32],
    start_seconds: f64,
    gaps: &[(f64, f64)],
    profile: &Profile,
    jobs: usize,
) -> Vec<(f64, f64)> {
    let rate = f64::from(profile.sample_rate);
    let sample = |seconds: f64| {
        (((seconds - start_seconds) * rate).round().max(0.0) as usize).min(samples.len())
    };
    map_in_order(gaps, jobs, |&(start, end)| {
        records(&samples[sample(start)..sample(end)], profile)
            .into_iter()
            .map(|(from, to)| (start + from, start + to))
            .collect::<Vec<_>>()
    })
    .into_iter()
    .flatten()
    .collect()
}

/// The blocks spanning `MIN_STRETCH_SECONDS` or more, those whose spans
/// overlap merged: one record's repeats can form several blocks that
/// overlap in time (0162).
pub fn stretches(blocks: &Blocks) -> Vec<(f64, f64)> {
    let mut long: Vec<(f64, f64)> = blocks
        .spans
        .iter()
        .copied()
        .filter(|(start, end)| end - start >= MIN_STRETCH_SECONDS)
        .collect();
    long.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1)));
    let mut merged: Vec<(f64, f64)> = Vec::new();
    for (start, end) in long {
        match merged.last_mut() {
            Some(last) if start < last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    merged
}

impl Repeats {
    /// The repeats of `samples` (one gap). Besides repeats at the same
    /// speed, each of `ratios` adds the repeats played that much faster
    /// than their first hearing (a pitch ride between them): the gap's
    /// hashes as if it played at the ratio, matched against its own at
    /// speed 1.0.
    pub fn find(samples: &[f32], profile: &Profile, ratios: &[f64]) -> Repeats {
        let points = points_at_speed(samples, profile, SpeedRatio(1.0));
        let cell_frames = profile.frames(CELL_SECONDS);
        let cells = (profile.frames(samples.len() as f64 / f64::from(profile.sample_rate))
            / cell_frames)
            .ceil() as usize;
        let mut by_ratio: Vec<(f64, Hits)> =
            vec![(1.0, repeats(&points, &points, 1.0, profile, cell_frames))];
        for &ratio in ratios.iter().filter(|&&ratio| ratio != 1.0) {
            let faster = points_at_speed(samples, profile, SpeedRatio(ratio));
            by_ratio.push((
                ratio,
                repeats(&points, &faster, ratio, profile, cell_frames),
            ));
        }
        Repeats {
            cells,
            cell_frames,
            by_ratio,
        }
    }

    /// The blocks that repeats of at least `min_hits` hits make.
    pub fn blocks(&self, min_hits: u32) -> Blocks {
        let joins: Vec<(usize, usize)> = self
            .by_ratio
            .iter()
            .flat_map(|(ratio, hits)| {
                hits.iter().filter(move |&(_, &hits)| hits >= min_hits).map(
                    move |(&(cell, lag), _)| {
                        (cell, later_cell(cell, lag, self.cell_frames, *ratio))
                    },
                )
            })
            .filter(|&(_, later)| later < self.cells)
            .collect();
        let groups = groups(self.cells, &joins);
        Blocks {
            min_hits,
            spans: groups
                .iter()
                .map(|group| {
                    let first = group.first().copied().unwrap_or(0);
                    let last = group.last().copied().unwrap_or(0);
                    (
                        first as f64 * CELL_SECONDS,
                        (last + 1) as f64 * CELL_SECONDS,
                    )
                })
                .collect(),
            cells_in_blocks: groups.iter().map(Vec::len).sum(),
            cells: self.cells,
        }
    }
}

/// Hits per cell of the earlier moment and lag step.
type Hits = BTreeMap<(usize, i64), u32>;

/// Hits per (cell of the earlier moment, lag step), for every pair of
/// occurrences of a hash at least `MIN_LAG_SECONDS` apart: the earlier in
/// `points` (the gap at speed 1.0), the later in `later` (the gap as if it
/// played at `ratio`, whose frames are `ratio` times the gap's). A repeat
/// played `ratio` times faster keeps one lag in these frames.
fn repeats(
    points: &[Point],
    later: &[Point],
    ratio: f64,
    profile: &Profile,
    cell_frames: f64,
) -> Hits {
    let occurrences = |points: &[Point]| {
        let mut occurrences: BTreeMap<PairHash, Vec<f64>> = BTreeMap::new();
        for_each_pair(points, |hash, anchor| {
            occurrences
                .entry(hash)
                .or_default()
                .push(points[anchor].frame);
        });
        occurrences
    };
    let earlier = occurrences(points);
    let later = occurrences(later);
    let min_lag = profile.frames(MIN_LAG_SECONDS);
    let mut hits: Hits = BTreeMap::new();
    for (hash, frames) in &earlier {
        let Some(laters) = later.get(hash) else {
            continue;
        };
        if frames.len() > MOST_OCCURRENCES || laters.len() > MOST_OCCURRENCES {
            continue;
        }
        for &earlier in frames {
            for &later in laters {
                if later / ratio - earlier >= min_lag {
                    let cell = (earlier / cell_frames) as usize;
                    *hits
                        .entry((cell, ((later - earlier) / LAG_STEP_FRAMES).round() as i64))
                        .or_default() += 1;
                }
            }
        }
    }
    hits
}

/// The cell `lag` reaches from the middle of `cell`, for a repeat played
/// `ratio` times faster: hits near a cell's edge may join the cell next to
/// the one their own moments reach.
fn later_cell(cell: usize, lag: i64, cell_frames: f64, ratio: f64) -> usize {
    let earlier = (cell as f64 + 0.5) * cell_frames;
    ((earlier + lag as f64 * LAG_STEP_FRAMES) / ratio / cell_frames) as usize
}

/// The groups of cells `joins` connects, each in order, in order of their
/// first cell; a cell no join reaches is in none.
fn groups(cells: usize, joins: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut parent: Vec<usize> = (0..cells).collect();
    let mut joined = vec![false; cells];
    for &(a, b) in joins {
        joined[a] = true;
        joined[b] = true;
        let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
        if ra != rb {
            parent[ra.max(rb)] = ra.min(rb);
        }
    }
    let mut by_root: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for cell in (0..cells).filter(|&cell| joined[cell]) {
        let root = root(&mut parent, cell);
        by_root.entry(root).or_default().push(cell);
    }
    let mut groups: Vec<Vec<usize>> = by_root.into_values().collect();
    groups.sort_by_key(|group| group[0]);
    groups
}

/// The group `cell` belongs to, halving the path to it on the way.
fn root(parent: &mut [usize], cell: usize) -> usize {
    let mut cell = cell;
    while parent[cell] != cell {
        parent[cell] = parent[parent[cell]];
        cell = parent[cell];
    }
    cell
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn joined_cells_form_groups_and_lone_cells_none() {
        let groups = groups(8, &[(0, 3), (3, 1), (5, 7)]);

        assert_eq!(groups, [vec![0, 1, 3], vec![5, 7]]);
    }

    #[test]
    fn stretches_are_long_blocks_with_overlapping_ones_merged() {
        let blocks = Blocks {
            min_hits: RECORD_HITS,
            spans: vec![
                (0.0, 40.0),
                (20.0, 70.0),
                (70.0, 100.0),
                (100.0, 125.0),
                (130.0, 160.0),
            ],
            cells_in_blocks: 0,
            cells: 32,
        };

        // 100-125 s is too short; 70-100 s only touches 20-70 s.
        assert_eq!(
            stretches(&blocks),
            [(0.0, 70.0), (70.0, 100.0), (130.0, 160.0)]
        );
    }

    #[test]
    fn a_lag_reaches_the_cell_its_middle_lands_in() {
        // Cells of 100 frames; a lag of 150 steps of 2 frames from cell 1.
        assert_eq!(later_cell(1, 150, 100.0, 1.0), 4);
    }

    fn beeps(seconds: f64, speed: f64, rate: f64) -> Vec<f32> {
        (0..(seconds * rate) as usize)
            .map(|n| {
                let t = n as f64 / rate * speed;
                let pitch = 300.0 + 150.0 * ((t * 3.0).floor() % 7.0);
                ((2.0 * std::f64::consts::PI * pitch * t).sin()
                    * if (t * 4.0).fract() < 0.5 { 0.5 } else { 0.0 }) as f32
            })
            .collect()
    }

    #[test]
    fn a_passage_played_again_faster_joins_only_at_its_ratio() {
        let profile = Profile::CURRENT;
        let rate = f64::from(profile.sample_rate);
        let silence = vec![0.0_f32; (12.0 * rate) as usize];
        // The beeps, a pause, then the same beeps 3% faster.
        let samples: Vec<f32> = [
            beeps(12.0, 1.0, rate),
            silence,
            beeps(12.0 / 1.03, 1.03, rate),
        ]
        .concat();

        let alone = Repeats::find(&samples, &profile, &[]).blocks(5);
        let at_ratio = Repeats::find(&samples, &profile, &[1.03]).blocks(5);

        assert!(alone.spans.is_empty(), "{alone:?}");
        assert_eq!(at_ratio.spans.len(), 1, "{at_ratio:?}");
        assert!(at_ratio.spans[0].1 > 24.0, "{at_ratio:?}");
    }

    #[test]
    fn a_passage_played_twice_is_one_block_and_noise_is_none() {
        let profile = Profile::CURRENT;
        let rate = f64::from(profile.sample_rate);
        // 12 s of a chirp of beeps, then 12 s of noise, then the beeps again.
        let beeps: Vec<f32> = (0..(12.0 * rate) as usize)
            .map(|n| {
                let t = n as f64 / rate;
                let pitch = 300.0 + 150.0 * ((t * 3.0).floor() % 7.0);
                ((2.0 * std::f64::consts::PI * pitch * t).sin()
                    * if (t * 4.0).fract() < 0.5 { 0.5 } else { 0.0 }) as f32
            })
            .collect();
        let mut state: u32 = 1;
        let noise: Vec<f32> = (0..(12.0 * rate) as usize)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (f64::from(state >> 8) / f64::from(1u32 << 24) - 0.5) as f32 * 0.05
            })
            .collect();
        let samples: Vec<f32> = [beeps.clone(), noise, beeps].concat();

        let at_five = Repeats::find(&samples, &profile, &[]).blocks(5);

        assert_eq!(at_five.min_hits, 5);
        assert_eq!(at_five.spans.len(), 1, "{at_five:?}");
        assert!(
            at_five.spans[0].0 < 12.0 && at_five.spans[0].1 > 24.0,
            "{at_five:?}"
        );
        // The block spans more than 30 s at the records' threshold too.
        assert_eq!(records(&samples, &profile).len(), 1);
    }
}

//! The gap against itself: a measured prototype of method 2 of step 2 in
//! `docs/research/2026-10-20-unidentified-gaps.md`. A drum & bass record
//! repeats its own material, so the gap's pair hashes, matched against the
//! gap's own at speed 1.0, line up at lags where one record plays again
//! what it played before. Stretches of the gap joined by such repeats form
//! blocks; the number of blocks is a count of the records in it.
//!
//! The gap is cut into cells of `CELL_SECONDS`. A repeat is a lag (in
//! frames, `LAG_STEP_FRAMES` wide) at which at least `min_hits` hashes of
//! one cell recur `MIN_LAG_SECONDS` or more later; it joins the two
//! cells. Blocks are the groups of joined cells (union-find); cells that
//! repeat nothing belong to no block. Hashes that occur more than
//! `MOST_OCCURRENCES` times in the gap are left out, as stop words.

use std::collections::BTreeMap;

use gunfinger_core::hash::{PairHash, Point, for_each_pair};
use gunfinger_core::profile::Profile;
use gunfinger_core::speed::{SpeedRatio, points_at_speed};
use serde::Serialize;

pub const CELL_SECONDS: f64 = 5.0;
/// Shorter lags are the record's own beat and the diagonal.
const MIN_LAG_SECONDS: f64 = 8.0;
const LAG_STEP_FRAMES: f64 = 2.0;
const MOST_OCCURRENCES: usize = 40;
/// The thresholds compared, in hits per cell and lag.
pub const MIN_HITS: [u32; 4] = [5, 10, 20, 40];

/// The blocks of one gap at each threshold.
#[derive(Debug, Serialize, PartialEq)]
pub struct Blocks {
    pub min_hits: u32,
    /// Each block's first and last cell, in seconds from the gap's start.
    pub spans: Vec<(f64, f64)>,
    /// Cells in a block, of all cells.
    pub cells_in_blocks: usize,
    pub cells: usize,
}

/// The repeats of `samples` (one gap) and the blocks they make at each of
/// `MIN_HITS`.
pub fn blocks(samples: &[f32], profile: &Profile) -> Vec<Blocks> {
    let points = points_at_speed(samples, profile, SpeedRatio(1.0));
    let cell_frames = profile.frames(CELL_SECONDS);
    let cells = (profile.frames(samples.len() as f64 / f64::from(profile.sample_rate))
        / cell_frames)
        .ceil() as usize;
    let repeats = repeats(&points, profile, cell_frames);
    MIN_HITS
        .iter()
        .map(|&min_hits| {
            let joins: Vec<(usize, usize)> = repeats
                .iter()
                .filter(|&(_, &hits)| hits >= min_hits)
                .map(|(&(cell, lag), _)| (cell, later_cell(cell, lag, cell_frames)))
                .filter(|&(_, later)| later < cells)
                .collect();
            let groups = groups(cells, &joins);
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
                cells,
            }
        })
        .collect()
}

/// Hits per (cell of the earlier moment, lag step), for every pair of
/// occurrences of a hash at least `MIN_LAG_SECONDS` apart.
fn repeats(points: &[Point], profile: &Profile, cell_frames: f64) -> BTreeMap<(usize, i64), u32> {
    let mut occurrences: BTreeMap<PairHash, Vec<f64>> = BTreeMap::new();
    for_each_pair(points, |hash, anchor| {
        occurrences
            .entry(hash)
            .or_default()
            .push(points[anchor].frame);
    });
    let min_lag = profile.frames(MIN_LAG_SECONDS);
    let mut hits: BTreeMap<(usize, i64), u32> = BTreeMap::new();
    for frames in occurrences.values() {
        if frames.len() > MOST_OCCURRENCES {
            continue;
        }
        for (place, &earlier) in frames.iter().enumerate() {
            for &later in &frames[place + 1..] {
                let lag = later - earlier;
                if lag >= min_lag {
                    let cell = (earlier / cell_frames) as usize;
                    *hits
                        .entry((cell, (lag / LAG_STEP_FRAMES).round() as i64))
                        .or_default() += 1;
                }
            }
        }
    }
    hits
}

/// The cell `lag` reaches from the middle of `cell`: hits near a cell's edge
/// may join the cell next to the one their own moments reach.
fn later_cell(cell: usize, lag: i64, cell_frames: f64) -> usize {
    let earlier = (cell as f64 + 0.5) * cell_frames;
    ((earlier + lag as f64 * LAG_STEP_FRAMES) / cell_frames) as usize
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
    fn a_lag_reaches_the_cell_its_middle_lands_in() {
        // Cells of 100 frames; a lag of 150 steps of 2 frames from cell 1.
        assert_eq!(later_cell(1, 150, 100.0), 4);
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

        let found = blocks(&samples, &profile);

        let at_five = &found[0];
        assert_eq!(at_five.min_hits, 5);
        assert_eq!(at_five.spans.len(), 1, "{at_five:?}");
        assert!(
            at_five.spans[0].0 < 12.0 && at_five.spans[0].1 > 24.0,
            "{at_five:?}"
        );
    }
}

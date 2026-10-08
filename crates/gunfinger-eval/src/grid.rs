//! The window grid: does the same brief play change level only because of
//! where the 10 s windows fall on it?
//!
//! A query is held-out audio, then a brief play of an indexed track, then
//! held-out audio again, joined by cuts. The lead-in grows by one second
//! per offset, so the brief play slides across the window grid in 1 s
//! steps while every sample of it, and the context around it, stays the
//! same. Lengths and source positions vary too.
//!
//! Rules are compared offline on the same detections: the frozen rule (200
//! hits in 3 windows) and a minimum aligned span in seconds instead of 3
//! windows, which does not depend on the grid.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::Confidence;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::mixes::{
    MixFound, PlannedMix, PlannedPlay, Pools, Searcher, draw_unused, search_mix, sweep_index,
};
use crate::rng::Rng;
use crate::scoring::evidence;
use crate::verifier::Verifier;

const LENGTHS: [f64; 6] = [10.0, 15.0, 20.0, 25.0, 30.0, 40.0];
/// Offsets of the brief play from the window grid, in seconds.
const OFFSETS: u32 = 10;
/// Indexed tracks, and source positions in each.
const TRACKS: usize = 8;
const POSITIONS: usize = 2;
/// Held-out audio before the brief play (plus the offset) and after it.
const LEAD_IN_SECONDS: f64 = 30.0;
const TAIL_SECONDS: f64 = 20.0;
const SPEED_RANGE: f64 = 0.06;
/// Minimum aligned spans tried in place of 3 windows, in seconds.
pub const SPANS: [f64; 3] = [10.0, 15.0, 20.0];

/// One brief play to slide: a track, a place in it, a speed, and the
/// held-out tracks around it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brief {
    pub asset: String,
    pub source_start_seconds: f64,
    pub speed: f64,
    pub before: String,
    pub before_ends_at_seconds: f64,
    pub after: String,
    pub after_starts_at_seconds: f64,
}

#[derive(Serialize, Deserialize)]
pub struct GridReport {
    pub seed: u64,
    pub ladder: String,
    pub indexed_assets: usize,
    pub briefs: Vec<Brief>,
    pub queries: Vec<GridQuery>,
}

#[derive(Serialize, Deserialize)]
pub struct GridQuery {
    /// Index into `briefs`.
    pub brief: usize,
    pub seconds: f64,
    pub offset_seconds: u32,
    /// The brief play's level under the frozen rule: `confident`,
    /// `possible`, `weak` or `none`.
    pub level: String,
    /// Detections of the brief play's cluster that overlap it.
    pub matching: Vec<MixFound>,
    /// Confident detections that match no play.
    pub wrong_confident: usize,
    pub strongest_false_hits: u32,
    /// With `--verify`: every detection that matches no play.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub false_candidates: Vec<MixFound>,
}

pub struct Options<'a> {
    pub seed: u64,
    /// Where the seed's sweep panel is kept (`Plan::for_seed`).
    pub panels: &'a Path,
    pub ladder_name: &'a str,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// Measure every detection with the peak verifier.
    pub verify: bool,
    pub jobs: usize,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<GridReport, String> {
    let (records, held_out, index) = sweep_index(
        library,
        store,
        clusters,
        options.seed,
        options.panels,
        options.matching,
    )?;
    let verifier = options.verify.then(|| Verifier::new(&records, None));
    let pools = Pools::new(&records, &held_out);
    let briefs = draw_briefs(&pools, clusters, &mut Rng::new(options.seed))
        .ok_or("the library has too few long tracks for the grid")?;
    let dir = work.join("grid").join(format!("seed-{}", options.seed));
    let mut planned: Vec<(usize, f64, u32, PlannedMix, PathBuf)> = Vec::new();
    for (number, brief) in briefs.iter().enumerate() {
        for seconds in LENGTHS {
            for offset in 0..OFFSETS {
                let name = format!("{number:02}-{seconds:02.0}s-{offset}");
                let path = dir.join(format!("{name}.mp3"));
                let mix = query(&name, brief, seconds, offset);
                planned.push((number, seconds, offset, mix, path));
            }
        }
    }
    let searcher = Searcher {
        index: &index,
        ladder: options.ladder,
        matching: options.matching,
        verifier: verifier.as_ref(),
    };
    let queries = map_in_order(
        &planned,
        options.jobs,
        |(number, seconds, offset, mix, path)| {
            let result = search_mix(mix, path, library, clusters, &searcher)?;
            let brief = &result.score.plays[1];
            let accepted = clusters.cluster_of(&brief.truth.asset);
            Ok(GridQuery {
                brief: *number,
                seconds: *seconds,
                offset_seconds: *offset,
                level: brief.level.clone(),
                matching: result
                    .detections
                    .iter()
                    .filter(|found| {
                        accepted.contains(&found.asset)
                            && found.start_seconds < brief.truth.end_seconds
                            && brief.truth.start_seconds < found.end_seconds
                    })
                    .cloned()
                    .collect(),
                wrong_confident: result.score.wrong_confident.len(),
                strongest_false_hits: result
                    .score
                    .false_candidates
                    .first()
                    .map_or(0, |found| found.hits),
                false_candidates: if verifier.is_some() {
                    result.score.false_candidates.clone()
                } else {
                    Vec::new()
                },
            })
        },
    );
    Ok(GridReport {
        seed: options.seed,
        ladder: options.ladder_name.to_owned(),
        indexed_assets: index.assets().len(),
        briefs,
        queries: queries
            .into_iter()
            .collect::<Result<Vec<GridQuery>, String>>()?,
    })
}

/// `TRACKS` indexed tracks from distinct clusters at `POSITIONS` places
/// each, every one between two different held-out tracks.
fn draw_briefs(pools: &Pools, clusters: &Clusters, rng: &mut Rng) -> Option<Vec<Brief>> {
    let mut tracks_used = BTreeSet::new();
    let mut briefs = Vec::new();
    for _ in 0..TRACKS {
        let track = draw_unused(&pools.indexed, &mut tracks_used, clusters, rng)?;
        for _ in 0..POSITIONS {
            let mut context_used = BTreeSet::new();
            let before = draw_unused(&pools.held_out, &mut context_used, clusters, rng)?;
            let after = draw_unused(&pools.held_out, &mut context_used, clusters, rng)?;
            let speed = 1.0 + (2.0 * rng.unit() - 1.0) * SPEED_RANGE;
            let longest = LENGTHS[LENGTHS.len() - 1] * speed;
            let in_track = |record: &PeakRecord, needed: f64, rng: &mut Rng| {
                let duration = record.header.duration_seconds;
                let earliest = 0.15 * duration;
                let latest = 0.85 * duration - needed;
                earliest + rng.unit() * (latest - earliest).max(0.0)
            };
            let lead_in = LEAD_IN_SECONDS + f64::from(OFFSETS);
            briefs.push(Brief {
                asset: track.header.source.path.clone(),
                source_start_seconds: in_track(track, longest, rng),
                speed,
                before: before.header.source.path.clone(),
                before_ends_at_seconds: in_track(before, lead_in, rng) + lead_in,
                after: after.header.source.path.clone(),
                after_starts_at_seconds: in_track(after, TAIL_SECONDS, rng),
            });
        }
    }
    Some(briefs)
}

/// The query for `brief` played for `seconds`, starting `offset` seconds
/// after a window boundary. The lead-in always ends at the same place in
/// its track, so only the grid moves.
fn query(name: &str, brief: &Brief, seconds: f64, offset: u32) -> PlannedMix {
    let lead_in = LEAD_IN_SECONDS + f64::from(offset);
    let cut = |asset: &str, source_start_seconds, speed, start_seconds, end_seconds| PlannedPlay {
        asset: asset.to_owned(),
        held_out: true,
        source_start_seconds,
        speed,
        start_seconds,
        end_seconds,
        fade_in_seconds: 0.0,
        fade_out_seconds: 0.0,
        bass_cut_until: None,
        bass_cut_from: None,
    };
    let end = lead_in + seconds;
    PlannedMix {
        name: name.to_owned(),
        seconds: end + TAIL_SECONDS,
        plays: vec![
            cut(
                &brief.before,
                brief.before_ends_at_seconds - lead_in,
                1.0,
                0.0,
                lead_in,
            ),
            PlannedPlay {
                held_out: false,
                ..cut(
                    &brief.asset,
                    brief.source_start_seconds,
                    brief.speed,
                    lead_in,
                    end,
                )
            },
            cut(
                &brief.after,
                brief.after_starts_at_seconds,
                1.0,
                end,
                end + TAIL_SECONDS,
            ),
        ],
    }
}

/// Whether any matching detection is confident when `min_span` seconds
/// between its first and last hit replace the 3-window minimum; `None` is
/// the frozen rule.
pub fn confident_under(matching: &[MixFound], min_span: Option<f64>) -> bool {
    matching.iter().any(|found| match min_span {
        None => found.confidence() == Confidence::Confident,
        Some(span) => {
            found.hits
                >= evidence(found.windows, found.hits, found.fitted)
                    .pass
                    .rule()
                    .min_hits
                && found.end_seconds - found.start_seconds >= span
        }
    })
}

pub fn print_summary(report: &GridReport) {
    println!(
        "window grid, seed {}, {} ladder: {} brief plays slid over {OFFSETS} offsets",
        report.seed,
        report.ladder,
        report.briefs.len()
    );
    let wrong: usize = report
        .queries
        .iter()
        .map(|query| query.wrong_confident)
        .sum();
    let strongest_false = report
        .queries
        .iter()
        .map(|query| query.strongest_false_hits)
        .max()
        .unwrap_or(0);
    println!("wrong confident: {wrong}; strongest false candidate {strongest_false} hits");
    let rules: Vec<(String, Option<f64>)> = std::iter::once((String::from("3 windows"), None))
        .chain(
            SPANS
                .iter()
                .map(|&span| (format!("span {span:.0} s"), Some(span))),
        )
        .collect();
    print!("{:>7} {:>10}", "length", "possible+");
    for (name, _) in &rules {
        print!(" {:>22}", format!("confident ({name})"));
    }
    println!();
    for seconds in LENGTHS {
        let here: Vec<&GridQuery> = report
            .queries
            .iter()
            .filter(|query| query.seconds == seconds)
            .collect();
        let possible = here
            .iter()
            .filter(|query| matches!(query.level.as_str(), "possible" | "confident"))
            .count();
        print!("{seconds:>6.0}s {possible:>5}/{:<4}", here.len());
        for (_, rule) in &rules {
            let confident = here
                .iter()
                .filter(|query| confident_under(&query.matching, *rule))
                .count();
            // Brief plays whose verdict changes with the offset alone.
            let flipping = (0..report.briefs.len())
                .filter(|&brief| {
                    let verdicts: BTreeSet<bool> = here
                        .iter()
                        .filter(|query| query.brief == brief)
                        .map(|query| confident_under(&query.matching, *rule))
                        .collect();
                    verdicts.len() > 1
                })
                .count();
            print!(
                " {:>22}",
                format!("{confident}/{}, {flipping} flip", here.len())
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brief() -> Brief {
        Brief {
            asset: String::from("a.mp3"),
            source_start_seconds: 60.0,
            speed: 1.03,
            before: String::from("b.mp3"),
            before_ends_at_seconds: 100.0,
            after: String::from("c.mp3"),
            after_starts_at_seconds: 30.0,
        }
    }

    #[test]
    fn sliding_moves_the_brief_play_on_the_grid_and_nothing_else() {
        let first = query("q", &brief(), 20.0, 0);
        let slid = query("q", &brief(), 20.0, 7);

        let played = |mix: &PlannedMix| {
            let play = &mix.plays[1];
            (
                play.source_start_seconds,
                play.end_seconds - play.start_seconds,
            )
        };
        assert_eq!(played(&first), played(&slid));
        assert_eq!(
            slid.plays[1].start_seconds - first.plays[1].start_seconds,
            7.0
        );
        for mix in [&first, &slid] {
            let lead_in = &mix.plays[0];
            assert!((lead_in.source_at(lead_in.end_seconds) - 100.0).abs() < 1e-9);
            assert_eq!(mix.plays[2].start_seconds, mix.plays[1].end_seconds);
        }
    }

    #[test]
    fn a_span_rule_ignores_the_window_count() {
        let found = MixFound {
            asset: String::from("a.mp3"),
            start_seconds: 31.0,
            end_seconds: 47.0,
            track_start_seconds: 0.0,
            speed: 1.0,
            windows: 2,
            hits: 450,
            fitted: false,
            verified: None,
        };

        assert!(!confident_under(std::slice::from_ref(&found), None));
        assert!(confident_under(std::slice::from_ref(&found), Some(15.0)));
        assert!(!confident_under(std::slice::from_ref(&found), Some(20.0)));
    }
}

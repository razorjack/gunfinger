//! Gaps: the stretches of a mix that no confident segment covers (step 1
//! of `docs/research/2026-10-20-unidentified-gaps.md`), how many tracks
//! each holds by its length (method 1 of step 2), and both against the
//! truth, as `docs/research/2026-10-21-gap-measures.md` defines them. It
//! reads saved reports and searches nothing.
//!
//! Covered time is the segments of confident plays, grouped as `seconds`
//! groups them (ADR 0013), so a gap between two segments of a returning
//! track counts. Possible plays cover nothing (ADR 0006); each gap counts
//! the ones it meets. The stretches, the entries, their clean intervals
//! and the median are the core's (`gunfinger_core::gaps`), which the CLI
//! prints from (ADR 0023, ADR 0024).

use gunfinger_core::confidence::Confidence;
use gunfinger_core::gaps::{MIN_INTERVALS, clean_intervals, median, uncovered};
use serde::Serialize;

use crate::seconds::{Label, Rule, Segment, Truth, labels};

/// The minimum lengths compared (the pre-registration's).
pub const MIN_LENGTHS: [f64; 6] = [10.0, 20.0, 30.0, 45.0, 60.0, 90.0];
/// A gap holds a track when it covers this much of it, or half of a
/// shorter track.
const HOLDS_SECONDS: f64 = 30.0;

/// The gaps of one mix at each minimum length, against its truth.
#[derive(Debug, Serialize)]
pub struct Listing {
    pub name: String,
    pub duration_seconds: f64,
    pub tracks: Vec<TrackTruth>,
    /// Where the entries of the tracklist start: confident plays, those
    /// over the same passage taken as one.
    pub entry_starts: Vec<f64>,
    pub at: Vec<AtLength>,
}

/// A track of the truth, numbered from 1 in its order.
#[derive(Debug, Serialize)]
pub struct TrackTruth {
    pub number: usize,
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Not in the index: left out, held out or absent.
    pub missed: bool,
    /// The seconds in which it plays alone, outside every transition.
    pub own_seconds: u32,
}

/// The gaps of at least `min_length_seconds`, and method 1's interval.
#[derive(Debug, Serialize)]
pub struct AtLength {
    pub min_length_seconds: f64,
    /// The median of `intervals`, when there are enough of them.
    pub median_interval_seconds: Option<f64>,
    /// The start-to-start intervals between consecutive entries that hold
    /// no listed gap.
    pub intervals: Vec<f64>,
    pub gaps: Vec<Gap>,
}

#[derive(Debug, Serialize)]
pub struct Gap {
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// Its seconds by the truth: a missed track playing alone, an indexed
    /// one, or a transition.
    pub missed_seconds: u32,
    pub indexed_seconds: u32,
    pub transition_seconds: u32,
    /// The numbers of the tracks it holds.
    pub missed_tracks: Vec<usize>,
    pub indexed_tracks: Vec<usize>,
    /// Possible plays that overlap it.
    pub possible_plays: usize,
    /// Method 1: its length over the median interval, rounded, at least 1.
    pub estimate: Option<u32>,
}

impl Gap {
    pub fn length_seconds(&self) -> f64 {
        self.end_seconds - self.start_seconds
    }

    /// The estimate's range: one fewer to one more, at least 1.
    pub fn range(&self) -> Option<(u32, u32)> {
        self.estimate
            .map(|estimate| (estimate.saturating_sub(1).max(1), estimate + 1))
    }

    fn overlaps(&self, start: f64, end: f64) -> bool {
        start < self.end_seconds && self.start_seconds < end
    }
}

/// What the truth says of one second.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Heard {
    /// The track of this number plays alone.
    Alone(usize),
    /// Two tracks play, within `margin` of a border, or none does.
    Transition,
}

/// Lists the gaps of `segments` over `duration_seconds` at each of
/// `min_lengths`, against `truth`. Within `margin` seconds of a border
/// both tracks are taken to play (a manifest's approximate starts);
/// exact truth uses 0.
pub fn list(
    name: &str,
    truth: &[Truth],
    segments: &[Segment],
    duration_seconds: f64,
    margin: f64,
    min_lengths: &[f64],
) -> Listing {
    let labels = labels(segments, Rule::Windows);
    let heard: Vec<(f64, Heard)> = whole_seconds(duration_seconds)
        .map(|second| (second, heard(truth, margin, second)))
        .collect();
    let tracks = truth
        .iter()
        .zip(1..)
        .map(|(track, number)| TrackTruth {
            number,
            start_seconds: track.start_seconds,
            end_seconds: track.end_seconds,
            missed: !track.in_library,
            own_seconds: count(&heard, |_, at| at == Heard::Alone(number)),
        })
        .collect();
    let covered = covered(&labels);
    let entry_starts = entry_starts(&labels);
    let at = min_lengths
        .iter()
        .map(|&min_length| {
            let mut gaps: Vec<Gap> = uncovered(&covered, 0.0, duration_seconds, min_length)
                .into_iter()
                .map(|(start, end)| gap(start, end, truth, &heard, &labels))
                .collect();
            let intervals = intervals_without_gaps(&entry_starts, &gaps);
            let median = (intervals.len() >= MIN_INTERVALS).then(|| median(&intervals));
            for gap in &mut gaps {
                gap.estimate = median.map(|median| estimate(gap.length_seconds(), median));
            }
            AtLength {
                min_length_seconds: min_length,
                median_interval_seconds: median,
                intervals,
                gaps,
            }
        })
        .collect();
    Listing {
        name: name.to_owned(),
        duration_seconds,
        tracks,
        entry_starts,
        at,
    }
}

/// The middle of each whole second, as `seconds` judges them.
fn whole_seconds(duration_seconds: f64) -> impl Iterator<Item = f64> {
    (0..)
        .map(|second| f64::from(second) + 0.5)
        .take_while(move |&second| second < duration_seconds)
}

fn heard(truth: &[Truth], margin: f64, second: f64) -> Heard {
    let mut playing = truth
        .iter()
        .zip(1..)
        .filter(|(track, _)| {
            track.start_seconds - margin <= second && second < track.end_seconds + margin
        })
        .map(|(_, number)| number);
    match (playing.next(), playing.next()) {
        (Some(number), None) => Heard::Alone(number),
        _ => Heard::Transition,
    }
}

fn count(heard: &[(f64, Heard)], keep: impl Fn(f64, Heard) -> bool) -> u32 {
    heard
        .iter()
        .filter(|&&(second, at)| keep(second, at))
        .count() as u32
}

/// The segments of confident plays: the time they cover.
fn covered(labels: &[Label]) -> Vec<(f64, f64)> {
    labels
        .iter()
        .filter(|label| label.level == Confidence::Confident)
        .flat_map(|label| label.spans.iter().copied())
        .collect()
}

/// The gap from `start` to `end`, judged by the truth.
fn gap(start: f64, end: f64, truth: &[Truth], heard: &[(f64, Heard)], labels: &[Label]) -> Gap {
    let inside = |second: f64| start <= second && second < end;
    let mut gap = Gap {
        start_seconds: start,
        end_seconds: end,
        missed_seconds: count(heard, |second, at| {
            inside(second) && matches!(at, Heard::Alone(number) if !truth[number - 1].in_library)
        }),
        indexed_seconds: count(heard, |second, at| {
            inside(second) && matches!(at, Heard::Alone(number) if truth[number - 1].in_library)
        }),
        transition_seconds: count(heard, |second, at| {
            inside(second) && at == Heard::Transition
        }),
        missed_tracks: Vec::new(),
        indexed_tracks: Vec::new(),
        possible_plays: 0,
        estimate: None,
    };
    for (track, number) in truth.iter().zip(1..) {
        if holds(&gap, track) {
            if track.in_library {
                gap.indexed_tracks.push(number);
            } else {
                gap.missed_tracks.push(number);
            }
        }
    }
    gap.possible_plays = labels
        .iter()
        .filter(|label| label.level == Confidence::Possible)
        .filter(|label| {
            label
                .spans
                .iter()
                .any(|&(start, end)| gap.overlaps(start, end))
        })
        .count();
    gap
}

/// A confident span ends where a gap starts, or starts where it ends,
/// within this much.
const NEIGHBOUR_SECONDS: f64 = 0.5;

/// The files of the confident plays whose segments end where `gap` starts
/// or start where it ends: its neighbours in the tracklist.
pub fn neighbours(segments: &[Segment], gap: &Gap) -> Vec<String> {
    let mut assets: Vec<String> = labels(segments, Rule::Windows)
        .iter()
        .filter(|label| label.level == Confidence::Confident)
        .filter(|label| {
            label.spans.iter().any(|&(start, end)| {
                (end - gap.start_seconds).abs() <= NEIGHBOUR_SECONDS
                    || (start - gap.end_seconds).abs() <= NEIGHBOUR_SECONDS
            })
        })
        .map(|label| label.asset.to_owned())
        .collect();
    assets.sort();
    assets.dedup();
    assets
}

/// Whether `gap` covers at least `HOLDS_SECONDS` of `track` or half of
/// it, whichever is less, or lies in it for at least half its length.
fn holds(gap: &Gap, track: &Truth) -> bool {
    let overlap =
        gap.end_seconds.min(track.end_seconds) - gap.start_seconds.max(track.start_seconds);
    let span = track.end_seconds - track.start_seconds;
    overlap > 0.0
        && (overlap >= HOLDS_SECONDS.min(span / 2.0) || overlap >= gap.length_seconds() / 2.0)
}

/// Where the tracklist's entries start: the confident plays in order,
/// one that overlaps the entry before it for most of the shorter of the
/// two joining that entry (`gunfinger_core::gaps::entry_starts`).
fn entry_starts(labels: &[Label]) -> Vec<f64> {
    let plays: Vec<(f64, f64)> = labels
        .iter()
        .filter(|label| label.level == Confidence::Confident)
        .filter_map(|label| {
            let start = label.spans.iter().map(|span| span.0).reduce(f64::min)?;
            let end = label.spans.iter().map(|span| span.1).reduce(f64::max)?;
            Some((start, end))
        })
        .collect();
    gunfinger_core::gaps::entry_starts(&plays)
}

/// The intervals between consecutive entry starts that no listed gap
/// overlaps: those hold time the tracklist does not account for.
fn intervals_without_gaps(entry_starts: &[f64], gaps: &[Gap]) -> Vec<f64> {
    let spans: Vec<(f64, f64)> = gaps
        .iter()
        .map(|gap| (gap.start_seconds, gap.end_seconds))
        .collect();
    clean_intervals(entry_starts, &spans)
}

fn estimate(length_seconds: f64, interval_seconds: f64) -> u32 {
    ((length_seconds / interval_seconds).round() as u32).max(1)
}

/// The counts `print_summary` shows for one minimum length, over listings.
#[derive(Debug, Default, PartialEq)]
struct Tally {
    gaps: usize,
    without_missed: usize,
    missed_tracks: usize,
    reached: usize,
    missed_seconds: u32,
    missed_seconds_in_gaps: u32,
    /// By the number of missed tracks held, 0 to 5 or more: gaps, exact
    /// estimates, estimates off by one, ranges that miss, no estimate.
    by_truth: [[usize; 5]; 6],
}

fn tally(listings: &[&Listing], min_length: f64) -> Tally {
    let mut tally = Tally::default();
    for listing in listings {
        let Some(at) = listing
            .at
            .iter()
            .find(|at| at.min_length_seconds == min_length)
        else {
            continue;
        };
        let missed: Vec<&TrackTruth> = listing.tracks.iter().filter(|track| track.missed).collect();
        tally.missed_tracks += missed.len();
        tally.reached += missed
            .iter()
            .filter(|track| {
                at.gaps
                    .iter()
                    .any(|gap| gap.missed_tracks.contains(&track.number))
            })
            .count();
        tally.missed_seconds += missed.iter().map(|track| track.own_seconds).sum::<u32>();
        for gap in &at.gaps {
            tally.gaps += 1;
            tally.missed_seconds_in_gaps += gap.missed_seconds;
            let truth = gap.missed_tracks.len();
            if truth == 0 {
                tally.without_missed += 1;
            }
            let row = &mut tally.by_truth[truth.min(5)];
            row[0] += 1;
            match (gap.estimate, gap.range()) {
                (Some(estimate), Some((low, high))) => {
                    let error = i64::from(estimate) - truth as i64;
                    row[1] += usize::from(error == 0);
                    row[2] += usize::from(error.abs() == 1);
                    row[3] += usize::from(!(low as usize..=high as usize).contains(&truth));
                }
                _ => row[4] += 1,
            }
        }
    }
    tally
}

/// Prints, for each minimum length, the gaps of `listings` against the
/// truth, every listing counted as it is (repeated gaps included).
pub fn print_summary(title: &str, listings: &[&Listing], min_lengths: &[f64]) {
    println!("{title}");
    let share = |count: u32, of: u32| 100.0 * f64::from(count) / f64::from(of.max(1));
    for &min_length in min_lengths {
        let tally = tally(listings, min_length);
        println!(
            "  at least {min_length:>2.0} s: {} gaps, {} holding no missed track; missed tracks reached {}/{}; missed seconds in gaps {:.1}% of {}",
            tally.gaps,
            tally.without_missed,
            tally.reached,
            tally.missed_tracks,
            share(tally.missed_seconds_in_gaps, tally.missed_seconds),
            tally.missed_seconds,
        );
        for (truth, row) in tally.by_truth.iter().enumerate() {
            if row[0] == 0 {
                continue;
            }
            let held = if truth == 5 {
                String::from("5+")
            } else {
                truth.to_string()
            };
            println!(
                "    holding {held:>2}: {} gaps, estimate exact {}, off by one {}, range misses {}, no estimate {}",
                row[0], row[1], row[2], row[3], row[4]
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    fn track(asset: &str, start_seconds: f64, end_seconds: f64, in_library: bool) -> Truth {
        Truth {
            accepted: BTreeSet::from([asset.to_owned()]),
            start_seconds,
            end_seconds,
            in_library,
        }
    }

    fn confident(asset: &str, start_seconds: f64, end_seconds: f64) -> Segment {
        Segment {
            asset: asset.to_owned(),
            start_seconds,
            end_seconds,
            windows: 6,
            hits: 1_500,
            fitted: true,
        }
    }

    fn possible(asset: &str, start_seconds: f64, end_seconds: f64) -> Segment {
        Segment {
            windows: 2,
            hits: 120,
            ..confident(asset, start_seconds, end_seconds)
        }
    }

    fn spans(at: &AtLength) -> Vec<(f64, f64)> {
        at.gaps
            .iter()
            .map(|gap| (gap.start_seconds, gap.end_seconds))
            .collect()
    }

    #[test]
    fn a_gaps_neighbours_are_the_confident_plays_at_its_edges() {
        let truth = [
            track("a", 0.0, 300.0, true),
            track("x", 300.0, 600.0, false),
            track("b", 600.0, 900.0, true),
        ];
        let found = [
            confident("a", 0.0, 310.0),
            possible("p", 320.0, 340.0),
            confident("b", 590.0, 900.0),
        ];
        let listing = list("m", &truth, &found, 900.0, 0.0, &[30.0]);

        let neighbours = neighbours(&found, &listing.at[0].gaps[0]);

        assert_eq!(neighbours, ["a", "b"]);
    }

    #[test]
    fn a_missed_track_between_two_found_ones_is_one_gap_holding_it() {
        let truth = [
            track("a", 0.0, 300.0, true),
            track("x", 300.0, 600.0, false),
            track("b", 600.0, 900.0, true),
        ];
        let found = [confident("a", 0.0, 310.0), confident("b", 590.0, 900.0)];

        let listing = list("m", &truth, &found, 900.0, 0.0, &[30.0]);

        let at = &listing.at[0];
        assert_eq!(spans(at), [(310.0, 590.0)]);
        assert_eq!(at.gaps[0].missed_tracks, [2]);
        assert!(at.gaps[0].indexed_tracks.is_empty());
        assert_eq!(at.gaps[0].missed_seconds, 280);
        assert_eq!(listing.tracks[1].own_seconds, 300);
    }

    #[test]
    fn the_mix_start_and_end_are_edges() {
        let truth = [track("a", 0.0, 100.0, true), track("b", 100.0, 200.0, true)];
        let found = [confident("a", 25.0, 100.0), confident("b", 100.0, 160.0)];

        let listing = list("m", &truth, &found, 200.0, 0.0, &[10.0]);

        assert_eq!(spans(&listing.at[0]), [(0.0, 25.0), (160.0, 200.0)]);
    }

    #[test]
    fn a_gap_between_two_segments_of_one_play_counts_and_a_possible_play_does_not_close_it() {
        // One play of `a` across 60 s (ADR 0006 groups gaps up to 90 s),
        // with a possible play of another file inside the gap.
        let truth = [track("a", 0.0, 300.0, true)];
        let found = [
            confident("a", 0.0, 100.0),
            confident("a", 160.0, 300.0),
            possible("remix", 110.0, 140.0),
        ];

        let listing = list("m", &truth, &found, 300.0, 0.0, &[30.0]);

        let at = &listing.at[0];
        assert_eq!(spans(at), [(100.0, 160.0)]);
        assert_eq!(at.gaps[0].possible_plays, 1);
        assert_eq!(at.gaps[0].indexed_tracks, [1]);
        assert!(at.gaps[0].missed_tracks.is_empty());
    }

    #[test]
    fn a_possible_segment_of_a_confident_play_covers_its_time() {
        // ADR 0013: a play is as confident as its strongest segment.
        let truth = [track("a", 0.0, 200.0, true)];
        let found = [possible("a", 0.0, 40.0), confident("a", 60.0, 200.0)];

        let listing = list("m", &truth, &found, 200.0, 0.0, &[10.0]);

        assert_eq!(spans(&listing.at[0]), [(40.0, 60.0)]);
    }

    #[test]
    fn the_minimum_length_leaves_short_gaps_out() {
        let truth = [track("a", 0.0, 100.0, true), track("b", 100.0, 200.0, true)];
        let found = [confident("a", 0.0, 95.0), confident("b", 115.0, 200.0)];

        let listing = list("m", &truth, &found, 200.0, 0.0, &[10.0, 30.0]);

        assert_eq!(spans(&listing.at[0]), [(95.0, 115.0)]);
        assert!(listing.at[1].gaps.is_empty());
    }

    #[test]
    fn a_short_missed_track_is_held_by_half_of_it_and_a_short_gap_by_half_of_itself() {
        let truth = [
            track("a", 0.0, 100.0, true),
            track("x", 90.0, 130.0, false),
            track("b", 120.0, 300.0, true),
        ];
        // 20 s of the 40 s play: half of it.
        let half_of_the_track = [confident("a", 0.0, 100.0), confident("b", 120.0, 300.0)];
        // 8 s of it, all of the gap.
        let inside_it = [confident("a", 0.0, 112.0), confident("b", 120.0, 300.0)];

        let held = |found: &[Segment]| {
            list("m", &truth, found, 300.0, 0.0, &[5.0]).at[0].gaps[0]
                .missed_tracks
                .clone()
        };

        assert_eq!(held(&half_of_the_track), [2]);
        assert_eq!(held(&inside_it), [2]);
    }

    #[test]
    fn a_gap_that_only_touches_a_missed_track_does_not_hold_it() {
        let truth = [
            track("a", 0.0, 300.0, true),
            track("x", 300.0, 600.0, false),
        ];
        // 40 s of the end of `a` and 10 s of `x`.
        let found = [
            confident("a", 0.0, 250.0),
            confident("other", 300.0 + 10.0, 600.0),
        ];

        let gap = &list("m", &truth, &found, 600.0, 0.0, &[30.0]).at[0].gaps[0];

        assert_eq!(gap.indexed_tracks, [1]);
        assert!(gap.missed_tracks.is_empty());
    }

    #[test]
    fn seconds_within_the_margin_of_a_border_are_transitions() {
        let truth = [
            track("a", 0.0, 300.0, true),
            track("x", 300.0, 600.0, false),
            track("b", 600.0, 900.0, true),
        ];
        let found = [confident("a", 0.0, 250.0), confident("b", 650.0, 900.0)];

        let listing = list("m", &truth, &found, 900.0, 30.0, &[30.0]);

        let gap = &listing.at[0].gaps[0];
        assert_eq!(gap.missed_seconds, 240);
        assert_eq!(gap.transition_seconds, 120);
        assert_eq!(gap.indexed_seconds, 40);
        assert_eq!(listing.tracks[1].own_seconds, 240);
    }

    #[test]
    fn rips_over_the_same_passage_are_one_entry() {
        let found = [
            confident("a", 0.0, 290.0),
            confident("a rip", 5.0, 300.0),
            confident("b", 280.0, 600.0),
            confident("c", 590.0, 900.0),
        ];
        let labels = labels(&found, Rule::Windows);

        assert_eq!(entry_starts(&labels), [0.0, 280.0, 590.0]);
    }

    #[test]
    fn the_estimate_is_the_gap_over_the_median_interval_without_the_gaps_own() {
        // Entries every 200 s, except the 600 s of two missed tracks.
        let mut truth = Vec::new();
        let mut found = Vec::new();
        for (number, start) in [0.0, 200.0, 400.0, 600.0, 1_200.0, 1_400.0]
            .iter()
            .enumerate()
        {
            let end = start + 200.0;
            truth.push(track(&format!("t{number}"), *start, end, true));
            found.push(confident(&format!("t{number}"), *start, end - 5.0));
        }
        truth.insert(4, track("x", 800.0, 1_000.0, false));
        truth.insert(5, track("y", 1_000.0, 1_200.0, false));

        let listing = list("m", &truth, &found, 1_600.0, 0.0, &[30.0]);

        let at = &listing.at[0];
        assert_eq!(at.intervals, [200.0, 200.0, 200.0, 200.0]);
        assert_eq!(at.median_interval_seconds, Some(200.0));
        let gap = &at.gaps[0];
        assert_eq!((gap.start_seconds, gap.end_seconds), (795.0, 1_200.0));
        assert_eq!(gap.missed_tracks, [5, 6]);
        assert_eq!(gap.estimate, Some(2));
        assert_eq!(gap.range(), Some((1, 3)));
    }

    #[test]
    fn too_few_intervals_give_no_estimate_and_a_short_gap_still_counts_one() {
        let truth = [track("a", 0.0, 200.0, true), track("b", 200.0, 400.0, true)];
        let found = [confident("a", 0.0, 190.0), confident("b", 230.0, 400.0)];

        let at = &list("m", &truth, &found, 400.0, 0.0, &[30.0]).at[0];

        assert_eq!(at.median_interval_seconds, None);
        assert_eq!(at.gaps[0].estimate, None);
        assert_eq!(estimate(40.0, 300.0), 1);
        assert_eq!(estimate(480.0, 300.0), 2);
    }

    #[test]
    fn the_tally_counts_estimates_by_the_missed_tracks_held() {
        let truth = [
            track("a", 0.0, 300.0, true),
            track("x", 300.0, 600.0, false),
            track("b", 600.0, 900.0, true),
        ];
        let found = [confident("a", 0.0, 310.0), confident("b", 590.0, 900.0)];
        let mut listing = list("m", &truth, &found, 900.0, 0.0, &[30.0]);
        listing.at[0].gaps[0].estimate = Some(3);

        let counted = tally(&[&listing], 30.0);

        assert_eq!(counted.gaps, 1);
        assert_eq!(counted.reached, 1);
        assert_eq!(counted.missed_tracks, 1);
        assert_eq!(counted.by_truth[1], [1, 0, 0, 1, 0]);
    }
}

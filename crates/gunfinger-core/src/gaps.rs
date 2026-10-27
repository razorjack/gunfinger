//! Time without a confident match: the stretches of a searched recording
//! that no segment of a confident play covers
//! (`docs/research/2026-10-21-gap-measures.md`, ADR 0023). A play is as
//! confident as its strongest segment (ADR 0013), so all its segments
//! cover their time, and a stretch between two segments of one play
//! counts. Possible plays cover nothing (ADR 0006). The CLI prints the
//! gaps; the harness lists them against the truth.
//!
//! How many tracks a gap holds (ADR 0024): the records its repeats count
//! (`repeats::records`, method 2) limited to the range its length gives
//! over the mix's intervals between entries (method 1's range B), as
//! experiment 0165 measured it.

/// The stretches of `start..end` that no span of `covered` touches, in
/// order, at least `min_seconds` long. The recording's start and end are
/// edges like a span's.
pub fn uncovered(
    covered: &[(f64, f64)],
    start: f64,
    end: f64,
    min_seconds: f64,
) -> Vec<(f64, f64)> {
    let mut covered = covered.to_vec();
    covered.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut stretches = Vec::new();
    let mut reached = start;
    for (from, to) in covered {
        if from > reached {
            stretches.push((reached, from.min(end)));
        }
        reached = reached.max(to);
    }
    if reached < end {
        stretches.push((reached, end));
    }
    stretches.retain(|(from, to)| to - from >= min_seconds);
    stretches
}

/// Two confident plays are one entry when they overlap for this share of
/// the shorter: the CLI's `SAME_PASSAGE_OVERLAP`, without its tests of
/// alignment and tags (the gap measures' entries).
const SAME_PASSAGE_OVERLAP: f64 = 0.5;
/// Fewer clean intervals than this give no range.
pub const MIN_INTERVALS: usize = 3;

/// Where the entries start: the confident plays (`start..end`) in order,
/// one that overlaps the entry before it for half of the shorter of the
/// two joining that entry.
pub fn entry_starts(plays: &[(f64, f64)]) -> Vec<f64> {
    let mut plays = plays.to_vec();
    plays.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut entries: Vec<(f64, f64)> = Vec::new();
    for (start, end) in plays {
        match entries.last_mut() {
            Some(last) if over_same_passage(*last, (start, end)) => last.1 = last.1.max(end),
            _ => entries.push((start, end)),
        }
    }
    entries.into_iter().map(|(start, _)| start).collect()
}

fn over_same_passage(a: (f64, f64), b: (f64, f64)) -> bool {
    let overlap = a.1.min(b.1) - a.0.max(b.0);
    let shorter = (a.1 - a.0).min(b.1 - b.0);
    overlap >= SAME_PASSAGE_OVERLAP * shorter
}

/// The intervals between consecutive entry starts that no listed gap
/// overlaps: the others hold time the entries do not account for.
pub fn clean_intervals(entry_starts: &[f64], gaps: &[(f64, f64)]) -> Vec<f64> {
    entry_starts
        .windows(2)
        .filter(|pair| {
            !gaps
                .iter()
                .any(|&(start, end)| pair[0] < end && start < pair[1])
        })
        .map(|pair| pair[1] - pair[0])
        .collect()
}

/// The median, the mean of the middle two for an even count (as Python's
/// `statistics.median`, which experiment 0154 used).
pub fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let middle = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[middle - 1] + sorted[middle]) / 2.0
    } else {
        sorted[middle]
    }
}

/// The lower and the upper quartile, as Python's `statistics.quantiles(n=4)`
/// (its default, exclusive method) cuts them; the measured range B used it.
/// `values` holds at least two.
fn quartiles(values: &[f64]) -> (f64, f64) {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    let count = sorted.len() as i64;
    let cut = |i: i64| {
        let m = count + 1;
        let j = (i * m / 4).clamp(1, count - 1);
        let delta = i * m - j * 4;
        let j = j as usize;
        (sorted[j - 1] * (4 - delta) as f64 + sorted[j] * delta as f64) / 4.0
    };
    (cut(1), cut(3))
}

/// Method 1's range B: the gap's length over the upper and over the lower
/// quartile of the clean intervals, each rounded (half to even, as
/// Python's `round`) and at least 1. None with fewer than `MIN_INTERVALS`.
pub fn length_range(length_seconds: f64, intervals: &[f64]) -> Option<(u32, u32)> {
    if intervals.len() < MIN_INTERVALS {
        return None;
    }
    let (lower, upper) = quartiles(intervals);
    let tracks = |interval: f64| ((length_seconds / interval).round_ties_even() as u32).max(1);
    Some((tracks(upper), tracks(lower)))
}

/// How many tracks a gap holds, as a range one either side of the count.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Estimate {
    pub low: u32,
    pub high: u32,
    pub method: Method,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    /// The records counted by the gap's repeats, limited to its length's
    /// range (0165).
    RepeatsWithinLength,
    /// The records counted by the gap's repeats alone: the mix has fewer
    /// than `MIN_INTERVALS` clean intervals.
    Repeats,
}

impl Method {
    /// The name JSON reports give the method.
    pub fn name(self) -> &'static str {
        match self {
            Method::RepeatsWithinLength => "repeats-within-length",
            Method::Repeats => "repeats",
        }
    }
}

/// The combined count: `records` (method 2) raised to the low end of
/// `length_range` or lowered to its high end; 0 stays 0 and, with no
/// range, the records stand alone.
pub fn combined(records: usize, length_range: Option<(u32, u32)>) -> u32 {
    let records = records as u32;
    match length_range {
        Some((low, high)) if records > 0 => records.max(low).min(high),
        _ => records,
    }
}

/// The estimate printed for a gap: the combined count n as n-1 to n+1, at
/// least 1. A count of 0 gives none: neither method can say that a gap
/// holds no track (ADR 0024).
pub fn estimate(records: usize, length_range: Option<(u32, u32)>) -> Option<Estimate> {
    let count = combined(records, length_range);
    (count > 0).then(|| Estimate {
        low: (count - 1).max(1),
        high: count.saturating_add(1),
        method: match length_range {
            Some(_) => Method::RepeatsWithinLength,
            None => Method::Repeats,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_start_and_the_end_are_edges() {
        let gaps = uncovered(&[(25.0, 100.0), (100.0, 160.0)], 0.0, 200.0, 10.0);

        assert_eq!(gaps, [(0.0, 25.0), (160.0, 200.0)]);
    }

    #[test]
    fn a_stretch_of_exactly_the_minimum_is_listed_and_a_shorter_one_is_not() {
        let covered = [(0.0, 10.0), (55.0, 100.0), (144.9, 200.0)];

        let gaps = uncovered(&covered, 0.0, 200.0, 45.0);

        assert_eq!(gaps, [(10.0, 55.0)]);
    }

    #[test]
    fn overlapping_and_nested_spans_cover_their_union() {
        let covered = [(50.0, 120.0), (0.0, 60.0), (70.0, 90.0), (180.0, 300.0)];

        let gaps = uncovered(&covered, 0.0, 300.0, 1.0);

        assert_eq!(gaps, [(120.0, 180.0)]);
    }

    #[test]
    fn edges_other_than_zero_bound_the_stretches() {
        // A search of 10:00-20:00 of a recording: times are the recording's.
        let gaps = uncovered(&[(700.0, 1100.0)], 600.0, 1200.0, 45.0);

        assert_eq!(gaps, [(600.0, 700.0), (1100.0, 1200.0)]);
    }

    #[test]
    fn plays_over_the_same_passage_are_one_entry() {
        // A rip of the first play, then plays that only touch.
        let plays = [(280.0, 600.0), (0.0, 290.0), (5.0, 300.0), (590.0, 900.0)];

        assert_eq!(entry_starts(&plays), [0.0, 280.0, 590.0]);
    }

    #[test]
    fn an_interval_that_meets_a_gap_is_not_clean() {
        let starts = [0.0, 200.0, 400.0, 1_000.0, 1_250.0];

        let intervals = clean_intervals(&starts, &[(590.0, 1_000.0)]);

        assert_eq!(intervals, [200.0, 200.0, 250.0]);
    }

    #[test]
    fn quartiles_are_cut_as_python_cuts_them() {
        // statistics.quantiles(values, n=4)[0] and [2].
        assert_eq!(quartiles(&[300.0, 200.0, 250.0]), (200.0, 300.0));
        assert_eq!(
            quartiles(&[180.0, 240.0, 250.0, 260.0, 300.0, 420.0]),
            (225.0, 330.0)
        );
        assert_eq!(
            quartiles(&[307.0, 316.0, 232.0, 233.0, 251.0, 235.0, 190.0]),
            (232.0, 307.0)
        );
    }

    #[test]
    fn the_length_range_divides_by_the_quartiles_and_needs_three_intervals() {
        let intervals = [180.0, 240.0, 250.0, 260.0, 300.0, 420.0];

        assert_eq!(length_range(100.0, &intervals), Some((1, 1)));
        assert_eq!(length_range(477.5, &intervals), Some((1, 2)));
        assert_eq!(length_range(900.0, &intervals), Some((3, 4)));
        assert_eq!(length_range(1_800.0, &intervals), Some((5, 8)));
        assert_eq!(length_range(900.0, &intervals[..2]), None);
    }

    #[test]
    fn rounding_halves_goes_to_the_even_count_as_python_rounds() {
        // 500 / 200 = 2.5 and 700 / 200 = 3.5.
        assert_eq!(length_range(500.0, &[200.0, 200.0, 200.0]), Some((2, 2)));
        assert_eq!(length_range(700.0, &[200.0, 200.0, 200.0]), Some((4, 4)));
    }

    #[test]
    fn the_estimate_is_one_either_side_of_the_count_at_least_one() {
        let within = |low, high| {
            Some(Estimate {
                low,
                high,
                method: Method::RepeatsWithinLength,
            })
        };
        assert_eq!(estimate(1, Some((1, 2))), within(1, 2));
        assert_eq!(estimate(2, Some((1, 3))), within(1, 3));
        assert_eq!(estimate(3, Some((2, 4))), within(2, 4));
        assert_eq!(estimate(4, Some((3, 5))), within(3, 5));
    }

    #[test]
    fn the_length_range_limits_the_records_when_the_methods_disagree() {
        // One merged stretch for three records' worth of length, and five
        // stretches in a gap of two.
        assert_eq!(combined(1, Some((3, 4))), 3);
        assert_eq!(combined(5, Some((1, 2))), 2);
        assert_eq!(
            estimate(1, Some((3, 4))).map(|e| (e.low, e.high)),
            Some((2, 4))
        );
    }

    #[test]
    fn without_a_range_the_records_stand_alone() {
        assert_eq!(
            estimate(3, None),
            Some(Estimate {
                low: 2,
                high: 4,
                method: Method::Repeats,
            })
        );
    }

    #[test]
    fn no_repeated_stretch_gives_no_estimate_even_with_a_range() {
        assert_eq!(combined(0, Some((2, 3))), 0);
        assert_eq!(estimate(0, Some((2, 3))), None);
        assert_eq!(estimate(0, None), None);
    }

    #[test]
    fn nothing_covered_is_one_stretch_over_the_whole_recording() {
        assert_eq!(uncovered(&[], 0.0, 90.0, 45.0), [(0.0, 90.0)]);
        assert!(uncovered(&[], 0.0, 30.0, 45.0).is_empty());
    }
}

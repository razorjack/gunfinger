//! Time without a confident match: the stretches of a searched recording
//! that no segment of a confident play covers
//! (`docs/research/2026-10-21-gap-measures.md`, ADR 0023). A play is as
//! confident as its strongest segment (ADR 0013), so all its segments
//! cover their time, and a stretch between two segments of one play
//! counts. Possible plays cover nothing (ADR 0006). The CLI prints the
//! gaps; the harness lists them against the truth.

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
    fn nothing_covered_is_one_stretch_over_the_whole_recording() {
        assert_eq!(uncovered(&[], 0.0, 90.0, 45.0), [(0.0, 90.0)]);
        assert!(uncovered(&[], 0.0, 30.0, 45.0).is_empty());
    }
}

//! Time without a confident match (ADR 0023): the stretches of the searched
//! part of a recording, at least `MIN_GAP_SECONDS` long, that no segment of
//! a confident play covers (`gunfinger_core::gaps`). Derived from the plays
//! whenever a report is rendered, never read from a saved one. A gap says
//! only that nothing confident was found there; it counts no tracks.

use serde::Serialize;

use crate::report::{FoundPlay, Level, Report};
use crate::table::timecode;

/// Shorter stretches are mostly blends and a found track's own uncovered
/// time (experiment 0159); 45 s is our first choice, to revisit once real
/// mixes show how the lines read.
pub const MIN_GAP_SECONDS: f64 = 45.0;

#[derive(Debug, PartialEq, Serialize)]
pub struct Gap {
    pub start_seconds: f64,
    pub end_seconds: f64,
    /// The positions, from 1 in the report's order, of the possible plays
    /// with a segment overlapping the gap. They identify nothing, so they
    /// neither close nor split it (ADR 0006).
    pub possible_plays: Vec<usize>,
}

impl Gap {
    /// "ID - ID (2:25 without a confident match)": the name DJ tracklists
    /// give a track nobody has identified, and how long nothing confident
    /// was found.
    pub fn label(&self) -> String {
        format!(
            "ID - ID ({} without a confident match)",
            timecode(self.end_seconds - self.start_seconds)
        )
    }
}

/// The report's gaps, in order.
pub fn gaps(report: &Report) -> Vec<Gap> {
    let covered: Vec<(f64, f64)> = report
        .plays
        .iter()
        .filter(|play| play.confidence == Level::Confident)
        .flat_map(spans)
        .collect();
    let start = report.query.start_seconds;
    let end = start + report.query.duration_seconds;
    gunfinger_core::gaps::uncovered(&covered, start, end, MIN_GAP_SECONDS)
        .into_iter()
        .map(|(start_seconds, end_seconds)| Gap {
            start_seconds,
            end_seconds,
            possible_plays: report
                .plays
                .iter()
                .zip(1..)
                .filter(|(play, _)| play.confidence != Level::Confident)
                .filter(|(play, _)| {
                    spans(play).any(|(from, to)| from < end_seconds && start_seconds < to)
                })
                .map(|(_, position)| position)
                .collect(),
        })
        .collect()
}

fn spans(play: &FoundPlay) -> impl Iterator<Item = (f64, f64)> + '_ {
    play.segments
        .iter()
        .map(|segment| (segment.start_seconds, segment.end_seconds))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::SharedWith;
    use crate::table::tests::report;

    /// The fixture's plays, confident ones cover 0:00-1:21 and 1:36-2:20
    /// (with 1:51-1:52 between c.wav's segments), on a recording of 4:00.
    fn four_minutes() -> Report {
        let mut report = report();
        report.query.duration_seconds = 240.0;
        report
    }

    fn gap(start_seconds: f64, end_seconds: f64, possible_plays: Vec<usize>) -> Gap {
        Gap {
            start_seconds,
            end_seconds,
            possible_plays,
        }
    }

    #[test]
    fn the_end_of_the_recording_is_an_edge() {
        assert_eq!(gaps(&four_minutes()), [gap(140.0, 240.0, Vec::new())]);
    }

    #[test]
    fn the_start_of_the_searched_part_is_an_edge() {
        let mut report = report();
        report.plays.drain(..2);

        assert_eq!(gaps(&report), [gap(0.0, 96.0, vec![1])]);
    }

    #[test]
    fn a_gap_of_exactly_45_seconds_is_listed_and_a_shorter_one_is_not() {
        let mut report = four_minutes();
        report.query.duration_seconds = 185.0;
        assert_eq!(gaps(&report), [gap(140.0, 185.0, Vec::new())]);

        report.query.duration_seconds = 184.9;
        assert!(gaps(&report).is_empty());
    }

    #[test]
    fn a_possible_play_inside_a_gap_neither_closes_nor_splits_it() {
        let mut report = four_minutes();
        // b.wav becomes possible: nothing confident covers 0:41-1:36, with
        // b.wav and insert.wav (plays 2 and 3) inside.
        report.plays[1].confidence = Level::Possible;
        report.plays[1].segments[0].confidence = Level::Possible;

        assert_eq!(
            gaps(&report),
            [gap(41.0, 96.0, vec![2, 3]), gap(140.0, 240.0, Vec::new())]
        );
    }

    #[test]
    fn a_play_the_tracklist_leaves_out_covers_nothing_and_is_named_in_the_gap() {
        let mut report = four_minutes();
        report.plays[1].confidence = Level::Possible;
        report.plays[1].segments[0].confidence = Level::Possible;
        report.plays[2].shares_material_with = Some(SharedWith {
            play: 1,
            asset: String::from("a.wav"),
        });

        assert_eq!(gaps(&report)[0], gap(41.0, 96.0, vec![2, 3]));
    }

    #[test]
    fn overlapping_confident_plays_cover_their_union() {
        // a.wav (0:00-0:41) and b.wav (0:38-1:21) overlap; c.wav's segments
        // leave 1 s between them.
        let mut report = four_minutes();
        report.plays.remove(2);

        assert_eq!(gaps(&report), [gap(140.0, 240.0, Vec::new())]);
    }

    #[test]
    fn a_stretch_between_two_segments_of_one_play_counts() {
        let mut report = four_minutes();
        report.plays[3].segments[1].start_seconds = 200.0;
        report.plays[3].segments[1].end_seconds = 240.0;
        report.plays[3].end_seconds = 240.0;

        assert_eq!(gaps(&report), [gap(111.0, 200.0, Vec::new())]);
    }

    #[test]
    fn a_possible_segment_of_a_confident_play_covers_its_time() {
        // ADR 0013: a play is as confident as its strongest segment.
        let mut report = four_minutes();
        report.plays[3].segments[1].confidence = Level::Possible;
        report.plays[3].segments[1].end_seconds = 240.0;
        report.plays[3].end_seconds = 240.0;

        assert!(gaps(&report).is_empty());
    }

    #[test]
    fn a_report_without_a_confident_play_is_one_gap() {
        let mut report = four_minutes();
        report
            .plays
            .retain(|play| play.confidence != Level::Confident);

        assert_eq!(gaps(&report), [gap(0.0, 240.0, vec![1])]);
    }

    #[test]
    fn the_label_gives_the_length_cut_to_the_second() {
        assert_eq!(
            gap(403.2, 880.8, Vec::new()).label(),
            "ID - ID (7:57 without a confident match)"
        );
    }
}

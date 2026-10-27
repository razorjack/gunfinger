//! Time without a confident match (ADR 0023): the stretches of the searched
//! part of a recording, at least `MIN_GAP_SECONDS` long, that no segment of
//! a confident play covers (`gunfinger_core::gaps`). Derived from the plays
//! whenever a report is rendered, never read from a saved one. A gap says
//! only that nothing confident was found there. Its estimate of how many
//! tracks it holds (ADR 0024) is derived at the same time, from the
//! repeats `identify` found in it (`Report::gap_repeats`) and the mix's
//! intervals between entries.

use gunfinger_core::gaps::{
    Estimate, Method, clean_intervals, entry_starts, estimate, length_range, uncovered,
};
use gunfinger_core::profile::Profile;
use gunfinger_core::repeats;
use serde::Serialize;

use crate::report::{FoundPlay, Level, Report, Stretch};
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
    /// How many tracks it holds, as a range; none without a repeated
    /// stretch or in a report without `gap_repeats`.
    pub tracks: Option<Tracks>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Tracks {
    pub low: u32,
    pub high: u32,
    pub method: TracksMethod,
}

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TracksMethod {
    /// The records the gap's repeats count, limited to the range its length
    /// gives over the mix's intervals between entries.
    RepeatsWithinLength,
    /// The records alone: the mix has too few intervals without a gap.
    Repeats,
}

impl From<Estimate> for Tracks {
    fn from(estimate: Estimate) -> Tracks {
        Tracks {
            low: estimate.low,
            high: estimate.high,
            method: match estimate.method {
                Method::RepeatsWithinLength => TracksMethod::RepeatsWithinLength,
                Method::Repeats => TracksMethod::Repeats,
            },
        }
    }
}

impl Gap {
    /// "ID - ID (7:57 without a confident match; about 1-3 tracks)": the
    /// name DJ tracklists give a track nobody has identified, how long
    /// nothing confident was found, and the estimate, always a range.
    pub fn label(&self) -> String {
        let length = timecode(self.end_seconds - self.start_seconds);
        match &self.tracks {
            Some(tracks) => format!(
                "ID - ID ({length} without a confident match; about {}-{} tracks)",
                tracks.low, tracks.high
            ),
            None => format!("ID - ID ({length} without a confident match)"),
        }
    }
}

/// The report's gaps, in order.
pub fn gaps(report: &Report) -> Vec<Gap> {
    let spans = uncovered_spans(report);
    let intervals = clean_intervals(&entry_starts(&confident_plays(report)), &spans);
    spans
        .iter()
        .map(|&(start_seconds, end_seconds)| Gap {
            start_seconds,
            end_seconds,
            possible_plays: report
                .plays
                .iter()
                .zip(1..)
                .filter(|(play, _)| play.confidence != Level::Confident)
                .filter(|(play, _)| {
                    segment_spans(play).any(|(from, to)| from < end_seconds && start_seconds < to)
                })
                .map(|(_, position)| position)
                .collect(),
            tracks: report.gap_repeats.as_ref().and_then(|stretches| {
                let records = stretches
                    .iter()
                    .filter(|stretch| {
                        let middle = (stretch.start_seconds + stretch.end_seconds) / 2.0;
                        start_seconds <= middle && middle < end_seconds
                    })
                    .count();
                let range = length_range(end_seconds - start_seconds, &intervals);
                estimate(records, range).map(Tracks::from)
            }),
        })
        .collect()
}

/// The repeated stretches of each gap of `report`, found in `samples`, the
/// decoded audio of the searched part at the profile's rate, one gap per
/// worker (`repeats::records_in_gaps`, ADR 0024).
pub fn find_repeats(
    report: &Report,
    samples: &[f32],
    profile: &Profile,
    jobs: usize,
) -> Vec<Stretch> {
    repeats::records_in_gaps(
        samples,
        report.query.start_seconds,
        &uncovered_spans(report),
        profile,
        jobs,
    )
    .into_iter()
    .map(|(start_seconds, end_seconds)| Stretch {
        start_seconds,
        end_seconds,
    })
    .collect()
}

fn uncovered_spans(report: &Report) -> Vec<(f64, f64)> {
    let covered: Vec<(f64, f64)> = report
        .plays
        .iter()
        .filter(|play| play.confidence == Level::Confident)
        .flat_map(segment_spans)
        .collect();
    let start = report.query.start_seconds;
    let end = start + report.query.duration_seconds;
    uncovered(&covered, start, end, MIN_GAP_SECONDS)
}

/// Each confident play from its first to its last segment: method 1's
/// entries are made from these (`gunfinger_core::gaps::entry_starts`).
fn confident_plays(report: &Report) -> Vec<(f64, f64)> {
    report
        .plays
        .iter()
        .filter(|play| play.confidence == Level::Confident)
        .map(|play| (play.start_seconds, play.end_seconds))
        .collect()
}

fn segment_spans(play: &FoundPlay) -> impl Iterator<Item = (f64, f64)> + '_ {
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
            tracks: None,
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
    fn an_excerpt_has_its_own_edges() {
        // The fixture's mix searched from 10:00 (`--start 10:00`) for 4:00:
        // times are the recording's, and nothing before 10:00 is a gap.
        let mut report = four_minutes();
        report.query.start_seconds = 600.0;
        for play in &mut report.plays {
            play.start_seconds += 600.0;
            play.end_seconds += 600.0;
            for segment in &mut play.segments {
                segment.start_seconds += 600.0;
                segment.end_seconds += 600.0;
            }
        }
        report.plays.drain(..2);

        assert_eq!(
            gaps(&report),
            [gap(600.0, 696.0, vec![1]), gap(740.0, 840.0, Vec::new())]
        );
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
        // A mark that cannot occur in a report read back (a.wav ends before
        // insert.wav starts); it shows that `gaps` ignores the mark.
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
    fn nested_confident_plays_cover_the_outer_one() {
        // a.wav now plays 0:00-2:30, over b.wav and c.wav: coverage reaches
        // 2:30, not c.wav's end at 2:20.
        let mut report = four_minutes();
        report.plays[0].end_seconds = 150.0;
        report.plays[0].segments[0].end_seconds = 150.0;

        assert_eq!(gaps(&report), [gap(150.0, 240.0, Vec::new())]);
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

    #[test]
    fn the_label_gives_the_estimate_as_a_range() {
        let gap = Gap {
            tracks: Some(Tracks {
                low: 1,
                high: 3,
                method: TracksMethod::RepeatsWithinLength,
            }),
            ..gap(403.2, 880.8, Vec::new())
        };

        assert_eq!(
            gap.label(),
            "ID - ID (7:57 without a confident match; about 1-3 tracks)"
        );
    }

    /// A recording of `duration_seconds` with a confident play over each of
    /// `plays`, and `repeats` found in its gaps.
    fn mix(plays: &[(f64, f64)], duration_seconds: f64, repeats: Option<&[(f64, f64)]>) -> Report {
        let mut report = report();
        let one_segment = report.plays[1].clone();
        report.plays = plays
            .iter()
            .map(|&(start, end)| {
                let mut play = one_segment.clone();
                (play.start_seconds, play.end_seconds) = (start, end);
                (play.segments[0].start_seconds, play.segments[0].end_seconds) = (start, end);
                play
            })
            .collect();
        report.query.duration_seconds = duration_seconds;
        report.gap_repeats = repeats.map(|repeats| {
            repeats
                .iter()
                .map(|&(start_seconds, end_seconds)| Stretch {
                    start_seconds,
                    end_seconds,
                })
                .collect()
        });
        report
    }

    /// Entries every 200 s from 0:00 to 10:00, then a gap of `tracks`
    /// tracks' length (and 5 s of the last entry's end), then one more
    /// entry: three clean intervals of 200 s.
    fn entries_around_a_gap(tracks: u32, repeats: &[(f64, f64)]) -> Report {
        let next = 800.0 + 200.0 * f64::from(tracks);
        mix(
            &[
                (0.0, 195.0),
                (200.0, 395.0),
                (400.0, 595.0),
                (600.0, 795.0),
                (next, next + 195.0),
            ],
            next + 195.0,
            Some(repeats),
        )
    }

    fn tracks(report: &Report) -> Vec<Option<(u32, u32, TracksMethod)>> {
        gaps(report)
            .into_iter()
            .map(|gap| {
                gap.tracks
                    .map(|tracks| (tracks.low, tracks.high, tracks.method))
            })
            .collect()
    }

    #[test]
    fn each_count_is_printed_one_either_side_at_least_one() {
        // One repeated stretch per track, in a gap as long as that many
        // intervals: both methods agree.
        let stretches = [
            (820.0, 900.0),
            (1_020.0, 1_100.0),
            (1_220.0, 1_300.0),
            (1_420.0, 1_500.0),
        ];
        for (tracks_held, expected) in [(1, (1, 2)), (2, (1, 3)), (3, (2, 4)), (4, (3, 5))] {
            let report = entries_around_a_gap(tracks_held, &stretches[..tracks_held as usize]);

            assert_eq!(
                tracks(&report),
                [Some((
                    expected.0,
                    expected.1,
                    TracksMethod::RepeatsWithinLength
                ))],
                "{tracks_held} tracks"
            );
        }
    }

    #[test]
    fn the_length_limits_the_repeats_where_the_two_disagree() {
        // Three tracks' length: one long stretch (records sharing a
        // passage) is raised to 3, six stretches are lowered to 3.
        let merged = entries_around_a_gap(3, &[(820.0, 1_350.0)]);
        let split = entries_around_a_gap(
            3,
            &[
                (800.0, 840.0),
                (850.0, 890.0),
                (900.0, 940.0),
                (1_000.0, 1_040.0),
                (1_100.0, 1_140.0),
                (1_200.0, 1_240.0),
            ],
        );

        assert_eq!(
            tracks(&merged),
            [Some((2, 4, TracksMethod::RepeatsWithinLength))]
        );
        assert_eq!(
            tracks(&split),
            [Some((2, 4, TracksMethod::RepeatsWithinLength))]
        );
    }

    #[test]
    fn without_three_clean_intervals_the_repeats_stand_alone() {
        // The only interval holds the gap.
        let report = mix(
            &[(0.0, 195.0), (800.0, 995.0)],
            995.0,
            Some(&[(220.0, 300.0), (420.0, 500.0)]),
        );

        assert_eq!(tracks(&report), [Some((1, 3, TracksMethod::Repeats))]);
    }

    #[test]
    fn gaps_at_the_start_and_the_end_get_estimates_from_the_stretches_inside_them() {
        let report = mix(
            &[
                (100.0, 295.0),
                (300.0, 495.0),
                (500.0, 695.0),
                (700.0, 895.0),
            ],
            1_100.0,
            Some(&[(10.0, 50.0), (900.0, 960.0), (1_000.0, 1_090.0)]),
        );

        // 100 s and 205 s over intervals of 200 s: 1 track each; the end's
        // two stretches are lowered to 1.
        assert_eq!(
            tracks(&report),
            [
                Some((1, 2, TracksMethod::RepeatsWithinLength)),
                Some((1, 2, TracksMethod::RepeatsWithinLength))
            ]
        );
    }

    #[test]
    fn within_a_wider_range_the_repeats_decide() {
        // Clean intervals of 180, 200, 250 and 300 s: a gap of 600 s gives
        // 2 to 3 tracks by its length.
        let plays = [
            (0.0, 175.0),
            (180.0, 375.0),
            (380.0, 625.0),
            (630.0, 925.0),
            (930.0, 1_125.0),
            (1_725.0, 1_920.0),
        ];
        let around = |repeats: &[(f64, f64)]| mix(&plays, 1_920.0, Some(repeats));
        let two = around(&[(1_150.0, 1_300.0), (1_400.0, 1_600.0)]);
        let three = around(&[(1_150.0, 1_300.0), (1_400.0, 1_500.0), (1_550.0, 1_700.0)]);
        let one = around(&[(1_150.0, 1_600.0)]);

        assert_eq!(
            tracks(&two),
            [Some((1, 3, TracksMethod::RepeatsWithinLength))]
        );
        assert_eq!(
            tracks(&three),
            [Some((2, 4, TracksMethod::RepeatsWithinLength))]
        );
        assert_eq!(
            tracks(&one),
            [Some((1, 3, TracksMethod::RepeatsWithinLength))]
        );
    }

    #[test]
    fn each_gap_counts_only_the_stretches_inside_it() {
        // No clean interval: the repeats alone, 1, 2 and 3 records.
        let report = mix(
            &[(100.0, 295.0), (800.0, 995.0)],
            1_500.0,
            Some(&[
                (10.0, 50.0),
                (300.0, 340.0),
                (400.0, 450.0),
                (1_000.0, 1_100.0),
                (1_150.0, 1_250.0),
                (1_300.0, 1_450.0),
            ]),
        );

        assert_eq!(
            tracks(&report),
            [
                Some((1, 2, TracksMethod::Repeats)),
                Some((1, 3, TracksMethod::Repeats)),
                Some((2, 4, TracksMethod::Repeats))
            ]
        );
    }

    #[test]
    fn the_methods_are_named_in_json_as_the_core_names_them() {
        for (method, named) in [
            (
                TracksMethod::RepeatsWithinLength,
                Method::RepeatsWithinLength,
            ),
            (TracksMethod::Repeats, Method::Repeats),
        ] {
            assert_eq!(
                serde_json::to_value(&method).ok(),
                Some(serde_json::Value::from(named.name()))
            );
        }
    }

    #[test]
    fn a_gap_without_a_repeated_stretch_has_no_estimate() {
        let report = entries_around_a_gap(2, &[]);

        assert_eq!(tracks(&report), [None]);
        assert_eq!(
            gaps(&report)[0].label(),
            "ID - ID (6:45 without a confident match)"
        );
    }

    #[test]
    fn a_report_saved_without_gap_repeats_has_no_estimate() {
        let mut report = entries_around_a_gap(2, &[(820.0, 900.0)]);
        report.gap_repeats = None;

        assert_eq!(tracks(&report), [None]);
    }

    /// Beeps of a fixed pattern for `seconds`.
    fn beeps(seconds: f64, rate: f64) -> Vec<f32> {
        (0..(seconds * rate) as usize)
            .map(|n| {
                let t = n as f64 / rate;
                let pitch = 300.0 + 150.0 * ((t * 3.0).floor() % 7.0);
                ((2.0 * std::f64::consts::PI * pitch * t).sin()
                    * if (t * 4.0).fract() < 0.5 { 0.5 } else { 0.0 }) as f32
            })
            .collect()
    }

    #[test]
    fn repeats_are_found_in_the_gaps_slice_of_the_samples_in_the_recordings_time() {
        // `--start 10:00 --duration 2:00`, nothing found: one gap. Its
        // audio is 20 s of silence, then a passage of 12 s played three
        // times.
        let profile = Profile::CURRENT;
        let rate = f64::from(profile.sample_rate);
        let mut report = mix(&[], 120.0, None);
        report.query.start_seconds = 600.0;
        let samples: Vec<f32> = [
            vec![0.0; (20.0 * rate) as usize],
            beeps(12.0, rate),
            beeps(12.0, rate),
            beeps(12.0, rate),
            vec![0.0; (64.0 * rate) as usize],
        ]
        .concat();

        let found = find_repeats(&report, &samples, &profile, 2);

        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            (615.0..625.0).contains(&found[0].start_seconds)
                && (650.0..660.0).contains(&found[0].end_seconds),
            "{found:?}"
        );
        report.gap_repeats = Some(found);
        assert_eq!(tracks(&report), [Some((1, 2, TracksMethod::Repeats))]);
    }
}

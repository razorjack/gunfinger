//! The formats a report can be written in.

use std::cell::RefCell;
use std::collections::HashMap;

use clap::ValueEnum;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::export;
use crate::gaps::{Gap, gaps};
use crate::names::TrackName;
use crate::report::Report;
use crate::style::Style;
use crate::table::table;
use crate::timeline::timeline;

#[derive(Clone, Copy, ValueEnum)]
pub enum ReportFormat {
    /// A numbered list of the tracks with their start times, named from
    /// the files' tags, with the other names their files give; an
    /// unnumbered "ID - ID" line marks each stretch of 45 s or more without
    /// a confident match.
    Tracklist,
    /// A table of every play: its time, the part of the track heard, speed,
    /// confidence, hits and file path, with its segments; and a row for
    /// each stretch of 45 s or more without a confident match.
    Human,
    /// Each play as a bar across the recording.
    Timeline,
    /// The whole report, with a `schema_version` and the stretches without
    /// a confident match (`gaps`); `gunfinger show` reads it.
    Json,
    /// One row per play, for spreadsheets.
    Csv,
    /// A cue sheet of the confident tracks, named from their tags.
    Cue,
}

impl ReportFormat {
    /// Formats that can follow one another on stdout under a header.
    pub fn is_for_people(self) -> bool {
        matches!(
            self,
            ReportFormat::Human | ReportFormat::Timeline | ReportFormat::Tracklist
        )
    }
}

/// `style` colours the table, the timeline and nothing else: the tracklist
/// stays plain text to be copied.
pub fn render(report: &Report, format: ReportFormat, style: Style) -> miette::Result<String> {
    // Names are asked for several times per entry; a file is read once.
    let read = RefCell::new(HashMap::new());
    let name = |asset: &str| {
        let stored = report.plays.iter().find_map(|play| {
            if play.asset == asset {
                play.tags.as_ref()
            } else {
                play.same_audio_tags.get(asset)
            }
        });
        match stored {
            Some(tags) => TrackName::named(tags.artist.as_deref(), tags.title.as_deref(), asset),
            None => read
                .borrow_mut()
                .entry(asset.to_owned())
                .or_insert_with(|| TrackName::from_file(&report.library, asset))
                .clone(),
        }
    };
    Ok(match format {
        ReportFormat::Human => table(report, style),
        ReportFormat::Timeline => timeline(report, style),
        ReportFormat::Json => {
            let with_gaps = WithGaps {
                report,
                gaps: gaps(report),
            };
            serde_json::to_string_pretty(&with_gaps).into_diagnostic()? + "\n"
        }
        ReportFormat::Csv => export::csv(report),
        ReportFormat::Cue => export::cue(report, name),
        ReportFormat::Tracklist => export::tracklist(report, name),
    })
}

/// The JSON report with its gaps, derived whenever it is written; reading
/// a report passes over them (ADR 0023).
#[derive(Serialize)]
struct WithGaps<'a> {
    #[serde(flatten)]
    report: &'a Report,
    gaps: Vec<Gap>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::{Level, PlayTags};
    use crate::style::ColorChoice;
    use crate::table::tests::report;

    #[test]
    fn tracklists_name_plays_from_the_tags_in_the_report() {
        let mut report = report();
        report.plays[0].tags = Some(PlayTags {
            artist: Some(String::from("Artist A")),
            title: Some(String::from("Track A")),
            album: None,
        });
        report.plays[1].tags = Some(PlayTags::default());
        report.plays[0]
            .same_audio_tags
            .insert("copy-of-a.wav".into(), PlayTags::default());

        let tracklist = render(
            &report,
            ReportFormat::Tracklist,
            Style::for_stdout(ColorChoice::Never),
        )
        .ok();

        assert_eq!(
            tracklist.as_deref(),
            Some(
                " 1.    0:00  Artist A - Track A\n 2.    0:38  b\n 3.    1:24  insert (possible)\n 4.    1:36  c\n"
            )
        );
    }

    #[test]
    fn the_json_report_lists_its_gaps_and_reads_back_without_them() {
        let mut report = report();
        // b.wav becomes possible (play 2) and the recording runs to 4:00.
        report.plays[1].confidence = Level::Possible;
        report.plays[1].segments[0].confidence = Level::Possible;
        report.query.duration_seconds = 240.0;

        let json = render(
            &report,
            ReportFormat::Json,
            Style::for_stdout(ColorChoice::Never),
        )
        .unwrap_or_default();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap_or_default();

        assert_eq!(value["schema_version"], 3);
        assert_eq!(
            value["gaps"],
            serde_json::json!([
                { "start_seconds": 41.0, "end_seconds": 96.0, "possible_plays": [2, 3] },
                { "start_seconds": 140.0, "end_seconds": 240.0, "possible_plays": [] }
            ])
        );
        let read: Result<Report, _> = serde_json::from_str(&json);
        assert!(read.is_ok_and(|read| read.plays.len() == 4));
    }

    #[test]
    fn a_json_report_without_gaps_says_so() {
        let json = render(
            &report(),
            ReportFormat::Json,
            Style::for_stdout(ColorChoice::Never),
        )
        .unwrap_or_default();
        let value: serde_json::Value = serde_json::from_str(&json).unwrap_or_default();

        assert_eq!(value["gaps"], serde_json::json!([]));
    }
}

//! The formats a report can be written in.

use clap::ValueEnum;
use miette::IntoDiagnostic;

use crate::export;
use crate::names::TrackName;
use crate::report::Report;
use crate::style::Style;
use crate::table::table;
use crate::timeline::timeline;

#[derive(Clone, Copy, ValueEnum)]
pub enum ReportFormat {
    /// A table of plays for people.
    Human,
    /// Each play as a bar across the recording.
    Timeline,
    /// The whole report, with a `schema_version`; `gunfinger show` reads it.
    Json,
    /// One row per play, for spreadsheets.
    Csv,
    /// A cue sheet of the confident recordings, named from their tags.
    Cue,
    /// A numbered list of the recordings, named from their tags.
    Tracklist,
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

/// `style` colours the human formats only.
pub fn render(report: &Report, format: ReportFormat, style: Style) -> miette::Result<String> {
    let name = |asset: &str| {
        let stored = report
            .plays
            .iter()
            .find(|play| play.asset == asset)
            .and_then(|play| play.tags.as_ref());
        match stored {
            Some(tags) => TrackName::named(tags.artist.as_deref(), tags.title.as_deref(), asset),
            None => TrackName::from_file(&report.library, asset),
        }
    };
    Ok(match format {
        ReportFormat::Human => table(report, style),
        ReportFormat::Timeline => timeline(report, style),
        ReportFormat::Json => serde_json::to_string_pretty(report).into_diagnostic()? + "\n",
        ReportFormat::Csv => export::csv(report),
        ReportFormat::Cue => export::cue(report, name),
        ReportFormat::Tracklist => export::tracklist(report, name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::report::PlayTags;
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
}

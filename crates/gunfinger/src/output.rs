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
    /// A numbered list of the tracks with their start times, named from
    /// the files' tags, with the other names their files give.
    Tracklist,
    /// A table of every play: its time, the part of the track heard, speed,
    /// confidence, hits and file path, with its segments.
    Human,
    /// Each play as a bar across the recording.
    Timeline,
    /// The whole report, with a `schema_version`; `gunfinger show` reads it.
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
}

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
    /// A cue sheet of the confident recordings, named from the files' tags.
    Cue,
    /// A numbered list of the recordings, named from the files' tags.
    Tracklist,
}

/// `style` colours the human formats only.
pub fn render(report: &Report, format: ReportFormat, style: Style) -> miette::Result<String> {
    let name = |asset: &str| TrackName::from_tags(&report.library, asset);
    Ok(match format {
        ReportFormat::Human => table(report, style),
        ReportFormat::Timeline => timeline(report, style),
        ReportFormat::Json => serde_json::to_string_pretty(report).into_diagnostic()? + "\n",
        ReportFormat::Csv => export::csv(report),
        ReportFormat::Cue => export::cue(report, name),
        ReportFormat::Tracklist => export::tracklist(report, name),
    })
}

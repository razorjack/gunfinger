//! The `pair` and `parts` commands: alignments of listed pairs, and files
//! that may hold more than one recording.

use std::path::PathBuf;

use super::clusters::other_ignore_file;
use crate::files::{read_lines, read_text, stem_or, write_json};
use crate::harness::Harness;
use crate::padding::current_sources;
use crate::{pair, parts};

#[derive(clap::Args)]
pub struct PairArgs {
    /// A file of pairs, one per line: two paths separated by a tab,
    /// corpus library paths or `second-library/<path>` with
    /// --other-peaks-dir. The report is `pairs-<file stem>.json`.
    pub pairs: PathBuf,
    /// Also search, with supported seconds, the rungs 0.1% apart
    /// within 1% of the fitted speed (`peaks, fine rung -1.0%` to
    /// `+1.0%`): a copy whose speed drifts loses its hashes away from
    /// the fitted speed.
    #[arg(long)]
    pub fine_rungs: bool,
}

#[derive(clap::Args)]
pub struct PartsArgs {
    /// The files to search, one store path of the other library per
    /// line, in order.
    pub queries: PathBuf,
    /// Store paths of files or folders to leave out of the queries,
    /// one per line; they stay in the index.
    #[arg(long)]
    pub leave_out: Option<PathBuf>,
    /// Search each file whole, not its first and last 60 s.
    #[arg(long)]
    pub whole: bool,
    /// The report.
    #[arg(long)]
    pub out: PathBuf,
}

pub fn pair(harness: &Harness, jobs: usize, args: PairArgs) -> Result<(), String> {
    let PairArgs { pairs, fine_rungs } = args;
    let listed: Vec<(String, String)> = read_text(&pairs)?
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.starts_with('#'))
        .map(|line| {
            line.split_once('\t')
                .map(|(query, found)| (query.to_owned(), found.to_owned()))
                .ok_or_else(|| format!("not two tab-separated paths: {line}"))
        })
        .collect::<Result<_, _>>()?;
    let other = harness.other_store()?;
    let reports = pair::run(
        &pair::Sources {
            library: &harness.library()?,
            store: &harness.store()?,
            other: other.as_ref(),
        },
        &listed,
        jobs,
        if fine_rungs {
            pair::FineRungs::Around
        } else {
            pair::FineRungs::Off
        },
    )?;
    let stem = stem_or(&pairs, "pairs");
    write_json(
        &harness.base_reports().join(format!("pairs-{stem}.json")),
        &reports,
    )?;
    pair::print_summary(&reports);
    Ok(())
}

pub fn parts(harness: &Harness, jobs: usize, args: PartsArgs) -> Result<(), String> {
    let PartsArgs {
        queries: queries_file,
        leave_out: leave_out_file,
        whole,
        out,
    } = args;
    let Some(other) = harness.other_store()? else {
        return Err(String::from("parts needs --other-peaks-dir"));
    };
    let queries = read_lines(&queries_file)?;
    let leave_out = leave_out_file
        .as_deref()
        .map(read_lines)
        .transpose()?
        .unwrap_or_default();
    let assets = current_sources(&other, jobs)?;
    let ignore = other_ignore_file(&other)?.map(|file| file.patterns);
    let left_out = parts::left_out(
        &queries,
        &assets,
        ignore.as_deref().unwrap_or(&[]),
        &leave_out,
    );
    let searched = if whole {
        parts::Searched::Whole
    } else {
        parts::Searched::Ends
    };
    let report = parts::find(
        &other,
        &assets,
        &harness.clusters()?,
        &queries,
        left_out,
        searched,
        jobs,
    )?;
    write_json(&out, &report)?;
    parts::print_summary(&report);
    println!("wrote {}", out.display());
    Ok(())
}

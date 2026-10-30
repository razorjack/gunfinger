//! The `sweep` command and its options for studying the fullest-list filter.

use std::path::PathBuf;

use crate::files::{read_lines, write_json};
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::library_map::LibraryMap;
use crate::padding::{ExactCopies, Padding};
use crate::{fullest, sweep};

#[derive(clap::Args)]
pub struct SweepArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    /// With --other-peaks-dir: search only the indexed excerpts whose
    /// recording has another rip that is not an identical copy, with
    /// each such excerpt's source file and its identical copies left
    /// out of the index, so that only the other rips can answer
    /// (`sweep-seed-<seed>-other-rips.json`).
    #[arg(long)]
    pub other_rips: bool,
    /// The speeds played: the sweep's own, within the ladder, or
    /// `edge`, ±8.4% to ±12% (session 51; `-edge` in the report's
    /// name).
    #[arg(long, value_enum, default_value_t = sweep::Speeds::Sweep)]
    pub speeds: sweep::Speeds,
    #[command(flatten)]
    pub filter: FilterStudy,
}

pub fn sweep(harness: &Harness, jobs: usize, args: SweepArgs) -> Result<(), String> {
    let SweepArgs {
        seed,
        other_rips,
        speeds,
        filter,
    } = args;
    run_sweep(harness, seed, other_rips, speeds, &filter, jobs)
}

/// Sweep options for measuring the fullest-list filter (session 50). Each
/// adds to the report's name, so no default report is written over.
#[derive(clap::Args)]
pub struct FilterStudy {
    /// Keep each query's first-pass candidates (detections of at least
    /// 10 hits, which the second pass measures again) and the lists the
    /// searched index sets aside; the report's name ends in `-trace`.
    #[arg(long)]
    trace_candidates: bool,
    /// Set the fullest lists aside ranked by distinct files or recordings
    /// holding them instead of postings (default matcher only); the
    /// report's name ends in `-rank-<ranking>`.
    #[arg(long, value_enum)]
    rank_fullest_by: Option<fullest::Ranking>,
    /// Search the index with the lists set aside that the harness's index
    /// saved in this file sets aside with the same files left out, and
    /// record how the two differ: for comparing with an index of an
    /// earlier store (session 64). The file is only read. Default matcher
    /// only; the report's name ends in `-set-aside-like`.
    #[arg(long, value_name = "FILE", conflicts_with = "rank_fullest_by")]
    set_aside_like: Option<PathBuf>,
    #[command(flatten)]
    copies: ExactCopyOptions,
}

/// Exact copies of chosen records added to the index (session 50).
#[derive(clap::Args)]
pub struct ExactCopyOptions {
    /// Add this many exact copies of each record the file `--copied`
    /// names to the index, in memory (`padding-copy/<i>/<path>`, counted
    /// as their original); the report's name ends in
    /// `-exact-<count>-<file stem>`.
    #[arg(long, requires = "copied")]
    exact_copies: Option<usize>,
    /// The paths of the records to copy, one per line, as the index names
    /// them (`second-library/<path>` for the other library's).
    #[arg(long, requires = "exact_copies")]
    copied: Option<PathBuf>,
}

pub const NO_FILTER_STUDY: FilterStudy = FilterStudy {
    trace_candidates: false,
    rank_fullest_by: None,
    set_aside_like: None,
    copies: ExactCopyOptions {
        exact_copies: None,
        copied: None,
    },
};

impl ExactCopyOptions {
    pub fn exact(&self) -> Result<Option<ExactCopies>, String> {
        let (Some(count), Some(file)) = (self.exact_copies, &self.copied) else {
            return Ok(None);
        };
        Ok(Some(ExactCopies {
            count,
            chosen: read_lines(file)?.into_iter().collect(),
        }))
    }

    pub fn suffix(&self) -> String {
        let (Some(count), Some(file)) = (self.exact_copies, &self.copied) else {
            return String::new();
        };
        let stem = file
            .file_stem()
            .map_or_else(String::new, |stem| stem.to_string_lossy().into_owned());
        format!("-exact-{count}-{stem}")
    }
}

impl FilterStudy {
    fn suffix(&self) -> String {
        let mut suffix = self.copies.suffix();
        if let Some(ranking) = self.rank_fullest_by {
            suffix.push_str(&format!("-rank-{}", ranking.name()));
        }
        if self.set_aside_like.is_some() {
            suffix.push_str("-set-aside-like");
        }
        if self.trace_candidates {
            suffix.push_str("-trace");
        }
        suffix
    }
}

pub fn run_sweep(
    harness: &Harness,
    seed: u64,
    other_rips: bool,
    speeds: sweep::Speeds,
    filter: &FilterStudy,
    jobs: usize,
) -> Result<(), String> {
    harness.check_panel(seed)?;
    if filter.rank_fullest_by.is_some() && harness.search.matching.name().is_some() {
        return Err(String::from(
            "--rank-fullest-by ranks the default matcher's lists; it takes no other matching option",
        ));
    }
    if filter.set_aside_like.is_some() && harness.search.matching.name().is_some() {
        return Err(String::from(
            "--set-aside-like sets aside the default matcher's lists; it takes no other matching option",
        ));
    }
    let padding = Padding {
        exact: filter.copies.exact()?,
        ..harness.padding(&UNCHANGED_INDEX)?
    };
    let map = match (other_rips, harness.map_file()) {
        (false, _) => None,
        (true, Some(map_file)) => Some(LibraryMap::load(&map_file)?),
        (true, None) => return Err(String::from("--other-rips needs --other-peaks-dir")),
    };
    let report = sweep::run(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        &harness.paths.work,
        &sweep::Options {
            same_titles: &harness.same_titles()?,
            seed,
            panels: &harness.paths.panels,
            padding: &padding,
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            verify: harness.search.verify,
            other_rips: map.as_ref(),
            trace: if filter.trace_candidates {
                sweep::Trace::Candidates
            } else {
                sweep::Trace::Off
            },
            ranking: filter.rank_fullest_by,
            set_aside_like: filter.set_aside_like.as_deref(),
            speeds,
            jobs,
        },
    )?;
    let other_rips = if other_rips { "-other-rips" } else { "" };
    let edge = match speeds {
        sweep::Speeds::Sweep => "",
        sweep::Speeds::Edge => "-edge",
    };
    let name = format!(
        "sweep-seed-{seed}{other_rips}{edge}{}.json",
        filter.suffix()
    );
    write_json(&harness.reports().join(name), &report)?;
    sweep::print_summary(&report);
    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        filter: FilterStudy,
    }

    fn filter(arguments: &[&str]) -> Result<FilterStudy, String> {
        Cli::try_parse_from([&["sweep"], arguments].concat())
            .map(|cli| cli.filter)
            .map_err(|error| error.to_string())
    }

    #[test]
    fn a_sweep_set_aside_like_a_saved_index_writes_a_report_of_its_own() {
        let like = filter(&["--set-aside-like", "old.index", "--trace-candidates"]).unwrap();

        assert_eq!(like.suffix(), "-set-aside-like-trace");
        assert_eq!(filter(&[]).unwrap().suffix(), "");
        assert!(
            filter(&[
                "--set-aside-like",
                "old.index",
                "--rank-fullest-by",
                "files"
            ])
            .is_err_and(|message| message.contains("cannot be used with"))
        );
    }
}

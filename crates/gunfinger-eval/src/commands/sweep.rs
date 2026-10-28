//! The `sweep` command and its options for studying the fullest-list filter.

use std::path::PathBuf;

use crate::files::{read_lines, write_json};
use crate::library_map::LibraryMap;
use crate::padding::{ExactCopies, Padding};
use crate::{Paths, UNCHANGED_INDEX};
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

pub fn sweep(paths: &Paths, jobs: usize, args: SweepArgs) -> Result<(), String> {
    let SweepArgs {
        seed,
        other_rips,
        speeds,
        filter,
    } = args;
    run_sweep(paths, seed, other_rips, speeds, &filter, jobs)
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
    #[command(flatten)]
    copies: Copies,
}

/// Exact copies of chosen records added to the index (session 50).
#[derive(clap::Args)]
pub struct Copies {
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
    copies: Copies {
        exact_copies: None,
        copied: None,
    },
};

impl Copies {
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
        if self.trace_candidates {
            suffix.push_str("-trace");
        }
        suffix
    }
}

pub fn run_sweep(
    paths: &Paths,
    seed: u64,
    other_rips: bool,
    speeds: sweep::Speeds,
    filter: &FilterStudy,
    jobs: usize,
) -> Result<(), String> {
    paths.check_panel(seed)?;
    if filter.rank_fullest_by.is_some() && paths.matching.name().is_some() {
        return Err(String::from(
            "--rank-fullest-by ranks the default matcher's lists; it takes no other matching option",
        ));
    }
    let padding = Padding {
        exact: filter.copies.exact()?,
        ..paths.padding(&UNCHANGED_INDEX)?
    };
    let map = match (other_rips, paths.map_file()) {
        (false, _) => None,
        (true, Some(map_file)) => Some(LibraryMap::load(&map_file)?),
        (true, None) => return Err(String::from("--other-rips needs --other-peaks-dir")),
    };
    let report = sweep::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &sweep::Options {
            same_titles: &paths.same_titles()?,
            seed,
            panels: &paths.panels,
            padding: &padding,
            ladder: &paths.rungs(),
            matching: &paths.matching,
            verify: paths.verify,
            other_rips: map.as_ref(),
            trace: if filter.trace_candidates {
                sweep::Trace::Candidates
            } else {
                sweep::Trace::Off
            },
            ranking: filter.rank_fullest_by,
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
    write_json(&paths.reports().join(name), &report)?;
    sweep::print_summary(&report);
    Ok(())
}

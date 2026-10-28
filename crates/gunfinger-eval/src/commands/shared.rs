//! The shared-material commands: `shared`, `shared-mixes`, `holders`,
//! `identifiability` and `versions`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use gunfinger_core::store::PeakStore;

use crate::files::{read_json, stem_or, write_json};
use crate::library_map::LibraryMap;
use crate::padding::SECOND_LIBRARY_PREFIX;
use crate::{Paths, UNCHANGED_INDEX};
use crate::{holders, identifiability, pair, shared, shared_mixes, versions};

#[derive(clap::Args)]
pub struct SharedArgs {
    /// The scenarios: JSON with `scenarios`, each naming `name`,
    /// `played`, `related`, `passage_start_seconds`,
    /// `passage_end_seconds` and optionally `played_recording` and
    /// `related_recording` (files of each recording beyond its
    /// duplicate cluster), and optionally `blend_partner` (`file`,
    /// `start_seconds`). The report is `<plan file stem>.json`, with
    /// `shared-` put first when the stem does not start with it.
    pub plan: PathBuf,
}

#[derive(clap::Args)]
pub struct SharedMixesArgs {
    /// The plan, as `scripts/analysis/shared_mixes.py plan` writes it.
    pub plan: PathBuf,
}

#[derive(clap::Args)]
pub struct HoldersArgs {
    /// JSON: a list of `name`, `asset`, `track_start_seconds`,
    /// `track_end_seconds` and optionally `left_out`.
    pub lookups: PathBuf,
}

#[derive(clap::Args)]
pub struct IdentifiabilityArgs {
    /// The panel, as `scripts/analysis/shared_panel.py` writes it.
    pub panel: PathBuf,
    #[arg(long, value_enum)]
    pub half: Half,
}

#[derive(clap::Args)]
pub struct VersionsArgs {
    /// JSON: `plays`, as `scripts/analysis/version_plays.py` writes them.
    pub plays: PathBuf,
    /// Where to write the timelines and summaries.
    #[arg(long)]
    pub out: PathBuf,
}

/// The plan's scenarios, rendered into `<work>/shared/<plan name>-<render
/// key>/` (`shared::render_key`); the report is `<plan name>.json`, with
/// `shared-` put first when the name does not start with it.
pub fn shared(paths: &Paths, jobs: usize, args: SharedArgs) -> Result<(), String> {
    let SharedArgs { plan: plan_file } = args;
    let plan: shared::Plan = read_json(&plan_file)?;
    let name = stem_or(&plan_file, "shared-material");
    let second_root = second_library_root(paths.other_store()?.as_ref())?;
    let library = paths.library()?;
    let renders = paths
        .work
        .join("shared")
        .join(format!("{name}-{}", shared::render_key(&plan)));
    eprintln!("queries rendered in {}", renders.display());
    let report = shared::run(
        &plan,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        &renders,
        &shared::Options {
            ladder: &paths.rungs(),
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            jobs,
        },
    )?;
    let report_name = if name.starts_with("shared") {
        name
    } else {
        format!("shared-{name}")
    };
    write_json(
        &paths.reports().join(format!("{report_name}.json")),
        &report,
    )?;
    shared::print_summary(&report);
    Ok(())
}

/// The plan's mixes, rendered into `<work>/shared-mixes/<plan name>/`
/// (each kept while its truth file matches); the report is `<plan
/// name>.json`.
pub fn shared_mixes(paths: &Paths, jobs: usize, args: SharedMixesArgs) -> Result<(), String> {
    let SharedMixesArgs { plan: plan_file } = args;
    let plan: shared_mixes::Plan = read_json(&plan_file)?;
    let name = stem_or(&plan_file, "shared-mixes");
    let second_root = second_library_root(paths.other_store()?.as_ref())?;
    let library = paths.library()?;
    let report = shared_mixes::run(
        &plan,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        &paths.work.join("shared-mixes").join(&name),
        &shared_mixes::Options {
            ladder: &paths.rungs(),
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            sets: &paths.sets(),
            jobs,
        },
    )?;
    write_json(&paths.reports().join(format!("{name}.json")), &report)?;
    shared_mixes::print_summary(&report);
    Ok(())
}

/// The holders of each lookup's span; the report is `holders-<file
/// stem>.json`.
pub fn holders(paths: &Paths, jobs: usize, args: HoldersArgs) -> Result<(), String> {
    let HoldersArgs {
        lookups: lookups_file,
    } = args;
    let lookups: Vec<holders::Lookup> = read_json(&lookups_file)?;
    let name = stem_or(&lookups_file, "lookups");
    let stood_for = match paths.map_file() {
        Some(map_file) => LibraryMap::load(&map_file)?.standing_corpus_files(SECOND_LIBRARY_PREFIX),
        None => BTreeMap::new(),
    };
    let looked = holders::run(
        &lookups,
        &paths.library()?,
        &paths.store()?,
        &holders::Options {
            matching: &paths.matching,
            padding: &paths.padding(&UNCHANGED_INDEX)?,
            stood_for: &stood_for,
            jobs,
        },
    )?;
    write_json(
        &paths.reports().join(format!("holders-{name}.json")),
        &looked,
    )?;
    eprintln!(
        "{} lookups, {} without peaks at hand",
        looked.len(),
        looked
            .iter()
            .filter(|looked| looked.error.is_some())
            .count()
    );
    Ok(())
}

pub fn identifiability(paths: &Paths, args: IdentifiabilityArgs) -> Result<(), String> {
    let IdentifiabilityArgs {
        panel: panel_file,
        half,
    } = args;
    let panel: identifiability::Panel = read_json(&panel_file)?;
    let other = paths.other_store()?;
    let second_root = second_library_root(other.as_ref())?;
    let library = paths.library()?;
    let report = identifiability::run(
        &panel,
        half.name(),
        &pair::Sources {
            library: &library,
            store: &paths.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
    )?;
    write_json(
        &paths
            .base_reports()
            .join(format!("identifiability-{}.json", half.name())),
        &report,
    )?;
    identifiability::print_summary(&report);
    Ok(())
}

pub fn versions(paths: &Paths, jobs: usize, args: VersionsArgs) -> Result<(), String> {
    let VersionsArgs { plays, out } = args;
    let list: versions::PlayList = read_json(&plays)?;
    let other = paths.other_store()?;
    let second_root = second_library_root(other.as_ref())?;
    let library = paths.library()?;
    let verified = versions::run(
        &list.plays,
        &pair::Sources {
            library: &library,
            store: &paths.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            second: second_root.as_deref(),
        },
        jobs,
    )?;
    write_json(&out, &verified)?;
    versions::print_summary(&verified);
    Ok(())
}

/// A half of a shared-material panel.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum Half {
    Development,
    Validation,
}

impl Half {
    fn name(self) -> &'static str {
        match self {
            Half::Development => "development",
            Half::Validation => "validation",
        }
    }
}

/// The root of the other library's files, as its store names it.
fn second_library_root(other: Option<&PeakStore>) -> Result<Option<PathBuf>, String> {
    let Some(other) = other else {
        return Ok(None);
    };
    let root = other
        .library()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| {
            format!(
                "{} names no library, so the files there cannot be found",
                other.dir().display()
            )
        })?;
    Ok(Some(PathBuf::from(root)))
}

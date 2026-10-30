//! The shared-material commands: `shared`, `shared-mixes`, `holders`,
//! `identifiability` and `versions`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use gunfinger_core::store::PeakStore;

use crate::files::{read_json, stem_or, write_json};
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::library_map::LibraryMap;
use crate::padding::OTHER_LIBRARY_PREFIX;
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
    /// Also measure each detection of 20 hits or more in the harmonic
    /// view (brief 66; `harmonic.rs`), with bass-killed and damaged
    /// queries added; the report is `<name>-harmonic.json`.
    #[arg(long)]
    pub harmonic: bool,
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
    pub half: PanelHalf,
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
pub fn shared(harness: &Harness, jobs: usize, args: SharedArgs) -> Result<(), String> {
    let SharedArgs {
        plan: plan_file,
        harmonic,
    } = args;
    let views = if harmonic {
        shared::Views::Harmonic
    } else {
        shared::Views::Off
    };
    let plan: shared::Plan = read_json(&plan_file)?;
    let name = stem_or(&plan_file, "shared-material");
    let other_root = other_library_root(harness.other_store()?.as_ref())?;
    let library = harness.library()?;
    let renders = harness
        .paths
        .work
        .join("shared")
        .join(format!("{name}-{}", shared::render_key(&plan, views)));
    eprintln!("queries rendered in {}", renders.display());
    let report = shared::run(
        &plan,
        &library,
        &harness.store()?,
        &harness.clusters()?,
        &shared::Roots {
            library: &library.root,
            other: other_root.as_deref(),
        },
        &renders,
        &shared::Options {
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            jobs,
            views,
        },
    )?;
    let report_name = if name.starts_with("shared") {
        name
    } else {
        format!("shared-{name}")
    };
    let report_name = match views {
        shared::Views::Off => report_name,
        shared::Views::Harmonic => format!("{report_name}-harmonic"),
    };
    write_json(
        &harness.reports().join(format!("{report_name}.json")),
        &report,
    )?;
    shared::print_summary(&report);
    Ok(())
}

/// The plan's mixes, rendered into `<work>/shared-mixes/<plan name>/`
/// (each kept while its truth file matches); the report is `<plan
/// name>.json`.
pub fn shared_mixes(harness: &Harness, jobs: usize, args: SharedMixesArgs) -> Result<(), String> {
    let SharedMixesArgs { plan: plan_file } = args;
    let plan: shared_mixes::Plan = read_json(&plan_file)?;
    let name = stem_or(&plan_file, "shared-mixes");
    let other_root = other_library_root(harness.other_store()?.as_ref())?;
    let library = harness.library()?;
    let report = shared_mixes::run(
        &plan,
        &library,
        &harness.store()?,
        &harness.clusters()?,
        &shared::Roots {
            library: &library.root,
            other: other_root.as_deref(),
        },
        &harness.paths.work.join("shared-mixes").join(&name),
        &shared_mixes::Options {
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            sets: &harness.sets(),
            jobs,
        },
    )?;
    write_json(&harness.reports().join(format!("{name}.json")), &report)?;
    shared_mixes::print_summary(&report);
    Ok(())
}

/// The holders of each lookup's span; the report is `holders-<file
/// stem>.json`.
pub fn holders(harness: &Harness, jobs: usize, args: HoldersArgs) -> Result<(), String> {
    let HoldersArgs {
        lookups: lookups_file,
    } = args;
    let lookups: Vec<holders::Lookup> = read_json(&lookups_file)?;
    let name = stem_or(&lookups_file, "lookups");
    let stood_for = match harness.map_file() {
        Some(map_file) => LibraryMap::load(&map_file)?.standing_corpus_files(OTHER_LIBRARY_PREFIX),
        None => BTreeMap::new(),
    };
    let looked = holders::run(
        &lookups,
        &harness.library()?,
        &harness.store()?,
        &holders::Options {
            matching: &harness.search.matching,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            stood_for: &stood_for,
            jobs,
        },
    )?;
    write_json(
        &harness.reports().join(format!("holders-{name}.json")),
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

pub fn identifiability(harness: &Harness, args: IdentifiabilityArgs) -> Result<(), String> {
    let IdentifiabilityArgs {
        panel: panel_file,
        half,
    } = args;
    let panel: identifiability::Panel = read_json(&panel_file)?;
    let other = harness.other_store()?;
    let other_root = other_library_root(other.as_ref())?;
    let library = harness.library()?;
    let report = identifiability::run(
        &panel,
        half.name(),
        &pair::Sources {
            library: &library,
            store: &harness.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            other: other_root.as_deref(),
        },
    )?;
    write_json(
        &harness
            .base_reports()
            .join(format!("identifiability-{}.json", half.name())),
        &report,
    )?;
    identifiability::print_summary(&report);
    Ok(())
}

pub fn versions(harness: &Harness, jobs: usize, args: VersionsArgs) -> Result<(), String> {
    let VersionsArgs { plays, out } = args;
    let list: versions::PlayList = read_json(&plays)?;
    let other = harness.other_store()?;
    let other_root = other_library_root(other.as_ref())?;
    let library = harness.library()?;
    let verified = versions::run(
        &list.plays,
        &pair::Sources {
            library: &library,
            store: &harness.store()?,
            other: other.as_ref(),
        },
        &shared::Roots {
            library: &library.root,
            other: other_root.as_deref(),
        },
        jobs,
    )?;
    write_json(&out, &verified)?;
    versions::print_summary(&verified);
    Ok(())
}

/// A half of a shared-material panel.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum PanelHalf {
    Development,
    Validation,
}

impl PanelHalf {
    fn name(self) -> &'static str {
        match self {
            PanelHalf::Development => "development",
            PanelHalf::Validation => "validation",
        }
    }
}

/// The root of the other library's files, as its store names it.
fn other_library_root(other: Option<&PeakStore>) -> Result<Option<PathBuf>, String> {
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

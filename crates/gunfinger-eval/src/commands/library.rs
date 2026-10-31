//! Commands that measure the library itself: manifests, hashes, the map of
//! the other library, related recordings, stereo width and the fullest
//! posting lists.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use gunfinger_core::library::ignore;
use gunfinger_core::store::PeakStore;

use super::sweep::ExactCopyOptions;
use crate::files::write_json;
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::padding::{Padding, current_sources};
use crate::{fullest, hash_cost, library_map, manifest, related, stereo, survival};

#[derive(clap::Args)]
pub struct ValidateArgs {
    /// Check only this set (a directory under `sets/`).
    pub set: Option<String>,
}

#[derive(clap::Args)]
pub struct SurvivalArgs {
    /// Library assets to measure, relative to the library root.
    pub assets: Vec<String>,
}

#[derive(Clone, Copy, clap::Args)]
pub struct HashCostArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
}

#[derive(clap::Args)]
pub struct StereoWidthArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    #[arg(long, default_value_t = 100)]
    pub sample: usize,
    /// Sample the other library (--other-peaks-dir) instead: its files
    /// with a current record, an equal share of each kind of source,
    /// leaving out what its ignore file and --exclude leave out. Its
    /// audio is only read.
    #[arg(long)]
    pub other: bool,
    /// Files of the other library left out of its sample, one path per
    /// line; a folder's path ends in `/`.
    #[arg(long, requires = "other")]
    pub exclude: Option<PathBuf>,
    /// Measure every MP3 under this folder instead (faulted renders).
    #[arg(long, conflicts_with = "other")]
    pub files: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct VariantStoreArgs {
    /// The store to write, never the library's own.
    #[arg(long)]
    pub out: PathBuf,
    /// Files decoded in place of library files: a library path, a tab and
    /// the file, one per line (a faulted copy of a rip).
    #[arg(long)]
    pub replace: Option<PathBuf>,
    /// Check each file's two channels and, where one is inverted (brief
    /// 67), pick its peaks from the downmix with that channel's polarity
    /// reversed; every other record is today's.
    #[arg(long)]
    pub repair_inverted: bool,
}

#[derive(clap::Args)]
pub struct FullestArgs {
    /// The share of non-empty lists, as for `--skip-fullest`.
    #[arg(long, default_value_t = 0.01)]
    pub share: f64,
    #[command(flatten)]
    pub copies: ExactCopyOptions,
    /// Pairs of files, two index paths per line separated by a tab:
    /// count the hashes both hold and those set aside (session 50).
    #[arg(long)]
    pub shared_by: Option<PathBuf>,
}

pub fn validate(harness: &Harness, args: ValidateArgs) -> Result<(), String> {
    let ValidateArgs { set } = args;
    manifest::validate(
        &harness.sets(),
        set.as_deref(),
        &harness.referable(&harness.library()?)?,
    )
}

pub fn survival(harness: &Harness, args: SurvivalArgs) -> Result<(), String> {
    let SurvivalArgs { assets } = args;
    survival::run(
        &harness.library()?,
        &harness.store()?,
        &assets,
        &harness.paths.work,
    )
}

pub fn hash_cost(harness: &Harness, jobs: usize, args: HashCostArgs) -> Result<(), String> {
    let HashCostArgs { seed } = args;
    hash_cost::run(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        seed,
        &harness.paths.panels,
        &harness.paths.work,
        jobs,
    )
}

pub fn map_library(harness: &Harness, jobs: usize) -> Result<(), String> {
    let (Some(other), Some(map_file)) = (harness.other_store()?, harness.map_file()) else {
        return Err(String::from("map-library needs --other-peaks-dir"));
    };
    let other_assets = current_sources(&other, jobs)?;
    let map = library_map::build(
        &harness.library()?,
        &harness.store()?,
        &other,
        &other_assets,
    )?;
    write_json(&map_file, &map)?;
    library_map::print_summary(&map);
    Ok(())
}

pub fn related(harness: &Harness, jobs: usize) -> Result<(), String> {
    let related = related::find(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        jobs,
    )?;
    write_json(&harness.reports().join("related-recordings.json"), &related)?;
    related::print_summary(&related);
    Ok(())
}

pub fn stereo_width(harness: &Harness, jobs: usize, args: StereoWidthArgs) -> Result<(), String> {
    let StereoWidthArgs {
        seed,
        sample,
        other,
        exclude,
        files,
    } = args;
    if harness.other.other_peaks_dir.is_some() && !other {
        return Err(String::from(
            "stereo-width reads --other-peaks-dir only with --other",
        ));
    }
    let (report, name) = if let Some(dir) = files {
        let name = dir.file_name().map_or_else(
            || String::from("files"),
            |name| name.to_string_lossy().into_owned(),
        );
        (
            stereo::file_widths(&dir, jobs)?,
            format!("stereo-width-files-{name}.json"),
        )
    } else if other {
        (
            other_widths(harness, exclude.as_deref(), seed, sample, jobs)?,
            format!("stereo-width-other-seed-{seed}.json"),
        )
    } else {
        (
            stereo::sample_widths(&harness.library()?, &harness.store()?, seed, sample, jobs)?,
            format!("stereo-width-seed-{seed}.json"),
        )
    };
    write_json(&harness.reports().join(name), &report)?;
    stereo::print_widths(&report);
    Ok(())
}

/// The census of the other library's files (experiment 0248).
fn other_widths(
    harness: &Harness,
    exclude: Option<&Path>,
    seed: u64,
    sample: usize,
    jobs: usize,
) -> Result<stereo::WidthReport, String> {
    let Some(other) = harness.other_store()? else {
        return Err(String::from("stereo-width --other needs --other-peaks-dir"));
    };
    let root = other
        .library()
        .map_err(|error| error.to_string())?
        .ok_or_else(|| format!("{} names no library", other.dir().display()))?;
    let root = Path::new(&root);
    let ignore = ignore::read(root)
        .map_err(|error| error.to_string())?
        .unwrap_or_default();
    let left_out = match exclude {
        Some(list) => stereo::LeftOut::parse(
            &std::fs::read_to_string(list)
                .map_err(|error| format!("cannot read {}: {error}", list.display()))?,
        ),
        None => stereo::LeftOut::parse(""),
    };
    let assets = current_sources(&other, jobs)?;
    stereo::sample_other_widths(&assets, root, &ignore, &left_out, seed, sample, jobs)
}

/// A store of the library's files under `--front-end`'s profile, for the
/// front-end variant's experiments (0245 and after).
pub fn variant_store(harness: &Harness, jobs: usize, args: VariantStoreArgs) -> Result<(), String> {
    let VariantStoreArgs {
        out,
        replace,
        repair_inverted,
    } = args;
    let replacements = match replace {
        Some(list) => read_replacements(&list)?,
        None => BTreeMap::new(),
    };
    let out = PeakStore::open(&out).map_err(|error| error.to_string())?;
    let built = stereo::build_store(
        &harness.library()?,
        &harness.store()?,
        &out,
        harness.search.matching.front_end,
        if repair_inverted {
            stereo::Polarity::RepairInverted
        } else {
            stereo::Polarity::AsDecoded
        },
        &replacements,
        jobs,
    )?;
    println!(
        "{}",
        serde_json::to_string(&built).map_err(|error| error.to_string())?
    );
    Ok(())
}

/// `library path<TAB>file` lines.
fn read_replacements(list: &Path) -> Result<BTreeMap<String, PathBuf>, String> {
    let text = std::fs::read_to_string(list)
        .map_err(|error| format!("cannot read {}: {error}", list.display()))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.split_once('\t')
                .map(|(asset, file)| (asset.to_owned(), PathBuf::from(file)))
                .ok_or_else(|| format!("{}: not `library path<TAB>file`: {line}", list.display()))
        })
        .collect()
}

pub fn fullest(harness: &Harness, args: FullestArgs) -> Result<(), String> {
    let FullestArgs {
        share,
        copies,
        shared_by,
    } = args;
    if !(share > 0.0 && share < 1.0) {
        return Err(String::from("--share is a share above 0 and below 1"));
    }
    let pairs = match &shared_by {
        Some(file) => fullest::read_pairs(file)?,
        None => Vec::new(),
    };
    let report = fullest::run(
        &harness.library()?,
        &harness.store()?,
        &Padding {
            exact: copies.exact()?,
            ..harness.padding(&UNCHANGED_INDEX)?
        },
        share,
        &pairs,
    )?;
    write_json(
        &harness
            .base_reports()
            .join(format!("fullest-{share}{}.json", copies.suffix())),
        &report,
    )?;
    fullest::print_summary(&report);
    Ok(())
}

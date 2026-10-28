//! Commands that measure the library itself: manifests, hashes, the map of
//! the other library, related recordings, stereo width and the fullest
//! posting lists.

use std::path::PathBuf;

use super::sweep::Copies;
use crate::files::write_json;
use crate::padding::{Padding, current_sources};
use crate::{Paths, UNCHANGED_INDEX};
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

#[derive(Clone, Copy, clap::Args)]
pub struct StereoWidthArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    #[arg(long, default_value_t = 100)]
    pub sample: usize,
}

#[derive(clap::Args)]
pub struct FullestArgs {
    /// The share of non-empty lists, as for `--skip-fullest`.
    #[arg(long, default_value_t = 0.01)]
    pub share: f64,
    #[command(flatten)]
    pub copies: Copies,
    /// Pairs of files, two index paths per line separated by a tab:
    /// count the hashes both hold and those set aside (session 50).
    #[arg(long)]
    pub shared_by: Option<PathBuf>,
}

pub fn validate(paths: &Paths, args: ValidateArgs) -> Result<(), String> {
    let ValidateArgs { set } = args;
    manifest::validate(
        &paths.sets(),
        set.as_deref(),
        &paths.referable(&paths.library()?)?,
    )
}

pub fn survival(paths: &Paths, args: SurvivalArgs) -> Result<(), String> {
    let SurvivalArgs { assets } = args;
    survival::run(&paths.library()?, &paths.store()?, &assets, &paths.work)
}

pub fn hash_cost(paths: &Paths, jobs: usize, args: HashCostArgs) -> Result<(), String> {
    let HashCostArgs { seed } = args;
    hash_cost::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        seed,
        &paths.panels,
        &paths.work,
        jobs,
    )
}

pub fn map_library(paths: &Paths, jobs: usize) -> Result<(), String> {
    let (Some(other), Some(map_file)) = (paths.other_store()?, paths.map_file()) else {
        return Err(String::from("map-library needs --other-peaks-dir"));
    };
    let other_assets = current_sources(&other, jobs)?;
    let map = library_map::build(&paths.library()?, &paths.store()?, &other, &other_assets)?;
    write_json(&map_file, &map)?;
    library_map::print_summary(&map);
    Ok(())
}

pub fn related(paths: &Paths, jobs: usize) -> Result<(), String> {
    let related = related::find(&paths.library()?, &paths.store()?, &paths.clusters()?, jobs)?;
    write_json(&paths.reports().join("related-recordings.json"), &related)?;
    related::print_summary(&related);
    Ok(())
}

pub fn stereo_width(paths: &Paths, jobs: usize, args: StereoWidthArgs) -> Result<(), String> {
    let StereoWidthArgs { seed, sample } = args;
    let report = stereo::sample_widths(&paths.library()?, &paths.store()?, seed, sample, jobs)?;
    write_json(
        &paths
            .reports()
            .join(format!("stereo-width-seed-{seed}.json")),
        &report,
    )?;
    stereo::print_widths(&report);
    Ok(())
}

pub fn fullest(paths: &Paths, args: FullestArgs) -> Result<(), String> {
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
        &paths.library()?,
        &paths.store()?,
        &Padding {
            exact: copies.exact()?,
            ..paths.padding(&UNCHANGED_INDEX)?
        },
        share,
        &pairs,
    )?;
    write_json(
        &paths
            .base_reports()
            .join(format!("fullest-{share}{}.json", copies.suffix())),
        &report,
    )?;
    fullest::print_summary(&report);
    Ok(())
}

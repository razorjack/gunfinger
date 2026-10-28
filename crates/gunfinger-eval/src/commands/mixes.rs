//! The `mixes` and `grid` commands: generated mixes with exact truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::Paths;
use crate::files::{read_json, write_json};
use crate::{grid, mixes, robust};

#[derive(clap::Args)]
pub struct MixesArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    #[arg(long, default_value_t = 8)]
    pub count: usize,
    /// `long`: plays of 2-6 minutes joined by crossfades of 10-60 s,
    /// with runs of 1-4 held-out tracks between indexed ones (report
    /// `mixes-long-seed-<seed>.json`).
    #[arg(long, value_enum, default_value = "short")]
    pub kind: mixes::MixKind,
    /// The other library's root, where the shared kind finds the
    /// shared-material panel's files under `second-library/`.
    #[arg(long)]
    pub other_root: Option<PathBuf>,
    /// With `--kind rides`: each ride's change of speed, a share drawn
    /// uniformly between the two values (one value: exactly that)
    /// [default: 0.01,0.03]. Rides drawn otherwise than by default go
    /// to `mixes-rides-<label>-seed-<seed>.json`.
    #[arg(long, value_delimiter = ',', num_args = 1..=2)]
    pub ride_change: Vec<f64>,
    /// With `--kind rides`: the ramp's length in seconds, drawn
    /// uniformly between the two values [default: 5,15].
    #[arg(long, value_delimiter = ',', num_args = 1..=2)]
    pub ride_seconds: Vec<f64>,
    /// With `--kind rides`: up, down or either, drawn for each play.
    #[arg(long, value_enum, default_value_t = mixes::RideDirection::Either)]
    pub ride_direction: mixes::RideDirection,
    /// With `--kind drops`: JSON whose `assets` lists the files the
    /// mixes may play as indexed records (as
    /// `scripts/analysis/tracklist_tags.py` writes it) [default: every
    /// indexed file].
    #[arg(long)]
    pub tagged: Option<PathBuf>,
    /// With `--kind drops`: also play the panel's pairs whose files
    /// this index lacks (the other library's, under --other-root), as
    /// records not in the index, for searching the renders with the
    /// other library (`mixes-drops-other-pairs-seed-<seed>.json`).
    #[arg(long, requires = "tagged")]
    pub other_pairs: bool,
    /// With `--kind drops`: line up the drops of the two records of
    /// each unrelated double drop, in beat phase: each record's
    /// strongest drop it can play then (the largest rise of loudness,
    /// moved to the beat) at the same mix time
    /// (`mixes-drops-aligned-drops-seed-<seed>.json`).
    #[arg(long, conflicts_with = "other_pairs")]
    pub aligned_drops: bool,
    /// Short mixes only: render them in stereo with the right channel
    /// under this `robust` stereo fault (such as `stereo-inverted`) over
    /// the second half of each mix, into their own renders and report
    /// (`mixes-<fault>-seed-<seed>.json`); `stereo-control` is the same
    /// mixes in stereo without a fault.
    #[arg(long)]
    pub stereo_fault: Option<String>,
}

#[derive(Clone, Copy, clap::Args)]
pub struct GridArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
}

pub fn mixes(paths: &Paths, jobs: usize, args: MixesArgs) -> Result<(), String> {
    let MixesArgs {
        seed,
        count,
        kind,
        other_root,
        ride_change,
        ride_seconds,
        ride_direction,
        tagged,
        other_pairs,
        aligned_drops,
        stereo_fault,
    } = args;
    if aligned_drops && kind != mixes::MixKind::Drops {
        return Err(String::from("--aligned-drops applies to --kind drops only"));
    }
    let rides = ride_draw(kind, &ride_change, &ride_seconds, ride_direction)?;
    let stereo_fault = stereo_fault
        .map(|name| match kind {
            mixes::MixKind::Short => robust::stereo_fault(&name),
            _ => Err(String::from("--stereo-fault applies to short mixes only")),
        })
        .transpose()?;
    let tagged = tagged.map(|path| tagged_assets(kind, &path)).transpose()?;
    let report = mixes::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &mixes::Options {
            same_titles: &paths.same_titles()?,
            seed,
            kind,
            other_root: other_root.as_deref(),
            panels: &paths.panels,
            count,
            ladder_name: paths.ladder.name(),
            ladder: &paths.rungs(),
            matching: &paths.matching,
            verify: paths.verify,
            jobs,
            rides,
            tagged: tagged.as_ref(),
            other_pairs,
            aligned_drops,
            stereo_fault,
        },
    )?;
    write_json(
        &paths.reports().join(format!(
            "mixes-{}.json",
            kind.name(seed, report.label().as_deref())
        )),
        &report,
    )?;
    mixes::print_summary(&report, kind);
    Ok(())
}

pub fn grid(paths: &Paths, jobs: usize, args: GridArgs) -> Result<(), String> {
    let GridArgs { seed } = args;
    let report = grid::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &grid::Options {
            same_titles: &paths.same_titles()?,
            seed,
            panels: &paths.panels,
            ladder_name: paths.ladder.name(),
            ladder: &paths.rungs(),
            matching: &paths.matching,
            verify: paths.verify,
            jobs,
        },
    )?;
    write_json(
        &paths.reports().join(format!("grid-seed-{seed}.json")),
        &report,
    )?;
    grid::print_summary(&report);
    Ok(())
}

/// The files `--tagged` lists, for the drops kind only.
fn tagged_assets(kind: mixes::MixKind, path: &Path) -> Result<BTreeSet<String>, String> {
    #[derive(serde::Deserialize)]
    struct Tagged {
        assets: BTreeSet<String>,
    }
    if kind != mixes::MixKind::Drops {
        return Err(String::from("--tagged applies to --kind drops"));
    }
    let tagged: Tagged = read_json(path)?;
    Ok(tagged.assets)
}

/// The rides `mixes --kind rides` draws: the default, with the options
/// given in place of its ranges.
fn ride_draw(
    kind: mixes::MixKind,
    change: &[f64],
    seconds: &[f64],
    direction: mixes::RideDirection,
) -> Result<mixes::RideDraw, String> {
    let default = mixes::RideDraw::default();
    let changed = !change.is_empty() || !seconds.is_empty() || direction != default.direction;
    if changed && kind != mixes::MixKind::Rides {
        return Err(String::from(
            "--ride-change, --ride-seconds and --ride-direction apply to --kind rides",
        ));
    }
    let range = |values: &[f64], default: (f64, f64)| match *values {
        [] => default,
        [value] => (value, value),
        [low, high, ..] => (low, high),
    };
    let draw = mixes::RideDraw {
        change: range(change, default.change),
        seconds: range(seconds, default.seconds),
        direction,
    };
    let (low, high) = draw.change;
    if !(0.0 <= low && low <= high && high < 0.07) {
        return Err(String::from(
            "--ride-change is one share or two in order, from 0 to below 0.07",
        ));
    }
    let (low, high) = draw.seconds;
    if !(0.0 < low && low <= high) {
        return Err(String::from(
            "--ride-seconds is one length or two in order, above 0",
        ));
    }
    Ok(draw)
}

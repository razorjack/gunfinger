//! The `mixes` and `grid` commands: generated mixes with exact truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::files::{read_json, write_json};
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::mixes::residual::{Pilot, Timing};
use crate::mixes::{DropsOptions, KindOptions, MixKind};
use crate::residual::Rule;
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
    pub kind: MixKind,
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
    /// With `--kind drops`: proposal 4's pilot (brief 65). The span of
    /// every play found confidently is searched again as the mix holds
    /// it, with denser query peaks, and with what the play explains taken
    /// out (a spectral mask from a render of the play); the report gains
    /// `residual` and goes to
    /// `mixes-drops-seed-<seed>-residual-<timing>-over-<over>.json`.
    #[arg(long)]
    pub residual: bool,
    /// With `--residual`: how many times the play's estimated power each
    /// bin loses.
    #[arg(long, requires = "residual", default_value_t = 2.0)]
    pub residual_over: f64,
    /// With `--residual`: where the play's time map comes from.
    #[arg(long, requires = "residual", value_enum, default_value_t = Timing::Truth)]
    pub residual_timing: Timing,
}

#[derive(Clone, Copy, clap::Args)]
pub struct GridArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
}

pub fn mixes(harness: &Harness, jobs: usize, args: MixesArgs) -> Result<(), String> {
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
        residual,
        residual_over,
        residual_timing,
    } = args;
    if residual && (kind != MixKind::Drops || other_pairs || aligned_drops) {
        return Err(String::from(
            "--residual applies to --kind drops without --other-pairs or --aligned-drops",
        ));
    }
    if residual_over <= 0.0 {
        return Err(String::from("--residual-over is a positive factor"));
    }
    let pilot = residual.then_some(Pilot {
        rule: Rule {
            over: residual_over,
        },
        timing: residual_timing,
    });
    if aligned_drops && kind != MixKind::Drops {
        return Err(String::from("--aligned-drops applies to --kind drops only"));
    }
    let rides = ride_draw(kind, &ride_change, &ride_seconds, ride_direction)?;
    let stereo_fault = stereo_fault
        .map(|name| match kind {
            MixKind::Short => robust::stereo_fault(&name),
            _ => Err(String::from("--stereo-fault applies to short mixes only")),
        })
        .transpose()?;
    let tagged = tagged.map(|path| tagged_assets(kind, &path)).transpose()?;
    let kind_options = match kind {
        MixKind::Short => KindOptions::Short { stereo_fault },
        MixKind::Long => KindOptions::Long,
        MixKind::Beatmatched => KindOptions::Beatmatched,
        MixKind::Shared => KindOptions::Shared,
        MixKind::Rides => KindOptions::Rides(rides),
        MixKind::Drops => KindOptions::Drops(DropsOptions {
            tagged: tagged.as_ref(),
            other_pairs,
            aligned_drops,
        }),
    };
    let report = mixes::run(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        &harness.paths.work,
        &mixes::Options {
            same_titles: &harness.same_titles()?,
            seed,
            kind: kind_options,
            other_root: other_root.as_deref(),
            panels: &harness.paths.panels,
            count,
            ladder_name: harness.search.ladder.name(),
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            verify: harness.search.verify,
            residual: pilot,
            jobs,
        },
    )?;
    let name = kind.name(seed, report.label().as_deref());
    let name = match pilot {
        None => name,
        Some(pilot) => format!(
            "{name}-residual-{}-over-{}",
            pilot.timing.name(),
            pilot.rule.over
        ),
    };
    write_json(
        &harness.reports().join(format!("mixes-{name}.json")),
        &report,
    )?;
    mixes::print_summary(&report, kind);
    Ok(())
}

pub fn grid(harness: &Harness, jobs: usize, args: GridArgs) -> Result<(), String> {
    let GridArgs { seed } = args;
    let report = grid::run(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        &harness.paths.work,
        &grid::Options {
            same_titles: &harness.same_titles()?,
            seed,
            panels: &harness.paths.panels,
            ladder_name: harness.search.ladder.name(),
            ladder: &harness.rungs(),
            matching: &harness.search.matching,
            padding: &harness.padding(&UNCHANGED_INDEX)?,
            verify: harness.search.verify,
            jobs,
        },
    )?;
    write_json(
        &harness.reports().join(format!("grid-seed-{seed}.json")),
        &report,
    )?;
    grid::print_summary(&report);
    Ok(())
}

/// The files `--tagged` lists, for the drops kind only.
fn tagged_assets(kind: MixKind, path: &Path) -> Result<BTreeSet<String>, String> {
    #[derive(serde::Deserialize)]
    struct Tagged {
        assets: BTreeSet<String>,
    }
    if kind != MixKind::Drops {
        return Err(String::from("--tagged applies to --kind drops"));
    }
    let tagged: Tagged = read_json(path)?;
    Ok(tagged.assets)
}

/// The rides `mixes --kind rides` draws: the default, with the options
/// given in place of its ranges.
fn ride_draw(
    kind: MixKind,
    change: &[f64],
    seconds: &[f64],
    direction: mixes::RideDirection,
) -> Result<mixes::RideDraw, String> {
    let default = mixes::RideDraw::default();
    let changed = !change.is_empty() || !seconds.is_empty() || direction != default.direction;
    if changed && kind != MixKind::Rides {
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

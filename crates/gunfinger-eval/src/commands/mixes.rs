//! The `mixes`, `counterexamples`, `passages` and `grid` commands:
//! generated mixes with exact truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::files::{read_json, write_json};
use crate::harness::{Harness, UNCHANGED_INDEX};
use crate::mixes::residual::{Pilot, Timing};
use crate::mixes::{DropsOptions, KindOptions, MixKind};
use crate::residual::{Fix, Rule};
use crate::{counterexamples, grid, mixes, passages, robust};

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
    /// `mixes-drops-seed-<seed>-residual-<timings>-over-<over>.json`
    /// (`-fix-<fixes>` added unless the only fix is `off`).
    #[arg(long)]
    pub residual: bool,
    /// With `--residual`: how many times the play's estimated power each
    /// bin loses.
    #[arg(long, requires = "residual", default_value_t = 2.0)]
    pub residual_over: f64,
    /// With `--residual`: where the play's time map comes from; several,
    /// separated by commas, are each masked and searched in one run.
    #[arg(
        long,
        requires = "residual",
        value_enum,
        value_delimiter = ',',
        default_value = "truth"
    )]
    pub residual_timing: Vec<Timing>,
    /// With `--residual`: what is done to the play's render before the mask
    /// (brief 70); several, separated by commas, are each searched.
    #[arg(
        long,
        requires = "residual",
        value_enum,
        value_delimiter = ',',
        default_value = "off"
    )]
    pub residual_fix: Vec<Fix>,
}

#[derive(clap::Args)]
pub struct CounterexamplesArgs {
    /// The sweep's seed, whose held-out clusters the index leaves out,
    /// and the samples' seed.
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    #[arg(long, default_value_t = 100)]
    pub samples: usize,
    /// Where the report (`counterexamples-seed-<seed>.json`), the recipes
    /// (`recipes/NNNN.json`) and the renders while they are searched go.
    #[arg(long)]
    pub out: PathBuf,
    /// Render and search a saved recipe's minimal mix again (a
    /// `recipes/NNNN.json`), print whether its failure holds, and write
    /// `replays/NNNN.json` under --out; no samples are drawn.
    #[arg(long)]
    pub replay: Option<PathBuf>,
}

pub fn counterexamples(
    harness: &Harness,
    jobs: usize,
    args: CounterexamplesArgs,
) -> Result<(), String> {
    let CounterexamplesArgs {
        seed,
        samples,
        out,
        replay,
    } = args;
    let library = harness.library()?;
    let store = harness.store()?;
    let clusters = harness.clusters()?;
    let same_titles = harness.same_titles()?;
    let ladder = harness.rungs();
    let padding = harness.padding(&UNCHANGED_INDEX)?;
    let options = counterexamples::Options {
        seed,
        samples,
        same_titles: &same_titles,
        panels: &harness.paths.panels,
        ladder_name: harness.search.ladder.name(),
        ladder: &ladder,
        matching: &harness.search.matching,
        padding: &padding,
        out: &out,
        jobs,
    };
    if let Some(path) = replay {
        let saved: counterexamples::SavedRecipe = read_json(&path)?;
        let replayed = counterexamples::replay(&library, &store, &clusters, &options, &saved)?;
        println!(
            "{}: {}, the failure {}, the outcome {} the saved one's",
            path.display(),
            replayed.class,
            if replayed.holds {
                "holds"
            } else {
                "does not hold"
            },
            if replayed.same_outcome {
                "equal to"
            } else {
                "differs from"
            }
        );
        let name = path.file_name().ok_or("--replay names a file")?;
        return write_json(&out.join("replays").join(name), &replayed);
    }
    let report = counterexamples::run(&library, &store, &clusters, &options)?;
    write_json(
        &out.join(format!("counterexamples-seed-{seed}.json")),
        &report,
    )?;
    counterexamples::print_summary(&report);
    Ok(())
}

#[derive(clap::Args)]
pub struct PassagesArgs {
    /// The batch's seed.
    #[arg(long, default_value_t = 6901)]
    pub seed: u64,
    /// Where the report (`passages-seed-<seed>.json`) and, while they are
    /// searched, the renders go.
    #[arg(long)]
    pub out: PathBuf,
    /// The other library's root, where the shared-material panel's files
    /// under `second-library/` are read for rendering.
    #[arg(long)]
    pub other_root: Option<PathBuf>,
    /// What is indexed and searched: each mix's unidentified spans, the
    /// whole mixes, or both.
    #[arg(long, value_enum, value_delimiter = ',', default_value = "gaps,whole")]
    pub mode: Vec<passages::Mode>,
    /// The relative speeds searched between two mixes, as shares either
    /// side of 1 (0.08: the product's ladder; more adds rungs past its
    /// ends, a step apart).
    #[arg(long, value_delimiter = ',', default_value = "0.08,0.18")]
    pub relative_speed_range: Vec<f64>,
    /// Keep the renders under --out.
    #[arg(long)]
    pub keep_renders: bool,
}

pub fn passages(harness: &Harness, jobs: usize, args: PassagesArgs) -> Result<(), String> {
    let PassagesArgs {
        seed,
        out,
        other_root,
        mode,
        relative_speed_range,
        keep_renders,
    } = args;
    if harness.search.extra_rungs > 0 {
        return Err(String::from(
            "passages sets its ladders by --relative-speed-range; leave out --extra-rungs",
        ));
    }
    if relative_speed_range.iter().any(|&range| range < 0.08) {
        return Err(String::from(
            "--relative-speed-range starts at the product's ladder, 0.08",
        ));
    }
    let library = harness.library()?;
    let store = harness.store()?;
    let clusters = harness.clusters()?;
    let same_titles = harness.same_titles()?;
    let ladder = harness.rungs();
    let padding = harness.padding(&UNCHANGED_INDEX)?;
    let options = passages::Options {
        seed,
        modes: &mode,
        ranges: &relative_speed_range,
        same_titles: &same_titles,
        panels: &harness.paths.panels,
        other_root: other_root.as_deref(),
        ladder: &ladder,
        matching: &harness.search.matching,
        padding: &padding,
        out: &out,
        keep_renders,
        jobs,
    };
    let report = passages::run(&library, &store, &clusters, &options)?;
    passages::save(&out, &report)?;
    passages::print_summary(&report);
    Ok(())
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
        residual_fix,
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
        timings: deduplicated(residual_timing),
        fixes: deduplicated(residual_fix),
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
    // Only the drops kind draws with the corpus clusters.
    let corpus_clusters = if kind == MixKind::Drops {
        harness.corpus_clusters()?
    } else {
        harness.clusters()?
    };
    let report = mixes::run(
        &harness.library()?,
        &harness.store()?,
        &harness.clusters()?,
        &harness.paths.work,
        &mixes::Options {
            same_titles: &harness.same_titles()?,
            corpus_clusters: &corpus_clusters,
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
            residual: pilot.as_ref(),
            jobs,
        },
    )?;
    let name = kind.name(seed, report.label().as_deref());
    let name = match &pilot {
        None => name,
        Some(pilot) => format!("{name}-{}", pilot.name()),
    };
    write_json(
        &harness.reports().join(format!("mixes-{name}.json")),
        &report,
    )?;
    mixes::print_summary(&report, kind);
    Ok(())
}

/// `values` in the order first given, each once.
fn deduplicated<T: PartialEq>(values: Vec<T>) -> Vec<T> {
    let mut kept = Vec::with_capacity(values.len());
    for value in values {
        if !kept.contains(&value) {
            kept.push(value);
        }
    }
    kept
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

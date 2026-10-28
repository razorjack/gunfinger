//! The standard evaluation and its relatives: `scan`, `calibrate`,
//! `robust`, `baseline`, `regress`, `loss` and `memory`.

use super::sweep::{NO_FILTER_STUDY, run_sweep};
use crate::files::write_json;
use crate::padding::Padding;
use crate::scan::{LeaveOut, SlotRun};
use crate::{IndexVariant, Paths, UNCHANGED_INDEX};
use crate::{calibrate, loss, memory, regress, robust, scan, stereo, sweep};

#[derive(clap::Args)]
pub struct ScanArgs {
    /// Name of the set directory under `sets/`.
    pub set: String,
    /// Leave this many referenced tracks (with their clusters) out of the
    /// index; their slots must then produce no confident detection.
    #[arg(long)]
    pub leave_out: Option<usize>,
    /// Seed for choosing the tracks to leave out.
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    /// Leave these manifest slots (with their clusters) out of the
    /// index instead, as runs of consecutive slots: `3,7-8,12-14`. The
    /// report is `scan-<set>-leave-out-slots-3_7-8_12-14.json`.
    #[arg(
        long,
        value_delimiter = ',',
        value_parser = SlotRun::parse,
        conflicts_with = "leave_out"
    )]
    pub leave_out_slots: Vec<SlotRun>,
    /// A changed index; the report is then named `scale-scan-...` so
    /// that calibrate and regress never read it.
    #[command(flatten)]
    pub variant: IndexVariant,
}

#[derive(clap::Args)]
pub struct CalibrateArgs {
    /// The development set whose scans (and leave-outs) are read.
    #[arg(long, default_value = "stakka-skynet-knowledge")]
    pub set: String,
}

#[derive(clap::Args)]
pub struct RobustArgs {
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
    /// Run only these conditions (and the control), by name.
    #[arg(long, value_delimiter = ',')]
    pub only: Vec<String>,
    /// What is searched of each query: the core's downmix, or, as a
    /// diagnostic for the stereo faults, one channel, the louder one or
    /// the side signal (report `robust-seed-<seed>-channels-<choice>.json`).
    #[arg(long, value_enum, default_value = "downmix")]
    pub query_channels: stereo::QueryChannels,
    #[command(flatten)]
    pub variant: IndexVariant,
}

#[derive(clap::Args)]
pub struct LossArgs {
    #[arg(long, default_value = "stakka-skynet-knowledge")]
    pub set: String,
}

#[derive(clap::Args)]
pub struct BaselineArgs {
    pub name: String,
    #[arg(long, default_value = "stakka-skynet-knowledge")]
    pub set: String,
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
}

#[derive(clap::Args)]
pub struct MemoryArgs {
    #[arg(long, default_value = "stakka-skynet-knowledge")]
    pub set: String,
    #[command(flatten)]
    pub variant: IndexVariant,
    /// Stop after this phase.
    #[arg(long, value_enum, default_value_t = memory::Phase::Searched)]
    pub until: memory::Phase,
    /// Search only this many minutes from the start of the set's audio.
    #[arg(long)]
    pub minutes: Option<u64>,
    /// Keep the first pass's lines and count them, instead of
    /// searching the way `identify` does.
    #[arg(long)]
    pub count_lines: bool,
}

#[derive(clap::Args)]
pub struct RegressArgs {
    /// Name of the baseline under `work/baselines/`.
    pub name: String,
    /// Compare the existing reports without running the evaluation.
    #[arg(long)]
    pub no_rerun: bool,
    #[arg(long, default_value = "stakka-skynet-knowledge")]
    pub set: String,
    #[arg(long, default_value_t = 2026)]
    pub seed: u64,
}

pub fn scan(paths: &Paths, jobs: usize, args: ScanArgs) -> Result<(), String> {
    let ScanArgs {
        set,
        leave_out,
        seed,
        leave_out_slots,
        variant,
    } = args;
    let leave_out = match leave_out {
        Some(count) => Some(LeaveOut::Drawn { count, seed }),
        None if !leave_out_slots.is_empty() => Some(LeaveOut::Slots(leave_out_slots)),
        None => None,
    };
    run_scan(paths, &set, leave_out.as_ref(), &variant, jobs)
}

pub fn calibrate(paths: &Paths, args: CalibrateArgs) -> Result<(), String> {
    let CalibrateArgs { set } = args;
    calibrate::run(&paths.reports(), &set, paths.matching.rule())
}

pub fn robust(paths: &Paths, jobs: usize, args: RobustArgs) -> Result<(), String> {
    let RobustArgs {
        seed,
        only,
        query_channels,
        variant,
    } = args;
    let ladder = paths.ladder;
    paths.check_panel(seed)?;
    let padding = paths.padding(&variant)?;
    let report = robust::run(
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &robust::Options {
            same_titles: &paths.same_titles()?,
            seed,
            panels: &paths.panels,
            only: &only,
            ladder_name: ladder.name(),
            ladder: &paths.rungs(),
            padding: &padding,
            matching: &paths.matching,
            verify: paths.verify,
            query_channels,
            jobs,
        },
    )?;
    let suffix = variant.suffix();
    let channels = match query_channels {
        stereo::QueryChannels::Downmix => String::new(),
        choice => format!("-channels-{}", choice.name()),
    };
    write_json(
        &paths
            .reports()
            .join(format!("robust-seed-{seed}{suffix}{channels}.json")),
        &report,
    )?;
    robust::print_summary(&report);
    Ok(())
}

pub fn loss(paths: &Paths, jobs: usize, args: LossArgs) -> Result<(), String> {
    let LossArgs { set } = args;
    let report = loss::run(
        &paths.sets(),
        &paths.library()?,
        &paths.store()?,
        &paths.clusters()?,
        &paths.work,
        &loss::Options {
            set: &set,
            ladder: &paths.rungs(),
            jobs,
        },
    )?;
    write_json(&paths.reports().join(format!("loss-{set}.json")), &report)?;
    loss::print_summary(&report);
    Ok(())
}

pub fn baseline(paths: &Paths, args: BaselineArgs) -> Result<(), String> {
    let BaselineArgs { name, set, seed } = args;
    regress::save(&paths.reports(), &paths.baseline(&name), &set, seed)
}

pub fn memory(paths: &Paths, jobs: usize, args: MemoryArgs) -> Result<(), String> {
    let MemoryArgs {
        set,
        variant,
        until,
        minutes,
        count_lines,
    } = args;
    let report = memory::run(
        &paths.sets(),
        &paths.library()?,
        &paths.store()?,
        &memory::Options {
            set: &set,
            // The measurement is the build's memory: never the saved index.
            padding: &Padding {
                saved: None,
                ..paths.padding(&variant)?
            },
            until,
            minutes,
            count_lines,
            ladder: &paths.rungs(),
            matching: paths.matching,
            jobs,
        },
    )?;
    let matcher = paths
        .matching
        .name()
        .map_or_else(String::new, |name| format!("-{name}"));
    write_json(
        &paths.work.join("memory").join(format!(
            "memory{}{}-jobs-{jobs}-until-{}{matcher}{}.json",
            paths
                .other_name()
                .map_or_else(String::new, |name| format!("-library-{name}")),
            variant.suffix(),
            format!("{until:?}").to_lowercase(),
            if count_lines { "-lines" } else { "" }
        )),
        &report,
    )?;
    memory::print_summary(&report);
    Ok(())
}

pub fn regress(paths: &Paths, jobs: usize, args: RegressArgs) -> Result<(), String> {
    let RegressArgs {
        name,
        no_rerun,
        set,
        seed,
    } = args;
    if !no_rerun {
        run_standard_evaluation(paths, &set, seed, jobs)?;
    }
    regress::compare(&paths.baseline(&name), &paths.reports(), &set, seed)
}

/// The sweep, the development scan and its leave-outs.
fn run_standard_evaluation(paths: &Paths, set: &str, seed: u64, jobs: usize) -> Result<(), String> {
    run_sweep(
        paths,
        seed,
        false,
        sweep::Speeds::Sweep,
        &NO_FILTER_STUDY,
        jobs,
    )?;
    run_scan(paths, set, None, &UNCHANGED_INDEX, jobs)?;
    for count in regress::LEAVE_OUTS {
        run_scan(
            paths,
            set,
            Some(&LeaveOut::Drawn { count, seed }),
            &UNCHANGED_INDEX,
            jobs,
        )?;
    }
    Ok(())
}

fn run_scan(
    paths: &Paths,
    set: &str,
    leave_out: Option<&LeaveOut>,
    variant: &IndexVariant,
    jobs: usize,
) -> Result<(), String> {
    let padding = paths.padding(variant)?;
    let library = paths.library()?;
    let report = scan::run(
        &paths.sets(),
        set,
        &library,
        &paths.store()?,
        &paths.clusters()?,
        &scan::Options {
            same_titles: &paths.same_titles()?,
            referable: &paths.referable(&library)?,
            leave_out,
            padding: &padding,
            matching: &paths.matching,
            ladder: &paths.rungs(),
            verify: paths.verify,
            jobs,
        },
    )?;
    let suffix = variant.suffix();
    let name = match leave_out {
        _ if !suffix.is_empty() => format!("scale-scan-{set}{suffix}.json"),
        Some(LeaveOut::Drawn { count, seed }) => {
            format!("scan-{set}-leave-out-{count}-seed-{seed}.json")
        }
        Some(LeaveOut::Slots(runs)) => format!(
            "scan-{set}-leave-out-slots-{}.json",
            runs.iter()
                .map(|run| run.name())
                .collect::<Vec<_>>()
                .join("_")
        ),
        None => format!("scan-{set}.json"),
    };
    write_json(&paths.reports().join(name), &report)?;
    scan::print_summary(&report);
    Ok(())
}

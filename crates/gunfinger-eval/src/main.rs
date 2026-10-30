//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod audition;
mod calibrate;
mod clusters;
mod commands;
mod files;
mod fullest;
mod gaps;
mod grid;
mod harness;
mod hash_cost;
mod holders;
mod identifiability;
mod library_map;
mod loss;
mod manifest;
mod matching;
mod memory;
mod mixes;
mod padding;
mod pair;
mod pair_review;
mod parts;
mod platform;
mod recall;
mod regress;
mod related;
mod render;
mod residual;
mod rng;
mod robust;
mod scan;
mod scoring;
mod seconds;
mod shared;
mod shared_mixes;
mod stereo;
mod survival;
mod sweep;
mod synthetic;
mod tempo;
mod titles;
mod verifier;
mod versions;

use std::num::NonZeroUsize;
use std::process::ExitCode;

use clap::Parser;

use crate::harness::Harness;

/// Development harness for Gunfinger: manifests, the speed sweep and scoring.
#[derive(Parser)]
#[command(name = "gunfinger-eval")]
struct Cli {
    #[command(flatten)]
    harness: Harness,

    /// Worker threads [default: one per core].
    #[arg(long, global = true)]
    jobs: Option<NonZeroUsize>,

    #[command(subcommand)]
    command: commands::Command,
}

fn main() -> ExitCode {
    let Cli {
        harness,
        jobs,
        command,
    } = Cli::parse();
    let jobs = jobs.map_or_else(available_parallelism, NonZeroUsize::get);
    match harness
        .search
        .matching
        .check()
        .and_then(|()| commands::run(&harness, jobs, command))
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

/// One worker thread per core.
fn available_parallelism() -> usize {
    std::thread::available_parallelism().map_or(1, NonZeroUsize::get)
}

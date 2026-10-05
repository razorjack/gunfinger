//! Development harness for Gunfinger: manifests, the speed sweep and scoring.
//!
//! This crate is the only one that reads ground truth. The core and the CLI
//! never see a manifest.

mod manifest;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use gunfinger_core::library::Library;
use gunfinger_core::timecode::format_timecode;

#[derive(Parser)]
#[command(name = "gunfinger-eval")]
struct Cli {
    /// Corpus root containing `library/` and `sets/`.
    #[arg(long, default_value = "corpus")]
    corpus: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Check every set manifest against the library.
    Validate,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Validate => validate(&cli.corpus),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn validate(corpus: &std::path::Path) -> Result<(), String> {
    let library = Library::scan(&corpus.join("library")).map_err(|error| error.to_string())?;
    let sets_dir = corpus.join("sets");
    let mut failures = 0;
    for name in manifest::set_names(&sets_dir)? {
        match manifest::load_set(&sets_dir, &name, &library) {
            Ok(set) => {
                let referenced = set
                    .tracks
                    .iter()
                    .filter(|track| track.is_referenced())
                    .count();
                let references: usize = set.tracks.iter().map(|track| track.references.len()).sum();
                println!(
                    "{name}: valid; {} tracks, {referenced} referenced ({references} reference files), {} absent",
                    set.tracks.len(),
                    set.tracks.len() - referenced
                );
                eprintln!("{name}: {} ({})", set.title, set.audio.display());
                for track in &set.tracks {
                    eprintln!(
                        "  {} {} [{} references]",
                        format_timecode(track.start),
                        track.label(),
                        track.references.len()
                    );
                }
            }
            Err(problems) => {
                failures += 1;
                println!("{name}: {} problems", problems.len());
                for problem in problems {
                    println!("  {problem}");
                }
            }
        }
    }
    if failures == 0 {
        Ok(())
    } else {
        Err(format!("{failures} manifests are invalid"))
    }
}

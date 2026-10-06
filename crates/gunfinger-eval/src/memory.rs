//! Memory by phase: resident memory after each step of a search the way
//! `identify` runs it, on the scale proxy (`synthetic`). The kernel's peak
//! for the whole run comes from `/usr/bin/time -l`; `--until` stops after
//! a phase, so the peak of each phase can be measured on its own.
//!
//! `identify` reads one peak record at a time while it builds the index and
//! keeps none. The scale proxy needs the library's records in memory to
//! make the copies from, so it holds them until the index is built.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use clap::ValueEnum;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::peaks::Peak;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::trace_with_progress;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::manifest::load_set;
use crate::matching::Matching;
use crate::synthetic;

/// The last phase to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Phase {
    /// The library's peak records in memory.
    Loaded,
    /// The index built, with its synthetic copies, and the records
    /// dropped.
    Built,
    /// The query decoded and searched.
    Searched,
}

pub struct Options<'a> {
    pub set: &'a str,
    pub synthetic_copies: usize,
    pub until: Phase,
    /// Search only this many minutes from the start of the set's audio.
    pub minutes: Option<u64>,
    /// Keep the first pass's lines and count them instead of searching.
    pub count_lines: bool,
    pub ladder: &'a [Rung],
    pub matching: Matching,
    pub jobs: usize,
}

#[derive(Serialize)]
pub struct MemoryReport {
    pub synthetic_copies: usize,
    pub jobs: usize,
    /// The opt-in matcher's name; `None` for the default.
    pub matching: Option<String>,
    pub assets: usize,
    /// Peaks of the library's own records (the copies have a few fewer).
    pub library_peaks: usize,
    pub postings: usize,
    pub query_seconds: f64,
    /// With `count_lines`: the first pass's distinct lines and its
    /// detections.
    pub lines: Option<usize>,
    pub detections: Option<usize>,
    pub phases: Vec<PhaseRow>,
}

/// Resident memory at the end of a phase beside what the data structures
/// alive at that moment occupy (struct sizes times counts).
#[derive(Serialize)]
pub struct PhaseRow {
    pub phase: String,
    pub seconds: f64,
    pub resident_bytes: u64,
    /// Peak records: `Peak`s.
    pub records_bytes: u64,
    /// The offsets table and the postings.
    pub index_bytes: u64,
}

pub fn run(
    sets_dir: &Path,
    library: &Library,
    store: &PeakStore,
    options: &Options,
) -> Result<MemoryReport, String> {
    let profile = Profile::CURRENT;
    let mut report = MemoryReport {
        synthetic_copies: options.synthetic_copies,
        jobs: options.jobs,
        matching: options.matching.name(),
        assets: 0,
        library_peaks: 0,
        postings: 0,
        query_seconds: 0.0,
        lines: None,
        detections: None,
        phases: Vec::new(),
    };

    let started = Instant::now();
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let peaks: usize = records.iter().map(|record| record.peaks.len()).sum();
    let records_bytes = (peaks * size_of::<Peak>()) as u64;
    report
        .phases
        .push(row(Phase::Loaded, started, records_bytes, 0));
    if options.until == Phase::Loaded {
        return Ok(report);
    }

    let started = Instant::now();
    let index = synthetic::index_with_copies(&records, options.synthetic_copies, &profile)
        .map_err(|error| error.to_string())?;
    let index = options.matching.index(index);
    drop(records);
    report.assets = index.assets().len();
    report.library_peaks = peaks;
    report.postings = index.posting_count();
    let index_bytes = index.size_bytes() as u64;
    report
        .phases
        .push(row(Phase::Built, started, 0, index_bytes));
    if options.until == Phase::Built {
        return Ok(report);
    }

    let started = Instant::now();
    let set = load_set(sets_dir, options.set, library).map_err(|problems| problems.join("; "))?;
    let excerpt = Excerpt {
        start: None,
        duration: options
            .minutes
            .map(|minutes| std::time::Duration::from_secs(60 * minutes)),
    };
    let audio =
        decode(&set.audio, profile.sample_rate, excerpt).map_err(|error| error.to_string())?;
    report.query_seconds = audio.duration().as_secs_f64();
    if options.count_lines {
        let trace = trace_with_progress(
            &index,
            &audio.samples,
            &profile,
            options.ladder,
            options.jobs,
            |_| {},
        );
        report.lines = Some(trace.lines.len());
        report.detections = Some(trace.detections.len());
    } else {
        options.matching.search(
            &index,
            &audio.samples,
            &profile,
            options.ladder,
            options.jobs,
        );
    }
    report
        .phases
        .push(row(Phase::Searched, started, 0, index_bytes));
    Ok(report)
}

fn row(phase: Phase, started: Instant, records_bytes: u64, index_bytes: u64) -> PhaseRow {
    PhaseRow {
        phase: format!("{phase:?}").to_lowercase(),
        seconds: started.elapsed().as_secs_f64(),
        resident_bytes: resident_bytes().unwrap_or(0),
        records_bytes,
        index_bytes,
    }
}

/// This process's resident set size as `ps` reports it (macOS and Linux).
/// Compressed and swapped pages are not resident, so on a machine short of
/// memory this undercounts; `/usr/bin/time -l` gives the kernel's peak.
fn resident_bytes() -> Option<u64> {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &std::process::id().to_string()])
        .output()
        .ok()?;
    let kilobytes: u64 = String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .ok()?;
    Some(kilobytes * 1024)
}

pub fn print_summary(report: &MemoryReport) {
    let megabytes = |bytes: u64| bytes as f64 / 1e6;
    println!(
        "memory: {} assets ({} synthetic copies each), {} library peaks, {} postings, {} jobs, {:.0} s of query, matcher {}",
        report.assets,
        report.synthetic_copies,
        report.library_peaks,
        report.postings,
        report.jobs,
        report.query_seconds,
        report.matching.as_deref().unwrap_or("default")
    );
    if let (Some(lines), Some(detections)) = (report.lines, report.detections) {
        println!("first pass: {lines} distinct lines, {detections} detections");
    }
    println!(
        "{:<9} {:>8} {:>11} {:>10} {:>10}",
        "phase", "seconds", "resident MB", "records MB", "index MB"
    );
    for phase in &report.phases {
        println!(
            "{:<9} {:>8.1} {:>11.0} {:>10.0} {:>10.0}",
            phase.phase,
            phase.seconds,
            megabytes(phase.resident_bytes),
            megabytes(phase.records_bytes),
            megabytes(phase.index_bytes)
        );
    }
}

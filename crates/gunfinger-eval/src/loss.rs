//! Where a real mix loses evidence: each identified play of a set is
//! aligned with its library record, window by window, and the hits it
//! keeps are counted against the reference hashes of the stretch heard.
//!
//! The same stretch of the library file, rendered alone at the play's
//! speed, is the control: the ratio of the mix's hits to the clean
//! render's separates what the mix did (blends, EQ, the vinyl and its rip)
//! from what the track's content and the analysis would lose anyway.
//! Each window is analysed on the rung nearest the play's fitted speed (as
//! the search sees it), at the fitted speed itself, and at the best local
//! speed near it (speed varying within the play).
//!
//! Key-locked plays are left out: the clean control renders turntable
//! playback. Development set only: the test set is not touched.

use std::collections::{BTreeSet, HashMap};
use std::path::Path;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::hash::{Point, for_each_pair};
use gunfinger_core::index::{Index, MAX_FRAMES};
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, WINDOW_SECONDS, search};
use gunfinger_core::speed::{Playback as Played, Rung, SpeedRatio, key_lock_ladder, ladder};
use gunfinger_core::store::{PeakRecord, PeakStore};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::manifest::load_set;
use crate::render::{Encoding, render_excerpt};

/// Octave bands of anchor frequency, as `gunfinger stats` counts peaks.
const BAND_EDGES_HZ: [f64; 6] = [125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0];
/// Hits within this many frames of the window's densest offset are on the
/// play's line (as `lines` gathers them).
const LINE_FRAMES: f64 = 2.0;
/// The densest offset is looked for this far around the expected one, to
/// absorb the fitted offset's error.
const SEARCH_FRAMES: f64 = 24.0;
/// Local speeds tried around the fitted one: steps of 0.05% up to ±0.3%.
const LOCAL_STEP: f64 = 0.0005;
const LOCAL_STEPS: i32 = 6;
/// Audio kept on either side of a window so peaks at its edges have their
/// whole neighbourhood.
const MARGIN_SECONDS: f64 = 2.0;

#[derive(Serialize, Deserialize)]
pub struct LossReport {
    pub set: String,
    pub plays: Vec<PlayLoss>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayLoss {
    pub asset: String,
    pub start_seconds: f64,
    pub end_seconds: f64,
    pub speed: f64,
    /// The rung the search analysed the play on, and its distance from the
    /// fitted speed in percent.
    pub nearest_rung: f64,
    pub rung_distance_percent: f64,
    pub windows: Vec<WindowLoss>,
}

#[derive(Serialize, Deserialize)]
pub struct WindowLoss {
    pub start_seconds: f64,
    /// Another identified play overlaps this window.
    pub blended: bool,
    /// Reference hashes whose anchor lies in the stretch heard, per band.
    pub available: Vec<u32>,
    /// Hits on the play's line in the mix, per band: on the nearest rung,
    /// at the fitted speed, and at the best local speed.
    pub mix_rung: Vec<u32>,
    pub mix_fitted: Vec<u32>,
    pub mix_local: Vec<u32>,
    pub local_speed: f64,
    /// The same on a clean render of the stretch at the fitted speed.
    pub clean_rung: Vec<u32>,
    pub clean_fitted: Vec<u32>,
}

pub struct Options<'a> {
    pub set: &'a str,
    pub ladder: &'a [Rung],
    pub jobs: usize,
}

pub fn run(
    sets_dir: &Path,
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    work: &Path,
    options: &Options,
) -> Result<LossReport, String> {
    let Options {
        set: set_name,
        ladder: ladder_rungs,
        jobs,
    } = *options;
    let set = load_set(sets_dir, set_name, library).map_err(|problems| problems.join("; "))?;
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let index = Index::build(&records).map_err(|error| error.to_string())?;
    let audio = decode(&set.audio, profile.sample_rate, Excerpt::default())
        .map_err(|error| error.to_string())?;
    let detections = search(&index, &audio.samples, &profile, ladder_rungs, jobs);

    let accepted: BTreeSet<String> = set
        .tracks
        .iter()
        .flat_map(|track| &track.references)
        .flat_map(|reference| clusters.cluster_of(reference))
        .collect();
    // Duplicate rips of one recording are found together; one play each.
    let mut identified: Vec<&Detection> = Vec::new();
    for detection in &detections {
        let cluster = clusters.cluster_of(&index.asset(detection.asset).path);
        let duplicate = identified.iter().any(|kept| {
            cluster.contains(&index.asset(kept.asset).path)
                && kept.start_seconds < detection.end_seconds
                && detection.start_seconds < kept.end_seconds
        });
        if detection.evidence.confidence() == Confidence::Confident
            && detection.playback == Played::Turntable
            && accepted.contains(&index.asset(detection.asset).path)
            && !duplicate
        {
            identified.push(detection);
        }
    }
    let dir = work.join("loss").join(set_name);
    let plays = map_in_order(&identified, jobs, |detection| {
        let record = records
            .iter()
            .find(|record| record.header.source.path == index.asset(detection.asset).path)
            .ok_or("an identified asset has no record")?;
        let others: Vec<(f64, f64)> = identified
            .iter()
            .filter(|other| {
                clusters.cluster_of(&index.asset(other.asset).path)
                    != clusters.cluster_of(&record.header.source.path)
            })
            .map(|other| (other.start_seconds, other.end_seconds))
            .collect();
        let clean_path = dir.join(format!(
            "{}-{:.0}.wav",
            detection.asset.0, detection.start_seconds
        ));
        analyse_play(
            detection,
            record,
            &audio.samples,
            &others,
            library,
            &clean_path,
        )
    });
    Ok(LossReport {
        set: set_name.to_owned(),
        plays: plays.into_iter().collect::<Result<_, String>>()?,
    })
}

/// The reference hashes of one record: anchor frames by hash, and each
/// anchor's band.
struct Reference {
    anchors: HashMap<u32, Vec<(f64, usize)>>,
    /// Anchor frames of every hash, sorted, with their bands.
    all: Vec<(f64, usize)>,
}

impl Reference {
    fn new(record: &PeakRecord, profile: &Profile) -> Reference {
        let points: Vec<Point> = record.peaks.iter().map(Point::from).collect();
        let mut anchors: HashMap<u32, Vec<(f64, usize)>> = HashMap::new();
        let mut all = Vec::new();
        for_each_pair(&points, |hash, anchor| {
            let frame = points[anchor].frame.round().min(f64::from(MAX_FRAMES - 1));
            let band = band(points[anchor].bin, profile);
            anchors.entry(hash.0).or_default().push((frame, band));
            all.push((frame, band));
        });
        all.sort_by(|a, b| a.0.total_cmp(&b.0));
        Reference { anchors, all }
    }

    /// Reference hashes per band whose anchor lies in `from..to` frames.
    fn available(&self, from: f64, to: f64) -> Vec<u32> {
        let mut counts = vec![0; BAND_EDGES_HZ.len()];
        for &(frame, band) in &self.all {
            if (from..to).contains(&frame) {
                counts[band] += 1;
            }
        }
        counts
    }
}

fn band(bin: f32, profile: &Profile) -> usize {
    let hz = f64::from(bin) * profile.bin_hz();
    BAND_EDGES_HZ
        .iter()
        .position(|&edge| hz < edge)
        .unwrap_or(BAND_EDGES_HZ.len() - 1)
}

/// The play's alignment: track time at query time `t` (seconds).
struct Alignment {
    start: f64,
    track_start: f64,
    speed: f64,
    playback: Played,
}

impl Alignment {
    fn track_at(&self, seconds: f64) -> f64 {
        self.track_start + (seconds - self.start) * self.speed
    }

    fn rung(&self, speed: f64) -> Rung {
        match self.playback {
            Played::Turntable => Rung::Turntable(SpeedRatio(speed)),
            Played::KeyLocked => Rung::KeyLocked(SpeedRatio(speed)),
        }
    }
}

fn analyse_play(
    detection: &Detection,
    record: &PeakRecord,
    mix: &[f32],
    others: &[(f64, f64)],
    library: &Library,
    clean_path: &Path,
) -> Result<PlayLoss, String> {
    let profile = Profile::CURRENT;
    let reference = Reference::new(record, &profile);
    let alignment = Alignment {
        start: detection.start_seconds,
        track_start: detection.track_start_seconds,
        speed: detection.speed.0,
        playback: detection.playback,
    };
    let rungs = match detection.playback {
        Played::Turntable => ladder(),
        Played::KeyLocked => key_lock_ladder(),
    };
    let nearest_rung = rungs
        .iter()
        .map(|rung| rung.speed().0)
        .min_by(|a, b| {
            (a - alignment.speed)
                .abs()
                .total_cmp(&(b - alignment.speed).abs())
        })
        .unwrap_or(alignment.speed);

    // The clean control: the stretch heard, rendered alone at the fitted
    // speed. Its time zero is query time `clean_offset`, a margin before
    // the play unless the track starts there.
    let clean_offset = detection.start_seconds
        - MARGIN_SECONDS.min(detection.track_start_seconds / alignment.speed);
    if !clean_path.exists() {
        if let Some(dir) = clean_path.parent() {
            std::fs::create_dir_all(dir).map_err(|error| error.to_string())?;
        }
        render_excerpt(
            &library.root.join(&record.header.source.path),
            alignment.track_at(clean_offset),
            detection.end_seconds + MARGIN_SECONDS - clean_offset,
            alignment.speed,
            Encoding::Lossless,
            clean_path,
        )?;
    }
    let clean = decode(clean_path, profile.sample_rate, Excerpt::default())
        .map_err(|error| error.to_string())?
        .samples;

    let first_window = (detection.start_seconds / WINDOW_SECONDS).floor() as u32;
    let last_window = (detection.end_seconds / WINDOW_SECONDS).floor() as u32;
    let mut windows = Vec::new();
    for window in first_window..=last_window {
        let from = (f64::from(window) * WINDOW_SECONDS).max(detection.start_seconds);
        let to = (f64::from(window + 1) * WINDOW_SECONDS).min(detection.end_seconds);
        if to - from < 1.0 {
            continue;
        }
        let at = |samples: &[f32], offset: f64, speed: f64| {
            hits(
                samples,
                offset,
                (from, to),
                &alignment,
                alignment.rung(speed),
                &reference,
                &profile,
            )
        };
        let local = (-LOCAL_STEPS..=LOCAL_STEPS)
            .map(|step| alignment.speed * (1.0 + f64::from(step) * LOCAL_STEP))
            .map(|speed| (speed, at(mix, 0.0, speed)))
            .max_by_key(|(_, hits)| hits.iter().sum::<u32>())
            .unwrap_or((alignment.speed, Vec::new()));
        let fps = profile.frames(1.0);
        windows.push(WindowLoss {
            start_seconds: from,
            blended: others.iter().any(|&(start, end)| start < to && from < end),
            available: reference
                .available(alignment.track_at(from) * fps, alignment.track_at(to) * fps),
            mix_rung: at(mix, 0.0, nearest_rung),
            mix_fitted: at(mix, 0.0, alignment.speed),
            mix_local: local.1,
            local_speed: local.0,
            clean_rung: at(&clean, clean_offset, nearest_rung),
            clean_fitted: at(&clean, clean_offset, alignment.speed),
        });
    }
    Ok(PlayLoss {
        asset: record.header.source.path.clone(),
        start_seconds: detection.start_seconds,
        end_seconds: detection.end_seconds,
        speed: alignment.speed,
        nearest_rung,
        rung_distance_percent: (alignment.speed - nearest_rung) * 100.0,
        windows,
    })
}

/// Hits per band on the play's line in query time `span` (seconds) of
/// `samples`, whose time zero is query time `offset`, analysed on `rung`.
/// The line is the densest offset within `SEARCH_FRAMES` of where the
/// alignment puts it.
fn hits(
    samples: &[f32],
    offset: f64,
    span: (f64, f64),
    alignment: &Alignment,
    rung: Rung,
    reference: &Reference,
    profile: &Profile,
) -> Vec<u32> {
    let rate = f64::from(profile.sample_rate);
    let slice_start = (span.0 - MARGIN_SECONDS).max(offset);
    let first = ((slice_start - offset) * rate) as usize;
    let last = (((span.1 + MARGIN_SECONDS - offset) * rate) as usize).min(samples.len());
    if first >= last {
        return vec![0; BAND_EDGES_HZ.len()];
    }
    let speed = rung.speed().0;
    let fps = profile.frames(1.0);
    let points = rung.points(&samples[first..last], profile);
    // (offset from the expected line, band) of every hit.
    let mut found: Vec<(f64, usize)> = Vec::new();
    for_each_pair(&points, |hash, anchor| {
        let point = points[anchor];
        let seconds = slice_start + point.frame / speed / fps;
        if !(span.0..span.1).contains(&seconds) {
            return;
        }
        let expected = alignment.track_at(seconds) * fps;
        if let Some(anchors) = reference.anchors.get(&hash.0) {
            for &(frame, band) in anchors {
                let from_line = frame - expected;
                if from_line.abs() <= SEARCH_FRAMES {
                    found.push((from_line, band));
                }
            }
        }
    });
    found.sort_by(|a, b| a.0.total_cmp(&b.0));
    // The densest span of 2 * LINE_FRAMES.
    let mut best = (0, 0);
    let mut end = 0;
    for start in 0..found.len() {
        end = end.max(start);
        while end < found.len() && found[end].0 - found[start].0 <= 2.0 * LINE_FRAMES {
            end += 1;
        }
        if end - start > best.1 - best.0 {
            best = (start, end);
        }
    }
    let mut counts = vec![0; BAND_EDGES_HZ.len()];
    for &(_, band) in &found[best.0..best.1] {
        counts[band] += 1;
    }
    counts
}

pub fn print_summary(report: &LossReport) {
    let windows: Vec<(&PlayLoss, &WindowLoss)> = report
        .plays
        .iter()
        .flat_map(|play| play.windows.iter().map(move |window| (play, window)))
        .collect();
    let total = |pick: &dyn Fn(&WindowLoss) -> &Vec<u32>, solo: Option<bool>| -> f64 {
        windows
            .iter()
            .filter(|(_, window)| solo.is_none_or(|solo| window.blended != solo))
            .map(|(_, window)| f64::from(pick(window).iter().sum::<u32>()))
            .sum()
    };
    println!(
        "evidence lost in {}: {} identified plays, {} windows",
        report.set,
        report.plays.len(),
        windows.len()
    );
    for (name, solo) in [
        ("all", None),
        ("solo", Some(true)),
        ("blended", Some(false)),
    ] {
        let available = total(&|window| &window.available, solo);
        let share =
            |pick: &dyn Fn(&WindowLoss) -> &Vec<u32>| total(pick, solo) / available.max(1.0);
        println!(
            "  {name:<8} of reference hashes kept: mix {:.1}% on the rung, {:.1}% at the fitted speed, {:.1}% at the local speed; clean {:.1}% on the rung, {:.1}% at the fitted speed",
            100.0 * share(&|window| &window.mix_rung),
            100.0 * share(&|window| &window.mix_fitted),
            100.0 * share(&|window| &window.mix_local),
            100.0 * share(&|window| &window.clean_rung),
            100.0 * share(&|window| &window.clean_fitted),
        );
    }
    print!("  by band, mix/clean at the fitted speed:");
    for (band, edge) in BAND_EDGES_HZ.iter().enumerate() {
        let sum = |pick: &dyn Fn(&WindowLoss) -> &Vec<u32>| -> f64 {
            windows
                .iter()
                .map(|(_, window)| f64::from(pick(window)[band]))
                .sum()
        };
        print!(
            "  <{edge:.0} Hz {:.2}",
            sum(&|window| &window.mix_fitted) / sum(&|window| &window.clean_fitted).max(1.0)
        );
    }
    println!();
    for play in &report.plays {
        let sum = |pick: &dyn Fn(&WindowLoss) -> &Vec<u32>| -> u32 {
            play.windows
                .iter()
                .map(|window| pick(window).iter().sum::<u32>())
                .sum()
        };
        println!(
            "  {:>7.0}-{:<7.0} {:+.3}% (rung {:+.2}%, {:+.3} off): available {:>6}, mix rung {:>5} fitted {:>5} local {:>5}; clean rung {:>5} fitted {:>5}  {}",
            play.start_seconds,
            play.end_seconds,
            (play.speed - 1.0) * 100.0,
            (play.nearest_rung - 1.0) * 100.0,
            play.rung_distance_percent,
            sum(&|window| &window.available),
            sum(&|window| &window.mix_rung),
            sum(&|window| &window.mix_fitted),
            sum(&|window| &window.mix_local),
            sum(&|window| &window.clean_rung),
            sum(&|window| &window.clean_fitted),
            play.asset
        );
    }
}

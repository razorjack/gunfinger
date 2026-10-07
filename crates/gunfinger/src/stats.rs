//! `gunfinger stats`: size and shape of the peak store and the index.

use std::path::Path;

use gunfinger_core::index::{Index, Posting};
use gunfinger_core::indexing::TrackLength;
use gunfinger_core::profile::Profile;
use miette::IntoDiagnostic;
use serde::Serialize;

use crate::Format;
use crate::catalog::Catalog;
use crate::console::Console;

/// The collection size the index must eventually hold.
const PROJECTED_TRACKS: f64 = 25_000.0;
/// Upper edges of the frequency bands peaks are counted in, in Hz: octaves
/// from the bass up.
const BAND_EDGES_HZ: [f64; 6] = [125.0, 250.0, 500.0, 1000.0, 2000.0, 4000.0];

#[derive(Serialize)]
struct Stats {
    schema_version: u32,
    assets: usize,
    audio_seconds: f64,
    mean_track_seconds: f64,
    peaks: Peaks,
    index: IndexSize,
    buckets: Buckets,
    projection: Projection,
}

#[derive(Serialize)]
struct Peaks {
    count: usize,
    per_second: f64,
    store_bytes: u64,
    store_bytes_per_second: f64,
    /// Share of peaks per octave band, lowest first.
    band_shares: Vec<Band>,
}

#[derive(Serialize)]
struct Band {
    up_to_hz: f64,
    share: f64,
}

#[derive(Serialize)]
struct IndexSize {
    postings: usize,
    postings_per_second: f64,
    bytes: usize,
    /// Including the offsets table.
    bytes_per_posting: f64,
    /// The same postings if each list were stored delta-coded with varints
    /// (an offline estimate for the on-disk layout), with the offsets table.
    delta_varint_bytes_per_posting: f64,
}

#[derive(Serialize)]
struct Buckets {
    count: usize,
    mean: f64,
    p99: u32,
    max: u32,
    /// Share of all postings held by the fullest 1% of buckets.
    fullest_percent_share: f64,
}

#[derive(Serialize)]
struct Projection {
    tracks: f64,
    audio_seconds: f64,
    postings: f64,
    index_bytes: f64,
    peak_store_bytes: f64,
}

pub fn run(
    library: &Path,
    peaks_dir: &Path,
    track_length: TrackLength,
    format: Format,
    console: &Console,
) -> miette::Result<()> {
    let catalog = Catalog::open(library, peaks_dir, None, track_length, console)?;
    let stats = measure(&catalog)?;
    match format {
        Format::Human => print_human(&stats),
        Format::Json => println!(
            "{}",
            serde_json::to_string_pretty(&stats).into_diagnostic()?
        ),
    }
    Ok(())
}

fn measure(catalog: &Catalog) -> miette::Result<Stats> {
    let profile = Profile::CURRENT;
    let assets = catalog.index.assets();
    let audio_seconds: f64 = assets.iter().map(|asset| asset.duration_seconds).sum();
    let mut store_bytes = 0;
    for asset in assets {
        store_bytes += catalog.store.record_bytes(&asset.path).into_diagnostic()?;
    }
    let census = peak_census(catalog, &profile);
    let postings = catalog.index.posting_count();
    let index_bytes = catalog.index.size_bytes();
    let mean_track_seconds = audio_seconds / assets.len() as f64;
    let projected_seconds = PROJECTED_TRACKS * mean_track_seconds;
    let postings_per_second = postings as f64 / audio_seconds;
    let posting_bytes = size_of::<Posting>() as f64;
    let offsets_bytes = (index_bytes as f64) - posting_bytes * postings as f64;

    Ok(Stats {
        schema_version: 1,
        assets: assets.len(),
        audio_seconds,
        mean_track_seconds,
        peaks: Peaks {
            count: census.count,
            per_second: census.count as f64 / audio_seconds,
            store_bytes,
            store_bytes_per_second: store_bytes as f64 / audio_seconds,
            band_shares: BAND_EDGES_HZ
                .iter()
                .zip(census.per_band)
                .map(|(&up_to_hz, count)| Band {
                    up_to_hz,
                    share: count as f64 / census.count.max(1) as f64,
                })
                .collect(),
        },
        index: IndexSize {
            postings,
            postings_per_second,
            bytes: index_bytes,
            bytes_per_posting: index_bytes as f64 / postings as f64,
            delta_varint_bytes_per_posting: (offsets_bytes
                + delta_varint_bytes(&catalog.index) as f64)
                / postings as f64,
        },
        buckets: buckets(&catalog.index),
        projection: Projection {
            tracks: PROJECTED_TRACKS,
            audio_seconds: projected_seconds,
            postings: postings_per_second * projected_seconds,
            index_bytes: offsets_bytes + posting_bytes * postings_per_second * projected_seconds,
            peak_store_bytes: store_bytes as f64 / audio_seconds * projected_seconds,
        },
    })
}

struct PeakCensus {
    count: usize,
    per_band: [usize; BAND_EDGES_HZ.len()],
}

/// Peaks of the indexed assets, counted per octave band. The catalog keeps
/// no peak records, so they are read again one at a time.
fn peak_census(catalog: &Catalog, profile: &Profile) -> PeakCensus {
    let mut census = PeakCensus {
        count: 0,
        per_band: [0; BAND_EDGES_HZ.len()],
    };
    for asset in catalog.sources.values() {
        let Ok(record) = catalog.store.load(asset, profile) else {
            continue;
        };
        for peak in &record.peaks {
            let hz = f64::from(peak.bin) * profile.bin_hz();
            let band = BAND_EDGES_HZ
                .iter()
                .position(|&edge| hz < edge)
                .unwrap_or(BAND_EDGES_HZ.len() - 1);
            census.per_band[band] += 1;
            census.count += 1;
        }
    }
    census
}

/// Bytes the posting lists would take delta-coded. A list is ordered by asset
/// and then frame; each posting stores the gap to the previous asset as a
/// varint, then its frame: as a varint gap when the asset repeats, in full
/// (as a varint) when it changes.
fn delta_varint_bytes(index: &Index) -> u64 {
    let mut bytes = 0;
    for list in index.posting_lists() {
        let mut previous: Option<Posting> = None;
        for &posting in list {
            let (asset_gap, frame) = match previous {
                Some(earlier) if earlier.asset() == posting.asset() => {
                    (0, posting.frame() - earlier.frame())
                }
                Some(earlier) => (posting.asset().0 - earlier.asset().0, posting.frame()),
                None => (posting.asset().0, posting.frame()),
            };
            bytes += varint_length(asset_gap) + varint_length(frame);
            previous = Some(posting);
        }
    }
    bytes
}

/// LEB128 length: one byte per started group of seven bits.
fn varint_length(mut value: u32) -> u64 {
    let mut length = 1;
    while value >= 0x80 {
        value >>= 7;
        length += 1;
    }
    length
}

fn buckets(index: &Index) -> Buckets {
    let mut sizes: Vec<u32> = index
        .posting_lists()
        .map(|list| list.len() as u32)
        .collect();
    sizes.sort_unstable();
    let total: u64 = sizes.iter().map(|&size| u64::from(size)).sum();
    let fullest = sizes.len().div_ceil(100);
    let in_fullest: u64 = sizes[sizes.len() - fullest..]
        .iter()
        .map(|&size| u64::from(size))
        .sum();
    Buckets {
        count: sizes.len(),
        mean: total as f64 / sizes.len() as f64,
        p99: sizes[sizes.len() * 99 / 100],
        max: sizes[sizes.len() - 1],
        fullest_percent_share: in_fullest as f64 / total.max(1) as f64,
    }
}

fn print_human(stats: &Stats) {
    let megabytes = |bytes: f64| bytes / 1e6;
    println!(
        "assets            {} ({:.1} h of audio, mean track {:.0} s)",
        stats.assets,
        stats.audio_seconds / 3600.0,
        stats.mean_track_seconds
    );
    println!(
        "peaks             {} ({:.1} per second)",
        stats.peaks.count, stats.peaks.per_second
    );
    println!(
        "peak store        {:.1} MB ({:.0} bytes per second of audio)",
        megabytes(stats.peaks.store_bytes as f64),
        stats.peaks.store_bytes_per_second
    );
    print!("peak bands        ");
    for band in &stats.peaks.band_shares {
        print!("<{:.0} Hz {:.1}%  ", band.up_to_hz, band.share * 100.0);
    }
    println!();
    println!(
        "postings          {} ({:.1} per second of audio)",
        stats.index.postings, stats.index.postings_per_second
    );
    println!(
        "index             {:.1} MB ({:.2} bytes per posting with the offsets table)",
        megabytes(stats.index.bytes as f64),
        stats.index.bytes_per_posting
    );
    println!(
        "delta-varint      {:.2} bytes per posting with the offsets table (offline estimate)",
        stats.index.delta_varint_bytes_per_posting
    );
    println!(
        "hash buckets      {}: mean {:.2}, p99 {}, max {}, fullest 1% hold {:.1}% of postings",
        stats.buckets.count,
        stats.buckets.mean,
        stats.buckets.p99,
        stats.buckets.max,
        stats.buckets.fullest_percent_share * 100.0
    );
    println!(
        "{:.0} tracks      {:.0} h of audio, {:.2e} postings, index {:.0} MB, peak store {:.0} MB",
        stats.projection.tracks,
        stats.projection.audio_seconds / 3600.0,
        stats.projection.postings,
        megabytes(stats.projection.index_bytes),
        megabytes(stats.projection.peak_store_bytes)
    );
}

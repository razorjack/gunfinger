//! Hash rarity (session 85), a diagnostic beside the hit count: each of a
//! detection's aligned hits weighted by how common its hash is in the
//! index searched, an inverse-document-frequency weight over the library.
//! A famous break's evidence is made of the commonest hashes (experiment
//! 0277); the question is whether, at equal hits, shared material scores
//! lower than a record's own.
//!
//! The second pass does not report which hashes it counted, so the count
//! is repeated here at the detection's alignment, as the second pass
//! counts (`search/refine.rs`, `hits_near` and `measure`): the query
//! analysed at the detection's speed, the asset's postings near the line
//! through its first hit, and per 10 s window of query time the densest
//! run within `RUN_FRAMES` when it holds `MIN_RUN_HITS`. Only hits within
//! the detection's span count, since the second pass's first and last
//! hits are its span. The analysis starts at another sample than the
//! second pass's, so the raw count can differ a little from the
//! detection's hits; both are reported. Detections are never changed.

use std::f64::consts::E;

use gunfinger_core::confidence::Pass;
use gunfinger_core::hash::for_each_pair;
use gunfinger_core::index::Index;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::Detection;
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::speed::{Playback, Rung};
use serde::{Deserialize, Serialize};

/// Detections with fewer hits are not measured: below the bands compared.
pub const MIN_HITS: u32 = 30;
/// The second pass's window, run width and minimum run (the matching
/// design's `window=10s line=2fr min-hits=3`).
const WINDOW_SECONDS: f64 = 10.0;
const RUN_FRAMES: f64 = 2.0;
const MIN_RUN_HITS: usize = 3;
/// The second pass keeps hits within 24 frames of its fitted alignment,
/// and the detection's first hit lies up to 24 frames from that.
const SEARCH_FRAMES: f64 = 48.0;
/// The analysis reaches this far beyond the span, as the second pass's.
const MARGIN_SECONDS: f64 = WINDOW_SECONDS;
/// A hit's query frame may lie this far outside the span: the analysis
/// frames fall elsewhere than the second pass's.
const SPAN_TOLERANCE_FRAMES: f64 = 1.0;

/// One detection's hits counted again, with their weights.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rarity {
    /// The hits found again at the detection's alignment.
    pub hits: u32,
    /// The sum of their weights, `weight` of each hash's list length.
    pub weighted: f64,
    /// Of `hits`, those whose list the first pass does not set aside.
    pub kept: u32,
    /// The median list length over the hits.
    pub median_list: u32,
}

/// A hit's weight: `1 / ln(e + L)` for a hash with `L` postings in the
/// index searched, every list counted (brief 85's registration). A hash
/// found once weighs 0.76, one in a list of 1,000 postings 0.14.
pub fn weight(list_length: usize) -> f64 {
    1.0 / (E + list_length as f64).ln()
}

/// `detection` counted again with every hit weighted; `None` below
/// `MIN_HITS`, for a first-pass detection, or for an empty span.
pub fn measure(
    index: &Index,
    detection: &Detection,
    samples: &[f32],
    profile: &Profile,
) -> Option<Rarity> {
    if detection.evidence.hits < MIN_HITS || detection.evidence.pass != Pass::Fitted {
        return None;
    }
    let speed = detection.speed.0;
    let rate = f64::from(profile.sample_rate);
    let first_sample = ((detection.start_seconds - MARGIN_SECONDS).max(0.0) * rate) as usize;
    let last_sample =
        (((detection.end_seconds + MARGIN_SECONDS) * rate) as usize).min(samples.len());
    if first_sample >= last_sample {
        return None;
    }
    let rung = match detection.playback {
        Playback::Turntable => Rung::Turntable(detection.speed),
        Playback::KeyLocked => Rung::KeyLocked(detection.speed),
    };
    let points = rung.signal_points(Signal::Mono(&samples[first_sample..last_sample]), profile);
    let start_frame = profile.frames(first_sample as f64 / rate);
    let span = profile.frames(detection.start_seconds) - SPAN_TOLERANCE_FRAMES
        ..=profile.frames(detection.end_seconds) + SPAN_TOLERANCE_FRAMES;
    let (query_start, reference_start) = (
        profile.frames(detection.start_seconds),
        profile.frames(detection.track_start_seconds),
    );
    let window_frames = profile.frames(WINDOW_SECONDS);
    let mut hits = Vec::new();
    for_each_pair(&points, |hash, anchor| {
        let query_frame = start_frame + points[anchor].frame / speed;
        if !span.contains(&query_frame) {
            return;
        }
        let expected = reference_start + speed * (query_frame - query_start);
        let list = index.postings(hash);
        let kept = !index.scanned_postings(hash).is_empty();
        for posting in index.postings_for_asset(hash, detection.asset) {
            let deviation = f64::from(posting.frame()) - expected;
            if deviation.abs() <= SEARCH_FRAMES {
                hits.push(Hit {
                    window: (query_frame / window_frames) as u32,
                    deviation,
                    list_length: list.len(),
                    kept,
                });
            }
        }
    });
    Some(counted(&mut hits))
}

/// One hit near the alignment.
#[derive(Clone, Copy, Debug)]
struct Hit {
    window: u32,
    deviation: f64,
    list_length: usize,
    kept: bool,
}

/// Each window's densest run of hits within `RUN_FRAMES`, when it holds
/// `MIN_RUN_HITS`, summed.
fn counted(hits: &mut [Hit]) -> Rarity {
    hits.sort_unstable_by(|a, b| {
        a.window
            .cmp(&b.window)
            .then(a.deviation.total_cmp(&b.deviation))
    });
    let runs: Vec<&Hit> = hits
        .chunk_by(|a, b| a.window == b.window)
        .map(|window| &window[densest(window)])
        .filter(|run| run.len() >= MIN_RUN_HITS)
        .flatten()
        .collect();
    let mut lengths: Vec<usize> = runs.iter().map(|hit| hit.list_length).collect();
    lengths.sort_unstable();
    Rarity {
        hits: runs.len() as u32,
        weighted: runs.iter().map(|hit| weight(hit.list_length)).sum(),
        kept: runs.iter().filter(|hit| hit.kept).count() as u32,
        median_list: lengths
            .get(lengths.len() / 2)
            .map_or(0, |&length| length as u32),
    }
}

/// The index range of the largest run of hits (sorted by deviation) whose
/// deviations lie within `RUN_FRAMES`, as the second pass's `densest`.
fn densest(hits: &[Hit]) -> std::ops::Range<usize> {
    let mut best = 0..0;
    let mut end = 0;
    for start in 0..hits.len() {
        end = end.max(start);
        while end < hits.len() && hits[end].deviation - hits[start].deviation <= RUN_FRAMES {
            end += 1;
        }
        if end - start > best.len() {
            best = start..end;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::search::{Matcher, search_signal_with};
    use gunfinger_core::speed::ladder;
    use gunfinger_core::store::{PeakRecord, RecordHeader};

    use super::*;

    #[test]
    fn a_common_hash_weighs_less_than_a_rare_one() {
        assert!((weight(0) - 1.0).abs() < 1e-12);
        assert!((weight(1) - 0.761).abs() < 0.001);
        assert!((weight(1_000) - 0.1448).abs() < 0.001);
        assert!(weight(25) > weight(100) && weight(100) > weight(1_000));
    }

    #[test]
    fn the_count_again_finds_the_second_passs_hits() {
        let profile = Profile::CURRENT;
        let original = track(120.0, profile.sample_rate);
        let index = Matcher::Fitted.index(Index::build(&[record("a.wav", &original)]).unwrap());
        let query = played(&original, 1.0021, 20.0, 60.0, profile.sample_rate);

        let detection = &strongest(&index, &query);
        let rarity = measure(&index, detection, &query, &profile).unwrap();

        assert_eq!(detection.evidence.pass, Pass::Fitted);
        let hits = f64::from(detection.evidence.hits);
        assert!(
            (f64::from(rarity.hits) - hits).abs() <= 0.05 * hits,
            "{} again for {hits}",
            rarity.hits
        );
        // Every hash of a lone record is in a list of its own postings.
        assert!(rarity.weighted < f64::from(rarity.hits) * weight(1) + 1e-9);
        // The fullest lists are set aside even in an index of one record.
        assert!(rarity.kept > 0 && rarity.kept <= rarity.hits);
        assert!(rarity.median_list >= 1);
    }

    #[test]
    fn a_play_whose_hashes_another_file_holds_weighs_less_at_equal_hits() {
        let profile = Profile::CURRENT;
        let original = track(120.0, profile.sample_rate);
        let other = track_with_seed(120.0, profile.sample_rate, 7);
        let alone = Matcher::Fitted
            .index(Index::build(&[record("a.wav", &original), record("b.wav", &other)]).unwrap());
        let shared = Matcher::Fitted.index(
            Index::build(&[record("a.wav", &original), record("copy.wav", &original)]).unwrap(),
        );
        let query = played(&original, 1.0, 20.0, 60.0, profile.sample_rate);

        let in_alone = strongest(&alone, &query);
        let in_shared = strongest(&shared, &query);
        let rare = measure(&alone, &in_alone, &query, &profile).unwrap();
        let common = measure(&shared, &in_shared, &query, &profile).unwrap();

        // The search finds the same hits; only their weights differ.
        assert_eq!(in_alone.evidence.hits, in_shared.evidence.hits);
        assert_eq!(rare.hits, common.hits);
        let mean = |rarity: &Rarity| rarity.weighted / f64::from(rarity.hits);
        assert!(mean(&common) < mean(&rare), "{common:?} {rare:?}");
        assert!(common.median_list >= 2);
    }

    #[test]
    fn weak_and_first_pass_detections_are_not_measured() {
        let profile = Profile::CURRENT;
        let original = track(120.0, profile.sample_rate);
        let index = Matcher::Fitted.index(Index::build(&[record("a.wav", &original)]).unwrap());
        let query = played(&original, 1.0, 20.0, 60.0, profile.sample_rate);
        let detection = strongest(&index, &query);

        let mut weak = detection.clone();
        weak.evidence.hits = MIN_HITS - 1;
        let mut ladder_only = detection.clone();
        ladder_only.evidence.pass = Pass::Ladder;

        assert_eq!(measure(&index, &weak, &query, &profile), None);
        assert_eq!(measure(&index, &ladder_only, &query, &profile), None);
    }

    fn strongest(index: &Index, query: &[f32]) -> Detection {
        search_signal_with(
            index,
            Signal::Mono(query),
            &Profile::CURRENT,
            &ladder(),
            1,
            Matcher::Fitted.options(),
        )
        .into_iter()
        .max_by_key(|detection| detection.evidence.hits)
        .unwrap()
    }

    fn track(seconds: f64, rate: u32) -> Vec<f32> {
        track_with_seed(seconds, rate, 85)
    }

    /// Tone bursts at pseudo-random frequencies, a new pair every 50 ms.
    fn track_with_seed(seconds: f64, rate: u32, seed: u64) -> Vec<f32> {
        let mut state = seed;
        let mut next = move || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            (state >> 33) as f64 / f64::from(1_u32 << 31)
        };
        let burst = (0.05 * f64::from(rate)) as usize;
        let mut samples = vec![0.0_f32; (seconds * f64::from(rate)) as usize];
        for chunk in samples.chunks_mut(burst) {
            let tones = [200.0 + 3300.0 * next(), 200.0 + 3300.0 * next()];
            for (i, sample) in chunk.iter_mut().enumerate() {
                let t = i as f64 / f64::from(rate);
                let envelope = (std::f64::consts::PI * i as f64 / burst as f64).sin();
                let value: f64 = tones
                    .iter()
                    .map(|hz| (2.0 * std::f64::consts::PI * hz * t).sin())
                    .sum();
                *sample = (0.3 * envelope * value) as f32;
            }
        }
        samples
    }

    /// `samples` played at `speed` from `from` seconds, for `seconds`.
    fn played(samples: &[f32], speed: f64, from: f64, seconds: f64, rate: u32) -> Vec<f32> {
        let rate = f64::from(rate);
        (0..(seconds * rate) as usize)
            .map(|i| {
                let at = from * rate + i as f64 * speed;
                let (whole, fraction) = (at.floor() as usize, at.fract() as f32);
                samples[whole] * (1.0 - fraction) + samples[whole + 1] * fraction
            })
            .collect()
    }

    fn record(path: &str, samples: &[f32]) -> PeakRecord {
        let profile = Profile::CURRENT;
        PeakRecord {
            header: RecordHeader {
                profile: profile.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 0,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds: samples.len() as f64 / f64::from(profile.sample_rate),
            },
            peaks: gunfinger_core::peaks::extract_peaks(samples, &profile),
        }
    }
}

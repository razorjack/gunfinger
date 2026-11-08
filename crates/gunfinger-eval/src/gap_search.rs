//! A second-chance search over the gaps (brief 84), a harness variant
//! that `identify` does not run: after the search of the ladder, the
//! stretches of the query that no segment of a confident play covers
//! (`gaps::uncovered`, ADR 0023) are searched again on other rungs only,
//! and the detections of both searches pass the search's last suppression
//! together (`search::joined`; with the default matcher ADR 0021's second
//! suppression). Every detection keeps its own evidence and so meets the
//! same confidence rule.

use gunfinger_core::confidence::Confidence;
use gunfinger_core::gaps::uncovered;
use gunfinger_core::index::Index;
use gunfinger_core::plays;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::{Detection, Options, WINDOW_SECONDS, joined, search_signal_with};
use gunfinger_core::spectrogram::Signal;
use gunfinger_core::speed::{
    Playback, Rung, key_lock_ladder, key_lock_ladder_with_extra_rungs, ladder,
    ladder_with_extra_rungs,
};

/// Shorter stretches are not searched again: they hold less than a window
/// of their own, and a play needs 3 windows to be confident.
pub const MIN_STRETCH_SECONDS: f64 = WINDOW_SECONDS;
/// Each stretch is searched with this much of the covered audio on either
/// side, so that a play that begins or ends under a found one keeps a
/// window of its evidence there.
pub const PADDING_SECONDS: f64 = WINDOW_SECONDS;

/// What is searched over the gaps.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GapSearch {
    /// The rungs past either end of each ladder searched that
    /// `--extra-rungs` would add.
    pub extra_rungs: u32,
    /// The key-locked rungs at the tempos of the turntable ladder searched.
    pub key_lock: bool,
}

impl GapSearch {
    pub fn is_off(self) -> bool {
        self.extra_rungs == 0 && !self.key_lock
    }

    /// The rungs searched over the gaps of a search of `ladder`, none of
    /// them in it.
    pub fn rungs(self, ladder: &[Rung]) -> Vec<Rung> {
        let mut rungs = Vec::new();
        if self.extra_rungs > 0 {
            rungs.extend(beyond(ladder, Playback::Turntable, self.extra_rungs));
            rungs.extend(beyond(ladder, Playback::KeyLocked, self.extra_rungs));
        }
        if self.key_lock {
            rungs.extend(
                ladder
                    .iter()
                    .filter(|rung| rung.playback() == Playback::Turntable)
                    .map(|rung| Rung::KeyLocked(rung.speed())),
            );
        }
        rungs.retain(|rung| !ladder.contains(rung));
        rungs
    }
}

/// The `count` rungs of `playback` past either end of its rungs in
/// `ladder`, which are today's ladder with as many extra rungs past either
/// end: the rungs `--extra-rungs` would add, at the same speeds. None when
/// `ladder` has no rung of `playback`.
fn beyond(ladder: &[Rung], playback: Playback, count: u32) -> Vec<Rung> {
    let (standard, wider): (usize, fn(u32) -> Vec<Rung>) = match playback {
        Playback::Turntable => (self::ladder().len(), ladder_with_extra_rungs),
        Playback::KeyLocked => (key_lock_ladder().len(), key_lock_ladder_with_extra_rungs),
    };
    let present = ladder
        .iter()
        .filter(|rung| rung.playback() == playback)
        .count();
    if present == 0 {
        return Vec::new();
    }
    let extra = present.saturating_sub(standard) / 2;
    wider(extra as u32 + count)
        .into_iter()
        .filter(|rung| !ladder.contains(rung))
        .collect()
}

/// The stretches of a query of `seconds` that the search over the gaps
/// covers, in order: each stretch at least `MIN_STRETCH_SECONDS` long that
/// no segment of a confident play among `detections` covers, widened by
/// `PADDING_SECONDS` either side within the query, overlapping ones merged.
pub fn stretches(detections: &[Detection], seconds: f64) -> Vec<(f64, f64)> {
    let covered: Vec<(f64, f64)> = plays::group(detections)
        .iter()
        .filter(|play| play.confidence() == Confidence::Confident)
        .flat_map(|play| {
            play.segments()
                .iter()
                .map(|segment| (segment.start_seconds, segment.end_seconds))
        })
        .collect();
    let mut stretches: Vec<(f64, f64)> = Vec::new();
    for (start, end) in uncovered(&covered, 0.0, seconds, MIN_STRETCH_SECONDS) {
        let start = (start - PADDING_SECONDS).max(0.0);
        let end = (end + PADDING_SECONDS).min(seconds);
        match stretches.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => stretches.push((start, end)),
        }
    }
    stretches
}

/// One search of a query: what the search over its gaps repeats.
pub struct Search<'a> {
    pub index: &'a Index,
    pub profile: &'a Profile,
    pub ladder: &'a [Rung],
    pub jobs: usize,
    pub options: Options,
}

impl GapSearch {
    /// `first`, the detections of `search` over `signal`, joined with those
    /// of the stretches without a confident match searched on `rungs`; with
    /// the search off, `first` as it is.
    pub fn search(self, search: &Search, signal: Signal, first: Vec<Detection>) -> Vec<Detection> {
        if self.is_off() {
            return first;
        }
        let rungs = self.rungs(search.ladder);
        let rate = f64::from(search.profile.sample_rate);
        let stretches = stretches(&first, signal.len() as f64 / rate);
        let mut detections = first;
        for (start, end) in stretches {
            let from = (start * rate).round() as usize;
            let to = ((end * rate).round() as usize).min(signal.len());
            let offset = from as f64 / rate;
            let found = search_signal_with(
                search.index,
                signal.slice(from..to),
                search.profile,
                &rungs,
                search.jobs,
                search.options,
            );
            detections.extend(found.into_iter().map(|detection| Detection {
                start_seconds: detection.start_seconds + offset,
                end_seconds: detection.end_seconds + offset,
                ..detection
            }));
        }
        joined(detections, search.options)
    }
}

#[cfg(test)]
mod tests {
    use gunfinger_core::confidence::{Evidence, Pass};
    use gunfinger_core::index::AssetId;
    use gunfinger_core::search::Matcher;
    use gunfinger_core::speed::SpeedRatio;

    use super::*;

    fn detection(asset: u32, start_seconds: f64, end_seconds: f64, hits: u32) -> Detection {
        Detection {
            asset: AssetId(asset),
            start_seconds,
            end_seconds,
            track_start_seconds: 0.0,
            track_end_seconds: end_seconds - start_seconds,
            speed: SpeedRatio(1.0),
            playback: Playback::Turntable,
            evidence: Evidence {
                windows: 6,
                hits,
                pass: Pass::Fitted,
            },
        }
    }

    #[test]
    fn the_stretches_without_a_confident_play_are_padded_and_merged() {
        let detections = vec![
            detection(1, 30.0, 100.0, 900),
            detection(2, 105.0, 200.0, 900), // 5 s after the first: too short
            detection(3, 230.0, 300.0, 900), // 30 s after the second
            detection(4, 310.0, 400.0, 900), // 10 s after: padded, it merges
            detection(5, 420.0, 500.0, 100), // possible: covers nothing
            detection(6, 560.0, 590.0, 900),
        ];

        let stretches = stretches(&detections, 600.0);

        assert_eq!(
            stretches,
            [
                (0.0, 40.0),
                (190.0, 240.0),
                (290.0, 320.0),
                (390.0, 570.0),
                (580.0, 600.0)
            ]
        );
    }

    #[test]
    fn a_confident_play_covers_its_possible_segments_too() {
        // One asset at one alignment: a confident segment and a possible
        // one 30 s later make one play, so both cover their time.
        let mut later = detection(1, 130.0, 200.0, 100);
        later.track_start_seconds = 130.0;
        later.track_end_seconds = 200.0;
        let mut strong = detection(1, 0.0, 100.0, 900);
        strong.track_end_seconds = 100.0;

        let stretches = stretches(&[strong, later], 200.0);

        assert_eq!(stretches, [(90.0, 140.0)]);
    }

    #[test]
    fn a_query_without_a_confident_play_is_one_stretch() {
        assert_eq!(
            stretches(&[detection(1, 0.0, 30.0, 100)], 30.0),
            [(0.0, 30.0)]
        );
        assert_eq!(stretches(&[], 8.0), Vec::<(f64, f64)>::new());
    }

    #[test]
    fn extra_rungs_over_the_gaps_are_those_the_wider_ladder_adds() {
        let both: Vec<Rung> = ladder().into_iter().chain(key_lock_ladder()).collect();
        let gap = GapSearch {
            extra_rungs: 2,
            key_lock: false,
        };

        let rungs = gap.rungs(&both);

        let wider: Vec<Rung> = ladder_with_extra_rungs(2)
            .into_iter()
            .chain(key_lock_ladder_with_extra_rungs(2))
            .filter(|rung| !both.contains(rung))
            .collect();
        assert_eq!(rungs, wider);
        assert_eq!(rungs.len(), 8);
        // From a ladder already one rung wider, the next two out.
        let wide = ladder_with_extra_rungs(1);
        assert_eq!(
            gap.rungs(&wide),
            ladder_with_extra_rungs(3)
                .into_iter()
                .filter(|rung| !wide.contains(rung))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn key_lock_over_the_gaps_takes_the_turntable_ladders_tempos() {
        let gap = GapSearch {
            extra_rungs: 0,
            key_lock: true,
        };

        assert_eq!(gap.rungs(&ladder()), key_lock_ladder());
        let both: Vec<Rung> = ladder().into_iter().chain(key_lock_ladder()).collect();
        assert!(gap.rungs(&both).is_empty());
    }

    /// Tone bursts at pseudo-random frequencies, a new pair every 50 ms.
    fn track(seconds: f64, rate: u32) -> Vec<f32> {
        let mut state: u64 = 84;
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

    /// `samples` played at `speed` from its start, for `seconds`.
    fn played(samples: &[f32], speed: f64, seconds: f64, rate: u32) -> Vec<f32> {
        (0..(seconds * f64::from(rate)) as usize)
            .map(|i| {
                let at = i as f64 * speed;
                let (whole, fraction) = (at.floor() as usize, at.fract() as f32);
                samples[whole] * (1.0 - fraction) + samples[whole + 1] * fraction
            })
            .collect()
    }

    fn record(path: &str, samples: &[f32]) -> gunfinger_core::store::PeakRecord {
        use gunfinger_core::library::{Asset, Timestamp};
        use gunfinger_core::store::{PeakRecord, RecordHeader};
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

    #[test]
    fn a_play_past_the_ladders_end_is_found_only_over_the_gap() {
        let profile = Profile::CURRENT;
        let rate = profile.sample_rate;
        let original = track(150.0, rate);
        let index = Matcher::Fitted
            .index(Index::build(&[record("original.wav", &original)]).expect("an index"));
        // 30 s of silence, then the record 9.2% fast: past the ladder's
        // 8%, one rung pair beyond it.
        let mut query = vec![0.0_f32; 30 * rate as usize];
        query.extend(played(&original, 1.092, 90.0, rate));
        let turntable = ladder();
        let search = Search {
            index: &index,
            profile: &profile,
            ladder: &turntable,
            jobs: 1,
            options: Matcher::Fitted.options(),
        };
        let first = search_signal_with(
            &index,
            Signal::Mono(&query),
            &profile,
            &turntable,
            1,
            search.options,
        );
        assert!(
            !first
                .iter()
                .any(|detection| detection.evidence.is_confident())
        );

        let off = GapSearch::default().search(&search, Signal::Mono(&query), first.clone());
        let on = GapSearch {
            extra_rungs: 3,
            key_lock: false,
        }
        .search(&search, Signal::Mono(&query), first.clone());

        assert_eq!(off, first);
        let best = &on[0];
        assert!(best.evidence.is_confident(), "{best:?}");
        assert!((best.speed.0 - 1.092).abs() < 0.004, "{:?}", best.speed);
        // Query times are the whole query's, not the stretch's.
        assert!(
            (best.start_seconds - 30.0).abs() < 2.0,
            "{}",
            best.start_seconds
        );
    }
}

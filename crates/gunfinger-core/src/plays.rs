//! Plays: the detections of one asset that belong to one appearance of the
//! record in the mix.
//!
//! A play can break into several detections. A needle skip moves the
//! position in the track, so the hits move to a new line; a station insert
//! or a cut leaves a gap longer than a chain may bridge. Detections of the
//! same asset separated by a short gap are therefore one play, listed with
//! its segments.

use crate::confidence::{Confidence, Evidence};
use crate::index::AssetId;
use crate::search::Detection;
use crate::speed::SpeedRatio;

/// Longest gap between two segments of one play: room for a station ident,
/// a cut or a skip, short enough that a later return of the record is a new
/// play.
const MAX_GAP_SECONDS: f64 = 90.0;

/// One appearance of an asset in the query.
#[derive(Debug, Clone, PartialEq)]
pub struct Play {
    pub asset: AssetId,
    /// In order of query time; never empty.
    segments: Vec<Detection>,
}

impl Play {
    pub fn segments(&self) -> &[Detection] {
        &self.segments
    }

    pub fn start_seconds(&self) -> f64 {
        self.segments[0].start_seconds
    }

    pub fn end_seconds(&self) -> f64 {
        self.segments[self.segments.len() - 1].end_seconds
    }

    /// The strongest segment's confidence. Segments are not pooled into a
    /// stronger claim: evidence summed across gaps has no calibrated null.
    pub fn confidence(&self) -> Confidence {
        self.strongest().evidence.confidence()
    }

    /// The speed of the strongest segment.
    pub fn speed(&self) -> SpeedRatio {
        self.strongest().speed
    }

    /// The evidence of all segments together, for display.
    pub fn total_evidence(&self) -> Evidence {
        Evidence {
            windows: self
                .segments
                .iter()
                .map(|segment| segment.evidence.windows)
                .sum(),
            hits: self
                .segments
                .iter()
                .map(|segment| segment.evidence.hits)
                .sum(),
        }
    }

    fn strongest(&self) -> &Detection {
        self.segments
            .iter()
            .fold(&self.segments[0], |strongest, segment| {
                if segment.evidence.hits > strongest.evidence.hits {
                    segment
                } else {
                    strongest
                }
            })
    }
}

/// Groups the detections that are at least possible into plays, in order of
/// query time. Weak detections are left out: at chance level they would
/// stretch plays over unrelated audio.
pub fn group(detections: &[Detection]) -> Vec<Play> {
    let mut segments: Vec<&Detection> = detections
        .iter()
        .filter(|detection| detection.evidence.confidence() >= Confidence::Possible)
        .collect();
    segments.sort_by(|a, b| {
        a.asset
            .cmp(&b.asset)
            .then(a.start_seconds.total_cmp(&b.start_seconds))
    });

    let mut plays: Vec<Play> = Vec::new();
    for segment in segments {
        match plays.last_mut() {
            Some(play) if continues(play, segment) => play.segments.push(segment.clone()),
            _ => plays.push(Play {
                asset: segment.asset,
                segments: vec![segment.clone()],
            }),
        }
    }
    plays.sort_by(|a, b| a.start_seconds().total_cmp(&b.start_seconds()));
    plays
}

fn continues(play: &Play, segment: &Detection) -> bool {
    play.asset == segment.asset && segment.start_seconds - play.end_seconds() <= MAX_GAP_SECONDS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confidence::{MIN_HITS, MIN_POSSIBLE_HITS};

    fn detection(asset: u32, start_seconds: f64, end_seconds: f64, hits: u32) -> Detection {
        Detection {
            asset: AssetId(asset),
            start_seconds,
            end_seconds,
            speed: SpeedRatio(1.0 + f64::from(hits) / 1e5),
            evidence: Evidence { windows: 4, hits },
        }
    }

    fn spans(plays: &[Play]) -> Vec<(u32, f64, f64, usize)> {
        plays
            .iter()
            .map(|play| {
                (
                    play.asset.0,
                    play.start_seconds(),
                    play.end_seconds(),
                    play.segments.len(),
                )
            })
            .collect()
    }

    #[test]
    fn segments_of_one_asset_across_a_short_gap_are_one_play() {
        // A needle skip at 2177 s, then a station insert of 34 s.
        let detections = [
            detection(1, 2178.0, 2370.0, 901),
            detection(1, 2090.0, 2177.0, 313),
            detection(2, 4942.0, 4959.0, MIN_POSSIBLE_HITS + 24),
            detection(2, 4993.0, 5069.0, MIN_POSSIBLE_HITS + 24),
        ];

        let plays = group(&detections);

        assert_eq!(
            spans(&plays),
            [(1, 2090.0, 2370.0, 2), (2, 4942.0, 5069.0, 2)]
        );
        assert_eq!(plays[0].confidence(), Confidence::Confident);
        assert_eq!(plays[0].speed(), detections[0].speed);
        assert_eq!(plays[0].total_evidence().hits, 1214);
        assert_eq!(plays[1].confidence(), Confidence::Possible);
    }

    #[test]
    fn a_long_gap_or_another_asset_starts_a_new_play() {
        let detections = [
            detection(1, 0.0, 300.0, MIN_HITS),
            detection(1, 391.0, 500.0, MIN_HITS),
            detection(2, 300.0, 400.0, MIN_HITS),
        ];

        let plays = group(&detections);

        assert_eq!(
            spans(&plays),
            [
                (1, 0.0, 300.0, 1),
                (2, 300.0, 400.0, 1),
                (1, 391.0, 500.0, 1)
            ]
        );
    }

    #[test]
    fn weak_detections_are_left_out() {
        let detections = [
            detection(1, 0.0, 300.0, MIN_HITS),
            detection(1, 320.0, 340.0, MIN_POSSIBLE_HITS - 1),
            detection(2, 100.0, 200.0, 20),
        ];

        let plays = group(&detections);

        assert_eq!(spans(&plays), [(1, 0.0, 300.0, 1)]);
    }
}

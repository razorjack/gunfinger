//! Related recordings: different recordings that share material (a remix
//! and its original, a VIP, a reused sample or break), found by searching
//! every file against the library. When one of them plays, the other can
//! show up as a possible play (ADR 0006); this list says in advance which
//! recordings behave that way, and where in each track the shared part is.

use std::collections::BTreeMap;

use gunfinger_core::confidence::MIN_POSSIBLE_HITS;
use gunfinger_core::library::Library;
use gunfinger_core::store::PeakStore;
use serde::Serialize;

use crate::clusters::{Clusters, Pair, Source, self_match};

/// Pairs down to half the possible threshold are kept, to show what lies
/// just under it.
const KEPT_HITS: u32 = MIN_POSSIBLE_HITS / 2;

#[derive(Serialize)]
pub struct Related {
    pub criterion: String,
    /// One pair per two recordings (duplicate clusters), strongest first.
    pub pairs: Vec<Pair>,
}

pub fn find(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    jobs: usize,
) -> Result<Related, String> {
    let pairs = self_match(library, store, Source::Audio, jobs, |pair| {
        !pair.same_recording && pair.hits >= KEPT_HITS
    })?;
    Ok(Related {
        criterion: format!(
            "aligned evidence of {KEPT_HITS} hits or more between files of different duplicate clusters, searched at speed 0.98..1.02; {MIN_POSSIBLE_HITS} or more is the possible tier"
        ),
        pairs: strongest_per_recording_pair(pairs, clusters),
    })
}

/// Each two recordings once: both search directions and every member of
/// their duplicate clusters give the same relation; the strongest stands
/// for it. Pairs inside one cluster are rips of one recording.
fn strongest_per_recording_pair(pairs: Vec<Pair>, clusters: &Clusters) -> Vec<Pair> {
    let representative = |asset: &str| {
        clusters
            .cluster_of(asset)
            .into_iter()
            .next()
            .unwrap_or_else(|| asset.to_owned())
    };
    let mut strongest: BTreeMap<(String, String), Pair> = BTreeMap::new();
    for pair in pairs {
        let (a, b) = (representative(&pair.query), representative(&pair.found));
        if a == b {
            continue;
        }
        let key = (a.clone().min(b.clone()), a.max(b));
        if strongest.get(&key).is_none_or(|kept| pair.hits > kept.hits) {
            strongest.insert(key, pair);
        }
    }
    let mut pairs: Vec<Pair> = strongest.into_values().collect();
    pairs.sort_by(|a, b| b.hits.cmp(&a.hits).then(a.query.cmp(&b.query)));
    pairs
}

pub fn print_summary(related: &Related) {
    println!("{}", related.criterion);
    for pair in &related.pairs {
        let tier = if pair.hits >= MIN_POSSIBLE_HITS {
            "possible"
        } else {
            "below"
        };
        println!(
            "  {:>4} hits  {tier:<8}  {} {:.0}-{:.0} s  <->  {} {:.0}-{:.0} s",
            pair.hits,
            pair.query,
            pair.query_start_seconds,
            pair.query_end_seconds,
            pair.found,
            pair.found_start_seconds,
            pair.found_end_seconds
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pair(query: &str, found: &str, hits: u32) -> Pair {
        Pair {
            query: query.to_owned(),
            found: found.to_owned(),
            coverage: 0.1,
            speed: 1.0,
            hits,
            same_recording: false,
            query_start_seconds: 0.0,
            query_end_seconds: 0.0,
            found_start_seconds: 0.0,
            found_end_seconds: 0.0,
            owner_verdict: None,
            ladder_coverage: None,
        }
    }

    #[test]
    fn each_two_recordings_appear_once_with_their_strongest_evidence() {
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("a.mp3"), String::from("a-rip.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let pairs = vec![
            pair("a.mp3", "remix.mp3", 70),
            pair("remix.mp3", "a-rip.mp3", 84),
            pair("a.mp3", "a-rip.mp3", 400),
            pair("b.mp3", "c.mp3", 31),
        ];

        let kept = strongest_per_recording_pair(pairs, &clusters);

        let summary: Vec<(&str, &str, u32)> = kept
            .iter()
            .map(|pair| (pair.query.as_str(), pair.found.as_str(), pair.hits))
            .collect();
        assert_eq!(
            summary,
            [("remix.mp3", "a-rip.mp3", 84), ("b.mp3", "c.mp3", 31)]
        );
    }
}

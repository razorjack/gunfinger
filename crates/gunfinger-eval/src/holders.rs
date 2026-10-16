//! The library's map of shared passages (session 21's candidate (c),
//! `shared_mixes::lookup`) for a list of files and spans that no
//! `shared-mixes` run measured, such as the `shared` panel's detections
//! or the plays of an `identify` report (session 23): each span searched
//! against the index without the files its lookup leaves out, and every
//! file found with at least `shared_mixes::LISTED_HOLDER_HITS` hits.
//! Which of them lie outside the file's own recording is the reader's
//! question, under whichever clusters file it uses.

use std::collections::{BTreeMap, BTreeSet};

use gunfinger_core::indexing::load_records;
use gunfinger_core::library::Library;
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::matching::Matching;
use crate::padding::Padding;
use crate::shared_mixes::{Holder, listed, lookup};
use crate::verifier::Verifier;

/// One span of one file to look up. Unknown fields are refused, so a
/// misspelt `left_out` cannot silently search the whole index.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lookup {
    pub name: String,
    pub asset: String,
    pub track_start_seconds: f64,
    pub track_end_seconds: f64,
    /// The files the index leaves out, as the run that found the span did.
    #[serde(default)]
    pub left_out: BTreeSet<String>,
}

#[derive(Serialize)]
pub struct Looked {
    pub name: String,
    pub asset: String,
    pub track_start_seconds: f64,
    pub track_end_seconds: f64,
    pub left_out: Vec<String>,
    pub holders: Vec<Holder>,
    /// Why the span could not be looked up (the file's peaks not at hand).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

pub struct Options<'a> {
    pub matching: &'a Matching,
    pub padding: &'a Padding,
    pub jobs: usize,
}

/// Every lookup, in the order given; one index for each set of files
/// left out.
pub fn run(
    lookups: &[Lookup],
    library: &Library,
    store: &PeakStore,
    options: &Options,
) -> Result<Vec<Looked>, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let verifier = Verifier::new(&records, options.padding.second.as_ref());
    let mut looked: Vec<Option<Looked>> = lookups.iter().map(|_| None).collect();
    for (left_out, positions) in by_left_out(lookups) {
        let index = options
            .matching
            .index(options.padding.index(&records, &profile, &left_out)?);
        let results = map_in_order(&positions, options.jobs, |&position| {
            let lookup_of = &lookups[position];
            let found = verifier.peaks(&lookup_of.asset).map(|peaks| {
                lookup(
                    &index,
                    &peaks,
                    lookup_of.track_start_seconds,
                    lookup_of.track_end_seconds,
                    &profile,
                )
            });
            Looked {
                name: lookup_of.name.clone(),
                asset: lookup_of.asset.clone(),
                track_start_seconds: lookup_of.track_start_seconds,
                track_end_seconds: lookup_of.track_end_seconds,
                left_out: lookup_of.left_out.iter().cloned().collect(),
                error: found
                    .is_none()
                    .then(|| String::from("no peaks of this file at hand")),
                holders: found.map(listed).unwrap_or_default(),
            }
        });
        eprintln!(
            "{} lookups with {} files left out",
            positions.len(),
            left_out.len()
        );
        for (position, result) in positions.into_iter().zip(results) {
            looked[position] = Some(result);
        }
    }
    Ok(looked.into_iter().flatten().collect())
}

/// The lookups' positions, grouped by the files their index leaves out.
fn by_left_out(lookups: &[Lookup]) -> BTreeMap<BTreeSet<String>, Vec<usize>> {
    let mut groups: BTreeMap<BTreeSet<String>, Vec<usize>> = BTreeMap::new();
    for (position, lookup) in lookups.iter().enumerate() {
        groups
            .entry(lookup.left_out.clone())
            .or_default()
            .push(position);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lookups(text: &str) -> Result<Vec<Lookup>, serde_json::Error> {
        serde_json::from_str(text)
    }

    #[test]
    fn lookups_with_the_same_files_left_out_share_an_index() {
        let lookups = lookups(
            r#"[
                {"name": "a", "asset": "x.mp3", "track_start_seconds": 0, "track_end_seconds": 10},
                {"name": "b", "asset": "y.mp3", "track_start_seconds": 0, "track_end_seconds": 10,
                 "left_out": ["p.mp3", "p-rip.mp3"]},
                {"name": "c", "asset": "z.mp3", "track_start_seconds": 0, "track_end_seconds": 10},
                {"name": "d", "asset": "w.mp3", "track_start_seconds": 0, "track_end_seconds": 10,
                 "left_out": ["p-rip.mp3", "p.mp3"]}
            ]"#,
        )
        .unwrap();

        let groups = by_left_out(&lookups);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[&BTreeSet::new()], [0, 2]);
        assert_eq!(
            groups[&BTreeSet::from([String::from("p.mp3"), String::from("p-rip.mp3")])],
            [1, 3]
        );
    }

    #[test]
    fn a_lookup_with_an_unknown_field_is_refused() {
        let misspelt = r#"[{"name": "a", "asset": "x.mp3", "track_start_seconds": 0,
            "track_end_seconds": 10, "left_ot": ["p.mp3"]}]"#;

        assert!(lookups(misspelt).is_err());
        assert!(lookups(&misspelt.replace("left_ot", "left_out")).is_ok());
    }
}

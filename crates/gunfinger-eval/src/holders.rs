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
    /// The other library's files that corpus files stand for, as the
    /// index would name them, each with its corpus file (the copy map).
    pub stood_for: &'a BTreeMap<String, String>,
    pub jobs: usize,
}

/// Every lookup, in the order given; one index for each set of files
/// left out. Refuses lookups that leave out a file the index does not
/// hold.
pub fn run(
    lookups: &[Lookup],
    library: &Library,
    store: &PeakStore,
    options: &Options,
) -> Result<Vec<Looked>, String> {
    let profile = Profile::CURRENT;
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let indexed = |name: &str| {
        records
            .iter()
            .any(|record| record.header.source.path == name)
            || options
                .padding
                .second
                .as_ref()
                .is_some_and(|second| second.holds(name))
    };
    refuse_unindexed(lookups, indexed, options.stood_for)?;
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

/// Leaving out a name the index does not hold leaves nothing out, so a
/// misspelt name would search the whole index. The other library's copy
/// of a corpus file is such a name: the index holds that audio once,
/// under the corpus file's name, so the copy is accepted only beside its
/// corpus file, as when a whole recording is left out (`shared`'s
/// leave-outs). A corpus file without a copy is indexed under its own
/// name, like every corpus file.
fn refuse_unindexed(
    lookups: &[Lookup],
    indexed: impl Fn(&str) -> bool,
    stood_for: &BTreeMap<String, String>,
) -> Result<(), String> {
    let mut refused: BTreeMap<&String, String> = BTreeMap::new();
    for lookup in lookups {
        for name in lookup.left_out.iter().filter(|name| !indexed(name)) {
            match stood_for.get(name) {
                Some(corpus) if lookup.left_out.contains(corpus) => {}
                Some(corpus) => {
                    refused.insert(
                        name,
                        format!(
                            "the index holds this audio as the corpus file {corpus}; leave that out instead"
                        ),
                    );
                }
                None => {
                    refused.insert(name, String::from("no indexed file has this name"));
                }
            }
        }
    }
    if refused.is_empty() {
        return Ok(());
    }
    let reasons: Vec<String> = refused
        .iter()
        .map(|(name, reason)| format!("  {name}: {reason}"))
        .collect();
    Err(format!(
        "{} files to leave out are not in the index, so leaving them out would leave nothing out:\n{}",
        refused.len(),
        reasons.join("\n")
    ))
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
    fn a_file_to_leave_out_that_the_index_does_not_hold_is_refused() {
        let indexed = |name: &str| {
            [
                "Corpus - With A Copy.mp3",
                "Corpus - Without A Copy.mp3",
                "second-library/nas/Other - Track.m4a",
            ]
            .contains(&name)
        };
        let stood_for = BTreeMap::from([(
            String::from("second-library/nas/Corpus - With A Copy.mp3"),
            String::from("Corpus - With A Copy.mp3"),
        )]);
        let leaving_out = |names: &str| {
            lookups(&format!(
                r#"[{{"name": "a", "asset": "x.mp3", "track_start_seconds": 0,
                    "track_end_seconds": 10, "left_out": [{names}]}}]"#
            ))
            .unwrap()
        };

        let indexed_files = leaving_out(
            r#""Corpus - With A Copy.mp3", "Corpus - Without A Copy.mp3",
               "second-library/nas/Other - Track.m4a""#,
        );
        let misspelt = leaving_out(r#""Corpus - Without A Coppy.mp3""#);
        let copy = leaving_out(r#""second-library/nas/Corpus - With A Copy.mp3""#);
        let recording = leaving_out(
            r#""Corpus - With A Copy.mp3", "second-library/nas/Corpus - With A Copy.mp3""#,
        );

        assert_eq!(
            refuse_unindexed(&indexed_files, indexed, &stood_for),
            Ok(())
        );
        assert_eq!(refuse_unindexed(&recording, indexed, &stood_for), Ok(()));
        let refused = refuse_unindexed(&misspelt, indexed, &stood_for).unwrap_err();
        assert!(
            refused.contains("Corpus - Without A Coppy.mp3: no indexed file has this name"),
            "{refused}"
        );
        let refused = refuse_unindexed(&copy, indexed, &stood_for).unwrap_err();
        assert!(
            refused.contains(
                "second-library/nas/Corpus - With A Copy.mp3: the index holds this audio as the corpus file Corpus - With A Copy.mp3; leave that out instead"
            ),
            "{refused}"
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

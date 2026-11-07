//! What a baseline's reports were made from: the harness build, the peak
//! stores' revisions, the digests of the files scoring reads, the profile
//! and the matcher's settings. `baseline` writes it beside the reports as
//! `provenance.json`, and `regress` compares it with the current one.

use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::hash;
use gunfinger_core::indexing::library_revision;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::digest;
use crate::harness::Harness;

/// The file `baseline` writes beside the reports.
pub const FILE: &str = "provenance.json";

/// A peak store's current records under the run's profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoreState {
    pub records: usize,
    /// `indexing::library_revision` of the records' sources, in path
    /// order, as an index of the store alone records it
    /// (`scripts/analysis/store_revision.py` gives the same).
    pub revision: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Provenance {
    /// SHA-256 of the `gunfinger-eval` executable that wrote this.
    pub build: String,
    /// Written by `baseline --provenance-only` for reports saved earlier,
    /// after the current reports were found equal to them: the build and
    /// inputs that reproduced the reports, not those that first wrote them.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub after_the_fact: bool,
    pub set: String,
    pub seed: u64,
    pub corpus_store: StoreState,
    pub other_store: Option<StoreState>,
    /// SHA-256 of each file, or `missing`; for the panels, of a listing
    /// of every file's relative path and SHA-256 (`panels_digest`).
    pub clusters: String,
    pub pair_verdicts: String,
    pub same_titles: String,
    pub panels: String,
    pub profile: String,
    pub hash_design: String,
    pub ladder: String,
    pub extra_rungs: u32,
    /// The matcher's options as the harness parsed them.
    pub matching: String,
    pub verify: bool,
}

impl Provenance {
    pub fn now(harness: &Harness, set: &str, seed: u64, jobs: usize) -> Result<Provenance, String> {
        let exe = std::env::current_exe().map_err(|error| error.to_string())?;
        let profile = harness.search.matching.profile();
        let state = |store: &PeakStore| -> Result<StoreState, String> {
            let (sources, _) = store
                .current_sources(&profile, jobs)
                .map_err(|error| error.to_string())?;
            Ok(StoreState {
                records: sources.len(),
                revision: library_revision(&sources),
            })
        };
        Ok(Provenance {
            build: digest::of_file(&exe)?,
            after_the_fact: false,
            set: set.to_owned(),
            seed,
            corpus_store: state(&harness.store()?)?,
            other_store: harness.other_store()?.as_ref().map(state).transpose()?,
            clusters: file_digest(&harness.clusters_file())?,
            pair_verdicts: file_digest(&harness.paths.verdicts)?,
            same_titles: file_digest(&harness.paths.same_titles)?,
            panels: panels_digest(&harness.paths.panels)?,
            profile: profile.id(),
            hash_design: hash::design(),
            ladder: harness.search.ladder.name().to_owned(),
            extra_rungs: harness.search.extra_rungs,
            matching: format!("{:?}", harness.search.matching),
            verify: harness.search.verify,
        })
    }

    /// The inputs that differ, as `(field, baseline, now)`; the build and
    /// `after_the_fact` are not inputs.
    pub fn differences(&self, now: &Provenance) -> Vec<(&'static str, String, String)> {
        let store = |state: &Option<StoreState>| {
            state.as_ref().map_or_else(
                || String::from("none"),
                |state| format!("{} records, revision {}", state.records, state.revision),
            )
        };
        let fields = [
            ("set", self.set.clone(), now.set.clone()),
            ("seed", self.seed.to_string(), now.seed.to_string()),
            (
                "corpus store",
                store(&Some(self.corpus_store.clone())),
                store(&Some(now.corpus_store.clone())),
            ),
            (
                "other store",
                store(&self.other_store),
                store(&now.other_store),
            ),
            ("clusters", self.clusters.clone(), now.clusters.clone()),
            (
                "pair verdicts",
                self.pair_verdicts.clone(),
                now.pair_verdicts.clone(),
            ),
            (
                "same titles",
                self.same_titles.clone(),
                now.same_titles.clone(),
            ),
            ("panels", self.panels.clone(), now.panels.clone()),
            ("profile", self.profile.clone(), now.profile.clone()),
            (
                "hash design",
                self.hash_design.clone(),
                now.hash_design.clone(),
            ),
            ("ladder", self.ladder.clone(), now.ladder.clone()),
            (
                "extra rungs",
                self.extra_rungs.to_string(),
                now.extra_rungs.to_string(),
            ),
            ("matching", self.matching.clone(), now.matching.clone()),
            ("verify", self.verify.to_string(), now.verify.to_string()),
        ];
        fields
            .into_iter()
            .filter(|(_, before, after)| before != after)
            .collect()
    }
}

fn file_digest(path: &Path) -> Result<String, String> {
    if path.is_file() {
        digest::of_file(path)
    } else {
        Ok(String::from("missing"))
    }
}

/// SHA-256 of the lines `<relative path>\t<SHA-256>\n`, one per file
/// below `dir`, in path order; `missing` without the directory.
pub fn panels_digest(dir: &Path) -> Result<String, String> {
    if !dir.is_dir() {
        return Ok(String::from("missing"));
    }
    let mut files = Vec::new();
    collect_files(dir, &mut files)?;
    files.sort();
    let mut listing = String::new();
    for file in &files {
        let relative = file.strip_prefix(dir).unwrap_or(file);
        listing.push_str(&format!(
            "{}\t{}\n",
            relative.to_string_lossy().replace('\\', "/"),
            digest::of_file(file)?
        ));
    }
    Ok(digest::of_bytes(listing.as_bytes()))
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|error| format!("cannot read {}: {error}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn provenance() -> Provenance {
        Provenance {
            build: String::from("b"),
            after_the_fact: false,
            set: String::from("set"),
            seed: 1,
            corpus_store: StoreState {
                records: 262,
                revision: String::from("r"),
            },
            other_store: None,
            clusters: String::from("c"),
            pair_verdicts: String::from("p"),
            same_titles: String::from("t"),
            panels: String::from("n"),
            profile: String::from("profile"),
            hash_design: String::from("design"),
            ladder: String::from("both"),
            extra_rungs: 0,
            matching: String::from("Matching"),
            verify: false,
        }
    }

    #[test]
    fn the_build_is_not_an_input_and_every_other_field_is() {
        let before = provenance();
        let rebuilt = Provenance {
            build: String::from("other build"),
            after_the_fact: true,
            ..before.clone()
        };
        assert!(before.differences(&rebuilt).is_empty());
        let other = Provenance {
            same_titles: String::from("t2"),
            other_store: Some(StoreState {
                records: 40_666,
                revision: String::from("x"),
            }),
            ..before.clone()
        };
        let fields: Vec<&str> = before
            .differences(&other)
            .into_iter()
            .map(|(field, _, _)| field)
            .collect();
        assert_eq!(fields, ["other store", "same titles"]);
    }

    #[test]
    fn the_panels_digest_follows_names_and_contents() {
        let dir = std::env::temp_dir().join(format!("gunfinger-panels-{}", std::process::id()));
        fs::create_dir_all(dir.join("seed-1")).unwrap();
        fs::write(dir.join("seed-1/panel.json"), "a").unwrap();
        let first = panels_digest(&dir).unwrap();
        fs::write(dir.join("seed-1/panel.json"), "b").unwrap();
        let changed = panels_digest(&dir).unwrap();
        fs::write(dir.join("seed-1/panel.json"), "a").unwrap();
        let again = panels_digest(&dir).unwrap();
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(first, again);
        assert_ne!(first, changed);
        assert_eq!(panels_digest(&dir).unwrap(), "missing");
    }
}

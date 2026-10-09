//! The peak store compared with a library: what `doctor` reports and
//! `prune` removes.

use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, SystemTime};

use gunfinger_core::library::{Asset, Library};
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, SkipReason, Stored};
use miette::IntoDiagnostic;

/// A temporary file younger than this may belong to an `index` still
/// running.
const TEMPORARY_GRACE: Duration = Duration::from_secs(3600);

pub struct Survey {
    pub current: usize,
    /// Records of library files that changed since, or made under another
    /// profile: `index` replaces them.
    pub stale: usize,
    /// Library files with neither a current record nor a skip note.
    pub unindexed: usize,
    pub failed: usize,
    pub too_short: usize,
    pub too_long: usize,
    /// Library files whose tags the store holds.
    pub tagged: usize,
    /// Records and notes of files that are not in the library, because
    /// they are gone or ignored.
    pub orphans: Vec<Orphan>,
    /// Left by writes that never finished, older than an hour.
    pub leftovers: Vec<Stored>,
    pub unreadable: Vec<Stored>,
    pub bytes: u64,
    /// The length of the audio of each current record.
    pub current_lengths: Vec<Duration>,
}

pub struct Orphan {
    pub stored: Stored,
    /// The library path the record or note was made for.
    pub source: String,
    pub absence: Absence,
}

/// Why the file of an orphan is not in the library.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Absence {
    /// Deleted, moved or renamed since it was indexed.
    Gone,
    /// Still there, but the library's ignore file leaves it out.
    Ignored,
}

impl Orphan {
    pub fn kind(&self) -> &'static str {
        match self.stored {
            Stored::Record { .. } => "peak record",
            Stored::Skip { .. } => "skip note",
            Stored::Tags { .. } => "tag note",
            Stored::Unreadable { .. } | Stored::Temporary { .. } => "file",
        }
    }
}

impl Survey {
    pub fn orphans(&self, absence: Absence) -> Vec<&Orphan> {
        self.orphans
            .iter()
            .filter(|orphan| orphan.absence == absence)
            .collect()
    }

    pub fn records_and_notes(&self) -> usize {
        self.current
            + self.stale
            + self.failed
            + self.too_short
            + self.too_long
            + self.tagged
            + self.orphans.len()
    }
}

pub fn survey(library: &Library, store: &PeakStore, profile: &Profile) -> miette::Result<Survey> {
    let assets: BTreeMap<&str, &Asset> = library
        .assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let ignored: BTreeSet<&str> = library.ignored_paths().collect();
    let mut covered: BTreeSet<&str> = BTreeSet::new();
    let mut survey = Survey {
        current: 0,
        stale: 0,
        unindexed: 0,
        failed: 0,
        too_short: 0,
        too_long: 0,
        tagged: 0,
        orphans: Vec::new(),
        leftovers: Vec::new(),
        unreadable: Vec::new(),
        bytes: 0,
        current_lengths: Vec::new(),
    };
    for stored in store.survey().into_diagnostic()? {
        let metadata = std::fs::metadata(stored.file()).ok();
        survey.bytes += metadata.as_ref().map_or(0, std::fs::Metadata::len);
        let (source, matches) = match &stored {
            Stored::Record { header, .. } => {
                let asset = assets.get(header.source.path.as_str());
                let current = asset.is_some_and(|asset| header.is_current(asset, profile));
                if current {
                    survey.current_lengths.push(
                        Duration::try_from_secs_f64(header.duration_seconds).unwrap_or_default(),
                    );
                }
                (&header.source.path, current)
            }
            Stored::Skip { note, .. } => {
                let asset = assets.get(note.source.path.as_str());
                let applies = asset.is_some_and(|asset| note.source == **asset);
                if applies {
                    match note.reason {
                        SkipReason::Failed(_) => survey.failed += 1,
                        SkipReason::TooShort { .. } => survey.too_short += 1,
                        SkipReason::TooLong { .. } => survey.too_long += 1,
                    }
                }
                (&note.source.path, applies)
            }
            Stored::Tags { note, .. } => {
                let asset = assets.get(note.source.path.as_str());
                (
                    &note.source.path,
                    asset.is_some_and(|asset| note.source == **asset),
                )
            }
            Stored::Temporary { .. } => {
                let age = metadata
                    .and_then(|metadata| metadata.modified().ok())
                    .and_then(|modified| SystemTime::now().duration_since(modified).ok());
                if age.is_some_and(|age| age > TEMPORARY_GRACE) {
                    survey.leftovers.push(stored);
                }
                continue;
            }
            Stored::Unreadable { .. } => {
                survey.unreadable.push(stored);
                continue;
            }
        };
        match assets.get_key_value(source.as_str()) {
            Some((&path, _)) if matches => match stored {
                Stored::Tags { .. } => survey.tagged += 1,
                Stored::Record { .. } => {
                    survey.current += 1;
                    covered.insert(path);
                }
                _ => {
                    covered.insert(path);
                }
            },
            // A stale note is ignored by `index` and replaced in time.
            Some(_) => {
                if matches!(stored, Stored::Record { .. }) {
                    survey.stale += 1;
                }
            }
            None => {
                let absence = if ignored.contains(source.as_str()) {
                    Absence::Ignored
                } else {
                    Absence::Gone
                };
                let source = source.clone();
                survey.orphans.push(Orphan {
                    stored,
                    source,
                    absence,
                });
            }
        }
    }
    survey.unindexed = assets.len() - covered.len();
    Ok(survey)
}

/// The kinds of `orphans` counted, and their files, such as `33 peak
/// records and 33 tag notes of 33 files`.
pub fn counted_kinds(orphans: &[&Orphan]) -> String {
    let mut kinds: Vec<String> = Vec::new();
    for kind in ["peak record", "skip note", "tag note"] {
        let count = orphans
            .iter()
            .filter(|orphan| orphan.kind() == kind)
            .count();
        if count > 0 {
            kinds.push(counted(count, kind));
        }
    }
    let files: BTreeSet<&str> = orphans
        .iter()
        .map(|orphan| orphan.source.as_str())
        .collect();
    let listed = match kinds.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, others)) => format!("{} and {last}", others.join(", ")),
        None => String::from("nothing"),
    };
    format!("{listed} of {}", counted(files.len(), "file"))
}

/// `1 file`, `2 files`.
pub fn counted(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}

//! `gunfinger prune`: delete peak records and notes of files that are no
//! longer in the library or that its ignore file leaves out, and temporary
//! files left by interrupted runs.
//!
//! A wrong `--library` (a typo, an unmounted share, another library sharing
//! the store) makes every record look orphaned, so prune lists what it would
//! delete unless `--yes` is given, and refuses outright when the library is
//! empty or most of the store would go. A listing in which files or folders
//! vanished may have missed files that are still there, so prune refuses it
//! too and asks for another run (ADR 0015).

use std::path::Path;

use gunfinger_core::library::Library;
use gunfinger_core::library::ignore::IGNORE_FILE;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::{PeakStore, Stored};
use miette::{IntoDiagnostic, miette};

use crate::catalog::{scan_library, vanished_entries};
use crate::console::Console;
use crate::survey::{Absence, Orphan, counted, counted_kinds, survey};

/// Paths listed per group before `--yes`.
const LISTED: usize = 20;
/// Without `--force`, prune refuses to delete more than this share of the
/// store's records and notes.
const MAX_SHARE: f64 = 0.5;

pub struct Request<'a> {
    pub library: &'a Path,
    pub peaks_dir: &'a Path,
    pub yes: bool,
    pub force: bool,
    pub console: &'a Console,
}

pub fn run(request: &Request) -> miette::Result<()> {
    let library = scan_library(request.library, request.console)?;
    prune(&library, request)
}

/// Prunes the store against `library`, the listing of `request.library`.
fn prune(library: &Library, request: &Request) -> miette::Result<()> {
    if library.vanished > 0 {
        return Err(miette!(
            help = "run `gunfinger prune` again: it prunes only after a listing in which nothing vanished",
            "{} vanished while {} was listed (moved or deleted meanwhile), so files that are still there could look gone; nothing is pruned",
            vanished_entries(library.vanished),
            request.library.display()
        ));
    }
    if library.assets.is_empty() {
        return Err(miette!(
            help = "check --library, and that the share is mounted",
            "the library at {} has no audio files; nothing is pruned",
            request.library.display()
        ));
    }
    let store = PeakStore::open(request.peaks_dir).into_diagnostic()?;
    store.check_library(request.library).into_diagnostic()?;
    let survey = survey(library, &store, &Profile::CURRENT)?;
    let share = survey.orphans.len() as f64 / survey.records_and_notes().max(1) as f64;
    if share > MAX_SHARE && !request.force {
        return Err(miette!(
            help = "check --library; a peak store shared by two libraries holds records of both. --force prunes anyway",
            "{} of {} records and notes in {} are for files not in {}",
            survey.orphans.len(),
            survey.records_and_notes(),
            request.peaks_dir.display(),
            request.library.display()
        ));
    }

    let gone = survey.orphans(Absence::Gone);
    let ignored = survey.orphans(Absence::Ignored);
    let doomed: Vec<&Stored> = survey
        .orphans
        .iter()
        .map(|orphan| &orphan.stored)
        .chain(&survey.leftovers)
        .collect();
    if doomed.is_empty() {
        request.console.info("nothing to prune");
        return Ok(());
    }
    if !request.yes {
        println!("would delete {}:", counted(doomed.len(), "file"));
        list_orphans(
            &format!("{} gone from the library", counted_kinds(&gone)),
            &gone,
        );
        list_orphans(
            &format!("{} that {IGNORE_FILE} leaves out", counted_kinds(&ignored)),
            &ignored,
        );
        let leftovers: Vec<String> = survey
            .leftovers
            .iter()
            .map(|stored| stored.file().display().to_string())
            .collect();
        list(
            &format!(
                "{} left by interrupted runs",
                counted(leftovers.len(), "temporary file")
            ),
            &leftovers,
        );
        println!("run again with --yes to delete them");
        return Ok(());
    }
    let mut freed = 0;
    for stored in &doomed {
        freed += std::fs::metadata(stored.file()).map_or(0, |metadata| metadata.len());
        store.remove(stored).into_diagnostic()?;
    }
    println!(
        "deleted {} files, {:.1} MB",
        doomed.len(),
        freed as f64 / 1e6
    );
    Ok(())
}

/// By path, so that a folder's files are listed together.
fn list_orphans(heading: &str, orphans: &[&Orphan]) {
    let mut items: Vec<String> = orphans
        .iter()
        .map(|orphan| format!("{} ({})", orphan.source, orphan.kind()))
        .collect();
    items.sort();
    list(heading, &items);
}

/// A group of what would be deleted, under its heading; nothing for an
/// empty group.
fn list(heading: &str, items: &[String]) {
    if items.is_empty() {
        return;
    }
    println!("  {heading}:");
    for item in items.iter().take(LISTED) {
        println!("    {item}");
    }
    if items.len() > LISTED {
        println!("    ... and {} more", items.len() - LISTED);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::console::Verbosity;
    use crate::style::ColorChoice;
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::store::TagNote;
    use gunfinger_core::tags::Tags;

    fn asset(path: &str) -> Asset {
        Asset {
            path: path.to_owned(),
            size: 1,
            modified: Timestamp {
                seconds: 0,
                nanos: 0,
            },
        }
    }

    #[test]
    fn a_listing_in_which_entries_vanished_prunes_nothing() {
        let dir = std::env::temp_dir().join(format!("gunfinger-prune-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (root, peaks_dir) = (dir.join("library"), dir.join("peaks"));
        std::fs::create_dir_all(&peaks_dir).unwrap();
        let store = PeakStore::open(&peaks_dir).unwrap();
        let (kept, also_kept, missed) = (asset("a.wav"), asset("b.wav"), asset("c.wav"));
        for source in [&kept, &also_kept, &missed] {
            store
                .save_tags(&TagNote {
                    source: source.clone(),
                    tags: Tags::default(),
                })
                .unwrap();
        }
        // `c.wav` is not in the listing: gone, or in a folder that vanished
        // while it was listed and is still there.
        let mut library = Library {
            root: root.clone(),
            assets: vec![kept, also_kept],
            vanished: 1,
            ..Library::default()
        };
        let console = Console::new(ColorChoice::Never, Verbosity::Quiet);
        let request = Request {
            library: &root,
            peaks_dir: &peaks_dir,
            yes: true,
            force: true,
            console: &console,
        };

        let refused = prune(&library, &request);
        let note_after_refusal = store.tags(&missed).is_some();
        library.vanished = 0;
        let pruned = prune(&library, &request);
        let note_after_prune = store.tags(&missed).is_some();
        std::fs::remove_dir_all(&dir).unwrap();

        let error = format!("{:?}", refused.expect_err("prune must refuse"));
        assert!(
            error.contains("1 file or folder vanished while")
                && error.contains("nothing is pruned"),
            "{error}"
        );
        assert!(
            note_after_refusal,
            "nothing was deleted, even with --yes --force"
        );
        assert!(pruned.is_ok());
        assert!(!note_after_prune, "a complete listing prunes the note");
    }
}

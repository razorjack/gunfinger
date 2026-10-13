//! `index` on a library where one file's decode or header read hangs, as a
//! file on a NAS that stops answering does: a stand-in for FFmpeg or
//! FFprobe, in a temporary folder, hangs on that file and runs the real
//! program for the others. `PATH` is left alone. Skipped when FFmpeg is not
//! installed.

// Fixture helpers run outside `#[test]` functions, where a panic is the
// clearest way to fail the test.
#![allow(clippy::unwrap_used)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};

use gunfinger_core::decode::Decoder;
use gunfinger_core::indexing::{IndexingOptions, Outcome, TrackLength, index_library};
use gunfinger_core::library::Library;
use gunfinger_core::profile::Profile;
use gunfinger_core::store::PeakStore;

fn ffmpeg_available() -> bool {
    let found = Command::new("ffmpeg").arg("-version").output().is_ok();
    if !found {
        eprintln!("skipping stall test: `ffmpeg` is not on PATH");
    }
    found
}

fn scratch(test: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("stalls")
        .join(test);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn tone(path: &Path) {
    let status = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-y", "-f", "lavfi", "-i"])
        .arg("sine=frequency=440:sample_rate=44100:duration=3")
        .arg(path)
        .status()
        .unwrap();
    assert!(status.success());
}

/// Long enough for the real programs on a loaded machine.
const STALL: Duration = Duration::from_secs(5);

/// A stand-in for `program` that runs `on_hangs` for `hangs.wav` and the
/// real program for every other file.
fn stand_in(dir: &Path, program: &str, on_hangs: &str) -> PathBuf {
    let path = dir.join(format!("{program}-{}", on_hangs.replace(' ', "-")));
    fs::write(
        &path,
        format!(
            "#!/bin/sh\ncase \"$*\" in\n  *hangs.wav*) {on_hangs} ;;\n  *) exec {program} \"$@\" ;;\nesac\n"
        ),
    )
    .unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    path
}

/// A library of two short tones, `fine.wav` and `hangs.wav`, and an empty
/// peak store.
fn library(dir: &Path) -> (Library, PeakStore) {
    let library = dir.join("library");
    fs::create_dir_all(&library).unwrap();
    tone(&library.join("fine.wav"));
    tone(&library.join("hangs.wav"));
    (
        Library::scan(&library).unwrap(),
        PeakStore::open(&dir.join("peaks")).unwrap(),
    )
}

fn index(library: &Library, store: &PeakStore, decoder: Decoder) -> Vec<Outcome> {
    index_library(
        library,
        store,
        &Profile::CURRENT,
        &options(decoder),
        |_, _| {},
    )
}

fn options(decoder: Decoder) -> IndexingOptions {
    IndexingOptions {
        jobs: 2,
        length: TrackLength {
            min: Duration::ZERO,
            max: Duration::from_secs(900),
        },
        retry_skipped: false,
        decoder,
    }
}

#[test]
fn a_file_whose_decode_hangs_fails_for_this_run_only() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch("decode");
    let (library, store) = library(&dir);
    let hanging = Decoder {
        ffmpeg: stand_in(&dir, "ffmpeg", "exec sleep 60"),
        stall: STALL,
        ..Decoder::default()
    };
    let started = Instant::now();

    let first = index(&library, &store, hanging);
    let waited = started.elapsed();
    let second = index(&library, &store, Decoder::default());

    assert!(waited < Duration::from_secs(30), "{waited:?}");
    let [fine, hangs] = &first[..] else {
        panic!("{first:?}")
    };
    assert!(matches!(fine, Outcome::Extracted { .. }), "{fine:?}");
    let Outcome::Failed { reason } = hangs else {
        panic!("{hangs:?}")
    };
    assert!(
        reason.contains("hangs.wav could not be read: nothing came from `ffmpeg` for 5 s"),
        "{reason}"
    );
    assert!(store.skip_note(&library.assets[1]).is_none());
    // The next run tries it again, as a file that failed to decode would
    // not be.
    assert!(
        matches!(second[..], [Outcome::UpToDate, Outcome::Extracted { .. }]),
        "{second:?}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_file_whose_header_read_hangs_fails_or_keeps_its_tags_for_the_next_run() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch("probe");
    let (library, store) = library(&dir);
    let decoder = |ffprobe| Decoder {
        ffprobe,
        stall: STALL,
        ..Decoder::default()
    };
    let hanging = stand_in(&dir, "ffprobe", "exec sleep 60");
    let unreadable = stand_in(&dir, "ffprobe", "exit 1");
    let hangs = &library.assets[1];

    let stalled = index(&library, &store, decoder(hanging.clone()));
    assert!(store.skip_note(hangs).is_none());
    // Indexed while its header could not be read: a record without tags.
    let untagged = index(&library, &store, decoder(unreadable));
    assert!(store.tags(hangs).is_none());
    let stalled_tags = index(&library, &store, decoder(hanging));
    assert!(store.tags(hangs).is_none());
    let tagged = index(&library, &store, Decoder::default());

    let [Outcome::Extracted { .. }, Outcome::Failed { reason }] = &stalled[..] else {
        panic!("{stalled:?}")
    };
    assert!(
        reason.contains("hangs.wav could not be read: nothing came from `ffprobe` for 5 s"),
        "{reason}"
    );
    assert!(
        matches!(untagged[..], [Outcome::UpToDate, Outcome::Extracted { .. }]),
        "{untagged:?}"
    );
    assert!(
        matches!(stalled_tags[..], [Outcome::UpToDate, Outcome::UpToDate]),
        "{stalled_tags:?}"
    );
    assert!(
        matches!(tagged[..], [Outcome::UpToDate, Outcome::Tagged]),
        "{tagged:?}"
    );
    assert!(store.tags(hangs).is_some());
    let _ = fs::remove_dir_all(&dir);
}

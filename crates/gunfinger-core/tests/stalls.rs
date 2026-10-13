//! `index` on a library where one file's decode hangs, as a file on a NAS
//! that stops answering does: a stand-in for FFmpeg, in a temporary folder,
//! hangs on that file and runs the real `ffmpeg` for the others. `PATH` is
//! left alone. Skipped when FFmpeg is not installed.

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
    let dir = scratch("index");
    let library = dir.join("library");
    fs::create_dir_all(&library).unwrap();
    tone(&library.join("fine.wav"));
    tone(&library.join("hangs.wav"));
    let ffmpeg = dir.join("ffmpeg");
    fs::write(
        &ffmpeg,
        "#!/bin/sh\ncase \"$*\" in\n  *hangs.wav*) exec sleep 30 ;;\n  *) exec ffmpeg \"$@\" ;;\nesac\n",
    )
    .unwrap();
    fs::set_permissions(&ffmpeg, fs::Permissions::from_mode(0o755)).unwrap();
    let hanging = Decoder {
        ffmpeg,
        ffprobe: PathBuf::from("ffprobe"),
        stall: Duration::from_millis(500),
    };
    let library = Library::scan(&library).unwrap();
    let store = PeakStore::open(&dir.join("peaks")).unwrap();
    let started = Instant::now();

    let first = index_library(
        &library,
        &store,
        &Profile::CURRENT,
        &options(hanging),
        |_, _| {},
    );
    let waited = started.elapsed();
    let second = index_library(
        &library,
        &store,
        &Profile::CURRENT,
        &options(Decoder::default()),
        |_, _| {},
    );

    assert!(waited < Duration::from_secs(10), "{waited:?}");
    let [fine, hangs] = &first[..] else {
        panic!("{first:?}")
    };
    assert!(matches!(fine, Outcome::Extracted { .. }), "{fine:?}");
    let Outcome::Failed { reason } = hangs else {
        panic!("{hangs:?}")
    };
    assert!(
        reason.contains("hangs.wav could not be read: nothing came from `ffmpeg` for 0.5 s"),
        "{reason}"
    );
    assert!(store.skip_note(&library.assets[1]).is_none());
    // The next run tries it again, as a file that failed to decode would
    // not be.
    assert!(
        matches!(second[..], [Outcome::UpToDate, Outcome::Extracted { .. }]),
        "{second:?}"
    );
}

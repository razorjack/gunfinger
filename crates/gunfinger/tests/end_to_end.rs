//! End to end through the real binary, on synthetic music: index a library,
//! identify a mix built from it, and check what is found where.
//!
//! The mix, 150 s:
//!
//! | mix time | content |
//! |----------|---------|
//! | 0 to 42 s | track A from 10 s, at +3% |
//! | 38 to 82 s | track B from 5 s, at -5% (crossfade with A from 38 to 42 s) |
//! | 82 to 96 s | an insert that is not in the library |
//! | 96 to 112 s | track C from 20 s, at +6% |
//! | 112 to 140 s | track C again, 4 s further on: a needle skip |
//!
//! Track D is in the library but never played; `copy-of-a.wav` is a copy of
//! track A's file. Skipped when FFmpeg is not installed.

// Test helpers panic on failure: that is the clearest way to fail a test.
#![allow(clippy::unwrap_used)]

mod support;

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;
use support::{Mix, Track, write_wav};

fn ffmpeg_available() -> bool {
    let found = Command::new("ffmpeg").arg("-version").output().is_ok();
    if !found {
        eprintln!("skipping end-to-end test: `ffmpeg` is not on PATH");
    }
    found
}

fn scratch_dir(test: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(test);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn gunfinger(dir: &Path, args: &[&str]) -> Output {
    let output = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
        .args(args)
        .arg("--peaks-dir")
        .arg(dir.join("peaks"))
        .env_remove("GUNFINGER_PEAKS_DIR")
        .env_remove("GUNFINGER_JOBS")
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "gunfinger {args:?} failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

/// Builds the library and the mix of the module comment; returns the mix.
fn build_corpus(dir: &Path) -> PathBuf {
    let tracks: Vec<Track> = (1..=5).map(|seed| Track::random(seed, 75.0)).collect();
    let [a, b, c, d, insert] = &tracks[..] else {
        unreachable!()
    };
    for (name, track) in [("a.wav", a), ("b.wav", b), ("c.wav", c), ("d.wav", d)] {
        write_wav(
            &dir.join("library").join(name),
            &track.play(0.0, track.seconds, 1.0),
        );
    }
    std::fs::copy(dir.join("library/a.wav"), dir.join("library/copy-of-a.wav")).unwrap();

    let mut mix = Mix::new(150.0);
    mix.add(0.0, &a.play(10.0, 42.0, 1.03), 4.0);
    mix.add(38.0, &b.play(5.0, 44.0, 0.95), 4.0);
    mix.add(82.0, &insert.play(0.0, 14.0, 1.0), 0.5);
    mix.add(96.0, &c.play(20.0, 16.0, 1.06), 0.5);
    let after_skip = 20.0 + 16.0 * 1.06 + 4.0;
    mix.add(112.0, &c.play(after_skip, 28.0, 1.06), 0.01);
    let path = dir.join("mix.wav");
    write_wav(&path, mix.samples());
    path
}

fn plays_of<'a>(plays: &'a [Value], asset: &str) -> Vec<&'a Value> {
    plays.iter().filter(|play| play["asset"] == asset).collect()
}

fn seconds(play: &Value, field: &str) -> f64 {
    play[field].as_f64().unwrap()
}

/// Checks the reported position in the track against where the track was
/// started (`track_from`) at mix time `mix_from`.
fn assert_track_position(segment: &Value, mix_from: f64, track_from: f64, speed: f64) {
    for (mix_field, track_field) in [
        ("start_seconds", "track_start_seconds"),
        ("end_seconds", "track_end_seconds"),
    ] {
        let expected = track_from + (seconds(segment, mix_field) - mix_from) * speed;
        assert!(
            (seconds(segment, track_field) - expected).abs() < 0.1,
            "{track_field} should be {expected:.2}: {segment}"
        );
    }
}

#[test]
fn a_synthetic_mix_is_identified_end_to_end() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("end-to-end");
    let mix = build_corpus(&dir);
    let library = dir.join("library");
    let library = library.to_str().unwrap();

    gunfinger(&dir, &["index", library]);
    let output = gunfinger(
        &dir,
        &[
            "identify",
            mix.to_str().unwrap(),
            "--library",
            library,
            "--format",
            "json",
        ],
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let plays = report["plays"].as_array().unwrap();

    let saved = dir.join("report.json");
    std::fs::write(&saved, &output.stdout).unwrap();
    let tracklist = gunfinger(
        &dir,
        &["show", saved.to_str().unwrap(), "--format", "tracklist"],
    );
    assert_eq!(
        String::from_utf8_lossy(&tracklist.stdout),
        " 1.    0:00  a\n 2.    0:38  b\n 3.    1:36  c\n",
        "the saved report renders as a tracklist named after the untagged files"
    );

    for (asset, speed, from, to, track_from) in [
        ("a.wav", 1.03, 0.0, 42.0, 10.0),
        ("b.wav", 0.95, 38.0, 82.0, 5.0),
        ("c.wav", 1.06, 96.0, 140.0, 20.0),
    ] {
        let found = plays_of(plays, asset);
        assert_eq!(found.len(), 1, "{asset}: {found:?}");
        let play = found[0];
        assert_eq!(play["confidence"], "confident", "{asset}");
        assert!(
            (seconds(play, "speed") - speed).abs() < 0.001,
            "{asset}: speed {}",
            play["speed"]
        );
        assert!(
            (seconds(play, "start_seconds") - from).abs() < 5.0
                && (seconds(play, "end_seconds") - to).abs() < 5.0,
            "{asset}: {play}"
        );
        assert_track_position(&play["segments"][0], from, track_from, speed);
    }

    let a = plays_of(plays, "a.wav")[0];
    assert_eq!(
        a["same_audio"],
        serde_json::json!(["copy-of-a.wav"]),
        "the copy is listed with A, not as a play of its own: {a}"
    );

    let c = plays_of(plays, "c.wav")[0];
    let segments = c["segments"].as_array().unwrap();
    assert_eq!(
        segments.len(),
        2,
        "the needle skip splits C into two segments of one play: {c}"
    );
    assert!(
        (seconds(&segments[0], "end_seconds") - 112.0).abs() < 2.0
            && (seconds(&segments[1], "start_seconds") - 112.0).abs() < 2.0,
        "the segments meet at the skip: {c}"
    );
    let after_skip = 20.0 + 16.0 * 1.06 + 4.0;
    assert_track_position(&segments[1], 112.0, after_skip, 1.06);
    assert!(plays_of(plays, "d.wav").is_empty(), "D is never played");
    for play in plays {
        let inside_insert =
            seconds(play, "start_seconds") > 84.0 && seconds(play, "end_seconds") < 94.0;
        assert!(!inside_insert, "nothing plays inside the insert: {play}");
    }
}

#[test]
fn a_damaged_file_is_passed_over_until_it_changes() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("damaged-file");
    let library = dir.join("library");
    std::fs::create_dir_all(&library).unwrap();
    let damaged = library.join("damaged.mp3");
    std::fs::write(&damaged, b"not audio at all").unwrap();
    let library = library.to_str().unwrap();
    let index = || {
        let output = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
            .args(["index", library, "--peaks-dir"])
            .arg(dir.join("peaks"))
            .env("XDG_CONFIG_HOME", dir.join("config"))
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stderr).into_owned()
    };

    let first = index();
    let second = index();
    std::fs::write(&damaged, b"still not audio, and changed").unwrap();
    let after_change = index();

    assert!(first.contains("1 failed, 0 passed over"), "{first}");
    assert!(second.contains("0 failed, 1 passed over"), "{second}");
    assert!(
        after_change.contains("1 failed, 0 passed over"),
        "{after_change}"
    );
}

#[test]
fn prune_deletes_records_of_removed_files_only_when_asked() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("prune");
    let library = dir.join("library");
    for seed in [1, 2] {
        let track = Track::random(seed, 10.0);
        write_wav(
            &library.join(format!("{seed}.wav")),
            &track.play(0.0, track.seconds, 1.0),
        );
    }
    let library_arg = library.to_str().unwrap();
    gunfinger(&dir, &["index", library_arg]);
    std::fs::remove_file(library.join("2.wav")).unwrap();

    let listed = gunfinger(&dir, &["prune", "--library", library_arg]);
    let doctor = gunfinger(&dir, &["doctor", "--library", library_arg]);
    gunfinger(&dir, &["prune", "--library", library_arg, "--yes"]);
    let records = std::fs::read_dir(dir.join("peaks")).unwrap().count();

    let listed = String::from_utf8_lossy(&listed.stdout);
    assert!(
        listed.contains("would delete 1 files:\n  2.wav"),
        "{listed}"
    );
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        doctor.contains("1 records or notes are for files no longer in this library"),
        "{doctor}"
    );
    assert_eq!(records, 1, "only 1.wav's record is left");
}

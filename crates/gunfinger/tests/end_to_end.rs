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
//!
//! The tracks repeat their kick every bar, as drum & bass does. The default
//! matcher measures each candidate again over its span and up to 10 s
//! before it; after the skip, the kicks of the 4 s before it lie within
//! that measurement's reach of the new alignment, so the stronger part of C
//! takes them in and the part before the skip, which overlaps it, is
//! dropped. The single-pass matcher keeps both parts, as segments of one
//! play.

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
        .env("XDG_CACHE_HOME", dir.join("cache"))
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
    let identify = |extra: &[&str]| {
        let mut args = vec![
            "identify",
            mix.to_str().unwrap(),
            "--library",
            library,
            "--format",
            "json",
        ];
        args.extend(extra);
        gunfinger(&dir, &args)
    };
    let output = identify(&["--single-pass"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let plays = report["plays"].as_array().unwrap();
    let default: Value = serde_json::from_slice(&identify(&[]).stdout).unwrap();
    let default_plays = default["plays"].as_array().unwrap();

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

    for (plays, asset, speed, from, to, track_from) in [
        (plays, "a.wav", 1.03, 0.0, 42.0, 10.0),
        (plays, "b.wav", 0.95, 38.0, 82.0, 5.0),
        (plays, "c.wav", 1.06, 96.0, 140.0, 20.0),
        (default_plays, "a.wav", 1.03, 0.0, 42.0, 10.0),
        (default_plays, "b.wav", 0.95, 38.0, 82.0, 5.0),
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
    let c = plays_of(default_plays, "c.wav");
    assert_eq!(c.len(), 1, "{c:?}");
    assert_eq!(c[0]["confidence"], "confident", "{}", c[0]);
    assert!(
        (seconds(c[0], "speed") - 1.06).abs() < 0.001
            && (seconds(c[0], "end_seconds") - 140.0).abs() < 5.0,
        "the default matcher finds C after the skip: {}",
        c[0]
    );
    for plays in [plays, default_plays] {
        assert!(plays_of(plays, "d.wav").is_empty(), "D is never played");
        for play in plays {
            let inside_insert =
                seconds(play, "start_seconds") > 84.0 && seconds(play, "end_seconds") < 94.0;
            assert!(!inside_insert, "nothing plays inside the insert: {play}");
        }
    }

    let explained = gunfinger(
        &dir,
        &[
            "explain",
            mix.to_str().unwrap(),
            "--library",
            library,
            "--at",
            "1:05",
            "--around",
            "0:30",
            "--asset",
            "b.wav",
            "--windows",
        ],
    );
    let explained = String::from_utf8_lossy(&explained.stdout);
    let window_rows: Vec<&str> = explained
        .lines()
        .skip_while(|line| !line.starts_with("b.wav: lines per window"))
        .filter(|line| line.starts_with("  0:"))
        .collect();
    assert!(
        window_rows
            .iter()
            .any(|row| row.starts_with("  0:40-0:50") && row.ends_with("#1")),
        "the window grid starts at 0:30, and B's play takes a line in 0:40-0:50:\n{explained}"
    );
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
            .env("XDG_CACHE_HOME", dir.join("cache"))
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
fn only_files_that_need_indexing_are_warned_about() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("left-out");
    let library = dir.join("library");
    std::fs::create_dir_all(&library).unwrap();
    let track = Track::random(1, 20.0);
    write_wav(&library.join("a.wav"), &track.play(0.0, track.seconds, 1.0));
    std::fs::write(library.join("damaged.mp3"), b"not audio at all").unwrap();
    let library_arg = library.to_str().unwrap();
    let _ = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
        .args(["index", library_arg, "--peaks-dir"])
        .arg(dir.join("peaks"))
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .env("XDG_CACHE_HOME", dir.join("cache"))
        .output()
        .unwrap();
    let late = Track::random(2, 20.0);
    write_wav(
        &library.join("late.wav"),
        &late.play(0.0, late.seconds, 1.0),
    );
    let recording = library.join("a.wav");
    let identify = |extra: &[&str]| {
        let mut args = vec![
            "identify",
            recording.to_str().unwrap(),
            "--library",
            library_arg,
        ];
        args.extend_from_slice(extra);
        String::from_utf8_lossy(&gunfinger(&dir, &args).stderr).into_owned()
    };

    let normal = identify(&[]);
    let verbose = identify(&["--verbose"]);

    assert!(
        normal.contains("1 library file is not indexed") && normal.contains("--verbose lists them"),
        "{normal}"
    );
    assert!(!normal.contains("damaged.mp3"), "{normal}");
    assert!(!normal.contains("searched"), "{normal}");
    assert!(verbose.contains("\n  late.wav"), "{verbose}");
    assert!(
        verbose.contains("damaged.mp3 (it failed to decode)"),
        "{verbose}"
    );
    assert!(verbose.contains("searched"), "{verbose}");
}

#[test]
fn files_outside_the_track_length_are_passed_over_and_left_out() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("track-length");
    let library = dir.join("library");
    for (name, seed, seconds) in [
        ("loop.wav", 1, 4.0),
        ("a.wav", 2, 20.0),
        ("b.wav", 3, 30.0),
        ("set.wav", 4, 80.0),
    ] {
        let track = Track::random(seed, seconds);
        write_wav(&library.join(name), &track.play(0.0, track.seconds, 1.0));
    }
    let library_arg = library.to_str().unwrap();
    let range = ["--min-track", "10", "--max-track", "1:00"];
    let stderr = |output: Output| String::from_utf8_lossy(&output.stderr).into_owned();

    let first = stderr(gunfinger(
        &dir,
        &[&["index", library_arg][..], &range].concat(),
    ));
    let again = stderr(gunfinger(
        &dir,
        &[&["index", library_arg][..], &range].concat(),
    ));
    let doctor = gunfinger(
        &dir,
        &[&["doctor", "--library", library_arg][..], &range].concat(),
    );
    let doctor_narrowed = gunfinger(
        &dir,
        &["doctor", "--library", library_arg, "--max-track", "25"],
    );
    let recording = library.join("a.wav");
    let narrowed = stderr(gunfinger(
        &dir,
        &[
            "identify",
            recording.to_str().unwrap(),
            "--library",
            library_arg,
            "--max-track",
            "25",
            "--verbose",
        ],
    ));
    let none_in_range = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
        .args(["stats", "--library", library_arg, "--max-track", "15"])
        .arg("--peaks-dir")
        .arg(dir.join("peaks"))
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .env("XDG_CACHE_HOME", dir.join("cache"))
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let widened = stderr(gunfinger(
        &dir,
        &["index", library_arg, "--max-track", "2:00"],
    ));

    assert!(
        first.contains("skipped loop.wav: shorter than 0:10"),
        "{first}"
    );
    assert!(
        first.contains("skipped set.wav: longer than 1:00"),
        "{first}"
    );
    assert!(first.contains(": 2 extracted"), "{first}");
    assert!(again.contains("2 passed over as before"), "{again}");
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    assert!(doctor.contains("track length 0:10 to 1:00"), "{doctor}");
    assert!(
        doctor.contains("2 files passed over: 0 failed to decode, 1 too short, 1 too long"),
        "{doctor}"
    );
    assert!(doctor.contains("2 of 65536 assets"), "{doctor}");
    assert!(doctor.contains("2 audio files not counted"), "{doctor}");
    let doctor_narrowed = String::from_utf8_lossy(&doctor_narrowed.stdout);
    assert!(
        doctor_narrowed.contains("1 of 65536 assets"),
        "{doctor_narrowed}"
    );
    assert!(
        doctor_narrowed.contains("longest track 0:20"),
        "{doctor_narrowed}"
    );
    assert!(
        narrowed.contains(
            "1 file left out because the track length is up to 0:25 (--min-track, --max-track):\n  b.wav (0:30)"
        ),
        "{narrowed}"
    );
    assert!(!none_in_range.status.success());
    // The report wraps long lines, marking each continuation with `│`.
    let none_in_range = String::from_utf8_lossy(&none_in_range.stderr)
        .replace('│', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        none_in_range.contains(
            "the track length range (up to 0:15) leaves out every file with a peak record (2 files, 0:20 to 0:30)"
        ),
        "{none_in_range}"
    );
    assert!(
        none_in_range.contains("widen it with --min-track and --max-track"),
        "{none_in_range}"
    );
    assert!(widened.contains(": 2 extracted"), "{widened}");
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
    let records = std::fs::read_dir(dir.join("peaks"))
        .unwrap()
        .filter(|entry| {
            entry
                .as_ref()
                .unwrap()
                .path()
                .extension()
                .is_some_and(|extension| extension == "peaks")
        })
        .count();

    let listed = String::from_utf8_lossy(&listed.stdout);
    assert!(
        listed.contains(
            "would delete 2 files:\n  1 peak record and 1 tag note of 1 file gone from the library:\n    2.wav (peak record)\n    2.wav (tag note)"
        ),
        "{listed}"
    );
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        doctor.contains("1 peak record and 1 tag note of 1 file gone from this library"),
        "{doctor}"
    );
    assert!(
        doctor.contains("no .gunfingerignore at the library root"),
        "{doctor}"
    );
    assert_eq!(records, 1, "only 1.wav's record is left");
}

#[test]
fn ignored_files_are_left_out_and_told_apart_from_removed_ones() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("ignore");
    let library = dir.join("library");
    // Few enough ignored and removed files that prune does not refuse.
    for (seed, name) in [
        (1, "1.wav"),
        (2, "2.wav"),
        (3, "Mixed CD [Virus]/CD1/3.wav"),
        (4, "Mixed CD [Virus]/CD2/4.wav"),
        (5, "5.wav"),
        (6, "6.wav"),
        (7, "7.wav"),
    ] {
        let track = Track::random(seed, 10.0);
        write_wav(&library.join(name), &track.play(0.0, track.seconds, 1.0));
    }
    let library_arg = library.to_str().unwrap();
    gunfinger(&dir, &["index", library_arg]);
    std::fs::remove_file(library.join("2.wav")).unwrap();
    std::fs::write(
        library.join(".gunfingerignore"),
        "# Mixed CDs\n/Mixed CD [Virus]/\n/Renamed since/\n",
    )
    .unwrap();

    let listed = gunfinger(&dir, &["prune", "--library", library_arg]);
    let doctor = gunfinger(&dir, &["doctor", "--library", library_arg]);
    let indexed = gunfinger(&dir, &["index", library_arg]);
    let stats = gunfinger(&dir, &["stats", "--library", library_arg]);
    gunfinger(&dir, &["prune", "--library", library_arg, "--yes"]);
    let pruned = gunfinger(&dir, &["doctor", "--library", library_arg]);

    let listed = String::from_utf8_lossy(&listed.stdout);
    assert!(listed.contains("would delete 6 files:"), "{listed}");
    assert!(
        listed.contains(
            "  1 peak record and 1 tag note of 1 file gone from the library:\n    2.wav (peak record)\n    2.wav (tag note)\n"
        ),
        "{listed}"
    );
    assert!(
        listed.contains(
            "  2 peak records and 2 tag notes of 2 files that .gunfingerignore leaves out:\n"
        ),
        "{listed}"
    );
    assert!(
        listed.contains(
            "    Mixed CD [Virus]/CD1/3.wav (peak record)\n    Mixed CD [Virus]/CD1/3.wav (tag note)\n    Mixed CD [Virus]/CD2/4.wav (peak record)\n    Mixed CD [Virus]/CD2/4.wav (tag note)\n"
        ),
        "{listed}"
    );
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    for line in [
        "ok       4 audio files, 1 other files and folders passed over",
        &format!(
            "ok       read {}: 2 patterns, 2 audio files left out",
            library.join(".gunfingerignore").display()
        ),
        "ok       line 2, `/Mixed CD [Virus]/`, leaves out 2 audio files",
        "warning  line 3, `/Renamed since/`, leaves out no audio file: check it for a typo or a renamed folder",
        "warning  1 peak record and 1 tag note of 1 file gone from this library: `gunfinger prune` lists them and `gunfinger prune --yes` deletes them",
        "warning  2 peak records and 2 tag notes of 2 files that .gunfingerignore leaves out: `gunfinger prune` lists them and `gunfinger prune --yes` deletes them; until then, searches with --store-only still use their peak records",
    ] {
        assert!(doctor.contains(line), "{line} in {doctor}");
    }
    let indexed = String::from_utf8_lossy(&indexed.stderr);
    assert!(indexed.contains("indexing 4 audio files"), "{indexed}");
    assert!(indexed.contains("left out 2 audio files that"), "{indexed}");
    let stats = String::from_utf8_lossy(&stats.stdout);
    assert!(stats.contains("assets            4 "), "{stats}");
    let pruned = String::from_utf8_lossy(&pruned.stdout);
    assert!(
        !pruned.contains("that .gunfingerignore leaves out:"),
        "{pruned}"
    );
    assert!(!pruned.contains("gone from this library"), "{pruned}");
}

#[test]
fn an_invalid_ignore_file_stops_the_commands_that_scan_the_library() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("invalid-ignore");
    let library = dir.join("library");
    let track = Track::random(1, 10.0);
    write_wav(&library.join("1.wav"), &track.play(0.0, track.seconds, 1.0));
    let library_arg = library.to_str().unwrap();
    gunfinger(&dir, &["index", library_arg]);
    std::fs::write(library.join(".gunfingerignore"), "/Album/\n!/Album/CD1/\n").unwrap();

    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_gunfinger"))
            .args(args)
            .arg("--peaks-dir")
            .arg(dir.join("peaks"))
            .env("XDG_CONFIG_HOME", dir.join("config"))
            .env("XDG_CACHE_HOME", dir.join("cache"))
            .env("NO_COLOR", "1")
            .output()
            .unwrap()
    };
    let doctor = run(&["doctor", "--library", library_arg]);
    let stats = run(&["stats", "--library", library_arg]);
    let index = run(&["index", library_arg]);

    let expected = "line 2: `!` (negation) is not supported";
    assert!(!doctor.status.success());
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        doctor.contains("problem") && doctor.contains(expected),
        "{doctor}"
    );
    for refused in [stats, index] {
        assert!(!refused.status.success());
        // The message is wrapped to the terminal's width between borders.
        let message: String = String::from_utf8_lossy(&refused.stderr)
            .split_whitespace()
            .filter(|word| *word != "│")
            .collect::<Vec<_>>()
            .join(" ");
        assert!(message.contains(expected), "{message}");
    }
}

#[test]
fn a_copy_of_the_peak_store_names_tracks_without_the_library() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("store-only");
    let library = dir.join("library");
    std::fs::create_dir_all(&library).unwrap();
    let tracks: Vec<Track> = (1..=2).map(|seed| Track::random(seed, 40.0)).collect();
    for (track, name, tags) in [
        (
            &tracks[0],
            "a.flac",
            &["artist=Test Artist", "title=First Tune"][..],
        ),
        (&tracks[1], "b.flac", &[][..]),
    ] {
        let plain = dir.join("plain.wav");
        write_wav(&plain, &track.play(0.0, track.seconds, 1.0));
        let mut ffmpeg = Command::new("ffmpeg");
        ffmpeg.args(["-v", "error", "-y", "-i"]).arg(&plain);
        for tag in tags {
            ffmpeg.args(["-metadata", tag]);
        }
        assert!(ffmpeg.arg(library.join(name)).status().unwrap().success());
    }
    let mut mix = Mix::new(50.0);
    mix.add(0.0, &tracks[0].play(5.0, 24.0, 1.02), 0.5);
    mix.add(24.0, &tracks[1].play(5.0, 26.0, 0.98), 0.5);
    let recording = dir.join("mix.wav");
    write_wav(&recording, mix.samples());
    let library_arg = library.to_str().unwrap();
    let recording_arg = recording.to_str().unwrap();
    gunfinger(&dir, &["index", library_arg]);
    // The library goes away; a copy of its store goes to another computer
    // with no configuration.
    std::fs::rename(&library, dir.join("elsewhere")).unwrap();
    let copy = dir.join("stick");
    std::fs::create_dir_all(&copy).unwrap();
    for entry in std::fs::read_dir(dir.join("peaks")).unwrap() {
        let path = entry.unwrap().path();
        std::fs::copy(&path, copy.join(path.file_name().unwrap())).unwrap();
    }
    let identify = |args: &[&str]| {
        let output = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
            .args([
                "identify",
                recording_arg,
                "--playback",
                "turntable",
                "--peaks-dir",
            ])
            .arg(&copy)
            .args(args)
            .env("XDG_CONFIG_HOME", dir.join("no-config"))
            .env("XDG_CACHE_HOME", dir.join("cache"))
            .env("NO_COLOR", "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        output
    };

    let tracklist = identify(&["--format", "tracklist"]);
    let report = identify(&["--format", "json"]);
    let unreadable = identify(&["--format", "tracklist", "--library", library_arg]);

    let stdout = String::from_utf8_lossy(&tracklist.stdout);
    assert!(stdout.contains("Test Artist - First Tune"), "{stdout}");
    assert!(
        stdout.lines().any(|line| line.ends_with("  b")),
        "an untagged file is named by its file name: {stdout}"
    );
    let stderr = String::from_utf8_lossy(&tracklist.stderr);
    assert!(
        stderr.contains("no library given; searching the 2 records in the peak store"),
        "{stderr}"
    );
    let report: Value = serde_json::from_slice(&report.stdout).unwrap();
    assert!(
        report["library"]
            .as_str()
            .is_some_and(|root| root.ends_with("store-only/library")),
        "the report names the store's library: {}",
        report["library"]
    );
    let plays = report["plays"].as_array().unwrap();
    assert_eq!(plays_of(plays, "a.flac")[0]["tags"]["title"], "First Tune");
    assert_eq!(plays_of(plays, "b.flac")[0]["tags"], serde_json::json!({}));
    let stderr = String::from_utf8_lossy(&unreadable.stderr);
    assert!(
        stderr.contains("cannot read the library at")
            && stderr.contains("records in its peak store instead"),
        "{stderr}"
    );
    assert!(String::from_utf8_lossy(&unreadable.stdout).contains("Test Artist - First Tune"));
}

#[test]
fn a_peak_store_holds_the_records_of_one_library() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("two-libraries");
    let (first, second) = (dir.join("first"), dir.join("second"));
    for library in [&first, &second] {
        let track = Track::random(1, 10.0);
        write_wav(&library.join("a.wav"), &track.play(0.0, track.seconds, 1.0));
    }

    gunfinger(&dir, &["index", first.to_str().unwrap()]);
    let refused = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
        .args(["index", second.to_str().unwrap(), "--peaks-dir"])
        .arg(dir.join("peaks"))
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .env("XDG_CACHE_HOME", dir.join("cache"))
        .env("NO_COLOR", "1")
        .output()
        .unwrap();
    let doctor = gunfinger(&dir, &["doctor", "--library", first.to_str().unwrap()]);

    assert!(!refused.status.success());
    // The message is wrapped to the terminal's width between borders.
    let message: String = String::from_utf8_lossy(&refused.stderr)
        .split_whitespace()
        .filter(|word| *word != "│")
        .collect::<Vec<_>>()
        .join(" ");
    assert!(
        message.contains("holds the records of the library at"),
        "{message}"
    );
    let doctor = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        doctor.contains("it names this library as its own"),
        "{doctor}"
    );
}

#[test]
fn an_unindexed_library_is_named_in_the_advice_to_index_it() {
    let dir = scratch_dir("unindexed");
    std::fs::create_dir_all(dir.join("library")).unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_gunfinger"))
        .args(["stats", "--library", "library", "--peaks-dir", "peaks"])
        .current_dir(&dir)
        .env("XDG_CONFIG_HOME", dir.join("config"))
        .env("XDG_CACHE_HOME", dir.join("cache"))
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("run `gunfinger index library` first"),
        "{stderr}"
    );
}

#[test]
fn a_batch_keeps_one_report_per_recording_and_passes_over_reported_ones() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("batch");
    let track = Track::random(1, 30.0);
    write_wav(
        &dir.join("library/1.wav"),
        &track.play(0.0, track.seconds, 1.0),
    );
    let mut mix = Mix::new(25.0);
    mix.add(0.0, &track.play(3.0, 25.0, 1.02), 0.5);
    for name in ["first.wav", "second.wav"] {
        write_wav(&dir.join("mixes").join(name), mix.samples());
    }
    let library = dir.join("library");
    let library = library.to_str().unwrap();
    gunfinger(&dir, &["index", library]);
    let mixes = [dir.join("mixes/first.wav"), dir.join("mixes/second.wav")];
    let reports = dir.join("reports");
    let batch = |playback: &str, extra: &[&str]| {
        let mut args = vec!["identify"];
        args.extend(mixes.iter().map(|mix| mix.to_str().unwrap()));
        args.extend([
            "--library",
            library,
            "--save-dir",
            reports.to_str().unwrap(),
            "--playback",
            playback,
        ]);
        args.extend(extra);
        gunfinger(&dir, &args)
    };

    let first = batch("both", &[]);
    let second = batch("both", &[]);
    let single_pass = batch("both", &["--single-pass"]);
    let single_pass_report: Value =
        serde_json::from_slice(&std::fs::read(reports.join("first.json")).unwrap()).unwrap();
    let other_playback = batch("turntable", &[]);
    let other = Track::random(2, 30.0);
    write_wav(
        &dir.join("library/2.wav"),
        &other.play(0.0, other.seconds, 1.0),
    );
    gunfinger(&dir, &["index", library]);
    let grown_library = batch("turntable", &[]);

    let first = String::from_utf8_lossy(&first.stdout);
    assert_eq!(first.matches("confident").count(), 2, "{first}");
    for name in ["first.json", "second.json"] {
        let report: Value =
            serde_json::from_slice(&std::fs::read(reports.join(name)).unwrap()).unwrap();
        assert_eq!(report["plays"][0]["asset"], "1.wav", "{name}: {report}");
    }
    assert!(
        String::from_utf8_lossy(&second.stderr).contains("every recording already has a report")
    );
    let single_pass = String::from_utf8_lossy(&single_pass.stderr);
    assert!(
        single_pass.contains("its report was made with other matching settings"),
        "{single_pass}"
    );
    assert_eq!(
        single_pass_report["search"]["confidence"],
        "confident: 200 hits in 3 windows; possible: 60 hits"
    );
    let other_playback = String::from_utf8_lossy(&other_playback.stderr);
    assert!(
        other_playback.contains("its report was searched with `--playback both`"),
        "{other_playback}"
    );
    let grown_library = String::from_utf8_lossy(&grown_library.stderr);
    assert!(
        grown_library.contains("its report was made with another revision of the library"),
        "{grown_library}"
    );
    let report: Value =
        serde_json::from_slice(&std::fs::read(reports.join("first.json")).unwrap()).unwrap();
    assert_eq!(report["query"]["playback"], "turntable");
    assert_eq!(
        report["search"]["confidence"],
        "confident: 240 hits in 3 windows; possible: 60 hits"
    );
    let leftovers = std::fs::read_dir(&reports).unwrap().count();
    assert_eq!(leftovers, 2, "only the two reports, no temporary files");
}

#[test]
fn a_key_locked_play_needs_the_key_lock_rungs() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("key-lock");
    let track = Track::random(1, 60.0);
    write_wav(
        &dir.join("library/1.wav"),
        &track.play(0.0, track.seconds, 1.0),
    );
    let mut mix = Mix::new(40.0);
    mix.add(0.0, &track.play_key_locked(5.0, 40.0, 1.05), 0.5);
    let mix_path = dir.join("mix.wav");
    write_wav(&mix_path, mix.samples());
    let library = dir.join("library");
    let library = library.to_str().unwrap();
    gunfinger(&dir, &["index", library]);
    let identify = |playback: &str| {
        let output = gunfinger(
            &dir,
            &[
                "identify",
                mix_path.to_str().unwrap(),
                "--library",
                library,
                "--playback",
                playback,
                "--format",
                "json",
            ],
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        report["plays"].as_array().unwrap().clone()
    };

    let turntable = identify("turntable");
    let both = identify("both");

    assert!(
        turntable
            .iter()
            .all(|play| play["confidence"] != "confident"),
        "{turntable:?}"
    );
    assert_eq!(both.len(), 1, "{both:?}");
    assert_eq!(both[0]["confidence"], "confident");
    assert_eq!(both[0]["playback"], "key-locked");
    assert!(
        (seconds(&both[0], "speed") - 1.05).abs() < 0.001,
        "{}",
        both[0]
    );
}

#[test]
fn the_saved_index_is_used_while_current_and_rebuilt_when_not() {
    if !ffmpeg_available() {
        return;
    }
    let dir = scratch_dir("saved-index");
    let mix = build_corpus(&dir);
    let library = dir.join("library");
    let library = library.to_str().unwrap();
    gunfinger(&dir, &["index", library]);
    let identify = |saved: &str| {
        let output = gunfinger(
            &dir,
            &[
                "identify",
                mix.to_str().unwrap(),
                "--library",
                library,
                "--format",
                "json",
                "--verbose",
                "--saved-index",
                saved,
            ],
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        (
            report["plays"].clone(),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    };
    let saved_files = || -> Vec<PathBuf> {
        std::fs::read_dir(dir.join("cache/gunfinger/indexes"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect()
    };

    let (built, first) = identify("use");
    let (loaded, second) = identify("use");
    let (in_memory, off) = identify("off");

    assert!(first.contains("it is saved for the next search"), "{first}");
    assert!(second.contains("loaded the saved index"), "{second}");
    assert!(!off.contains("saved"), "{off}");
    assert_eq!(loaded, built);
    assert_eq!(in_memory, built);
    let files = saved_files();
    assert_eq!(files.len(), 1, "{files:?}");
    assert!(files[0].to_str().unwrap().ends_with(".index"));

    let whole = std::fs::read(&files[0]).unwrap();
    std::fs::write(&files[0], &whole[..whole.len() / 2]).unwrap();
    let (after_truncation, truncated) = identify("use");
    assert!(truncated.contains("cannot be read"), "{truncated}");
    assert_eq!(after_truncation, built);
    assert_eq!(
        std::fs::read(&files[0]).unwrap(),
        whole,
        "rebuilt as before"
    );

    let extra = Track::random(9, 75.0);
    write_wav(
        &dir.join("library/e.wav"),
        &extra.play(0.0, extra.seconds, 1.0),
    );
    gunfinger(&dir, &["index", library]);
    let (rebuilt, changed) = identify("use");
    let (reloaded, again) = identify("use");
    assert!(changed.contains("out of date"), "{changed}");
    assert!(again.contains("loaded the saved index"), "{again}");
    assert_eq!(rebuilt, identify("off").0);
    assert_eq!(reloaded, rebuilt);
    assert_eq!(saved_files(), files, "one saved index per peak store");
}

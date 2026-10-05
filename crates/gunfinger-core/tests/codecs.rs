//! Decoding of every codec and container the collection uses.
//!
//! Fixtures are generated with FFmpeg at test time: two seconds of a stereo
//! 44.1 kHz tone, sometimes with cover art attached. The tests are skipped
//! when FFmpeg is not installed.

// Fixture helpers run outside `#[test]` functions, where a panic is the
// clearest way to fail the test.
#![allow(clippy::unwrap_used)]

use std::path::{Path, PathBuf};
use std::process::Command;

use gunfinger_core::decode::{Audio, DecodeError, Excerpt, decode};

const RATE: u32 = 8000;
const TONE_SECONDS: f64 = 2.0;

fn ffmpeg_available() -> bool {
    let found = Command::new("ffmpeg").arg("-version").output().is_ok();
    if !found {
        eprintln!("skipping codec test: `ffmpeg` is not on PATH");
    }
    found
}

fn fixture_dir(test: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("codecs")
        .join(test);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn ffmpeg(args: &[&str]) {
    let status = Command::new("ffmpeg")
        .args(["-nostdin", "-v", "error", "-y"])
        .args(args)
        .status()
        .unwrap();
    assert!(status.success(), "ffmpeg {args:?} failed");
}

fn tone(dir: &Path, name: &str, codec_args: &[&str]) -> PathBuf {
    let path = dir.join(name);
    let source = format!("sine=frequency=440:sample_rate=44100:duration={TONE_SECONDS}");
    let mut args = vec!["-f", "lavfi", "-i", &source, "-ac", "2"];
    args.extend(codec_args);
    args.push(path.to_str().unwrap());
    ffmpeg(&args);
    path
}

fn cover(dir: &Path) -> PathBuf {
    let path = dir.join("cover.png");
    ffmpeg(&[
        "-f",
        "lavfi",
        "-i",
        "color=c=red:s=64x64",
        "-frames:v",
        "1",
        path.to_str().unwrap(),
    ]);
    path
}

/// Muxes `audio` with `cover` as an attached picture, cover first, so that
/// stream 0 of the result is the image.
fn with_cover(dir: &Path, audio: &Path, cover: &Path, name: &str, extra: &[&str]) -> PathBuf {
    let path = dir.join(name);
    let mut args = vec![
        "-i",
        cover.to_str().unwrap(),
        "-i",
        audio.to_str().unwrap(),
        "-map",
        "0:v",
        "-map",
        "1:a",
        "-c",
        "copy",
        "-disposition:v:0",
        "attached_pic",
    ];
    args.extend(extra);
    args.push(path.to_str().unwrap());
    ffmpeg(&args);
    path
}

fn assert_is_the_tone(audio: &Audio) {
    assert_eq!(audio.sample_rate, RATE);
    let seconds = audio.duration().as_secs_f64();
    // Lossy encoders add priming and padding of a few tens of milliseconds.
    assert!(
        (seconds - TONE_SECONDS).abs() < 0.1,
        "decoded {seconds} s, expected {TONE_SECONDS} s"
    );
    let rms =
        (audio.samples.iter().map(|s| s * s).sum::<f32>() / audio.samples.len() as f32).sqrt();
    // A full-scale-ish sine has an RMS near 0.09 for FFmpeg's default amplitude
    // of 1/8; silence or an image decoded as noise would not.
    assert!((0.05..0.15).contains(&rms), "unexpected RMS {rms}");
}

fn decode_whole(path: &Path) -> Audio {
    decode(path, RATE, Excerpt::default()).unwrap()
}

#[test]
fn mp3() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("mp3");
    let path = tone(&dir, "tone.mp3", &["-c:a", "libmp3lame", "-b:a", "128k"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn mp3_with_cover_art() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("mp3-cover");
    let audio = tone(&dir, "tone.mp3", &["-c:a", "libmp3lame", "-b:a", "128k"]);
    let path = with_cover(
        &dir,
        &audio,
        &cover(&dir),
        "covered.mp3",
        &["-id3v2_version", "3"],
    );
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn aac_in_m4a() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("m4a");
    let path = tone(&dir, "tone.m4a", &["-c:a", "aac", "-b:a", "128k"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn m4a_with_cover_art() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("m4a-cover");
    let audio = tone(&dir, "tone.m4a", &["-c:a", "aac", "-b:a", "128k"]);
    let path = with_cover(&dir, &audio, &cover(&dir), "covered.m4a", &[]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn opus() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("opus");
    let path = tone(&dir, "tone.opus", &["-c:a", "libopus", "-b:a", "96k"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn flac() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("flac");
    let path = tone(&dir, "tone.flac", &["-c:a", "flac"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn alac_in_m4a() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("alac");
    let path = tone(&dir, "tone.m4a", &["-c:a", "alac"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn wav() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("wav");
    let path = tone(&dir, "tone.wav", &["-c:a", "pcm_s16le"]);
    assert_is_the_tone(&decode_whole(&path));
}

#[test]
fn an_excerpt_has_the_requested_length() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("excerpt");
    let path = tone(&dir, "tone.wav", &["-c:a", "pcm_s16le"]);
    let excerpt = Excerpt {
        start: Some(std::time::Duration::from_millis(500)),
        duration: Some(std::time::Duration::from_millis(1000)),
    };

    let audio = decode(&path, RATE, excerpt).unwrap();

    assert_eq!(audio.samples.len(), RATE as usize);
}

#[test]
fn an_image_alone_is_not_audio() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("image-only");
    let path = cover(&dir);

    let error = decode(&path, RATE, Excerpt::default()).unwrap_err();

    assert!(matches!(error, DecodeError::Failed { .. }), "{error}");
}

#[test]
fn a_truncated_file_is_rejected() {
    if !ffmpeg_available() {
        return;
    }
    let dir = fixture_dir("truncated");
    let source = "sine=frequency=440:sample_rate=44100:duration=20";
    let whole = dir.join("whole.m4a");
    ffmpeg(&[
        "-f",
        "lavfi",
        "-i",
        source,
        "-c:a",
        "aac",
        "-movflags",
        "+faststart",
        whole.to_str().unwrap(),
    ]);
    let bytes = std::fs::read(&whole).unwrap();
    let cut = dir.join("cut.m4a");
    std::fs::write(&cut, &bytes[..bytes.len() / 2]).unwrap();

    let result = decode(&cut, RATE, Excerpt::default());

    assert!(
        matches!(
            result,
            Err(DecodeError::Truncated { .. } | DecodeError::Failed { .. })
        ),
        "{result:?}"
    );
}

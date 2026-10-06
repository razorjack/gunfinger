//! `gunfinger listen`: hear what was found. Plays the recording at a moment,
//! then each track found there from the same place in the track and at the
//! speed it was played, so the two can be compared by ear.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use miette::miette;

use crate::console::Console;
use crate::report::{FoundPlay, Playback, Report, Segment};
use crate::table::{named, span, timecode};

/// Rate the speed change is applied at; any common rate works.
const PLAYBACK_RATE: f64 = 48_000.0;

/// A stretch of audio to play.
#[derive(Debug, PartialEq)]
pub struct Clip {
    pub label: String,
    pub path: PathBuf,
    pub start_seconds: f64,
    /// 1.0 for the recording itself; a track's play speed in the mix.
    pub speed: f64,
    pub playback: Playback,
}

/// The recording at `at`, then every track playing at `at`, strongest
/// first.
pub fn clips(report: &Report, at: f64) -> miette::Result<Vec<Clip>> {
    let mut playing: Vec<&FoundPlay> = report
        .plays
        .iter()
        .filter(|play| play.start_seconds <= at && at <= play.end_seconds)
        .collect();
    if playing.is_empty() {
        let nearby: Vec<String> = report
            .plays
            .iter()
            .filter(|play| {
                (play.start_seconds - at).abs() < 300.0 || (play.end_seconds - at).abs() < 300.0
            })
            .map(|play| {
                format!(
                    "{} {}",
                    span(play.start_seconds, play.end_seconds),
                    play.asset
                )
            })
            .collect();
        return Err(miette!(
            help = if nearby.is_empty() {
                String::from("nothing was found within 5 minutes either")
            } else {
                format!("found nearby:\n{}", nearby.join("\n"))
            },
            "nothing was found playing at {}",
            timecode(at)
        ));
    }
    playing.sort_by_key(|play| std::cmp::Reverse(play.hits));

    let mut clips = vec![Clip {
        label: format!("the recording at {}", timecode(at)),
        path: report.query.path.clone(),
        start_seconds: at,
        speed: 1.0,
        playback: Playback::Turntable,
    }];
    for play in playing {
        let segment = nearest_segment(play, at);
        let in_track = segment.track_start_seconds + (at - segment.start_seconds) * segment.speed;
        clips.push(Clip {
            label: format!(
                "{} at {}, {:+.2}%",
                named(&play.asset, segment.playback),
                timecode(in_track),
                (segment.speed - 1.0) * 100.0
            ),
            path: report.library.join(&play.asset),
            start_seconds: in_track.max(0.0),
            speed: segment.speed,
            playback: segment.playback,
        });
    }
    Ok(clips)
}

/// The segment holding `at`, or across a gap the one that ends or starts
/// closest to it.
fn nearest_segment(play: &FoundPlay, at: f64) -> &Segment {
    let distance = |segment: &Segment| {
        if at < segment.start_seconds {
            segment.start_seconds - at
        } else {
            (at - segment.end_seconds).max(0.0)
        }
    };
    play.segments
        .iter()
        .min_by(|a, b| distance(a).total_cmp(&distance(b)))
        .unwrap_or(&play.segments[0])
}

/// Plays the clips one after another with `ffplay`.
pub fn play(clips: &[Clip], seconds: f64, console: &Console) -> miette::Result<()> {
    if let Some(missing) = clips.iter().find(|clip| !clip.path.is_file()) {
        return Err(miette!(
            help = "if the library has moved, give its root with --library",
            "cannot find {}",
            missing.path.display()
        ));
    }
    for clip in clips {
        console.info(format_args!("playing {}", clip.label));
        let status = Command::new("ffplay")
            .args(ffplay_args(clip, seconds))
            .stdin(Stdio::null())
            .status()
            .map_err(|error| {
                miette!(
                    help = "install FFmpeg with ffplay, for example `brew install ffmpeg`",
                    "cannot run ffplay: {error}"
                )
            })?;
        if !status.success() {
            return Err(miette!("ffplay could not play {}", clip.path.display()));
        }
    }
    Ok(())
}

/// The commands `play` would run, for copying into a shell.
pub fn commands(clips: &[Clip], seconds: f64) -> String {
    let mut lines: Vec<String> = clips
        .iter()
        .map(|clip| {
            let args: Vec<String> = ffplay_args(clip, seconds)
                .into_iter()
                .map(|arg| shell_quote(&arg))
                .collect();
            format!("ffplay {}", args.join(" "))
        })
        .collect();
    lines.push(String::new());
    lines.join("\n")
}

/// Reads `seconds` of the track and changes its speed the way it was
/// played: resampled (pitch and tempo together) or time-stretched (tempo
/// only).
fn ffplay_args(clip: &Clip, seconds: f64) -> Vec<String> {
    let mut args: Vec<String> = ["-hide_banner", "-loglevel", "error", "-nodisp", "-autoexit"]
        .map(String::from)
        .to_vec();
    args.extend([
        String::from("-ss"),
        format!("{:.2}", clip.start_seconds),
        String::from("-t"),
        format!("{:.2}", seconds * clip.speed),
    ]);
    if clip.speed != 1.0 {
        let filter = match clip.playback {
            Playback::Turntable => format!(
                "aresample={PLAYBACK_RATE:.0},asetrate={:.0},aresample={PLAYBACK_RATE:.0}",
                PLAYBACK_RATE * clip.speed
            ),
            Playback::KeyLocked => format!("atempo={:.4}", clip.speed),
        };
        args.extend([String::from("-af"), filter]);
    }
    args.push(clip.path.to_string_lossy().into_owned());
    args
}

fn shell_quote(arg: &str) -> String {
    if arg
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || "-_./:=,+".contains(c))
    {
        arg.to_owned()
    } else {
        format!("'{}'", arg.replace('\'', "'\\''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::tests::report;

    #[test]
    fn the_recording_comes_first_then_each_track_at_its_place_and_speed() {
        let clips = clips(&report(), 40.0).unwrap_or_default();

        let summary: Vec<(String, f64, f64)> = clips
            .iter()
            .map(|clip| {
                (
                    clip.path.to_string_lossy().into_owned(),
                    (clip.start_seconds * 100.0).round() / 100.0,
                    clip.speed,
                )
            })
            .collect();
        assert_eq!(
            summary,
            [
                (String::from("/mixes/mix.wav"), 40.0, 1.0),
                // a: 853 hits, from 10 s at +3%: 10 + 40 * 1.03.
                (String::from("/library/a.wav"), 51.2, 1.03),
                // b: 818 hits, from 5 s at 38 s, at -5%: 5 + 2 * 0.95.
                (String::from("/library/b.wav"), 6.9, 0.95),
            ]
        );
    }

    #[test]
    fn across_a_needle_skip_the_nearest_segment_gives_the_place() {
        let clips = clips(&report(), 111.5).unwrap_or_default();

        // c's first segment ends at 111 s: 20 + 15 * 1.06 at the end, plus
        // half a second beyond it.
        assert_eq!(clips.len(), 2);
        assert!((clips[1].start_seconds - (20.0 + 15.5 * 1.06)).abs() < 1e-9);
    }

    #[test]
    fn nothing_playing_is_an_error_naming_what_is_nearby() {
        let error = clips(&report(), 145.0)
            .err()
            .map(|error| format!("{error:?}"));

        assert!(error.is_some_and(|error| error.contains("c.wav")));
    }

    #[test]
    fn printed_commands_quote_paths_and_change_the_speed() {
        let clip = Clip {
            label: String::new(),
            path: PathBuf::from("/library/Skynet & Stakka - Decoy.mp3"),
            start_seconds: 42.5,
            speed: 1.03,
            playback: Playback::Turntable,
        };

        assert_eq!(
            commands(&[clip], 8.0),
            "ffplay -hide_banner -loglevel error -nodisp -autoexit -ss 42.50 -t 8.24 -af aresample=48000,asetrate=49440,aresample=48000 '/library/Skynet & Stakka - Decoy.mp3'\n"
        );
    }
}

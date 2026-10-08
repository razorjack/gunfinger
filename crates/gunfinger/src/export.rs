//! Formats for other programs and for people writing up a mix: CSV, a cue
//! sheet and a numbered tracklist.
//!
//! The cue sheet and the tracklist list recordings, not plays. Two rips or
//! uploads of one recording are both found where it plays, at the same place
//! in the track once each play's speed is taken out (an upload sped up by 3%
//! is found 3% slower); such plays are one entry, named after the strongest,
//! with the other names the tags give.

use crate::names::TrackName;
use crate::report::{FoundPlay, Level, Playback, Report, SharedWith};
use crate::table::timecode;

/// Plays of different assets are one recording when they overlap for at
/// least this share of the shorter play...
const SAME_RECORDING_OVERLAP: f64 = 0.5;
/// ...and the track would have started at the same mix time, to within this
/// many seconds. Uploads of one track agree to 1.6 s in the development mix
/// at NAS scale; the remixes sharing material with a played track lie 86
/// and 193 s away (experiment 0044's report).
const SAME_RECORDING_SECONDS: f64 = 5.0;
/// Cue sheet times count frames of 1/75 s.
const CUE_FRAMES_PER_SECOND: f64 = 75.0;

/// One row per play, times in seconds.
pub fn csv(report: &Report) -> String {
    let mut lines = vec![String::from(
        "start_seconds,end_seconds,track_start_seconds,track_end_seconds,speed_percent,confidence,hits,windows,segments,playback,asset,same_audio,shares_material_with_play",
    )];
    for play in &report.plays {
        lines.push(format!(
            "{:.1},{:.1},{:.1},{:.1},{:.2},{},{},{},{},{},{},{},{}",
            play.start_seconds,
            play.end_seconds,
            play.track_start_seconds,
            play.track_end_seconds,
            (play.speed - 1.0) * 100.0,
            play.confidence.label(),
            play.hits,
            play.windows,
            play.segments.len(),
            match play.playback {
                Playback::Turntable => "turntable",
                Playback::KeyLocked => "key-locked",
            },
            csv_field(&play.asset),
            csv_field(&play.same_audio.join(";")),
            play.shares_material_with
                .as_ref()
                .map_or_else(String::new, |shared| shared.play.to_string())
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

/// RFC 4180: a field with a comma, quote or line break is quoted, and its
/// quotes doubled.
fn csv_field(text: &str) -> String {
    if text.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", text.replace('"', "\"\""))
    } else {
        text.to_owned()
    }
}

/// A recording heard in the mix: one or more plays of assets holding it.
pub struct Entry<'a> {
    /// Strongest first.
    pub plays: Vec<&'a FoundPlay>,
}

impl Entry<'_> {
    pub fn start_seconds(&self) -> f64 {
        self.plays
            .iter()
            .map(|play| play.start_seconds)
            .fold(f64::INFINITY, f64::min)
    }

    pub fn confidence(&self) -> Level {
        self.plays[0].confidence
    }

    pub fn asset(&self) -> &str {
        &self.plays[0].asset
    }

    /// The names of the other plays that differ from the first play's and
    /// from each other, ignoring case.
    pub fn other_names(&self, name: impl Fn(&str) -> TrackName) -> Vec<String> {
        let mut seen = vec![name(self.asset()).full().to_lowercase()];
        let mut others = Vec::new();
        for play in &self.plays[1..] {
            let full = name(&play.asset).full();
            if !seen.contains(&full.to_lowercase()) {
                seen.push(full.to_lowercase());
                others.push(full);
            }
        }
        others
    }
}

/// Groups plays of the same recording, in order of start time.
pub fn entries(report: &Report) -> Vec<Entry<'_>> {
    let mut entries: Vec<Entry> = Vec::new();
    for play in &report.plays {
        match entries
            .iter_mut()
            .find(|entry| entry.plays.iter().any(|other| same_recording(play, other)))
        {
            Some(entry) => entry.plays.push(play),
            None => entries.push(Entry { plays: vec![play] }),
        }
    }
    for entry in &mut entries {
        entry.plays.sort_by_key(|play| {
            (
                play.confidence != Level::Confident,
                std::cmp::Reverse(play.hits),
            )
        });
    }
    entries.sort_by(|a, b| a.start_seconds().total_cmp(&b.start_seconds()));
    entries
}

/// Whether two plays are one recording on two records. A possible play
/// marked as sharing material with another recording stays an entry of its
/// own, whatever lines up.
fn same_recording(a: &FoundPlay, b: &FoundPlay) -> bool {
    if a.shares_material_with.is_some() || b.shares_material_with.is_some() {
        return false;
    }
    let overlap = a.end_seconds.min(b.end_seconds) - a.start_seconds.max(b.start_seconds);
    let shorter = (a.end_seconds - a.start_seconds).min(b.end_seconds - b.start_seconds);
    overlap >= SAME_RECORDING_OVERLAP * shorter
        && (track_started_at(a) - track_started_at(b)).abs() <= SAME_RECORDING_SECONDS
}

/// The mix time at which the play's track would have started, had it been
/// played from its beginning at the play's speed: the place in the track
/// with the record's own speed taken out. Rips and uploads of one recording
/// at different native speeds agree on it.
fn track_started_at(play: &FoundPlay) -> f64 {
    play.start_seconds - play.track_start_seconds / play.speed
}

/// A numbered list of the recordings, with their start in the mix;
/// possible entries are marked, with the name of the entry they share
/// material with.
pub fn tracklist(report: &Report, name: impl Fn(&str) -> TrackName) -> String {
    let entries = entries(report);
    let entry_name = |shared: &SharedWith| {
        let play = shared
            .play
            .checked_sub(1)
            .and_then(|index| report.plays.get(index));
        entries
            .iter()
            .find(|entry| {
                play.is_some_and(|play| entry.plays.iter().any(|other| std::ptr::eq(*other, play)))
            })
            .map_or_else(|| name(&shared.asset), |entry| name(entry.asset()))
            .full()
    };
    let mut lines = Vec::new();
    for (number, entry) in entries.iter().enumerate() {
        let mark = match (entry.confidence(), &entry.plays[0].shares_material_with) {
            (Level::Confident, _) => String::new(),
            (_, Some(shared)) => {
                format!(" (possible; shares material with {})", entry_name(shared))
            }
            (Level::Possible | Level::Weak, None) => String::from(" (possible)"),
        };
        let others = entry.other_names(&name);
        let also = if others.is_empty() {
            String::new()
        } else {
            format!("  (also: {})", others.join(", "))
        };
        lines.push(format!(
            "{:>2}. {:>7}  {}{also}{mark}",
            number + 1,
            timecode(entry.start_seconds()),
            name(entry.asset()).full()
        ));
    }
    lines.push(String::new());
    lines.join("\n")
}

/// A cue sheet of the confident entries for the searched recording. Cue
/// times must increase, so an entry starting with or before the previous
/// one is placed a frame after it.
pub fn cue(report: &Report, name: impl Fn(&str) -> TrackName) -> String {
    let file = report
        .query
        .path
        .file_name()
        .map_or_else(String::new, |name| name.to_string_lossy().into_owned());
    let file_type = match report
        .query
        .path
        .extension()
        .map(|extension| extension.to_string_lossy().to_lowercase())
        .as_deref()
    {
        Some("mp3") => "MP3",
        Some("aif" | "aiff") => "AIFF",
        _ => "WAVE",
    };
    let mut lines = vec![
        String::from("REM COMMENT \"gunfinger identify\""),
        format!("FILE \"{}\" {file_type}", cue_text(&file)),
    ];
    let mut previous_frame: Option<u64> = None;
    let confident = entries(report)
        .into_iter()
        .filter(|entry| entry.confidence() == Level::Confident);
    for (number, entry) in confident.enumerate() {
        let name = name(entry.asset());
        let mut frame = (entry.start_seconds().max(0.0) * CUE_FRAMES_PER_SECOND).round() as u64;
        if let Some(previous) = previous_frame {
            frame = frame.max(previous + 1);
        }
        previous_frame = Some(frame);
        lines.push(format!("  TRACK {:02} AUDIO", number + 1));
        lines.push(format!("    TITLE \"{}\"", cue_text(&name.title)));
        if let Some(artist) = &name.artist {
            lines.push(format!("    PERFORMER \"{}\"", cue_text(artist)));
        }
        lines.push(format!("    INDEX 01 {}", cue_time(frame)));
    }
    lines.push(String::new());
    lines.join("\n")
}

/// Cue sheets have no escape for quotes.
fn cue_text(text: &str) -> String {
    text.replace('"', "'")
}

/// `MM:SS:FF`; minutes go past 99 in long recordings.
fn cue_time(frame: u64) -> String {
    let frames_per_second = CUE_FRAMES_PER_SECOND as u64;
    let seconds = frame / frames_per_second;
    format!(
        "{:02}:{:02}:{:02}",
        seconds / 60,
        seconds % 60,
        frame % frames_per_second
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::table::tests::report;

    fn names(asset: &str) -> TrackName {
        match asset {
            "a.wav" => TrackName {
                artist: Some(String::from("Artist A")),
                title: String::from("Track \"A\""),
            },
            _ => TrackName::from_file_name(asset),
        }
    }

    #[test]
    fn csv_has_one_row_per_play_and_quotes_awkward_fields() {
        let mut report = report();
        report.plays[1].asset = String::from("b, the remix.wav");

        let csv = csv(&report);

        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 5);
        assert_eq!(
            lines[1],
            "0.0,41.0,10.0,52.2,3.00,confident,853,4,1,turntable,a.wav,copy-of-a.wav,"
        );
        assert!(lines[2].ends_with(",key-locked,\"b, the remix.wav\",,"));
    }

    #[test]
    fn the_tracklist_numbers_recordings_and_marks_possible_ones() {
        let tracklist = tracklist(&report(), names);

        assert_eq!(
            tracklist,
            " 1.    0:00  Artist A - Track \"A\"\n 2.    0:38  b\n 3.    1:24  insert (possible)\n 4.    1:36  c\n"
        );
    }

    /// The play of `report` at `index` found again on another record that
    /// plays natively `native` times as fast as the first, from the same
    /// place in the track.
    fn on_another_record(report: &Report, index: usize, asset: &str, native: f64) -> FoundPlay {
        let mut play = report.plays[index].clone();
        play.asset = String::from(asset);
        play.speed /= native;
        play.track_start_seconds /= native;
        play.track_end_seconds /= native;
        play
    }

    #[test]
    fn uploads_at_another_native_speed_are_one_entry_with_their_names() {
        let mut report = report();
        let fast = on_another_record(&report, 1, "b-sped-up-upload.wav", 1.035);
        let mut same_name = on_another_record(&report, 1, "B.wav", 0.98);
        same_name.hits -= 100;
        report.plays.insert(2, fast);
        report.plays.insert(3, same_name);

        let tracklist = tracklist(&report, names);

        assert_eq!(
            tracklist,
            " 1.    0:00  Artist A - Track \"A\"\n 2.    0:38  b  (also: b-sped-up-upload)\n 3.    1:24  insert (possible)\n 4.    1:36  c\n"
        );
    }

    #[test]
    fn a_possible_play_names_the_entry_it_shares_material_with() {
        let mut report = report();
        let mut fast = on_another_record(&report, 1, "b-sped-up-upload.wav", 1.035);
        fast.hits -= 100;
        let mut vip = on_another_record(&report, 1, "b-vip.wav", 1.0);
        vip.confidence = Level::Possible;
        vip.shares_material_with = Some(SharedWith {
            play: 3,
            asset: String::from("b-sped-up-upload.wav"),
        });
        report.plays.insert(2, fast);
        report.plays.insert(3, vip);

        let tracklist = tracklist(&report, names);

        assert_eq!(
            tracklist,
            " 1.    0:00  Artist A - Track \"A\"\n 2.    0:38  b  (also: b-sped-up-upload)\n 3.    0:38  b-vip (possible; shares material with b)\n 4.    1:24  insert (possible)\n 5.    1:36  c\n"
        );
    }

    #[test]
    fn another_place_in_the_track_or_shared_material_is_another_entry() {
        let mut report = report();
        let mut remix = on_another_record(&report, 1, "b-remix.wav", 1.0);
        remix.track_start_seconds += 30.0;
        let mut shared = on_another_record(&report, 1, "b-vip.wav", 1.0);
        shared.confidence = Level::Possible;
        shared.shares_material_with = Some(SharedWith {
            play: 2,
            asset: String::from("b.wav"),
        });
        report.plays.insert(2, remix);
        report.plays.insert(3, shared);

        let assets: Vec<Vec<String>> = entries(&report)
            .iter()
            .map(|entry| entry.plays.iter().map(|play| play.asset.clone()).collect())
            .collect();

        assert_eq!(
            assets,
            [
                vec!["a.wav"],
                vec!["b.wav"],
                vec!["b-remix.wav"],
                vec!["b-vip.wav"],
                vec!["insert.wav"],
                vec!["c.wav"]
            ]
        );
    }

    #[test]
    fn rips_of_one_recording_are_one_entry() {
        let mut report = report();
        let mut other_rip = report.plays[1].clone();
        other_rip.asset = String::from("b-other-rip.wav");
        other_rip.hits += 100;
        other_rip.speed += 0.002;
        other_rip.start_seconds += 2.0;
        other_rip.track_start_seconds += 1.9;
        report.plays.insert(2, other_rip);

        let entries = entries(&report);

        let assets: Vec<Vec<&str>> = entries
            .iter()
            .map(|entry| entry.plays.iter().map(|play| play.asset.as_str()).collect())
            .collect();
        assert_eq!(
            assets,
            [
                vec!["a.wav"],
                vec!["b-other-rip.wav", "b.wav"],
                vec!["insert.wav"],
                vec!["c.wav"]
            ]
        );
    }

    #[test]
    fn the_cue_sheet_lists_confident_entries_in_frames() {
        let cue = cue(&report(), names);

        assert_eq!(
            cue,
            "\
REM COMMENT \"gunfinger identify\"
FILE \"mix.wav\" WAVE
  TRACK 01 AUDIO
    TITLE \"Track 'A'\"
    PERFORMER \"Artist A\"
    INDEX 01 00:00:00
  TRACK 02 AUDIO
    TITLE \"b\"
    INDEX 01 00:38:00
  TRACK 03 AUDIO
    TITLE \"c\"
    INDEX 01 01:36:00
"
        );
    }

    #[test]
    fn cue_times_count_minutes_past_an_hour() {
        assert_eq!(cue_time(75 * 3725 + 12), "62:05:12");
    }
}

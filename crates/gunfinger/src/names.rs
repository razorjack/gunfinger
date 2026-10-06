//! Track names for people, from the library files' tags.

use std::path::Path;
use std::process::{Command, Stdio};

use serde_json::Value;

/// `artist - title` from a file's tags when it has a title, otherwise its
/// file name.
pub struct TrackName {
    pub artist: Option<String>,
    pub title: String,
}

impl TrackName {
    pub fn from_file_name(asset: &str) -> TrackName {
        TrackName {
            artist: None,
            title: Path::new(asset)
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or(asset)
                .to_owned(),
        }
    }

    /// Reads the tags of `asset` under `library` with `ffprobe`; falls back
    /// to the file name when the file, `ffprobe` or a title is missing.
    pub fn from_tags(library: &Path, asset: &str) -> TrackName {
        let tags = probe_tags(&library.join(asset));
        let tag = |key: &str| tags.as_ref().and_then(|tags| find_tag(tags, key));
        match tag("title") {
            Some(title) => TrackName::tagged(tag("artist"), &title),
            None => TrackName::from_file_name(asset),
        }
    }

    /// Some files repeat the artist in the title (`Kraken - Side Effects`
    /// by `Kraken`); the repetition is dropped.
    fn tagged(artist: Option<String>, title: &str) -> TrackName {
        let title = artist
            .as_deref()
            .and_then(|artist| title.strip_prefix(artist))
            .and_then(|rest| rest.strip_prefix(" - "))
            .unwrap_or(title);
        TrackName {
            artist,
            title: title.to_owned(),
        }
    }

    pub fn full(&self) -> String {
        match &self.artist {
            Some(artist) => format!("{artist} - {}", self.title),
            None => self.title.clone(),
        }
    }
}

/// Container tags (MP3, MP4) and stream tags (Ogg, Opus) together.
fn probe_tags(path: &Path) -> Option<Value> {
    let output = Command::new("ffprobe")
        .args(["-v", "error", "-show_entries", "format_tags:stream_tags"])
        .args(["-of", "json"])
        .arg(path)
        .stdin(Stdio::null())
        .output()
        .ok()?;
    serde_json::from_slice(&output.stdout).ok()
}

/// Tag keys differ in case between formats (`title`, `TITLE`).
fn find_tag(probe: &Value, key: &str) -> Option<String> {
    let format_tags = probe.get("format").and_then(|format| format.get("tags"));
    let stream_tags = probe
        .get("streams")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|stream| stream.get("tags"));
    format_tags
        .into_iter()
        .chain(stream_tags)
        .filter_map(Value::as_object)
        .flat_map(|tags| tags.iter())
        .find(|(name, _)| name.eq_ignore_ascii_case(key))
        .and_then(|(_, value)| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn tags_are_found_in_the_container_or_a_stream_in_any_case() {
        let mp3 = json!({"format": {"tags": {"title": "Decoy", "artist": "Skynet & Stakka"}}});
        let opus =
            json!({"streams": [{"tags": {"TITLE": "Dominion", "ARTIST": "Kraken"}}], "format": {}});

        assert_eq!(find_tag(&mp3, "artist").as_deref(), Some("Skynet & Stakka"));
        assert_eq!(find_tag(&opus, "title").as_deref(), Some("Dominion"));
        assert_eq!(find_tag(&opus, "album"), None);
    }

    #[test]
    fn an_artist_repeated_in_the_title_is_dropped() {
        let repeated = TrackName::tagged(Some(String::from("Kraken")), "Kraken - Side Effects");
        let kept = TrackName::tagged(Some(String::from("Kraken")), "Krakenhead");

        assert_eq!(repeated.full(), "Kraken - Side Effects");
        assert_eq!(kept.full(), "Kraken - Krakenhead");
    }

    #[test]
    fn a_file_without_tags_is_named_by_its_file_name() {
        let name = TrackName::from_tags(Path::new("/nonexistent"), "extra/Kraken - Dominion.opus");

        assert_eq!(name.full(), "Kraken - Dominion");
    }
}

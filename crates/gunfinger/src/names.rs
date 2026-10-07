//! Track names for people, from the tags the peak store holds or, for older
//! reports, from the library files' tags.

use std::path::Path;

use gunfinger_core::decode::probe;

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

    /// From tags already read; the file name of `asset` without a title.
    pub fn named(artist: Option<&str>, title: Option<&str>, asset: &str) -> TrackName {
        match title {
            Some(title) => TrackName::tagged(artist.map(str::to_owned), title),
            None => TrackName::from_file_name(asset),
        }
    }

    /// Reads the tags of `asset` under `library` with `ffprobe`; falls back
    /// to the file name when the file, `ffprobe` or a title is missing.
    pub fn from_file(library: &Path, asset: &str) -> TrackName {
        let tags = probe(&library.join(asset))
            .map(|probe| probe.tags)
            .unwrap_or_default();
        TrackName::named(tags.artist.as_deref(), tags.title.as_deref(), asset)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_artist_repeated_in_the_title_is_dropped() {
        let repeated = TrackName::tagged(Some(String::from("Kraken")), "Kraken - Side Effects");
        let kept = TrackName::tagged(Some(String::from("Kraken")), "Krakenhead");

        assert_eq!(repeated.full(), "Kraken - Side Effects");
        assert_eq!(kept.full(), "Kraken - Krakenhead");
    }

    #[test]
    fn a_file_without_tags_is_named_by_its_file_name() {
        let read = TrackName::from_file(Path::new("/nonexistent"), "extra/Kraken - Dominion.opus");
        let stored = TrackName::named(Some("Kraken"), None, "extra/Kraken - Dominion.opus");

        assert_eq!(read.full(), "Kraken - Dominion");
        assert_eq!(stored.full(), "Kraken - Dominion");
    }
}

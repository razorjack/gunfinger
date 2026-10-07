//! What a file's tags say about its track.

/// The tags a tracklist shows. Any of them may be missing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Tags {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
}

impl Tags {
    /// Keeps the first non-empty `artist`, `title` or `album`, whatever the
    /// key's case (`TITLE` in Vorbis comments).
    pub(crate) fn offer(&mut self, key: &str, value: &str) {
        let slot = match key.to_ascii_lowercase().as_str() {
            "artist" => &mut self.artist,
            "title" => &mut self.title,
            "album" => &mut self.album,
            _ => return,
        };
        let value = value.trim();
        if slot.is_none() && !value.is_empty() {
            *slot = Some(value.to_owned());
        }
    }

    /// These tags, with what they lack taken from `other`.
    pub(crate) fn or(self, other: Tags) -> Tags {
        Tags {
            artist: self.artist.or(other.artist),
            title: self.title.or(other.title),
            album: self.album.or(other.album),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_value_of_each_tag_is_kept_in_any_case() {
        let mut tags = Tags::default();

        tags.offer("TITLE", "Dominion");
        tags.offer("title", "Dominion (A Side)");
        tags.offer("artist", "  ");
        tags.offer("Artist", "Kraken");
        tags.offer("genre", "Drum & Bass");

        assert_eq!(
            tags,
            Tags {
                artist: Some(String::from("Kraken")),
                title: Some(String::from("Dominion")),
                album: None,
            }
        );
    }
}

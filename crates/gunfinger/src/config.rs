//! Settings: command-line flags, then environment variables, then the
//! configuration file, then built-in defaults.
//!
//! The file is `$XDG_CONFIG_HOME/gunfinger/config.toml`, or
//! `~/.config/gunfinger/config.toml`, unless `--config` (`GUNFINGER_CONFIG`)
//! names another; one file per library keeps separate libraries apart.
//!
//! ```toml
//! library = "~/Music/library"
//! peaks_dir = "~/.local/share/gunfinger/peaks"
//! jobs = 8
//! color = "auto"
//! playback = "turntable"   # vinyl only; the default "both" also finds key lock
//! min_track = "1:30"       # shorter files are samples and loops
//! max_track = "15:00"      # longer files are mixes; the default is 17:00
//! ```

use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::time::Duration;

use gunfinger_core::indexing::TrackLength;
use gunfinger_core::timecode::{format_timecode, parse_timecode};
use miette::{IntoDiagnostic, WrapErr, miette};
use serde::{Deserialize, Deserializer};

use crate::playback::PlaybackChoice;
use crate::style::ColorChoice;

const DEFAULT_PEAKS_DIR: &str = "work/peaks";
/// Below the 17:28 a posting's frame addresses (ADR 0010).
const DEFAULT_MAX_TRACK: Duration = Duration::from_secs(17 * 60);

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    library: Option<PathBuf>,
    peaks_dir: Option<PathBuf>,
    jobs: Option<NonZeroUsize>,
    color: Option<ColorChoice>,
    playback: Option<PlaybackChoice>,
    #[serde(default, deserialize_with = "timecode")]
    min_track: Option<Duration>,
    #[serde(default, deserialize_with = "timecode")]
    max_track: Option<Duration>,
}

/// What the flags and environment variables gave.
pub struct Given {
    pub config: Option<PathBuf>,
    pub peaks_dir: Option<PathBuf>,
    pub jobs: Option<NonZeroUsize>,
    pub color: Option<ColorChoice>,
    pub min_track: Option<Duration>,
    pub max_track: Option<Duration>,
}

pub struct Settings {
    pub peaks_dir: PathBuf,
    pub jobs: usize,
    pub color: ColorChoice,
    /// How searches assume the records were played, unless `--playback`
    /// says otherwise.
    pub playback: PlaybackChoice,
    /// Library files outside it are not indexed and not searched.
    pub track_length: TrackLength,
    library: Option<PathBuf>,
    /// The configuration file read, if any.
    pub file: Option<PathBuf>,
}

impl Settings {
    pub fn resolve(given: Given) -> miette::Result<Settings> {
        let path = given
            .config
            .or_else(|| default_path().filter(|path| path.is_file()));
        let file = match &path {
            Some(path) => read(path)?,
            None => File::default(),
        };
        let track_length = TrackLength {
            min: given.min_track.or(file.min_track).unwrap_or_default(),
            max: given
                .max_track
                .or(file.max_track)
                .unwrap_or(DEFAULT_MAX_TRACK),
        };
        if track_length.min >= track_length.max {
            return Err(miette!(
                help = "set the shortest track (--min-track, `min_track`) below the longest (--max-track, `max_track`)",
                "no file can be a track: the shortest track, {}, is not shorter than the longest, {}",
                format_timecode(track_length.min),
                format_timecode(track_length.max)
            ));
        }
        Ok(Settings {
            peaks_dir: given
                .peaks_dir
                .or(file.peaks_dir.map(|dir| expand_home(&dir)))
                .unwrap_or_else(|| PathBuf::from(DEFAULT_PEAKS_DIR)),
            jobs: given.jobs.or(file.jobs).map_or_else(
                || std::thread::available_parallelism().map_or(1, NonZeroUsize::get),
                NonZeroUsize::get,
            ),
            color: given.color.or(file.color).unwrap_or(ColorChoice::Auto),
            playback: file.playback.unwrap_or(PlaybackChoice::Both),
            track_length,
            library: file.library.map(|library| expand_home(&library)),
            file: path,
        })
    }

    /// The library given on the command line, or the configured one.
    pub fn library(&self, given: Option<PathBuf>) -> miette::Result<PathBuf> {
        self.library_if_any(given)
            .ok_or_else(|| miette!(help = library_help(), "no library given"))
    }

    /// The library given on the command line, the configured one, or none.
    pub fn library_if_any(&self, given: Option<PathBuf>) -> Option<PathBuf> {
        given.or_else(|| self.library.clone())
    }
}

/// How to name a library.
pub fn library_help() -> String {
    format!(
        "pass --library, or set `library = \"...\"` in {}",
        default_path().map_or_else(
            || String::from("the configuration file"),
            |path| path.display().to_string()
        )
    )
}

fn read(path: &Path) -> miette::Result<File> {
    let text = std::fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot read the configuration file {}", path.display()))?;
    toml::from_str(&text)
        .into_diagnostic()
        .wrap_err_with(|| format!("{} is not a valid configuration file", path.display()))
}

/// A length such as `"1:30"` or `"15:00"`, as on the command line.
fn timecode<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Duration>, D::Error> {
    let text = String::deserialize(deserializer)?;
    parse_timecode(&text)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

fn default_path() -> Option<PathBuf> {
    let config_home = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|dir| !dir.is_empty())
        .map(PathBuf::from)
        .or_else(|| home().map(|home| home.join(".config")))?;
    Some(config_home.join("gunfinger").join("config.toml"))
}

fn home() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .filter(|home| !home.is_empty())
        .map(PathBuf::from)
}

/// `~/music` in a configuration file means the home directory's `music`.
fn expand_home(path: &Path) -> PathBuf {
    match (path.strip_prefix("~"), home()) {
        (Ok(rest), Some(home)) => home.join(rest),
        _ => path.to_path_buf(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn given() -> Given {
        Given {
            config: None,
            peaks_dir: None,
            jobs: None,
            color: None,
            min_track: None,
            max_track: None,
        }
    }

    fn write_config(name: &str, text: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("gunfinger-config-{name}"));
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("config.toml");
        let _ = std::fs::write(&path, text);
        path
    }

    #[test]
    fn the_file_fills_in_what_flags_leave_out() {
        let path = write_config(
            "fills",
            "library = \"/music\"\npeaks_dir = \"/peaks\"\njobs = 3\ncolor = \"never\"\nplayback = \"both\"\n",
        );
        let flags = Given {
            config: Some(path),
            jobs: NonZeroUsize::new(5),
            ..given()
        };

        let settings = Settings::resolve(flags).ok();

        let settings = settings.as_ref();
        assert_eq!(
            settings.map(|s| s.peaks_dir.clone()),
            Some(PathBuf::from("/peaks"))
        );
        assert_eq!(settings.map(|s| s.jobs), Some(5));
        assert_eq!(settings.map(|s| s.color), Some(ColorChoice::Never));
        assert_eq!(settings.map(|s| s.playback), Some(PlaybackChoice::Both));
        assert_eq!(
            settings.and_then(|s| s.library(None).ok()),
            Some(PathBuf::from("/music"))
        );
        assert_eq!(
            settings.and_then(|s| s.library(Some(PathBuf::from("/other"))).ok()),
            Some(PathBuf::from("/other"))
        );
    }

    #[test]
    fn without_settings_both_playbacks_are_searched() {
        let path = write_config("empty", "");

        let settings = Settings::resolve(Given {
            config: Some(path),
            ..given()
        });

        assert_eq!(
            settings.ok().map(|s| s.playback),
            Some(PlaybackChoice::Both)
        );
    }

    #[test]
    fn track_lengths_come_from_flags_then_the_file() {
        let path = write_config("lengths", "min_track = \"1:30\"\nmax_track = \"15:00\"\n");
        let flags = Given {
            config: Some(path.clone()),
            max_track: Some(Duration::from_secs(600)),
            ..given()
        };

        let from_file = Settings::resolve(Given {
            config: Some(path),
            ..given()
        });
        let with_flag = Settings::resolve(flags);
        let by_default = Settings::resolve(Given {
            config: Some(write_config("no-lengths", "")),
            ..given()
        });

        assert_eq!(
            from_file.ok().map(|s| s.track_length),
            Some(TrackLength {
                min: Duration::from_secs(90),
                max: Duration::from_secs(900)
            })
        );
        assert_eq!(
            with_flag.ok().map(|s| s.track_length.max),
            Some(Duration::from_secs(600))
        );
        assert_eq!(
            by_default.ok().map(|s| s.track_length),
            Some(TrackLength {
                min: Duration::ZERO,
                max: Duration::from_secs(1020)
            })
        );
    }

    #[test]
    fn a_track_length_that_admits_nothing_is_an_error() {
        let path = write_config(
            "bad-lengths",
            "min_track = \"15:00\"\nmax_track = \"1:30\"\n",
        );
        let not_a_time = write_config("not-a-time", "max_track = \"a quarter\"\n");

        let empty = Settings::resolve(Given {
            config: Some(path),
            ..given()
        });
        let unreadable = Settings::resolve(Given {
            config: Some(not_a_time),
            ..given()
        })
        .err()
        .map(|error| format!("{error:?}"));

        assert!(empty.is_err());
        assert!(unreadable.is_some_and(|error| error.contains("a quarter")));
    }

    #[test]
    fn an_unknown_key_is_an_error_naming_the_file() {
        let path = write_config("unknown", "libary = \"/music\"\n");

        let error = Settings::resolve(Given {
            config: Some(path),
            ..given()
        })
        .err()
        .map(|error| format!("{error:?}"));

        assert!(error.is_some_and(|error| error.contains("libary")));
    }

    #[test]
    fn a_tilde_means_the_home_directory() {
        let expanded = expand_home(Path::new("~/music"));

        assert!(expanded.ends_with("music"));
        assert!(!expanded.starts_with("~"));
    }
}

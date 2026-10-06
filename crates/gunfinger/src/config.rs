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
//! playback = "turntable"   # or "key-lock", or "both" for CD and digital sets
//! ```

use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};

use miette::{IntoDiagnostic, WrapErr, miette};
use serde::Deserialize;

use crate::playback::PlaybackChoice;
use crate::style::ColorChoice;

const DEFAULT_PEAKS_DIR: &str = "work/peaks";

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    library: Option<PathBuf>,
    peaks_dir: Option<PathBuf>,
    jobs: Option<NonZeroUsize>,
    color: Option<ColorChoice>,
    playback: Option<PlaybackChoice>,
}

/// What the flags and environment variables gave.
pub struct Given {
    pub config: Option<PathBuf>,
    pub peaks_dir: Option<PathBuf>,
    pub jobs: Option<NonZeroUsize>,
    pub color: Option<ColorChoice>,
}

pub struct Settings {
    pub peaks_dir: PathBuf,
    pub jobs: usize,
    pub color: ColorChoice,
    /// How searches assume the records were played, unless `--playback`
    /// says otherwise.
    pub playback: PlaybackChoice,
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
            playback: file.playback.unwrap_or(PlaybackChoice::Turntable),
            library: file.library.map(|library| expand_home(&library)),
            file: path,
        })
    }

    /// The library given on the command line, or the configured one.
    pub fn library(&self, given: Option<PathBuf>) -> miette::Result<PathBuf> {
        given.or_else(|| self.library.clone()).ok_or_else(|| {
            miette!(
                help = format!(
                    "pass --library, or set `library = \"...\"` in {}",
                    default_path().map_or_else(
                        || String::from("the configuration file"),
                        |path| path.display().to_string()
                    )
                ),
                "no library given"
            )
        })
    }
}

fn read(path: &Path) -> miette::Result<File> {
    let text = std::fs::read_to_string(path)
        .into_diagnostic()
        .wrap_err_with(|| format!("cannot read the configuration file {}", path.display()))?;
    toml::from_str(&text)
        .into_diagnostic()
        .wrap_err_with(|| format!("{} is not a valid configuration file", path.display()))
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

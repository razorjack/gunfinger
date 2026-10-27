//! Reading the harness's input files and writing its reports, with the
//! file's path in every error.

use std::fs;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

pub fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("cannot read {}: {error}", path.display()))
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    serde_json::from_str(&read_text(path)?).map_err(|error| format!("{}: {error}", path.display()))
}

pub fn read_toml<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    toml::from_str(&read_text(path)?).map_err(|error| format!("{}: {error}", path.display()))
}

/// The file's lines that are not empty, as written.
pub fn read_lines(path: &Path) -> Result<Vec<String>, String> {
    Ok(read_text(path)?
        .lines()
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect())
}

/// Writes `value` as pretty JSON, creating the directory first.
pub fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let text = serde_json::to_string_pretty(value).map_err(|error| error.to_string())?;
    write_text(path, &text)
}

/// Writes `text`, creating the directory first.
pub fn write_text(path: &Path, text: &str) -> Result<(), String> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    }
    fs::write(path, text).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

/// The file name without its extension, or `default` for a path without
/// one; for naming a report after its input.
pub fn stem_or(path: &Path, default: &str) -> String {
    path.file_stem().map_or_else(
        || default.to_owned(),
        |stem| stem.to_string_lossy().into_owned(),
    )
}

/// The file name without its extension, or the whole path for a path
/// without one.
pub fn stem(path: &Path) -> String {
    stem_or(path, &path.display().to_string())
}

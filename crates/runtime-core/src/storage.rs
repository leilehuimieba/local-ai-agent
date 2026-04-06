use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

pub(crate) fn read_jsonl<T>(path: &Path) -> Vec<T>
where
    T: for<'de> Deserialize<'de>,
{
    let file = match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim_start_matches('\u{feff}').to_string())
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| serde_json::from_str::<T>(&line).ok())
        .collect()
}

pub(crate) fn append_jsonl<T>(path: PathBuf, value: &T) -> Result<(), String>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let payload = serde_json::to_string(value).map_err(|error| error.to_string())?;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    writeln!(file, "{payload}").map_err(|error| error.to_string())
}

pub(crate) fn overwrite_jsonl<T>(path: PathBuf, values: &[T]) -> Result<(), String>
where
    T: Serialize,
{
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    for value in values {
        let payload = serde_json::to_string(value).map_err(|error| error.to_string())?;
        writeln!(file, "{payload}").map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub(crate) fn write_artifact(path: PathBuf, content: &str) -> Result<String, String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(&path, content).map_err(|error| error.to_string())?;
    Ok(path.display().to_string())
}

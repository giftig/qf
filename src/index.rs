mod models;
#[cfg(test)]
mod tests;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Error as IOError;

use serde_json;
use thiserror::Error;

use crate::index::models::ImportIndex;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("IO error accessing index file: {0}")]
    IO(#[from] IOError),
    #[error("Malformed index file: JSON error: {0}")]
    Malformed(#[from] serde_json::Error),
}

type Result<T> = std::result::Result<T, IndexError>;

/// Resolve a language into the absolute path to the file
fn get_import_index_filename(lang: &str) -> Result<PathBuf> {
    let home = env::var("HOME").ok().unwrap_or_else(|| "".to_string());
    let mut path = PathBuf::from(&home);
    path.push(".qf");
    path.push("index");
    path.push("imports");
    path.push(format!("{lang}.json"));
    Ok(path)
}

/// Load the import index for a given language if it exists
pub fn get_import_index(lang: &str, file: &Option<String>) -> Result<ImportIndex> {
    let f = file
        .as_ref()
        .map(|ff| Ok(PathBuf::from(ff)))
        .unwrap_or_else(|| get_import_index_filename(lang))?;

    let data: String = fs::read_to_string(&f)?;

    Ok(serde_json::from_str(&data)?)
}

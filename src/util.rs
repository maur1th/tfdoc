//! Utility functions

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Returns a list of Terraform files based on the file extension `.tf`
///
/// # Errors
/// - Unable to read file
/// - Unable to unwrap entry
pub fn list_tf_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().unwrap_or_else(|| OsStr::new(""));
        if !path.is_dir() && extension == "tf" {
            result.push(path);
        }
    }
    Ok(result)
}

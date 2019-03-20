use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            result.push(entry.path());
        }
    }
    Ok(result)
}

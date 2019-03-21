use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn list_files(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut result: Vec<PathBuf> = vec![];
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let extension = path.extension().unwrap_or(OsStr::new(""));
        if !path.is_dir() && extension == "tf" {
            result.push(path);
        }
    }
    Ok(result)
}

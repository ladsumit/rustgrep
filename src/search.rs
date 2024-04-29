use rayon::prelude::*;
use regex::Regex;
use std::fs::{self, DirEntry};
use std::path::Path;

/// Search for a pattern in all files within a specified directory.
pub fn search_directory(pattern: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(pattern)?;
    visit_dirs(Path::new(path), &|entry: &DirEntry| {
        let path = entry.path();
        if let Ok(contents) = fs::read_to_string(&path) {
            if re.is_match(&contents) {
                println!("Match found in file: {:?}", path);
            }
        }
    })?;

    Ok(())
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

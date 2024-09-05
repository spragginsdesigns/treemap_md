use std::path::PathBuf;
use walkdir::WalkDir;
use rayon::prelude::*;
use std::io;

pub fn traverse_directory(root: &PathBuf, excluded_dirs: &[PathBuf]) -> io::Result<Vec<(PathBuf, bool, u64)>> {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|e| {
            !excluded_dirs.iter().any(|excluded| e.path().starts_with(excluded))
        })
        .par_bridge()
        .map(|entry| {
            let entry = entry?;
            let path = entry.path().to_path_buf();
            let is_dir = entry.file_type().is_dir();
            let size = entry.metadata()?.len();
            Ok((path, is_dir, size))
        })
        .collect()
}
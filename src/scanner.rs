use walkdir::WalkDir;

/// Recursively scan for `.rs` files in the given root directory.
pub fn scan_rust_files(root: &str) -> Vec<String> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file() && entry.path().extension().map_or(false, |e| e == "rs")
        })
        .map(|entry| entry.path().display().to_string())
        .collect()
}


use std::fs;
use std::path::Path;

/// Discovers and returns the content of all plugin files (*.rs) under /plugins
pub fn load_plugin_sources(plugin_dir: &str) -> Vec<(String, String)> {
    let mut plugins = Vec::new();

    if let Ok(entries) = fs::read_dir(plugin_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                        plugins.push((name.to_string(), content));
                    }
                }
            }
        }
    }

    plugins
}


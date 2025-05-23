use chrono::Utc;
use std::fs;

pub fn validate_rust_file(file_path: &str) -> Result<(), String> {
    println!("[{}] Validating file: {}", Utc::now(), file_path);

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    if !content.contains("fn main") {
        return Err("Missing `fn main` entry point.".into());
    }

    if content.contains("let _") {
        return Err("Contains unused variable pattern `let _`.".into());
    }

    Ok(())
}


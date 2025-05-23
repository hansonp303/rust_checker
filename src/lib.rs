use chrono::Utc;
use std::fs;

pub mod unused_checker;
pub mod scanner;

use unused_checker::check_unused_imports;

/// Validate a single Rust file for basic rules.
pub fn validate_rust_file(file_path: &str) -> Result<(), String> {
    println!("[{}] Validating file: {}", Utc::now(), file_path);

    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;

    // Rule 1: Check for missing main
    if !content.contains("fn main") {
        return Err("Missing `fn main` entry point.".into());
    }

    // Rule 2: Check for unused variable pattern
    if content.contains("let _") {
        return Err("Contains unused variable pattern `let _`.".into());
    }

    // Rule 3: Check for unused imports (heuristic)
    if let Some(warning) = check_unused_imports(&content) {
        println!("{}", warning);
    }

    Ok(())
}


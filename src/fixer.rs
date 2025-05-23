use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

/// Removes or comments out lines suspected of unused imports
pub fn auto_fix_unused_imports(file_path: &str) -> Result<(), String> {
    let input = File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(input);

    let mut cleaned_lines = Vec::new();
    for line in reader.lines() {
        let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
        if line.trim_start().starts_with("use") && line.contains("unused") {
            cleaned_lines.push(format!("// [auto-removed] {}", line));
        } else {
            cleaned_lines.push(line);
        }
    }

    fs::write(file_path, cleaned_lines.join("\n"))
        .map_err(|e| format!("Failed to write fixed file: {}", e))?;

    Ok(())
}


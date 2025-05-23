use crate::report::{ValidationSummary, FileValidationResult};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn export_to_html(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let output_path = Path::new(path);
    fs::create_dir_all(output_path.parent().unwrap_or(Path::new("target")))
        .map_err(|e| format!("Failed to create output directory: {}", e))?;

    let mut file = File::create(output_path).map_err(|e| format!("Failed to create file: {}", e))?;

    writeln!(file, "<!DOCTYPE html><html><head><meta charset='utf-8'><title>Rust Checker Report</title>
    <style>
    body {{ font-family: Arial; background: #fdfdfd; color: #333; }}
    h1 {{ color: #005f8a; }}
    table {{ width: 100%; border-collapse: collapse; }}
    th, td {{ padding: 8px; border: 1px solid #ccc; }}
    .pass {{ background-color: #e0ffe0; }}
    .fail {{ background-color: #ffe0e0; }}
    </style></head><body>")?;

    writeln!(file, "<h1>Rust Checker Validation Summary</h1>")?;
    writeln!(
        file,
        "<p><strong>Files Checked:</strong> {} | <strong>Passed:</strong> {} | <strong>Failed:</strong> {}</p>",
        summary.total_files, summary.passed, summary.failed
    )?;

    writeln!(file, "<table><tr><th>File</th><th>Status</th><th>Error</th></tr>")?;
    for FileValidationResult { file, passed, error } in &summary.results {
        let class = if *passed { "pass" } else { "fail" };
        writeln!(
            file,
            "<tr class=\"{}\"><td>{}</td><td>{}</td><td>{}</td></tr>",
            class,
            file,
            if *passed { " Passed" } else { " Failed" },
            error.clone().unwrap_or_default()
        )?;
    }

    writeln!(file, "</table></body></html>")?;

    Ok(())
}


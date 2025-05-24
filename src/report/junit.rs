use crate::report::ValidationSummary;
use std::fs::File;
use std::io::Write;

pub fn export_to_junit_xml(summary: &ValidationSummary, path: &str) -> Result<(), String> {
    let mut file = File::create(path).map_err(|e| format!("Failed to create XML file: {}", e))?;

    writeln!(file, r#"<?xml version="1.0" encoding="UTF-8"?>"#).unwrap();
    writeln!(
        file,
        r#"<testsuite name="RustChecker" tests="{}" failures="{}">"#,
        summary.total_files, summary.failed
    ).unwrap();

    for result in &summary.results {
        writeln!(
            file,
            r#"  <testcase classname="rust_checker" name="{}">"#,
            result.file
        ).unwrap();

        if let Some(error) = &result.error {
            writeln!(
                file,
                r#"    <failure message="{}"/>"#,
                xml_escape(error)
            ).unwrap();
        }

        writeln!(file, r#"  </testcase>"#).unwrap();
    }

    writeln!(file, r#"</testsuite>"#).unwrap();
    Ok(())
}

fn xml_escape(input: &str) -> String {
    input
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
}


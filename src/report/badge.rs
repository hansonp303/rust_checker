use crate::report::ValidationSummary;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn export_svg_badge(summary: &ValidationSummary, output_path: &str) -> Result<(), String> {
    let label = "rust_checker";
    let status = if summary.failed == 0 { "passing" } else { "failing" };
    let color = if summary.failed == 0 { "#4c1" } else { "#e05d44" };

    let svg = format!(r#"<svg xmlns="http://www.w3.org/2000/svg" width="150" height="20">
  <linearGradient id="b" x2="0" y2="100%">
    <stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
    <stop offset="1" stop-opacity=".1"/>
  </linearGradient>
  <mask id="a">
    <rect width="150" height="20" rx="3" fill="#fff"/>
  </mask>
  <g mask="url(#a)">
    <rect width="90" height="20" fill="#555"/>
    <rect x="90" width="60" height="20" fill="{color}"/>
    <rect width="150" height="20" fill="url(#b)"/>
  </g>
  <g fill="#fff" text-anchor="middle"
     font-family="DejaVu Sans,Verdana,Geneva,sans-serif" font-size="11">
    <text x="45" y="15" fill="#010101" fill-opacity=".3">{label}</text>
    <text x="45" y="14">{label}</text>
    <text x="120" y="15" fill="#010101" fill-opacity=".3">{status}</text>
    <text x="120" y="14">{status}</text>
  </g>
</svg>"#,
        label = label,
        status = status,
        color = color
    );

    let output_file = Path::new(output_path);
    fs::create_dir_all(output_file.parent().unwrap_or(Path::new("target")))
        .map_err(|e| format!("Failed to create output dir: {}", e))?;

    let mut file = File::create(output_file).map_err(|e| format!("Failed to create badge file: {}", e))?;
    file.write_all(svg.as_bytes()).map_err(|e| format!("Failed to write SVG badge: {}", e))?;

    Ok(())
}


use std::env;
use std::process::Command;
use std::str;
use rust_checker::{
    validate_rust_file,
    scanner::scan_rust_files,
    rules::RuleConfig,
    report::{
        FileValidationResult, ValidationSummary, print_json_report,
        html::export_to_html,
        badge::export_svg_badge,
    },
    tooling::{run_fmt_check, run_clippy_check},
    config::Config,
    fixer::auto_fix_unused_imports,
};
use chrono::Utc;
use colored::*;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Usage: cargo run -- <path_to_rust_project> [flags]".red());
        eprintln!("{}", "\nOptional flags:".blue());
        eprintln!("  --skip-main             Skip `fn main` check");
        eprintln!("  --allow-unused-var      Allow `let _` pattern");
        eprintln!("  --allow-unused-import   Allow unused `use` statements");
        eprintln!("  --json                  Output report as JSON");
        eprintln!("  --check-fmt             Run `cargo fmt --check`");
        eprintln!("  --check-clippy          Run `cargo clippy --quiet -- -Dwarnings`");
        eprintln!("  --fix                   Auto-fix unused imports (heuristic)\n");
        std::process::exit(1);
    }

    let project_path = &args[1];
    let file_config = Config::load(".rustchecker.toml");
    let config = RuleConfig::from_args_and_config(&args, file_config.rules);
    let output_json = args.contains(&"--json".to_string());
    let check_fmt = args.contains(&"--check-fmt".to_string());
    let check_clippy = args.contains(&"--check-clippy".to_string());
    let auto_fix = args.contains(&"--fix".to_string());

    println!(
        "{}",
        format!("[{}] Checking Rust project recursively at: {}\n", Utc::now(), project_path).blue()
    );

    if check_fmt {
        if run_fmt_check(project_path) {
            println!("{}", " cargo fmt check passed.".green());
        } else {
            eprintln!("{}", " cargo fmt check failed. Please format your code.".red());
        }
    }

    if check_clippy {
        if run_clippy_check(project_path) {
            println!("{}", " cargo clippy check passed.".green());
        } else {
            eprintln!("{}", " cargo clippy check failed. Please fix lint issues.".red());
        }
    }

    let output = Command::new("cargo")
        .arg("check")
        .current_dir(project_path)
        .output()
        .expect("Failed to execute cargo check");

    if output.status.success() {
        println!("{}", " No compilation errors found.\n".green());
    } else {
        println!("{}", " Compilation errors detected:".red());
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
        parse_and_display_errors(stderr);
    }

    let rust_files = scan_rust_files(project_path);
    if rust_files.is_empty() {
        println!("{}", "  No .rs files found in the directory.".yellow());
        return;
    }

    if auto_fix {
        for file_path in &rust_files {
            match auto_fix_unused_imports(file_path) {
                Ok(_) => println!(" ️ Auto-fixed unused imports in {}", file_path),
                Err(e) => eprintln!(" Failed to fix {}: {}", file_path, e),
            }
        }
    }

    let results: Vec<_> = rust_files
        .par_iter()
        .map(|file_path| {
            let result = validate_rust_file(file_path, &config);
            (file_path.clone(), result)
        })
        .collect();

    let mut passed = 0;
    let mut failed = 0;
    let mut output_results = Vec::new();

    for (file_path, result) in results {
        match result {
            Ok(_) => {
                println!("{}", format!(" {} passed validation.", file_path).green());
                passed += 1;
                output_results.push(FileValidationResult {
                    file: file_path,
                    passed: true,
                    error: None,
                });
            }
            Err(e) => {
                eprintln!("{}", format!(" {} failed validation: {}", file_path, e).red());
                failed += 1;
                output_results.push(FileValidationResult {
                    file: file_path,
                    passed: false,
                    error: Some(e),
                });
            }
        }
    }

    let summary = ValidationSummary {
        total_files: passed + failed,
        passed,
        failed,
        results: output_results,
    };

    if output_json {
        print_json_report(&summary);
    } else {
        println!(
            "\n{}",
            format!(
                " Summary:  {} passed |  {} failed |  {} total files checked",
                summary.passed, summary.failed, summary.total_files
            )
            .bold()
        );
    }

    //  HTML report
    if let Err(e) = export_to_html(&summary, "target/report.html") {
        eprintln!(" Failed to export HTML report: {}", e);
    } else {
        println!("{}", " HTML report saved to target/report.html".blue());
    }

    //  ️ SVG badge
    if let Err(e) = export_svg_badge(&summary, "target/status-badge.svg") {
        eprintln!(" Failed to export badge: {}", e);
    } else {
        println!("{}", " ️ Badge saved to target/status-badge.svg".blue());
    }

    //  Exit with non-zero code if validation failed
    if failed > 0 {
        std::process::exit(1);
    }
}

fn parse_and_display_errors(output: &str) {
    for line in output.lines() {
        if line.contains("error:") {
            println!("{}", format!("\n{}", line).red());
        } else if line.contains("--> ") {
            println!("{}", line.yellow());
        } else {
            println!("  {}", line);
        }
    }
}


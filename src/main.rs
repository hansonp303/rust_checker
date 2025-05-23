use std::env;
use std::process::Command;
use std::str;
use rust_checker::{validate_rust_file, scanner::scan_rust_files};
use chrono::Utc;
use colored::*; // Add this for colored terminal output

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", "Usage: cargo run -- <path_to_rust_project>".red());
        std::process::exit(1);
    }

    let project_path = &args[1];
    println!(
        "{}",
        format!("[{}] Checking Rust project recursively at: {}\n", Utc::now(), project_path).blue()
    );

    // Step 1: Run cargo check
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

    // Step 2: Recursively validate each Rust file
    let rust_files = scan_rust_files(project_path);
    if rust_files.is_empty() {
        println!("{}", " No .rs files found in the directory.".yellow());
    }

    for file_path in rust_files {
        match validate_rust_file(&file_path) {
            Ok(_) => println!("{}", format!(" {} passed validation.", file_path).green()),
            Err(e) => eprintln!("{}", format!(" {} failed validation: {}", file_path, e).red()),
        }
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


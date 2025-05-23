use std::env;
use std::process::Command;
use std::str;
use rust_checker::{validate_rust_file, scanner::scan_rust_files};
use chrono::Utc;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <path_to_rust_project>");
        std::process::exit(1);
    }

    let project_path = &args[1];
    println!("[{}] Checking Rust project recursively at: {}\n", Utc::now(), project_path);

    // Step 1: Run cargo check
    let output = Command::new("cargo")
        .arg("check")
        .current_dir(project_path)
        .output()
        .expect("Failed to execute cargo check");

    if output.status.success() {
        println!(" No compilation errors found.\n");
    } else {
        println!(" Compilation errors detected:");
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
        parse_and_display_errors(stderr);
    }

    // Step 2: Recursively validate each Rust file
    let rust_files = scan_rust_files(project_path);
    if rust_files.is_empty() {
        println!("️ No .rs files found in the directory.");
    }

    for file_path in rust_files {
        match validate_rust_file(&file_path) {
            Ok(_) => println!(" {} passed validation.", file_path),
            Err(e) => eprintln!(" {} failed validation: {}", file_path, e),
        }
    }
}

fn parse_and_display_errors(output: &str) {
    for line in output.lines() {
        if line.contains("error:") {
            println!("\n {}", line);
        } else if line.contains("--> ") {
            println!(" {}", line);
        } else {
            println!("  {}", line);
        }
    }
}


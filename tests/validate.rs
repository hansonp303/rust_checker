use rust_checker::validate_rust_file;
use std::fs::File;
use std::io::Write;

#[test]
fn test_validate_valid_file() {
    let path = "tests/temp_valid.rs";
    let mut file = File::create(&path).unwrap();
    writeln!(file, "fn main() {{ println!(\"Hello\"); }}").unwrap();

    let result = validate_rust_file(&path);
    assert!(result.is_ok());

    std::fs::remove_file(&path).unwrap(); // Clean up after test
}

#[test]
fn test_validate_missing_main() {
    let path = "tests/temp_invalid.rs";
    let mut file = File::create(&path).unwrap();
    writeln!(file, "fn helper() {{}}").unwrap();

    let result = validate_rust_file(&path);
    assert!(result.is_err());

    std::fs::remove_file(&path).unwrap(); // Clean up after test
}


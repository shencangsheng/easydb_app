use super::file_utils::{ensure_path_exists, find_files};
use crate::context::error::AppError;
use std::fs::File;
use std::io::Write;

#[test]
fn ensure_path_exists_rejects_missing_literal_path() {
    let err = ensure_path_exists("/tmp/easydb_missing_file_12345.csv").unwrap_err();
    assert!(matches!(err, AppError::FileNotFound { .. }));
}

#[test]
fn ensure_path_exists_accepts_existing_file() {
    let path = std::env::temp_dir().join("easydb_file_utils_test.csv");
    File::create(&path)
        .unwrap()
        .write_all(b"a,b\n1,2\n")
        .unwrap();

    ensure_path_exists(path.to_str().unwrap()).unwrap();

    std::fs::remove_file(path).unwrap();
}

#[test]
fn find_files_rejects_pattern_with_no_matches() {
    let err = find_files("/tmp/easydb_no_match_*.csv").unwrap_err();
    assert!(matches!(err, AppError::FileNotFound { .. }));
}

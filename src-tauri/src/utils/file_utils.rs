use crate::context::error::AppError;
use crate::context::schema::AppResult;
use glob::glob;
use std::path::Path;

fn has_glob_metacharacters(path: &str) -> bool {
    path.contains('*') || path.contains('?') || path.contains('[')
}

/// Ensure the path refers to at least one readable file (literal path or glob pattern).
pub fn ensure_path_exists(path: &str) -> AppResult<()> {
    // DataFusion supports remote URLs (HTTP/S3, etc.); skip local existence checks.
    if path.contains("://") {
        return Ok(());
    }
    if has_glob_metacharacters(path) {
        find_files(path).map(|_| ())
    } else {
        let p = Path::new(path);
        if p.exists() && (p.is_file() || p.is_dir()) {
            Ok(())
        } else {
            Err(AppError::FileNotFound {
                file_name: path.to_string(),
            })
        }
    }
}

pub fn find_files(pattern: &str) -> AppResult<Vec<String>> {
    let mut files: Vec<String> = Vec::new();

    for entry in glob(pattern)? {
        let path = entry?;
        if path.is_file() {
            if let Some(path) = path.to_str() {
                files.push(path.to_string());
            }
        }
    }

    if files.is_empty() {
        Err(AppError::FileNotFound {
            file_name: pattern.to_string(),
        })
    } else {
        Ok(files)
    }
}

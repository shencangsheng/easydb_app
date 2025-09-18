use crate::context::error::AppError;
use crate::context::schema::AppResult;
use glob::glob;

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
        Err(AppError::BadRequest {
            message: "No files found".to_string(),
        })
    } else {
        Ok(files)
    }
}

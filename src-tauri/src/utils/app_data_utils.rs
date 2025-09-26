use crate::context::schema::AppResult;
use std::path::PathBuf;
use tauri::path::BaseDirectory;
use tauri::Manager;

pub fn get_app_data_dir(app: &tauri::AppHandle) -> AppResult<PathBuf> {
    match app.path().resolve("data", BaseDirectory::AppData) {
        Ok(path) => {
            std::fs::create_dir_all(&path)?;
            Ok(path)
        }
        Err(e) => Err(e.into()),
    }
}


use tauri::command;
use crate::context::error::AppError;
use crate::context::schema::AppResult;

#[command]
pub async fn open_url(url: String) -> AppResult<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| AppError::InternalServer {
                message: format!("Failed to open URL: {}", e),
            })?;
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| AppError::InternalServer {
                message: format!("Failed to open URL: {}", e),
            })?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| AppError::InternalServer {
                message: format!("Failed to open URL: {}", e),
            })?;
    }
    
    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        return Err(AppError::InternalServer {
            message: "Opening URLs not supported on this platform".to_string(),
        });
    }
    
    Ok(())
}

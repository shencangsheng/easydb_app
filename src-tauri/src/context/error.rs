use arrow_schema::ArrowError;
use crate::context::error::AppError::{BadRequest, InternalServer};
use calamine::XlsxError;
use datafusion::error::DataFusionError;
use derive_more::with_trait::{Display, Error};
use glob::{GlobError, PatternError};
use sqlparser::parser::ParserError;
use tauri::ipc::InvokeError;
use tauri::App;
use tokio::task::JoinError;

#[derive(Debug, Display, Error, Clone)]
pub enum AppError {
    BadRequest { message: String },
    FileNotFound { file_name: String },
    InternalServer { message: String },
}

impl AppError {
    fn message(&self) -> String {
        match self {
            AppError::BadRequest { message } => message.to_string(),
            AppError::FileNotFound { file_name } => format!("File not found: {}", file_name),
            AppError::InternalServer { message } => message.to_string(),
        }
    }

    fn log_backtrace() {}
}

impl From<ParserError> for AppError {
    fn from(error: ParserError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<AppError> for InvokeError {
    fn from(err: AppError) -> Self {
        AppError::log_backtrace();
        InvokeError::from(err.message())
    }
}

impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::log_backtrace();
        InternalServer {
            message: error.to_string(),
        }
    }
}

impl From<PatternError> for AppError {
    fn from(error: PatternError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<GlobError> for AppError {
    fn from(error: GlobError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<XlsxError> for AppError {
    fn from(error: XlsxError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<JoinError> for AppError {
    fn from(error: JoinError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(error: rusqlite::Error) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<DataFusionError> for AppError {
    fn from(error: DataFusionError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<ArrowError> for AppError {
    fn from(error: ArrowError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}
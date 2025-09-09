use crate::context::error::AppError::BadRequest;
use derive_more::with_trait::{Display, Error};
use polars::error::PolarsError;
use sqlparser::parser::ParserError;
use tauri::ipc::InvokeError;

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
}

impl From<PolarsError> for AppError {
    fn from(error: PolarsError) -> Self {
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<ParserError> for AppError {
    fn from(error: ParserError) -> Self {
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        InvokeError::from(self.message())
    }
}

use crate::context::error::AppError::{BadRequest, InternalServer};
use backtrace::Backtrace;
use derive_more::with_trait::{Display, Error};
use polars::error::PolarsError;
use sqlparser::parser::ParserError;
use std::fmt;
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

    fn log_backtrace() {
        println!("Error: {:?}", Backtrace::new());
    }
}

impl From<PolarsError> for AppError {
    fn from(error: PolarsError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl From<ParserError> for AppError {
    fn from(error: ParserError) -> Self {
        AppError::log_backtrace();
        BadRequest {
            message: error.to_string(),
        }
    }
}

impl Into<InvokeError> for AppError {
    fn into(self) -> InvokeError {
        AppError::log_backtrace();
        InvokeError::from(self.message())
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

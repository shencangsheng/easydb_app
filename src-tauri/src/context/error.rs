use crate::context::error::AppError::BadRequest;
use derive_more::with_trait::{Display, Error};
use polars::error::PolarsError;
use sqlparser::parser::ParserError;
use std::backtrace::Backtrace;

#[derive(Debug, Display, Error, Clone)]
pub enum AppError {
    BadRequest { message: String },
    FileNotFound { file_name: String },
    InternalServer { message: String },
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

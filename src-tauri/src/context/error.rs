use std::backtrace::Backtrace;
use derive_more::with_trait::{Display, Error};
use polars::error::PolarsError;
use crate::context::error::AppError::BadRequest;

#[derive(Debug, Display, Error, Clone)]
pub enum AppError {
    BadRequest { message: String },
    FileNotFound {file_name: String},
    InternalServer { message: String },
}

impl From<PolarsError> for AppError {
    fn from(error: PolarsError) -> Self {
        BadRequest {
            message: error.to_string(),
        }
    }
}
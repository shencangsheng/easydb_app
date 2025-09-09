use crate::context::error::AppError;

pub type AppResult<T> = Result<T, AppError>;


use crate::context::error::AppError;

pub type EasyDBResult<T> = Result<T, AppError>;


use std::future::Future;
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use tokio::task;

pub mod app;
pub mod query;
pub mod utils;

pub async fn run_blocking<F, T>(f: F) -> AppResult<T>
where
    F: FnOnce() -> AppResult<T> + Send + 'static,
    T: Send + 'static,
{
    task::spawn_blocking(f)
        .await
        .map_err(|e| AppError::InternalServer {
            message: e.to_string(),
        })?
}

pub async fn run_blocking_async<F, Fut, T>(f: F) -> AppResult<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = AppResult<T>> + Send + 'static,
    T: Send + 'static,
{
    task::spawn(async move {
        f().await
    })
        .await
        .map_err(|e| AppError::InternalServer {
            message: e.to_string(),
        })?
}
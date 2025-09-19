use crate::context::error::AppError;
use crate::context::schema::AppResult;
use tokio::task;

pub mod query;

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

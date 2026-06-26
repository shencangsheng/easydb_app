use crate::commands::run_blocking;
use crate::context::schema::AppResult;
use crate::utils::db_utils;
use crate::utils::excel_cache::{self, ClearExcelCacheResult, ExcelCacheEntry};
use serde::Serialize;
use tauri::{command, AppHandle};

#[derive(Serialize)]
pub struct ExcelIndexSettingsResponse {
    pub enabled: bool,
    pub threshold_mb: u64,
}

#[derive(Serialize)]
pub struct ExcelCacheStatsResponse {
    pub entry_count: usize,
    pub total_bytes: u64,
    pub entries: Vec<ExcelCacheEntry>,
}

#[command]
pub async fn get_excel_index_settings(app: AppHandle) -> AppResult<ExcelIndexSettingsResponse> {
    run_blocking(move || {
        let settings = db_utils::get_excel_index_settings(&app)?;
        Ok(ExcelIndexSettingsResponse {
            enabled: settings.enabled,
            threshold_mb: settings.threshold_mb,
        })
    })
    .await
}

#[command]
pub async fn set_excel_index_settings(
    app: AppHandle,
    enabled: bool,
    threshold_mb: u64,
) -> AppResult<()> {
    run_blocking(move || db_utils::set_excel_index_settings(&app, enabled, threshold_mb)).await
}

#[command]
pub async fn get_excel_cache_stats(app: AppHandle) -> AppResult<ExcelCacheStatsResponse> {
    run_blocking(move || {
        let entries = excel_cache::list_entries(&app)?;
        let total_bytes = entries.iter().map(|e| e.parquet_bytes).sum();
        Ok(ExcelCacheStatsResponse {
            entry_count: entries.len(),
            total_bytes,
            entries,
        })
    })
    .await
}

#[command]
pub async fn clear_excel_cache(app: AppHandle) -> AppResult<ClearExcelCacheResult> {
    run_blocking(move || excel_cache::clear_all(&app)).await
}

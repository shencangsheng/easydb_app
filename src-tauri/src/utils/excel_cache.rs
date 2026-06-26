use crate::context::schema::AppResult;
use crate::utils::app_data_utils::get_app_data_dir;
use chrono::Local;
use datafusion::arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::AppHandle;

const CACHE_SUBDIR: &str = "excel_cache";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExcelCacheMeta {
    pub hash: String,
    pub source_path: String,
    pub sheet_name: String,
    pub infer_schema: bool,
    pub source_mtime_secs: u64,
    pub source_size: u64,
    pub created_at: String,
    pub parquet_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExcelCacheEntry {
    pub hash: String,
    pub source_path: String,
    pub sheet_name: String,
    pub infer_schema: bool,
    pub created_at: String,
    pub parquet_bytes: u64,
}

pub struct ExcelCacheKey {
    pub hash: String,
    pub source_path: String,
    pub sheet_name: String,
    pub infer_schema: bool,
    pub source_mtime_secs: u64,
    pub source_size: u64,
}

impl ExcelCacheKey {
    pub fn from_source(
        source_path: &str,
        sheet_name: &str,
        infer_schema: bool,
    ) -> AppResult<Self> {
        let path = Path::new(source_path);
        let absolute = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let absolute_str = absolute.to_string_lossy().to_string();
        let meta = fs::metadata(&absolute)?;
        let mtime_secs = meta
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let size = meta.len();
        let hash = compute_hash(&absolute_str, mtime_secs, size, sheet_name, infer_schema);

        Ok(Self {
            hash,
            source_path: absolute_str,
            sheet_name: sheet_name.to_string(),
            infer_schema,
            source_mtime_secs: mtime_secs,
            source_size: size,
        })
    }
}

pub(crate) fn compute_hash(
    absolute_path: &str,
    mtime_secs: u64,
    size: u64,
    sheet_name: &str,
    infer_schema: bool,
) -> String {
    let payload = format!(
        "{}|{}|{}|{}|{}",
        absolute_path, mtime_secs, size, sheet_name, infer_schema
    );
    let digest = Sha256::digest(payload.as_bytes());
    format!("{:x}", digest)
}

pub fn cache_dir(app: &AppHandle) -> AppResult<PathBuf> {
    cache_dir_from_base(&get_app_data_dir(app)?)
}

pub(crate) fn cache_dir_from_base(base: &Path) -> AppResult<PathBuf> {
    let dir = base.join(CACHE_SUBDIR);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn parquet_path(cache_dir: &Path, hash: &str) -> PathBuf {
    cache_dir.join(format!("{hash}.parquet"))
}

fn meta_path(cache_dir: &Path, hash: &str) -> PathBuf {
    cache_dir.join(format!("{hash}.meta.json"))
}

pub fn lookup_at(cache_dir: &Path, key: &ExcelCacheKey) -> AppResult<Option<PathBuf>> {
    let parquet = parquet_path(cache_dir, &key.hash);
    let meta_file = meta_path(cache_dir, &key.hash);

    if !parquet.is_file() || !meta_file.is_file() {
        return Ok(None);
    }

    let meta_content = fs::read_to_string(&meta_file)?;
    let meta: ExcelCacheMeta = serde_json::from_str(&meta_content)?;

    if meta.source_mtime_secs != key.source_mtime_secs || meta.source_size != key.source_size {
        return Ok(None);
    }

    Ok(Some(parquet))
}

pub fn lookup(app: &AppHandle, key: &ExcelCacheKey) -> AppResult<Option<PathBuf>> {
    lookup_at(&cache_dir(app)?, key)
}

pub fn write_batch_at(
    cache_dir: &Path,
    key: &ExcelCacheKey,
    batch: &RecordBatch,
) -> AppResult<PathBuf> {
    fs::create_dir_all(cache_dir)?;
    let parquet = parquet_path(cache_dir, &key.hash);
    let meta_file = meta_path(cache_dir, &key.hash);

    let file = fs::File::create(&parquet)?;
    let mut writer = ArrowWriter::try_new(file, batch.schema(), None)?;
    writer.write(batch)?;
    writer.close()?;

    let parquet_bytes = fs::metadata(&parquet)?.len();
    let meta = ExcelCacheMeta {
        hash: key.hash.clone(),
        source_path: key.source_path.clone(),
        sheet_name: key.sheet_name.clone(),
        infer_schema: key.infer_schema,
        source_mtime_secs: key.source_mtime_secs,
        source_size: key.source_size,
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        parquet_bytes,
    };
    fs::write(&meta_file, serde_json::to_string_pretty(&meta)?)?;

    Ok(parquet)
}

pub fn write_batch(app: &AppHandle, key: &ExcelCacheKey, batch: &RecordBatch) -> AppResult<PathBuf> {
    write_batch_at(&cache_dir(app)?, key, batch)
}

pub fn list_entries_at(cache_dir: &Path) -> AppResult<Vec<ExcelCacheEntry>> {
    let mut entries = Vec::new();

    if !cache_dir.is_dir() {
        return Ok(entries);
    }

    for entry in fs::read_dir(cache_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if !file_name.ends_with(".meta.json") {
            continue;
        }
        let content = fs::read_to_string(&path)?;
        if let Ok(meta) = serde_json::from_str::<ExcelCacheMeta>(&content) {
            entries.push(ExcelCacheEntry {
                hash: meta.hash,
                source_path: meta.source_path,
                sheet_name: meta.sheet_name,
                infer_schema: meta.infer_schema,
                created_at: meta.created_at,
                parquet_bytes: meta.parquet_bytes,
            });
        }
    }

    entries.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(entries)
}

pub fn list_entries(app: &AppHandle) -> AppResult<Vec<ExcelCacheEntry>> {
    list_entries_at(&cache_dir(app)?)
}

pub fn total_size_bytes(app: &AppHandle) -> AppResult<u64> {
    let entries = list_entries(app)?;
    Ok(entries.iter().map(|e| e.parquet_bytes).sum())
}

#[derive(Debug, Clone, Serialize)]
pub struct ClearExcelCacheResult {
    pub deleted_count: usize,
    pub freed_bytes: u64,
}

pub fn clear_all_at(cache_dir: &Path) -> AppResult<ClearExcelCacheResult> {
    let entries = list_entries_at(cache_dir)?;
    let freed_bytes: u64 = entries.iter().map(|e| e.parquet_bytes).sum();
    let deleted_count = entries.len();

    for entry in &entries {
        let _ = fs::remove_file(parquet_path(cache_dir, &entry.hash));
        let _ = fs::remove_file(meta_path(cache_dir, &entry.hash));
    }

    Ok(ClearExcelCacheResult {
        deleted_count,
        freed_bytes,
    })
}

pub fn clear_all(app: &AppHandle) -> AppResult<ClearExcelCacheResult> {
    clear_all_at(&cache_dir(app)?)
}

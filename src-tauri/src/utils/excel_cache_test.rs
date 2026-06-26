use super::excel_cache::{
    cache_dir_from_base, clear_all_at, compute_hash, lookup_at, write_batch_at, ExcelCacheKey,
};
use datafusion::arrow::array::{Int64Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

fn sample_batch() -> RecordBatch {
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]));
    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int64Array::from(vec![Some(1), Some(2)])),
            Arc::new(StringArray::from(vec![Some("a"), Some("b")])),
        ],
    )
    .expect("batch")
}

fn temp_cache_root() -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "easydb_excel_cache_test_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&root).expect("create temp root");
    root
}

fn write_temp_file(content: &[u8]) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "easydb_cache_source_{}_{}.xlsx",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::write(&path, content).expect("write temp file");
    path
}

#[test]
fn test_compute_hash_is_deterministic() {
    let h1 = compute_hash("/tmp/a.xlsx", 100, 500, "Sheet1", true);
    let h2 = compute_hash("/tmp/a.xlsx", 100, 500, "Sheet1", true);
    let h3 = compute_hash("/tmp/a.xlsx", 101, 500, "Sheet1", true);
    assert_eq!(h1, h2);
    assert_ne!(h1, h3);
}

#[test]
fn test_write_batch_and_lookup_hit() {
    let root = temp_cache_root();
    let cache_dir = cache_dir_from_base(&root).expect("cache dir");
    let source = write_temp_file(b"fake-xlsx-content-for-cache");
    let source_str = source.to_string_lossy().to_string();

    let key = ExcelCacheKey::from_source(&source_str, "Sheet1", true).expect("key");
    let written = write_batch_at(&cache_dir, &key, &sample_batch()).expect("write");
    assert!(written.is_file());

    let found = lookup_at(&cache_dir, &key).expect("lookup");
    assert_eq!(found, Some(written));

    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn test_lookup_miss_after_source_modified() {
    let root = temp_cache_root();
    let cache_dir = cache_dir_from_base(&root).expect("cache dir");
    let source = write_temp_file(b"version-1");
    let source_str = source.to_string_lossy().to_string();

    let key = ExcelCacheKey::from_source(&source_str, "Sheet1", true).expect("key");
    write_batch_at(&cache_dir, &key, &sample_batch()).expect("write");

    fs::write(&source, b"version-2-extended").expect("modify source");
    let new_key = ExcelCacheKey::from_source(&source_str, "Sheet1", true).expect("new key");

    let found = lookup_at(&cache_dir, &new_key).expect("lookup");
    assert!(found.is_none());

    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn test_clear_all_removes_entries() {
    let root = temp_cache_root();
    let cache_dir = cache_dir_from_base(&root).expect("cache dir");
    let source = write_temp_file(b"clear-test");
    let source_str = source.to_string_lossy().to_string();
    let key = ExcelCacheKey::from_source(&source_str, "Sheet1", false).expect("key");
    write_batch_at(&cache_dir, &key, &sample_batch()).expect("write");

    let result = clear_all_at(&cache_dir).expect("clear");
    assert_eq!(result.deleted_count, 1);
    assert!(result.freed_bytes > 0);
    assert!(fs::read_dir(&cache_dir).expect("read dir").next().is_none());

    let _ = fs::remove_file(&source);
    let _ = fs::remove_dir_all(&root);
}

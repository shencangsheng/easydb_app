use super::context::{
    collect, detect_json_newline_delimited, get_json_read_options, get_sql_context,
    path_file_extension, register,
};
use crate::commands::query::ColumnTypeInfo;
use crate::context::schema::AppResult;
use crate::sql::parse::parse_statements;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::display::{ArrayFormatter, FormatOptions};
use datafusion::prelude::JsonReadOptions;
use sqlparser::ast::SetExpr::Select;
use sqlparser::ast::{Statement, TableFactor, TableFunctionArgs};

// ═══════════════════════════════════════════════════════════════════════
// Test helpers
// ═══════════════════════════════════════════════════════════════════════

/// Write `content` to a uniquely named temp file and return its path. The name
/// embeds the process id and the requested suffix so parallel tests don't
/// collide. Returns the path so callers can clean it up afterwards.
fn write_temp(suffix: &str, content: &str) -> AppResult<std::path::PathBuf> {
    let mut path = std::env::temp_dir();
    path.push(format!("easydb_ctx_{}_{}", std::process::id(), suffix));
    std::fs::write(&path, content)?;
    Ok(path)
}

/// Run a query through the same pipeline the Tauri commands use: rewrite the
/// `read_*` table functions into registered tables, then collect the results.
async fn run_query(sql: &str) -> AppResult<(Vec<ColumnTypeInfo>, Vec<RecordBatch>)> {
    let mut ctx = get_sql_context();
    let rewritten = register(&mut ctx, sql, None, None).await?;
    collect(&mut ctx, &rewritten).await
}

/// Index of the column named `name` within the collected metadata.
fn col_index(columns: &[ColumnTypeInfo], name: &str) -> usize {
    columns
        .iter()
        .position(|c| c.column_name == name)
        .unwrap_or_else(|| panic!("column '{}' not found", name))
}

/// Flatten one column across all record batches into display strings, so
/// assertions are independent of how DataFusion chunks the output.
fn column_strings(batches: &[RecordBatch], col: usize) -> AppResult<Vec<String>> {
    let options = FormatOptions::default();
    let mut out = Vec::new();
    for batch in batches {
        let formatter = ArrayFormatter::try_new(batch.column(col), &options)?;
        for row in 0..batch.num_rows() {
            out.push(formatter.value(row).to_string());
        }
    }
    Ok(out)
}

fn total_rows(batches: &[RecordBatch]) -> usize {
    batches.iter().map(|b| b.num_rows()).sum()
}

/// Pull the `TableFunctionArgs` out of the first table function in `sql`.
fn first_table_args(sql: &str) -> AppResult<Option<TableFunctionArgs>> {
    let mut ast = parse_statements(sql)?;
    if ast.is_empty() {
        return Ok(None);
    }
    if let Statement::Query(query) = ast.remove(0) {
        if let Select(select) = *query.body {
            if let Some(twj) = select.from.into_iter().next() {
                if let TableFactor::Table { args, .. } = twj.relation {
                    return Ok(args);
                }
            }
        }
    }
    Ok(None)
}

/// Shared sample matching `fixtures/users.*`.
const USERS_NDJSON: &str = "\
{\"id\":1,\"name\":\"alice\",\"age\":30,\"active\":true,\"score\":9.5}
{\"id\":2,\"name\":\"bob\",\"age\":25,\"active\":false,\"score\":7.25}
{\"id\":3,\"name\":\"carol\",\"age\":41,\"active\":true,\"score\":8.8}
{\"id\":4,\"name\":\"dan\",\"age\":38,\"active\":false,\"score\":5.0}
{\"id\":5,\"name\":\"eve\",\"age\":29,\"active\":true,\"score\":9.99}
";

const USERS_JSON_ARRAY: &str = "\
[
  {\"id\":1,\"name\":\"alice\",\"age\":30,\"active\":true,\"score\":9.5},
  {\"id\":2,\"name\":\"bob\",\"age\":25,\"active\":false,\"score\":7.25},
  {\"id\":3,\"name\":\"carol\",\"age\":41,\"active\":true,\"score\":8.8},
  {\"id\":4,\"name\":\"dan\",\"age\":38,\"active\":false,\"score\":5.0},
  {\"id\":5,\"name\":\"eve\",\"age\":29,\"active\":true,\"score\":9.99}
]
";

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests — path_file_extension
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_path_file_extension_common_cases() {
    assert_eq!(path_file_extension("users.json").as_deref(), Some(".json"));
    assert_eq!(
        path_file_extension("data/users.ndjson").as_deref(),
        Some(".ndjson")
    );
    assert_eq!(
        path_file_extension("dir/users_*.json").as_deref(),
        Some(".json")
    );
}

#[test]
fn test_path_file_extension_no_extension_returns_none() {
    assert_eq!(path_file_extension("dir/"), None);
    assert_eq!(path_file_extension("plainfile"), None);
}

#[test]
fn test_path_file_extension_multiple_dots_uses_last() {
    assert_eq!(
        path_file_extension("archive.2026.json").as_deref(),
        Some(".json")
    );
}

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests — detect_json_newline_delimited
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_detect_json_newline_delimited_sniffs_array_content() -> AppResult<()> {
    // Leading whitespace before `[` must still be detected as an array.
    let path = write_temp("detect_array.json", "  \n  [ {\"a\":1} ]")?;
    let result = detect_json_newline_delimited(&path.to_string_lossy());
    let _ = std::fs::remove_file(&path);
    assert!(!result, "array content should not be newline-delimited");
    Ok(())
}

#[test]
fn test_detect_json_newline_delimited_sniffs_ndjson_content() -> AppResult<()> {
    // NDJSON content inside a `.json` file must be detected as newline-delimited.
    let path = write_temp("detect_nd.json", "{\"a\":1}\n{\"a\":2}\n")?;
    let result = detect_json_newline_delimited(&path.to_string_lossy());
    let _ = std::fs::remove_file(&path);
    assert!(result, "ndjson content should be newline-delimited");
    Ok(())
}

#[test]
fn test_detect_json_newline_delimited_skips_utf8_bom() -> AppResult<()> {
    // A UTF-8 BOM before a `[` must not be mistaken for NDJSON content.
    let path = write_temp("detect_bom_array.json", "\u{feff}[ {\"a\":1} ]")?;
    let result = detect_json_newline_delimited(&path.to_string_lossy());
    let _ = std::fs::remove_file(&path);
    assert!(!result, "BOM-prefixed array should not be newline-delimited");
    Ok(())
}

#[test]
fn test_detect_json_newline_delimited_falls_back_to_extension() {
    // Unreadable paths (e.g. globs) fall back to the extension.
    assert!(detect_json_newline_delimited("missing/dir/*.ndjson"));
    assert!(!detect_json_newline_delimited("missing/dir/*.json"));
    assert!(!detect_json_newline_delimited("missing/dir/"));
}

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests — get_json_read_options
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_get_json_read_options_preserves_base_when_no_args() -> AppResult<()> {
    let mut args: Option<TableFunctionArgs> = None;
    let base = JsonReadOptions::default().newline_delimited(false);
    let result = get_json_read_options(&mut args, base)?;

    // The array-format flag set by the caller must survive untouched.
    assert!(!result.newline_delimited);
    assert_eq!(result.file_extension, ".json");
    Ok(())
}

#[test]
fn test_get_json_read_options_overrides_file_extension() -> AppResult<()> {
    let mut args =
        first_table_args("SELECT * FROM read_ndjson('x', file_extension => '.custom')")?;
    let base = JsonReadOptions::default();
    let result = get_json_read_options(&mut args, base)?;

    assert_eq!(result.file_extension, ".custom");
    Ok(())
}

#[test]
fn test_get_json_read_options_ignores_unknown_args() -> AppResult<()> {
    let mut args = first_table_args("SELECT * FROM read_json('x', unknown_opt => 'value')")?;
    let base = JsonReadOptions::default().newline_delimited(false);
    let result = get_json_read_options(&mut args, base)?;

    assert!(!result.newline_delimited);
    assert_eq!(result.file_extension, ".json");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests — read_json / read_ndjson end-to-end
// ═══════════════════════════════════════════════════════════════════════

/// Assert the collected users data matches the shared fixture regardless of
/// column ordering or batch chunking.
fn assert_users(columns: &[ColumnTypeInfo], batches: &[RecordBatch]) -> AppResult<()> {
    assert_eq!(columns.len(), 5, "expected 5 columns");
    assert_eq!(total_rows(batches), 5, "expected 5 rows");

    let id = col_index(columns, "id");
    let name = col_index(columns, "name");
    let active = col_index(columns, "active");
    let score = col_index(columns, "score");

    assert_eq!(columns[id].arrow_type, "Int64");
    assert_eq!(columns[name].arrow_type, "Utf8");
    assert_eq!(columns[active].arrow_type, "Boolean");
    assert_eq!(columns[score].arrow_type, "Float64");

    assert_eq!(
        column_strings(batches, name)?,
        vec!["alice", "bob", "carol", "dan", "eve"]
    );
    assert_eq!(
        column_strings(batches, active)?,
        vec!["true", "false", "true", "false", "true"]
    );
    Ok(())
}

#[tokio::test]
async fn test_read_json_parses_standard_array() -> AppResult<()> {
    let path = write_temp("array.json", USERS_JSON_ARRAY)?;
    let sql = format!("SELECT * FROM read_json('{}')", path.to_string_lossy());

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (columns, batches) = result?;

    assert_users(&columns, &batches)
}

#[tokio::test]
async fn test_read_ndjson_parses_newline_delimited() -> AppResult<()> {
    let path = write_temp("nd.ndjson", USERS_NDJSON)?;
    let sql = format!("SELECT * FROM read_ndjson('{}')", path.to_string_lossy());

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (columns, batches) = result?;

    assert_users(&columns, &batches)
}

/// Regression: the frontend maps dropped `.json` files to `read_ndjson(...)`.
/// Because the extension filter is derived from the path, NDJSON content stored
/// in a `.json` file must still load.
#[tokio::test]
async fn test_read_ndjson_supports_json_extension() -> AppResult<()> {
    let path = write_temp("ndjson_in.json", USERS_NDJSON)?;
    let sql = format!("SELECT * FROM read_ndjson('{}')", path.to_string_lossy());

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (_columns, batches) = result?;

    assert_eq!(total_rows(&batches), 5);
    Ok(())
}

/// `read_json` auto-detects newline-delimited content, so NDJSON stored in a
/// `.json` file loads correctly through `read_json`.
#[tokio::test]
async fn test_read_json_handles_ndjson_content() -> AppResult<()> {
    let path = write_temp("json_with_ndjson.json", USERS_NDJSON)?;
    let sql = format!("SELECT * FROM read_json('{}')", path.to_string_lossy());

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (columns, batches) = result?;

    assert_users(&columns, &batches)
}

/// `read_json` also accepts a real `.ndjson` file (frontend now routes
/// `.ndjson` here too).
#[tokio::test]
async fn test_read_json_reads_ndjson_file() -> AppResult<()> {
    let path = write_temp("real.ndjson", USERS_NDJSON)?;
    let sql = format!("SELECT * FROM read_json('{}')", path.to_string_lossy());

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (columns, batches) = result?;

    assert_users(&columns, &batches)
}

/// A WHERE clause over the parsed array confirms typed columns are queryable,
/// not just present.
#[tokio::test]
async fn test_read_json_array_is_filterable() -> AppResult<()> {
    let path = write_temp("filter.json", USERS_JSON_ARRAY)?;
    let sql = format!(
        "SELECT name FROM read_json('{}') WHERE active = true ORDER BY id",
        path.to_string_lossy()
    );

    let result = run_query(&sql).await;
    let _ = std::fs::remove_file(&path);
    let (columns, batches) = result?;

    assert_eq!(columns.len(), 1);
    assert_eq!(column_strings(&batches, 0)?, vec!["alice", "carol", "eve"]);
    Ok(())
}

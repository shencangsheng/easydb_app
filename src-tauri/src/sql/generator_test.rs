use super::generator::{
    format_bool_for_sql, format_cell_for_sql, format_value_for_sql,
    generate_sql_inserts_from_batches, generate_sql_update_from_batches, parse_sql_type,
    resolve_export_specs, strip_float_zero_suffix, truncate_record_batches, ExportColumnConfig,
    SqlType,
};
use crate::commands::query::Dialect;
use datafusion::arrow::array::{
    BooleanArray, Float64Array, Int32Array, Int64Array, StringArray, TimestampNanosecondArray,
};
use datafusion::arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use datafusion::arrow::record_batch::RecordBatch;
use std::sync::Arc;

// ═══════════════════════════════════════════════════════════════════════
// Stability Tests
// ═══════════════════════════════════════════════════════════════════════

// ─── parse_sql_type ───────────────────────────────────────────────────

#[test]
fn test_parse_sql_type_bool_variants() {
    assert_eq!(parse_sql_type("BOOL"), SqlType::Bool);
    assert_eq!(parse_sql_type("BOOLEAN"), SqlType::Bool);
    assert_eq!(parse_sql_type("bool"), SqlType::Bool);
    assert_eq!(parse_sql_type("boolean"), SqlType::Bool);
}

#[test]
fn test_parse_sql_type_int_variants() {
    assert_eq!(parse_sql_type("INT"), SqlType::Int);
    assert_eq!(parse_sql_type("INTEGER"), SqlType::Int);
    assert_eq!(parse_sql_type("BIGINT"), SqlType::Int);
    assert_eq!(parse_sql_type("SMALLINT"), SqlType::Int);
    assert_eq!(parse_sql_type("TINYINT"), SqlType::Int);
    assert_eq!(parse_sql_type("int"), SqlType::Int);
}

#[test]
fn test_parse_sql_type_int4_and_float8() {
    // INT4 starts with "INT" -> Int; FLOAT8 starts with "FLOAT" -> Float
    assert_eq!(parse_sql_type("INT4"), SqlType::Int);
    assert_eq!(parse_sql_type("FLOAT8"), SqlType::Float);
}

#[test]
fn test_parse_sql_type_float_variants() {
    assert_eq!(parse_sql_type("DOUBLE"), SqlType::Float);
    assert_eq!(parse_sql_type("FLOAT"), SqlType::Float);
    assert_eq!(parse_sql_type("DECIMAL"), SqlType::Float);
    assert_eq!(parse_sql_type("NUMERIC"), SqlType::Float);
    assert_eq!(parse_sql_type("REAL"), SqlType::Float);
    assert_eq!(parse_sql_type("DOUBLE PRECISION"), SqlType::Float);
    assert_eq!(parse_sql_type("FLOAT8"), SqlType::Float);
}

#[test]
fn test_parse_sql_type_text_variants() {
    assert_eq!(parse_sql_type("TEXT"), SqlType::Text);
    assert_eq!(parse_sql_type("CHAR"), SqlType::Text);
    assert_eq!(parse_sql_type("VARCHAR"), SqlType::Text);
    assert_eq!(parse_sql_type("VARCHAR(255)"), SqlType::Text);
    assert_eq!(parse_sql_type("CHARACTER"), SqlType::Text);
    assert_eq!(parse_sql_type("STRING"), SqlType::Text);
}

#[test]
fn test_parse_sql_type_unknown() {
    assert_eq!(parse_sql_type(""), SqlType::Unknown);
    assert_eq!(parse_sql_type("DATE"), SqlType::Unknown);
    assert_eq!(parse_sql_type("TIMESTAMP"), SqlType::Unknown);
    assert_eq!(parse_sql_type("BLOB"), SqlType::Unknown);
    assert_eq!(parse_sql_type("JSON"), SqlType::Unknown);
}

// ─── resolve_export_specs ─────────────────────────────────────────────

#[test]
fn test_resolve_export_specs_default_no_config() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let specs = resolve_export_specs(&headers, None).unwrap();
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0].source_index, 0);
    assert_eq!(specs[0].export_name, "a");
    assert_eq!(specs[0].sql_type, SqlType::Unknown);
    assert_eq!(specs[1].source_index, 1);
    assert_eq!(specs[1].export_name, "b");
    assert_eq!(specs[1].sql_type, SqlType::Unknown);
}

#[test]
fn test_resolve_export_specs_with_config() {
    let headers = vec!["id".to_string(), "name".to_string()];
    let config = vec![
        ExportColumnConfig {
            source_column_name: "id".to_string(),
            export_column_name: "user_id".to_string(),
            sql_type: "INT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "name".to_string(),
            export_column_name: "user_name".to_string(),
            sql_type: "VARCHAR".to_string(),
        },
    ];
    let specs = resolve_export_specs(&headers, Some(&config)).unwrap();
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0].export_name, "user_id");
    assert_eq!(specs[0].sql_type, SqlType::Int);
    assert_eq!(specs[1].export_name, "user_name");
    assert_eq!(specs[1].sql_type, SqlType::Text);
}

#[test]
fn test_resolve_export_specs_rejects_empty_export_name() {
    let headers = vec!["id".to_string()];
    let config = vec![ExportColumnConfig {
        source_column_name: "id".to_string(),
        export_column_name: "  ".to_string(),
        sql_type: "INT".to_string(),
    }];
    assert!(resolve_export_specs(&headers, Some(&config)).is_err());
}

#[test]
fn test_resolve_export_specs_rejects_duplicate_export_names() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let config = vec![
        ExportColumnConfig {
            source_column_name: "a".to_string(),
            export_column_name: "col".to_string(),
            sql_type: "INT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "b".to_string(),
            export_column_name: "col".to_string(),
            sql_type: "TEXT".to_string(),
        },
    ];
    let result = resolve_export_specs(&headers, Some(&config));
    assert!(result.is_err());
    let err_msg = format!("{:?}", result.unwrap_err());
    assert!(
        err_msg.contains("Duplicate export column name"),
        "Expected duplicate error, got: {}",
        err_msg
    );
}

#[test]
fn test_resolve_export_specs_rejects_missing_source_column() {
    let headers = vec!["a".to_string()];
    let config = vec![ExportColumnConfig {
        source_column_name: "nonexistent".to_string(),
        export_column_name: "col".to_string(),
        sql_type: "INT".to_string(),
    }];
    assert!(resolve_export_specs(&headers, Some(&config)).is_err());
}

#[test]
fn test_resolve_export_specs_rejects_empty_config() {
    let headers = vec!["a".to_string()];
    let config: Vec<ExportColumnConfig> = vec![];
    assert!(resolve_export_specs(&headers, Some(&config)).is_err());
}

#[test]
fn test_resolve_export_specs_trims_export_name() {
    let headers = vec!["id".to_string()];
    let config = vec![ExportColumnConfig {
        source_column_name: "id".to_string(),
        export_column_name: "  user_id  ".to_string(),
        sql_type: "INT".to_string(),
    }];
    let specs = resolve_export_specs(&headers, Some(&config)).unwrap();
    assert_eq!(specs[0].export_name, "user_id");
}

#[test]
fn test_resolve_export_specs_allows_same_source_different_export() {
    let headers = vec!["a".to_string(), "b".to_string()];
    let config = vec![
        ExportColumnConfig {
            source_column_name: "a".to_string(),
            export_column_name: "col_a".to_string(),
            sql_type: "INT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "b".to_string(),
            export_column_name: "col_b".to_string(),
            sql_type: "TEXT".to_string(),
        },
    ];
    let specs = resolve_export_specs(&headers, Some(&config)).unwrap();
    assert_eq!(specs.len(), 2);
    assert_eq!(specs[0].export_name, "col_a");
    assert_eq!(specs[1].export_name, "col_b");
}

// ─── format_bool_for_sql ──────────────────────────────────────────────

#[test]
fn test_format_bool_for_sql_known_values() {
    assert_eq!(format_bool_for_sql("true"), "true");
    assert_eq!(format_bool_for_sql("false"), "false");
    assert_eq!(format_bool_for_sql("TRUE"), "true");
    assert_eq!(format_bool_for_sql("FALSE"), "false");
    assert_eq!(format_bool_for_sql("1"), "true");
    assert_eq!(format_bool_for_sql("0"), "false");
    assert_eq!(format_bool_for_sql("NULL"), "NULL");
}

#[test]
fn test_format_bool_for_sql_unrecognized_returns_null() {
    // Previously returned a quoted string which caused PostgreSQL type mismatch
    assert_eq!(format_bool_for_sql("yes"), "NULL");
    assert_eq!(format_bool_for_sql("no"), "NULL");
    assert_eq!(format_bool_for_sql("2"), "NULL");
    assert_eq!(format_bool_for_sql("random"), "NULL");
}

// ─── format_cell_for_sql: boolean-to-numeric conversion ──────────────

#[test]
fn test_format_cell_bool_to_int() {
    assert_eq!(format_cell_for_sql("true", SqlType::Int, false), "1");
    assert_eq!(format_cell_for_sql("false", SqlType::Int, false), "0");
}

#[test]
fn test_format_cell_bool_to_float() {
    assert_eq!(format_cell_for_sql("true", SqlType::Float, false), "1");
    assert_eq!(format_cell_for_sql("false", SqlType::Float, false), "0");
}

// ─── format_cell_for_sql: integer type ────────────────────────────────

#[test]
fn test_format_cell_int_type() {
    assert_eq!(format_cell_for_sql("42", SqlType::Int, false), "42");
    assert_eq!(format_cell_for_sql("-5", SqlType::Int, false), "-5");
    assert_eq!(format_cell_for_sql("3.7", SqlType::Int, false), "3"); // truncates float
    assert_eq!(format_cell_for_sql("NULL", SqlType::Int, false), "NULL");
}

#[test]
fn test_format_cell_int_large_unsigned() {
    assert_eq!(
        format_cell_for_sql("18446744073709551615", SqlType::Int, false),
        "18446744073709551615"
    );
}

#[test]
fn test_format_cell_int_non_numeric_returns_null() {
    // Regression: previously produced quoted string like 'hello' which is invalid SQL for INT columns
    assert_eq!(format_cell_for_sql("hello", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("N/A", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("yes", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("abc123", SqlType::Int, false), "NULL");
}

// ─── format_cell_for_sql: float type ──────────────────────────────────

#[test]
fn test_format_cell_float_type() {
    assert_eq!(format_cell_for_sql("3.14", SqlType::Float, false), "3.14");
    assert_eq!(format_cell_for_sql("42", SqlType::Float, false), "42");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Float, false), "NULL");
}

#[test]
fn test_format_cell_float_non_numeric_returns_null() {
    // Regression: previously produced quoted string like 'hello' which is invalid SQL for FLOAT columns
    assert_eq!(format_cell_for_sql("hello", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("N/A", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("yes", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("abc123", SqlType::Float, false), "NULL");
}

#[test]
fn test_format_cell_non_finite_float_returns_null() {
    // NaN and Infinity are invalid SQL literals; must return NULL instead
    assert_eq!(format_cell_for_sql("NaN", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("inf", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("-inf", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("infinity", SqlType::Float, false), "NULL");
    assert_eq!(format_cell_for_sql("-infinity", SqlType::Float, false), "NULL");
}

#[test]
fn test_format_cell_non_finite_int_returns_null() {
    // NaN and Infinity must not be silently cast to 0 or i64::MAX
    assert_eq!(format_cell_for_sql("NaN", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("inf", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("-inf", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("infinity", SqlType::Int, false), "NULL");
    assert_eq!(format_cell_for_sql("-infinity", SqlType::Int, false), "NULL");
}

// ─── format_cell_for_sql: bool type ───────────────────────────────────

#[test]
fn test_format_cell_bool_type() {
    assert_eq!(format_cell_for_sql("true", SqlType::Bool, false), "true");
    assert_eq!(format_cell_for_sql("false", SqlType::Bool, false), "false");
    assert_eq!(format_cell_for_sql("1", SqlType::Bool, false), "true");
    assert_eq!(format_cell_for_sql("0", SqlType::Bool, false), "false");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Bool, false), "NULL");
}

#[test]
fn test_format_cell_bool_type_unrecognized_returns_null() {
    assert_eq!(format_cell_for_sql("maybe", SqlType::Bool, false), "NULL");
    assert_eq!(format_cell_for_sql("2", SqlType::Bool, false), "NULL");
}

// ─── format_cell_for_sql: text type ───────────────────────────────────

#[test]
fn test_format_cell_text_type() {
    assert_eq!(format_cell_for_sql("hello", SqlType::Text, false), "'hello'");
    assert_eq!(format_cell_for_sql("it's", SqlType::Text, false), "'it''s'"); // escapes quotes
    assert_eq!(format_cell_for_sql("NULL", SqlType::Text, false), "NULL");
}

// ─── format_cell_for_sql: unknown type ────────────────────────────────

#[test]
fn test_format_cell_unknown_type() {
    assert_eq!(format_cell_for_sql("42", SqlType::Unknown, false), "42");
    assert_eq!(format_cell_for_sql("3.14", SqlType::Unknown, false), "3.14");
    assert_eq!(format_cell_for_sql("true", SqlType::Unknown, false), "true");
    assert_eq!(format_cell_for_sql("false", SqlType::Unknown, false), "false");
    assert_eq!(format_cell_for_sql("hello", SqlType::Unknown, false), "'hello'");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Unknown, false), "NULL");
}

// ─── strip_float_zero_suffix ──────────────────────────────────────────

#[test]
fn test_strip_float_zero_suffix() {
    assert_eq!(strip_float_zero_suffix("42.0"), "42");
    assert_eq!(strip_float_zero_suffix("3.14"), "3.14");
    assert_eq!(strip_float_zero_suffix("hello"), "hello");
    assert_eq!(strip_float_zero_suffix("-1.0"), "-1");
}

// ─── format_value_for_sql ─────────────────────────────────────────────

#[test]
fn test_format_value_for_sql() {
    assert_eq!(format_value_for_sql("hello", false), "'hello'");
    assert_eq!(format_value_for_sql("it's", false), "'it''s'");
    assert_eq!(format_value_for_sql("NULL", false), "NULL");
    // strip_float_suffix strips ".0" but still wraps in quotes (this is a string formatter)
    assert_eq!(format_value_for_sql("42.0", true), "'42'");
    assert_eq!(format_value_for_sql("3.14", true), "'3.14'");
}

// ═══════════════════════════════════════════════════════════════════════
// Performance Tests
// ═══════════════════════════════════════════════════════════════════════

#[test]
fn test_perf_parse_sql_type_enum_approach() {
    use std::time::Instant;

    let type_strings = [
        "INT",
        "BIGINT",
        "DOUBLE",
        "FLOAT",
        "BOOL",
        "BOOLEAN",
        "TEXT",
        "VARCHAR",
        "CHAR",
        "DECIMAL",
        "NUMERIC",
        "REAL",
        "",
        "DATE",
        "TIMESTAMP",
        "SMALLINT",
        "TINYINT",
    ];
    let iterations = 100_000;

    let start = Instant::now();
    for _ in 0..iterations {
        for ts in &type_strings {
            let t = parse_sql_type(ts);
            std::hint::black_box(t);
        }
    }
    let duration = start.elapsed();

    eprintln!(
        "[PERF] test_perf_parse_sql_type_enum_approach | duration={}ms | iterations={} | types={} | per_call={:.3}µs",
        duration.as_millis(),
        iterations,
        type_strings.len(),
        duration.as_secs_f64() * 1_000_000.0 / (iterations as f64 * type_strings.len() as f64),
    );

    assert!(
        duration.as_millis() < 500,
        "parse_sql_type too slow: {:?} for {} iterations",
        duration,
        iterations
    );
}

#[test]
fn test_perf_format_cell_for_sql_enum_dispatch() {
    use std::time::Instant;

    let values = [
        "42", "3.14", "true", "false", "hello", "NULL", "1", "0", "-5", "100.0",
    ];
    let types = [
        SqlType::Int,
        SqlType::Float,
        SqlType::Bool,
        SqlType::Text,
        SqlType::Unknown,
    ];
    let iterations = 50_000;

    let start = Instant::now();
    for _ in 0..iterations {
        for v in &values {
            for t in &types {
                std::hint::black_box(format_cell_for_sql(v, *t, false));
            }
        }
    }
    let duration = start.elapsed();
    let total_calls = iterations * values.len() * types.len();

    eprintln!(
        "[PERF] test_perf_format_cell_for_sql_enum_dispatch | duration={}ms | iterations={} | total_calls={} | per_call={:.3}µs",
        duration.as_millis(),
        iterations,
        total_calls,
        duration.as_secs_f64() * 1_000_000.0 / total_calls as f64,
    );

    assert!(
        duration.as_secs() < 2,
        "format_cell_for_sql hot loop too slow: {:?} for {} iterations",
        duration,
        iterations
    );
}

#[test]
fn test_perf_resolve_export_specs_large_columns() {
    use std::time::Instant;

    let column_count = 500;
    let headers: Vec<String> = (0..column_count).map(|i| format!("col_{}", i)).collect();
    let config: Vec<ExportColumnConfig> = (0..column_count)
        .map(|i| ExportColumnConfig {
            source_column_name: format!("col_{}", i),
            export_column_name: format!("export_col_{}", i),
            sql_type: if i % 3 == 0 {
                "INT".to_string()
            } else if i % 3 == 1 {
                "TEXT".to_string()
            } else {
                "BOOL".to_string()
            },
        })
        .collect();

    let start = Instant::now();
    let result = resolve_export_specs(&headers, Some(&config)).unwrap();
    let duration = start.elapsed();

    assert_eq!(result.len(), column_count);

    eprintln!(
        "[PERF] test_perf_resolve_export_specs_large_columns | duration={}ms | columns={} | per_column={:.3}µs",
        duration.as_millis(),
        column_count,
        duration.as_secs_f64() * 1_000_000.0 / column_count as f64,
    );

    assert!(
        duration.as_millis() < 100,
        "resolve_export_specs too slow for {} columns: {:?}",
        column_count,
        duration
    );
}

// ─── empty_text_as_null ──────────────────────────────────────────────

#[test]
fn test_format_cell_empty_text_as_null_enabled() {
    assert_eq!(format_cell_for_sql("", SqlType::Text, true), "NULL");
    assert_eq!(format_cell_for_sql("", SqlType::Unknown, true), "NULL");
    assert_eq!(format_cell_for_sql("hello", SqlType::Text, true), "'hello'");
    assert_eq!(format_cell_for_sql("42", SqlType::Unknown, true), "42");
}

#[test]
fn test_format_cell_empty_text_as_null_disabled() {
    assert_eq!(format_cell_for_sql("", SqlType::Text, false), "''");
    assert_eq!(format_cell_for_sql("", SqlType::Unknown, false), "''");
}

#[test]
fn test_format_cell_empty_text_as_null_no_effect_on_non_text_types() {
    assert_eq!(format_cell_for_sql("", SqlType::Int, true), "NULL");
    assert_eq!(format_cell_for_sql("", SqlType::Float, true), "NULL");
    assert_eq!(format_cell_for_sql("", SqlType::Bool, true), "NULL");
}

#[test]
fn test_format_cell_null_value_ignores_empty_text_as_null() {
    assert_eq!(format_cell_for_sql("NULL", SqlType::Text, true), "NULL");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Text, false), "NULL");
}

// ─── truncate_record_batches ──────────────────────────────────────────

fn make_int_batch(row_count: i32) -> RecordBatch {
    let schema = Schema::new(vec![Field::new("id", DataType::Int32, false)]);
    let values: Vec<i32> = (0..row_count).collect();
    let array = Int32Array::from(values);
    RecordBatch::try_new(Arc::new(schema), vec![Arc::new(array)]).expect("test batch")
}

#[test]
fn test_truncate_record_batches_keeps_all_when_under_limit() {
    let batches = vec![make_int_batch(3), make_int_batch(2)];
    let truncated = truncate_record_batches(batches, 5);
    assert_eq!(truncated.len(), 2);
    assert_eq!(truncated[0].num_rows(), 3);
    assert_eq!(truncated[1].num_rows(), 2);
}

#[test]
fn test_truncate_record_batches_slices_within_single_batch() {
    let batches = vec![make_int_batch(5)];
    let truncated = truncate_record_batches(batches, 3);
    assert_eq!(truncated.len(), 1);
    assert_eq!(truncated[0].num_rows(), 3);
}

#[test]
fn test_truncate_record_batches_slices_across_batches() {
    let batches = vec![make_int_batch(3), make_int_batch(4)];
    let truncated = truncate_record_batches(batches, 5);
    assert_eq!(truncated.len(), 2);
    assert_eq!(truncated[0].num_rows(), 3);
    assert_eq!(truncated[1].num_rows(), 2);
}

// ═══════════════════════════════════════════════════════════════════════
// End-to-end batch -> SQL pipeline (arrow 58 ArrayFormatter contract)
//
// These lock the SQL literal output produced by the upgraded arrow/datafusion
// formatting code, which was previously asserted nowhere.
// ═══════════════════════════════════════════════════════════════════════

/// 2026-05-26 19:41:00 UTC expressed in nanoseconds since the epoch.
/// Built from chrono so the magic value stays self-documenting.
fn sample_ts_nanos() -> i64 {
    use chrono::{DateTime, NaiveDate, Utc};
    let dt = NaiveDate::from_ymd_opt(2026, 5, 26)
        .expect("valid date")
        .and_hms_opt(19, 41, 0)
        .expect("valid time");
    DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc)
        .timestamp_nanos_opt()
        .expect("timestamp fits in i64")
}

/// The arrow-58 `ArrayFormatter` rendering of `sample_ts_nanos()` for a
/// `Timestamp(Nanosecond, None)` column. Locks the display contract.
const TS_DISPLAY: &str = "2026-05-26T19:41:00";

/// Build a multi-type RecordBatch exercising every arrow array kind the SQL
/// generator can encounter: Int64, Float64, Utf8, Boolean, Timestamp(ns).
/// Row layout (3 rows, with NULLs scattered across columns):
///   id    price  name        active  ts
///   1     9.87   alice       true    <ts>
///   2     NULL   o'brien     false   NULL
///   NULL  2.5    NULL        NULL    <ts>
fn make_multitype_batch() -> RecordBatch {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("price", DataType::Float64, true),
        Field::new("name", DataType::Utf8, true),
        Field::new("active", DataType::Boolean, true),
        Field::new("ts", DataType::Timestamp(TimeUnit::Nanosecond, None), true),
    ]);
    let ts = sample_ts_nanos();
    let id = Int64Array::from(vec![Some(1_i64), Some(2), None]);
    let price = Float64Array::from(vec![Some(9.87_f64), None, Some(2.5)]);
    let name = StringArray::from(vec![Some("alice"), Some("o'brien"), None]);
    let active = BooleanArray::from(vec![Some(true), Some(false), None]);
    let ts_arr = TimestampNanosecondArray::from(vec![Some(ts), None, Some(ts)]);
    RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(id),
            Arc::new(price),
            Arc::new(name),
            Arc::new(active),
            Arc::new(ts_arr),
        ],
    )
    .expect("test batch")
}

fn typed_columns() -> Vec<ExportColumnConfig> {
    vec![
        ExportColumnConfig {
            source_column_name: "id".to_string(),
            export_column_name: "id".to_string(),
            sql_type: "BIGINT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "price".to_string(),
            export_column_name: "price".to_string(),
            sql_type: "DOUBLE".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "name".to_string(),
            export_column_name: "name".to_string(),
            sql_type: "TEXT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "active".to_string(),
            export_column_name: "active".to_string(),
            sql_type: "BOOL".to_string(),
        },
    ]
}

// ─── generate_sql_inserts_from_batches ────────────────────────────────

#[test]
fn test_inserts_empty_batches_returns_empty_string() {
    let sql = generate_sql_inserts_from_batches(
        Vec::new(),
        "t",
        1000,
        &Dialect::MySQL,
        None,
        false,
    )
    .expect("ok");
    assert_eq!(sql, "");
}

#[test]
fn test_inserts_single_row_mysql() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![Some(7_i64)])),
            Arc::new(StringArray::from(vec![Some("bob")])),
        ],
    )
    .expect("batch");

    let sql =
        generate_sql_inserts_from_batches(vec![batch], "users", 1000, &Dialect::MySQL, None, false)
            .expect("ok");
    assert_eq!(
        sql,
        "INSERT INTO `users` (`id`, `name`) VALUES\n(7, 'bob');"
    );
}

#[test]
fn test_inserts_multitype_mysql_backticks_nulls_and_escaping() {
    let sql = generate_sql_inserts_from_batches(
        vec![make_multitype_batch()],
        "t",
        1000,
        &Dialect::MySQL,
        Some(&typed_columns()),
        false,
    )
    .expect("ok");

    let expected = "INSERT INTO `t` (`id`, `price`, `name`, `active`) VALUES\n\
         (1, 9.87, 'alice', true),\n\
         (2, NULL, 'o''brien', false),\n\
         (NULL, 2.5, NULL, NULL);";
    assert_eq!(sql, expected);
}

#[test]
fn test_inserts_multitype_postgres_double_quotes() {
    let sql = generate_sql_inserts_from_batches(
        vec![make_multitype_batch()],
        "t",
        1000,
        &Dialect::PostgreSQL,
        Some(&typed_columns()),
        false,
    )
    .expect("ok");

    let expected = "INSERT INTO \"t\" (\"id\", \"price\", \"name\", \"active\") VALUES\n\
         (1, 9.87, 'alice', true),\n\
         (2, NULL, 'o''brien', false),\n\
         (NULL, 2.5, NULL, NULL);";
    assert_eq!(sql, expected);
}

#[test]
fn test_inserts_timestamp_literal_contract() {
    // No export config => Unknown type => timestamp rendered as quoted string.
    // Locks the arrow-58 timestamp display format.
    let schema = Schema::new(vec![Field::new(
        "ts",
        DataType::Timestamp(TimeUnit::Nanosecond, None),
        true,
    )]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(TimestampNanosecondArray::from(vec![Some(
            sample_ts_nanos(),
        )]))],
    )
    .expect("batch");

    let sql =
        generate_sql_inserts_from_batches(vec![batch], "t", 1000, &Dialect::MySQL, None, false)
            .expect("ok");
    assert_eq!(
        sql,
        format!("INSERT INTO `t` (`ts`) VALUES\n('{}');", TS_DISPLAY)
    );
}

// ─── max_values_per_insert batching ───────────────────────────────────

#[test]
fn test_inserts_batching_splits_into_multiple_statements() {
    // 5 rows, limit 2 => 3 INSERT statements (2 + 2 + 1).
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(Int64Array::from(vec![1_i64, 2, 3, 4, 5]))],
    )
    .expect("batch");

    let sql =
        generate_sql_inserts_from_batches(vec![batch], "t", 2, &Dialect::MySQL, None, false)
            .expect("ok");

    let statement_count = sql.matches("INSERT INTO").count();
    assert_eq!(statement_count, 3, "expected 3 statements, got: {}", sql);
    assert_eq!(
        sql,
        "INSERT INTO `t` (`id`) VALUES\n(1),\n(2);\n\
         INSERT INTO `t` (`id`) VALUES\n(3),\n(4);\n\
         INSERT INTO `t` (`id`) VALUES\n(5);"
    );
}

#[test]
fn test_inserts_batching_limit_zero_floors_to_one() {
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(Int64Array::from(vec![1_i64, 2]))],
    )
    .expect("batch");

    let sql =
        generate_sql_inserts_from_batches(vec![batch], "t", 0, &Dialect::MySQL, None, false)
            .expect("ok");
    // limit 0 floors to 1 => one statement per row
    assert_eq!(sql.matches("INSERT INTO").count(), 2);
    assert_eq!(
        sql,
        "INSERT INTO `t` (`id`) VALUES\n(1);\nINSERT INTO `t` (`id`) VALUES\n(2);"
    );
}

// ─── export column subset + rename ────────────────────────────────────

#[test]
fn test_inserts_column_subset_and_rename() {
    let config = vec![
        ExportColumnConfig {
            source_column_name: "name".to_string(),
            export_column_name: "full_name".to_string(),
            sql_type: "TEXT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "id".to_string(),
            export_column_name: "user_id".to_string(),
            sql_type: "BIGINT".to_string(),
        },
    ];
    let sql = generate_sql_inserts_from_batches(
        vec![make_multitype_batch()],
        "t",
        1000,
        &Dialect::MySQL,
        Some(&config),
        false,
    )
    .expect("ok");

    // Only the two selected columns appear, in config order, with renamed headers.
    let expected = "INSERT INTO `t` (`full_name`, `user_id`) VALUES\n\
         ('alice', 1),\n\
         ('o''brien', 2),\n\
         (NULL, NULL);";
    assert_eq!(sql, expected);
}

// ─── empty_text_as_null through the full batch pipeline ───────────────

#[test]
fn test_inserts_empty_text_as_null_toggle() {
    let schema = Schema::new(vec![Field::new("note", DataType::Utf8, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(StringArray::from(vec![Some(""), Some("hi")]))],
    )
    .expect("batch");
    let config = vec![ExportColumnConfig {
        source_column_name: "note".to_string(),
        export_column_name: "note".to_string(),
        sql_type: "TEXT".to_string(),
    }];

    let sql_null = generate_sql_inserts_from_batches(
        vec![batch.clone()],
        "t",
        1000,
        &Dialect::MySQL,
        Some(&config),
        true,
    )
    .expect("ok");
    assert_eq!(
        sql_null,
        "INSERT INTO `t` (`note`) VALUES\n(NULL),\n('hi');"
    );

    let sql_keep = generate_sql_inserts_from_batches(
        vec![batch],
        "t",
        1000,
        &Dialect::MySQL,
        Some(&config),
        false,
    )
    .expect("ok");
    assert_eq!(sql_keep, "INSERT INTO `t` (`note`) VALUES\n(''),\n('hi');");
}

// ─── generate_sql_update_from_batches ─────────────────────────────────

#[test]
fn test_update_empty_batches_returns_empty_string() {
    let sql =
        generate_sql_update_from_batches(Vec::new(), "t", &["id"], &Dialect::MySQL, None, false)
            .expect("ok");
    assert_eq!(sql, "");
}

#[test]
fn test_update_one_statement_per_row_set_excludes_where() {
    let sql = generate_sql_update_from_batches(
        vec![make_multitype_batch()],
        "t",
        &["id"],
        &Dialect::MySQL,
        Some(&typed_columns()),
        false,
    )
    .expect("ok");

    // One UPDATE per row; SET lists the non-WHERE exported columns; WHERE uses id.
    let expected = "UPDATE `t` SET `price` = 9.87, `name` = 'alice', `active` = true WHERE `id` = 1;\n\
         UPDATE `t` SET `price` = NULL, `name` = 'o''brien', `active` = false WHERE `id` = 2;\n\
         UPDATE `t` SET `price` = 2.5, `name` = NULL, `active` = NULL WHERE `id` = NULL;\n";
    assert_eq!(sql, expected);
    assert_eq!(sql.matches("UPDATE `t`").count(), 3);
}

#[test]
fn test_update_postgres_dialect() {
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![Some(1_i64)])),
            Arc::new(StringArray::from(vec![Some("alice")])),
        ],
    )
    .expect("batch");

    let sql = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["id"],
        &Dialect::PostgreSQL,
        None,
        false,
    )
    .expect("ok");
    assert_eq!(sql, "UPDATE \"t\" SET \"name\" = 'alice' WHERE \"id\" = 1;\n");
}

#[test]
fn test_update_where_column_renamed() {
    let config = vec![
        ExportColumnConfig {
            source_column_name: "id".to_string(),
            export_column_name: "user_id".to_string(),
            sql_type: "BIGINT".to_string(),
        },
        ExportColumnConfig {
            source_column_name: "name".to_string(),
            export_column_name: "full_name".to_string(),
            sql_type: "TEXT".to_string(),
        },
    ];
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![Some(1_i64)])),
            Arc::new(StringArray::from(vec![Some("alice")])),
        ],
    )
    .expect("batch");

    let sql = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["id"],
        &Dialect::MySQL,
        Some(&config),
        false,
    )
    .expect("ok");
    // WHERE uses the renamed export name; SET excludes the WHERE column.
    assert_eq!(
        sql,
        "UPDATE `t` SET `full_name` = 'alice' WHERE `user_id` = 1;\n"
    );
}

#[test]
fn test_update_where_column_not_in_export_used_for_value_only() {
    // Export only `name`; WHERE on `id` which is not in the export subset.
    // `id` must still be available for the WHERE clause but not appear in SET.
    let config = vec![ExportColumnConfig {
        source_column_name: "name".to_string(),
        export_column_name: "name".to_string(),
        sql_type: "TEXT".to_string(),
    }];
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![Some(9_i64)])),
            Arc::new(StringArray::from(vec![Some("zoe")])),
        ],
    )
    .expect("batch");

    let sql = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["id"],
        &Dialect::MySQL,
        Some(&config),
        false,
    )
    .expect("ok");
    // The WHERE column is absent from the export config, so its SQL type defaults
    // to Text (`unwrap_or(SqlType::Text)`); the numeric key is therefore quoted.
    assert_eq!(sql, "UPDATE `t` SET `name` = 'zoe' WHERE `id` = '9';\n");
}

#[test]
fn test_update_missing_where_column_errors() {
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(Int64Array::from(vec![Some(1_i64)]))],
    )
    .expect("batch");

    let result = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["nonexistent"],
        &Dialect::MySQL,
        None,
        false,
    );
    assert!(result.is_err());
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("WHERE column"),
        "expected WHERE column error, got: {}",
        msg
    );
}

#[test]
fn test_update_multiple_where_columns_and_joined() {
    let schema = Schema::new(vec![
        Field::new("tenant_id", DataType::Int64, true),
        Field::new("user_id", DataType::Int64, true),
        Field::new("name", DataType::Utf8, true),
    ]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![
            Arc::new(Int64Array::from(vec![Some(1_i64)])),
            Arc::new(Int64Array::from(vec![Some(42_i64)])),
            Arc::new(StringArray::from(vec![Some("alice")])),
        ],
    )
    .expect("batch");

    let sql = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["tenant_id", "user_id"],
        &Dialect::MySQL,
        None,
        false,
    )
    .expect("ok");
    assert_eq!(
        sql,
        "UPDATE `t` SET `name` = 'alice' WHERE `tenant_id` = 1 AND `user_id` = 42;\n"
    );
}

#[test]
fn test_update_multiple_where_columns_excluded_from_set() {
    let sql = generate_sql_update_from_batches(
        vec![make_multitype_batch()],
        "t",
        &["id", "name"],
        &Dialect::MySQL,
        Some(&typed_columns()),
        false,
    )
    .expect("ok");
    assert_eq!(
        sql,
        "UPDATE `t` SET `price` = 9.87, `active` = true WHERE `id` = 1 AND `name` = 'alice';\n\
         UPDATE `t` SET `price` = NULL, `active` = false WHERE `id` = 2 AND `name` = 'o''brien';\n\
         UPDATE `t` SET `price` = 2.5, `active` = NULL WHERE `id` = NULL AND `name` = NULL;\n"
    );
}

#[test]
fn test_update_empty_where_columns_errors() {
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(Int64Array::from(vec![Some(1_i64)]))],
    )
    .expect("batch");

    let result = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &[],
        &Dialect::MySQL,
        None,
        false,
    );
    assert!(result.is_err());
}

#[test]
fn test_update_duplicate_where_columns_errors() {
    let schema = Schema::new(vec![Field::new("id", DataType::Int64, true)]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(Int64Array::from(vec![Some(1_i64)]))],
    )
    .expect("batch");

    let result = generate_sql_update_from_batches(
        vec![batch],
        "t",
        &["id", "id"],
        &Dialect::MySQL,
        None,
        false,
    );
    assert!(result.is_err());
    let msg = format!("{:?}", result.unwrap_err());
    assert!(
        msg.contains("Duplicate WHERE column"),
        "expected duplicate WHERE column error, got: {}",
        msg
    );
}

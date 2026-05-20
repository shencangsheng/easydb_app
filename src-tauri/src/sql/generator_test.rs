use super::generator::{
    format_bool_for_sql, format_cell_for_sql, format_value_for_sql, parse_sql_type,
    resolve_export_specs, strip_float_zero_suffix, ExportColumnConfig, SqlType,
};

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
    assert_eq!(format_cell_for_sql("true", SqlType::Int), "1");
    assert_eq!(format_cell_for_sql("false", SqlType::Int), "0");
}

#[test]
fn test_format_cell_bool_to_float() {
    assert_eq!(format_cell_for_sql("true", SqlType::Float), "1");
    assert_eq!(format_cell_for_sql("false", SqlType::Float), "0");
}

// ─── format_cell_for_sql: integer type ────────────────────────────────

#[test]
fn test_format_cell_int_type() {
    assert_eq!(format_cell_for_sql("42", SqlType::Int), "42");
    assert_eq!(format_cell_for_sql("-5", SqlType::Int), "-5");
    assert_eq!(format_cell_for_sql("3.7", SqlType::Int), "3"); // truncates float
    assert_eq!(format_cell_for_sql("NULL", SqlType::Int), "NULL");
}

#[test]
fn test_format_cell_int_large_unsigned() {
    assert_eq!(
        format_cell_for_sql("18446744073709551615", SqlType::Int),
        "18446744073709551615"
    );
}

#[test]
fn test_format_cell_int_non_numeric_returns_null() {
    // Regression: previously produced quoted string like 'hello' which is invalid SQL for INT columns
    assert_eq!(format_cell_for_sql("hello", SqlType::Int), "NULL");
    assert_eq!(format_cell_for_sql("N/A", SqlType::Int), "NULL");
    assert_eq!(format_cell_for_sql("yes", SqlType::Int), "NULL");
    assert_eq!(format_cell_for_sql("abc123", SqlType::Int), "NULL");
}

// ─── format_cell_for_sql: float type ──────────────────────────────────

#[test]
fn test_format_cell_float_type() {
    assert_eq!(format_cell_for_sql("3.14", SqlType::Float), "3.14");
    assert_eq!(format_cell_for_sql("42", SqlType::Float), "42");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Float), "NULL");
}

#[test]
fn test_format_cell_float_non_numeric_returns_null() {
    // Regression: previously produced quoted string like 'hello' which is invalid SQL for FLOAT columns
    assert_eq!(format_cell_for_sql("hello", SqlType::Float), "NULL");
    assert_eq!(format_cell_for_sql("N/A", SqlType::Float), "NULL");
    assert_eq!(format_cell_for_sql("yes", SqlType::Float), "NULL");
    assert_eq!(format_cell_for_sql("abc123", SqlType::Float), "NULL");
}

// ─── format_cell_for_sql: bool type ───────────────────────────────────

#[test]
fn test_format_cell_bool_type() {
    assert_eq!(format_cell_for_sql("true", SqlType::Bool), "true");
    assert_eq!(format_cell_for_sql("false", SqlType::Bool), "false");
    assert_eq!(format_cell_for_sql("1", SqlType::Bool), "true");
    assert_eq!(format_cell_for_sql("0", SqlType::Bool), "false");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Bool), "NULL");
}

#[test]
fn test_format_cell_bool_type_unrecognized_returns_null() {
    assert_eq!(format_cell_for_sql("maybe", SqlType::Bool), "NULL");
    assert_eq!(format_cell_for_sql("2", SqlType::Bool), "NULL");
}

// ─── format_cell_for_sql: text type ───────────────────────────────────

#[test]
fn test_format_cell_text_type() {
    assert_eq!(format_cell_for_sql("hello", SqlType::Text), "'hello'");
    assert_eq!(format_cell_for_sql("it's", SqlType::Text), "'it''s'"); // escapes quotes
    assert_eq!(format_cell_for_sql("NULL", SqlType::Text), "NULL");
}

// ─── format_cell_for_sql: unknown type ────────────────────────────────

#[test]
fn test_format_cell_unknown_type() {
    assert_eq!(format_cell_for_sql("42", SqlType::Unknown), "42");
    assert_eq!(format_cell_for_sql("3.14", SqlType::Unknown), "3.14");
    assert_eq!(format_cell_for_sql("true", SqlType::Unknown), "true");
    assert_eq!(format_cell_for_sql("false", SqlType::Unknown), "false");
    assert_eq!(format_cell_for_sql("hello", SqlType::Unknown), "'hello'");
    assert_eq!(format_cell_for_sql("NULL", SqlType::Unknown), "NULL");
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
                std::hint::black_box(format_cell_for_sql(v, *t));
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

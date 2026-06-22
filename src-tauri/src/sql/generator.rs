use crate::commands::query::Dialect;
use crate::context::schema::AppResult;
use datafusion::arrow::error::ArrowError;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::display::{ArrayFormatter, FormatOptions};
use datafusion::dataframe::DataFrame;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ExportColumnConfig {
    pub source_column_name: String,
    pub export_column_name: String,
    pub sql_type: String,
}

/// Pre-parsed SQL type category to avoid repeated string allocations in the hot formatting loop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SqlType {
    Bool,
    Int,
    Float,
    Text,
    Unknown,
}

pub(crate) fn parse_sql_type(sql_type: &str) -> SqlType {
    let upper = sql_type.to_uppercase();
    if upper.starts_with("BOOL") {
        SqlType::Bool
    } else if upper.starts_with("INT")
        || upper.starts_with("BIGINT")
        || upper.starts_with("SMALLINT")
        || upper.starts_with("TINYINT")
    {
        SqlType::Int
    } else if upper.starts_with("DOUBLE") || upper.starts_with("FLOAT") || upper.starts_with("DECIMAL") || upper.starts_with("NUMERIC") || upper.starts_with("REAL") {
        SqlType::Float
    } else if upper.starts_with("TEXT") || upper.starts_with("CHAR") || upper.starts_with("VARCHAR") || upper.starts_with("STR") {
        SqlType::Text
    } else {
        SqlType::Unknown
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ColumnExportSpec {
    pub(crate) source_index: usize,
    pub(crate) export_name: String,
    pub(crate) sql_type: SqlType,
}

pub(crate) fn resolve_export_specs(
    headers: &[String],
    export_columns: Option<&[ExportColumnConfig]>,
) -> AppResult<Vec<ColumnExportSpec>> {
    let Some(export_columns) = export_columns else {
        return Ok(headers
            .iter()
            .enumerate()
            .map(|(idx, name)| ColumnExportSpec {
                source_index: idx,
                export_name: name.clone(),
                sql_type: SqlType::Unknown,
            })
            .collect());
    };

    if export_columns.is_empty() {
        return Err(crate::context::error::AppError::BadRequest {
            message: "At least one column must be selected for export".to_string(),
        });
    }

    let header_index: std::collections::HashMap<&str, usize> = headers
        .iter()
        .enumerate()
        .map(|(idx, name)| (name.as_str(), idx))
        .collect();

    let mut seen_names = std::collections::HashSet::new();
    let mut specs = Vec::with_capacity(export_columns.len());
    for col in export_columns {
        let export_name = col.export_column_name.trim();
        if export_name.is_empty() {
            return Err(crate::context::error::AppError::BadRequest {
                message: format!(
                    "Export column name cannot be empty for source column '{}'",
                    col.source_column_name
                ),
            });
        }

        if seen_names.contains(export_name) {
            return Err(crate::context::error::AppError::BadRequest {
                message: format!(
                    "Duplicate export column name '{}'. Each export column must have a unique name.",
                    export_name
                ),
            });
        }
        seen_names.insert(export_name.to_string());

        let source_index = header_index
            .get(col.source_column_name.as_str())
            .copied()
            .ok_or_else(|| crate::context::error::AppError::BadRequest {
                message: format!(
                    "Source column '{}' not found in query result",
                    col.source_column_name
                ),
            })?;

        specs.push(ColumnExportSpec {
            source_index,
            export_name: export_name.to_string(),
            sql_type: parse_sql_type(&col.sql_type),
        });
    }

    Ok(specs)
}

pub(crate) fn format_cell_for_sql(
    formatted_value: &str,
    col_type: SqlType,
    empty_text_as_null: bool,
) -> String {
    if formatted_value == "NULL" {
        return "NULL".to_string();
    }

    // When empty_text_as_null is enabled, treat empty strings as NULL for text-like types
    if empty_text_as_null && formatted_value.is_empty() {
        match col_type {
            SqlType::Text | SqlType::Unknown => return "NULL".to_string(),
            _ => {}
        }
    }

    if col_type == SqlType::Unknown {
        if formatted_value.parse::<i64>().is_ok()
            || formatted_value.parse::<f64>().is_ok()
            || formatted_value == "true"
            || formatted_value == "false"
        {
            return strip_float_zero_suffix(formatted_value);
        }
        return format_value_for_sql(formatted_value, true);
    }

    if col_type == SqlType::Bool {
        format_bool_for_sql(formatted_value)
    } else if col_type == SqlType::Int || col_type == SqlType::Float {
        if formatted_value == "true" {
            "1".to_string()
        } else if formatted_value == "false" {
            "0".to_string()
        } else if let Ok(i) = formatted_value.parse::<i64>() {
            i.to_string()
        } else if let Ok(u) = formatted_value.parse::<u64>() {
            u.to_string()
        } else if let Ok(f) = formatted_value.parse::<f64>() {
            if f.is_finite() {
                if col_type == SqlType::Int {
                    format!("{}", f as i64)
                } else {
                    formatted_value.to_string()
                }
            } else {
                "NULL".to_string()
            }
        } else {
            "NULL".to_string()
        }
    } else {
        format_value_for_sql(formatted_value, true)
    }
}

/// Strip trailing ".0" from float-formatted values that are actually integers
pub(crate) fn strip_float_zero_suffix(value: &str) -> String {
    if value.ends_with(".0") {
        // Check that the part before ".0" is a valid integer
        let integer_part = &value[..value.len() - 2];
        if integer_part.parse::<i64>().is_ok() || integer_part.parse::<u64>().is_ok() {
            return integer_part.to_string();
        }
    }
    value.to_string()
}

/// Format a value as a SQL boolean literal (true/false, unquoted)
pub(crate) fn format_bool_for_sql(value: &str) -> String {
    if value == "NULL" {
        "NULL".to_string()
    } else {
        match value.to_ascii_lowercase().as_str() {
            "true" | "1" => "true".to_string(),
            "false" | "0" => "false".to_string(),
            _ => "NULL".to_string(),
        }
    }
}

/// Helper function to format a value from Arrow Array for SQL
/// When strip_float_suffix is true, removes trailing ".0" from float values
pub(crate) fn format_value_for_sql(value: &str, strip_float_suffix: bool) -> String {
    if value == "NULL" {
        "NULL".to_string()
    } else {
        let display_value = if strip_float_suffix {
            strip_float_zero_suffix(value)
        } else {
            value.to_string()
        };
        // Escape single quotes and wrap in quotes for string values
        format!("'{}'", display_value.replace("'", "''"))
    }
}

/// Extract rows from RecordBatch, optionally filtering/renaming columns via export specs.
fn extract_rows_from_batches<F>(
    batches: Vec<RecordBatch>,
    export_specs: &[ColumnExportSpec],
    empty_text_as_null: bool,
    mut on_row: F,
) -> AppResult<()>
where
    F: FnMut(Vec<String>) -> AppResult<()>,
{
    if batches.is_empty() {
        return Ok(());
    }

    let options = FormatOptions::default().with_null("NULL");

    for batch in batches {
        let formatters = batch
            .columns()
            .iter()
            .map(|c| ArrayFormatter::try_new(c.as_ref(), &options))
            .collect::<Result<Vec<_>, ArrowError>>()
            .map_err(|e| crate::context::error::AppError::BadRequest {
                message: format!("Failed to create formatter: {}", e),
            })?;

        for row_idx in 0..batch.num_rows() {
            let mut cells = Vec::with_capacity(export_specs.len());
            for spec in export_specs {
                let formatted_value = formatters[spec.source_index]
                    .value(row_idx)
                    .to_string();
                cells.push(format_cell_for_sql(&formatted_value, spec.sql_type, empty_text_as_null));
            }
            on_row(cells)?;
        }
    }

    Ok(())
}

/// Helper function to extract column headers from the provided RecordBatches
fn extract_headers_from_batches(batches: &[RecordBatch]) -> Vec<String> {
    batches
        .first()
        .map(|batch| {
            batch
                .schema()
                .fields()
                .iter()
                .map(|f| f.name().to_string())
                .collect()
        })
        .unwrap_or_default()
}

/// Keep at most `max_rows` across one or more RecordBatches.
pub fn truncate_record_batches(batches: Vec<RecordBatch>, max_rows: usize) -> Vec<RecordBatch> {
    let mut remaining = max_rows;
    let mut result = Vec::new();
    for batch in batches {
        if remaining == 0 {
            break;
        }
        let rows = batch.num_rows();
        if rows <= remaining {
            result.push(batch);
            remaining -= rows;
        } else {
            result.push(batch.slice(0, remaining));
            break;
        }
    }
    result
}

/// Generate SQL insert statements from collected RecordBatches.
pub fn generate_sql_inserts_from_batches(
    batches: Vec<RecordBatch>,
    table_name: &str,
    max_values_per_insert: usize,
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
    empty_text_as_null: bool,
) -> AppResult<String> {
    let headers = extract_headers_from_batches(&batches);
    if headers.is_empty() {
        return Ok(String::new());
    }

    let export_specs = resolve_export_specs(&headers, export_columns)?;

    let mut sql_statements = String::new();
    let columns = export_specs
        .iter()
        .map(|spec| match db_dialect {
            Dialect::MySQL => format!("`{}`", spec.export_name),
            Dialect::PostgreSQL => format!("\"{}\"", spec.export_name),
        })
        .collect::<Vec<String>>()
        .join(", ");
    let insert_header_template = match db_dialect {
        Dialect::MySQL => format!("INSERT INTO `{}` ({}) VALUES\n", table_name, columns),
        Dialect::PostgreSQL => format!("INSERT INTO \"{}\" ({}) VALUES\n", table_name, columns),
    };
    let chunk_limit = max_values_per_insert.max(1);
    let mut pending_rows: Vec<Vec<String>> = Vec::with_capacity(chunk_limit);
    let mut is_first_insert = true;

    let mut flush_chunk = |rows: &mut Vec<Vec<String>>| -> AppResult<()> {
        if rows.is_empty() {
            return Ok(());
        }

        if !is_first_insert {
            sql_statements.push('\n');
        } else {
            is_first_insert = false;
        }

        sql_statements.push_str(&insert_header_template);

        let values_clauses = rows
            .iter()
            .map(|row| format!("({})", row.join(", ")))
            .collect::<Vec<String>>()
            .join(",\n");

        sql_statements.push_str(&values_clauses);
        sql_statements.push(';');
        rows.clear();
        Ok(())
    };

    extract_rows_from_batches(batches, &export_specs, empty_text_as_null, |row| {
        pending_rows.push(row);
        if pending_rows.len() == chunk_limit {
            flush_chunk(&mut pending_rows)?;
        }
        Ok(())
    })?;
    flush_chunk(&mut pending_rows)?;

    Ok(sql_statements)
}

/// Generate SQL insert statements from DataFrame
/// Splits large datasets into multiple INSERT statements for better performance
pub async fn generate_sql_inserts(
    df: DataFrame,
    table_name: &str,
    max_values_per_insert: usize,
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
    empty_text_as_null: bool,
) -> AppResult<String> {
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;
    generate_sql_inserts_from_batches(
        batches,
        table_name,
        max_values_per_insert,
        db_dialect,
        export_columns,
        empty_text_as_null,
    )
}

struct WhereColumnSpec {
    source_index: usize,
    export_name: String,
    sql_type: SqlType,
}

fn resolve_where_column_specs(
    headers: &[String],
    export_specs: &[ColumnExportSpec],
    export_columns: Option<&[ExportColumnConfig]>,
    where_columns: &[&str],
) -> AppResult<Vec<WhereColumnSpec>> {
    if where_columns.is_empty() {
        return Err(crate::context::error::AppError::BadRequest {
            message: "At least one WHERE column is required for UPDATE statements".to_string(),
        });
    }

    let mut seen = std::collections::HashSet::new();
    let mut specs = Vec::with_capacity(where_columns.len());

    for where_column in where_columns {
        if !seen.insert(*where_column) {
            return Err(crate::context::error::AppError::BadRequest {
                message: format!("Duplicate WHERE column '{}'", where_column),
            });
        }

        let source_index = headers
            .iter()
            .position(|h| h == where_column)
            .ok_or_else(|| crate::context::error::AppError::BadRequest {
                message: format!("WHERE column '{}' not found in data", where_column),
            })?;

        let sql_type = export_columns
            .and_then(|cols| {
                cols.iter()
                    .find(|c| c.source_column_name == *where_column)
                    .map(|c| parse_sql_type(&c.sql_type))
            })
            .unwrap_or(SqlType::Text);

        let export_name = export_specs
            .iter()
            .find(|spec| spec.source_index == source_index)
            .map(|spec| spec.export_name.clone())
            .unwrap_or_else(|| where_column.to_string());

        specs.push(WhereColumnSpec {
            source_index,
            export_name,
            sql_type,
        });
    }

    Ok(specs)
}

/// Generate SQL UPDATE statements from collected RecordBatches.
pub fn generate_sql_update_from_batches(
    batches: Vec<RecordBatch>,
    table_name: &str,
    where_columns: &[&str],
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
    empty_text_as_null: bool,
) -> AppResult<String> {
    let headers = extract_headers_from_batches(&batches);

    if headers.is_empty() {
        return Ok(String::new());
    }

    let export_specs = resolve_export_specs(&headers, export_columns)?;
    let where_specs = resolve_where_column_specs(
        &headers,
        &export_specs,
        export_columns,
        where_columns,
    )?;

    let where_source_indices: std::collections::HashSet<usize> =
        where_specs.iter().map(|spec| spec.source_index).collect();

    let mut extract_specs = export_specs.clone();
    for where_spec in &where_specs {
        if !extract_specs
            .iter()
            .any(|s| s.source_index == where_spec.source_index)
        {
            extract_specs.push(ColumnExportSpec {
                source_index: where_spec.source_index,
                export_name: where_spec.export_name.clone(),
                sql_type: where_spec.sql_type,
            });
        }
    }

    let exported_source_indices: std::collections::HashSet<usize> =
        export_specs.iter().map(|s| s.source_index).collect();

    let mut sql_statements = String::new();

    extract_rows_from_batches(batches, &extract_specs, empty_text_as_null, |row| {
        let mut set_clauses = Vec::new();
        let mut values_by_index = std::collections::HashMap::new();

        for (spec, value) in extract_specs.iter().zip(row.iter()) {
            values_by_index.insert(spec.source_index, value.clone());
            if exported_source_indices.contains(&spec.source_index)
                && !where_source_indices.contains(&spec.source_index)
            {
                let export_name = &spec.export_name;
                set_clauses.push(match db_dialect {
                    Dialect::MySQL => format!("`{}` = {}", export_name, value),
                    Dialect::PostgreSQL => format!("\"{}\" = {}", export_name, value),
                });
            }
        }

        if set_clauses.is_empty() {
            return Ok(());
        }

        let mut where_clauses = Vec::with_capacity(where_specs.len());
        for where_spec in &where_specs {
            let Some(value) = values_by_index.get(&where_spec.source_index) else {
                return Ok(());
            };
            where_clauses.push(match db_dialect {
                Dialect::MySQL => format!("`{}` = {}", where_spec.export_name, value),
                Dialect::PostgreSQL => format!("\"{}\" = {}", where_spec.export_name, value),
            });
        }

        let where_clause = where_clauses.join(" AND ");
        let update_statement = match db_dialect {
            Dialect::MySQL => format!(
                "UPDATE `{}` SET {} WHERE {};\n",
                table_name,
                set_clauses.join(", "),
                where_clause
            ),
            Dialect::PostgreSQL => format!(
                "UPDATE \"{}\" SET {} WHERE {};\n",
                table_name,
                set_clauses.join(", "),
                where_clause
            ),
        };
        sql_statements.push_str(&update_statement);

        Ok(())
    })?;

    Ok(sql_statements)
}

/// Generate SQL UPDATE statements from DataFrame
/// Creates UPDATE statements with WHERE conditions based on the specified columns
pub async fn generate_sql_update(
    df: DataFrame,
    table_name: &str,
    where_columns: &[&str],
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
    empty_text_as_null: bool,
) -> AppResult<String> {
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;
    generate_sql_update_from_batches(
        batches,
        table_name,
        where_columns,
        db_dialect,
        export_columns,
        empty_text_as_null,
    )
}

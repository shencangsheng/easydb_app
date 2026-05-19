use crate::commands::query::{is_sql_bool_type, is_sql_numeric_type, Dialect};
use crate::context::schema::AppResult;
use datafusion::arrow::error::ArrowError;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::display::{ArrayFormatter, FormatOptions};
use datafusion::dataframe::DataFrame;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ExportColumnConfig {
    pub source_column_name: String,
    pub export_column_name: String,
    pub sql_type: String,
}

#[derive(Clone)]
struct ColumnExportSpec {
    source_index: usize,
    export_name: String,
    sql_type: String,
}

fn resolve_export_specs(
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
                sql_type: String::new(),
            })
            .collect());
    };

    if export_columns.is_empty() {
        return Err(crate::context::error::AppError::BadRequest {
            message: "At least one column must be selected for export".to_string(),
        });
    }

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

        let source_index = headers
            .iter()
            .position(|h| h == &col.source_column_name)
            .ok_or_else(|| crate::context::error::AppError::BadRequest {
                message: format!(
                    "Source column '{}' not found in query result",
                    col.source_column_name
                ),
            })?;

        specs.push(ColumnExportSpec {
            source_index,
            export_name: export_name.to_string(),
            sql_type: col.sql_type.clone(),
        });
    }

    Ok(specs)
}

fn format_cell_for_sql(formatted_value: &str, col_type: &str) -> String {
    if formatted_value == "NULL" {
        return "NULL".to_string();
    }

    if col_type.is_empty() {
        if formatted_value.parse::<i64>().is_ok()
            || formatted_value.parse::<f64>().is_ok()
            || formatted_value == "true"
            || formatted_value == "false"
        {
            return strip_float_zero_suffix(formatted_value);
        }
        return format_value_for_sql(formatted_value, true);
    }

    if is_sql_bool_type(col_type) {
        format_bool_for_sql(formatted_value)
    } else if is_sql_numeric_type(col_type) {
        if formatted_value == "true" || formatted_value == "false" {
            formatted_value.to_string()
        } else if col_type.to_uppercase().starts_with("INT") {
            if formatted_value.parse::<i64>().is_ok()
                || formatted_value.parse::<u64>().is_ok()
            {
                formatted_value.to_string()
            } else if let Ok(f) = formatted_value.parse::<f64>() {
                format!("{}", f as i64)
            } else {
                format_value_for_sql(formatted_value, false)
            }
        } else if formatted_value.parse::<i64>().is_ok() || formatted_value.parse::<f64>().is_ok() {
            formatted_value.to_string()
        } else {
            format_value_for_sql(formatted_value, false)
        }
    } else {
        format_value_for_sql(formatted_value, true)
    }
}

/// Strip trailing ".0" from float-formatted values that are actually integers
fn strip_float_zero_suffix(value: &str) -> String {
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
fn format_bool_for_sql(value: &str) -> String {
    if value == "NULL" {
        "NULL".to_string()
    } else {
        match value.to_ascii_lowercase().as_str() {
            "true" | "1" => "true".to_string(),
            "false" | "0" => "false".to_string(),
            _ => format_value_for_sql(value, false),
        }
    }
}

/// Helper function to format a value from Arrow Array for SQL
/// When strip_float_suffix is true, removes trailing ".0" from float values
fn format_value_for_sql(value: &str, strip_float_suffix: bool) -> String {
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
                cells.push(format_cell_for_sql(&formatted_value, &spec.sql_type));
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

/// Generate SQL insert statements from DataFrame
/// Splits large datasets into multiple INSERT statements for better performance
pub async fn generate_sql_inserts(
    df: DataFrame,
    table_name: &str,
    max_values_per_insert: usize,
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
) -> AppResult<String> {
    // Collect RecordBatches from DataFrame
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;

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

    extract_rows_from_batches(batches, &export_specs, |row| {
        pending_rows.push(row);
        if pending_rows.len() == chunk_limit {
            flush_chunk(&mut pending_rows)?;
        }
        Ok(())
    })?;
    flush_chunk(&mut pending_rows)?;

    Ok(sql_statements)
}

/// Generate SQL UPDATE statements from DataFrame
/// Creates UPDATE statements with WHERE conditions based on the specified column
pub async fn generate_sql_update(
    df: DataFrame,
    table_name: &str,
    where_column: &str,
    db_dialect: &Dialect,
    export_columns: Option<&[ExportColumnConfig]>,
) -> AppResult<String> {
    // Collect RecordBatches from DataFrame
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;

    let headers = extract_headers_from_batches(&batches);

    if headers.is_empty() {
        return Ok(String::new());
    }

    // Validate that the WHERE column exists
    if !headers.contains(&where_column.to_string()) {
        return Err(crate::context::error::AppError::BadRequest {
            message: format!("WHERE column '{}' not found in data", where_column),
        });
    }

    let where_column_index = headers
        .iter()
        .position(|h| h == where_column)
        .ok_or_else(|| crate::context::error::AppError::BadRequest {
            message: format!("WHERE column '{}' not found", where_column),
        })?;

    let export_specs = resolve_export_specs(&headers, export_columns)?;

    let where_sql_type = export_columns
        .and_then(|cols| {
            cols.iter()
                .find(|c| c.source_column_name == where_column)
                .map(|c| c.sql_type.clone())
        })
        .unwrap_or_else(|| "TEXT".to_string());

    let where_export_name = export_specs
        .iter()
        .find(|spec| spec.source_index == where_column_index)
        .map(|spec| spec.export_name.clone())
        .unwrap_or_else(|| where_column.to_string());

    let mut extract_specs = export_specs.clone();
    if !extract_specs
        .iter()
        .any(|s| s.source_index == where_column_index)
    {
        extract_specs.push(ColumnExportSpec {
            source_index: where_column_index,
            export_name: where_export_name.clone(),
            sql_type: where_sql_type,
        });
    }

    let exported_source_indices: std::collections::HashSet<usize> =
        export_specs.iter().map(|s| s.source_index).collect();

    let mut sql_statements = String::new();

    extract_rows_from_batches(batches, &extract_specs, |row| {
        let mut set_clauses = Vec::new();
        let mut where_value: Option<String> = None;

        for (spec, value) in extract_specs.iter().zip(row.iter()) {
            if spec.source_index == where_column_index {
                where_value = Some(value.clone());
            } else if exported_source_indices.contains(&spec.source_index) {
                let export_name = &spec.export_name;
                set_clauses.push(match db_dialect {
                    Dialect::MySQL => format!("`{}` = {}", export_name, value),
                    Dialect::PostgreSQL => format!("\"{}\" = {}", export_name, value),
                });
            }
        }

        if !set_clauses.is_empty() {
            let Some(where_value) = where_value else {
                return Ok(());
            };
            let where_clause = match db_dialect {
                Dialect::MySQL => format!("`{}` = {}", where_export_name, where_value),
                Dialect::PostgreSQL => format!("\"{}\" = {}", where_export_name, where_value),
            };
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
        }

        Ok(())
    })?;

    Ok(sql_statements)
}

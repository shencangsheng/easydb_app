use crate::commands::query::Dialect;
use crate::context::schema::AppResult;
use datafusion::arrow::error::ArrowError;
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::arrow::util::display::{ArrayFormatter, FormatOptions};
use datafusion::dataframe::DataFrame;
use datafusion_table_providers::mysql::MySQL;

/// Helper function to format a value from Arrow Array for SQL
fn format_value_for_sql(value: &str) -> String {
    if value == "NULL" {
        "NULL".to_string()
    } else {
        // Escape single quotes and wrap in quotes for string values
        format!("'{}'", value.replace("'", "''"))
    }
}

/// Helper function to extract rows from RecordBatch in a streaming fashion
fn extract_rows_from_batches<F>(batches: Vec<RecordBatch>, mut on_row: F) -> AppResult<()>
where
    F: FnMut(Vec<String>) -> AppResult<()>,
{
    let Some(first_batch) = batches.first() else {
        return Ok(());
    };

    let width = first_batch.schema().fields().len();
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
            let mut cells = Vec::with_capacity(width);
            for formatter in &formatters {
                let formatted_value = formatter.value(row_idx).to_string();
                // Check if the value is numeric or boolean (not wrapped in quotes)
                let sql_value = if formatted_value == "NULL" {
                    "NULL".to_string()
                } else if formatted_value.parse::<i64>().is_ok()
                    || formatted_value.parse::<f64>().is_ok()
                    || formatted_value == "true"
                    || formatted_value == "false"
                {
                    formatted_value
                } else {
                    format_value_for_sql(&formatted_value)
                };
                cells.push(sql_value);
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

    let mut sql_statements = String::new();
    let columns = headers
        .iter()
        .map(|h| match db_dialect {
            Dialect::MySQL => format!("`{}`", h),
            Dialect::PostgreSQL => format!("\"{}\"", h),
        })
        .collect::<Vec<String>>()
        .join(", ");
    let insert_header_template = match db_dialect {
        Dialect::MySQL => format!("INSERT INTO `{}` ({}) VALUES\n", table_name, columns),
        Dialect::PostgreSQL => format!("INSERT INTO \"{}\" ({}) VALUES\n", table_name, columns)
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

    extract_rows_from_batches(batches, |row| {
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

    // Find the index of the WHERE column
    let where_column_index = headers
        .iter()
        .position(|h| h == where_column)
        .ok_or_else(|| crate::context::error::AppError::BadRequest {
            message: format!("WHERE column '{}' not found", where_column),
        })?;

    let mut sql_statements = String::new();

    extract_rows_from_batches(batches, |row| {
        let mut set_clauses = Vec::new();

        for (col_index, (header, value)) in headers.iter().zip(row.iter()).enumerate() {
            if col_index != where_column_index {
                set_clauses.push(match db_dialect {
                    Dialect::MySQL => format!("`{}` = {}", header, value),
                    Dialect::PostgreSQL => format!("\"{}\" = {}", header, value)
                });
            }
        }

        if !set_clauses.is_empty() {
            let where_value = &row[where_column_index];
            let where_clause = 
            match db_dialect {
                Dialect::MySQL => format!("`{}` = {}", where_column, where_value),
                Dialect::PostgreSQL => format!("\"{}\" = {}", where_column, where_value)
            };
            let update_statement = 
            match db_dialect {
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
                )
            };
            sql_statements.push_str(&update_statement);
        }

        Ok(())
    })?;

    Ok(sql_statements)
}

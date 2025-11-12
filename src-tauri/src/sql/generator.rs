use crate::context::schema::AppResult;
use arrow::util::display::{ArrayFormatter, FormatOptions};
use arrow_array::RecordBatch;
use arrow_schema::ArrowError;
use datafusion::dataframe::DataFrame;

/// Helper function to format a value from Arrow Array for SQL
fn format_value_for_sql(value: &str) -> String {
    if value == "NULL" {
        "NULL".to_string()
    } else {
        // Escape single quotes and wrap in quotes for string values
        format!("'{}'", value.replace("'", "''"))
    }
}

/// Helper function to extract rows from RecordBatch
fn extract_rows_from_batches(
    batches: Vec<RecordBatch>,
) -> AppResult<(Vec<String>, Vec<Vec<String>>)> {
    if batches.is_empty() {
        return Ok((Vec::new(), Vec::new()));
    }

    let first_batch = &batches[0];
    let headers: Vec<String> = first_batch
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().to_string())
        .collect();

    let width = headers.len();
    let options = FormatOptions::default().with_null("NULL");
    let mut rows = Vec::new();

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
            rows.push(cells);
        }
    }

    Ok((headers, rows))
}

/// Generate SQL insert statements from DataFrame
/// Splits large datasets into multiple INSERT statements for better performance
pub async fn generate_sql_inserts(
    df: DataFrame,
    table_name: &str,
    max_values_per_insert: usize,
) -> AppResult<String> {
    // Collect RecordBatches from DataFrame
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;

    let (headers, rows) = extract_rows_from_batches(batches)?;

    let mut sql_statements = String::new();

    // Generate INSERT INTO statement header template
    let columns = headers
        .iter()
        .map(|h| format!("`{}`", h))
        .collect::<Vec<String>>()
        .join(", ");
    let insert_header_template = format!("INSERT INTO `{}` ({}) VALUES\n", table_name, columns);

    // Split rows into chunks for multiple INSERT statements
    let chunks: Vec<&[Vec<String>]> = rows.chunks(max_values_per_insert).collect();

    for (chunk_index, chunk) in chunks.iter().enumerate() {
        // Add header for each INSERT statement
        sql_statements.push_str(&insert_header_template);

        // Generate VALUES clauses for this chunk
        let mut values_clauses = Vec::new();
        for row in *chunk {
            let values = row.join(", ");
            values_clauses.push(format!("({})", values));
        }

        // Add VALUES clauses
        sql_statements.push_str(&values_clauses.join(",\n"));
        sql_statements.push(';');

        // Add newline between INSERT statements (except for the last one)
        if chunk_index < chunks.len() - 1 {
            sql_statements.push('\n');
        }
    }

    Ok(sql_statements)
}

/// Generate SQL UPDATE statements from DataFrame
/// Creates UPDATE statements with WHERE conditions based on the specified column
pub async fn generate_sql_update(
    df: DataFrame,
    table_name: &str,
    where_column: &str,
) -> AppResult<String> {
    // Collect RecordBatches from DataFrame
    let batches = df
        .collect()
        .await
        .map_err(|e| crate::context::error::AppError::BadRequest {
            message: format!("Failed to collect DataFrame: {}", e),
        })?;

    let (headers, rows) = extract_rows_from_batches(batches)?;

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

    // Generate UPDATE statements for each row
    for row in rows {
        let mut set_clauses = Vec::new();

        // Build SET clauses for all columns except the WHERE column
        for (col_index, (header, value)) in headers.iter().zip(row.iter()).enumerate() {
            if col_index != where_column_index {
                set_clauses.push(format!("`{}` = {}", header, value));
            }
        }

        // Only generate UPDATE statement if there are columns to update
        if !set_clauses.is_empty() {
            // Build WHERE clause
            let where_value = &row[where_column_index];
            let where_clause = format!("`{}` = {}", where_column, where_value);

            // Generate the complete UPDATE statement
            let update_statement = format!(
                "UPDATE `{}` SET {} WHERE {};\n",
                table_name,
                set_clauses.join(", "),
                where_clause
            );

            sql_statements.push_str(&update_statement);
        }
    }

    Ok(sql_statements)
}

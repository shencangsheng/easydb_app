use crate::context::schema::AppResult;
use polars::prelude::{AnyValue, DataFrame};

/// Generate SQL insert statements from DataFrame
/// Splits large datasets into multiple INSERT statements for better performance
pub fn generate_sql_inserts(
    df: &DataFrame,
    table_name: &str,
    max_values_per_insert: usize,
) -> AppResult<String> {
    let headers: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();
    let mut sql_statements = String::new();

    // Generate INSERT INTO statement header template
    let columns = headers
        .iter()
        .map(|h| format!("`{}`", h))
        .collect::<Vec<String>>()
        .join(", ");
    let insert_header_template = format!("INSERT INTO `{}` ({}) VALUES\n", table_name, columns);

    let height = df.height();
    let width = df.width();

    // Convert DataFrame to rows for processing
    let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];

    let mut row_i = 0;
    df.iter().for_each(|col| {
        col.rechunk().iter().enumerate().for_each(|(index, value)| {
            rows.get_mut(index).unwrap()[row_i] = match value {
                AnyValue::Null => "NULL".to_string(),
                AnyValue::String(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
                AnyValue::Int64(i) => i.to_string(),
                AnyValue::Float64(f) => f.to_string(),
                AnyValue::Boolean(b) => b.to_string(),
                _ => format!("'{}'", value.to_string().replace("'", "''")),
            };
        });
        row_i += 1;
    });

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
pub fn generate_sql_update(
    df: &DataFrame,
    table_name: &str,
    where_column: &str,
) -> AppResult<String> {
    let headers: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();

    // Validate that the WHERE column exists
    if !headers.contains(&where_column.to_string()) {
        return Err(crate::context::error::AppError::BadRequest {
            message: format!("WHERE column '{}' not found in data", where_column),
        });
    }

    let mut sql_statements = String::new();
    let height = df.height();
    let width = df.width();

    // Convert DataFrame to rows for processing
    let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];

    let mut row_i = 0;
    df.iter().for_each(|col| {
        col.rechunk().iter().enumerate().for_each(|(index, value)| {
            rows.get_mut(index).unwrap()[row_i] = match value {
                AnyValue::Null => "NULL".to_string(),
                AnyValue::String(s) => format!("'{}'", s.replace("'", "''")), // Escape single quotes
                AnyValue::Int64(i) => i.to_string(),
                AnyValue::Float64(f) => f.to_string(),
                AnyValue::Boolean(b) => b.to_string(),
                _ => format!("'{}'", value.to_string().replace("'", "''")),
            };
        });
        row_i += 1;
    });

    // Find the index of the WHERE column
    let where_column_index = headers
        .iter()
        .position(|h| h == where_column)
        .ok_or_else(|| crate::context::error::AppError::BadRequest {
            message: format!("WHERE column '{}' not found", where_column),
        })?;

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

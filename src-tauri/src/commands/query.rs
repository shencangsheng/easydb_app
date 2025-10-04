use crate::commands::run_blocking;
use crate::context::context::{collect, register};
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::sql::generator::generate_sql_inserts;
use crate::utils::date_utils::time_difference_from_now;
use crate::utils::db_utils;
use crate::utils::db_utils::insert_query_history;
use chrono::Utc;
use dirs;
use polars::io::SerWriter;
use polars::prelude::{AnyValue, CsvWriter};
use polars::sql::SQLContext;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::Write;
use tauri::{command, AppHandle};

#[derive(Serialize)]
pub struct FetchResult {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub query_time: String,
}

#[derive(Serialize)]
pub struct FetchHistory {
    pub sql: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct WriterResult {
    pub query_time: String,
    pub file_name: String,
}

#[command]
pub async fn fetch(app: AppHandle, sql: String) -> AppResult<FetchResult> {
    run_blocking(move || {
        let start = Utc::now();
        let mut context = SQLContext::new();

        let new_sql = register(&mut context, &sql, Some("200".to_string()))?;
        let df = collect(&mut context, &new_sql).map_err(|err| {
            let _ = insert_query_history(&app, &sql, "fail");
            err
        })?;

        let height = df.height();
        let width = df.width();

        let header: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();

        let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];

        let mut row_i = 0;
        df.iter().for_each(|col| {
            col.rechunk().iter().enumerate().for_each(|(index, value)| {
                rows.get_mut(index).unwrap()[row_i] = match value {
                    AnyValue::Null => "NULL".to_string(),
                    _ => value.to_string().replace("\"", ""),
                }
            });
            row_i += 1;
        });

        insert_query_history(&app, &sql, "successful")?;

        Ok(FetchResult {
            header,
            rows,
            query_time: time_difference_from_now(start),
        })
    })
    .await
}

#[command]
pub async fn sql_history(app: AppHandle) -> AppResult<Vec<FetchHistory>> {
    run_blocking(move || {
        let conn = db_utils::conn(&app)?;
        let mut stmt = conn
            .prepare("select sql, status, created_at from sql_history order by id desc limit 30")?;

        let rows = stmt.query_map([], |row| {
            Ok(FetchHistory {
                sql: row.get(0)?,
                status: row.get(1)?,
                created_at: row.get(2)?,
            })
        })?;

        let mut results = Vec::with_capacity(30);

        for row in rows {
            results.push(row?);
        }
        Ok(results)
    })
    .await
}

#[command]
pub async fn writer(
    _app: AppHandle,
    file_type: String,
    sql: String,
    table_name: Option<String>,
    max_values_per_insert: Option<usize>,
) -> AppResult<WriterResult> {
    run_blocking(move || {
        let mut downloads_dir = dirs::download_dir().ok_or_else(|| AppError::BadRequest {
            message: "Couldn't find the current working directory".to_string(),
        })?;
        let start = Utc::now();

        // Validate required parameters for SQL export
        if file_type.to_lowercase() == "sql" {
            if table_name.is_none() {
                return Err(AppError::BadRequest {
                    message: "Table name is required for SQL export".to_string(),
                });
            }
            if max_values_per_insert.is_none() {
                return Err(AppError::BadRequest {
                    message: "Max values per insert is required for SQL export".to_string(),
                });
            }
        }

        let mut context = SQLContext::new();
        let new_sql = register(&mut context, &sql, None)?;
        let mut df = collect(&mut context, &new_sql)?;

        // Determine file extension
        let file_extension = match file_type.to_lowercase().as_str() {
            "csv" => "csv",
            "tsv" => "tsv",
            "sql" => "sql",
            _ => {
                return Err(AppError::BadRequest {
                    message: "Unsupported file type. Supported types: csv, tsv, sql".to_string(),
                })
            }
        };

        downloads_dir.push(format!(
            "easydb_{}.{}",
            Utc::now().timestamp_millis(),
            file_extension
        ));

        // Create file once for all formats
        let mut file = File::create(&downloads_dir)?;

        match file_type.to_lowercase().as_str() {
            "csv" => {
                CsvWriter::new(file).finish(&mut df)?;
            }
            "tsv" => {
                // Use CsvWriter with tab separator for TSV
                CsvWriter::new(file).with_separator(b'\t').finish(&mut df)?;
            }
            "sql" => {
                // Generate SQL insert statements
                // We already validated that these are not None above
                let table_name_value = table_name.unwrap();
                let max_values = max_values_per_insert.unwrap();
                let sql_content = generate_sql_inserts(&df, &table_name_value, max_values)?;
                writeln!(file, "{}", sql_content)?;
            }
            _ => unreachable!(), // This case is handled above
        }

        Ok(WriterResult {
            query_time: time_difference_from_now(start),
            file_name: fs::canonicalize(&downloads_dir)?.display().to_string(),
        })
    })
    .await
}

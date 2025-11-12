use crate::commands::{run_blocking, run_blocking_async};
use crate::context::context::{collect, get_data_frame, get_sql_context, register};
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::sql::generator::{generate_sql_inserts, generate_sql_update};
use crate::utils::date_utils::time_difference_from_now;
use crate::utils::db_utils;
use crate::utils::db_utils::insert_query_history;
use arrow::util::display::{ArrayFormatter, FormatOptions};
use arrow_schema::ArrowError;
use chrono::Utc;
use datafusion::config::CsvOptions;
use datafusion::dataframe::DataFrameWriteOptions;
use dirs;
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
pub async fn fetch(
    app: AppHandle,
    sql: String,
    offset: usize,
    limit: usize,
) -> AppResult<FetchResult> {
    run_blocking_async(async move || {
        let start = Utc::now();
        let mut context = get_sql_context();

        let new_sql = register(&mut context, &sql, Some(limit), Some(offset)).await?;
        let records = collect(&mut context, &new_sql).await.map_err(|err| {
            let _ = insert_query_history(&app, &sql, "fail");
            err
        })?;

        let row = &records[0];
        let width = row.columns().len();

        let header: Vec<String> = row
            .schema()
            .fields()
            .iter()
            .map(|c| c.name().to_string())
            .collect();

        // Pre-calculate the total number of rows to avoid frequent reallocation
        let total_rows: usize = records.iter().map(|r| r.num_rows()).sum();
        let mut rows: Vec<Vec<String>> = Vec::with_capacity(total_rows);
        let options = FormatOptions::default().with_null("NULL");

        for record in records {
            let formatters = record
                .columns()
                .iter()
                .map(|c| ArrayFormatter::try_new(c.as_ref(), &options))
                .collect::<Result<Vec<_>, ArrowError>>()?;

            for row in 0..record.num_rows() {
                let mut cells = Vec::with_capacity(width);
                for (_, formatter) in formatters.iter().enumerate() {
                    cells.push(formatter.value(row).to_string());
                }
                rows.push(cells);
            }
        }

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
            .prepare("select sql, status, created_at from sql_history order by id desc limit 50")?;

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
    sql_statement_type: Option<String>,
    where_column: Option<String>,
) -> AppResult<WriterResult> {
    run_blocking_async(async move || {
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

            let statement_type = sql_statement_type
                .as_ref()
                .map(|s| s.to_uppercase())
                .unwrap_or_else(|| "INSERT".to_string());
            match statement_type.as_str() {
                "INSERT" => {
                    if max_values_per_insert.is_none() {
                        return Err(AppError::BadRequest {
                            message: "Max values per insert is required for INSERT statements"
                                .to_string(),
                        });
                    }
                }
                "UPDATE" => {
                    if where_column.is_none() {
                        return Err(AppError::BadRequest {
                            message: "WHERE column is required for UPDATE statements".to_string(),
                        });
                    }
                }
                _ => {
                    return Err(AppError::BadRequest {
                        message: "Invalid SQL statement type. Supported types: INSERT, UPDATE"
                            .to_string(),
                    });
                }
            }
        }

        let mut context = get_sql_context();
        let new_sql = register(&mut context, &sql, None, None).await?;
        let df = get_data_frame(&mut context, &new_sql).await?;

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
            Utc::now().format("%Y%m%d%H%M%S").to_string(),
            file_extension
        ));
        let file_path = downloads_dir.to_string_lossy().to_string();

        match file_type.to_lowercase().as_str() {
            "csv" => {
                df.write_csv(&file_path, DataFrameWriteOptions::new(), None)
                    .await?;
            }
            "tsv" => {
                let mut options = CsvOptions::default();
                options.delimiter = b'\t';
                df.write_csv(&file_path, DataFrameWriteOptions::new(), None)
                    .await?;
            }
            "sql" => {
                // Generate SQL statements based on statement type
                let table_name_value = table_name.unwrap();
                let statement_type = sql_statement_type
                    .as_ref()
                    .map(|s| s.to_uppercase())
                    .unwrap_or_else(|| "INSERT".to_string());

                let sql_content = match statement_type.as_str() {
                    "INSERT" => {
                        let max_values = max_values_per_insert.unwrap();
                        generate_sql_inserts(df, &table_name_value, max_values).await?
                    }
                    "UPDATE" => {
                        let where_column_value = where_column.unwrap();
                        generate_sql_update(df, &table_name_value, &where_column_value).await?
                    }
                    _ => {
                        return Err(AppError::BadRequest {
                            message: "Invalid SQL statement type".to_string(),
                        });
                    }
                };

                let mut file = File::create(&downloads_dir)?;
                write!(file, "{}", sql_content)?;
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

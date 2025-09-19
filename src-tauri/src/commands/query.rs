use crate::commands::run_blocking;
use crate::context::context::{collect, register};
use crate::context::error::AppError;
use crate::context::schema::AppResult;
use crate::utils::date_utils::time_difference_from_now;
use chrono::Utc;
use polars::prelude::AnyValue;
use polars::sql::SQLContext;
use serde::Serialize;
use std::time::Instant;
use tauri::command;

#[derive(Serialize)]
pub struct FetchResult {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub query_time: String,
}

#[command]
pub async fn fetch(sql: String) -> AppResult<FetchResult> {
    run_blocking(move || {
        let start = Utc::now();
        let mut context = SQLContext::new();

        let new_sql = register(&mut context, &sql, Some("200".to_string()))?;
        let df = collect(&mut context, &new_sql)?;

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

        Ok(FetchResult {
            header,
            rows,
            query_time: time_difference_from_now(start),
        })
    })
    .await
}

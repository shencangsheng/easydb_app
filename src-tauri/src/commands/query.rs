use crate::commands::run_blocking;
use crate::context::context::{collect, register};
use crate::context::schema::AppResult;
use crate::utils::date_utils::time_difference_from_now;
use crate::utils::db_utils;
use crate::utils::db_utils::insert_query_history;
use chrono::Utc;
use polars::prelude::AnyValue;
use polars::sql::SQLContext;
use serde::Serialize;
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
        let mut stmt = conn.prepare(
            "select sql, status, created_at from sql_history order by id desc limit 30",
        )?;

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

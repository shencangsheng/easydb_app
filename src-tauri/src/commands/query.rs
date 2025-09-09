use crate::context::context::{collect, register};
use crate::context::schema::AppResult;
use polars::sql::SQLContext;
use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
pub struct FetchResult {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

#[command]
pub fn fetch(sql: String) -> AppResult<FetchResult> {
    let mut context = SQLContext::new();

    let new_sql = register(&mut context, &sql, Some("200".to_string()))?;

    let df = collect(&mut context, &new_sql)?;

    let height = df.height();
    let width = df.width();

    let header: Vec<String> = df.column_iter().map(|c| c.name().to_string()).collect();

    let mut rows: Vec<Vec<String>> = vec![vec![String::new(); width]; height];

    let mut row_i = 0;
    df.iter().for_each(|col| {
        col.iter().enumerate().for_each(|(index, value)| {
            rows.get_mut(index).unwrap()[row_i] = value.to_string();
        });
        row_i += 1;
    });

    Ok(FetchResult { header, rows })
}

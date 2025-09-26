use crate::context::schema::AppResult;
use crate::utils::app_data_utils::get_app_data_dir;
use chrono::Local;
use rusqlite::{params, Connection};
use tauri::AppHandle;

pub fn conn(app: &AppHandle) -> AppResult<Connection> {
    Ok(Connection::open(get_app_data_dir(app)?.join("sqlite.db"))?)
}

pub fn init(app: &AppHandle) {
    let conn = conn(app).expect("Failed to establish database connection.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sql_history (
                  id INTEGER PRIMARY KEY,
                  sql text,
                  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                  status text
                  )",
        [],
    )
    .expect("Failed to create query_history");
}

pub fn insert_query_history(app: &AppHandle, sql: &str, status: &str) -> AppResult<()> {
    conn(app)?.execute(
        r#"
                        insert into sql_history ( sql, status, created_at )
                        values
                        (?1, ?2, ?3)
                        "#,
        params![
            sql,
            status,
            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
        ],
    )?;

    Ok(())
}

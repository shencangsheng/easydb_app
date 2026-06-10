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

    conn.execute(
        "CREATE TABLE IF NOT EXISTS saved_queries (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  sql TEXT NOT NULL,
                  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
                  )",
        [],
    )
    .expect("Failed to create saved_queries");
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

pub fn insert_saved_query(app: &AppHandle, name: &str, sql: &str) -> AppResult<i64> {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let conn = conn(app)?;
    conn.execute(
        r#"
        INSERT INTO saved_queries (name, sql, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4)
        "#,
        params![name, sql, &now, &now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn list_saved_queries(app: &AppHandle) -> AppResult<Vec<(i64, String, String, String)>> {
    let conn = conn(app)?;
    let mut stmt = conn
        .prepare("SELECT id, name, sql, created_at FROM saved_queries ORDER BY updated_at DESC")?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
            row.get::<_, String>(3)?,
        ))
    })?;

    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

pub fn delete_saved_query(app: &AppHandle, id: i64) -> AppResult<()> {
    conn(app)?.execute("DELETE FROM saved_queries WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn list_sql_history(
    app: &AppHandle,
    limit: Option<i64>,
    keyword: Option<&str>,
) -> AppResult<Vec<(String, String, String)>> {
    let conn = conn(app)?;
    let lim = match limit.unwrap_or(50) {
        l if l <= 0 => -1,
        l => l,
    };

    let mut results = Vec::new();

    if let Some(k) = keyword.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let pattern = format!("%{}%", k);
        let mut stmt = conn.prepare(
            "SELECT sql, status, created_at FROM sql_history WHERE sql LIKE ?1 ORDER BY id DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![pattern, lim], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        for row in rows {
            results.push(row?);
        }
    } else {
        let mut stmt = conn.prepare(
            "SELECT sql, status, created_at FROM sql_history ORDER BY id DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![lim], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        for row in rows {
            results.push(row?);
        }
    }

    Ok(results)
}

pub fn delete_sql_history_before(app: &AppHandle, before: &str) -> AppResult<usize> {
    let deleted = conn(app)?.execute(
        "DELETE FROM sql_history WHERE created_at < ?1",
        params![before],
    )?;
    Ok(deleted)
}

pub fn delete_all_sql_history(app: &AppHandle) -> AppResult<usize> {
    let deleted = conn(app)?.execute("DELETE FROM sql_history", [])?;
    Ok(deleted)
}

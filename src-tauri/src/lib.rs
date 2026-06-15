use crate::commands::app::restart_app;
use crate::commands::query::{
    delete_saved_query, delete_sql_history_before, fetch, fetch_column_types, fetch_page,
    generate_sql_content, list_saved_queries, save_query, sql_history, writer,
};
use crate::commands::utils::open_url;
use crate::utils::db_utils;
use tauri::Listener;

pub mod commands;

pub mod context;

pub mod reader;
pub mod sql;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            app.handle().plugin(tauri_plugin_dialog::init())?;
            db_utils::init(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            delete_saved_query,
            delete_sql_history_before,
            fetch,
            fetch_column_types,
            fetch_page,
            generate_sql_content,
            list_saved_queries,
            open_url,
            restart_app,
            save_query,
            sql_history,
            writer
        ])
        .on_page_load(|window, _| {
            window.listen("tauri://error", |event| {
                println!("Error event received: {:?}", event);
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

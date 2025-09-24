use crate::commands::app::restart_app;
use crate::commands::query::fetch;
use crate::commands::utils::open_url;
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
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![fetch, open_url, restart_app])
        .on_page_load(|window, _| {
            window.listen("tauri://error", |event| {
                println!("Error event received: {:?}", event);
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

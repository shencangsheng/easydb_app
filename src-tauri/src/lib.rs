use tauri::Listener;
use crate::commands::query::fetch;

mod commands;

mod context;

mod sql;

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
      Ok(())
    })
      .invoke_handler(tauri::generate_handler![fetch])
      .on_page_load(|window, _| {
          window.listen("tauri://error", |event| {
              println!("Error event received: {:?}", event);
          });
      })
      .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

mod commands;
mod models;
mod storage;

use storage::Storage;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize storage
            let storage = Storage::new(app.handle())
                .expect("Failed to initialize storage");
            
            app.manage(Mutex::new(storage));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_notes,
            commands::get_note,
            commands::create_note,
            commands::update_note,
            commands::delete_note,
            commands::search_notes,
            commands::get_notes_by_status,
            commands::get_stats,
        ]);

    app_builder
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

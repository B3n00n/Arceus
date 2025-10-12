mod commands;
mod core;
mod services;

use commands::update_commands::{check_for_updates, download_and_install_update, skip_update, close_updater_and_show_main};
use services::update_service::create_update_service;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let update_service = create_update_service(app.handle().clone());
            app.manage(update_service);

            // Show the updater window on startup
            if let Some(updater_window) = app.handle().get_webview_window("updater") {
                let _ = updater_window.show();
                let _ = updater_window.set_focus();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            check_for_updates,
            download_and_install_update,
            skip_update,
            close_updater_and_show_main
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
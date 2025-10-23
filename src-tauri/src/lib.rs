mod commands;
mod core;
mod handlers;
mod net;
mod network;
mod protocol;
mod services;
mod storage;

use commands::{
    execute_shell, get_device, get_devices, get_installed_apps, get_volume, install_local_apk,
    install_remote_apk, launch_app, ping_devices, request_battery, set_device_name, set_volume,
    restart_devices, uninstall_app,
    add_apk, list_apks, open_apk_folder, remove_apk,
    check_for_updates, close_updater_and_show_main, download_and_install_update, skip_update,
};

use core::{AppConfig, EventBus};
use handlers::HandlerRegistry;
use network::{ConnectionManager, HttpServer, TcpServer};
use services::{update_service::create_update_service, ApkService, BatteryMonitor, DeviceService};
use storage::DeviceNamesStore;

use std::sync::Arc;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            tracing::info!("Starting Arceus application...");

            let update_service = create_update_service(app.handle().clone());
            app.manage(update_service);

            let app_handle = app.handle().clone();

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let config = AppConfig::with_paths(
                app_data_dir.join("apks"),
                app_data_dir.join("arceus.db"),
            );

            config.validate().expect("Invalid configuration");

            std::fs::create_dir_all(&config.apk_directory)
                .expect("Failed to create APK directory");

            let event_bus = Arc::new(EventBus::new(app_handle.clone()));

            let device_names_store = Arc::new(
                DeviceNamesStore::new(&config.database_path)
                    .expect("Failed to initialize device names store"),
            );

            let connection_manager =
                Arc::new(ConnectionManager::new(config.server.max_connections));

            let mut handler_registry = HandlerRegistry::new();

            // Register all response handlers
            use handlers::r#impl::*;
            handler_registry.register(Arc::new(DeviceConnectedHandler::new()));
            handler_registry.register(Arc::new(HeartbeatHandler::new()));
            handler_registry.register(Arc::new(BatteryStatusHandler::new()));
            handler_registry.register(Arc::new(VolumeStatusHandler::new()));
            handler_registry.register(Arc::new(LaunchAppResponseHandler::new()));
            handler_registry.register(Arc::new(ShellExecutionResponseHandler::new()));
            handler_registry.register(Arc::new(InstalledAppsResponseHandler::new()));
            handler_registry.register(Arc::new(PingResponseHandler::new()));
            handler_registry.register(Arc::new(ApkInstallResponseHandler::new()));
            handler_registry.register(Arc::new(UninstallAppResponseHandler::new()));
            handler_registry.register(Arc::new(VolumeSetResponseHandler::new()));

            let handler_registry = Arc::new(handler_registry);

            let tcp_server = Arc::new(TcpServer::new(
                config.server.clone(),
                Arc::clone(&connection_manager),
                Arc::clone(&handler_registry),
                Arc::clone(&event_bus),
                Arc::clone(&device_names_store),
            ));

            let http_server = Arc::new(HttpServer::new(
                config.server.http_port,
                config.apk_directory.clone(),
                Arc::clone(&event_bus),
            ));

            let device_service = Arc::new(DeviceService::new(
                Arc::clone(&connection_manager),
                Arc::clone(&device_names_store),
            ));

            let apk_service = Arc::new(ApkService::new(
                config.apk_directory.clone(),
                Arc::clone(&http_server),
            ));

            let battery_monitor = Arc::new(BatteryMonitor::new(
                Arc::clone(&connection_manager),
                config.server.battery_update_interval,
            ));

            app.manage(Arc::clone(&device_service));
            app.manage(Arc::clone(&apk_service));

            let tcp_server_clone = Arc::clone(&tcp_server);
            tauri::async_runtime::spawn(async move {
                if let Err(e) = tcp_server_clone.start().await {
                    tracing::error!("TCP server error: {}", e);
                }
            });

            let http_server_clone = Arc::clone(&http_server);
            tauri::async_runtime::spawn(async move {
                if let Err(e) = http_server_clone.start().await {
                    tracing::error!("HTTP server error: {}", e);
                }
            });

            let battery_monitor_clone = Arc::clone(&battery_monitor);
            tauri::async_runtime::spawn(async move {
                battery_monitor_clone.start().await;
            });

            if let Some(updater_window) = app.get_webview_window("updater") {
                let _ = updater_window.show();
                let _ = updater_window.set_focus();
            } else if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window.show();
                let _ = main_window.set_focus();
            }

            tracing::info!("Arceus application started successfully");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_devices,
            get_device,
            set_device_name,
            launch_app,
            uninstall_app,
            request_battery,
            ping_devices,
            set_volume,
            get_volume,
            execute_shell,
            get_installed_apps,
            install_remote_apk,
            install_local_apk,
            restart_devices,
            list_apks,
            add_apk,
            remove_apk,
            open_apk_folder,
            check_for_updates,
            download_and_install_update,
            skip_update,
            close_updater_and_show_main,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
mod domain;
mod application;
mod infrastructure;
mod presentation;
mod commands;
mod core;
mod protocol;
mod net;
use commands::{
    close_all_apps, execute_shell, get_device, get_devices, get_installed_apps, get_volume,
    install_local_apk, install_remote_apk, launch_app, ping_devices, request_battery,
    set_device_name, set_volume, restart_devices, uninstall_app,
    add_apk, list_apks, open_apk_folder, remove_apk,
    check_for_updates, close_updater_and_show_main, download_and_install_update, skip_update,
    start_game, get_current_game, stop_game,
};
use core::{AppConfig, EventBus};
use application::services::{ApkApplicationService, BatteryMonitor, DeviceApplicationService, GameApplicationService, HttpServerService};
use infrastructure::repositories::{FsApkRepository, InMemoryDeviceRepository, SledDeviceNameRepository};
use infrastructure::network::TcpServer;
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

            use application::services::update_service::create_update_service;
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

            let device_repo = Arc::new(InMemoryDeviceRepository::new());
            let device_name_repo = Arc::new(
                SledDeviceNameRepository::new(&config.database_path)
                    .expect("Failed to initialize device names repository"),
            );

            let http_host = if config.server.tcp_host == "0.0.0.0" {
                if let Ok(local_ip) = local_ip_address::local_ip() {
                    local_ip.to_string()
                } else {
                    tracing::warn!("Could not detect local IP address, using localhost (remote devices won't be able to download APKs)");
                    "127.0.0.1".to_string()
                }
            } else {
                config.server.tcp_host.clone()
            };
            tracing::info!("APK download base URL: http://{}:{}", http_host, config.server.http_port);
            let base_url = format!("http://{}:{}", http_host, config.server.http_port);
            let apk_repo = Arc::new(FsApkRepository::new(
                config.apk_directory.clone(),
                base_url,
            ));

            let (tcp_server, shutdown_rx, session_manager) = TcpServer::new(
                config.server.clone(),
                device_repo.clone(),
                device_name_repo.clone(),
                event_bus.clone(),
            );
            let tcp_server = Arc::new(tcp_server);

            let command_executor = Arc::new(crate::domain::services::CommandExecutor::new(
                device_repo.clone(),
                session_manager,
            ));

            let device_service = Arc::new(DeviceApplicationService::new(
                device_repo.clone(),
                device_name_repo.clone(),
                command_executor.clone(),
            ));

            let apk_service = Arc::new(ApkApplicationService::new(apk_repo.clone()));

            let game_service = Arc::new(GameApplicationService::new(Arc::clone(&event_bus)));

            let battery_interval = std::time::Duration::from_secs(config.server.battery_update_interval);
            let battery_monitor = Arc::new(BatteryMonitor::new(
                device_repo.clone(),
                command_executor.clone(),
                battery_interval,
            ));

            app.manage(device_service);
            app.manage(apk_service);
            app.manage(game_service);

            let tcp_server_clone = Arc::clone(&tcp_server);
            tauri::async_runtime::spawn(async move {
                if let Err(e) = tcp_server_clone.start().await {
                    tracing::error!("TCP server error: {}", e);
                }
            });

            let apk_port = config.server.http_port;
            let apk_dir = config.apk_directory.clone();
            let event_bus_for_http = Arc::clone(&event_bus);
            tauri::async_runtime::spawn(async move {
                match HttpServerService::start_server(apk_port, apk_dir, "APK Server").await {
                    Ok((mut child, url)) => {
                        event_bus_for_http.http_server_started(apk_port, url);

                        if let Err(e) = child.wait().await {
                            tracing::error!("APK HTTP server process error: {}", e);
                        }
                    }
                    Err(e) => {
                        tracing::error!("Failed to start APK HTTP server: {}", e);
                    }
                }
            });

            let battery_monitor_clone = Arc::clone(&battery_monitor);
            tauri::async_runtime::spawn(async move {
                battery_monitor_clone.start(shutdown_rx).await;
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
            close_all_apps,
            list_apks,
            add_apk,
            remove_apk,
            open_apk_folder,
            check_for_updates,
            download_and_install_update,
            skip_update,
            close_updater_and_show_main,
            start_game,
            get_current_game,
            stop_game,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
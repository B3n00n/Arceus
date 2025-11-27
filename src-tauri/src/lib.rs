mod api;
mod app;
mod application;
mod domain;
mod infrastructure;
mod net;

use api::*;
use app::{AppConfig, AppState, EventBus, ServerManager, setup_signal_handlers};
use application::services::{
    ApkApplicationService, BatteryMonitor, DeviceApplicationService,
    GameApplicationService, update_service::create_update_service
};
use infrastructure::repositories::{
    FsApkRepository, InMemoryDeviceRepository, SledDeviceNameRepository
};
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

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            tracing::info!("Initializing Arceus application");

            let update_service = create_update_service(app.handle().clone());
            app.manage(update_service);

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

            let event_bus = Arc::new(EventBus::new(app.handle().clone()));
            let device_repo = Arc::new(InMemoryDeviceRepository::new());
            let device_name_repo = Arc::new(
                SledDeviceNameRepository::new(&config.database_path)
                    .expect("Failed to initialize device name repository"),
            );

            let http_host = if config.server.tcp_host == "0.0.0.0" {
                local_ip_address::local_ip()
                    .map(|ip| ip.to_string())
                    .unwrap_or_else(|_| {
                        tracing::warn!("Could not detect local IP, using localhost");
                        "127.0.0.1".to_string()
                    })
            } else {
                config.server.tcp_host.clone()
            };
            let base_url = format!("http://{}:{}", http_host, config.server.http_port);
            let apk_repo = Arc::new(FsApkRepository::new(
                config.apk_directory.clone(),
                base_url,
            ));

            let (tcp_server, _, session_manager) = TcpServer::new(
                config.server.clone(),
                device_repo.clone(),
                device_name_repo.clone(),
                event_bus.clone(),
            );
            let tcp_server = Arc::new(tcp_server);

            let command_executor = Arc::new(crate::domain::services::CommandExecutor::new(
                device_repo.clone(),
                session_manager.clone(),
            ));

            let device_service = Arc::new(DeviceApplicationService::new(
                device_repo.clone(),
                device_name_repo.clone(),
                command_executor.clone(),
            ));
            let apk_service = Arc::new(ApkApplicationService::new(apk_repo.clone()));
            let game_service = Arc::new(GameApplicationService::new(event_bus.clone()));

            let battery_interval = std::time::Duration::from_secs(config.server.battery_update_interval);
            let battery_monitor = Arc::new(BatteryMonitor::new(
                device_repo.clone(),
                session_manager.clone(),
                command_executor.clone(),
                battery_interval,
            ));

            let app_state = Arc::new(AppState::new(tcp_server.clone()));
            let server_manager = Arc::new(ServerManager::new(
                tcp_server.clone(),
                config.clone(),
                event_bus.clone(),
                battery_monitor.clone(),
            ));

            app.manage(device_service);
            app.manage(apk_service);
            app.manage(game_service);
            app.manage(app_state.clone());
            app.manage(server_manager);

            setup_signal_handlers(app_state.clone());

            if let Some(updater_window) = app.get_webview_window("updater") {
                let _ = updater_window.show();
                let _ = updater_window.set_focus();
            } else if let Some(main_window) = app.get_webview_window("main") {
                if let (Some(server_mgr), Some(app_state)) = (
                    app.try_state::<Arc<ServerManager>>(),
                    app.try_state::<Arc<AppState>>(),
                ) {
                    server_mgr.start(&app_state);
                }
                let _ = main_window.show();
                let _ = main_window.set_focus();
            }

            tracing::info!("Arceus initialization complete");
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
            display_message,
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
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri application");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::Exit = event {
            tracing::info!("Application exiting");
            if let Some(app_state) = app_handle.try_state::<Arc<AppState>>() {
                app_state.shutdown();
            }
            tracing::info!("Shutdown complete");
        }
    });
}

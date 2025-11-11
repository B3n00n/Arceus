use crate::app::AppState;
use std::sync::Arc;

pub fn setup_signal_handlers(app_state: Arc<AppState>) {
    tauri::async_runtime::spawn(async move {
        wait_for_shutdown_signal().await;
        tracing::info!("Shutdown signal received, initiating graceful shutdown");
        app_state.shutdown();
    });
}

async fn wait_for_shutdown_signal() {

    use tokio::signal::windows;

    let mut ctrl_c = windows::ctrl_c()
        .expect("Failed to install Ctrl+C handler");
    let mut ctrl_break = windows::ctrl_break()
        .expect("Failed to install Ctrl+Break handler");
    let mut ctrl_close = windows::ctrl_close()
        .expect("Failed to install console close handler");
    let mut ctrl_shutdown = windows::ctrl_shutdown()
        .expect("Failed to install system shutdown handler");

    tokio::select! {
        _ = ctrl_c.recv() => {
            tracing::info!("Received Ctrl+C");
        }
        _ = ctrl_break.recv() => {
            tracing::info!("Received Ctrl+Break");
        }
        _ = ctrl_close.recv() => {
            tracing::info!("Received console close event");
        }
        _ = ctrl_shutdown.recv() => {
            tracing::info!("Received system shutdown event");
        }
    }
}

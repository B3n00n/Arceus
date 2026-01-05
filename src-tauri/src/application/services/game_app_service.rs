use crate::application::services::HttpServerService;
use crate::app::EventBus;
use crate::infrastructure::process::HiddenCommandSync;
use crate::domain::models::{GameConfig, GameState};
use crate::infrastructure::game::{GameProcess, GameProcessManager};
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::process::Child;
use tokio::sync::broadcast;

const GAME_CONTENT_PORT: u16 = 8000;

pub type GameResult<T> = std::result::Result<T, GameApplicationError>;

#[derive(Debug, thiserror::Error)]
pub enum GameApplicationError {
    #[error("Arceus error: {0}")]
    Arceus(#[from] crate::app::error::ArceusError),

    #[error("Game is already running: {0}")]
    GameAlreadyRunning(String),

    #[error("No game is currently running")]
    NoGameRunning,

    #[error("{0}")]
    ValidationError(String),
}

pub struct GameApplicationService {
    event_bus: Arc<EventBus>,
    current_game: Arc<RwLock<Option<RunningGame>>>,
}

struct RunningGame {
    state: GameState,
    shutdown_tx: broadcast::Sender<()>,
}

impl GameApplicationService {
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            event_bus,
            current_game: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start_game(&self, config: GameConfig) -> GameResult<GameState> {
        {
            let current = self.current_game.read();
            if current.is_some() {
                return Err(GameApplicationError::GameAlreadyRunning(
                    current.as_ref().unwrap().state.config.name.clone(),
                ));
            }
        }

        config
            .validate()
            .map_err(|e| GameApplicationError::ValidationError(e))?;

        tracing::info!(
            game = %config.name,
            exe = ?config.exe_path,
            content_dir = ?config.content_path,
            "Starting game"
        );

        let process_manager = GameProcessManager::new(config.clone());
        let game_process = process_manager.start().await?;

        let process_id = game_process.process_id();

        let http_server_process = HttpServerService::start_server(
            GAME_CONTENT_PORT,
            config.content_path.clone(),
            &format!("Game Content Server ({})", config.name),
        )
        .await?;

        let content_server_url = format!("http://127.0.0.1:{}", GAME_CONTENT_PORT);
        let game_state = GameState::new(config.clone(), process_id, content_server_url.clone());
        let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(1);

        {
            let mut current = self.current_game.write();
            *current = Some(RunningGame {
                state: game_state.clone(),
                shutdown_tx: shutdown_tx.clone(),
            });
        }

        self.event_bus.game_started(
            config.name.clone(),
            process_id,
            content_server_url.clone(),
        );

        let current_game = Arc::clone(&self.current_game);
        let event_bus = Arc::clone(&self.event_bus);
        let game_name_for_lifecycle = config.name.clone();

        tauri::async_runtime::spawn(async move {
            Self::manage_game_lifecycle(
                game_process,
                http_server_process,
                shutdown_rx,
                current_game,
                event_bus,
                game_name_for_lifecycle,
            )
            .await;
        });

        tracing::info!(
            game = %config.name,
            pid = ?process_id,
            content_url = %content_server_url,
            "Game started successfully"
        );

        Ok(game_state)
    }

    pub fn get_current_game(&self) -> Option<GameState> {
        self.current_game
            .read()
            .as_ref()
            .map(|running| running.state.clone())
    }

    pub async fn stop_game(&self) -> GameResult<()> {
        let current = self.current_game.read();
        if current.is_none() {
            return Err(GameApplicationError::NoGameRunning);
        }

        let game_name = current.as_ref().unwrap().state.config.name.clone();
        let shutdown_tx = current.as_ref().unwrap().shutdown_tx.clone();

        tracing::info!(
            game = %game_name,
            "Stopping game (process will be terminated)"
        );

        let _ = shutdown_tx.send(());

        Ok(())
    }

    async fn manage_game_lifecycle(
        game_process: GameProcess,
        mut http_server_process: Child,
        mut shutdown_rx: broadcast::Receiver<()>,
        current_game: Arc<RwLock<Option<RunningGame>>>,
        event_bus: Arc<EventBus>,
        game_name: String,
    ) {
        let pid = game_process.process_id();

        tokio::select! {
            result = game_process.wait() => {
                if let Err(e) = result {
                    tracing::error!(
                        game = %game_name,
                        error = %e,
                        "Error waiting for game process"
                    );
                }
                tracing::info!(game = %game_name, "Game process exited naturally");
            }
            _ = shutdown_rx.recv() => {
                tracing::info!(game = %game_name, "Manual shutdown requested, killing game process");
                if let Some(process_id) = pid {
                    let _ = HiddenCommandSync::new("taskkill")
                        .args(["/F", "/T", "/PID", &process_id.to_string()])
                        .output();
                }
            }
        }

        tracing::info!(
            game = %game_name,
            "Stopping Python HTTP server"
        );

        if let Err(e) = http_server_process.kill().await {
            tracing::error!(
                game = %game_name,
                error = %e,
                "Failed to kill HTTP server process"
            );
        }

        {
            let mut current = current_game.write();
            *current = None;
        }

        event_bus.game_stopped(game_name.clone());

        tracing::info!(
            game = %game_name,
            "Game lifecycle ended, cleanup complete"
        );
    }
}

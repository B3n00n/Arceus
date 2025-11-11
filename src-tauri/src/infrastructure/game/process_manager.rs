use crate::core::{error::ArceusError, Result};
use crate::domain::models::GameConfig;
use std::process::Stdio;
use tokio::process::{Child, Command};

/// Manages the lifecycle of a Unity game process
pub struct GameProcessManager {
    config: GameConfig,
}

impl GameProcessManager {
    pub fn new(config: GameConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self) -> Result<GameProcess> {
        tracing::info!(
            game = %self.config.name,
            exe = ?self.config.exe_path,
            "Starting game process"
        );

        if !self.config.exe_path.exists() {
            return Err(ArceusError::Config(format!(
                "Game executable not found: {:?}",
                self.config.exe_path
            )));
        }

        let exe_dir = self.config.exe_path.parent().ok_or_else(|| {
            ArceusError::Config(format!(
                "Cannot determine parent directory of executable: {:?}",
                self.config.exe_path
            ))
        })?;

        let child = Command::new(&self.config.exe_path)
            .current_dir(exe_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .spawn()
            .map_err(|e| {
                ArceusError::Config(format!(
                    "Failed to spawn game process: {} (path: {:?})",
                    e, self.config.exe_path
                ))
            })?;

        let process_id = child.id();

        tracing::info!(
            game = %self.config.name,
            pid = ?process_id,
            "Game process started"
        );

        Ok(GameProcess::new(child, self.config.name.clone()))
    }
}

pub struct GameProcess {
    child: Child,
    game_name: String,
}

impl GameProcess {
    fn new(child: Child, game_name: String) -> Self {
        Self {
            child,
            game_name,
        }
    }

    pub fn process_id(&self) -> Option<u32> {
        self.child.id()
    }

    pub async fn wait(mut self) -> Result<()> {
        let pid = self.process_id();

        tracing::info!(
            game = %self.game_name,
            pid = ?pid,
            "Waiting for game process to exit"
        );

        let status = self.child.wait().await.map_err(|e| {
            ArceusError::Config(format!(
                "Error waiting for game process: {}",
                e
            ))
        })?;

        tracing::info!(
            game = %self.game_name,
            pid = ?pid,
            exit_code = ?status.code(),
            "Game process exited"
        );

        Ok(())
    }
}

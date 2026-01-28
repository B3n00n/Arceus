use crate::{
    error::{AppError, Result},
    models::{ArcadeConfigResponse, GameAssignmentResponse},
    repositories::{ArcadeRepository, GameRepository},
    services::GcsService,
};
use std::sync::Arc;

pub struct ArcadeService {
    arcade_repo: Arc<ArcadeRepository>,
    game_repo: Arc<GameRepository>,
    gcs_service: Arc<GcsService>,
}

impl ArcadeService {
    pub fn new(arcade_repo: Arc<ArcadeRepository>, game_repo: Arc<GameRepository>, gcs_service: Arc<GcsService>) -> Self {
        Self {
            arcade_repo,
            game_repo,
            gcs_service,
        }
    }

    /// Authenticate and get arcade configuration
    pub async fn get_arcade_config(&self, machine_id: &str) -> Result<ArcadeConfigResponse> {
        // Find arcade by machine ID
        let arcade = self
            .arcade_repo
            .find_by_machine_id(machine_id)
            .await?
            .ok_or(AppError::InvalidMachineId)?;

        // Update last seen
        self.arcade_repo.update_last_seen(arcade.id).await?;

        Ok(arcade.into())
    }

    /// Get all game versions available to an arcade (based on its channel)
    pub async fn get_arcade_games(&self, machine_id: &str) -> Result<Vec<GameAssignmentResponse>> {
        // Authenticate arcade
        let arcade = self
            .arcade_repo
            .find_by_machine_id(machine_id)
            .await?
            .ok_or(AppError::InvalidMachineId)?;

        // Update last seen
        self.arcade_repo.update_last_seen(arcade.id).await?;

        // Get all versions available to this arcade based on its channel
        let available_versions = self.game_repo.get_arcade_available_games(arcade.id).await?;

        // Build response with full game and version details
        let mut responses = Vec::new();

        for version in available_versions {
            // Get game info
            let game = self
                .game_repo
                .get_game_by_id(version.game_id)
                .await?
                .ok_or(AppError::GameNotFound)?;

            // Generate signed URL for background image
            let background_image_url = {
                let bg_path = format!("{}/{}BG.jpg", game.name, game.name);
                self.gcs_service
                    .generate_signed_download_url(&bg_path)
                    .await
                    .ok()
            };

            responses.push(GameAssignmentResponse {
                game_id: game.id,
                game_name: game.name.clone(),
                assigned_version: version.into(),
                background_image_url,
            });
        }

        Ok(responses)
    }

    /// Update the installed games JSON for an arcade
    pub async fn update_installed_games(
        &self,
        machine_id: &str,
        games_json: serde_json::Value,
    ) -> Result<()> {
        // Authenticate arcade
        let arcade = self
            .arcade_repo
            .find_by_machine_id(machine_id)
            .await?
            .ok_or(AppError::InvalidMachineId)?;

        // Update the installed_games field
        self.arcade_repo
            .update_installed_games(arcade.id, games_json)
            .await?;

        // Update last seen
        self.arcade_repo.update_last_seen(arcade.id).await?;

        Ok(())
    }
}

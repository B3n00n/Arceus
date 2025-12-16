use crate::{
    error::{AppError, Result},
    models::{ArcadeConfigResponse, GameAssignmentResponse},
    repositories::{ArcadeRepository, GameRepository},
};

pub struct ArcadeService {
    arcade_repo: ArcadeRepository,
    game_repo: GameRepository,
}

impl ArcadeService {
    pub fn new(arcade_repo: ArcadeRepository, game_repo: GameRepository) -> Self {
        Self {
            arcade_repo,
            game_repo,
        }
    }

    /// Authenticate and get arcade configuration
    pub async fn get_arcade_config(&self, mac_address: &str) -> Result<ArcadeConfigResponse> {
        // Find arcade by MAC address
        let arcade = self
            .arcade_repo
            .find_by_mac_address(mac_address)
            .await?
            .ok_or(AppError::InvalidMacAddress)?;

        // Update last seen
        self.arcade_repo.update_last_seen(arcade.id).await?;

        Ok(arcade.into())
    }

    /// Get all game assignments for an arcade
    pub async fn get_arcade_games(&self, mac_address: &str) -> Result<Vec<GameAssignmentResponse>> {
        // Authenticate arcade
        let arcade = self
            .arcade_repo
            .find_by_mac_address(mac_address)
            .await?
            .ok_or(AppError::InvalidMacAddress)?;

        // Update last seen
        self.arcade_repo.update_last_seen(arcade.id).await?;

        // Get all assignments
        let assignments = self.game_repo.get_arcade_assignments(arcade.id).await?;

        // Build response with full game and version details
        let mut responses = Vec::new();

        for assignment in assignments {
            // Get game info
            let game = self
                .game_repo
                .get_game_by_id(assignment.game_id)
                .await?
                .ok_or(AppError::GameNotFound)?;

            // Get assigned version
            let assigned_version = self
                .game_repo
                .get_version_by_id(assignment.assigned_version_id)
                .await?
                .ok_or(AppError::GameVersionNotFound)?;

            // Get current version if exists
            let current_version = if let Some(current_id) = assignment.current_version_id {
                self.game_repo
                    .get_version_by_id(current_id)
                    .await?
                    .map(|v| v.into())
            } else {
                None
            };

            responses.push(GameAssignmentResponse {
                game_id: game.id,
                game_name: game.name,
                assigned_version: assigned_version.into(),
                current_version,
            });
        }

        Ok(responses)
    }

    /// Update current version status for a game
    pub async fn update_game_status(
        &self,
        mac_address: &str,
        game_id: i32,
        current_version_id: Option<i32>,
    ) -> Result<()> {
        // Authenticate arcade
        let arcade = self
            .arcade_repo
            .find_by_mac_address(mac_address)
            .await?
            .ok_or(AppError::InvalidMacAddress)?;

        // Verify the version exists if provided
        if let Some(version_id) = current_version_id {
            let version = self
                .game_repo
                .get_version_by_id(version_id)
                .await?
                .ok_or(AppError::GameVersionNotFound)?;

            // Verify version belongs to the game
            if version.game_id != game_id {
                return Err(AppError::Internal(
                    "Version does not belong to specified game".to_string(),
                ));
            }
        }

        // Update the current version
        self.game_repo
            .update_current_version(arcade.id, game_id, current_version_id)
            .await?;

        Ok(())
    }
}

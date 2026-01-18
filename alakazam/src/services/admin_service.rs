use crate::{
    error::{AppError, Result},
    models::{Arcade, ArcadeGameAssignment, Game, GameVersion},
    repositories::{ArcadeRepository, GameRepository},
};
use std::sync::Arc;

pub struct AdminService {
    arcade_repo: Arc<ArcadeRepository>,
    game_repo: Arc<GameRepository>,
}

impl AdminService {
    pub fn new(arcade_repo: Arc<ArcadeRepository>, game_repo: Arc<GameRepository>) -> Self {
        Self {
            arcade_repo,
            game_repo,
        }
    }

    pub async fn create_arcade(&self, name: &str, machine_id: &str) -> Result<Arcade> {
        self.arcade_repo.create(name, machine_id, "active").await
    }

    pub async fn list_arcades(&self) -> Result<Vec<Arcade>> {
        self.arcade_repo.list_all().await
    }

    pub async fn get_arcade(&self, id: i32) -> Result<Arcade> {
        self.arcade_repo
            .get_by_id(id)
            .await?
            .ok_or(AppError::ArcadeNotFound)
    }

    pub async fn update_arcade(&self, id: i32, name: &str, status: &str) -> Result<Arcade> {
        self.get_arcade(id).await?;

        self.arcade_repo.update(id, name, status).await
    }

    pub async fn delete_arcade(&self, id: i32) -> Result<()> {
        self.get_arcade(id).await?;

        // Note: CASCADE delete will remove assignments automatically
        self.arcade_repo.delete(id).await
    }

    pub async fn get_arcade_assignments(&self, arcade_id: i32) -> Result<Vec<ArcadeGameAssignment>> {
        self.get_arcade(arcade_id).await?;

        self.game_repo.get_arcade_assignments(arcade_id).await
    }

    pub async fn create_game(&self, name: &str) -> Result<Game> {
        self.game_repo.create_game(name).await
    }

    pub async fn list_games(&self) -> Result<Vec<Game>> {
        self.game_repo.list_all_games().await
    }

    pub async fn get_game(&self, id: i32) -> Result<Game> {
        self.game_repo
            .get_game_by_id(id)
            .await?
            .ok_or(AppError::GameNotFound)
    }

    pub async fn update_game(&self, id: i32, name: &str) -> Result<Game> {
        self.get_game(id).await?;

        self.game_repo.update_game(id, name).await
    }

    pub async fn delete_game(&self, id: i32) -> Result<()> {
        self.get_game(id).await?;

        // CASCADE delete will remove versions and assignments
        self.game_repo.delete_game(id).await
    }

    pub async fn create_game_version(
        &self,
        game_id: i32,
        version: &str,
        gcs_path: &str,
    ) -> Result<GameVersion> {
        self.get_game(game_id).await?;

        self.game_repo.create_version(game_id, version, gcs_path).await
    }

    pub async fn list_game_versions(&self, game_id: i32) -> Result<Vec<GameVersion>> {
        self.get_game(game_id).await?;

        self.game_repo.list_versions_by_game(game_id).await
    }

    pub async fn get_game_version(&self, version_id: i32) -> Result<GameVersion> {
        self.game_repo
            .get_version_by_id(version_id)
            .await?
            .ok_or(AppError::GameVersionNotFound)
    }

    pub async fn update_game_version(
        &self,
        version_id: i32,
        version: &str,
        gcs_path: &str,
    ) -> Result<GameVersion> {
        self.get_game_version(version_id).await?;

        self.game_repo.update_version(version_id, version, gcs_path).await
    }

    pub async fn delete_game_version(&self, version_id: i32) -> Result<()> {
        let _game_version = self.get_game_version(version_id).await?;

        // Check if version is assigned to any arcade
        let all_assignments = self.game_repo.list_all_assignments().await?;
        let is_assigned = all_assignments.iter().any(|a|
            a.assigned_version_id == version_id ||
            a.current_version_id == Some(version_id)
        );

        if is_assigned {
            return Err(AppError::Internal(
                "Cannot delete game version that is assigned to arcades".to_string()
            ));
        }

        self.game_repo.delete_version(version_id).await
    }

    pub async fn create_assignment(
        &self,
        arcade_id: i32,
        game_id: i32,
        assigned_version_id: i32,
    ) -> Result<ArcadeGameAssignment> {
        self.get_arcade(arcade_id).await?;

        self.get_game(game_id).await?;

        let version = self.get_game_version(assigned_version_id).await?;
        if version.game_id != game_id {
            return Err(AppError::Internal(
                "Game version does not belong to the specified game".to_string()
            ));
        }

        self.game_repo
            .create_assignment(arcade_id, game_id, assigned_version_id)
            .await
    }

    pub async fn update_assignment(
        &self,
        assignment_id: i32,
        assigned_version_id: i32,
    ) -> Result<ArcadeGameAssignment> {
        let assignment = self.game_repo
            .get_assignment_by_id(assignment_id)
            .await?
            .ok_or(AppError::NoAssignment)?;

        let version = self.get_game_version(assigned_version_id).await?;
        if version.game_id != assignment.game_id {
            return Err(AppError::Internal(
                "Game version does not belong to the assignment's game".to_string()
            ));
        }

        self.game_repo
            .update_assignment(assignment_id, assigned_version_id)
            .await
    }

    pub async fn delete_assignment(&self, assignment_id: i32) -> Result<()> {
        self.game_repo
            .get_assignment_by_id(assignment_id)
            .await?
            .ok_or(AppError::NoAssignment)?;

        self.game_repo.delete_assignment(assignment_id).await
    }

    pub async fn list_all_assignments(&self) -> Result<Vec<ArcadeGameAssignment>> {
        self.game_repo.list_all_assignments().await
    }
}

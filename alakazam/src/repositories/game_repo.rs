use crate::{
    error::Result,
    models::{ArcadeGameAssignment, Game, GameVersion},
};
use sqlx::PgPool;

pub struct GameRepository {
    pool: PgPool,
}

impl GameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get all game assignments for an arcade
    pub async fn get_arcade_assignments(&self, arcade_id: i32) -> Result<Vec<ArcadeGameAssignment>> {
        let assignments = sqlx::query_as::<_, ArcadeGameAssignment>(
            "SELECT id, arcade_id, game_id, assigned_version_id, current_version_id, updated_at
             FROM arcade_game_assignments
             WHERE arcade_id = $1"
        )
        .bind(arcade_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(assignments)
    }

    /// Get game by ID
    pub async fn get_game_by_id(&self, game_id: i32) -> Result<Option<Game>> {
        let game = sqlx::query_as::<_, Game>(
            "SELECT id, name, created_at
             FROM games
             WHERE id = $1"
        )
        .bind(game_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(game)
    }

    /// Get game version by ID
    pub async fn get_version_by_id(&self, version_id: i32) -> Result<Option<GameVersion>> {
        let version = sqlx::query_as::<_, GameVersion>(
            "SELECT id, game_id, version, gcs_path, release_date
             FROM game_versions
             WHERE id = $1"
        )
        .bind(version_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(version)
    }

    /// Update current version for an arcade game assignment
    pub async fn update_current_version(
        &self,
        arcade_id: i32,
        game_id: i32,
        version_id: Option<i32>,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE arcade_game_assignments
             SET current_version_id = $3, updated_at = NOW()
             WHERE arcade_id = $1 AND game_id = $2"
        )
        .bind(arcade_id)
        .bind(game_id)
        .bind(version_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create new game
    pub async fn create_game(&self, name: &str) -> Result<Game> {
        let game = sqlx::query_as::<_, Game>(
            "INSERT INTO games (name)
             VALUES ($1)
             RETURNING id, name, created_at"
        )
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(game)
    }

    /// List all games
    pub async fn list_all_games(&self) -> Result<Vec<Game>> {
        let games = sqlx::query_as::<_, Game>(
            "SELECT id, name, created_at
             FROM games
             ORDER BY name ASC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    /// Update game
    pub async fn update_game(&self, id: i32, name: &str) -> Result<Game> {
        let game = sqlx::query_as::<_, Game>(
            "UPDATE games
             SET name = $2
             WHERE id = $1
             RETURNING id, name, created_at"
        )
        .bind(id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(game)
    }

    /// Delete game
    pub async fn delete_game(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM games WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Create new game version
    pub async fn create_version(&self, game_id: i32, version: &str, gcs_path: &str) -> Result<GameVersion> {
        let game_version = sqlx::query_as::<_, GameVersion>(
            "INSERT INTO game_versions (game_id, version, gcs_path)
             VALUES ($1, $2, $3)
             RETURNING id, game_id, version, gcs_path, release_date"
        )
        .bind(game_id)
        .bind(version)
        .bind(gcs_path)
        .fetch_one(&self.pool)
        .await?;

        Ok(game_version)
    }

    /// List all versions for a game
    pub async fn list_versions_by_game(&self, game_id: i32) -> Result<Vec<GameVersion>> {
        let versions = sqlx::query_as::<_, GameVersion>(
            "SELECT id, game_id, version, gcs_path, release_date
             FROM game_versions
             WHERE game_id = $1
             ORDER BY release_date DESC"
        )
        .bind(game_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(versions)
    }

    /// Update game version
    pub async fn update_version(&self, id: i32, version: &str, gcs_path: &str) -> Result<GameVersion> {
        let game_version = sqlx::query_as::<_, GameVersion>(
            "UPDATE game_versions
             SET version = $2, gcs_path = $3
             WHERE id = $1
             RETURNING id, game_id, version, gcs_path, release_date"
        )
        .bind(id)
        .bind(version)
        .bind(gcs_path)
        .fetch_one(&self.pool)
        .await?;

        Ok(game_version)
    }

    /// Delete game version
    pub async fn delete_version(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM game_versions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Create arcade game assignment
    pub async fn create_assignment(
        &self,
        arcade_id: i32,
        game_id: i32,
        assigned_version_id: i32,
    ) -> Result<ArcadeGameAssignment> {
        let assignment = sqlx::query_as::<_, ArcadeGameAssignment>(
            "INSERT INTO arcade_game_assignments (arcade_id, game_id, assigned_version_id)
             VALUES ($1, $2, $3)
             RETURNING id, arcade_id, game_id, assigned_version_id, current_version_id, updated_at"
        )
        .bind(arcade_id)
        .bind(game_id)
        .bind(assigned_version_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(assignment)
    }

    /// Update assignment (change assigned version)
    pub async fn update_assignment(
        &self,
        id: i32,
        assigned_version_id: i32,
    ) -> Result<ArcadeGameAssignment> {
        let assignment = sqlx::query_as::<_, ArcadeGameAssignment>(
            "UPDATE arcade_game_assignments
             SET assigned_version_id = $2, updated_at = NOW()
             WHERE id = $1
             RETURNING id, arcade_id, game_id, assigned_version_id, current_version_id, updated_at"
        )
        .bind(id)
        .bind(assigned_version_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(assignment)
    }

    /// Get assignment by ID
    pub async fn get_assignment_by_id(&self, id: i32) -> Result<Option<ArcadeGameAssignment>> {
        let assignment = sqlx::query_as::<_, ArcadeGameAssignment>(
            "SELECT id, arcade_id, game_id, assigned_version_id, current_version_id, updated_at
             FROM arcade_game_assignments
             WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(assignment)
    }

    /// Delete assignment
    pub async fn delete_assignment(&self, id: i32) -> Result<()> {
        sqlx::query("DELETE FROM arcade_game_assignments WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// List all assignments
    pub async fn list_all_assignments(&self) -> Result<Vec<ArcadeGameAssignment>> {
        let assignments = sqlx::query_as::<_, ArcadeGameAssignment>(
            "SELECT id, arcade_id, game_id, assigned_version_id, current_version_id, updated_at
             FROM arcade_game_assignments
             ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(assignments)
    }
}

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
}

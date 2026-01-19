use crate::{
    error::Result,
    models::{ChannelInfo, Game, GameVersion, GameVersionWithChannels},
};
use sqlx::PgPool;

pub struct GameRepository {
    pool: PgPool,
}

impl GameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // ========================================================================
    // GAME CRUD
    // ========================================================================

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

    // ========================================================================
    // GAME VERSION CRUD
    // ========================================================================

    /// Create new game version (unpublished by default)
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

    /// Get version with channels included
    pub async fn get_version_with_channels(&self, version_id: i32) -> Result<Option<GameVersionWithChannels>> {
        // First get the version
        let version = match self.get_version_by_id(version_id).await? {
            Some(v) => v,
            None => return Ok(None),
        };

        // Then get its channels
        let channels = sqlx::query_as::<_, ChannelInfo>(
            "SELECT rc.id, rc.name
             FROM game_version_channels gvc
             JOIN release_channels rc ON gvc.channel_id = rc.id
             WHERE gvc.version_id = $1
             ORDER BY rc.id ASC"
        )
        .bind(version_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(GameVersionWithChannels {
            id: version.id,
            game_id: version.game_id,
            version: version.version,
            gcs_path: version.gcs_path,
            release_date: version.release_date,
            channels,
        }))
    }

    /// List all versions for a game with their channels
    pub async fn list_versions_with_channels(&self, game_id: i32) -> Result<Vec<GameVersionWithChannels>> {
        let versions = self.list_versions_by_game(game_id).await?;

        let mut versions_with_channels = Vec::new();
        for version in versions {
            if let Some(version_with_channels) = self.get_version_with_channels(version.id).await? {
                versions_with_channels.push(version_with_channels);
            }
        }

        Ok(versions_with_channels)
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

    // ========================================================================
    // RELEASE CHANNEL OPERATIONS
    // ========================================================================

    /// Publish version to multiple channels
    pub async fn publish_version_to_channels(&self, version_id: i32, channel_ids: &[i32]) -> Result<()> {
        for channel_id in channel_ids {
            sqlx::query(
                "INSERT INTO game_version_channels (version_id, channel_id)
                 VALUES ($1, $2)
                 ON CONFLICT (version_id, channel_id) DO NOTHING"
            )
            .bind(version_id)
            .bind(channel_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// Unpublish version from all channels
    pub async fn unpublish_version_from_all_channels(&self, version_id: i32) -> Result<()> {
        sqlx::query("DELETE FROM game_version_channels WHERE version_id = $1")
            .bind(version_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Set version's channels (replaces existing)
    pub async fn set_version_channels(&self, version_id: i32, channel_ids: &[i32]) -> Result<()> {
        // Remove all existing channels
        self.unpublish_version_from_all_channels(version_id).await?;

        // Add new channels
        if !channel_ids.is_empty() {
            self.publish_version_to_channels(version_id, channel_ids).await?;
        }

        Ok(())
    }

    /// Get all game versions available to an arcade (based on arcade's channel)
    pub async fn get_arcade_available_games(&self, arcade_id: i32) -> Result<Vec<GameVersion>> {
        let results = sqlx::query_as::<_, GameVersion>(
            "SELECT DISTINCT ON (gv.game_id)
                gv.id, gv.game_id, gv.version, gv.gcs_path, gv.release_date
             FROM game_versions gv
             JOIN game_version_channels gvc ON gv.id = gvc.version_id
             JOIN arcades a ON a.channel_id = gvc.channel_id
             WHERE a.id = $1
             ORDER BY gv.game_id, gv.release_date DESC"
        )
        .bind(arcade_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }
}

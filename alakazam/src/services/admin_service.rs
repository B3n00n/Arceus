use crate::{
    error::{AppError, Result},
    models::{Arcade, Customer, Game, GameVersion, GameVersionWithChannels, ReleaseChannel},
    repositories::{ArcadeRepository, ChannelRepository, CustomerRepository, GameRepository},
};
use std::sync::Arc;

pub struct AdminService {
    arcade_repo: Arc<ArcadeRepository>,
    channel_repo: Arc<ChannelRepository>,
    customer_repo: Arc<CustomerRepository>,
    game_repo: Arc<GameRepository>,
}

impl AdminService {
    pub fn new(
        arcade_repo: Arc<ArcadeRepository>,
        channel_repo: Arc<ChannelRepository>,
        customer_repo: Arc<CustomerRepository>,
        game_repo: Arc<GameRepository>,
    ) -> Self {
        Self {
            arcade_repo,
            channel_repo,
            customer_repo,
            game_repo,
        }
    }

    // ========================================================================
    // ARCADE OPERATIONS
    // ========================================================================

    pub async fn create_arcade(&self, name: &str, machine_id: &str, channel_id: i32) -> Result<Arcade> {
        // Verify channel exists
        self.get_channel(channel_id).await?;

        self.arcade_repo.create(name, machine_id, "active", channel_id).await
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
        self.arcade_repo.delete(id).await
    }

    pub async fn get_arcade_with_games(&self, id: i32) -> Result<(Arcade, Vec<i32>)> {
        let arcade = self.get_arcade(id).await?;
        let game_ids = self.arcade_repo.get_assigned_game_ids(id).await?;
        Ok((arcade, game_ids))
    }

    pub async fn get_assigned_game_ids(&self, arcade_id: i32) -> Result<Vec<i32>> {
        self.arcade_repo.get_assigned_game_ids(arcade_id).await
    }

    pub async fn set_game_assignments(&self, arcade_id: i32, game_ids: &[i32]) -> Result<()> {
        self.arcade_repo.set_game_assignments(arcade_id, game_ids).await
    }

    pub async fn update_arcade_channel(&self, arcade_id: i32, channel_id: i32) -> Result<Arcade> {
        // Verify arcade exists
        self.get_arcade(arcade_id).await?;

        // Verify channel exists
        self.get_channel(channel_id).await?;

        // Update arcade's channel
        self.arcade_repo.update_channel(arcade_id, channel_id).await
    }

    // ========================================================================
    // CUSTOMER OPERATIONS
    // ========================================================================

    pub async fn create_customer(
        &self,
        name: &str,
        phone_number: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer> {
        self.customer_repo.create(name, phone_number, email).await
    }

    pub async fn list_customers(&self) -> Result<Vec<Customer>> {
        self.customer_repo.list_all().await
    }

    pub async fn get_customer(&self, id: i32) -> Result<Customer> {
        self.customer_repo
            .get_by_id(id)
            .await?
            .ok_or(AppError::CustomerNotFound)
    }

    pub async fn update_customer(
        &self,
        id: i32,
        name: &str,
        phone_number: Option<&str>,
        email: Option<&str>,
    ) -> Result<Customer> {
        self.get_customer(id).await?;
        self.customer_repo.update(id, name, phone_number, email).await
    }

    pub async fn delete_customer(&self, id: i32) -> Result<()> {
        self.get_customer(id).await?;

        // Check if customer has arcades assigned
        if self.customer_repo.has_arcades(id).await? {
            return Err(AppError::CustomerHasArcades);
        }

        self.customer_repo.delete(id).await
    }

    pub async fn get_customer_arcade_ids(&self, customer_id: i32) -> Result<Vec<i32>> {
        self.customer_repo.get_arcade_ids(customer_id).await
    }

    pub async fn get_customer_with_arcade_ids(&self, id: i32) -> Result<(Customer, Vec<i32>)> {
        let customer = self.get_customer(id).await?;
        let arcade_ids = self.customer_repo.get_arcade_ids(id).await?;
        Ok((customer, arcade_ids))
    }

    pub async fn set_customer_arcades(&self, customer_id: i32, arcade_ids: &[i32]) -> Result<()> {
        // Verify customer exists
        self.get_customer(customer_id).await?;

        // Set arcade assignments
        self.customer_repo.set_arcade_assignments(customer_id, arcade_ids).await
    }

    // ========================================================================
    // RELEASE CHANNEL OPERATIONS
    // ========================================================================

    pub async fn list_channels(&self) -> Result<Vec<ReleaseChannel>> {
        self.channel_repo.list_all().await
    }

    pub async fn get_channel(&self, id: i32) -> Result<ReleaseChannel> {
        self.channel_repo
            .get_by_id(id)
            .await?
            .ok_or(AppError::ChannelNotFound)
    }

    pub async fn create_channel(&self, name: &str, description: Option<&str>) -> Result<ReleaseChannel> {
        self.channel_repo.create(name, description).await
    }

    pub async fn update_channel(
        &self,
        id: i32,
        description: Option<&str>,
    ) -> Result<ReleaseChannel> {
        self.get_channel(id).await?;
        self.channel_repo.update(id, description).await
    }

    pub async fn delete_channel(&self, id: i32) -> Result<()> {
        self.get_channel(id).await?;

        // This will fail with FK constraint if arcades or versions use it
        self.channel_repo.delete(id).await
    }

    // ========================================================================
    // GAME OPERATIONS
    // ========================================================================

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
        // CASCADE delete will remove versions
        self.game_repo.delete_game(id).await
    }

    // ========================================================================
    // GAME VERSION OPERATIONS
    // ========================================================================

    pub async fn create_game_version(
        &self,
        game_id: i32,
        version: &str,
        gcs_path: &str,
    ) -> Result<GameVersion> {
        self.get_game(game_id).await?;
        self.game_repo.create_version(game_id, version, gcs_path).await
    }

    pub async fn list_game_versions_with_channels(&self, game_id: i32) -> Result<Vec<GameVersionWithChannels>> {
        self.get_game(game_id).await?;
        self.game_repo.list_versions_with_channels(game_id).await
    }

    pub async fn get_game_version(&self, version_id: i32) -> Result<GameVersion> {
        self.game_repo
            .get_version_by_id(version_id)
            .await?
            .ok_or(AppError::GameVersionNotFound)
    }

    pub async fn get_game_version_with_channels(&self, version_id: i32) -> Result<GameVersionWithChannels> {
        self.game_repo
            .get_version_with_channels(version_id)
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
        self.get_game_version(version_id).await?;
        // CASCADE delete will remove channel associations
        self.game_repo.delete_version(version_id).await
    }

    // ========================================================================
    // CHANNEL PUBLISHING OPERATIONS
    // ========================================================================

    /// Replace all channels a version is published to (removes existing, sets new)
    pub async fn replace_version_channels(&self, version_id: i32, channel_ids: &[i32]) -> Result<GameVersionWithChannels> {
        // Verify version exists
        self.get_game_version(version_id).await?;

        // Verify all channels exist
        for channel_id in channel_ids {
            self.get_channel(*channel_id).await?;
        }

        // Replace channels
        self.game_repo.set_version_channels(version_id, channel_ids).await?;

        // Return version with channels
        self.get_game_version_with_channels(version_id).await
    }

    /// Unpublish version from all channels
    pub async fn unpublish_version_from_all(&self, version_id: i32) -> Result<()> {
        // Verify version exists
        self.get_game_version(version_id).await?;

        // Unpublish from all channels
        self.game_repo.unpublish_version_from_all_channels(version_id).await
    }
}

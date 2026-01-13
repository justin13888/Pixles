use async_graphql::*;
use chrono::{DateTime, Utc};
use entity::share_link::Model as ShareLinkModel;
use entity::share_link::ShareLinkType as EntityShareLinkType;

use super::album::Album;
use super::asset::AssetMetadata;

/// Type of content being shared
#[derive(Enum, Clone, Copy, Eq, PartialEq)]
pub enum ShareLinkType {
    Album,
    Asset,
    Selection,
}

impl From<EntityShareLinkType> for ShareLinkType {
    fn from(t: EntityShareLinkType) -> Self {
        match t {
            EntityShareLinkType::Album => ShareLinkType::Album,
            EntityShareLinkType::Asset => ShareLinkType::Asset,
            EntityShareLinkType::Selection => ShareLinkType::Selection,
        }
    }
}

/// A public share link for sharing content externally
pub struct ShareLink {
    pub model: ShareLinkModel,
}

#[Object]
impl ShareLink {
    async fn id(&self) -> ID {
        ID::from(&self.model.id)
    }

    async fn token(&self) -> &String {
        &self.model.token
    }

    /// Full URL for sharing
    async fn url(&self) -> String {
        // TODO: Get base URL from config
        format!("https://pixles.app/s/{}", self.model.token)
    }

    async fn share_type(&self) -> ShareLinkType {
        self.model.share_type.clone().into()
    }

    async fn allow_download(&self) -> bool {
        self.model.allow_download
    }

    async fn password_protected(&self) -> bool {
        self.model.password_hash.is_some()
    }

    async fn expires_at(&self) -> Option<DateTime<Utc>> {
        self.model.expires_at
    }

    async fn view_count(&self) -> i32 {
        self.model.view_count
    }

    async fn created_at(&self) -> DateTime<Utc> {
        self.model.created_at
    }

    // TODO: Resolve album/assets via dataloader based on share_type
    // async fn album(&self) -> Option<Album>
    // async fn assets(&self) -> Vec<AssetMetadata>
}

// ===== Inputs =====

#[derive(InputObject)]
pub struct CreateShareLinkInput {
    /// Album ID (for album shares)
    pub album_id: Option<ID>,
    /// Asset IDs (for asset or selection shares)
    pub asset_ids: Option<Vec<ID>>,
    /// Whether downloads are allowed (default: true)
    pub allow_download: Option<bool>,
    /// Optional password protection
    pub password: Option<String>,
    /// Optional expiration date
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(InputObject)]
pub struct UpdateShareLinkInput {
    pub allow_download: Option<bool>,
    /// Set to null to remove password
    pub password: Option<String>,
    /// Set to null to remove expiration
    pub expires_at: Option<DateTime<Utc>>,
}

// ===== Query =====

#[derive(Default)]
pub struct ShareQuery;

#[Object]
impl ShareQuery {
    /// Get all share links created by current user
    async fn my_links(&self, ctx: &Context<'_>) -> Result<Vec<ShareLink>> {
        todo!("Implement list user's share links")
    }

    /// Get share link by ID
    async fn by_id(&self, ctx: &Context<'_>, id: ID) -> Result<ShareLink> {
        todo!("Implement get share link by ID")
    }
}

// ===== Mutations =====

#[derive(Default)]
pub struct ShareMutation;

#[Object]
impl ShareMutation {
    /// Create a public share link
    async fn create_share_link(
        &self,
        ctx: &Context<'_>,
        input: CreateShareLinkInput,
    ) -> Result<ShareLink> {
        todo!("Implement create share link")
    }

    /// Update share link settings
    async fn update_share_link(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateShareLinkInput,
    ) -> Result<ShareLink> {
        todo!("Implement update share link")
    }

    /// Revoke/delete a share link
    async fn delete_share_link(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        todo!("Implement delete share link")
    }
}

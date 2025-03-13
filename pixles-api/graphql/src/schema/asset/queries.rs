use crate::schema::{user::User, SortDirection};

use super::{AssetFilter, AssetMetadata, AssetSort, AssetType, UploadSession, UploadSessionFilter};
use async_graphql::*;
use chrono::Utc;

pub struct AssetQuery;

#[Object]
impl AssetQuery {
    /// Get asset metadata by ID
    async fn by_id(&self, ctx: &Context<'_>, id: ID) -> Result<AssetMetadata> {
        todo!()
    }

    /// Search assets based on album ID, filter, and pagination
    /// If sorting without a direction, assumes ascending order
    async fn search(
        &self,
        ctx: &Context<'_>,
        album_ids: Option<Vec<ID>>,
        filter: Option<AssetFilter>,
        sort: Option<AssetSort>,
        sort_direction: Option<SortDirection>,
    ) -> Result<Vec<AssetMetadata>> {
        // TODO: Add sorting and pagination
        // TODO: Generate presigned URL for each asset on-the-fly
        // TODO: Sort appropriately
        // Ok(vec![AssetMetadata::new(
        //     ID::from("1"),
        //     AssetType::Photo,
        //     "test.jpg".to_string(),
        //     100,
        //     "/test.jpg".to_string(),
        //     100,
        //     100,
        //     Utc::now(),
        //     Utc::now(),
        //     Utc::now(),
        //     None,
        //     vec![],
        //     User::default(),
        // )])
        Ok(vec![])
    }

    /// Get upload session by ID
    async fn upload_session(&self, ctx: &Context<'_>, id: ID) -> Result<UploadSession> {
        todo!()
    }

    /// Get all upload sessions for the current user
    async fn my_upload_sessions(
        &self,
        ctx: &Context<'_>,
        filter: UploadSessionFilter,
    ) -> Result<Vec<UploadSession>> {
        // TODO: Add pagination and sorting
        todo!()
    }

    /// Get all upload sessions. Only available to admins
    async fn all_upload_sessions(
        &self,
        ctx: &Context<'_>,
        filter: UploadSessionFilter,
    ) -> Result<Vec<UploadSession>> {
        // TODO: Add guard
        // TODO: Add pagination and sorting
        todo!()
    }
}

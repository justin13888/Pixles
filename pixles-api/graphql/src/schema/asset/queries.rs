use super::{AssetFilter, AssetMetadata, UploadSession, UploadSessionFilter};
use async_graphql::*;

pub struct AssetQuery;

#[Object]
impl AssetQuery {
    /// Get asset metadata by ID
    async fn by_id(&self, ctx: &Context<'_>, id: ID) -> Result<AssetMetadata> {
        todo!()
    }

    /// Search assets based on album ID, filter, and pagination
    async fn search(
        &self,
        ctx: &Context<'_>,
        album_id: Option<ID>,
        filter: AssetFilter,
    ) -> Result<Vec<AssetMetadata>> {
        // TODO: Add sorting and pagination
        // TODO: Generate presigned URL for each asset on-the-fly
        todo!()
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

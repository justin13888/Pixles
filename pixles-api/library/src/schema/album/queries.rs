use super::{Album, AlbumFilter};
use async_graphql::*;

pub struct AlbumQuery;

#[Object]
impl AlbumQuery {
    /// Get album by ID
    async fn by_id(&self, ctx: &Context<'_>, id: ID) -> Result<Album> {
        todo!()
    }

    /// Get all albums for the current user
    async fn mine(&self, ctx: &Context<'_>) -> Result<Vec<Album>> {
        todo!()
    }

    /// Search albums
    async fn search(&self, ctx: &Context<'_>, filter: AlbumFilter) -> Result<Vec<Album>> {
        todo!()
    }
}

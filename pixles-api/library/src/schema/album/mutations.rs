use super::{Album, CreateAlbumInput, ShareAlbumInput, UpdateAlbumInput};
use async_graphql::*;

pub struct AlbumMutation;

#[Object]
impl AlbumMutation {
    /// Create an album
    async fn create(&self, ctx: &Context<'_>, input: CreateAlbumInput) -> Result<Album> {
        todo!()
    }

    /// Update an album
    async fn update(&self, ctx: &Context<'_>, id: ID, input: UpdateAlbumInput) -> Result<Album> {
        todo!()
    }

    /// Delete an album
    async fn delete(&self, ctx: &Context<'_>, id: ID) -> Result<Album> {
        todo!()
    }

    /// Share an album
    async fn share(&self, ctx: &Context<'_>, id: ID, input: ShareAlbumInput) -> Result<Album> {
        todo!()
    }
}

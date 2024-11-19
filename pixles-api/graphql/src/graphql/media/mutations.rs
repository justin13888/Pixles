use super::types::{CreateMediaInput, Media, UpdateMediaInput};
use async_graphql::*;

pub struct MediaMutation;

#[Object]
impl MediaMutation {
    async fn create_media(&self, ctx: &Context<'_>, input: CreateMediaInput) -> Result<Media> {
        todo!()
    }

    async fn update_media(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: UpdateMediaInput,
    ) -> Result<Media> {
        todo!()
    }
}

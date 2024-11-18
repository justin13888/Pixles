use super::types::Media;
use async_graphql::*;

pub struct MediaQuery;

#[Object]
impl MediaQuery {
    async fn get_media(&self, ctx: &Context<'_>, id: ID) -> Result<Media> {
        todo!()
    }

    async fn list_medias(&self, ctx: &Context<'_>) -> Result<Vec<Media>> {
        todo!()
    }
}

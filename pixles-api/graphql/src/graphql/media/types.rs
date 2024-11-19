use async_graphql::*;

#[derive(SimpleObject)]
pub struct Media {
    id: ID,
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct CreateMediaInput {
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct UpdateMediaInput {
    name: Option<String>,
    email: Option<String>,
}

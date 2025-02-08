use async_graphql::*;

#[derive(SimpleObject)]
pub struct Admin {
    id: ID,
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct CreateAdminInput {
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct UpdateAdminInput {
    name: Option<String>,
    email: Option<String>,
}

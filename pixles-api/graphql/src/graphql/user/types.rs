use async_graphql::*;

#[derive(SimpleObject)]
pub struct User {
    id: ID,
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    name: String,
    email: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    name: Option<String>,
    email: Option<String>,
}

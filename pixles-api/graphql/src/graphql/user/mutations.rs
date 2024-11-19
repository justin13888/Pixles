use super::types::{CreateUserInput, UpdateUserInput, User};
use async_graphql::*;

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<User> {
        todo!()
    }

    async fn update_user(&self, ctx: &Context<'_>, id: ID, input: UpdateUserInput) -> Result<User> {
        todo!()
    }
}

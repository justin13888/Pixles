use super::types::User;
use async_graphql::*;

pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        todo!()
    }

    async fn list_users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        todo!()
    }
}

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

    async fn me(&self, ctx: &Context<'_>) -> Result<Option<User>> {
        // let auth_service = ctx.data::<YourAuthService>()?;

        // // Get authorization header from context
        // let token = ctx.data::<String>()?;

        // auth_service.get_current_user(token).await

        todo!()
    }
}

use super::types::User;
use async_graphql::*;
use chrono::Utc;
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
        // TODO: Implement
        Ok(User {
            id: "123".to_string(),
            username: "testuser".to_string(),
            name: "Test User".to_string(),
            email: "test@test.com".to_string(),
            account_verified: true,
            needs_onboarding: false,
            created_at: Utc::now(),
            modified_at: Utc::now(),
            deleted_at: None,
        })
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

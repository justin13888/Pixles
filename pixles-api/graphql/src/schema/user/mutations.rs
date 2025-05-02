use crate::loaders::Loaders;

use super::{
    AuthResponse, LoginUserInput, RegisterUserResponse,
    types::{RegisterUserInput, UpdateUserInput, User},
};
use async_graphql::*;
use secrecy::ExposeSecret;

pub struct UserMutation;

#[Object]
impl UserMutation {
    /// Login a user
    async fn login(&self, ctx: &Context<'_>, input: LoginUserInput) -> Result<AuthResponse> {
        let LoginUserInput { email, password } = input;

        // get_hashed_password_by_email; // TODO
        // Get your service from the context
        // let auth_service = ctx.data::<YourAuthService>()?;

        // Call your existing service function
        // let (user, token) = auth_service.login(email, password).await?;

        let token = "foo".to_string(); // TODO
        let Loaders { user_loader, .. } = ctx.data_unchecked::<Loaders>();

        // let user = user_loader
        //     .load_one(email.clone())
        //     .await
        //     .map_err(|_| "User not found")?;

        // let auth_service = ctx.data::<YourAuthService>()?;
        // // Call your existing service function
        // match auth_service.login_user(email, password).await {
        //     Ok((user, token)) => Ok(AuthResponse { token, user }),
        //     Err(_) => Err("Invalid email or password".into()),
        // }
        // TODO: Add success/failure metrics

        Ok(AuthResponse { token, user: None })
    }

    /// Update a user
    /// If user_id is not itself, requires admin privileges
    // async fn update_user(&self, ctx: &Context<'_>, user_id: ID, input: UpdateUserInput) -> Result<User> {
    //     todo!()
    // }

    // TODO: Delete user

    /// Refresh a user's token
    async fn refresh_token(&self, ctx: &Context<'_>, token: String) -> Result<AuthResponse> {
        // TODO: Implement token rejection
        // TODO: Add metric for token usage patterns
        todo!()
    }

    /// Logout the user
    /// Returns true if the user was logged out, false if they were not logged in
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        todo!()
    }

    /// Logout all sessions for a user
    /// Requires admin privileges
    async fn logout_all(&self, ctx: &Context<'_>, user_id: String) -> Result<bool> {
        todo!()
    }

    /// Revoke a specific refresh token
    /// Requires admin privileges
    async fn revoke_token(&self, ctx: &Context<'_>, token: String) -> Result<AuthResponse> {
        todo!()
        // TODO: Decide between token blacklist or whitelisting
    }

    // TODO: Add these mutations vv
    // #[graphql(guard = "RoleGuard::new(Role::Admin)")]
    // async fn oauth_start(
    //     State(state): State<AppState>,
    //     Path(provider): Path<String>,
    // ) -> impl IntoResponse {
    //     // 1. Generate PKCE challenge
    //     // 2. Store state parameter in Redis
    //     // 3. Redirect to authorization endpoint
    // }

    // async fn oauth_callback(
    //     State(state): State<AppState>,
    //     Query(params): Query<OAuthCallbackParams>,
    // ) -> impl IntoResponse {
    //     // 1. Verify state parameter
    //     // 2. Exchange code for tokens
    //     // 3. Fetch user info
    //     // 4. Create/update user record
    //     // 5. Generate JWT
    //     // 6. Redirect to client with token
    // }
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches

// TODO: Double check on implementation with best practices

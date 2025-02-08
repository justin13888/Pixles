use crate::loaders::Loaders;

use super::{
    types::{RegisterUserInput, UpdateUserInput, User},
    AuthResponse, LoginUserInput, RegisterUserResponse,
};
use async_graphql::*;
use serde::{Deserialize, Serialize};

pub struct UserMutation;

#[Object]
impl UserMutation {
    async fn register(
        &self,
        ctx: &Context<'_>,
        input: RegisterUserInput,
    ) -> Result<RegisterUserResponse> {
        let RegisterUserInput {
            name,
            email,
            password,
        } = input;

        let mut errors = vec![];

        // Validate email format
        if !email.contains('@') {
            errors.push("Invalid email format".to_string());
        }

        // Validate password strength
        if password.len() < 8 {
            errors.push("Password must be at least 8 characters long".to_string());
        } // TODO: Add more validation rules

        if !errors.is_empty() {
            return Ok(RegisterUserResponse {
                success: false,
                data: None,
                errors,
            });
        }

        // Get your service from the context
        // let auth_service = ctx.data::<YourAuthService>()?;

        // Call your existing service function
        // let (user, token) = auth_service.register_user(email, password).await?;

        let token = "foo".to_string();
        let user = User {
            id: "1".into(),
            name,
            username: "fsdf".into(),
            email,
            account_verified: false,
            created_at: chrono::Utc::now(),
            deleted_at: None,
            modified_at: chrono::Utc::now(),
            needs_onboarding: true,
        }; // TODO

        Ok(RegisterUserResponse {
            success: true,
            data: Some(AuthResponse {
                token,
                user: Some(user),
            }),
            errors: vec![],
        })
    }

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

        Ok(AuthResponse { token, user: None })
    }

    // async fn update_user(&self, ctx: &Context<'_>, id: ID, input: UpdateUserInput) -> Result<User> {
    //     todo!()
    // }
}

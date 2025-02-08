use async_graphql::*;
use chrono::{DateTime, Utc};
use entity::user;
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub email: String,
    pub account_verified: bool,
    pub needs_onboarding: bool,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<user::Model> for User {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            name: user.name,
            email: user.email,
            account_verified: user.account_verified,
            needs_onboarding: user.needs_onboarding,
            created_at: user.created_at,
            modified_at: user.modified_at,
            deleted_at: user.deleted_at,
        }
    }
}

#[derive(InputObject)]
pub struct RegisterUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginUserInput {
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct UpdateUserInput {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: Option<User>,
}

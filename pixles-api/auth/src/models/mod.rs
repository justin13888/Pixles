pub mod errors;
pub mod requests;
pub mod responses;

use secrecy::{ExposeSecret, SecretString};
use serde::Serializer;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub fn serialize_secret<S>(secret: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(secret.expose_secret())
}

pub fn serialize_secret_option<S>(
    secret: &Option<SecretString>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(s) = secret {
        serializer.serialize_str(s.expose_secret())
    } else {
        serializer.serialize_none()
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"email": "johndoe@email.com"}))]
pub struct ResetPasswordRequestPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResetPasswordPayload {
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

pub mod errors;
pub mod requests;
pub mod responses;

use schemars::JsonSchema;
use secrecy::{ExposeSecret, SecretString};
use serde::Serializer;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(example = example_reset_password_request())]
pub struct ResetPasswordRequestPayload {
    pub email: String,
}

fn example_reset_password_request() -> ResetPasswordRequestPayload {
    ResetPasswordRequestPayload {
        email: "johndoe@email.com".to_string(),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct ResetPasswordPayload {
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

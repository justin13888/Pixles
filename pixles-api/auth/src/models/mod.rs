pub mod errors;
pub mod requests;
pub mod responses;

pub use requests::*;
pub use responses::*; // Assuming we want responses exported too

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

use salvo::oapi::ToSchema;
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

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[salvo(schema(example = json!({"email": "johndoe@email.com"})))]
pub struct ResetPasswordRequestPayload {
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ResetPasswordPayload {
    pub token: String,
    pub new_password: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserProfile {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

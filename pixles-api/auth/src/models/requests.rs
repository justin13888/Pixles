use schemars::JsonSchema;
use secrecy::{SecretBox, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(example = example_register_request())]
pub struct RegisterRequest {
    pub username: String,
    pub name: String,
    pub email: String,
    #[schemars(with = "String")]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub password: SecretString,
}

fn example_register_request() -> RegisterRequest {
    RegisterRequest {
        username: "johndoe".to_string(),
        name: "John Doe".to_string(),
        email: "johndoe@email.com".to_string(),
        password: SecretString::from("password"),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
#[schemars(example = example_login_request())]
pub struct LoginRequest {
    pub email: String,
    #[schemars(with = "String")]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub password: SecretString,
}

fn example_login_request() -> LoginRequest {
    LoginRequest {
        email: "johndoe@email.com".to_string(),
        password: SecretString::from("password"),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct RefreshTokenRequest {
    #[schemars(with = "String")]
    #[serde(serialize_with = "crate::models::serialize_secret")]
    pub refresh_token: SecretString,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct UpdateProfileRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    #[schemars(with = "Option<String>")]
    #[serde(serialize_with = "crate::models::serialize_secret_option")]
    pub current_password: Option<SecretString>,
    #[schemars(with = "Option<String>")]
    #[serde(serialize_with = "crate::models::serialize_secret_option")]
    pub new_password: Option<SecretString>,
}

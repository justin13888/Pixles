pub mod errors;
pub mod requests;
pub mod responses;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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

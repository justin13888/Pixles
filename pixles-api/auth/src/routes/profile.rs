use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use model::user::UpdateUser;
use secrecy::ExposeSecret;
use service::user as UserService;

use crate::claims::Claims;
use crate::errors::ClaimValidationError;
use crate::models::UserProfile;
use crate::models::requests::UpdateProfileRequest;
use crate::models::responses::{UpdateUserProfileResponses, UserProfileResponses};
use crate::state::AppState;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::headers::get_token_from_headers;

/// Get user profile
pub async fn get_user_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> UserProfileResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return UserProfileResponses::Unauthorized(e.into()),
    };
    let token = match Claims::decode(
        token_string.expose_secret(),
        &state.config.jwt_eddsa_decoding_key,
    ) {
        Ok(token) => token,
        Err(e) => return UserProfileResponses::Unauthorized(ClaimValidationError::from(e).into()),
    };
    let user_id = token.claims.sub;

    // Fetch user profile from database
    let user_model = match UserService::Query::find_user_by_id(&state.conn, user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return UserProfileResponses::UserNotFound,
        Err(e) => return UserProfileResponses::InternalServerError(e.into()),
    };

    let profile = UserProfile {
        user_id: user_model.id,
        username: user_model.username,
        email: user_model.email,
        created_at: user_model.created_at.to_rfc3339(),
        updated_at: user_model.modified_at.to_rfc3339(),
    };

    UserProfileResponses::Success(profile)
}

/// Update user profile
pub async fn update_user_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> UpdateUserProfileResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return UpdateUserProfileResponses::Unauthorized(e.into()),
    };
    let token = match Claims::decode(
        token_string.expose_secret(),
        &state.config.jwt_eddsa_decoding_key,
    ) {
        Ok(token) => token,
        Err(e) => {
            return UpdateUserProfileResponses::Unauthorized(ClaimValidationError::from(e).into());
        }
    };
    let user_id = token.claims.sub;

    // Fetch current user to check password if needed
    let current_user = match UserService::Query::find_user_by_id(&state.conn, user_id.clone()).await
    {
        Ok(Some(user)) => user,
        Ok(None) => return UpdateUserProfileResponses::UserNotFound,
        Err(e) => return UpdateUserProfileResponses::InternalServerError(e.into()),
    };

    // Handle password change
    let new_password_hash = if let (Some(current_password), Some(new_password)) = (
        payload.current_password.as_ref(),
        payload.new_password.as_ref(),
    ) {
        match verify_password(
            current_password.expose_secret(),
            &current_user.password_hash,
        ) {
            Ok(true) => match hash_password(new_password.expose_secret()) {
                Ok(hash) => Some(hash),
                Err(e) => return UpdateUserProfileResponses::InternalServerError(e.into()),
            },
            Ok(false) => return UpdateUserProfileResponses::InvalidPassword,
            Err(e) => return UpdateUserProfileResponses::InternalServerError(e.into()),
        }
    } else {
        None
    };

    // Update user profile in database
    let updated_user = match UserService::Mutation::update_user(
        &state.conn,
        user_id,
        UpdateUser {
            username: payload.username,
            name: None, // Name update not exposed in request yet?
            email: payload.email,
            password_hash: new_password_hash,
        },
    )
    .await
    {
        Ok(user) => user,
        Err(sea_orm::DbErr::RecordNotFound(_)) => return UpdateUserProfileResponses::UserNotFound,
        Err(e) => return UpdateUserProfileResponses::InternalServerError(e.into()),
    };

    let updated_profile = UserProfile {
        user_id: updated_user.id,
        username: updated_user.username,
        email: updated_user.email,
        created_at: updated_user.created_at.to_rfc3339(),
        updated_at: updated_user.modified_at.to_rfc3339(),
    };

    UpdateUserProfileResponses::Success(updated_profile)
}

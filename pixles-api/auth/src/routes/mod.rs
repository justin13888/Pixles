use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use docs::TAGS;
use jsonwebtoken::EncodingKey;
use secrecy::ExposeSecret;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    claims::{Claims, Scope},
    error::AuthError,
    models::*,
    state::AppState,
    utils::get_token_from_headers,
};

// TODO: Verify utoipa actually matches the docs

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    tag = TAGS::AUTH,
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User successfully registered", body = TokenResponse),
        (status = 409, description = "User already exists", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn register_user(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), AuthError> {
    // In a real implementation, you would:
    // 1. Validate input
    // 2. Check if user already exists
    // 3. Hash password
    // 4. Store user in database
    // 5. Generate JWT tokens

    // TODO

    // For this example, we'll simulate a successful registration
    let user_id = "user123"; // In real app, this would be from DB

    let token_response = generate_tokens(user_id, &config.jwt_eddsa_encoding_key)?;

    Ok((StatusCode::CREATED, Json(token_response)))
}

/// Login a user
#[utoipa::path(
    post,
    path = "/login",
    tag = TAGS::AUTH,
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn login_user(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
    // In a real implementation, you would:
    // 1. Validate credentials against database
    // 2. Generate JWT tokens if valid

    // TODO: Implement login

    // For this example, we'll simulate a successful login
    // In real app, verify credentials from database
    if payload.email == "test@example.com" && payload.password == "password" {
        let user_id = "user123"; // In real app, this would be from DB

        let token_response = generate_tokens(user_id, &config.jwt_eddsa_encoding_key)?;

        Ok(Json(token_response))
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

/// Refresh an access token using a refresh token
#[utoipa::path(
    post,
    path = "/refresh",
    tag = TAGS::AUTH,
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = TokenResponse),
        (status = 401, description = "Invalid refresh token", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn refresh_token(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
    // In a real implementation, you would:
    // 1. Validate refresh token
    // 2. Check if refresh token is in whitelist or not in blacklist
    // 3. Generate new access token

    // TODO: Implement refresh token

    // For this example, we'll simulate token validation and renewal
    let token = Claims::decode(&payload.refresh_token, &config.jwt_eddsa_decoding_key)?;
    let user_id = token.claims.sub;

    let token_response = generate_tokens(&user_id, &config.jwt_eddsa_encoding_key)?;

    Ok(Json(token_response))
}

/// Validate an access token
#[utoipa::path(
    post,
    path = "/validate",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Token validation result", body = ValidateTokenResponse),
        (status = 401, description = "Invalid token", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn validate_token(
    State(AppState { config, .. }): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<ValidateTokenResponse>, AuthError> {
    // Get token string
    let token_string = get_token_from_headers(&headers).map_err(Into::<AuthError>::into)?;

    // Validate token
    match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key) {
        Ok(token) => Ok(Json(ValidateTokenResponse::Valid(token.claims.sub))),
        Err(_) => Ok(Json(ValidateTokenResponse::Invalid)),
    }
}

/// Request password reset
#[utoipa::path(
    post,
    path = "/password-reset-request",
    tag = TAGS::AUTH,
    request_body = ResetPasswordRequestPayload,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn reset_password_request(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> Result<StatusCode, AuthError> {
    // In a real implementation, you would:
    // 1. Check if user exists
    // 2. Generate a password reset token
    // 3. Store token in database with expiry
    // 4. Send email with reset link

    // TODO: Implement reset password request

    // For this example, we'll simulate successful request
    // In real app, check if email exists in database
    if payload.email == "test@example.com" {
        // In real app, send email with reset link
        Ok(StatusCode::OK)
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

/// Reset password with token
#[utoipa::path(
    post,
    path = "/password-reset",
    tag = TAGS::AUTH,
    request_body = ResetPasswordPayload,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 401, description = "Invalid or expired token", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn reset_password(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> Result<StatusCode, AuthError> {
    // In a real implementation, you would:
    // 1. Validate reset token from database
    // 2. Check if token is expired
    // 3. Update user's password
    // 4. Invalidate the reset token

    // TODO

    // For this example, we'll simulate successful password reset
    // In real app, validate token and update password in database
    if payload.token == "valid_reset_token" {
        // In real app, update password and invalidate token
        Ok(StatusCode::OK)
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

/// Get user profile
#[utoipa::path(
    get,
    path = "/profile",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "User profile retrieved", body = UserProfile),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn get_user_profile(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserProfile>, AuthError> {
    // Authorize user
    let token_string = get_token_from_headers(&headers).map_err(Into::<AuthError>::into)?;
    let token = Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
        .map_err(Into::<AuthError>::into)?;
    let user_id = token.claims.sub;

    // TODO: Implement get user profile

    // In a real app, fetch user profile from database
    let profile = UserProfile {
        user_id,
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-01-01T00:00:00Z".to_string(),
    };

    Ok(Json(profile))
}

/// Update user profile
#[utoipa::path(
    post,
    path = "/profile",
    tag = TAGS::AUTH,
    request_body = UpdateProfileRequest,
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Profile updated successfully", body = UserProfile),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 404, description = "User not found", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn update_user_profile(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfile>, AuthError> {
    // Authorize user
    let token_string = get_token_from_headers(&headers).map_err(Into::<AuthError>::into)?;
    let token = Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
        .map_err(Into::<AuthError>::into)?;
    let user_id = token.claims.sub;

    // TODO: Implement update user profile

    // In a real app, update user profile in database
    let updated_profile = UserProfile {
        user_id,
        username: payload.username.unwrap_or_else(|| "testuser".to_string()),
        email: payload
            .email
            .unwrap_or_else(|| "test@example.com".to_string()),
        created_at: "2023-01-01T00:00:00Z".to_string(),
        updated_at: "2023-05-01T00:00:00Z".to_string(), // Updated timestamp
    }; // TODO

    Ok(Json(updated_profile))
}

/// Logout user and invalidate tokens
#[utoipa::path(
    post,
    path = "/logout",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Unauthorized", body = ApiError),
        (status = 500, description = "Internal server error", body = ApiError)
    ),
    tags = ["Pixles Authentication API"]
)]
async fn logout(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> Result<StatusCode, AuthError> {
    // Authorize user
    let token_string = get_token_from_headers(&headers).map_err(Into::<AuthError>::into)?;
    let token = Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
        .map_err(Into::<AuthError>::into)?;
    let user_id = token.claims.sub;

    // TODO: Implement logout

    // In a real app, add token to blacklist or remove from whitelist
    // Here we just validate it
    // let _user_id = validate_jwt_token(token)?;
    // TODO

    Ok(StatusCode::OK)
}

/// Generate access and refresh token pairs
fn generate_tokens(user_id: &str, encoding_key: &EncodingKey) -> Result<TokenResponse, AuthError> {
    let user_id = user_id.to_string();

    // Create access token claims
    let access_claims =
        Claims::new_access_token(user_id.clone(), vec![Scope::ReadUser, Scope::WriteUser]);

    // Create refresh token claims
    let refresh_claims = Claims::new_refresh_token(user_id);

    // Generate tokens
    let access_token = access_claims
        .encode(encoding_key)
        .map_err(Into::<AuthError>::into)?;
    let access_token_expires_by = access_claims.exp;
    let refresh_token = refresh_claims
        .encode(encoding_key)
        .map_err(Into::<AuthError>::into)?;

    Ok(TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_by: access_token_expires_by,
    })
}

pub(super) fn get_router(state: AppState) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_user_profile, update_user_profile)) // Profile routes (/profile)
        .routes(routes!(register_user)) // POST /register
        .routes(routes!(login_user)) // POST /login
        .routes(routes!(refresh_token)) // POST /refresh
        .routes(routes!(validate_token)) // POST /validate
        .routes(routes!(reset_password_request)) // POST /password-reset-request
        .routes(routes!(reset_password)) // POST /password-reset
        .routes(routes!(logout)) // POST /logout
        .with_state(state)
}

// TODO: Add unit tests

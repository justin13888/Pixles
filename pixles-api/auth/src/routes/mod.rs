use axum::Json;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use docs::TAGS;
use jsonwebtoken::EncodingKey;
use secrecy::ExposeSecret;
use service::user as UserService;
use tracing::trace;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::claims::{Claims, Scope};
use crate::errors::{AuthError, ClaimValidationError};
use crate::models::requests::{
    LoginRequest, RefreshTokenRequest, RegisterRequest, UpdateProfileRequest,
};
use crate::models::responses::{
    LoginResponses, LogoutResponses, PasswordResetResponses, RefreshTokenResponses,
    RegisterUserResponses, ResetPasswordRequestResponses, TokenResponse,
    UpdateUserProfileResponses, UserProfileResponses, ValidateTokenResponse,
    ValidateTokenResponses,
};
use crate::models::{ResetPasswordPayload, ResetPasswordRequestPayload, UserProfile};
use crate::state::AppState;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::headers::get_token_from_headers;

// TODO: Verify utoipa actually matches the docs

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    tag = TAGS::AUTH,
    request_body = RegisterRequest,
    responses(RegisterUserResponses),
    tags = ["Pixles Authentication API"]
)]
async fn register_user(
    State(AppState { config, conn, .. }): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> RegisterUserResponses {
    // In a real implementation, you would:
    // 1. Validate input
    // 2. Check if user already exists
    // 3. Hash password
    // 4. Store user in database
    // 5. Generate JWT tokens

    // TODO

    // Validate request
    let RegisterRequest {
        username,
        name,
        email,
        password,
    } = request;

    // Check if user is allowed
    if !UserService::is_valid_username(&username)
    {
        trace!("Invalid username: {}", username);
        return RegisterUserResponses::InvalidUsername;
    }

    // Validate email format
    if !UserService::is_valid_email(&email)
    {
        trace!("Invalid email: {}", email);
        return RegisterUserResponses::InvalidEmail;
    }

    // Validate password strength
    if let Err(e) = UserService::is_valid_password(&password)
    {
        trace!("Invalid password: {}", e);
        return RegisterUserResponses::InvalidPassword;
    }

    // Check if email already exists
    match UserService::Query::find_user_by_email(&conn, &email).await
    {
        Ok(user) =>
        {
            if user.is_some()
            {
                trace!("User with email {} already exists", email);
                return RegisterUserResponses::UserAlreadyExists;
            }
        }
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    }

    // Check if username already exists
    let user = match UserService::Query::find_user_by_username(&conn, &username).await
    {
        Ok(user) => user,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };
    if user.is_some()
    {
        trace!("User with username {} already exists", username);
        return RegisterUserResponses::UserAlreadyExists;
    }

    // After validation, now create user
    let hashed_password = match hash_password(&password)
    {
        Ok(hashed_password) => hashed_password,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };
    let user =
        match UserService::Mutation::create_user(&conn, email, name, username, hashed_password)
            .await
        {
            Ok(user) => user,
            Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
        };
    let user_id = &user.id;

    // Generate tokens
    let token_response = match generate_tokens(&user_id, &config.jwt_eddsa_encoding_key)
    {
        Ok(token_response) => token_response,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };

    RegisterUserResponses::Success(token_response)
}

/// Login a user
#[utoipa::path(
    post,
    path = "/login",
    tag = TAGS::AUTH,
    request_body = LoginRequest,
    responses(LoginResponses),
    tags = ["Pixles Authentication API"]
)]
async fn login_user(
    State(AppState { config, conn, .. }): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> LoginResponses {
    // In a real implementation, you would:
    // 1. Validate credentials against database
    // 2. Generate JWT tokens if valid

    // TODO: Implement login
    let LoginRequest { email, password } = request;

    // Find email by email
    let user = match UserService::Query::find_user_by_email(&conn, &email).await
    {
        Ok(user) => user,
        Err(e) => return LoginResponses::InternalServerError(e.into()),
    };

    if let Some(user) = user
    {
        // Verify password
        let hashed_password = user.hashed_password;
        let is_password_valid = match verify_password(&password, &hashed_password)
        {
            Ok(is_valid) => is_valid,
            Err(e) => return LoginResponses::InternalServerError(e.into()),
        };

        if is_password_valid
        {
            match generate_tokens(&user.id, &config.jwt_eddsa_encoding_key)
            {
                Ok(token_response) => LoginResponses::Success(token_response),
                Err(e) => LoginResponses::InternalServerError(e.into()),
            }
        }
        else
        {
            LoginResponses::InvalidCredentials
        }
    }
    else
    {
        // Run dummy password hash to prevent timing attacks
        let _ = verify_password("random", "random").unwrap();
        // TODO: Unit test to verify distribution of timing is uncorrelated to password
        // correctness

        LoginResponses::InvalidCredentials
    }
}

/// Refresh an access token using a refresh token
#[utoipa::path(
    post,
    path = "/refresh",
    tag = TAGS::AUTH,
    request_body = RefreshTokenRequest,
    responses(RefreshTokenResponses),
    tags = ["Pixles Authentication API"]
)]
async fn refresh_token(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> RefreshTokenResponses {
    // In a real implementation, you would:
    // 1. Validate refresh token
    // 2. Check if refresh token is in whitelist (to keep track of active sessions
    //    and allow logout all devices)
    // 3. Generate new access token

    // TODO: Implement refresh token
    // TODO: Implement token rejection
    // TODO: Add metric for token usage patterns

    // For this example, we'll simulate token validation and renewal
    let token = match Claims::decode(&payload.refresh_token, &config.jwt_eddsa_decoding_key)
    {
        Ok(token) => token,
        Err(e) =>
        {
            return RefreshTokenResponses::InvalidRefreshToken(ClaimValidationError::from(e).into())
        }
    };
    let user_id = token.claims.sub;

    let token_response = match generate_tokens(&user_id, &config.jwt_eddsa_encoding_key)
    {
        Ok(token_response) => token_response,
        Err(e) => return RefreshTokenResponses::InternalServerError(e.into()),
    };

    RefreshTokenResponses::Success(token_response)
}

/// Validate an access token
#[utoipa::path(
    post,
    path = "/validate",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(ValidateTokenResponses),
    tags = ["Pixles Authentication API"]
)]
async fn validate_token(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> ValidateTokenResponses {
    // Get token string
    let token_string = match get_token_from_headers(&headers)
    {
        Ok(token_string) => token_string,
        Err(e) => return ValidateTokenResponses::Invalid(e.into()),
    };

    // Validate token
    match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
    {
        Ok(token) => ValidateTokenResponses::Valid(ValidateTokenResponse::Valid(token.claims.sub)),
        Err(e) => ValidateTokenResponses::Invalid(ClaimValidationError::from(e).into()),
    }
}

/// Request password reset
#[utoipa::path(
    post,
    path = "/password-reset-request",
    tag = TAGS::AUTH,
    request_body = ResetPasswordRequestPayload,
    responses(ResetPasswordRequestResponses),
    tags = ["Pixles Authentication API"]
)]
async fn reset_password_request(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordRequestPayload>,
) -> ResetPasswordRequestResponses {
    // In a real implementation, you would:
    // 1. Check if user exists
    // 2. Generate a password reset token
    // 3. Store token in database with expiry
    // 4. Send email with reset link

    // TODO: Implement reset password request

    // For this example, we'll simulate successful request
    // In real app, check if email exists in database
    if payload.email == "test@example.com"
    {
        // In real app, send email with reset link
    }
    else
    {
        todo!()
        // TODO: Ensure it doesn't leak if email exists with consistent response
        // time
    }

    ResetPasswordRequestResponses::Success
}

/// Reset password with token
#[utoipa::path(
    post,
    path = "/password-reset",
    tag = TAGS::AUTH,
    request_body = ResetPasswordPayload,
    responses(PasswordResetResponses),
    tags = ["Pixles Authentication API"]
)]
async fn reset_password(
    State(AppState { config, .. }): State<AppState>,
    Json(payload): Json<ResetPasswordPayload>,
) -> PasswordResetResponses {
    // In a real implementation, you would:
    // 1. Validate reset token from database
    // 2. Check if token is expired
    // 3. Update user's password
    // 4. Invalidate the reset token

    // TODO

    // For this example, we'll simulate successful password reset
    // In real app, validate token and update password in database
    if payload.token == "valid_reset_token"
    {
        // In real app, update password and invalidate token
        PasswordResetResponses::Success
    }
    else
    {
        PasswordResetResponses::InvalidToken
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
    responses(UserProfileResponses),
    tags = ["Pixles Authentication API"]
)]
async fn get_user_profile(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> UserProfileResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers)
    {
        Ok(token_string) => token_string,
        Err(e) => return UserProfileResponses::Unauthorized(e.into()),
    };
    let token = match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
    {
        Ok(token) => token,
        Err(e) => return UserProfileResponses::Unauthorized(ClaimValidationError::from(e).into()),
    };
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

    UserProfileResponses::Success(profile)
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
    responses(UpdateUserProfileResponses),
    tags = ["Pixles Authentication API"]
)]
async fn update_user_profile(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> UpdateUserProfileResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers)
    {
        Ok(token_string) => token_string,
        Err(e) => return UpdateUserProfileResponses::Unauthorized(e.into()),
    };
    let token = match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
    {
        Ok(token) => token,
        Err(e) =>
        {
            return UpdateUserProfileResponses::Unauthorized(ClaimValidationError::from(e).into())
        }
    };
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

    UpdateUserProfileResponses::Success(updated_profile)
}

/// Logout user and invalidate tokens
#[utoipa::path(
    post,
    path = "/logout",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(LogoutResponses),
    tags = ["Pixles Authentication API"]
)]
async fn logout(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> LogoutResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers)
    {
        Ok(token_string) => token_string,
        Err(e) => return LogoutResponses::Unauthorized(e.into()),
    };
    let token = match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key)
    {
        Ok(token) => token,
        Err(e) => return LogoutResponses::Unauthorized(ClaimValidationError::from(e).into()),
    };
    let user_id = token.claims.sub;

    // TODO: Implement logout

    // In a real app, add token to blacklist or remove from whitelist
    // Here we just validate it
    // let _user_id = validate_jwt_token(token)?;
    // TODO

    LogoutResponses::Success
}

/// testing
#[derive(utoipa::IntoResponses)]
enum TestingResponses {
    #[response(status = 200, description = "Testing successful")]
    Success,
}

impl axum::response::IntoResponse for TestingResponses {
    fn into_response(self) -> axum::response::Response {
        match self
        {
            TestingResponses::Success => Json("ok".to_string()).into_response(),
        }
    }
}

#[utoipa::path(
    post,
    path = "/testing",
    tag = TAGS::AUTH,
    security(
        ("bearer" = [])
    ),
    responses(TestingResponses),
    tags = ["Pixles Authentication API"]
)]
#[axum::debug_handler]
async fn testing(State(_state): State<AppState>) -> impl axum::response::IntoResponse {
    TestingResponses::Success
}

// TODO: Implement logout all devices

// TODO: Implement unregister user

/// Generate access and refresh token pairs
fn generate_tokens(
    user_id: &str,
    encoding_key: &EncodingKey,
) -> Result<TokenResponse, jsonwebtoken::errors::Error> {
    let user_id = user_id.to_string();

    // Create access token claims
    let access_claims =
        Claims::new_access_token(user_id.clone(), vec![Scope::ReadUser, Scope::WriteUser]);

    // Create refresh token claims
    let refresh_claims = Claims::new_refresh_token(user_id);

    // Generate tokens
    let access_token = access_claims.encode(encoding_key)?;
    let access_token_expires_by = access_claims.exp;
    let refresh_token = refresh_claims.encode(encoding_key)?;

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
        .routes(routes!(testing)) // POST /testing
        .routes(routes!(refresh_token)) // POST /refresh
        .routes(routes!(validate_token)) // POST /validate
        .routes(routes!(reset_password_request)) // POST /password-reset-request
        .routes(routes!(reset_password)) // POST /password-reset
        .routes(routes!(logout)) // POST /logout
        .with_state(state)
}

// TODO: Alerting
// - Multiple failed login attempts
// - Unusual authentication patterns
// - Rate limit threshold breaches

// TODO: Double check on implementation with best practices
// TODO: Add unit tests

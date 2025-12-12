use axum::Json;
use axum::extract::State;
use axum::http::HeaderMap;
use docs::TAGS;
use jsonwebtoken::EncodingKey;
use secrecy::ExposeSecret;
use service::user as UserService;
use tracing::trace;

use crate::claims::{Claims, Scope};
use crate::errors::{AuthError, ClaimValidationError};
use crate::models::errors::BadRegisterUserRequestError;
use crate::models::requests::{LoginRequest, RefreshTokenRequest, RegisterRequest};
use crate::models::responses::{
    LoginResponses, LogoutResponses, RefreshTokenResponses, RegisterUserResponses, TokenResponse,
    ValidateTokenResponse, ValidateTokenResponses,
};
use crate::session::SessionManager;
use crate::state::AppState;
use crate::utils::hash::{hash_password, verify_password};
use crate::utils::headers::get_token_from_headers;

/// Register a new user
#[utoipa::path(
    post,
    path = "/register",
    tag = TAGS::AUTH,
    request_body = RegisterRequest,
    responses(RegisterUserResponses),
    tags = ["Pixles Authentication API"]
)]
pub async fn register_user(
    State(AppState {
        config,
        conn,
        session_manager,
        ..
    }): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> RegisterUserResponses {
    // Validate request
    let RegisterRequest {
        username,
        name,
        email,
        password,
    } = request;

    // Check if user is allowed
    if !UserService::is_valid_username(&username) {
        trace!("Invalid username: {}", username);
        return RegisterUserResponses::BadRequest(BadRegisterUserRequestError::Username);
    }

    // Validate email format
    if !UserService::is_valid_email(&email) {
        trace!("Invalid email: {}", email);
        return RegisterUserResponses::BadRequest(BadRegisterUserRequestError::Email);
    }

    // Validate password strength
    if let Err(e) = UserService::is_valid_password(&password) {
        trace!("Invalid password: {}", e);
        return RegisterUserResponses::BadRequest(BadRegisterUserRequestError::Password);
    }

    // Check if email already exists
    match UserService::Query::find_user_by_email(&conn, &email).await {
        Ok(user) => {
            if user.is_some() {
                trace!("User with email {} already exists", email);
                return RegisterUserResponses::UserAlreadyExists;
            }
        }
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    }

    // Check if username already exists
    let user = match UserService::Query::find_user_by_username(&conn, &username).await {
        Ok(user) => user,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };
    if user.is_some() {
        trace!("User with username {} already exists", username);
        return RegisterUserResponses::UserAlreadyExists;
    }

    // After validation, now create user
    let password_hash = match hash_password(&password) {
        Ok(password_hash) => password_hash,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };
    let user = match UserService::Mutation::create_user(&conn, email, name, username, password_hash)
        .await
    {
        Ok(user) => user,
        Err(e) => return RegisterUserResponses::InternalServerError(e.into()),
    };
    let user_id = &user.id;

    // Generate tokens
    let token_response =
        match generate_tokens(user_id, &config.jwt_eddsa_encoding_key, &session_manager).await {
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
pub async fn login_user(
    State(AppState {
        config,
        conn,
        session_manager,
        ..
    }): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> LoginResponses {
    let LoginRequest { email, password } = request;

    // Find email by email
    let user = match UserService::Query::find_user_by_email(&conn, &email).await {
        Ok(user) => user,
        Err(e) => return LoginResponses::InternalServerError(e.into()),
    };

    if let Some(user) = user {
        // Verify password
        let password_hash = user.password_hash;
        let is_password_valid = match verify_password(&password, &password_hash) {
            Ok(is_valid) => is_valid,
            Err(e) => return LoginResponses::InternalServerError(e.into()),
        };

        if is_password_valid {
            // Track success
            if let Err(e) = UserService::Mutation::track_login_success(&conn, user.id.clone()).await
            {
                tracing::error!("Failed to track login success: {}", e);
            }

            match generate_tokens(&user.id, &config.jwt_eddsa_encoding_key, &session_manager).await
            {
                Ok(token_response) => LoginResponses::Success(token_response),
                Err(e) => LoginResponses::InternalServerError(e.into()),
            }
        } else {
            // Track failure
            if let Err(e) = UserService::Mutation::track_login_failure(&conn, user.id).await {
                tracing::error!("Failed to track login failure: {}", e);
            }

            LoginResponses::InvalidCredentials
        }
    } else {
        // Run dummy password hash to prevent timing attacks
        let _ = verify_password("random", "random").unwrap();
        // TODO: Unit test to verify distribution of timing is uncorrelated to password
        // correctness
        //
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
pub async fn refresh_token(
    State(AppState {
        config,
        session_manager,
        ..
    }): State<AppState>,
    Json(payload): Json<RefreshTokenRequest>,
) -> RefreshTokenResponses {
    // For this example, we'll simulate token validation and renewal
    let token = match Claims::decode(&payload.refresh_token, &config.jwt_eddsa_decoding_key) {
        Ok(token) => token,
        Err(e) => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::from(e).into(),
            );
        }
    };

    let sid = match token.claims.sid {
        Some(sid) => sid,
        None => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::TokenInvalid("Missing session ID".to_string()).into(),
            );
        }
    };

    // Validate session
    let session = match session_manager.get_session(&sid).await {
        Ok(Some(s)) => s,
        Ok(None) => {
            return RefreshTokenResponses::InvalidRefreshToken(
                ClaimValidationError::TokenInvalid("Session not found".to_string()).into(),
            );
        }
        Err(e) => return RefreshTokenResponses::InternalServerError(e.into()),
    };

    // Verify user ownership
    if session.user_id != token.claims.sub {
        return RefreshTokenResponses::InvalidRefreshToken(
            ClaimValidationError::TokenInvalid("Session user mismatch".to_string()).into(),
        );
    }

    // Revoke old session (Rotation)
    if let Err(e) = session_manager.revoke_session(&sid).await {
        return RefreshTokenResponses::InternalServerError(e.into());
    }

    let user_id = token.claims.sub;

    let token_response =
        match generate_tokens(&user_id, &config.jwt_eddsa_encoding_key, &session_manager).await {
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
pub async fn validate_token(
    State(AppState { config, .. }): State<AppState>,
    headers: HeaderMap,
) -> ValidateTokenResponses {
    // Get token string
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return ValidateTokenResponses::Invalid(e.into()),
    };

    // Validate token
    match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key) {
        Ok(token) => ValidateTokenResponses::Valid(ValidateTokenResponse::Valid(token.claims.sub)),
        Err(e) => ValidateTokenResponses::Invalid(ClaimValidationError::from(e).into()),
    }
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
pub async fn logout(
    State(AppState {
        config,
        session_manager,
        ..
    }): State<AppState>,
    headers: HeaderMap,
) -> LogoutResponses {
    // Authorize user
    let token_string = match get_token_from_headers(&headers) {
        Ok(token_string) => token_string,
        Err(e) => return LogoutResponses::Unauthorized(e.into()),
    };

    // Just validate the token signatures
    let token = match Claims::decode(token_string.expose_secret(), &config.jwt_eddsa_decoding_key) {
        Ok(t) => t,
        Err(e) => return LogoutResponses::Unauthorized(ClaimValidationError::from(e).into()),
    };

    // If it has a session ID, revoke it
    if let Some(sid) = token.claims.sid {
        if let Err(e) = session_manager.revoke_session(&sid).await {
            return LogoutResponses::InternalServerError(e.into());
        }
    }

    LogoutResponses::Success
}

/// Generate access and refresh token pairs
pub(crate) async fn generate_tokens(
    user_id: &str,
    encoding_key: &EncodingKey,
    session_manager: &SessionManager,
) -> Result<TokenResponse, AuthError> {
    let user_id = user_id.to_string();

    // Create session
    let sid = session_manager
        .create_session(user_id.clone(), None, None)
        .await
        .map_err(|e| AuthError::InternalServerError(e.into()))?;

    // Create access token claims
    let access_claims =
        Claims::new_access_token(user_id.clone(), vec![Scope::ReadUser, Scope::WriteUser]);

    // Create refresh token claims
    let refresh_claims = Claims::new_refresh_token(user_id, sid);

    // Generate tokens
    let access_token = access_claims
        .encode(encoding_key)
        .map_err(|e| AuthError::InternalServerError(e.into()))?;
    let access_token_expires_by = access_claims.exp;
    let refresh_token = refresh_claims
        .encode(encoding_key)
        .map_err(|e| AuthError::InternalServerError(e.into()))?;

    Ok(TokenResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_by: access_token_expires_by,
    })
}

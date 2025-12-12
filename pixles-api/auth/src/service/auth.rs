use sea_orm::DatabaseConnection;
use service::user as UserService;

use super::token::TokenService;
use crate::claims::{Claims, Scope};
use crate::config::AuthConfig;
use crate::errors::{AuthError, ClaimValidationError};
use crate::models::requests::RegisterRequest;
use crate::models::responses::TokenResponse;
use crate::session::SessionManager;
use crate::utils::hash::{hash_password, verify_password};
use crate::validation::RegistrationValidator;

#[derive(Clone)]
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    /// Get [Claims] from token string
    pub fn get_claims(&self, token: &str) -> Result<Claims, ClaimValidationError> {
        let claims = Claims::decode(token, &self.config.jwt_eddsa_decoding_key)?;
        Ok(claims.claims)
    }

    /// Validate [Claims] by scopes
    pub fn validate_claims(
        &self,
        claims: &Claims,
        required_scopes: &[Scope],
    ) -> Result<(), ClaimValidationError> {
        if !claims.has_scopes(required_scopes) {
            return Err(ClaimValidationError::InvalidScopes);
        }
        Ok(())
    }

    pub async fn register_user(
        &self,
        conn: &DatabaseConnection,
        session_manager: &SessionManager,
        request: RegisterRequest,
    ) -> Result<TokenResponse, AuthError> {
        // Validation
        if let Err(e) = RegistrationValidator::validate(&request) {
            return Err(AuthError::BadRequest(e));
        }

        let RegisterRequest {
            username,
            name,
            email,
            password,
        } = request;

        // Check duplicates
        if let Ok(Some(_)) = UserService::Query::find_user_by_email(conn, &email).await {
            return Err(AuthError::UserAlreadyExists);
        }
        if let Ok(Some(_)) = UserService::Query::find_user_by_username(conn, &username).await {
            return Err(AuthError::UserAlreadyExists);
        }

        let password_hash =
            hash_password(&password).map_err(|e| AuthError::InternalServerError(eyre::eyre!(e)))?;

        // TODO: Handle unique constraint violation from DB if race condition occurs
        let user = UserService::Mutation::create_user(conn, email, name, username, password_hash)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        self.generate_token_pair(&user.id, session_manager).await
    }

    pub async fn authenticate_user(
        &self,
        conn: &DatabaseConnection,
        session_manager: &SessionManager,
        email: &str,
        password: &str,
    ) -> Result<TokenResponse, AuthError> {
        let user = UserService::Query::find_user_by_email(conn, email)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        if let Some(user) = user {
            let is_valid = verify_password(password, &user.password_hash)
                .map_err(|e| AuthError::InternalServerError(eyre::eyre!(e)))?;

            if is_valid {
                let _ = UserService::Mutation::track_login_success(conn, user.id.clone()).await;
                return self.generate_token_pair(&user.id, session_manager).await;
            } else {
                let _ = UserService::Mutation::track_login_failure(conn, user.id).await;
            }
        } else {
            // Timing attack mitigation
            let _ = verify_password("random", "random");
        }

        Err(AuthError::InvalidCredentials)
    }

    pub async fn generate_token_pair(
        &self,
        user_id: &str,
        session_manager: &SessionManager,
    ) -> Result<TokenResponse, AuthError> {
        let sid = session_manager
            .create_session(user_id.to_string(), None, None)
            .await?;

        let (access_token, expires_by) = TokenService::create_access_token(
            user_id,
            vec![Scope::ReadUser, Scope::WriteUser],
            &self.config.jwt_eddsa_encoding_key,
        )?;

        let refresh_token =
            TokenService::create_refresh_token(user_id, sid, &self.config.jwt_eddsa_encoding_key)?;

        Ok(TokenResponse {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_by,
        })
    }
}

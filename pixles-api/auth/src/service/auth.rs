use model::user::CreateUser;
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
use secrecy::{ExposeSecret, SecretString};

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

        let password_hash = hash_password(password.expose_secret())
            .map_err(|e| AuthError::InternalServerError(eyre::eyre!(e)))?;

        // TODO: Handle unique constraint violation from DB if race condition occurs
        let user = UserService::Mutation::create_user(
            conn,
            CreateUser {
                username,
                name,
                email,
                password_hash,
            },
        )
        .await
        .map_err(|e| AuthError::InternalServerError(e.into()))?;

        self.generate_token_pair(&user.id, session_manager).await
    }

    pub async fn authenticate_user(
        &self,
        conn: &DatabaseConnection,
        session_manager: &SessionManager,
        email: &str,
        password: &SecretString,
    ) -> Result<TokenResponse, AuthError> {
        let user = UserService::Query::find_user_by_email(conn, email)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        if let Some(user) = user {
            tracing::info!("User found: {}", user.id);
            let is_valid = verify_password(password.expose_secret(), &user.password_hash)
                .map_err(|e| AuthError::InternalServerError(eyre::eyre!(e)))?;

            if is_valid {
                let _ = UserService::Mutation::track_login_success(conn, user.id.clone()).await;
                return self.generate_token_pair(&user.id, session_manager).await;
            } else {
                let _ = UserService::Mutation::track_login_failure(conn, user.id).await;
            }
        } else {
            tracing::info!("User not found");
            // Timing attack mitigation
            // Uses a valid hash to ensure verify_password performs work
            let dummy_hash = "$argon2id$v=19$m=19456,t=2,p=1$tYPnkCUH2lh52Sj6ZZwkbg$nn/VtIvxWjJoVIWHIpgvesIzTrUvtrczdkXaxmgZ/+w";
            let _ = verify_password("random", dummy_hash);
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
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            token_type: "Bearer".to_string(),
            expires_by,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::InMemorySessionStorage;
    use base64::Engine;
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use ring::signature::{Ed25519KeyPair, KeyPair};
    use std::time::Duration;

    fn get_test_keys() -> (EncodingKey, DecodingKey) {
        let doc: Vec<u8> = base64::engine::general_purpose::STANDARD
            .decode("MC4CAQAwBQYDK2VwBCIEIG73KilXg8qazIq8mNGzuPEHYPLY3WXR1uOS7ZxNkefV")
            .unwrap();
        let pair = Ed25519KeyPair::from_pkcs8_maybe_unchecked(&doc).unwrap();
        let encoding_key = EncodingKey::from_ed_der(&doc);
        let decoding_key = DecodingKey::from_ed_der(pair.public_key().as_ref());

        (encoding_key, decoding_key)
    }

    fn get_test_service() -> AuthService {
        let (encoding_key, decoding_key) = get_test_keys();
        let config = AuthConfig {
            host: "localhost".to_string(),
            port: 8080,
            domain: "example.com".to_string(),
            jwt_eddsa_encoding_key: encoding_key,
            jwt_eddsa_decoding_key: decoding_key,
            jwt_refresh_token_duration_seconds: 3600,
            jwt_access_token_duration_seconds: 300,
            valkey_url: "redis://localhost:6379".to_string(),
        };
        AuthService::new(config)
    }

    #[test]
    fn test_validate_claims_valid() {
        let service = get_test_service();
        let claims = Claims::new_access_token("user1".to_string(), vec![Scope::ReadUser]);

        assert!(service.validate_claims(&claims, &[Scope::ReadUser]).is_ok());
    }

    #[test]
    fn test_validate_claims_invalid_scope() {
        let service = get_test_service();
        let claims = Claims::new_access_token("user1".to_string(), vec![Scope::ReadUser]);

        let result = service.validate_claims(&claims, &[Scope::WriteUser]);
        assert!(matches!(result, Err(ClaimValidationError::InvalidScopes)));
    }

    #[tokio::test]
    async fn test_generate_token_pair() {
        let service = get_test_service();
        let storage = Box::new(InMemorySessionStorage::new());
        let session_manager = SessionManager::new_with_storage(storage, Duration::from_secs(3600));

        let result = service
            .generate_token_pair("user123", &session_manager)
            .await;
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(!response.access_token.expose_secret().is_empty());
        assert!(!response.refresh_token.expose_secret().is_empty());
        assert_eq!(response.token_type, "Bearer");

        // Verify refresh token has session
        let claims = service
            .get_claims(response.refresh_token.expose_secret())
            .unwrap();
        assert!(claims.sid.is_some());

        // Verify session exists
        let sid = claims.sid.unwrap();
        let session = session_manager.get_session(&sid).await.unwrap();
        assert!(session.is_some());
        assert_eq!(session.unwrap().user_id, "user123");
    }
}

use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::Pool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use model::errors::InternalServerError;

pub mod storage;
pub use self::storage::{InMemorySessionStorage, RedisSessionStorage, SessionStorage};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Session {
    pub user_id: String,
    pub created_at: i64,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Clone)]
pub struct SessionManager {
    storage: Arc<Box<dyn SessionStorage>>,
    ttl: Duration,
}

impl SessionManager {
    pub async fn new(redis_url: String, ttl: Duration) -> Result<Self, InternalServerError> {
        let manager =
            RedisConnectionManager::new(redis_url).map_err(|e| InternalServerError::from(e))?;
        let pool = Pool::builder()
            .build(manager)
            .await
            .map_err(InternalServerError::from)?;

        Ok(Self {
            storage: Arc::new(Box::new(RedisSessionStorage::new(pool))),
            ttl,
        })
    }

    // For testing
    pub fn new_with_storage(storage: Box<dyn SessionStorage>, ttl: Duration) -> Self {
        Self {
            storage: Arc::new(storage),
            ttl,
        }
    }

    pub async fn create_session(
        &self,
        user_id: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Result<String, InternalServerError> {
        let sid = nanoid::nanoid!();
        let session = Session {
            user_id: user_id.clone(),
            created_at: chrono::Utc::now().timestamp(),
            user_agent,
            ip_address,
        };

        let session_data = serde_json::to_string(&session).map_err(InternalServerError::from)?;

        self.storage
            .save_session(&sid, session_data, self.ttl)
            .await?;
        self.storage
            .add_user_session(&user_id, &sid, self.ttl)
            .await?;

        Ok(sid)
    }

    pub async fn get_session(&self, sid: &str) -> Result<Option<Session>, InternalServerError> {
        let session_data = self.storage.get_session(sid).await?;

        match session_data {
            Some(data) => {
                let session: Session =
                    serde_json::from_str(&data).map_err(InternalServerError::from)?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    pub async fn revoke_session(&self, sid: &str) -> Result<(), InternalServerError> {
        self.storage.delete_session(sid).await
    }

    pub async fn revoke_all_for_user(&self, user_id: &str) -> Result<(), InternalServerError> {
        let sessions = self.storage.get_user_sessions(user_id).await?;

        for sid in sessions {
            self.storage.delete_session(&sid).await?;
        }

        self.storage.delete_user_sessions_key(user_id).await?;

        Ok(())
    }

    // MFA attempt tracking methods
    pub async fn increment_mfa_attempt(
        &self,
        mfa_token_jti: &str,
    ) -> Result<i32, InternalServerError> {
        self.storage.increment_mfa_attempt(mfa_token_jti).await
    }

    pub async fn get_mfa_attempts(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError> {
        self.storage.get_mfa_attempts(mfa_token_jti).await
    }

    pub async fn clear_mfa_attempts(&self, mfa_token_jti: &str) -> Result<(), InternalServerError> {
        self.storage.clear_mfa_attempts(mfa_token_jti).await
    }
}

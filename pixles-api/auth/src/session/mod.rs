use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::Pool;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

use crate::errors::AuthError;

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
    pub async fn new(redis_url: String, ttl: Duration) -> Result<Self, AuthError> {
        let manager = RedisConnectionManager::new(redis_url)
            .map_err(|e| AuthError::InternalServerError(e.into()))?;
        let pool = Pool::builder()
            .build(manager)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

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
    ) -> Result<String, AuthError> {
        let sid = nanoid::nanoid!();
        let session = Session {
            user_id: user_id.clone(),
            created_at: chrono::Utc::now().timestamp(),
            user_agent,
            ip_address,
        };

        let session_data = serde_json::to_string(&session)
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        self.storage
            .save_session(&sid, session_data, self.ttl)
            .await?;
        self.storage
            .add_user_session(&user_id, &sid, self.ttl)
            .await?;

        Ok(sid)
    }

    pub async fn get_session(&self, sid: &str) -> Result<Option<Session>, AuthError> {
        let session_data = self.storage.get_session(sid).await?;

        match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data)
                    .map_err(|e| AuthError::InternalServerError(e.into()))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    pub async fn revoke_session(&self, sid: &str) -> Result<(), AuthError> {
        self.storage.delete_session(sid).await
    }

    pub async fn revoke_all_for_user(&self, user_id: &str) -> Result<(), AuthError> {
        let sessions = self.storage.get_user_sessions(user_id).await?;

        for sid in sessions {
            self.storage.delete_session(&sid).await?;
        }

        self.storage.delete_user_sessions_key(user_id).await?;

        Ok(())
    }
}

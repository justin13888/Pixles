use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::Pool;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::errors::AuthError;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Session {
    pub user_id: String,
    pub created_at: i64,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
}

#[derive(Clone)]
pub struct SessionManager {
    client: Pool<RedisConnectionManager>,
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

        Ok(Self { client: pool, ttl })
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

        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        let key = format!("pixles:session:{}", sid);

        // Save session with TTL
        let _: () = con
            .set_ex(&key, session_data, self.ttl.as_secs())
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        // Add to user sessions set (for revocation)
        let user_key = format!("pixles:user_sessions:{}", user_id);
        let _: () = con
            .sadd(&user_key, &sid)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;
        // Set expiry on user set too (refresh it)
        let _: () = con
            .expire(&user_key, self.ttl.as_secs() as i64)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        Ok(sid)
    }

    pub async fn get_session(&self, sid: &str) -> Result<Option<Session>, AuthError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        let key = format!("pixles:session:{}", sid);

        // Use Option<String> to handle NIL gracefully
        let session_data: Option<String> = con
            .get(&key)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        let session = match session_data {
            Some(data) => {
                let session: Session = serde_json::from_str(&data)
                    .map_err(|e| AuthError::InternalServerError(e.into()))?;
                Some(session)
            }
            None => None,
        };
        Ok(session)
    }

    pub async fn revoke_session(&self, sid: &str) -> Result<(), AuthError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        let key = format!("pixles:session:{}", sid);

        let _: () = con
            .del(&key)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        Ok(())
    }

    pub async fn revoke_all_for_user(&self, user_id: &str) -> Result<(), AuthError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        let user_key = format!("pixles:user_sessions:{}", user_id);

        // Use Option<Vec<String>> or just Vec<String>. smembers returns Vec.
        let sessions: Vec<String> = con.smembers(&user_key).await.map_err(|e| {
            // If key doesn't exist, smembers returns empty vec?
            // Redis result usually maps nil to empty vec for smembers?
            // Let's assume it might error if connection failed.
            AuthError::InternalServerError(e.into())
        })?;

        for sid in sessions {
            let key = format!("pixles:session:{}", sid);
            let _: () = con
                .del(&key)
                .await
                .map_err(|e| AuthError::InternalServerError(e.into()))?;
        }

        let _: () = con
            .del(&user_key)
            .await
            .map_err(|e| AuthError::InternalServerError(e.into()))?;

        Ok(())
    }
}

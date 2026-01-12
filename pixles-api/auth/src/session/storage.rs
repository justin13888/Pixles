use async_trait::async_trait;
use bb8_redis::RedisConnectionManager;
use bb8_redis::bb8::Pool;
use redis::AsyncCommands;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use model::errors::InternalServerError;

#[async_trait::async_trait]
pub trait SessionStorage: Send + Sync {
    /// Saves a session to storage with the given session ID and data.
    ///
    /// # Arguments
    /// * `sid` - The session identifier
    /// * `session_data` - The serialized session data to store
    /// * `ttl` - Time-to-live duration for the session
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an `InternalServerError` if the operation fails.
    async fn save_session(
        &self,
        sid: &str,
        session_data: String,
        ttl: Duration,
    ) -> Result<(), InternalServerError>;
    
    /// Retrieves session data for the given session ID.
    ///
    /// # Arguments
    /// * `sid` - The session identifier to look up
    ///
    /// # Returns
    /// Returns `Ok(Some(session_data))` if the session exists, `Ok(None)` if not found,
    /// or an `InternalServerError` if the operation fails.
    async fn get_session(&self, sid: &str) -> Result<Option<String>, InternalServerError>;
    
    /// Deletes a session from storage.
    ///
    /// # Arguments
    /// * `sid` - The session identifier to delete
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an `InternalServerError` if the operation fails.
    async fn delete_session(&self, sid: &str) -> Result<(), InternalServerError>;
    
    /// Associates a session with a user, allowing tracking of all active sessions for a user.
    ///
    /// # Arguments
    /// * `user_id` - The user identifier
    /// * `sid` - The session identifier to associate with the user
    /// * `ttl` - Time-to-live duration for this user-session association
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an `InternalServerError` if the operation fails.
    async fn add_user_session(
        &self,
        user_id: &str,
        sid: &str,
        ttl: Duration,
    ) -> Result<(), InternalServerError>;
    
    /// Retrieves all active session IDs for a given user.
    ///
    /// # Arguments
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    /// Returns `Ok(Vec<session_ids>)` with all active session IDs for the user,
    /// or an `InternalServerError` if the operation fails.
    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<String>, InternalServerError>;
    
    /// Deletes the user sessions key, effectively removing the association between a user and their sessions.
    ///
    /// Note: This does not delete the individual sessions themselves, only the tracking key.
    ///
    /// # Arguments
    /// * `user_id` - The user identifier
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an `InternalServerError` if the operation fails.
    async fn delete_user_sessions_key(&self, user_id: &str) -> Result<(), InternalServerError>;

    // MFA attempt tracking
    /// Increments the MFA attempt counter for a given MFA token.
    ///
    /// # Arguments
    /// * `mfa_token_jti` - The JWT ID (jti) of the MFA token
    ///
    /// # Returns
    /// Returns `Ok(attempt_count)` with the new attempt count after incrementing,
    /// or an `InternalServerError` if the operation fails.
    async fn increment_mfa_attempt(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError>;
    
    /// Retrieves the current MFA attempt count for a given MFA token.
    ///
    /// # Arguments
    /// * `mfa_token_jti` - The JWT ID (jti) of the MFA token
    ///
    /// # Returns
    /// Returns `Ok(attempt_count)` with the current attempt count (0 if no attempts recorded),
    /// or an `InternalServerError` if the operation fails.
    async fn get_mfa_attempts(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError>;
    
    /// Clears the MFA attempt counter for a given MFA token.
    ///
    /// This should be called after successful MFA verification.
    ///
    /// # Arguments
    /// * `mfa_token_jti` - The JWT ID (jti) of the MFA token
    ///
    /// # Returns
    /// Returns `Ok(())` on success, or an `InternalServerError` if the operation fails.
    async fn clear_mfa_attempts(&self, mfa_token_jti: &str) -> Result<(), InternalServerError>;
}

#[derive(Clone)]
pub struct RedisSessionStorage {
    client: Pool<RedisConnectionManager>,
}

impl RedisSessionStorage {
    pub fn new(client: Pool<RedisConnectionManager>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SessionStorage for RedisSessionStorage {
    async fn save_session(
        &self,
        sid: &str,
        session_data: String,
        ttl: Duration,
    ) -> Result<(), InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:session:{}", sid);
        let _: () = con
            .set_ex(&key, session_data, ttl.as_secs())
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(())
    }

    async fn get_session(&self, sid: &str) -> Result<Option<String>, InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:session:{}", sid);
        let data: Option<String> = con
            .get(&key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(data)
    }

    async fn delete_session(&self, sid: &str) -> Result<(), InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:session:{}", sid);
        let _: () = con
            .del(&key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(())
    }

    async fn add_user_session(
        &self,
        user_id: &str,
        sid: &str,
        ttl: Duration,
    ) -> Result<(), InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let user_key = format!("pixles:user_sessions:{}", user_id);
        let _: () = con
            .sadd(&user_key, sid)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let _: () = con
            .expire(&user_key, ttl.as_secs() as i64)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(())
    }

    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<String>, InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let user_key = format!("pixles:user_sessions:{}", user_id);
        let sessions: Vec<String> = con
            .smembers(&user_key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(sessions)
    }

    async fn delete_user_sessions_key(&self, user_id: &str) -> Result<(), InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let user_key = format!("pixles:user_sessions:{}", user_id);
        let _: () = con
            .del(&user_key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(())
    }

    async fn increment_mfa_attempt(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:mfa_attempts:{}", mfa_token_jti);
        let count: i32 = con
            .incr(&key, 1)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        // Set expiry to 5 minutes if this is the first attempt
        if count == 1 {
            let _: () = con
                .expire(&key, 300) // 5 minutes
                .await
                .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        }
        Ok(count)
    }

    async fn get_mfa_attempts(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:mfa_attempts:{}", mfa_token_jti);
        let count: Option<i32> = con
            .get(&key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(count.unwrap_or(0))
    }

    async fn clear_mfa_attempts(&self, mfa_token_jti: &str) -> Result<(), InternalServerError> {
        let mut con = self
            .client
            .get()
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        let key = format!("pixles:mfa_attempts:{}", mfa_token_jti);
        let _: () = con
            .del(&key)
            .await
            .map_err(|e| InternalServerError::from(eyre::eyre!(e)))?;
        Ok(())
    }
}

pub struct InMemorySessionStorage {
    sessions: Arc<RwLock<HashMap<String, String>>>,
    user_sessions: Arc<RwLock<HashMap<String, Vec<String>>>>,
    mfa_attempts: Arc<RwLock<HashMap<String, i32>>>,
}

impl Default for InMemorySessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemorySessionStorage {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            user_sessions: Arc::new(RwLock::new(HashMap::new())),
            mfa_attempts: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl SessionStorage for InMemorySessionStorage {
    async fn save_session(
        &self,
        sid: &str,
        session_data: String,
        _ttl: Duration,
    ) -> Result<(), InternalServerError> {
        self.sessions
            .write()
            .await
            .insert(sid.to_string(), session_data);
        Ok(())
    }

    async fn get_session(&self, sid: &str) -> Result<Option<String>, InternalServerError> {
        Ok(self.sessions.read().await.get(sid).cloned())
    }

    async fn delete_session(&self, sid: &str) -> Result<(), InternalServerError> {
        self.sessions.write().await.remove(sid);
        Ok(())
    }

    async fn add_user_session(
        &self,
        user_id: &str,
        sid: &str,
        _ttl: Duration,
    ) -> Result<(), InternalServerError> {
        let mut user_sessions = self.user_sessions.write().await;
        user_sessions
            .entry(user_id.to_string())
            .or_default()
            .push(sid.to_string());
        Ok(())
    }

    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<String>, InternalServerError> {
        Ok(self
            .user_sessions
            .read()
            .await
            .get(user_id)
            .cloned()
            .unwrap_or_default())
    }

    async fn delete_user_sessions_key(&self, user_id: &str) -> Result<(), InternalServerError> {
        self.user_sessions.write().await.remove(user_id);
        Ok(())
    }

    async fn increment_mfa_attempt(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError> {
        let mut attempts = self.mfa_attempts.write().await;
        let count = attempts.entry(mfa_token_jti.to_string()).or_insert(0);
        *count += 1;
        Ok(*count)
    }

    async fn get_mfa_attempts(&self, mfa_token_jti: &str) -> Result<i32, InternalServerError> {
        Ok(*self
            .mfa_attempts
            .read()
            .await
            .get(mfa_token_jti)
            .unwrap_or(&0))
    }

    async fn clear_mfa_attempts(&self, mfa_token_jti: &str) -> Result<(), InternalServerError> {
        self.mfa_attempts.write().await.remove(mfa_token_jti);
        Ok(())
    }
}

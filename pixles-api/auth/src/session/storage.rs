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
    async fn save_session(
        &self,
        sid: &str,
        session_data: String,
        ttl: Duration,
    ) -> Result<(), InternalServerError>;
    async fn get_session(&self, sid: &str) -> Result<Option<String>, InternalServerError>;
    async fn delete_session(&self, sid: &str) -> Result<(), InternalServerError>;
    async fn add_user_session(
        &self,
        user_id: &str,
        sid: &str,
        ttl: Duration,
    ) -> Result<(), InternalServerError>;
    async fn get_user_sessions(&self, user_id: &str) -> Result<Vec<String>, InternalServerError>;
    async fn delete_user_sessions_key(&self, user_id: &str) -> Result<(), InternalServerError>;
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
}

pub struct InMemorySessionStorage {
    sessions: Arc<RwLock<HashMap<String, String>>>,
    user_sessions: Arc<RwLock<HashMap<String, Vec<String>>>>,
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
}

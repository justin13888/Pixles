use crate::error::UploadError;
use crate::models::upload_session::UploadSession;
use bb8_redis::redis::AsyncCommands;
use bb8_redis::{RedisConnectionManager, bb8::Pool};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::Duration;

// TODO: Validate this code

#[derive(Clone)]
pub struct UploadSessionManager {
    pool: Pool<RedisConnectionManager>,
    expiration: Duration,
}

impl UploadSessionManager {
    pub async fn new(valkey_url: &str) -> Result<Self, UploadError> {
        let manager = RedisConnectionManager::new(valkey_url)?;
        let pool = Pool::builder().build(manager).await?;
        Ok(Self {
            pool,
            expiration: Duration::from_secs(24 * 60 * 60), // 24 hours default
        })
    }

    fn key(&self, upload_id: &str) -> String {
        format!("upload:session:{}", upload_id)
    }

    /// Create a new upload session in Redis using HSET.
    /// This sets all fields at once during session creation.
    pub async fn create(&self, session: &UploadSession) -> Result<(), UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(&session.id);

        // Build field-value pairs for HSET
        let fields: Vec<(&str, String)> = vec![
            ("id", session.id.clone()),
            ("user_id", session.user_id.clone()),
            ("filename", session.filename.clone().unwrap_or_default()),
            (
                "content_type",
                session.content_type.clone().unwrap_or_default(),
            ),
            (
                "total_size",
                session
                    .total_size
                    .map(|s| s.to_string())
                    .unwrap_or_default(),
            ),
            ("received_bytes", session.received_bytes.to_string()),
            (
                "is_complete",
                if session.is_complete { "1" } else { "0" }.to_string(),
            ),
            ("created_at", session.created_at.to_rfc3339()),
            ("expires_at", session.expires_at.to_rfc3339()),
        ];

        // Use HSET with multiple fields
        let _: () = bb8_redis::redis::cmd("HSET")
            .arg(&key)
            .arg(fields[0].0)
            .arg(&fields[0].1)
            .arg(fields[1].0)
            .arg(&fields[1].1)
            .arg(fields[2].0)
            .arg(&fields[2].1)
            .arg(fields[3].0)
            .arg(&fields[3].1)
            .arg(fields[4].0)
            .arg(&fields[4].1)
            .arg(fields[5].0)
            .arg(&fields[5].1)
            .arg(fields[6].0)
            .arg(&fields[6].1)
            .arg(fields[7].0)
            .arg(&fields[7].1)
            .arg(fields[8].0)
            .arg(&fields[8].1)
            .query_async(&mut *conn)
            .await?;

        // Set expiration
        let _: () = conn.expire(&key, self.expiration.as_secs() as i64).await?;

        Ok(())
    }

    /// Atomically increment received_bytes using HINCRBY.
    /// Returns the new value of received_bytes.
    pub async fn increment_received_bytes(
        &self,
        upload_id: &str,
        bytes: u64,
    ) -> Result<u64, UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);

        let new_value: i64 = conn.hincr(&key, "received_bytes", bytes as i64).await?;

        Ok(new_value as u64)
    }

    /// Atomically mark the upload as complete.
    pub async fn mark_complete(&self, upload_id: &str) -> Result<(), UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);

        let _: () = conn.hset(&key, "is_complete", "1").await?;

        Ok(())
    }

    /// Get a session by ID using HGETALL.
    pub async fn get(&self, upload_id: &str) -> Result<Option<UploadSession>, UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);

        let fields: HashMap<String, String> = conn.hgetall(&key).await?;

        if fields.is_empty() {
            return Ok(None);
        }

        // Parse fields into UploadSession
        let session = self.parse_session_from_hash(upload_id, fields)?;
        Ok(Some(session))
    }

    /// Delete a session from Redis.
    pub async fn delete(&self, upload_id: &str) -> Result<(), UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);
        let _: () = conn.del(key).await?;
        Ok(())
    }

    /// Parse a HashMap from HGETALL into an UploadSession struct.
    fn parse_session_from_hash(
        &self,
        upload_id: &str,
        fields: HashMap<String, String>,
    ) -> Result<UploadSession, UploadError> {
        let get_field = |name: &str| -> Result<String, UploadError> {
            fields.get(name).cloned().ok_or_else(|| {
                UploadError::Unknown(format!("Missing field '{}' in session {}", name, upload_id))
            })
        };

        let id = get_field("id")?;
        let user_id = get_field("user_id")?;

        let filename = fields
            .get("filename")
            .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });

        let content_type = fields
            .get("content_type")
            .and_then(|s| if s.is_empty() { None } else { Some(s.clone()) });

        let total_size = fields
            .get("total_size")
            .and_then(|s| if s.is_empty() { None } else { s.parse().ok() });

        let received_bytes: u64 = get_field("received_bytes")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid received_bytes: {}", e)))?;

        let is_complete = get_field("is_complete")? == "1";

        let created_at: DateTime<Utc> = get_field("created_at")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid created_at: {}", e)))?;

        let expires_at: DateTime<Utc> = get_field("expires_at")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid expires_at: {}", e)))?;

        Ok(UploadSession {
            id,
            user_id,
            filename,
            content_type,
            total_size,
            received_bytes,
            is_complete,
            created_at,
            expires_at,
        })
    }
}

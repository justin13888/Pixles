use crate::error::UploadError;
use crate::models::session::{UploadSession, UploadStatus};
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
        // Build field-value pairs for HSET
        // key is string, value is bytes
        let mut fields: Vec<(&str, Vec<u8>)> = vec![
            ("id", session.id.as_bytes().to_vec()),
            ("user_id", session.user_id.as_bytes().to_vec()),
            (
                "filename",
                session
                    .filename
                    .as_ref()
                    .map(|s| s.as_bytes().to_vec())
                    .unwrap_or_default(),
            ),
            (
                "content_type",
                session
                    .content_type
                    .as_ref()
                    .map(|s| s.as_bytes().to_vec())
                    .unwrap_or_default(),
            ),
            (
                "total_size",
                session
                    .total_size
                    .map(|s| s.to_string().into_bytes())
                    .unwrap_or_default(),
            ),
            (
                "received_bytes",
                session.received_bytes.to_string().into_bytes(),
            ),
            (
                "status",
                serde_json::to_string(&session.status)
                    .unwrap_or_else(|_| "\"Pending\"".to_string())
                    .into_bytes(),
            ),
            ("created_at", session.created_at.to_rfc3339().into_bytes()),
            ("expires_at", session.expires_at.to_rfc3339().into_bytes()),
        ];

        // Handle expected_hash (binary u64)
        fields.push((
            "expected_hash",
            session.expected_hash.to_le_bytes().to_vec(),
        ));

        // Store album_id if present
        if let Some(album_id) = &session.album_id {
            fields.push(("album_id", album_id.as_bytes().to_vec()));
        }

        // Use HSET with multiple fields
        let mut cmd = bb8_redis::redis::cmd("HSET");
        cmd.arg(&key);
        for (field, value) in fields {
            cmd.arg(field).arg(value);
        }
        let _: () = cmd.query_async(&mut *conn).await?;

        // Set expiration
        let _: () = conn
            .expire(
                &key,
                i64::try_from(self.expiration.as_secs()).unwrap_or(i64::MAX),
            )
            .await?;

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

    /// Atomically update the upload status.
    pub async fn update_status(
        &self,
        upload_id: &str,
        status: UploadStatus,
    ) -> Result<(), UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);

        let status_json = serde_json::to_string(&status)?;
        let _: () = conn.hset(&key, "status", status_json).await?;

        Ok(())
    }

    /// Get a session by ID using HGETALL.
    pub async fn get(&self, upload_id: &str) -> Result<Option<UploadSession>, UploadError> {
        let mut conn = self.pool.get().await?;
        let key = self.key(upload_id);

        let fields: HashMap<String, Vec<u8>> = conn.hgetall(&key).await?;

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
        fields: HashMap<String, Vec<u8>>,
    ) -> Result<UploadSession, UploadError> {
        let get_string = |name: &str| -> Result<String, UploadError> {
            let bytes = fields.get(name).ok_or_else(|| {
                UploadError::Unknown(format!("Missing field '{name}' in session {upload_id}"))
            })?;
            String::from_utf8(bytes.clone())
                .map_err(|e| UploadError::Unknown(format!("Invalid UTF-8 in field {name}: {}", e)))
        };

        let id = get_string("id")?;
        let user_id = get_string("user_id")?;

        let filename = fields.get("filename").and_then(|b| {
            if b.is_empty() {
                None
            } else {
                String::from_utf8(b.clone()).ok()
            }
        });

        let content_type = fields.get("content_type").and_then(|b| {
            if b.is_empty() {
                None
            } else {
                String::from_utf8(b.clone()).ok()
            }
        });

        let total_size = fields.get("total_size").and_then(|b| {
            if b.is_empty() {
                None
            } else {
                String::from_utf8(b.clone())
                    .ok()
                    .and_then(|s| s.parse().ok())
            }
        });

        let received_bytes: u64 = get_string("received_bytes")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid received_bytes: {}", e)))?;

        let status_str = get_string("status")?;
        let status: UploadStatus = serde_json::from_str(&status_str)
            .map_err(|e| UploadError::Unknown(format!("Invalid status '{}': {}", status_str, e)))?;

        let expected_hash = fields
            .get("expected_hash")
            .and_then(|bytes| {
                if bytes.len() == 8 {
                    Some(u64::from_le_bytes(bytes.clone().try_into().unwrap()))
                } else {
                    None
                }
            })
            .ok_or_else(|| {
                UploadError::Unknown(format!(
                    "Missing or invalid expected_hash in session {upload_id}"
                ))
            })?;

        let created_at: DateTime<Utc> = get_string("created_at")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid created_at: {}", e)))?;

        let expires_at: DateTime<Utc> = get_string("expires_at")?
            .parse()
            .map_err(|e| UploadError::Unknown(format!("Invalid expires_at: {}", e)))?;

        let album_id = fields
            .get("album_id")
            .and_then(|bytes| String::from_utf8(bytes.clone()).ok());

        Ok(UploadSession {
            id,
            user_id,
            filename,
            content_type,
            total_size,
            received_bytes,
            status,
            expected_hash,
            album_id,
            created_at,
            expires_at,
        })
    }
}

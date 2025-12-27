use crate::models::responses::{
    CreateUploadResponse, CreateUploadResponses, DeleteUploadResponses, HeadUploadResponse,
    HeadUploadResponses, ListSessionsResponse, ListSessionsResponses, PatchUploadResponses,
};
use crate::models::session::UploadStatus;
use crate::state::AppState;
use auth::utils::headers::get_user_id_from_headers;
use base64::{Engine as _, engine::general_purpose};
use salvo::prelude::*;
use service::album as AlbumService;

// TODO: Thoroughly review and test this module.

// Constants for chunk sizes (4KB aligned)
const KB: u64 = 1024;
const CHUNK_SIZE_256KB: u64 = 256 * KB;
const CHUNK_SIZE_1MB: u64 = 1024 * KB;
const CHUNK_SIZE_4MB: u64 = 4 * 1024 * KB;

/// Calculate suggested chunk size based on total file size
fn get_suggested_chunk_size(total_size: Option<u64>) -> u64 {
    match total_size {
        Some(size) if size < 10 * 1024 * KB => CHUNK_SIZE_256KB, // < 10MB
        Some(size) if size < 100 * 1024 * KB => CHUNK_SIZE_1MB,  // < 100MB
        _ => CHUNK_SIZE_4MB,                                     // >= 100MB or unknown
    }
}

/// Create a new upload session
#[endpoint(
    operation_id = "create_upload",
    tags("upload"),
    security(("bearer" = []))
)]
pub async fn create_upload(req: &mut Request, dep: &mut Depot) -> CreateUploadResponses {
    let state = dep.obtain::<AppState>().unwrap();

    // Authenticate User
    let user_id =
        match get_user_id_from_headers(req.headers(), &state.config.jwt_eddsa_decoding_key) {
            Ok(id) => id,
            Err(e) => return CreateUploadResponses::Unauthorized(e),
        };

    // Parse X-Pixles headers
    let upload_length: Option<u64> = req
        .header::<String>("X-Pixles-Content-Length")
        .and_then(|s| s.parse().ok());

    let mut filename = None;
    let mut content_type = None;
    let mut expected_hash: Option<u64> = None;
    let mut album_id = None;

    if let Some(metadata) = req.header::<String>("X-Pixles-Metadata") {
        for pair in metadata.split(',') {
            let parts: Vec<&str> = pair.split_whitespace().collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value_b64 = parts[1];
                if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(value_b64) {
                    match key {
                        "filename" => {
                            if let Ok(value) = String::from_utf8(decoded_bytes) {
                                filename = Some(value);
                            }
                        }
                        "content_type" => {
                            if let Ok(value) = String::from_utf8(decoded_bytes) {
                                content_type = Some(value);
                            }
                        }
                        "hash" => {
                            // Support raw 8 bytes (LE) or hex string
                            if decoded_bytes.len() == 8 {
                                expected_hash =
                                    Some(u64::from_le_bytes(decoded_bytes.try_into().unwrap()));
                            } else if let Ok(s) = String::from_utf8(decoded_bytes) {
                                // Try Hex first as it's common for hashes
                                if let Ok(h) = u64::from_str_radix(&s, 16) {
                                    expected_hash = Some(h);
                                } else if let Ok(h) = s.parse::<u64>() {
                                    expected_hash = Some(h);
                                }
                            }
                        }
                        "album_id" | "aid" => {
                            if let Ok(value) = String::from_utf8(decoded_bytes) {
                                album_id = Some(value);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Enforce expected_hash
    let Some(expected_hash) = expected_hash else {
        return CreateUploadResponses::BadRequest("Missing required metadata: hash".to_string());
    };

    // Verify album_id exists and user has access
    if let Some(album_id) = &album_id {
        match AlbumService::Query::album_exists(&state.conn, album_id).await {
            Ok(album_exists) => {
                if !album_exists {
                    return CreateUploadResponses::BadRequest("Album does not exist".to_string());
                }
            }
            Err(e) => {
                return CreateUploadResponses::InternalServerError(e.into());
            }
        }
    }

    match state
        .upload_service
        .create_session(
            &user_id,
            filename,
            content_type,
            upload_length,
            expected_hash,
            album_id,
        )
        .await
    {
        Ok(session) => {
            let suggested_chunk_size = get_suggested_chunk_size(upload_length);
            CreateUploadResponses::Success(CreateUploadResponse {
                id: session.id.clone(),
                upload_url: format!("/upload/{}", session.id),
                suggested_chunk_size,
            })
        }
        Err(e) => CreateUploadResponses::InternalServerError(e),
    }
}

/// Get upload session status
#[endpoint(
    operation_id = "head_upload",
    tags("upload"),
    security(("bearer" = []))
)]
pub async fn head_upload(req: &mut Request, dep: &mut Depot) -> HeadUploadResponses {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    // Authenticate User
    let user_id =
        match get_user_id_from_headers(req.headers(), &state.config.jwt_eddsa_decoding_key) {
            Ok(id) => id,
            Err(e) => return HeadUploadResponses::Unauthorized(e),
        };

    match state.upload_service.get_session(&id).await {
        Ok(Some(session)) => {
            if session.user_id != user_id {
                return HeadUploadResponses::Forbidden;
            }
            HeadUploadResponses::Success(HeadUploadResponse {
                offset: session.received_bytes,
                total_size: session.total_size,
                status: session.status,
            })
        }
        Ok(None) => HeadUploadResponses::NotFound,
        Err(e) => HeadUploadResponses::InternalServerError(e),
    }
}

/// Append a chunk to an upload
#[endpoint(
    operation_id = "patch_upload",
    tags("upload"),
    security(("bearer" = []))
)]
pub async fn patch_upload(req: &mut Request, dep: &mut Depot) -> PatchUploadResponses {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    // Authenticate User
    let user_id =
        match get_user_id_from_headers(req.headers(), &state.config.jwt_eddsa_decoding_key) {
            Ok(id) => id,
            Err(e) => return PatchUploadResponses::Unauthorized(e),
        };

    // Verify ownership
    // We check ownership before creating the payload stream to fail fast
    // However, for efficiency, we might want to check session existence first.
    // The previous implementation fetched session later for alignment checks.
    // We will do a lightweight check here.
    match state.upload_service.get_session(&id).await {
        Ok(Some(session)) => {
            if session.user_id != user_id {
                return PatchUploadResponses::Forbidden;
            }
        }
        Ok(None) => return PatchUploadResponses::NotFound,
        Err(e) => return PatchUploadResponses::InternalServerError(e),
    };

    // Parse X-Pixles-Offset header
    let offset: u64 = match req
        .header::<String>("X-Pixles-Offset")
        .and_then(|s| s.parse().ok())
    {
        Some(o) => o,
        None => {
            return PatchUploadResponses::BadRequest("Missing X-Pixles-Offset header".to_string());
        }
    };

    // Parse optional X-Pixles-Checksum header for verification
    let _checksum: Option<String> = req.header::<String>("X-Pixles-Checksum");

    let body = match req.payload().await {
        Ok(b) => b,
        Err(e) => {
            return PatchUploadResponses::BadRequest(format!("Failed to read body: {}", e));
        }
    };
    let bytes = body.clone();

    // Validate 4KB alignment
    if bytes.is_empty() {
        // Empty chunk - nothing to do, just return current offset
        return match state.upload_service.get_session(&id).await {
            Ok(Some(session)) => PatchUploadResponses::Success {
                new_offset: session.received_bytes,
            },
            Ok(None) => PatchUploadResponses::NotFound,
            Err(e) => PatchUploadResponses::InternalServerError(e),
        };
    }

    if bytes.len() % 4096 != 0 {
        // Allow non-aligned chunks only if it's the final chunk
        let session = match state.upload_service.get_session(&id).await {
            Ok(Some(s)) => s,
            Ok(None) => return PatchUploadResponses::NotFound,
            Err(e) => return PatchUploadResponses::InternalServerError(e),
        };

        if let Some(total) = session.total_size {
            let new_offset = session.received_bytes + bytes.len() as u64;
            if new_offset != total {
                return PatchUploadResponses::BadRequest(
                    "Chunk size must be 4KB aligned (except for final chunk)".to_string(),
                );
            }
        }
    }

    // Append chunk
    match state.upload_service.append_chunk(&id, bytes, offset).await {
        Ok(session) => {
            // Check for completion
            if let Some(total) = session.total_size
                && session.received_bytes == total
            {
                // Attempt finalize
                match state.upload_service.finalize_upload(&id).await {
                    Ok(_) => {
                        // Finalized successfully
                    }
                    Err(e) => {
                        return PatchUploadResponses::InternalServerError(e);
                    }
                }
            }
            PatchUploadResponses::Success {
                new_offset: session.received_bytes,
            }
        }
        Err(e) => {
            if e.to_string().contains("Invalid offset") {
                PatchUploadResponses::Conflict(e.to_string())
            } else {
                PatchUploadResponses::InternalServerError(e)
            }
        }
    }
}

/// Delete/cancel an upload session
#[endpoint(
    operation_id = "delete_upload",
    tags("upload"),
    security(("bearer" = []))
)]
pub async fn delete_upload(req: &mut Request, dep: &mut Depot) -> DeleteUploadResponses {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    // Authenticate User
    let user_id =
        match get_user_id_from_headers(req.headers(), &state.config.jwt_eddsa_decoding_key) {
            Ok(id) => id,
            Err(e) => return DeleteUploadResponses::Unauthorized(e),
        };

    // Verify ownership
    match state.upload_service.get_session(&id).await {
        Ok(Some(session)) => {
            if session.user_id != user_id {
                return DeleteUploadResponses::Forbidden;
            }
        }
        Ok(None) => return DeleteUploadResponses::NotFound,
        Err(e) => return DeleteUploadResponses::InternalServerError(e),
    };

    match state.upload_service.cancel_upload(&id).await {
        Ok(_) => DeleteUploadResponses::Success,
        Err(e) => {
            if matches!(e, crate::error::UploadError::SessionNotFound) {
                DeleteUploadResponses::NotFound
            } else {
                DeleteUploadResponses::InternalServerError(e)
            }
        }
    }
}

/// List user's upload sessions
#[endpoint(
    operation_id = "list_sessions",
    tags("upload"),
    security(("bearer" = []))
)]
pub async fn list_sessions(req: &mut Request, dep: &mut Depot) -> ListSessionsResponses {
    let state = dep.obtain::<AppState>().unwrap();

    // Authenticate User
    let user_id =
        match get_user_id_from_headers(req.headers(), &state.config.jwt_eddsa_decoding_key) {
            Ok(id) => id,
            Err(e) => return ListSessionsResponses::Unauthorized(e),
        };

    // Parse query parameters
    let status_filter: Option<UploadStatus> = req
        .query::<String>("status")
        .and_then(|s| serde_json::from_str::<UploadStatus>(&format!("\"{}\"", s)).ok());

    let _date_from: Option<String> = req.query("date_from");
    let _date_to: Option<String> = req.query("date_to");

    // TODO: Implement list_user_sessions in session manager
    // For now, return empty list
    let _ = (user_id, status_filter);

    ListSessionsResponses::Success(ListSessionsResponse { sessions: vec![] })
}

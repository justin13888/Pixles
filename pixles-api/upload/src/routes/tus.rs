use crate::state::AppState;
use auth::claims::Claims;
use auth::utils::headers::get_token_from_headers;
use base64::{Engine as _, engine::general_purpose};
use salvo::prelude::*;
use secrecy::ExposeSecret;

// TODO: Validate TUS v1 and v2 standards compliance

#[handler]
pub async fn create_upload(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();

    let upload_length: Option<u64> = req
        .header::<String>("Upload-Length")
        .and_then(|s| s.parse().ok());

    // Authenticate User
    let headers = req.headers();
    let user_id = match get_token_from_headers(headers) {
        Ok(token_string) => {
            match Claims::decode(
                token_string.expose_secret(),
                &state.config.jwt_eddsa_decoding_key,
            ) {
                Ok(token) => token.claims.sub,
                Err(e) => {
                    res.status_code(StatusCode::UNAUTHORIZED);
                    res.render(format!("Invalid token: {}", e));
                    return;
                }
            }
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(format!("Authentication required: {}", e));
            return;
        }
    };

    let mut filename = None;
    let mut content_type = None;

    if let Some(metadata) = req.header::<String>("Upload-Metadata") {
        for pair in metadata.split(',') {
            let parts: Vec<&str> = pair.split_whitespace().collect();
            if parts.len() == 2 {
                let key = parts[0];
                let value_b64 = parts[1];
                if let Ok(decoded_bytes) = general_purpose::STANDARD.decode(value_b64)
                    && let Ok(value) = String::from_utf8(decoded_bytes)
                {
                    match key {
                        "filename" | "name" => filename = Some(value),
                        "filetype" | "content_type" | "type" => content_type = Some(value),
                        _ => {}
                    }
                }
            }
        }
    }

    match state
        .upload_service
        .create_session(&user_id, filename, content_type, upload_length)
        .await
    {
        Ok(session) => {
            res.status_code(StatusCode::CREATED);
            res.add_header("Location", format!("/upload/{}", session.id), true)
                .ok();
            res.add_header("Tus-Resumable", "1.0.0", true).ok();
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Failed to create upload: {}", e));
        }
    }
}

#[handler]
pub async fn head_upload(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    match state.upload_service.get_session(&id).await {
        Ok(Some(session)) => {
            res.status_code(StatusCode::OK);
            res.add_header("Tus-Resumable", "1.0.0", true).ok();
            res.add_header("Upload-Offset", session.received_bytes.to_string(), true)
                .ok();
            if let Some(len) = session.total_size {
                res.add_header("Upload-Length", len.to_string(), true).ok();
            }
            res.add_header("Cache-Control", "no-store", true).ok();
        }
        Ok(None) => {
            res.status_code(StatusCode::NOT_FOUND);
        }
        Err(_) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[handler]
pub async fn patch_upload(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    let offset: u64 = match req
        .header::<String>("Upload-Offset")
        .and_then(|s| s.parse().ok())
    {
        Some(o) => o,
        None => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render("Missing Upload-Offset header");
            return;
        }
    };

    let body = match req.payload().await {
        Ok(b) => b,
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(format!("Failed to read body: {}", e));
            return;
        }
    };
    let bytes = body.clone();

    match state.upload_service.append_chunk(&id, bytes, offset).await {
        Ok(session) => {
            res.status_code(StatusCode::NO_CONTENT);
            res.add_header("Tus-Resumable", "1.0.0", true).ok();
            res.add_header("Upload-Offset", session.received_bytes.to_string(), true)
                .ok();

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
                        // If finalization fails, we probably should signal 500, but header is already 204?
                        // Actually we haven't sent response yet.
                        res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                        res.render(format!("Upload verification/finalization failed: {}", e));
                        return;
                    }
                }
            }
        }
        Err(e) => {
            if e.to_string().contains("Invalid offset") {
                res.status_code(StatusCode::CONFLICT);
            } else {
                res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            }
            res.render(format!("Upload failed: {}", e));
        }
    }
}

#[handler]
pub async fn delete_upload(req: &mut Request, dep: &mut Depot, res: &mut Response) {
    let state = dep.obtain::<AppState>().unwrap();
    let id = req.param::<String>("id").unwrap();

    match state.upload_service.cancel_upload(&id).await {
        Ok(_) => {
            res.status_code(StatusCode::NO_CONTENT);
        }
        Err(e) => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(format!("Failed to cancel upload: {}", e));
        }
    }
}

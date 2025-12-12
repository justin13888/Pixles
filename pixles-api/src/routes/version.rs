use axum::{Json, response::IntoResponse};
use docs::TAGS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
}

/// Get API version info
#[utoipa::path(
    get,
    path = "/version",
    tag = TAGS::API,
    security(),
    responses(
        (status = 200, description = "Version info", body = VersionResponse)
    )
)]
pub async fn get_version() -> impl IntoResponse {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    Json(VersionResponse {
        name: name.to_string(),
        version: version.to_string(),
    })
}

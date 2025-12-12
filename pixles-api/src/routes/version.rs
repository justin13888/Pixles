use aide::axum::IntoApiResponse;
use axum::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
}

/// Get API version info
pub async fn get_version() -> impl IntoApiResponse {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    Json(VersionResponse {
        name: name.to_string(),
        version: version.to_string(),
    })
}

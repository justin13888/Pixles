use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct VersionResponse {
    pub name: String,
    pub version: String,
}

pub async fn get_version() -> impl IntoResponse {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");

    Json(VersionResponse {
        name: name.to_string(),
        version: version.to_string(),
    })
}

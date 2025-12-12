use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get},
    openapi::{Info, License, OpenApi, Tag},
    scalar::Scalar,
    transform::TransformOpenApi,
};
use axum::{Extension, Json, Router};
use tracing::debug;

#[allow(non_snake_case)]
pub mod TAGS {
    pub const API: &str = "api";
    pub const AUTH: &str = "auth";
    pub const UPLOAD: &str = "upload";
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Pixles API")
        .description("Pixles API Documentation")
        .version("0.1.0")
        .tag(Tag {
            name: TAGS::API.into(),
            description: Some("Pixles API".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: TAGS::AUTH.into(),
            description: Some("Pixles Authentication API".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: TAGS::UPLOAD.into(),
            description: Some("Pixles Upload API".into()),
            ..Default::default()
        })
        .security_scheme(
            "bearer",
            aide::openapi::SecurityScheme::Http {
                scheme: "bearer".into(),
                bearer_format: Some("JWT".into()),
                description: Some("JWT Bearer token authentication".into()),
                extensions: Default::default(),
            },
        )
}

async fn serve_openapi(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

pub fn get_base_openapi() -> OpenApi {
    OpenApi {
        info: Info {
            title: "Pixles API".into(),
            description: Some("Pixles API Documentation".into()),
            version: "0.1.0".into(),
            license: Some(License {
                name: "GNU Affero General Public License v3.0 or later".into(),
                identifier: Some("AGPL-3.0-or-later".into()),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn get_router(v1_router: ApiRouter) -> (Router, OpenApi) {
    let mut api = get_base_openapi();

    let mut app = ApiRouter::new()
        .nest("/v1", v1_router)
        .api_route("/openapi.json", get(serve_openapi));

    if cfg!(feature = "openapi") {
        const SCALAR_ROUTE: &str = "/openapi";
        debug!("OpenAPI documentation enabled at {}", SCALAR_ROUTE);

        // Use route() for Scalar's axum_route() which returns ApiMethodRouter
        app = app.route(SCALAR_ROUTE, Scalar::new("/openapi.json").axum_route());
    } else {
        debug!("OpenAPI Documentation is not used");
    }

    let router: Router = app.finish_api_with(&mut api, api_docs);
    (router.layer(Extension(api.clone())), api)
}

use axum::Router;
use tracing::debug;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable as ScalarServable};

#[allow(non_snake_case)]
pub mod TAGS {
    pub const API: &str = "api";
    pub const AUTH: &str = "auth";
    pub const UPLOAD: &str = "upload";
}

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = TAGS::API, description = "Pixles API"),
        (name = TAGS::AUTH, description = "Pixles Authentication API"),
        (name = TAGS::UPLOAD, description = "Pixles Upload API"),
    ),
    servers(
        (url = "/v1", description = "Current Server URL")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            ) // TODO: Replace this scheme ^^
        }
    }
}

pub fn get_router(router: OpenApiRouter) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(router)
        .split_for_parts();

    if cfg!(feature = "openapi") {
        const SCALAR_ROUTE: &str = "/openapi";
        debug!("OpenAPI documentation enabled at {}", SCALAR_ROUTE);
        let mut router = router.merge(Scalar::with_url(SCALAR_ROUTE, api.clone()));
        let get_openapi_json = async move || axum::Json(api);
        router = router.route("/openapi.json", axum::routing::get(get_openapi_json));

        router
    } else {
        debug!("OpenAPI Documentation is not used");
        router
    }
}

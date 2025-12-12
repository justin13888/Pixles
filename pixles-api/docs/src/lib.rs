use axum::Router;
use tracing::debug;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
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
    info(
        title = "Pixles API",
        description = "Pixles API Documentation",
        version = "0.1.0",
        license(
            name = "GNU Affero General Public License v3.0 or later",
            identifier = "AGPL-3.0-or-later",
        ),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = TAGS::API, description = "Pixles API"),
        (name = TAGS::AUTH, description = "Pixles Authentication API"),
        (name = TAGS::UPLOAD, description = "Pixles Upload API"),
    ),
    servers(
        (url = "/", description = "Current Server URL"),
    ),
    security(
        ("bearer" = [])
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);

        components.add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

pub fn get_router(v1_router: OpenApiRouter) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/v1", v1_router)
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

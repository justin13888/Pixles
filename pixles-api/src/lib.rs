use environment::Environment;
use eyre::Result;
use salvo::oapi::security::{Http, HttpAuthScheme, SecurityScheme};
use salvo::oapi::{Info, License, OpenApi, Tag};
use salvo::prelude::*;
use sea_orm::DatabaseConnection;

pub mod routes;

/// OpenAPI tag constants
pub mod tags {
    pub const API: &str = "api";
    pub const AUTH: &str = "auth";
    pub const UPLOAD: &str = "upload";
}

/// Create OpenAPI specification with proper metadata
pub fn create_openapi_spec() -> OpenApi {
    let info = Info::new("Pixles API", "0.1.0")
        .description("Pixles API Documentation")
        .license(
            License::new("GNU Affero General Public License v3.0 or later")
                .url("https://www.gnu.org/licenses/agpl-3.0.html"),
        );

    OpenApi::with_info(info)
        .tags([
            Tag::new(tags::API).description("Pixles API"),
            Tag::new(tags::AUTH).description("Pixles Authentication API"),
            Tag::new(tags::UPLOAD).description("Pixles Upload API"),
        ])
        .add_security_scheme(
            "bearer",
            SecurityScheme::Http(
                Http::new(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description("JWT Bearer token authentication"),
            ),
        )
}

/// Create the main router for the API
pub async fn create_router(conn: DatabaseConnection, env: &Environment) -> Result<Router> {
    let mut v1_router = Router::new();

    #[cfg(feature = "auth")]
    {
        v1_router = v1_router.push(
            Router::with_path("auth").push(auth::get_router(conn.clone(), &env.server).await?),
        );
    }
    // TODO: Re-enable when graphql is production-ready
    // #[cfg(feature = "graphql")]
    // {
    //     v1_router = v1_router.push(
    //         Router::with_path("graphql").push(
    //             graphql::get_router(conn.clone(), &env.server, (&env.server).into()).await?,
    //         ),
    //     );
    // }
    // TODO: Re-enable when metadata is production-ready
    // #[cfg(feature = "metadata")]
    // {
    //     v1_router = v1_router.push(
    //         Router::with_path("metadata").push(
    //             metadata::get_router(conn.clone(), &env.server).await?,
    //         ),
    //     );
    // }
    #[cfg(feature = "upload")]
    {
        v1_router = v1_router.push(
            Router::with_path("upload").push(upload::get_router(conn.clone(), &env.server).await?),
        );
    }

    // Add version endpoint
    v1_router = v1_router.push(Router::with_path("version").get(routes::version::get_version));

    // Wrap API routes in /v1 prefix
    let v1_router = Router::with_path("v1").push(v1_router);

    // Build the final router
    let router;
    #[cfg(feature = "openapi")]
    {
        // Build OpenAPI documentation (at root level, not under /v1)
        let doc = create_openapi_spec().merge_router(&v1_router);

        router = Router::new()
            .push(v1_router)
            .push(doc.into_router("/openapi.json"))
            .push(SwaggerUi::new("/openapi.json").into_router("/swagger-ui"))
            .push(Scalar::new("/openapi.json").into_router("/openapi"));
    }
    #[cfg(not(feature = "openapi"))]
    {
        router = v1_router;
    }

    Ok(router)
}

// Re-export dependency crates if needed by binaries
pub use auth;
// TODO: Re-enable when graphql is production-ready
// pub use graphql;
// TODO: Re-enable when metadata is production-ready
// pub use metadata;
// TODO: Re-enable when upload is production-ready
// pub use upload;

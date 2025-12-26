use aide::axum::{ApiRouter, routing::get};
use aide::openapi::OpenApi;
use axum::Router;
use environment::Environment;
use eyre::Result;
use routes::version::get_version;
use sea_orm::DatabaseConnection;

pub mod routes;

pub async fn create_app(conn: DatabaseConnection, env: &Environment) -> Result<(Router, OpenApi)> {
    let mut api_router = ApiRouter::new();
    let mut router = Router::new();

    #[cfg(feature = "auth")]
    {
        api_router = api_router.nest("/auth", auth::get_router(conn.clone(), &env.server).await?);
    }
    #[cfg(feature = "graphql")]
    {
        router = router.nest(
            "/graphql",
            graphql::get_router(conn.clone(), &env.server, (&env.server).into()).await?,
        );
    }
    #[cfg(feature = "metadata")]
    {
        router = router.nest(
            "/metadata",
            metadata::get_router(conn.clone(), &env.server).await?,
        );
    }
    #[cfg(feature = "upload")]
    {
        api_router = api_router.nest(
            "/upload",
            upload::get_router(conn.clone(), &env.server).await?,
        );
    }

    api_router = api_router.api_route_with(
        "/version",
        aide::axum::routing::get_with(get_version, |op| op.id("get_version")),
        |op| op.description("Get API version info"),
    );
    let (docs_router_part, api) = docs::get_router(api_router);

    // Merge docs_router into app router
    let router = router.merge(docs_router_part);

    Ok((router, api))
}

// Re-export dependency crates if needed by binaries, though they usually have their own
pub use auth;
pub use docs;
pub use graphql;
pub use metadata;
pub use upload;

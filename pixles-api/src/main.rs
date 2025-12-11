use axum::Router;
use environment::Environment;
use eyre::{Result, eyre};
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use routes::version::get_version;
use sea_orm::Database;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};
use tokio::net::TcpListener;
use tracing::{debug, info};
use tracing_subscriber::fmt::format::FmtSpan;
use utoipa_axum::{router::OpenApiRouter, routes};

mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Load environment settings
    let env =
        Environment::load().map_err(|e| eyre!("Failed to load environment settings: {:?}", e))?;

    // Set up logging
    if cfg!(debug_assertions) {
        // Development configuration: Pretty printing with colors
        tracing_subscriber::fmt()
            .with_max_level(env.log_level)
            .with_target(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::FULL)
            .with_ansi(true)
            .pretty()
            .init();
    } else {
        // Production configuration: JSON format
        tracing_subscriber::fmt()
            .with_max_level(env.log_level)
            .with_target(true)
            .with_file(true)
            .with_span_events(FmtSpan::FULL)
            .json()
            .init();
    }

    debug!("Environment settings loaded: {:?}", env);

    // Initialize database connection
    let conn = Arc::new(Database::connect(env.database.url).await?);

    // Run auto migration in dev
    #[cfg(debug_assertions)]
    {
        Migrator::up(conn.as_ref(), None).await?;
    }

    let mut openapi_router = OpenApiRouter::new();
    let mut router = Router::new();

    #[cfg(feature = "auth")]
    {
        openapi_router =
            openapi_router.nest("/auth", auth::get_router(conn.clone(), &env.server).await?);
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
        openapi_router = openapi_router.nest(
            "/upload",
            upload::get_router(conn.clone(), &env.server).await?,
        );
    }

    use crate::routes::version::__path_get_version;
    openapi_router = openapi_router.routes(routes!(get_version)); // TODO: Add this to OpenAPI

    let docs_router = docs::get_router(openapi_router); // Should include docs only if 'openapi' feature is enabled
    let router = router.merge(docs_router);

    let router = Router::new().nest("/v1", router);
    let app = router.into_make_service();

    // Start server
    info!(
        "Starting server on http://{}:{}/",
        env.server.host, env.server.port
    );

    // Setup listenfd
    let mut listenfd = ListenFd::from_env();
    let listener = match listenfd.take_tcp_listener(0).unwrap() {
        // if we are given a tcp listener on listen fd 0, we use that one
        Some(listener) => {
            listener.set_nonblocking(true).unwrap();
            TcpListener::from_std(listener).unwrap()
        }
        // otherwise fall back to local listening
        None => TcpListener::bind(SocketAddrV4::new(
            env.server.host.parse::<Ipv4Addr>()?,
            env.server.port,
        ))
        .await
        .unwrap(),
    };

    // Serve
    info!("Server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .map_err(|e| eyre!("Axum server error: {:?}", e))?;

    Ok(())
}

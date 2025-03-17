use axum::Router;
use environment::Environment;
use eyre::{eyre, Result};
use listenfd::ListenFd;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};
use tokio::net::TcpListener;
use tracing::{debug, info};
use tracing_subscriber::fmt::format::FmtSpan;

#[cfg(not(any(feature = "graphql", feature = "upload")))]
compile_error!("At least one of the features \"graphql\" or \"upload\" must be enabled");

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
    Migrator::up(conn.as_ref(), None).await?;

    let mut router = Router::new();
    #[cfg(feature = "graphql")]
    {
        router = router.merge(graphql::get_router(conn.clone(), env.server.clone().into()).await?);
    }
    #[cfg(feature = "upload")]
    {
        router = router.merge(upload::get_router(conn.clone(), env.server.clone().into()).await?);
    }

    let app = router.into_make_service();

    // Start server
    info!(
        "GraphQL server running at http://{}:{}/graphql",
        env.server.host, env.server.port
    );

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

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .await
        .map_err(|e| eyre!("Axum server error: {:?}", e))?;

    Ok(())
}

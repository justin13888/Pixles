use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{HeaderMap, Method},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use context::get_user_context_from_headers;
use environment::Environment;
use eyre::{eyre, Result};
use listenfd::ListenFd;
use loaders::Loaders;
use schema::{create_schema, AppSchema};
use sea_orm::Database;
use state::AppState;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{debug, info};
use tracing_subscriber::fmt::format::FmtSpan;

mod context;
mod environment;
mod loaders;
mod models;
mod schema;
mod state;

async fn graphql_handler(
    State(AppState { schema, .. }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> impl IntoResponse {
    // Create user context
    let user_context = get_user_context_from_headers(&headers);

    // Add the user context to the request
    let mut req = req.into_inner();
    req = req.data(user_context);

    GraphQLResponse::from(schema.execute(req).await)
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            // .header()
            .title("Pixles API")
            .finish(),
    )
}

pub async fn start() -> Result<()> {
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
    // Migrator::up(&conn, None).await?; // TODO

    // Create loaders
    let loaders = Loaders::new(conn.clone());

    // Build GraphQL schema
    let schema: AppSchema = create_schema(loaders);

    // Define state
    let state = AppState { schema, conn };

    // Build router
    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        );

    // Start server
    info!(
        "GraphQL server running at http://{}:{}/graphql",
        env.server.host, env.server.port
    );
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

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
        .map_err(|e| eyre!("Server error: {:?}", e))
}

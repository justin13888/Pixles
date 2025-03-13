use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::Arc,
};

use async_graphql::{http::GraphiQLSource, Response};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{HeaderMap, Method},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use axum::{
    http::{header, HeaderName},
    routing::post,
};
use context::{AppContext, DbContext, UserContext};
use environment::{Environment, ServerConfig};
use eyre::{eyre, Result};
use listenfd::ListenFd;
use loaders::Loaders;
use pixles_api_migration::{Migrator, MigratorTrait};
use schema::{create_schema, AppSchema};
use sea_orm::Database;
use state::AppState;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::{debug, info, trace};
use tracing_subscriber::fmt::format::FmtSpan;

mod constants;
mod context;
mod environment;
mod hash;
mod jwt;
mod loaders;
mod schema;
mod state;

async fn graphql_handler(
    State(AppState {
        schema,
        conn,
        config,
    }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> impl IntoResponse {
    // Create user context
    let user_context = match UserContext::from_headers(&headers, &config) {
        Ok(user_context) => user_context,
        Err(e) => return GraphQLResponse::from(Response::from_errors(vec![e.into()])),
    };
    trace!("User context created: {:?}", user_context);

    // Add the user context to the request
    let mut req = req.into_inner();
    req = req.data(AppContext {
        user: user_context,
        db: DbContext { conn },
    });

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

fn create_cors_layer() -> CorsLayer {
    // CorsLayer::permissive()
    CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(vec![Method::POST, Method::GET, Method::OPTIONS])
        .allow_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            HeaderName::from_static("apollo-require-preflight"),
            HeaderName::from_static("apollo-query-plan"),
            HeaderName::from_static("x-apollo-operation-id"),
            HeaderName::from_static("x-apollo-operation-name"),
        ])
        .expose_headers(vec![
            HeaderName::from_static("x-cache"),
            HeaderName::from_static("x-cache-hit"),
        ])
        .max_age(Duration::from_secs(7200)) // 5 minutes
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
    Migrator::up(conn.as_ref(), None).await?;

    // Create loaders
    let loaders = Loaders::new(conn.clone());

    // Build GraphQL schema
    let schema: AppSchema = create_schema(loaders);

    // Define state
    let state = AppState {
        schema,
        conn,
        config: env.server.clone(),
    };

    // Build router
    let mut app = Router::new().route(
        "/graphql",
        get(graphql_handler).post(graphql_handler).with_state(state),
    );
    #[cfg(debug_assertions)]
    {
        app = app.route("/playground", get(graphiql));
    }
    app = app.layer(create_cors_layer());

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
        .map_err(|e| eyre!("Server error: {:?}", e))
}

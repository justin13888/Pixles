use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
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
use eyre::{eyre, Result, WrapErr};
use listenfd::ListenFd;
use schema::{create_schema, AppSchema};
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};

mod context;
mod environment;
mod graphql;
mod models;
mod schema;

async fn graphql_handler(
    State(schema): State<AppSchema>,
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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Load environment settings
    let env =
        Environment::load().map_err(|e| eyre!("Failed to load environment settings: {:?}", e))?;
    println!("Environment settings loaded: {:?}", env);
    // Build GraphQL schema
    let schema = create_schema();

    // Build router
    let app = Router::new()
        .route("/graphql", get(graphiql).post(graphql_handler))
        .with_state(schema)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::predicate(|_, _| true))
                .allow_methods([Method::GET, Method::POST]),
        );

    // Start server
    println!(
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

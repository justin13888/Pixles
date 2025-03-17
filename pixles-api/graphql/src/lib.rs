use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, Response};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::http::{header, HeaderName};
use axum::{
    extract::State,
    http::{HeaderMap, Method},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use context::{AppContext, DbContext, UserContext};
use environment::{Environment, ServerConfig};
use eyre::{eyre, Result};
use loaders::Loaders;
use schema::{create_schema, AppSchema};
use sea_orm::DatabaseConnection;
use state::AppState;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::trace;

mod auth;
mod constants;
mod context;
mod hash;
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

pub async fn get_graphql_router(
    conn: Arc<DatabaseConnection>,
    config: ServerConfig,
) -> Result<Router> {
    // Create loaders
    let loaders = Loaders::new(conn.clone());

    // Build GraphQL schema
    let schema: AppSchema = create_schema(loaders);

    // Define state
    let state = AppState {
        schema,
        conn,
        config,
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

    Ok(app)
}

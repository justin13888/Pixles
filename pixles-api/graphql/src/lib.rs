use std::sync::Arc;

use async_graphql::{Response, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use auth::config::AuthConfig;
use auth::service::AuthService;
use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderName, Method, header},
    response::{Html, IntoResponse},
    routing::{get, post},
};
use config::GraphqlServerConfig;
use context::{AppContext, DbContext, UserContext};
use eyre::Result;
use loaders::Loaders;
use schema::{AppSchema, create_schema};
use sea_orm::DatabaseConnection;
use state::AppState;
use std::time::Duration;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tracing::trace;

mod config;
mod constants;
mod context;
mod loaders;
mod schema;
mod state;

async fn graphql_handler(
    State(AppState {
        schema,
        conn,
        config,
        auth_service,
    }): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> impl IntoResponse {
    // Create user context
    let user_context = match UserContext::from_headers(&headers, &auth_service) {
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

async fn graphiql(endpoint: &str) -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(endpoint)
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

pub async fn get_router<C: Into<GraphqlServerConfig>>(
    conn: Arc<DatabaseConnection>,
    config: C,
    auth_config: AuthConfig,
) -> Result<Router> {
    let config = config.into();
    // Create loaders
    let loaders = Loaders::new(conn.clone());

    // Build GraphQL schema
    let schema: AppSchema = create_schema(loaders);

    // Define state
    let state = AppState {
        schema,
        conn,
        config,
        auth_service: Arc::new(AuthService::new(auth_config)),
    };

    // Build router
    let mut app = Router::new().route("/", post(graphql_handler).with_state(state));
    #[cfg(debug_assertions)]
    {
        app = app.route("/playground", get(|| graphiql("/v1/graphql")));
    }
    app = app.layer(create_cors_layer());

    Ok(app)
}

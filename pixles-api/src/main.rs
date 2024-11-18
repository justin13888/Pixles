use async_graphql::{
    http::GraphiQLSource, Context, EmptySubscription, ErrorExtensions, FieldResult, Object, Schema,
    SimpleObject, ID,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::{HeaderMap, Method},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use context::get_user_context_from_headers;
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::{AllowOrigin, CorsLayer};

mod context;

// Domain models
#[derive(SimpleObject, Clone, Serialize, Deserialize)]
struct Photo {
    id: ID,
    title: String,
    url: String,
}

#[derive(SimpleObject, Clone, Serialize, Deserialize)]
struct Album {
    id: ID,
    title: String,
    photos: Vec<Photo>,
}

// Application state
struct AppState {
    albums: RwLock<HashMap<ID, Album>>,
}

// GraphQL Query root
struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn album(&self, ctx: &Context<'_>, id: ID) -> FieldResult<Option<Album>> {
        // Check authorization
        // check_auth(ctx)?;

        let state = ctx.data::<Arc<AppState>>()?;
        let albums = state.albums.read().await;
        Ok(albums.get(&id).cloned())
    }

    async fn albums(&self, ctx: &Context<'_>) -> FieldResult<Vec<Album>> {
        // Check authorization
        // check_auth(ctx)?;

        let state = ctx.data::<Arc<AppState>>()?;
        let albums = state.albums.read().await;
        Ok(albums.values().cloned().collect())
    }
}

// GraphQL Mutation root
struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_album(&self, ctx: &Context<'_>, title: String) -> FieldResult<Album> {
        // Check authorization
        // check_auth(ctx)?;

        let state = ctx.data::<Arc<AppState>>()?;
        let mut albums = state.albums.write().await;

        let album = Album {
            id: ID::from(uuid::Uuid::new_v4().to_string()),
            title,
            photos: vec![],
        };

        albums.insert(album.id.clone(), album.clone());
        Ok(album)
    }

    async fn add_photo(
        &self,
        ctx: &Context<'_>,
        album_id: ID,
        title: String,
        url: String,
    ) -> FieldResult<Album> {
        // Check authorization
        // check_auth(ctx)?;

        let state = ctx.data::<Arc<AppState>>()?;
        let mut albums = state.albums.write().await;

        let album = albums.get_mut(&album_id).ok_or_else(|| {
            async_graphql::Error::new("Album not found").extend_with(|_, e| {
                e.set("code", "NOT_FOUND");
            })
        })?;

        let photo = Photo {
            id: ID::from(uuid::Uuid::new_v4().to_string()),
            title,
            url,
        };

        album.photos.push(photo);
        Ok(album.clone())
    }
}

async fn graphql_handler(
    State(schema): State<Schema<QueryRoot, MutationRoot, EmptySubscription>>,
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
async fn main() {
    // Initialize application state
    let state = Arc::new(AppState {
        albums: RwLock::new(HashMap::new()),
    });

    // Build GraphQL schema
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state)
        .finish();

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
    println!("GraphQL server running at http://localhost:3000/graphql");
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
        None => TcpListener::bind("127.0.0.1:3000").await.unwrap(),
    };

    // run it
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

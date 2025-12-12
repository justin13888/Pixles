// use std::{
//     fs,
//     io::Write,
//     path::Path,
//     sync::Arc,
//     time::{SystemTime, UNIX_EPOCH},
// };
// use axum::{
//     extract::{Multipart, State},
//     http::StatusCode,
//     Router,
// };
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use docs::TAGS;
use hyper::{HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use utoipa::{IntoParams, ToSchema};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

mod files;

// // TODO: Verify
// async fn upload_file(
//     State(state): State<Arc<AppState>>,
//     mut multipart: Multipart,
// ) -> Result<(StatusCode, String), UploadError> {
//     // TODO: remove
//     info!("Uploading file");
//     let mut uploaded_files = Vec::new();

//     while let Some(field) = multipart
//         .next_field()
//         .await
//         .map_err(|e| UploadError::ParseError(format!("Failed to process multipart form: {}", e)))?
//     {
//         // Extract field metadata
//         let file_name = match field.file_name() {
//             Some(name) => name.to_string(),
//             None => continue, // Skip fields without a filename
//         };

//         let content_type = field
//             .content_type()
//             .map(|ct| ct.to_string())
//             .unwrap_or_else(|| "application/octet-stream".to_string());

//         // Generate a unique ID and determine storage path
//         let file_id = Uuid::new_v4().to_string();
//         let storage_path = Path::new(&state.config.upload_dir).join(&file_id);

//         // TODO: remove
//         debug!("Processing file: {:?}, {:?}", file_id, storage_path);

//         // Read the file data and check size constraints
//         let data = field
//             .bytes()
//             .await
//             .map_err(|_| UploadError::ParseError(String::from("Failed to read file data")))?;

//         // Check file size
//         if data.len() > state.config.max_file_size {
//             return Err(UploadError::FileTooLarge);
//         }

//         // Check if adding this file would exceed the cache limit
//         if state.file_db.would_exceed_cache_limit(data.len()) {
//             return Err(UploadError::CacheFull);
//         }

//         // Save the file
//         let mut file = fs::File::create(&storage_path)?;
//         file.write_all(&data)?;

//         // Create and save metadata
//         let now = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .unwrap_or_default()
//             .as_secs();

//         let metadata = FileMetadata {
//             id: file_id.clone(),
//             original_filename: file_name,
//             content_type,
//             size: data.len(),
//             uploaded_at: now,
//             path: storage_path.to_string_lossy().to_string(),
//         };

//         state.file_db.save_metadata(&metadata).await?;

//         uploaded_files.push(file_id);
//     }

//     if uploaded_files.is_empty() {
//         Ok((
//             StatusCode::BAD_REQUEST,
//             String::from("No files were uploaded"),
//         ))
//     } else if uploaded_files.len() == 1 {
//         Ok((
//             StatusCode::CREATED,
//             format!("File uploaded with ID: {}", uploaded_files[0]),
//         ))
//     } else {
//         Ok((
//             StatusCode::CREATED,
//             format!("Files uploaded with IDs: {}", uploaded_files.join(", ")),
//         ))
//     }
// }

// TODO: Check all code below this line vv

/// In-memory todo store
type Store = Mutex<Vec<Todo>>;

/// Item to do.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
struct Todo {
    id: i32,
    #[schema(example = "Buy groceries")]
    value: String,
    done: bool,
}

/// Todo operation errors
#[derive(Serialize, Deserialize, ToSchema)]
enum TodoError {
    /// Todo already exists conflict.
    #[schema(example = "Todo already exists")]
    Conflict(String),
    /// Todo not found by id.
    #[schema(example = "id = 1")]
    NotFound(String),
    /// Todo operation unauthorized
    #[schema(example = "missing api key")]
    Unauthorized(String),
}

/// List all Todo items
///
/// List all Todo items from in-memory storage.
#[utoipa::path(
    get,
    path = "",
    tag = TAGS::UPLOAD,
    responses(
        (status = 200, description = "List all todos successfully", body = [Todo])
    )
)]
async fn list_todos(State(store): State<Arc<Store>>) -> Json<Vec<Todo>> {
    let todos = store.lock().await.clone();

    Json(todos)
}

/// Todo search query
#[derive(Deserialize, IntoParams)]
struct TodoSearchQuery {
    /// Search by value. Search is incase sensitive.
    value: String,
    /// Search by `done` status.
    done: bool,
}

/// Search Todos by query params.
///
/// Search `Todo`s by query params and return matching `Todo`s.
#[utoipa::path(
    get,
    path = "/search",
    tag = TAGS::UPLOAD,
    params(
        TodoSearchQuery
    ),
    responses(
        (status = 200, description = "List matching todos by query", body = [Todo])
    )
)]
async fn search_todos(
    State(store): State<Arc<Store>>,
    query: Query<TodoSearchQuery>,
) -> Json<Vec<Todo>> {
    Json(
        store
            .lock()
            .await
            .iter()
            .filter(|todo| {
                todo.value.to_lowercase() == query.value.to_lowercase() && todo.done == query.done
            })
            .cloned()
            .collect(),
    )
}

/// Create new Todo
///
/// Tries to create a new Todo item to in-memory storage or fails with 409 conflict if already exists.
#[utoipa::path(
    post,
    path = "",
    tag = TAGS::UPLOAD,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
async fn create_todo(State(store): State<Arc<Store>>, Json(todo): Json<Todo>) -> impl IntoResponse {
    let mut todos = store.lock().await;

    todos
        .iter_mut()
        .find(|existing_todo| existing_todo.id == todo.id)
        .map(|found| {
            (
                StatusCode::CONFLICT,
                Json(TodoError::Conflict(format!(
                    "todo already exists: {}",
                    found.id
                ))),
            )
                .into_response()
        })
        .unwrap_or_else(|| {
            todos.push(todo.clone());

            (StatusCode::CREATED, Json(todo)).into_response()
        })
}

/// Mark Todo item done by id
///
/// Mark Todo item done by given id. Return only status 200 on success or 404 if Todo is not found.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = TAGS::UPLOAD,
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 404, description = "Todo not found")
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        (), // <-- make optional authentication
        ("api_key" = [])
    )
)]
async fn mark_done(
    Path(id): Path<i32>,
    State(store): State<Arc<Store>>,
    headers: HeaderMap,
) -> StatusCode {
    match check_api_key(false, headers) {
        Ok(_) => (),
        Err(_) => return StatusCode::UNAUTHORIZED,
    }

    let mut todos = store.lock().await;

    todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| {
            todo.done = true;
            StatusCode::OK
        })
        .unwrap_or(StatusCode::NOT_FOUND)
}

/// Delete Todo item by id
///
/// Delete Todo item from in-memory storage by id. Returns either 200 success of 404 with TodoError if Todo is not found.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = TAGS::UPLOAD,
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 401, description = "Unauthorized to delete Todo", body = TodoError, example = json!(TodoError::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Todo not found", body = TodoError, example = json!(TodoError::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    )
)]
async fn delete_todo(
    Path(id): Path<i32>,
    State(store): State<Arc<Store>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match check_api_key(true, headers) {
        Ok(_) => (),
        Err(error) => return error.into_response(),
    }

    let mut todos = store.lock().await;

    let len = todos.len();

    todos.retain(|todo| todo.id != id);

    if todos.len() != len {
        StatusCode::OK.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(format!("id = {id}"))),
        )
            .into_response()
    }
}

// normally you should create a middleware for this but this is sufficient for sake of example.
fn check_api_key(
    require_api_key: bool,
    headers: HeaderMap,
) -> Result<(), (StatusCode, Json<TodoError>)> {
    match headers.get("todo_apikey") {
        Some(header) if header != "utoipa-rocks" => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("incorrect api key"))),
        )),
        None if require_api_key => Err((
            StatusCode::UNAUTHORIZED,
            Json(TodoError::Unauthorized(String::from("missing api key"))),
        )),
        _ => Ok(()),
    }
}

pub(super) fn get_router(state: AppState) -> OpenApiRouter {
    // TODO: Complete implementation
    let store = Arc::new(Store::default());
    OpenApiRouter::new()
        .routes(routes!(list_todos, create_todo))
        .routes(routes!(search_todos))
        .routes(routes!(mark_done, delete_todo))
        .with_state(store)
    // .with_state(Arc::new(state))
}

// TODO: Rewrite this completely later

use aide::OperationOutput;
use aide::axum::{
    ApiRouter,
    routing::{get, put},
};
use aide::openapi::{Operation, Response};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use docs::TAGS;
use hyper::{HeaderMap, StatusCode};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::state::AppState;

mod files;

/// In-memory todo store
type Store = Mutex<Vec<Todo>>;

/// Item to do.
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
struct Todo {
    id: i32,
    #[schemars(example = example_todo_value())]
    value: String,
    done: bool,
}

fn example_todo_value() -> String {
    "Buy groceries".to_string()
}

/// Todo operation errors
#[derive(Serialize, Deserialize, JsonSchema)]
enum TodoError {
    /// Todo already exists conflict.
    Conflict(String),
    /// Todo not found by id.
    NotFound(String),
    /// Todo operation unauthorized
    Unauthorized(String),
}

/// List all Todo items
async fn list_todos(State(store): State<Arc<Store>>) -> impl IntoResponse + OperationOutput {
    let todos = store.lock().await.clone();
    Json(todos)
}

/// Todo search query
#[derive(Deserialize, JsonSchema)]
struct TodoSearchQuery {
    /// Search by value. Search is incase sensitive.
    value: String,
    /// Search by `done` status.
    done: bool,
}

/// Search Todos by query params.
async fn search_todos(
    State(store): State<Arc<Store>>,
    query: Query<TodoSearchQuery>,
) -> impl IntoResponse + OperationOutput {
    Json(
        store
            .lock()
            .await
            .iter()
            .filter(|todo| {
                todo.value.to_lowercase() == query.value.to_lowercase() && todo.done == query.done
            })
            .cloned()
            .collect::<Vec<_>>(),
    )
}

struct CreateTodoResponse(axum::response::Response);

impl IntoResponse for CreateTodoResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

impl OperationOutput for CreateTodoResponse {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(201),
                Response {
                    description: "Todo created successfully".into(),
                    ..Default::default()
                },
            ),
            (
                Some(409),
                Response {
                    description: "Todo already exists".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

/// Create new Todo
async fn create_todo(
    State(store): State<Arc<Store>>,
    Json(todo): Json<Todo>,
) -> CreateTodoResponse {
    let mut todos = store.lock().await;

    let response = todos
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
        });

    CreateTodoResponse(response)
}

struct MarkDoneResponse(StatusCode);

impl IntoResponse for MarkDoneResponse {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

impl OperationOutput for MarkDoneResponse {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Todo marked as done".into(),
                    ..Default::default()
                },
            ),
            (
                Some(401),
                Response {
                    description: "Unauthorized - invalid API key".into(),
                    ..Default::default()
                },
            ),
            (
                Some(404),
                Response {
                    description: "Todo not found".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

/// Mark Todo item done by id
async fn mark_done(
    Path(id): Path<i32>,
    State(store): State<Arc<Store>>,
    headers: HeaderMap,
) -> MarkDoneResponse {
    match check_api_key(false, headers) {
        Ok(_) => (),
        Err(_) => return MarkDoneResponse(StatusCode::UNAUTHORIZED),
    }

    let mut todos = store.lock().await;

    let status = todos
        .iter_mut()
        .find(|todo| todo.id == id)
        .map(|todo| {
            todo.done = true;
            StatusCode::OK
        })
        .unwrap_or(StatusCode::NOT_FOUND);

    MarkDoneResponse(status)
}

struct DeleteTodoResponse(axum::response::Response);

impl IntoResponse for DeleteTodoResponse {
    fn into_response(self) -> axum::response::Response {
        self.0
    }
}

impl OperationOutput for DeleteTodoResponse {
    type Inner = Self;

    fn inferred_responses(
        _ctx: &mut aide::generate::GenContext,
        _operation: &mut Operation,
    ) -> Vec<(Option<u16>, Response)> {
        vec![
            (
                Some(200),
                Response {
                    description: "Todo deleted successfully".into(),
                    ..Default::default()
                },
            ),
            (
                Some(401),
                Response {
                    description: "Unauthorized - invalid or missing API key".into(),
                    ..Default::default()
                },
            ),
            (
                Some(404),
                Response {
                    description: "Todo not found".into(),
                    ..Default::default()
                },
            ),
        ]
    }
}

/// Delete Todo item by id
async fn delete_todo(
    Path(id): Path<i32>,
    State(store): State<Arc<Store>>,
    headers: HeaderMap,
) -> DeleteTodoResponse {
    match check_api_key(true, headers) {
        Ok(_) => (),
        Err(error) => return DeleteTodoResponse(error.into_response()),
    }

    let mut todos = store.lock().await;

    let len = todos.len();

    todos.retain(|todo| todo.id != id);

    let response = if todos.len() != len {
        StatusCode::OK.into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(format!("id = {id}"))),
        )
            .into_response()
    };

    DeleteTodoResponse(response)
}

fn check_api_key(
    require_api_key: bool,
    headers: HeaderMap,
) -> Result<(), (StatusCode, Json<TodoError>)> {
    match headers.get("todo_apikey") {
        Some(header) if header != "aide-rocks" => Err((
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

pub(super) fn get_router(_state: AppState) -> ApiRouter {
    let store = Arc::new(Store::default());
    ApiRouter::new()
        .api_route_with("/", get(list_todos).post(create_todo), |op| {
            op.tag(TAGS::UPLOAD).description("List or create todos")
        })
        .api_route_with("/search", get(search_todos), |op| {
            op.tag(TAGS::UPLOAD)
                .description("Search todos by query params")
        })
        .api_route_with("/{id}", put(mark_done).delete(delete_todo), |op| {
            op.tag(TAGS::UPLOAD).description("Mark todo done or delete")
        })
        .with_state(store)
}

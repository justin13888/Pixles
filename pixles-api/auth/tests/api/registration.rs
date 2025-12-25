use crate::common::setup;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use secrecy::ExposeSecret;
use tower::ServiceExt;

#[tokio::test]
async fn register_user_success() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/register")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "username": "testuser",
                        "name": "Test User",
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // Verify response body
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: auth::models::responses::TokenResponse =
        serde_json::from_slice(&body_bytes).unwrap();
    assert!(!tokens.access_token.expose_secret().is_empty());
    assert!(!tokens.refresh_token.expose_secret().is_empty());

    // Check DB for user
    let user = service::user::Query::find_user_by_email(&context.db, "test@example.com")
        .await
        .expect("Failed to query user")
        .expect("User should exist");

    assert_eq!(user.username, "testuser");
}

#[tokio::test]
async fn register_user_duplicate_email() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // First registration
    let _ = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/register")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "username": "testuser1",
                        "name": "Test User",
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Second registration with same email
    let response = app
        .oneshot(
            Request::builder()
                .uri("/register")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "username": "testuser2",
                        "name": "Test User 2",
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should fail (BadRequest or Conflict depends on implementation, likely BadRegisterUserRequestError which maps to ? 400 or 409)
    // The implementation returns RegisterUserResponses::UserAlreadyExists which maps to 409 Conflict commonly, or 400.
    // Let's check `errors.rs` or just assert generically.
    // Actually `RegisterUserResponses` derive IntoResponse.
    assert_eq!(response.status(), StatusCode::CONFLICT);
}

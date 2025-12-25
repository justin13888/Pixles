use crate::common::setup;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use secrecy::ExposeSecret;
use tower::ServiceExt;

#[tokio::test]
async fn login_user_success() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // Register user
    let _ = app
        .clone()
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

    // Login
    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "email": "test@example.com",
                        "password": "password123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: auth::models::responses::TokenResponse =
        serde_json::from_slice(&body_bytes).unwrap();
    assert!(!tokens.access_token.expose_secret().is_empty());
    assert!(!tokens.refresh_token.expose_secret().is_empty());
}

#[tokio::test]
async fn login_user_wrong_password() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // Register user
    let _ = app
        .clone()
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

    // Login wrong password
    let response = app
        .oneshot(
            Request::builder()
                .uri("/login")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "email": "test@example.com",
                        "password": "wrongpassword"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Expect 401 Unauthorized
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

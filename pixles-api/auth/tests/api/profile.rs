use crate::common::setup;
use auth::models::UserProfile;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use secrecy::ExposeSecret;
use tower::ServiceExt;

#[tokio::test]
async fn get_user_profile_success() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // Register
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
        .clone()
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

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: auth::models::responses::TokenResponse =
        serde_json::from_slice(&body_bytes).unwrap();
    let token = tokens.access_token;

    // Get Profile
    let response = app
        .oneshot(
            Request::builder()
                .uri("/profile")
                .method("GET")
                .header("Authorization", format!("Bearer {}", token.expose_secret()))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let profile: UserProfile = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(profile.username, "testuser");
    assert_eq!(profile.email, "test@example.com");
}

#[tokio::test]
async fn update_user_profile_success() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // Register & Login setup (simplified for brevity, should use helper if possible but inline is fine)
    // Register
    let _ = app.clone()
        .oneshot(
            Request::builder()
                .uri("/register")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&serde_json::json!({
                    "username": "testuser", "name": "Test User", "email": "test@example.com", "password": "password123"
                })).unwrap()))
                .unwrap(),
        )
        .await.unwrap();

    // Login
    let resp = app.clone().oneshot(
        Request::builder().uri("/login").method("POST").header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&serde_json::json!({"email": "test@example.com", "password": "password123"})).unwrap())).unwrap()
    ).await.unwrap();
    let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
        .await
        .unwrap();
    let tokens: auth::models::responses::TokenResponse = serde_json::from_slice(&body).unwrap();
    let token = tokens.access_token;

    // Update Profile
    let response = app
        .oneshot(
            Request::builder()
                .uri("/profile")
                .method("POST")
                .header("Authorization", format!("Bearer {}", token.expose_secret()))
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "username": "newusername"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Verify DB
    let user = service::user::Query::find_user_by_email(&context.db, "test@example.com")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user.username, "newusername");
}

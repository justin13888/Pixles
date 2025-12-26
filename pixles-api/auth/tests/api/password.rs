use crate::common::setup;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn password_reset_flow() {
    let context = setup().await;
    let app = auth::get_router(context.db.clone(), context.app_state.config.clone())
        .await
        .expect("Failed to create router");
    let app: axum::Router = app.into();

    // Register
    let _ = app.clone()
        .oneshot(
            Request::builder()
                .uri("/register")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(serde_json::to_vec(&serde_json::json!({
                    "username": "resetuser", "name": "Reset User", "email": "reset@example.com", "password": "oldpassword"
                })).unwrap()))
                .unwrap(),
        )
        .await.unwrap();

    // 1. Request Password Reset
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/password-reset-request")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "email": "reset@example.com"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Retrieve token from DB
    // We need to use `pixles_api_service` which might not be exposed as helper in `context` but we have `context.db`.
    // I need `pixles_api_entity` to query. `pixles-api-auth` depends on it.
    // I can use `pixles_api_service::UserService`.
    let user = service::user::Query::find_user_by_email(&context.db, "reset@example.com")
        .await
        .expect("Failed to query user")
        .expect("User should exist");

    let reset_token = user.password_reset_token.expect("Should have reset token");

    // 2. Reset Password
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/password-reset")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "token": reset_token,
                        "new_password": "newpassword123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 3. Login with New Password
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/login")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&serde_json::json!({
                        "email": "reset@example.com",
                        "password": "newpassword123"
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

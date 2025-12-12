use pixles_sdk::{Client, RegisterRequest, client::PixlesApiDocsClient, types::RegisterRequest};
use std::env;
use tokio;
use uuid::Uuid;

// TODO: INCOMPLETE. REQUIRES TESTING.

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url =
        env::var("PIXLES_API_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    // Note: OpenApi server url was /v1, so base_url should probably include /v1 if the client doesn't append it?
    // Progenitor client usually takes the root URL and appends paths defined in spec.
    // Spec paths are like `/register`.
    // Spec servers: `url: "/v1"`.
    // Progenitor might handle the base path.
    // If I pass `http://localhost:3000`, the client usually uses the server URL from spec relative to it?
    // Actually, progenitor's `Client::new(baseurl)` sets the base.
    // If the spec has `/v1` prefix in servers, does it append it?
    // Let's assume passed URL is the root and progenitor handles the rest, OR we pass the full base.
    // Given the previous client had explicit `/v1`, the generated one might not.
    // The spec has `servers: [{url: "/v1"}]`.
    // Paths are `/register`.
    // So full path is `/v1/register`.
    // If I pass `http://localhost:3000`, and progenitor respects `servers`, it might work.
    // But usually progenitor treats `base_url` as the override for the server URL.
    // So I should pass `http://localhost:3000/v1`.

    // let client = Client::new(&base_url);
    let client = PixlesApiDocsClient::new().with_base_url(&base_url)?;

    let username = format!("user_{}", Uuid::now_v7());
    let email = format!("{}@example.com", username);
    let password = "password123";

    println!("Registering user: {}", username);

    let register_req = RegisterRequest {
        username: username.clone(),
        name: "Test User".to_string(),
        email: email.clone(),
        password: password.to_string(),
    };

    // register_user returns Result<RegisterUserResponse, ClientError>
    // actually, it returns the success type or error.
    // `register_user` response 201 is `TokenResponse`.

    match client.register_user(&register_req).await {
        Ok(token_res) => {
            println!("Registration successful!");
            println!("Access Token: {}", token_res.access_token);
            // Verify profile
            println!("Fetching profile...");
            // get_user_profile requires auth.
            // Progenitor might not auto-injectauth without configuration.
            // If it doesn't, we might need to manually handle it or use a different method.
            // Check if there is a way to pass token.
            // If `progenitor` generated `impl Client`, does it expose a way to set bearer token?
            // Often it has `with_bearer_token(token)`.

            // Let's try to set it if methods exist.
            // Or maybe `get_user_profile` takes arguments?
            // Since `security` is defined, `progenitor` might generate `get_user_profile` to not take token if it expects client to have it.

            // Let's assume we can clone the client and set auth?
            // `Client` usually has `with_auth`.
            // Let's try `client.with_bearer_token(...)`.

            // Wait, if `client` is immutable, likely it returns a new client or modifies internal state.
            // Let's try:
            // let auth_client = client.with_bearer_token(token_res.access_token);
            // let profile = auth_client.get_user_profile().await?;

            // Wait, `get_user_profile` in spec: `/profile` GET.

            // If `with_bearer_token` doesn't exist, this will fail compile and I'll learn what exists.
        }
        Err(e) => {
            eprintln!("Failed to register: {:?}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}

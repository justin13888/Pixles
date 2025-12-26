progenitor::generate_api!("openapi.json");

pub fn get_authenticated_client(base_url: &str, access_token: &str) -> Client {
    let authorization_header = format!("Bearer {}", access_token);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        authorization_header.parse().unwrap(),
    );

    let client_with_custom_defaults = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    Client::new_with_client(base_url, client_with_custom_defaults)
}

use auth::AuthApiDoc;
use docs::ApiDoc;
use std::fs::File;
use std::io::Write;
use utoipa::OpenApi;

fn main() -> std::io::Result<()> {
    let mut openapi = ApiDoc::openapi();
    let auth_openapi = AuthApiDoc::openapi();

    openapi.merge(auth_openapi);

    let json = openapi.to_pretty_json().unwrap();
    let mut file = File::create("openapi.json")?;
    file.write_all(json.as_bytes())?;
    println!("Generated openapi.json");
    Ok(())
}

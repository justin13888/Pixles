use environment::Environment;
use eyre::Result;
use pixles_api::create_app;
use sea_orm::Database;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment settings
    let env = Environment::load()
        .map_err(|e| eyre::eyre!("Failed to load environment settings: {:?}", e))?;

    // Initialize database connection
    // Note: This requires a running database. For SDK generation in CI/CD,
    // we might need to mock this or ensure DB is present.
    let conn = Database::connect(env.database.url.clone()).await?;

    // Create the app router and spec
    let (_, api) = create_app(conn, &env).await?;

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&api).unwrap();

    // Write to file
    let mut file = File::create("openapi.json")?;
    file.write_all(json.as_bytes())?;

    println!("Generated openapi.json");
    Ok(())
}

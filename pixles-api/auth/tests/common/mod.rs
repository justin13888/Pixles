use auth::config::AuthConfig;
use auth::state::AppState;
use migration::Migrator;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::sync::Once;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
use testcontainers_modules::postgres::Postgres;

static TRACING: Once = Once::new();

pub struct TestContext {
    pub post: Option<ContainerAsync<Postgres>>,
    pub valkey: Option<ContainerAsync<GenericImage>>,
    pub app_state: AppState,
    pub db: DatabaseConnection,
}

pub async fn setup() -> TestContext {
    TRACING.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("info,sqlx=error,sea_orm=error")
            .with_test_writer()
            .init();
    });

    // Start Postgres or use external
    let (postgres_container, connection_string) =
        if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
            (None, url)
        } else {
            let container = Postgres::default()
                .with_tag("17")
                .start()
                .await
                .expect("Failed to start Postgres");
            let port = container
                .get_host_port_ipv4(5432)
                .await
                .expect("Failed to get Postgres port");
            (
                Some(container),
                format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port),
            )
        };

    // Connect and Migrate
    let db = Database::connect(&connection_string)
        .await
        .expect("Failed to connect to database");

    // Use refresh to ensure clean state for tests
    Migrator::refresh(&db)
        .await
        .expect("Failed to run migrations");

    // Start Valkey or use external
    let (valkey_container, valkey_url) = if let Ok(url) = std::env::var("TEST_VALKEY_URL") {
        (None, url)
    } else {
        let container = GenericImage::new("valkey/valkey", "8.0.1")
            .with_exposed_port(testcontainers::core::ContainerPort::Tcp(6379))
            .with_wait_for(testcontainers::core::WaitFor::message_on_stdout(
                "Ready to accept connections",
            ))
            .start()
            .await
            .expect("Failed to start Valkey");
        let port = container
            .get_host_port_ipv4(6379)
            .await
            .expect("Failed to get Valkey port");
        (Some(container), format!("redis://127.0.0.1:{}", port))
    };

    // Create AppState
    // We need keys. For tests we can generate random ones or use fixed ones.
    // Since AuthConfig expects EncodingKey/DecodingKey which need keys.
    // Let's generate ephemeral keys.

    // NOTE: This part requires `ring` or similar to generate keys if we want to mimic real setup
    // Or we can just load dummy ones.
    // AuthConfig::from(&ServerConfig) logic uses loaded config.
    // We'll construct AuthConfig manually or via a helper.

    // For simplicity, let's create a minimal config.
    // BUT AuthConfig structs fields are pub, so we can instantiate directly.
    // Generate ephemeral keys provided by user history or just static for tests.
    // Private: MC4CAQAwBQYDK2VwBCIEIN6eTvXEL7xMZWHY8rTk7VbQSGSuRkle5MVfiiYUStLF
    // Public: MCowBQYDK2VwAyEA66iVaMz1x2ogToGm5Hw34aITBLLqz0iEonbwjK57pWU=

    use base64::Engine;
    let engine = base64::engine::general_purpose::STANDARD;

    let priv_bytes = engine
        .decode("MC4CAQAwBQYDK2VwBCIEIN6eTvXEL7xMZWHY8rTk7VbQSGSuRkle5MVfiiYUStLF")
        .expect("Failed to decode priv key");
    let pub_bytes = engine
        .decode("MCowBQYDK2VwAyEA66iVaMz1x2ogToGm5Hw34aITBLLqz0iEonbwjK57pWU=")
        .expect("Failed to decode pub key");

    let enc_key = jsonwebtoken::EncodingKey::from_ed_der(&priv_bytes);
    let dec_key = jsonwebtoken::DecodingKey::from_ed_der(&pub_bytes);

    let config = AuthConfig {
        host: "127.0.0.1".to_string(),
        port: 0,
        domain: "localhost".to_string(),
        jwt_eddsa_encoding_key: enc_key,
        jwt_eddsa_decoding_key: dec_key,
        jwt_refresh_token_duration_seconds: 60,
        jwt_access_token_duration_seconds: 10,
        valkey_url: valkey_url.clone(),
    };

    let session_manager =
        auth::session::SessionManager::new(valkey_url.clone(), std::time::Duration::from_secs(60))
            .await
            .expect("Failed to create session manager");

    let email_service = auth::service::EmailService::new();

    let app_state = AppState::new(db.clone(), config, session_manager, email_service);

    TestContext {
        post: postgres_container,
        valkey: valkey_container,
        app_state,
        db,
    }
}

use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tambahkan_ini_ke_database_my_api::{AppState, app, config::app::AppConfig};

pub struct TestContext {
    pub state: AppState,
}

impl TestContext {
    pub async fn new() -> Self {
        dotenvy::dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        // Use a small pool for testing
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("Failed to connect to test database");

        // We might want to run migrations or clear tables here
        // For now, let's assume the test DB is ready or handled by the user

        let config = AppConfig::global();

        let cache = moka::future::Cache::builder()
            .max_capacity(100)
            .time_to_live(std::time::Duration::from_secs(60))
            .build();

        let state = AppState {
            pool,
            config,
            cache,
        };

        Self { state }
    }
}

pub async fn setup_app() -> (Router, PgPool) {
    let context = TestContext::new().await;
    let pool = context.state.pool.clone();
    (app(context.state), pool)
}

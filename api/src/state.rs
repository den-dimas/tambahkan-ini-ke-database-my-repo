use crate::config::app::AppConfig;
use moka::future::Cache;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: &'static AppConfig,
    pub cache: Cache<String, serde_json::Value>,
    pub s3_client: aws_sdk_s3::Client,
}

impl axum::extract::FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for &'static AppConfig {
    fn from_ref(state: &AppState) -> Self {
        state.config
    }
}

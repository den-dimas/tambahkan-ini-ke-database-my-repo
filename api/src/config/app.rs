use std::{env, sync::OnceLock};

use dotenvy::dotenv;

#[derive(Debug)]
pub struct AppConfig {
    pub db_url: String,
    pub server_addr: String,
    pub server_port: String,
    pub jwt_secret: String,
    pub cors_origins: Vec<String>,
    pub r2_access_key_id: String,
    pub r2_secret_access_key: String,
    pub r2_endpoint: String,
    pub r2_bucket_name: String,
    pub r2_public_domain: Option<String>,
}

pub static APP_CONFIG: OnceLock<AppConfig> = OnceLock::new();

impl AppConfig {
    pub fn global() -> &'static AppConfig {
        APP_CONFIG.get_or_init(|| AppConfig::load_from_env())
    }

    fn load_from_env() -> AppConfig {
        dotenv().ok();

        let cors_origins = env::var("CORS_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        AppConfig {
            db_url: env::var("DB_URL").expect("DB_URL must be set."),
            server_addr: env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            server_port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "change_me_in_production".to_string()),
            cors_origins,
            r2_access_key_id: env::var("R2_ACCESS_KEY_ID").expect("R2_ACCESS_KEY_ID must be set"),
            r2_secret_access_key: env::var("R2_SECRET_ACCESS_KEY")
                .expect("R2_SECRET_ACCESS_KEY must be set"),
            r2_endpoint: env::var("R2_ENDPOINT").expect("R2_ENDPOINT must be set"),
            r2_bucket_name: env::var("R2_BUCKET_NAME").expect("R2_BUCKET_NAME must be set"),
            r2_public_domain: env::var("R2_PUBLIC_DOMAIN").ok(),
        }
    }
}

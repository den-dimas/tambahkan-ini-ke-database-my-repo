use std::{env, sync::OnceLock};

use dotenvy::dotenv;

#[derive(Debug)]
pub struct AppConfig {
    pub db_url: String,
    pub server_addr: String,
    pub server_port: String,
    pub jwt_secret: String,
    pub cors_origins: Vec<String>,
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
        }
    }
}

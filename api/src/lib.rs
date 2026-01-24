use sqlx::PgPool;
use std::net::SocketAddr;

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, header};
use moka::future::Cache;
use tokio::net::TcpListener;
use tower_governor::GovernorLayer;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::app::AppConfig;
use crate::state::AppState;

pub mod config;
pub mod database;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod services;
pub mod state;
pub mod utils;

pub async fn serve() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "tambahkan_ini_ke_database_my_api=debug,tower_http=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AppConfig::global();

    tracing::info!("Connecting to database at {}...", config.db_url);

    let pool = PgPool::connect(&config.db_url)
        .await
        .expect("Failed to connect to the database");

    let cache = Cache::builder()
        .max_capacity(1000)
        .time_to_live(std::time::Duration::from_secs(300)) // 5 minutes
        .build();

    let s3_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .credentials_provider(aws_sdk_s3::config::Credentials::new(
            &config.r2_access_key_id,
            &config.r2_secret_access_key,
            None,
            None,
            "Static",
        ))
        .endpoint_url(&config.r2_endpoint)
        .region(aws_sdk_s3::config::Region::new("auto"))
        .load()
        .await;

    let s3_client = aws_sdk_s3::Client::new(&s3_config);

    let state = AppState {
        pool,
        config,
        cache,
        s3_client,
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port.parse::<u16>().unwrap()));
    tracing::info!("Server listening on {}.", addr);

    let app = app(state);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

pub fn app(state: AppState) -> axum::Router {
    let cors = CorsLayer::new()
        .allow_origin(
            state
                .config
                .cors_origins
                .iter()
                .map(|s| s.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods(Any)
        .allow_headers(Any);

    let governor_config = crate::middleware::rate_limit::create_governor_config();

    axum::Router::new()
        .nest(
            "/api/v1",
            axum::Router::new()
                .nest("/auth", crate::routes::auth::routes())
                .nest("/people", crate::routes::people::routes()),
        )
        .layer(GovernorLayer {
            config: governor_config,
        })
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains"),
        ))
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB limit
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

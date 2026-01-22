use sqlx::PgPool;
use std::net::SocketAddr;

use axum::extract::{ConnectInfo, DefaultBodyLimit};
use axum::http::{HeaderValue, header};
use moka::future::Cache;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::app::AppConfig;

pub mod config;
pub mod database;
pub mod models;
pub mod routes;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: &'static AppConfig,
    pub cache: Cache<String, serde_json::Value>,
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

    let state = AppState {
        pool,
        config,
        cache,
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

    use axum::http::Request;
    use tower_governor::key_extractor::KeyExtractor;

    #[derive(Clone, Copy)]
    struct PeerAddrExtractor;

    impl KeyExtractor for PeerAddrExtractor {
        type Key = std::net::IpAddr;

        fn extract<B>(
            &self,
            req: &Request<B>,
        ) -> Result<Self::Key, tower_governor::errors::GovernorError> {
            req.extensions()
                .get::<axum::extract::ConnectInfo<SocketAddr>>()
                .map(|ConnectInfo(addr)| addr.ip())
                .or_else(|| {
                    // Fallback for tests or when ConnectInfo is missing
                    Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
                })
                .ok_or(tower_governor::errors::GovernorError::UnableToExtractKey)
        }
    }

    let governor_config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .key_extractor(PeerAddrExtractor)
            .finish()
            .unwrap(),
    );

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

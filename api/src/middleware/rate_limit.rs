use axum::extract::ConnectInfo;
use axum::http::Request;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_governor::governor::{GovernorConfig, GovernorConfigBuilder};
use tower_governor::key_extractor::KeyExtractor;

#[derive(Clone, Copy)]
pub struct PeerAddrExtractor;

impl KeyExtractor for PeerAddrExtractor {
    type Key = std::net::IpAddr;

    fn extract<B>(
        &self,
        req: &Request<B>,
    ) -> Result<Self::Key, tower_governor::errors::GovernorError> {
        req.extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ConnectInfo(addr)| addr.ip())
            .or_else(|| {
                // Fallback for tests or when ConnectInfo is missing
                Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)))
            })
            .ok_or(tower_governor::errors::GovernorError::UnableToExtractKey)
    }
}

use governor::middleware::NoOpMiddleware;

pub fn create_governor_config() -> Arc<GovernorConfig<PeerAddrExtractor, NoOpMiddleware>> {
    Arc::new(
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(5)
            .key_extractor(PeerAddrExtractor)
            .finish()
            .unwrap(),
    )
}

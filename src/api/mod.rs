pub mod polymarket_api;
pub mod predyx;
pub mod ordinals;

use async_trait::async_trait;
use anyhow::Result;

pub use crate::api::polymarket_api::PolymarketClient;
pub use crate::api::predyx::PredyxRealClient;
pub use crate::api::ordinals::OrdinalsMarketplace;
pub use crate::models::{Market, MarketOdds};

#[async_trait]
pub trait MarketClient: Send + Sync {
    async fn fetch_markets(&self) -> Result<Vec<Market>>;
    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>>;
    fn is_configured(&self) -> bool;
}

#[async_trait]
impl MarketClient for PolymarketClient {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        self.fetch_markets().await
    }

    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        self.fetch_odds(market_id).await
    }

    fn is_configured(&self) -> bool {
        self.is_configured()
    }
}

#[async_trait]
impl MarketClient for PredyxRealClient {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        self.fetch_markets().await
    }

    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        self.fetch_odds(market_id).await
    }

    fn is_configured(&self) -> bool {
        self.is_configured()
    }
}

#[async_trait]
impl MarketClient for OrdinalsMarketplace {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        self.fetch_markets().await
    }

    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        self.fetch_odds(market_id).await
    }

    fn is_configured(&self) -> bool {
        self.is_configured()
    }
}

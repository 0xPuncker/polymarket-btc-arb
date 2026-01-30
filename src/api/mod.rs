pub mod polymarket;

use async_trait::async_trait;
use anyhow::Result;

pub use crate::api::polymarket::PolymarketClient;
pub use crate::models::{Market, MarketOdds};

#[async_trait]
pub trait MarketClient: Send + Sync {
    async fn fetch_markets(&self) -> Result<Vec<Market>>;
    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>>;
    fn is_configured(&self) -> bool;
}

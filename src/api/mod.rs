pub mod polymarket;
pub mod btc;

use async_trait::async_trait;
use anyhow::Result;

use crate::models::{Market, MarketOdds};

#[async_trait]
pub trait MarketClient: Send + Sync {
    async fn fetch_markets(&self) -> Result<Vec<Market>>;
    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>>;
}

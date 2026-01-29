use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::warn;

use crate::models::{Market, MarketOdds};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdinalsMarketplace {
    pub endpoint: String,
    pub wallet_address: String,
}

impl OrdinalsMarketplace {
    pub fn new(endpoint: String, wallet_address: String) -> Self {
        Self {
            endpoint,
            wallet_address,
        }
    }

    pub async fn fetch_markets(&self) -> Result<Vec<Market>> {
        warn!("Ordinals marketplace not implemented yet - returning empty markets");

        // TODO: Implement actual API calls
        // - Connect to Gamma API or other Ordinals marketplace
        // - Fetch available prediction markets
        // - Convert to our Market format

        Ok(vec![])
    }

    pub async fn fetch_odds(&self, _market_id: &str) -> Result<Vec<MarketOdds>> {
        warn!("Ordinals odds not implemented yet");
        Ok(vec![])
    }

    pub fn is_configured(&self) -> bool {
        !self.endpoint.is_empty() && !self.wallet_address.is_empty()
    }
}

impl Default for OrdinalsMarketplace {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            wallet_address: String::new(),
        }
    }
}
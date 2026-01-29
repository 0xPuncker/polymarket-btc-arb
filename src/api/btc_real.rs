use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn, debug};
use rust_decimal::Decimal;

use crate::models::{Market, MarketOdds};
use crate::models::btc_market::BtcMarketType;
use crate::api::MarketClient;

/// Predyx - Lightning Network prediction markets
///
/// API Documentation: https://beta.predyx.com/docs
/// Note: Currently in beta, requires API key
pub struct PredyxClient {
    client: Client,
    api_key: Option<String>,
    base_url: String,
}

impl PredyxClient {
    pub fn new(api_key: Option<String>) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://beta.predyx.com/api/v1".to_string(),
        }
    }

    /// Fetch all markets
    pub async fn fetch_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/markets", self.base_url);

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("X-API-Key", key);
        }

        let response = request
            .send()
            .await?
            .error_for_status()?;

        let markets_response: PredyxMarketsResponse = response.json().await?;

        // Convert to our Market format
        let markets: Vec<Market> = markets_response
            .data
            .into_iter()
            .map(|m| Market {
                id: m.market_id,
                question: m.question,
                description: Some(m.description),
                outcomes: m.outcomes.clone(),
                end_time: m.end_date,
                volume: Some(m.volume),
                liquidity: None,
            })
            .collect();

        info!("Fetched {} markets from Predyx", markets.len());
        Ok(markets)
    }

    /// Fetch odds for a specific market
    pub async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        let url = format!("{}/markets/{}?include_orderbook=true", self.base_url, market_id);

        let mut request = self.client.get(&url);

        if let Some(key) = &self.api_key {
            request = request.header("X-API-Key", key);
        }

        let response = request
            .send()
            .await?
            .error_for_status()?;

        let market_response: PredyxMarketResponse = response.json().await?;

        let mut odds = Vec::new();

        if let Some(orderbook) = &market_response.order_book {
            for outcome in &market_response.outcomes {
                if let Some(orders) = orderbook.orders.get(&outcome.to_lowercase()) {
                    // Get best bid/ask prices
                    let best_bid = orders.bids.first();
                    let best_ask = orders.asks.first();

                    if let (Some(bid), Some(ask)) = (best_bid, best_ask) {
                        let price = (bid.price + ask.price) / Decimal::from(2);

                        odds.push(MarketOdds {
                            market_id: market_response.market_id.clone(),
                            outcome: outcome.clone(),
                            odds: price,
                            source: crate::models::MarketSource::BitcoinPredictionMarket,
                            timestamp: chrono::Utc::now(),
                        });
                    }
                }
            }
        }

        info!("Fetched {} odds from Predyx", odds.len());
        Ok(odds)
    }

    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }
}

// Predyx API response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredyxMarketsResponse {
    pub data: Vec<PredyxMarket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredyxMarket {
    pub market_id: String,
    pub question: String,
    pub description: String,
    pub outcomes: Vec<String>,
    pub end_date: Option<String>,
    pub volume: f64,
    pub order_book: Option<PredyxOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredyxOrderBook {
    #[serde(rename = "orderBook")]
    pub orders: serde_json::Value,
}

// Ordinals marketplace integration (placeholder)
pub struct OrdinalsMarketplace {
    endpoint: String,
    wallet_address: String,
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

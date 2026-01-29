use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

use crate::models::{Market, MarketOdds};

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
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredyxMarketResponse {
    #[serde(rename = "orderBook")]
    pub order_book: Option<PredyxOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredyxOrderBook {
    pub orders: serde_json::Map<String, serde_json::Value>,
}

/// Predyx - Lightning Network prediction markets
///
/// Live API: https://beta.predyx.com/api/v1
/// Beta requires API key
///
/// Features:
/// - Elections, sports, AI prediction markets
/// - Lightning Network payouts
/// - Real-time order books
pub struct PredyxRealClient {
    client: reqwest::Client,
    api_key: Option<String>,
    base_url: String,
}

impl PredyxRealClient {
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

        let markets: Vec<Market> = markets_response
            .data
            .into_iter()
            .map(|m| Market {
                id: m.market_id.clone(),
                question: m.question.clone(),
                description: Some(m.description.clone()),
                outcomes: m.outcomes.clone(),
                end_time: m.end_date.as_ref().and_then(|d| d.parse().ok()),
                volume: Some(Decimal::from(m.volume)),
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

        if let Some(orderbook) = market_response.order_book {
            if let Some(prices) = orderbook.orders.get("outcome_prices") {
                if let Some(price_obj) = prices.as_object() {
                    for (outcome, price_value) in price_obj {
                        if let Some(price_str) = price_value.as_str() {
                            if let Ok(price) = price_str.parse::<f64>() {
                                odds.push(MarketOdds {
                                    market_id: market_response.market_id.clone(),
                                    outcome: outcome.clone(),
                                    odds: Decimal::from(price / 100.0),
                                    source: crate::models::MarketSource::BitcoinPredictionMarket,
                                    timestamp: Utc::now(),
                                });
                            }
                        }
                    }
                }
            }
        }

        info!("Fetched {} odds from Predyx", odds.len());
        Ok(odds)
    }

    pub fn is_configured(&self) -> bool {
        self.api_key.is_some() && !self.api_key.as_ref().unwrap().is_empty()
    }

    /// Execute a trade on Predyx
    ///
    /// This would require Lightning Network wallet integration
    /// For now, returns a mock result
    pub async fn execute_trade(&self, market_id: &str, outcome: &str, amount: f64) -> Result<String> {
        info!("Executing Predyx trade: {} outcome: {} amount: {}", market_id, outcome, amount);

        // TODO: Implement actual Lightning Network payment
        // Steps:
        // 1. Get Lightning invoice from market
        // 2. Pay invoice via LN wallet
        // 3. Wait for confirmation
        // 4. Submit payment proof to market

        warn!("Predyx trade execution not implemented - returning mock tx hash");

        // Mock transaction hash
        Ok(format!("predyx_{}_{}_{:x}", market_id, outcome, chrono::Utc::now().timestamp()))
    }
}

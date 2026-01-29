use anyhow::Result;
use reqwest::Client;
use async_trait::async_trait;

use crate::api::MarketClient;
use crate::models::{Market, MarketOdds};
use crate::api::polymarket_response::PolymarketMarketsResponse;
use crate::api::polymarket_response::PolymarketMarketResponse;
use crate::api::polymarket_response::PolymarketOrderBook;

const GAMMA_API_BASE: &str = "https://gamma-api.polymarket.com";

/// Polymarket API client
pub struct PolymarketClient {
    client: Client,
}

impl PolymarketClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

/// Polymarket API response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarketsResponse {
    pub markets: Vec<PolymarketMarket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarket {
    pub id: String,
    pub question: String,
    pub description: String,
    pub outcomes: Vec<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub volume: f64,
    #[serde(rename = "orderBook")]
    pub order_book: Option<PolymarketOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarketResponse {
    pub id: String,
    pub question: String,
    pub outcomes: Vec<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub volume: f64,
    #[serde(rename = "orderBook")]
    pub order_book: Option<PolymarketOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketOrderBook {
    pub outcome: String,
    pub bids: Vec<PolymarketPriceLevel>,
    #[serde(rename = "best_bid")]
    pub best_bid: Option<PolymarketPriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketPriceLevel {
    pub price: f64,
}

impl PolymarketClient {
    /// Fetch all markets from Gamma API
    pub async fn fetch_raw_markets(&self) -> Result<PolymarketMarketsResponse> {
        let url = format!("{}/markets", GAMMA_API_BASE);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let response: PolymarketMarketsResponse = response.json().await?;
        Ok(response)
    }

    /// Fetch order books for a specific market
    pub async fn fetch_raw_market(&self, market_id: &str) -> Result<PolymarketMarketResponse> {
        let url = format!("{}/markets/{}", GAMMA_API_BASE, market_id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let market: PolymarketMarketResponse = response.json().await?;
        Ok(market)
    }
}

#[async_trait]
impl MarketClient for PolymarketClient {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        let response = self.fetch_raw_markets().await?;

        let markets: Vec<Market> = response
            .markets
            .into_iter()
            .map(|m| Market {
                id: m.id.clone(),
                question: m.question.clone(),
                description: Some(m.description.clone()),
                outcomes: m.outcomes.clone(),
                end_time: m.end_date.as_ref().and_then(|d| d.parse().ok()),
                volume: Some(rust_decimal::Decimal::from(m.volume)),
                liquidity: None,
            })
            .collect();

        Ok(markets)
    }

    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        let market = self.fetch_raw_market(market_id).await?;
        let mut odds = Vec::new();

        for order_book in market.order_book {
            if let Some(best_bid) = order_book.best_bid {
                odds.push(MarketOdds {
                    market_id: market.id.clone(),
                    outcome: order_book.outcome.clone(),
                    odds: rust_decimal::Decimal::from(best_bid.price / 100.0),
                    source: crate::models::MarketSource::Polymarket,
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        Ok(odds)
    }

    fn is_configured(&self) -> bool {
        true
    }
}

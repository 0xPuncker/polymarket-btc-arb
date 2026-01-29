use anyhow::Result;
use reqwest::Client;
use async_trait::async_trait;

use crate::api::MarketClient;
use crate::models::{Market, MarketOdds, MarketSource};
use crate::models::polymarket::PolymarketMarket;

const GAMMA_API_BASE: &str = "https://gamma-api.polymarket.com";

pub struct PolymarketClient {
    client: Client,
}

impl PolymarketClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    async fn fetch_raw_markets(&self) -> Result<Vec<PolymarketMarket>> {
        let url = format!("{}/markets", GAMMA_API_BASE);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let markets: Vec<PolymarketMarket> = response.json().await?;
        Ok(markets)
    }
}

impl Default for PolymarketClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MarketClient for PolymarketClient {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        let raw = self.fetch_raw_markets().await?;

        let markets: Vec<Market> = raw
            .into_iter()
            .map(|m| Market {
                id: m.id,
                question: m.question,
                description: m.description,
                outcomes: m.outcomes,
                end_time: m.end_time,
                volume: m.volume,
                liquidity: m.liquidity,
            })
            .collect();

        Ok(markets)
    }

    async fn fetch_odds(&self, market_id: &str) -> Result<Vec<MarketOdds>> {
        let url = format!("{}/markets/{}", GAMMA_API_BASE, market_id);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let market: PolymarketMarket = response.json().await?;
        let mut odds = Vec::new();

        // Extract odds from order books (best bid/ask)
        for order_book in market.order_books {
            if let Some(best_bid) = order_book.bids.first() {
                odds.push(MarketOdds {
                    market_id: market.id.clone(),
                    outcome: order_book.outcome.clone(),
                    odds: best_bid.price,
                    source: MarketSource::Polymarket,
                    timestamp: chrono::Utc::now(),
                });
            }
        }

        Ok(odds)
    }
}

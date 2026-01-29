use anyhow::Result;
use reqwest::Client;
use async_trait::async_trait;
use std::str::FromStr;

use crate::api::MarketClient;
use crate::models::{Market, MarketOdds, MarketSource};

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
}

#[async_trait]
impl MarketClient for PolymarketClient {
    async fn fetch_markets(&self) -> Result<Vec<Market>> {
        let url = format!("{}/markets", GAMMA_API_BASE);
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?;

        let markets_data: serde_json::Value = response.json().await?;
        
        let mut markets = Vec::new();
        
        if let Some(markets_array) = markets_data.as_array() {
            for m in markets_array {
                if let Some(obj) = m.as_object() {
                    let id = obj.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let question = obj.get("question").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    
                    let outcomes = obj.get("outcomes")
                        .and_then(|arr| arr.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str())
                                .map(|s| s.to_string())
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    
                    let volume = obj.get("volume")
                        .and_then(|v| v.as_f64())
                        .map(|f| rust_decimal::Decimal::from(f));
                    
                    markets.push(Market {
                        id: id.clone(),
                        question: question.clone(),
                        description: obj.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        outcomes: outcomes.clone(),
                        end_time: None,
                        volume: volume,
                        liquidity: None,
                    });
                }
            }
        }

        tracing::info!("Fetched {} markets from Polymarket", markets.len());
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

        let market_data: serde_json::Value = response.json().await?;
        let mut odds = Vec::new();

        if let Some(order_book) = market_data.get("orderBook") {
            if let Some(token_orders) = order_book.get("tokenOrders") {
                if let Some(orders_array) = token_orders.as_array() {
                    for order in orders_array {
                        if let Some(obj) = order.as_object() {
                            let outcome = obj.get("outcomeToken").and_then(|v| v.as_str()).unwrap_or("").to_string();
                            let price = obj.get("price")
                                .and_then(|v| v.as_str())
                                .and_then(|s| s.parse::<f64>().ok())
                                .map(|p| rust_decimal::Decimal::from(p / 100.0));
                            
                            if let Some(p) = price {
                                odds.push(MarketOdds {
                                    market_id: market_id.to_string(),
                                    outcome: outcome.clone(),
                                    odds: p,
                                    source: MarketSource::Polymarket,
                                    timestamp: chrono::Utc::now(),
                                });
                            }
                        }
                    }
                }
            }
        }

        tracing::info!("Fetched {} odds from Polymarket", odds.len());
        Ok(odds)
    }

    fn is_configured(&self) -> bool {
        // Polymarket doesn't require wallet config for reading markets
        true
    }
}

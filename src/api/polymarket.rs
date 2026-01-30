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
                        .and_then(|f| rust_decimal::Decimal::try_from(f).ok())
                        .unwrap_or_default();
                    
                    markets.push(Market {
                        id: id.clone(),
                        question: question.clone(),
                        description: obj.get("description").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        outcomes: outcomes.clone(),
                        end_time: None,
                        volume: Some(volume),
                        liquidity: None,
                    });
                }
            }
        }

        tracing::info!("Fetched {} markets from Polymarket", markets.len());
        Ok(markets)
    }

    async fn fetch_odds(&self, _market_id: &str) -> Result<Vec<MarketOdds>> {
        tracing::info!("Odds fetching not implemented yet");
        Ok(vec![])
    }

    fn is_configured(&self) -> bool {
        true
    }
}

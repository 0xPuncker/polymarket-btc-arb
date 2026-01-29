pub mod polymarket;
pub mod btc_market;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Market {
    pub id: String,
    pub question: String,
    pub description: Option<String>,
    pub outcomes: Vec<String>,
    pub end_time: Option<DateTime<Utc>>,
    pub volume: Option<Decimal>,
    pub liquidity: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketOdds {
    pub market_id: String,
    pub outcome: String,
    pub odds: Decimal,
    pub source: MarketSource,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MarketSource {
    Polymarket,
    BitcoinPredictionMarket,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub polymarket_odds: MarketOdds,
    pub btc_market_odds: MarketOdds,
    pub implied_profit: Decimal,
    pub confidence: f64,
    pub detected_at: DateTime<Utc>,
}

impl ArbitrageOpportunity {
    pub fn new(
        polymarket_odds: MarketOdds,
        btc_market_odds: MarketOdds,
        implied_profit: Decimal,
        confidence: f64,
    ) -> Self {
        Self {
            polymarket_odds,
            btc_market_odds,
            implied_profit,
            confidence,
            detected_at: Utc::now(),
        }
    }
}

impl std::fmt::Display for MarketSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarketSource::Polymarket => write!(f, "Polymarket"),
            MarketSource::BitcoinPredictionMarket => write!(f, "BitcoinPredictionMarket"),
            MarketSource::Custom(s) => write!(f, "{}", s),
        }
    }
}

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarketsResponse {
    pub data: Vec<PolymarketMarket>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarket {
    pub market_id: String,
    pub question: String,
    pub description: String,
    pub outcomes: Vec<String>,
    pub end_date: Option<String>,
    pub volume: f64,
    pub order_book: Option<PolymarketOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketOrderBook {
    pub outcome: String,
    pub bids: Vec<PolymarketPriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketPriceLevel {
    pub price: f64,
}

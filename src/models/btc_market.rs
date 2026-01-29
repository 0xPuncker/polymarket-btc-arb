use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Represents a Bitcoin-based prediction market (hypothetical)
/// Could be Lightning Network based, Ordinals markets, or Bitcoin L2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcPredictionMarket {
    pub id: String,
    pub question: String,
    pub outcomes: Vec<String>,
    pub end_time: Option<DateTime<Utc>>,
    pub market_type: BtcMarketType,
    pub order_books: Vec<BtcOrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BtcMarketType {
    LightningNetwork,
    Ordinals,
    Stacks,
    RSK,
    Liquid,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcOrderBook {
    pub outcome: String,
    pub bids: Vec<BtcPriceLevel>,
    pub asks: Vec<BtcPriceLevel>,
    pub currency: String, // "BTC", "sats", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BtcPriceLevel {
    pub price: Decimal, // in BTC or sats
    pub size: Decimal,
}

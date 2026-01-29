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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, std::fmt::Display)]
pub enum BtcMarketType {
    #[serde(rename = "lightning")]
    LightningNetwork,
    #[serde(rename = "ordinals")]
    Ordinals,
    #[serde(rename = "stacks")]
    Stacks,
    #[serde(rename = "rsk")]
    RSK,
    #[serde(rename = "liquid")]
    Liquid,
    #[serde(rename = "custom")]
    Custom(String),
}

impl std::fmt::Display for BtcMarketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BtcMarketType::LightningNetwork => write!(f, "LightningNetwork"),
            BtcMarketType::Ordinals => write!(f, "Ordinals"),
            BtcMarketType::Stacks => write!(f, "Stacks"),
            BtcMarketType::RSK => write!(f, "RSK"),
            BtcMarketType::Liquid => write!(f, "Liquid"),
            BtcMarketType::Custom(s) => write!(f, "{}", s),
        }
    }
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

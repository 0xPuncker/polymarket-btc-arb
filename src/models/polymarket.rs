use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarket {
    pub id: String,
    #[serde(alias = "question")]
    pub question: String,
    pub description: Option<String>,
    #[serde(alias = "outcomes")]
    pub outcomes: Vec<String>,
    #[serde(alias = "end_time", deserialize_with = "deserialize_optional_timestamp")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(alias = "volume")]
    pub volume: Option<Decimal>,
    #[serde(alias = "liquidity")]
    pub liquidity: Option<Decimal>,
    #[serde(default)]
    pub order_books: Vec<OrderBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub outcome: String,
    #[serde(default)]
    pub bids: Vec<PriceLevel>,
    #[serde(default)]
    pub asks: Vec<PriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLevel {
    pub price: Decimal,
    pub size: Decimal,
}

fn deserialize_optional_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => {
            // Try parsing as ISO8601
            s.parse::<DateTime<Utc>>().map(Some).map_err(serde::de::Error::custom)
        }
        None => Ok(None),
    }
}

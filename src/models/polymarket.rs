use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarket {
    pub id: String,
    #[serde(alias = "question")]
    pub question: String,
    pub description: Option<String>,
    #[serde(alias = "outcomes", deserialize_with = "deserialize_json_string")]
    pub outcomes: Vec<String>,
    #[serde(alias = "end_time", deserialize_with = "deserialize_optional_timestamp")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "deserialize_option_string_to_decimal")]
    pub volume: Option<Decimal>,
    #[serde(default, deserialize_with = "deserialize_option_string_to_decimal")]
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

fn deserialize_json_string<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // Handle string representation of JSON array
    if s.starts_with('[') {
        serde_json::from_str(&s).map_err(serde::de::Error::custom)
    } else if s.contains(',') {
        // Handle comma-separated string
        Ok(s.split(',').map(|s| s.trim().to_string()).collect())
    } else {
        // Single outcome as string
        Ok(vec![s])
    }
}

fn deserialize_optional_timestamp<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
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

fn deserialize_option_string_to_decimal<'de, D>(
    deserializer: D,
) -> Result<Option<Decimal>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                s.parse::<Decimal>().map(Some).map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
}

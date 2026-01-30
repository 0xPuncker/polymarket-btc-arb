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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketMarketResponse {
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
pub struct PolymarketOrderBook {
    pub outcome: String,
    pub bids: Vec<PolymarketPriceLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketPriceLevel {
    pub price: f64,
}

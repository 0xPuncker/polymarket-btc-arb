use anyhow::Result;
use tokio::time::{interval, Duration};
use tracing::{info, warn, debug};

use crate::api::MarketClient;
use crate::api::polymarket::PolymarketClient;
use crate::api::btc::BtcMarketClient;
use crate::models::{Market, MarketOdds};
use crate::models::btc_market::BtcMarketType;
use crate::arbitrage::ArbitrageDetector;

pub struct MarketMonitor {
    polymarket_client: PolymarketClient,
    btc_client: BtcMarketClient,
    arbitrage_detector: ArbitrageDetector,
}

impl MarketMonitor {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            polymarket_client: PolymarketClient::new(),
            btc_client: BtcMarketClient::new(BtcMarketType::Custom("generic".to_string())),
            arbitrage_detector: ArbitrageDetector::new(),
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting market monitor loop...");

        let mut tick = interval(Duration::from_secs(60));
        tick.tick().await; // Skip first tick

        loop {
            tick.tick().await;

            if let Err(e) = self.tick().await {
                warn!("Monitor tick error: {}", e);
            }
        }
    }

    async fn tick(&self) -> Result<()> {
        debug!("Fetching Polymarket markets...");
        let poly_markets: Vec<Market> = self.polymarket_client.fetch_markets().await?;
        info!("Fetched {} Polymarket markets", poly_markets.len());

        // Fetch odds for top markets by volume
        let top_markets: Vec<_> = poly_markets
            .into_iter()
            .filter(|m| m.volume.is_some())
            .take(10)
            .collect();

        for market in top_markets {
            debug!("Fetching odds for market: {}", market.question);

            if let Ok(odds) = self.polymarket_client.fetch_odds(&market.id).await {
                // Check for arbitrage opportunities
                let btc_odds: Vec<MarketOdds> = self.btc_client.fetch_odds(&market.id).await.unwrap_or_default();

                if !btc_odds.is_empty() {
                    if let Some(opportunity) = self
                        .arbitrage_detector
                        .detect(&odds, &btc_odds)
                    {
                        info!("Arbitrage opportunity found: {:?}", opportunity);
                    }
                }
            }
        }

        Ok(())
    }
}

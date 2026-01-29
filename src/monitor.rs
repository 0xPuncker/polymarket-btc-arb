use anyhow::Result;
use tracing::{info, debug, warn};

use crate::api::{MarketClient, PolymarketClient};
use crate::arbitrage::ArbitrageDetector;
use crate::config::Config;
use crate::models::{Market, MarketOdds};
use crate::api::predyx::PredyxRealClient;

pub struct MarketMonitor {
    config: Config,
    polymarket_client: PolymarketClient,
    arbitrage_detector: ArbitrageDetector,
}

impl MarketMonitor {
    pub async fn new(config_path: &str) -> Result<Self> {
        let config = Config::load_or_default(config_path)?;

        info!(
            "Auto-execute trading: {} (profit threshold: {:.1}%)",
            config.trading.auto_execute,
            config.general.min_profit_threshold
        );

        Ok(Self {
            polymarket_client: PolymarketClient::new(),
            arbitrage_detector: ArbitrageDetector::new(),
            config,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting market monitor loop...");

        let mut tick = tokio::time::interval(tokio::time::Duration::from_secs(60));
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

        let mut opportunities_found = 0;

        for market in top_markets {
            debug!("Fetching odds for market: {}", market.question);

            match self.polymarket_client.fetch_odds(&market.id).await {
                Ok(poly_odds) => {
                    // Check for arbitrage opportunities with BTC markets
                    let btc_odds: Vec<MarketOdds> = Vec::new();

                    if !btc_odds.is_empty() {
                        if let Some(_opportunity) = self
                            .arbitrage_detector
                            .detect(&poly_odds, &btc_odds, self.config.general.min_profit_threshold)
                        {
                            opportunities_found += 1;

                            info!(
                                "Arbitrage opportunity found: {:.2}% profit (threshold: {:.1}%)",
                                _opportunity.implied_profit * rust_decimal::Decimal::from(100),
                                self.config.general.min_profit_threshold * rust_decimal::Decimal::from(100),
                            );
                        }
                    }
                }
                Err(e) => {
                    debug!("Failed to fetch odds for {}: {}", market.id, e);
                }
            }
        }

        if opportunities_found > 0 {
            info!("Found {} arbitrage opportunities this tick", opportunities_found);
        } else {
            debug!("No arbitrage opportunities found this tick");
        }

        Ok(())
    }
}

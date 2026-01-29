use anyhow::Result;
use tokio::time::{interval, Duration};
use tracing::{info, debug, warn};
use rust_decimal::Decimal;

use crate::api::{MarketClient, PolymarketClient, BtcMarketClient};
use crate::api::btc::BtcMarketType;
use crate::models::{Market, MarketOdds};
use crate::arbitrage::ArbitrageDetector;
use crate::trader::ArbitrageExecutor;
use crate::config::{Config, TradeConfig};

pub struct MarketMonitor {
    polymarket_client: PolymarketClient,
    btc_client: BtcMarketClient,
    arbitrage_detector: ArbitrageDetector,
    trade_executor: ArbitrageExecutor,
    config: Config,
    auto_execute: bool,
}

impl MarketMonitor {
    pub async fn new(config_path: &str) -> Result<Self> {
        let config = Config::load_or_default(config_path)?;
        let auto_execute = config.trading.auto_execute;

        info!(
            "Auto-execute trading: {}",
            if auto_execute { "enabled" } else { "disabled (monitoring only)" }
        );

        Ok(Self {
            polymarket_client: PolymarketClient::new(),
            btc_client: BtcMarketClient::new(BtcMarketType::Custom("generic".to_string())),
            arbitrage_detector: ArbitrageDetector::new(),
            trade_executor: ArbitrageExecutor::new(
                config.polymarket.clone(),
                config.bitcoin.clone(),
            ),
            config,
            auto_execute,
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

        let mut opportunities_found = 0;

        for market in top_markets {
            debug!("Fetching odds for market: {}", market.question);

            match self.polymarket_client.fetch_odds(&market.id).await {
                Ok(poly_odds) => {
                    // Check for arbitrage opportunities
                    let btc_odds: Vec<MarketOdds> = self
                        .btc_client
                        .fetch_odds(&market.id)
                        .await
                        .unwrap_or(Vec::new());

                    if !btc_odds.is_empty() {
                        if let Some(opportunity) = self
                            .arbitrage_detector
                            .detect(&poly_odds, &btc_odds)
                        {
                            opportunities_found += 1;
                            info!(
                                "Arbitrage opportunity found: {:.2}% profit - {} vs {}",
                                opportunity.implied_profit * Decimal::from(100),
                                opportunity.polymarket_odds.outcome,
                                opportunity.btc_market_odds.outcome
                            );

                            // Execute trade if auto_execute is enabled
                            if self.auto_execute {
                                self.execute_opportunity(&opportunity).await?;
                            } else {
                                info!("(Auto-execute disabled - would need confirmation)");
                            }
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

    async fn execute_opportunity(&self, opportunity: &crate::models::ArbitrageOpportunity) -> Result<()> {
        let trade_config = TradeConfig::from_config(&self.config);

        // Check if position size is within limits
        if opportunity.implied_profit < trade_config.min_profit_threshold {
            info!(
                "Skipping trade: profit {:.2}% below threshold {}%",
                opportunity.implied_profit * Decimal::from(100),
                trade_config.min_profit_threshold * Decimal::from(100)
            );
            return Ok(());
        }

        info!("Executing arbitrage trade...");

        let result = self
            .trade_executor
            .execute(opportunity, &trade_config)
            .await?;

        info!("Trade result: {:?}", result.status);

        if result.status == crate::trader::TradeStatus::Success {
            info!("Trade executed successfully!");
        } else if result.status == crate::trader::TradeStatus::Failed {
            warn!("Trade failed: {:?}", result.error);
        }

        Ok(())
    }
}

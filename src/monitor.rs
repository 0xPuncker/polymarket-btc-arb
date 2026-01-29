use anyhow::Result;
use tokio::time::{interval, Duration};
use tracing::{info, debug, warn};
use rust_decimal::Decimal;

use crate::api::{MarketClient, PolymarketClient};
use crate::arbitrage::ArbitrageDetector;
use crate::trader_real::{ArbitrageExecutor, TradeExecutor};
use crate::config::{Config, TradeConfig};
use crate::positions::{PositionManager};

pub struct MarketMonitor {
    polymarket_client: PolymarketClient,
    arbitrage_detector: ArbitrageDetector,
    trade_executor: ArbitrageExecutor,
    config: Config,
    auto_execute: bool,
    min_profit_threshold: Decimal,
}

impl MarketMonitor {
    pub async fn new(config_path: &str) -> Result<Self> {
        let config = Config::load_or_default(config_path)?;
        let auto_execute = config.trading.auto_execute;
        let min_profit_threshold = config.general.min_profit_threshold;

        info!(
            "Auto-execute trading: {} (profit threshold: {:.1}%)",
            if auto_execute { "enabled" } else { "disabled (monitoring only)" },
            min_profit_threshold
        );

        Ok(Self {
            polymarket_client: PolymarketClient::new(),
            arbitrage_detector: ArbitrageDetector::with_threshold(min_profit_threshold),
            trade_executor: ArbitrageExecutor::new(
                config.polymarket.clone(),
                config.bitcoin.clone(),
                config.bitcoin.protocol.clone(),
            ),
            config,
            auto_execute,
            min_profit_threshold,
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
        let poly_markets: Vec<crate::models::Market> = self.polymarket_client.fetch_markets().await?;
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
                    let btc_odds: Vec<crate::models::MarketOdds> = self
                        .trade_executor
                        .fetch_btc_odds(&market.id)
                        .await
                        .unwrap_or(Vec::new());

                    if !btc_odds.is_empty() {
                        if let Some(opportunity) = self
                            .arbitrage_detector
                            .detect(&poly_odds, &btc_odds, self.min_profit_threshold)
                        {
                            opportunities_found += 1;

                            let is_profitable = opportunity.implied_profit >= self.min_profit_threshold;

                            info!(
                                "Arbitrage opportunity found: {:.2}% profit (threshold: {:.1}% - {})",
                                opportunity.implied_profit * Decimal::from(100),
                                self.min_profit_threshold * Decimal::from(100),
                                if is_profitable { "EXECUTING" } else { "MONITORING" }
                            );

                            // Execute trade if auto_execute is enabled and profit meets threshold
                            if self.auto_execute && is_profitable {
                                self.execute_opportunity(&opportunity).await?;
                            } else {
                                info!("(Auto-execute disabled - trade requires manual trigger)");
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

        // Validate against risk management
        if let Err(e) = self.validate_trade(&opportunity, &trade_config).await {
            warn!("Trade validation failed: {}", e);
            return Err(e);
        }

        info!("Executing arbitrage trade...");

        let result = self
            .trade_executor
            .execute(opportunity, &trade_config)
            .await?;

        info!("Trade result: {:?}", result.status);

        if result.status == crate::trader_real::PositionStatus::Closed {
            info!("Trade executed successfully!");
        } else if result.status == crate::trader_real::PositionStatus::Open {
            warn!("Trade failed: {:?}", result.error);
        }

        Ok(())
    }

    async fn validate_trade(
        &self,
        opportunity: &crate::models::ArbitrageOpportunity,
        config: &TradeConfig,
    ) -> Result<()> {
        // Check against risk limits
        let open_positions = self.trade_executor.get_open_positions();

        // Max position count check
        if open_positions.len() >= config.risk.max_open_positions as usize {
            return Err(anyhow::anyhow!(
                "Max open positions ({}) reached - {}",
                config.risk.max_open_positions,
                open_positions.len()
            ));
        }

        // Daily loss check
        let realized_pnl = self.trade_executor.calculate_realized_pnl();
        if realized_pnl < config.risk.max_daily_loss {
            warn!("Daily loss limit approaching: {:.2} / {:.2}",
                realized_pnl.abs(),
                config.risk.max_daily_loss
            );

            if config.risk.stop_on_max_loss {
                return Err(anyhow::anyhow!(
                    "Max daily loss limit reached: {:.2} / {:.2} - trading stopped",
                    realized_pnl.abs(),
                    config.risk.max_daily_loss
                ));
            }
        }

        Ok(())
    }
}

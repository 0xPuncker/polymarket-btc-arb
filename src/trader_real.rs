use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::str::FromStr;

use crate::models::{ArbitrageOpportunity};
use crate::config::{Config, TradeConfig, BitcoinConfig};
use crate::positions::{Position, PositionStatus};

/// Result of a trade execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub polymarket_tx: Option<String>,
    pub btc_tx: Option<String>,
    pub status: TradeStatus,
    pub executed_at: DateTime<Utc>,
    pub error: Option<String>,
    pub position_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeStatus {
    Success,
    Partial,
    Failed,
    Pending,
}

/// Trait for trade execution across different protocols
#[async_trait]
pub trait TradeExecutor: Send + Sync {
    async fn execute_arbitrage(
        &self,
        opportunity: &ArbitrageOpportunity,
        config: &TradeConfig,
    ) -> Result<TradeResult>;

    async fn approve_token(
        &self,
        token: &str,
        amount: rust_decimal::Decimal,
    ) -> Result<String>;

    async fn get_balance(&self) -> Result<rust_decimal::Decimal>;

    fn is_configured(&self) -> bool;
}

/// Polymarket trade executor
pub struct PolymarketTradeExecutor {
    config: crate::config::PolymarketConfig,
}

impl PolymarketTradeExecutor {
    pub fn new(config: crate::config::PolymarketConfig) -> Self {
        Self { config }
    }

    /// Check if wallet is configured
    pub fn is_configured(&self) -> bool {
        self.config.private_key.is_some()
            && !self.config.private_key.as_ref().unwrap().is_empty()
    }
}

#[async_trait]
impl TradeExecutor for PolymarketTradeExecutor {
    async fn execute_arbitrage(
        &self,
        opportunity: &ArbitrageOpportunity,
        _config: &TradeConfig,
    ) -> Result<TradeResult> {
        tracing::info!(
            "Executing Polymarket trade: {} at {}",
            opportunity.polymarket_odds.outcome,
            opportunity.polymarket_odds.odds
        );

        if !self.is_configured() {
            return Ok(TradeResult {
                polymarket_tx: None,
                btc_tx: None,
                status: TradeStatus::Failed,
                executed_at: Utc::now(),
                error: Some("Polymarket wallet not configured".to_string()),
                position_id: None,
            });
        }

        // TODO: Implement actual trading logic
        // Steps:
        // 1. Approve USDC on Polymarket
        // 2. Place order on CLOB exchange
        // 3. Wait for fill
        // 4. Return transaction hash

        Ok(TradeResult {
            polymarket_tx: Some("0x_mock_polymarket_tx".to_string()),
            btc_tx: None,
            status: TradeStatus::Pending,
            executed_at: Utc::now(),
            error: Some("Not implemented yet - requires CLOB integration".to_string()),
            position_id: None,
        })
    }

    async fn approve_token(
        &self,
        token: &str,
        amount: rust_decimal::Decimal,
    ) -> Result<String> {
        tracing::info!("Approving token on Polymarket: {} amount: {}", token, amount);

        if !self.is_configured() {
            return Err(anyhow::anyhow!("Polymarket wallet not configured"));
        }

        // TODO: Implement actual approval transaction
        // Contract call to approve USDC to Polymarket CLOB
        Ok("0x0000000000000000000000000000000000000000".to_string())
    }

    async fn get_balance(&self) -> Result<rust_decimal::Decimal> {
        if !self.is_configured() {
            return Ok(rust_decimal::Decimal::ZERO);
        }

        // TODO: Query actual balance from Polygon
        Ok(rust_decimal::Decimal::ZERO)
    }
}

/// Bitcoin trade executor with real API clients
pub struct BtcTradeExecutorReal {
    config: BitcoinConfig,
    predyx_client: Option<crate::api::predyx::PredyxRealClient>,
    ordinals_client: Option<crate::api::ordinals::OrdinalsMarketplaceClient>,
    btc_market_type: crate::models::btc_market::BtcMarketType,
}

impl BtcTradeExecutorReal {
    pub fn new(config: BitcoinConfig, btc_market_type: crate::models::btc_market::BtcMarketType) -> Self {
        Self {
            config,
            predyx_client: None,
            ordinals_client: None,
            btc_market_type,
        }
    }

    /// Check if wallet is configured
    pub fn is_configured(&self) -> bool {
        match self.btc_market_type {
            crate::models::btc_market::BtcMarketType::LightningNetwork => {
                self.config.lightning.as_ref()
                    .and_then(|l| l.endpoint.as_ref())
                    .map(|e| !e.is_empty())
                    .unwrap_or(false)
            }
            crate::models::btc_market::BtcMarketType::Ordinals => {
                self.config.ordinals.as_ref()
                    .map(|o| !o.ordinals_api_endpoint.is_empty() && !o.ordinals_wallet_address.is_empty())
                    .unwrap_or(false)
            }
            crate::models::btc_market::BtcMarketType::Stacks => {
                self.config.stacks.as_ref()
                    .and_then(|s| s.stacks_api_key.is_some() && !s.stacks_api_key.as_ref().unwrap().is_empty())
                    .unwrap_or(false)
            }
            crate::models::btc_market::BtcMarketType::RSK => {
                self.config.rsk.as_ref()
                    .and_then(|r| r.rsk_rpc_url.is_some() && !r.rsk_rpc_url.as_ref().unwrap().is_empty())
                    .unwrap_or(false)
            }
            crate::models::btc_market::BtcMarketType::Liquid => {
                self.config.liquid.as_ref()
                    .and_then(|l| l.liquid_rpc_url.is_some() && !l.liquid_rpc_url.as_ref().unwrap().is_empty())
                    .unwrap_or(false)
            }
            crate::models::btc_market::BtcMarketType::Custom(_) => false,
        }
    }

    pub fn initialize_clients(&mut self) {
        if let Some(ref predyx_api_key) = self.config.lightning.predyx_api_key {
            if self.btc_market_type == crate::models::btc_market::BtcMarketType::LightningNetwork {
                self.predyx_client = Some(crate::api::predyx::PredyxRealClient::new(Some(predyx_api_key.clone())));
            }
        }

        if let Some(ref api_endpoint) = self.config.ordinals.ordinals_api_endpoint {
            if let Some(ref wallet_address) = self.config.ordinals.ordinals_wallet_address {
                if self.btc_market_type == crate::models::btc_market::BtcMarketType::Ordinals {
                    self.ordinals_client = Some(crate::api::ordinals::OrdinalsMarketplaceClient::new(
                        api_endpoint.clone(),
                        wallet_address.clone(),
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl TradeExecutor for BtcTradeExecutorReal {
    async fn execute_arbitrage(
        &self,
        _opportunity: &ArbitrageOpportunity,
        _config: &TradeConfig,
    ) -> Result<TradeResult> {
        tracing::info!("BTC trade executor - checking configuration");

        if !self.is_configured() {
            return Ok(TradeResult {
                polymarket_tx: None,
                btc_tx: None,
                status: TradeStatus::Failed,
                executed_at: Utc::now(),
                error: Some(format!("BTC {} wallet not configured", self.btc_market_type)),
                position_id: None,
            });
        }

        Ok(TradeResult {
            polymarket_tx: None,
            btc_tx: None,
            status: TradeStatus::Pending,
            executed_at: Utc::now(),
            error: Some("BTC trade execution requires configuration".to_string()),
            position_id: None,
        })
    }

    async fn approve_token(
        &self,
        _token: &str,
        _amount: rust_decimal::Decimal,
    ) -> Result<String> {
        // Bitcoin doesn't typically need token approval
        Ok("N/A".to_string())
    }

    async fn get_balance(&self) -> Result<rust_decimal::Decimal> {
        if !self.is_configured() {
            return Ok(rust_decimal::Decimal::ZERO);
        }

        // TODO: Query actual balance based on protocol
        Ok(rust_decimal::Decimal::ZERO)
    }
}

/// Orchestrates arbitrage execution across both markets with position tracking
pub struct ArbitrageExecutor {
    polymarket_executor: PolymarketTradeExecutor,
    btc_executor: BtcTradeExecutorReal,
    position_manager: crate::positions::PositionManager,
}

impl ArbitrageExecutor {
    pub fn new(
        polymarket_config: crate::config::PolymarketConfig,
        btc_config: BitcoinConfig,
        btc_market_type: crate::models::btc_market::BtcMarketType,
    ) -> Self {
        Self {
            polymarket_executor: PolymarketTradeExecutor::new(polymarket_config),
            btc_executor: BtcTradeExecutorReal::new(btc_config, btc_market_type),
            position_manager: crate::positions::PositionManager::new(),
        }
    }

    /// Execute both sides of arbitrage trade
    pub async fn execute(
        &mut self,
        opportunity: &ArbitrageOpportunity,
        config: &TradeConfig,
    ) -> Result<TradeResult> {
        tracing::info!(
            "Executing arbitrage: {:.2}% profit",
            opportunity.implied_profit * rust_decimal::Decimal::from(100)
        );

        // Initialize API clients
        self.btc_executor.initialize_clients();

        // Execute trades
        let poly_result = self
            .polymarket_executor
            .execute_arbitrage(opportunity, config)
            .await?;

        let btc_result = self
            .btc_executor
            .execute_arbitrage(opportunity, config)
            .await?;

        // Determine overall status
        let status = if poly_result.status == TradeStatus::Success
            && btc_result.status == TradeStatus::Success
        {
            TradeStatus::Success
        } else if poly_result.status == TradeStatus::Pending
            || btc_result.status == TradeStatus::Pending
        {
            TradeStatus::Pending
        } else if poly_result.status == TradeStatus::Success
            || btc_result.status == TradeStatus::Success
        {
            TradeStatus::Partial
        } else {
            TradeStatus::Failed
        };

        // Create position if both sides attempted
        let position_id = if poly_result.polymarket_tx.is_some() || btc_result.btc_tx.is_some() {
            let entry_price = (opportunity.polymarket_odds.odds + opportunity.btc_market_odds.odds) / rust_decimal::Decimal::from(2);
            self.position_manager
                .open_position(
                    opportunity,
                    entry_price,
                    config.max_position_size,
                    poly_result.polymarket_tx.clone(),
                    btc_result.btc_tx.clone(),
                )
                .ok()
        } else {
            None
        };

        Ok(TradeResult {
            polymarket_tx: poly_result.polymarket_tx,
            btc_tx: btc_result.btc_tx,
            status,
            executed_at: Utc::now(),
            error: poly_result.error.or(btc_result.error),
            position_id,
        })
    }

    /// Get position manager reference
    pub fn position_manager(&self) -> &crate::positions::PositionManager {
        &self.position_manager
    }

    /// Get mutable position manager reference
    pub fn position_manager_mut(&mut self) -> &mut crate::positions::PositionManager {
        &mut self.position_manager
    }

    /// Get open positions
    pub fn get_open_positions(&self) -> Vec<Position> {
        self.position_manager.get_open_positions()
    }

    /// Calculate total realized PnL
    pub fn calculate_realized_pnl(&self) -> rust_decimal::Decimal {
        self.position_manager.calculate_realized_pnl()
    }

    /// Calculate total unrealized PnL
    pub fn calculate_unrealized_pnl(&self, current_prices: &std::collections::HashMap<String, rust_decimal::Decimal>) -> rust_decimal::Decimal {
        self.position_manager.calculate_unrealized_pnl(current_prices)
    }

    pub fn fetch_btc_odds(&self, market_id: &str) -> Result<Vec<crate::models::MarketOdds>> {
        if let Some(ref predyx_client) = self.btc_executor.predyx_client {
            predyx_client.fetch_odds(market_id).await
        } else if let Some(ref ordinals_client) = self.btc_executor.ordinals_client {
            ordinals_client.fetch_odds(market_id).await
        } else {
            Ok(Vec::new())
        }
    }
}

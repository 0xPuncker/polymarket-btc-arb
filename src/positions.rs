use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::{Market, ArbitrageOpportunity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub id: String,
    pub market_id: String,
    pub market_question: String,
    pub position_type: PositionType,
    pub entry_price: Decimal,
    pub size: Decimal,
    pub side: TradeSide,
    pub status: PositionStatus,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub exit_price: Option<Decimal>,
    pub pnl: Option<Decimal>,
    pub tx_ids: PositionTxIds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionTxIds {
    pub polymarket_tx: Option<String>,
    pub btc_tx: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionType {
    Arbitrage,
    Speculative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TradeSide {
    Long,
    Short,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PositionState {
    Open,
    Closed,
    Failed,
    Partial,
}

// Type alias for backwards compatibility
pub use PositionState as PositionStatus;

/// Position manager tracks all open and closed positions
pub struct PositionManager {
    positions: HashMap<String, Position>,
    open_positions: Vec<String>,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
            open_positions: Vec::new(),
        }
    }

    /// Open a new position
    pub fn open_position(
        &mut self,
        opportunity: &ArbitrageOpportunity,
        entry_price: Decimal,
        size: Decimal,
        poly_tx: Option<String>,
        btc_tx: Option<String>,
    ) -> Result<String> {
        let id = uuid::Uuid::new_v4().to_string();
        let position = Position {
            id: id.clone(),
            market_id: opportunity.polymarket_odds.market_id.clone(),
            market_question: opportunity.polymarket_odds.outcome.clone(),
            position_type: PositionType::Arbitrage,
            entry_price,
            size,
            side: TradeSide::Long,
            status: PositionStatus::Open,
            opened_at: Utc::now(),
            closed_at: None,
            exit_price: None,
            pnl: None,
            tx_ids: PositionTxIds {
                polymarket_tx: poly_tx,
                btc_tx: btc_tx,
            },
        };

        self.positions.insert(id.clone(), position);
        self.open_positions.push(id.clone());

        tracing::info!(
            "Opened position {}: {} at {}",
            id,
            opportunity.polymarket_odds.outcome,
            entry_price
        );

        Ok(id)
    }

    /// Close a position
    pub fn close_position(
        &mut self,
        position_id: &str,
        exit_price: Decimal,
    ) -> Result<Position> {
        let position = self
            .positions
            .get_mut(position_id)
            .ok_or_else(|| anyhow::anyhow!("Position not found: {}", position_id))?;

        if position.status != PositionStatus::Open {
            return Err(anyhow::anyhow!("Position is not open: {}", position_id));
        }

        position.status = PositionStatus::Closed;
        position.closed_at = Some(Utc::now());
        position.exit_price = Some(exit_price);
        position.pnl = Some(exit_price - position.entry_price);

        self.open_positions.retain(|id| id != position_id);

        tracing::info!(
            "Closed position {}: PnL = {}",
            position_id,
            position.pnl.unwrap_or(Decimal::ZERO)
        );

        Ok(position.clone())
    }

    /// Get all open positions
    pub fn get_open_positions(&self) -> Vec<Position> {
        self.open_positions
            .iter()
            .filter_map(|id| self.positions.get(id).cloned())
            .collect()
    }

    /// Get position by ID
    pub fn get_position(&self, position_id: &str) -> Option<Position> {
        self.positions.get(position_id).cloned()
    }

    /// Get all positions (including closed)
    pub fn get_all_positions(&self) -> Vec<Position> {
        self.positions.values().cloned().collect()
    }

    /// Calculate total unrealized PnL for open positions
    pub fn calculate_unrealized_pnl(&self, current_prices: &HashMap<String, Decimal>) -> Decimal {
        self.open_positions
            .iter()
            .filter_map(|id| self.positions.get(id))
            .map(|pos| {
                current_prices
                    .get(&pos.market_id)
                    .map(|price| price - pos.entry_price)
                    .unwrap_or(Decimal::ZERO)
            })
            .sum()
    }

    /// Calculate total realized PnL for closed positions
    pub fn calculate_realized_pnl(&self) -> Decimal {
        self.positions
            .values()
            .filter(|pos| pos.status == PositionStatus::Closed)
            .map(|pos| pos.pnl.unwrap_or(Decimal::ZERO))
            .sum()
    }

    /// Get position count for a market
    pub fn get_market_position_count(&self, market_id: &str) -> usize {
        self.positions
            .values()
            .filter(|pos| pos.market_id == market_id && pos.status == PositionStatus::Open)
            .count()
    }
}

impl Default for PositionManager {
    fn default() -> Self {
        Self::new()
    }
}

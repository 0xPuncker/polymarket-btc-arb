use anyhow::Result;
use std::path::Path;
use std::fs;
use std::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub polymarket: PolymarketConfig,
    pub bitcoin: BitcoinConfig,
    pub trading: TradingConfig,
    pub risk: RiskConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    #[serde(default = "default_min_profit")]
    pub min_profit_threshold: Decimal,
    #[serde(default = "default_max_position")]
    pub max_position_size: Decimal,
    #[serde(default = "default_max_slippage")]
    pub max_slippage: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymarketConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub private_key: Option<String>,
    #[serde(default = "default_network")]
    pub network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitcoinConfig {
    pub protocol: String,
    #[serde(default)]
    pub lightning: Option<LightningConfig>,
    #[serde(default)]
    pub ordinals: Option<OrdinalsConfig>,
    #[serde(default)]
    pub stacks: Option<StacksConfig>,
    #[serde(default)]
    pub rsk: Option<RskConfig>,
    #[serde(default)]
    pub liquid: Option<LiquidConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightningConfig {
    pub endpoint: String,
    #[serde(default)]
    pub macaroon_path: Option<String>,
    #[serde(default)]
    pub cert_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrdinalsConfig {
    pub address: String,
    pub ordinal_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StacksConfig {
    pub network: String,
    #[serde(default)]
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RskConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidConfig {
    pub rpc_url: String,
    #[serde(default)]
    pub private_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingConfig {
    #[serde(default = "default_auto_execute")]
    pub auto_execute: bool,
    #[serde(default = "default_require_confirmation")]
    pub require_confirmation: bool,
    #[serde(default = "default_max_concurrent")]
    pub max_concurrent_trades: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    #[serde(default = "default_max_daily_loss")]
    pub max_daily_loss: Decimal,
    #[serde(default = "default_stop_on_loss")]
    pub stop_on_max_loss: bool,
    #[serde(default = "default_max_positions")]
    pub max_open_positions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_file")]
    pub file: String,
}

// Trade execution configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeConfig {
    pub polymarket_wallet: PolymarketWalletConfig,
    pub btc_wallet: BtcWalletConfig,
    pub max_position_size: Decimal,
    pub min_profit_threshold: Decimal,
    pub max_slippage: Decimal,
    pub require_confirmation: bool,
}

impl TradeConfig {
    pub fn from_config(config: &Config) -> Self {
        Self {
            polymarket_wallet: config.polymarket.clone(),
            btc_wallet: config.bitcoin.clone(),
            max_position_size: config.general.max_position_size,
            min_profit_threshold: config.general.min_profit_threshold,
            max_slippage: config.general.max_slippage,
            require_confirmation: config.trading.require_confirmation,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig::default(),
            polymarket: PolymarketConfig::default(),
            bitcoin: BitcoinConfig::default(),
            trading: TradingConfig::default(),
            risk: RiskConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            min_profit_threshold: default_min_profit(),
            max_position_size: default_max_position(),
            max_slippage: default_max_slippage(),
        }
    }
}

impl Default for PolymarketConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://polygon-rpc.com".to_string(),
            private_key: None,
            network: default_network(),
        }
    }
}

impl Default for BitcoinConfig {
    fn default() -> Self {
        Self {
            protocol: "lightning".to_string(),
            lightning: None,
            ordinals: None,
            stacks: None,
            rsk: None,
            liquid: None,
        }
    }
}

impl Default for TradingConfig {
    fn default() -> Self {
        Self {
            auto_execute: default_auto_execute(),
            require_confirmation: default_require_confirmation(),
            max_concurrent_trades: default_max_concurrent(),
        }
    }
}

impl Default for RiskConfig {
    fn default() -> Self {
        Self {
            max_daily_loss: default_max_daily_loss(),
            stop_on_max_loss: default_stop_on_loss(),
            max_open_positions: default_max_positions(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: default_log_file(),
        }
    }
}

impl Default for TradeConfig {
    fn default() -> Self {
        Self {
            polymarket_wallet: PolymarketWalletConfig::default(),
            btc_wallet: BtcWalletConfig::default(),
            max_position_size: default_max_position(),
            min_profit_threshold: default_min_profit(),
            max_slippage: default_max_slippage(),
            require_confirmation: default_require_confirmation(),
        }
    }
}

// Re-export types for trader module
pub use PolymarketConfig as PolymarketWalletConfig;
pub use BitcoinConfig as BtcWalletConfig;

// Default value functions
fn default_min_profit() -> Decimal {
    Decimal::from_str("0.05").unwrap_or(Decimal::ZERO)
}

fn default_max_position() -> Decimal {
    Decimal::from_str("1000.0").unwrap_or(Decimal::from(1000))
}

fn default_max_slippage() -> Decimal {
    Decimal::from_str("0.01").unwrap_or(Decimal::from_str("0.01").unwrap())
}

fn default_network() -> String {
    "polygon".to_string()
}

fn default_auto_execute() -> bool {
    false
}

fn default_require_confirmation() -> bool {
    true
}

fn default_max_concurrent() -> u32 {
    3
}

fn default_max_daily_loss() -> Decimal {
    Decimal::from_str("500.0").unwrap_or(Decimal::from(500))
}

fn default_stop_on_loss() -> bool {
    true
}

fn default_max_positions() -> u32 {
    5
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_file() -> String {
    "/var/log/polymarket-btc-arb.log".to_string()
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn load_or_default<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            Self::load(path)
        } else {
            Ok(Self::default())
        }
    }
}

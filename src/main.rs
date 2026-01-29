mod api;
mod models;
mod monitor;
mod arbitrage;
mod matcher;
mod config;
mod positions;

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let env_filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(&format!("{}={}", log_level));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.3.0 (Live Auto-Execute)");

    // Load config
    let config_path = std::env::var("POLYMARKET_CONFIG")
        .unwrap_or_else(|_| "config.toml".to_string());

    // Create monitor instance
    let monitor = crate::monitor::MarketMonitor::new(&config_path).await?;

    // Start monitoring
    if let Err(e) = monitor.run().await {
        error!("Monitor error: {}", e);
    }

    Ok(())
}

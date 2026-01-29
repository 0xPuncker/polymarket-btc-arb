mod api;
mod models;
mod monitor;
mod arbitrage;
mod matcher;
mod trader_real;
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
    let mut env_filter = tracing_subscriber::EnvFilter::from_default_env();
    
    // Parse log level string into directive
    if let Ok(directive) = tracing_subscriber::filter::Directive::from_str(&log_level) {
        env_filter = env_filter.add_directive(directive);
    }
    
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.2.0 (Live Auto-Execute)");

    // Load config
    let config_path = std::env::var("POLYMARKET_CONFIG")
        .unwrap_or_else(|_| "config.toml".to_string());

    // Create monitor instance
    let monitor = monitor::MarketMonitor::new(&config_path).await?;

    // Start monitoring
    if let Err(e) = monitor.run().await {
        error!("Monitor error: {}", e);
    }

    Ok(())
}

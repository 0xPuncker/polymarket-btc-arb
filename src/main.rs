mod api;
mod models;
mod monitor;
mod arbitrage;
mod matcher;
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
    env_filter = env_filter.add_directive(log_level);
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.2.0 (Simplified)");

    info!("Monitor running in simplified mode - monitoring only");
    info!("Trading features available but require configuration");

    Ok(())
}

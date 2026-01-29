mod api;
mod models;
mod monitor;
mod arbitrage;
mod matcher;
mod trader;

use anyhow::Result;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into())
        )
        .init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.1.0");

    // Create monitor instance
    let monitor = monitor::MarketMonitor::new().await?;

    // Start monitoring
    if let Err(e) = monitor.run().await {
        error!("Monitor error: {}", e);
    }

    Ok(())
}

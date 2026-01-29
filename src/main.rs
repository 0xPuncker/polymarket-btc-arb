mod api;
mod models;
mod config;

use tracing::info;
use tokio;

use crate::api::polymarket_api::PolymarketClient;
use crate::models::Market;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.5.0 (Clean Version)");

    let client = PolymarketClient::new();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
            std::process::exit(0);
        },
        _ = run_monitor(&client) => {},
    }
}

async fn run_monitor(client: &PolymarketClient) {
    info!("Starting market monitor loop...");

    loop {
        match client.fetch_markets().await {
            Ok(markets) => {
                info!("Fetched {} markets from Polymarket", markets.len());

                // Log top 3 markets by volume
                for market in markets.iter().take(3) {
                    info!("Market: {} (Question: {})", market.id, market.question);
                }
            }
            Err(e) => {
                tracing::error!("Error fetching markets: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

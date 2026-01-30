mod api;
mod models;
mod config;
mod matcher;

use tracing::info;
use tokio;

use crate::api::MarketClient;
use crate::api::PolymarketClient;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    info!("Starting Polymarket-BTC Arbitrage Monitor v0.10.0 (Minimal Version)");

    let client = PolymarketClient::new();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
            std::process::exit(0);
        },
        _ = run_monitor(client) => {},
    }
}

async fn run_monitor(client: impl MarketClient) {
    info!("Starting market monitor loop...");

    loop {
        match client.fetch_markets().await {
            Ok(markets) => {
                let count = markets.len();
                info!("Fetched {} markets", count);

                for market in markets.iter().take(3) {
                    info!("Market: {} (Question: {})", market.id, market.question);
                }
            }
            Err(e) => {
                info!("Error fetching markets: {}", e);
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

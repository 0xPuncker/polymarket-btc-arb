use anyhow::Result;
use tracing::info;
use tokio::time::{sleep, Duration};

use crate::api::polymarket_api::PolymarketClient;
use crate::models::Market;
use crate::config::Config;

pub struct Monitor {
    client: PolymarketClient,
    config: Config,
}

impl Monitor {
    pub async fn new() -> Result<Self> {
        let config = Config::default();
        
        info!("Starting Polymarket-BTC Arbitrage Monitor v0.4.0");
        info!("Configuration loaded");
        info!("Auto-execute: {}", config.trading.auto_execute);

        Ok(Self {
            client: PolymarketClient::new(),
            config,
        })
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting market monitor loop...");

        loop {
            if let Err(e) = self.tick().await {
                info!("Monitor tick error: {}", e);
            }

            sleep(Duration::from_secs(60)).await;
        }
    }

    async fn tick(&self) -> Result<()> {
        info!("Fetching Polymarket markets...");
        let markets = self.client.fetch_markets().await?;
        
        info!("Fetched {} markets", markets.len());
        
        // Log top 5 markets by volume
        let top_markets: Vec<_> = markets
            .into_iter()
            .filter(|m| m.volume.is_some())
            .take(5)
            .collect();
        
        for market in top_markets {
            info!("Market: {} (Volume: {})", market.question, market.volume.unwrap_or_default());
        }

        Ok(())
    }
}

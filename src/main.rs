mod api;
mod models;
mod config;
mod matcher;
mod monitor;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let monitor = match monitor::Monitor::new().await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to initialize monitor: {}", e);
            std::process::exit(1);
        }
    };

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received Ctrl+C, shutting down...");
        },
        _ = monitor.run() => {},
    }

    tracing::info!("Polymarket-BTC Arbitrage Monitor stopped");
}

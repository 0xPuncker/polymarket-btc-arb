use anyhow::Result;
use tracing::info;

use crate::api::btc_real::predyx::PredyxRealClient;
use crate::config::BitcoinConfig;

pub struct BtcMarketClient {
    predyx_client: Option<PredyxRealClient>,
    config: BitcoinConfig,
}

impl BtcMarketClient {
    pub fn new(config: BitcoinConfig) -> Self {
        Self {
            predyx_client: None,
            config,
        }
    }
}

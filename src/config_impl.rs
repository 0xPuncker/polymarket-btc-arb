impl Default for PolymarketWalletConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://polygon-rpc.com".to_string(),
            private_key: None,
            network: "polygon".to_string(),
        }
    }
}

impl Default for BtcWalletConfig {
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

impl Default for PolymarketWalletConfig {
    fn default() -> Self {
        Self {
            rpc_url: "https://polygon-rpc.com".to_string(),
            private_key: None,
            network: "polygon".to_string(),
        }
    }
}

impl Default for BtcWalletConfig {
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

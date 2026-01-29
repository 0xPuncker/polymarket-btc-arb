# Wallet Configuration Guide

## Current Status: Monitoring Only

**No wallet configuration needed for current functionality.**

The project currently **only monitors and detects arbitrage opportunities** - it does **not execute trades**.

## What's Implemented Now:

- ✅ Polymarket market data fetching (read-only)
- ✅ Odds comparison across markets
- ✅ Arbitrage opportunity detection
- ❌ **No trade execution**
- ❌ **No wallet integration**

## To Execute Arbitrage Trades (Future Enhancement)

To actually perform arbitrage, you would need:

### 1. Polymarket Side
- **Wallet:** MetaMask, Rabby, or similar EVM wallet
- **Network:** Polygon (Matic) network
- **Currency:** USDC (stablecoin)
- **Configuration:** Private key or wallet RPC endpoint

```rust
// Example future config
POLYMARKET_PRIVATE_KEY="0x..."
POLYMARKET_RPC_URL="https://polygon-rpc.com"
```

### 2. Bitcoin Market Side

Depends on which protocol you're using:

**Lightning Network:**
- Wallet: LND, c-lightning, or Phoenix
- Configuration: `LND_CERT`, `LND_MACAROON`, or similar
- Node: Run your own LN node or use a service

**Ordinals:**
- Wallet: Ordinals-compatible wallet (Xverse, Hiro, etc.)
- Configuration: Ordinal inscription IDs, payment addresses

**Stacks:**
- Wallet: Hiro Wallet, Xverse
- Network: Stacks mainnet/testnet
- Configuration: STX private key or wallet RPC

**RSK (Rootstock):**
- Wallet: RSK-compatible wallet (MetaMask with RSK network)
- Configuration: RSK private key, RBTC address

**Liquid Network:**
- Wallet: Liquid-compatible wallet (Elements, etc.)
- Configuration: L-BTC addresses, sidechain keys

### 3. Example Trading Config Structure

```rust
// Config for future implementation
pub struct TradeConfig {
    // Polymarket
    polymarket_private_key: String,
    polymarket_rpc_url: String,

    // Bitcoin protocol
    btc_protocol: BtcMarketType,
    btc_wallet_config: BtcWalletConfig,

    // Risk management
    max_position_size: Decimal,
    min_profit_threshold: Decimal,
    max_slippage: Decimal,
}

pub enum BtcWalletConfig {
    Lightning {
        macaroon_path: String,
        cert_path: String,
        endpoint: String,
    },
    Ordinals {
        address: String,
        ordinal_id: String,
    },
    Stacks {
        private_key: String,
        network: StacksNetwork,
    },
    // ... etc.
}
```

## Security Best Practices (When Implementing Wallets)

1. **Never commit private keys to Git**
   - Use `.gitignore` for `.env`, `config.toml`, key files
   - Use environment variables
   - Consider encrypted storage

2. **Use Hardware Wallets for Production**
   - Cold wallets (Ledger, Trezor)
   - HSM integration
   - Multi-signature setups

3. **Separate Testing vs Production**
   - Testnet for development
   - Mainnet with small amounts initially
   - Different keys for different environments

4. **Gas Fee Management**
   - Estimate fees before execution
   - Set reasonable gas limits
   - Monitor network congestion

## Current Project Status

```
Phase 1: ✅ DONE
- Market data fetching (Polymarket Gamma API)
- Odds monitoring (60s interval)
- Arbitrage detection logic

Phase 2: ⏳ NOT IMPLEMENTED
- Wallet integration
- Trade execution
- Position management
- Risk controls

Phase 3: ⏳ FUTURE
- Automated trading bots
- Multi-strategy arbitrage
- Backtesting
- Performance analytics
```

## Next Steps (If You Want Trading)

1. Decide which Bitcoin protocol to integrate first
2. Set up testnet environments for both sides
3. Implement wallet signers for each protocol
4. Add trade execution module
5. Implement position tracking and risk management
6. Test with small amounts

---

**Bottom line:** No wallet needed now. The project is a passive monitor. Trading execution is a separate feature that would require significant additional development.

# Polymarket-BTC Arbitrage Monitor

A Rust-based monitoring system for detecting arbitrage opportunities between Polymarket (USDC-based) and Bitcoin-based prediction markets.

## Project Structure

```
src/
├── main.rs          # Entry point
├── api/
│   ├── mod.rs       # MarketClient trait definition
│   ├── polymarket.rs # Polymarket API client
│   └── btc.rs       # Bitcoin market client (placeholder)
├── models/
│   ├── mod.rs       # Core data models
│   ├── polymarket.rs # Polymarket-specific models
│   └── btc_market.rs # BTC market models
├── monitor.rs       # Main monitoring loop
└── arbitrage.rs     # Arbitrage detection logic
```

## Features

- **Polymarket Integration**: Fetches markets and odds via Polymarket's Gamma API
- **Bitcoin Market Support**: Multi-protocol support for BTC-based prediction markets:
  - Lightning Network markets
  - Ordinals/BRC-20 markets
  - Stacks blockchain
  - RSK (Rootstock)
  - Liquid Network
- **Fuzzy Outcome Matching**: Jaccard similarity algorithm for matching outcomes across platforms
- **Arbitrage Detection**: Compares odds across markets to find profit opportunities
- **Configurable Threshold**: Minimum 5% profit threshold (adjustable)
- **Continuous Monitoring**: Polls every 60 seconds

## Running

```bash
# Build
cargo build --release

# Run
cargo run --release
```

## Configuration

Set environment variables for customization:

```bash
# Minimum profit threshold (default: 0.05 = 5%)
MIN_PROFIT_THRESHOLD=0.10

# Polling interval in seconds (default: 60)
POLL_INTERVAL=30

# Log level
RUST_LOG=info
```

## API Clients

### Polymarket Client
- Fetches top markets by volume
- Extracts odds from order books
- Supports real-time probability tracking

### Bitcoin Market Client
Multi-protocol client with support for:
- **Lightning Network**: `BtcMarketClient::lightning(endpoint)`
- **Ordinals**: `BtcMarketClient::ordinals(endpoint)`
- **Stacks**: `BtcMarketClient::stacks(endpoint, auth_token)`
- **RSK**: `BtcMarketClient::rsk(endpoint)`
- **Liquid**: `BtcMarketClient::liquid(endpoint)`

Usage example:
```rust
let btc_client = BtcMarketClient::lightning("https://ln-prediction.example.com".to_string());
let markets = btc_client.fetch_markets().await?;
```

### Fuzzy Outcome Matcher
Uses Jaccard similarity to match outcomes across platforms:
- Normalizes text (lowercase, removes punctuation)
- Word-level tokenization
- Configurable similarity threshold (default: 0.8)

Example:
```rust
let matcher = OutcomeMatcher::new();
assert!(matcher.outcomes_match("YES - Trump wins", "Trump Wins - Yes"));
```

## Arbitrage Logic

The detector compares odds for matching outcomes across markets:

1. Fetch odds from Polymarket
2. Fetch odds from Bitcoin market
3. Match outcomes by name (case-insensitive)
4. Calculate implied profit: `(BTC odds - Poly odds) / Poly odds`
5. Report opportunities above threshold

## Future Enhancements

- Connect to real Bitcoin prediction markets
- Add automated trade execution
- WebSocket support for real-time updates
- Fuzzy matching for outcome comparison
- Risk management and position sizing
- Historical performance tracking

## License

MIT

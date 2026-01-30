# TODO Implementation Time Estimates

## Summary

**Total Estimated Time:** 30-47 hours
**Range:** 3-6 days (10-hour workdays)

---

## Polymarket Trade Execution (3 TODOs)

### TODO 1: Implement actual trading logic
**Location:** `src/trader.rs:90`  
**Estimated Time:** 4-6 hours

**Requirements:**
- Integrate with Polymarket CLOB API
- Approve USDC tokens
- Place limit/market orders
- Wait for order fills
- Handle order status updates
- Return transaction hash

**Complexity:** High  
**Dependencies:**
- Polymarket CLOB API documentation
- Web3 library (alloy/ethers)
- Polygon RPC endpoint

**Steps:**
1. Research Polymarket CLOB API (1 hour)
2. Set up web3 client (1 hour)
3. Implement USDC approval (1 hour)
4. Implement order placement (1-2 hours)
5. Add order monitoring (1-2 hours)
6. Error handling (1-2 hours)

---

### TODO 2: Implement actual approval transaction
**Location:** `src/trader.rs:118`  
**Estimated Time:** 2-3 hours

**Requirements:**
- Call ERC20 approve() function
- Gas estimation
- Transaction signing
- Broadcast to Polygon
- Wait for confirmation

**Complexity:** Medium  
**Dependencies:** Web3 library, ERC20 ABI

**Steps:**
1. Set up ERC20 ABI (30 min)
2. Implement approve() call (1 hour)
3. Add gas estimation (30 min)
4. Transaction signing & broadcast (1 hour)
5. Confirmation wait (30 min)

---

### TODO 3: Query actual balance from Polygon
**Location:** `src/trader.rs:128`  
**Estimated Time:** 1-2 hours

**Requirements:**
- Query ERC20 balanceOf() for USDC
- Query native ETH balance
- Parse and return Decimal

**Complexity:** Low  
**Dependencies:** Web3 library

**Steps:**
1. Implement USDC balance query (30 min)
2. Implement ETH balance query (30 min)
3. Add error handling (30-60 min)

---

**Polymarket Subtotal:** 7-11 hours

---

## Bitcoin Trade Execution (6 TODOs)

### TODO 4: Query actual balance based on protocol
**Location:** `src/trader.rs:220`  
**Estimated Time:** 3-4 hours

**Requirements:**
- Lightning: Query LND channel balance
- Ordinals: Query wallet balance
- Stacks: Query STX balance
- RSK: Query RBTC balance
- Liquid: Query L-BTC balance

**Complexity:** Medium  
**Dependencies:** Multiple protocol-specific APIs

**Steps:**
1. Lightning balance query (1 hour)
2. Ordinals balance query (30 min)
3. Stacks balance query (1 hour)
4. RSK balance query (1 hour)
5. Liquid balance query (1 hour)

---

### TODO 5: Implement LN payment
**Location:** `src/trader.rs:229`  
**Estimated Time:** 4-6 hours

**Requirements:**
- Integrate with LND REST or gRPC API
- Create Lightning invoices
- Validate invoices
- Send payments
- Monitor payment status
- Handle payment failures

**Complexity:** High  
**Dependencies:** LND node, LND API

**Steps:**
1. Set up LND client (1 hour)
2. Invoice creation/validation (1 hour)
3. Payment sending (1-2 hours)
4. Status monitoring (1-2 hours)
5. Error handling & retry logic (1-2 hours)

---

### TODO 6: Implement Ordinals trade
**Location:** `src/trader.rs:248`  
**Estimated Time:** 5-7 hours

**Requirements:**
- Integrate with Ordinals marketplace API
- List/Buy inscriptions
- Create inscriptions (for selling)
- Wallet signing
- Transaction broadcast

**Complexity:** High  
**Dependencies:** Ordinals marketplace API

**Steps:**
1. Research marketplace API (1-2 hours)
2. Inscription listing (2 hours)
3. Inscription buying (2 hours)
4. Transaction handling (1-2 hours)

---

### TODO 7: Implement Stacks smart contract call
**Location:** `src/trader.rs:267`  
**Estimated Time:** 4-5 hours

**Requirements:**
- Integrate with Stacks blockchain API
- Smart contract interaction
- STX private key signing
- Transaction broadcast
- Mempool monitoring

**Complexity:** High  
**Dependencies:** Stacks blockchain API

**Steps:**
1. Set up Stacks client (1 hour)
2. Contract interaction (1-2 hours)
3. Transaction signing (1 hour)
4. Broadcast & monitoring (1-2 hours)

---

### TODO 8: Implement RSK smart contract call
**Location:** `src/trader.rs:286`  
**Estimated Time:** 3-4 hours

**Requirements:**
- EVM-like integration on RSK network
- Smart contract calls
- Transaction signing
- Gas estimation

**Complexity:** Medium  
**Dependencies:** Web3 library, RSK RPC

**Steps:**
1. Set up RSK RPC client (30 min)
2. Contract call implementation (1-2 hours)
3. Transaction handling (1-2 hours)

---

### TODO 9: Implement Liquid transaction
**Location:** `src/trader.rs:302`  
**Estimated Time:** 4-5 hours

**Requirements:**
- Confidential transactions on Liquid
- Elements API integration
- Liquid-specific address format
- Transaction signing

**Complexity:** High  
**Dependencies:** Elements API

**Steps:**
1. Research Elements API (1 hour)
2. Transaction construction (2 hours)
3. Signing & broadcast (1-2 hours)

---

**Bitcoin Subtotal:** 23-36 hours

---

## Additional Work Required

### Bug Fixes Needed (Not in TODOs)
**Estimated Time:** 2-3 hours

**Issues:**
1. TradeConfig → TradingConfig (typo in trader.rs)
2. Import TradeConfig from config
3. Test compilation fixes

---

### Integration Work
**Estimated Time:** 4-6 hours

**Tasks:**
- Activate ArbitrageDetector in monitor loop
- Activate PositionManager in monitor loop
- Load config from file
- Wire up all components
- Integration testing

---

### Testing
**Estimated Time:** 6-10 hours

**Tasks:**
- Unit tests for each TODO
- Integration tests
- End-to-end tests
- Testnet testing
- Error scenario testing

---

## Total Time Breakdown

| Category | Time | Notes |
|----------|------|-------|
| Polymarket TODOs | 7-11 hours | 3 tasks |
| Bitcoin TODOs | 23-36 hours | 6 tasks |
| Bug Fixes | 2-3 hours | Not in TODOs |
| Integration | 4-6 hours | Wiring components |
| Testing | 6-10 hours | Unit + integration |
| **Total** | **42-66 hours** | **4-7 days** |

---

## Optimistic Scenario (Experienced Developer)

**Assumptions:**
- Familiar with Rust, web3, and Bitcoin protocols
- APIs well-documented
- No major roadblocks

**Time:** 30-40 hours (3-4 days)

---

## Realistic Scenario

**Assumptions:**
- Good Rust experience
- Some web3/Bitcoin experience
- APIs require investigation
- Some debugging required

**Time:** 42-56 hours (4-6 days)

---

## Conservative Scenario

**Assumptions:**
- Learning new protocols
- API documentation unclear
- Unexpected edge cases
- Thorough testing

**Time:** 50-70 hours (5-7 days)

---

## Recommended Approach

### Phase 1: Minimal Viable Product (15-20 hours)
1. Fix TradeConfig bug (2-3 hours)
2. Implement Polymarket trading (7-11 hours)
3. Add Lightning payment (4-6 hours)
4. Basic integration (2-4 hours)
5. Smoke tests (2-4 hours)

### Phase 2: Full Feature Set (25-30 hours)
6. Implement remaining BTC protocols (15-20 hours)
7. Comprehensive testing (6-10 hours)

### Phase 3: Polish (4-6 hours)
8. Error handling (2-3 hours)
9. Logging & monitoring (2-3 hours)
10. Documentation (1-2 hours)

---

## Prerequisites

Before starting:
- [ ] Polymarket CLOB API access
- [ ] Test accounts on all platforms
- [ ] Testnet RPC endpoints
- [ ] API documentation for all protocols
- [ ] Development environment set up
- [ ] LND node (for Lightning testing)

---

**Last Updated:** 2026-01-30 08:47 CET

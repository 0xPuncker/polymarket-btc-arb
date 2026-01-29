use rust_decimal::Decimal;
use tracing::debug;

use crate::models::{MarketOdds, ArbitrageOpportunity};
use crate::matcher::OutcomeMatcher;

pub struct ArbitrageDetector {
    min_profit_threshold: Decimal,
    matcher: OutcomeMatcher,
}

impl ArbitrageDetector {
    pub fn new() -> Self {
        Self {
            min_profit_threshold: Decimal::from_str_exact("0.05").unwrap(), // 5% minimum
            matcher: OutcomeMatcher::new(),
        }
    }

    pub fn detect(
        &self,
        polymarket_odds: &[MarketOdds],
        btc_odds: &[MarketOdds],
    ) -> Option<ArbitrageOpportunity> {
        // Find matching outcomes and compare odds using fuzzy matching
        for poly_odd in polymarket_odds {
            if let Some(best_match) = self.matcher.find_best_match(poly_odd, btc_odds) {
                // Calculate implied profit
                let implied_profit = self.calculate_implied_profit(poly_odd, &best_match);

                if implied_profit >= self.min_profit_threshold {
                    debug!(
                        "Found arbitrage: Poly {} vs BTC {} = {}% profit (match: {} -> {})",
                        poly_odd.odds,
                        best_match.odds,
                        implied_profit * Decimal::from(100),
                        poly_odd.outcome,
                        best_match.outcome
                    );

                    return Some(ArbitrageOpportunity::new(
                        poly_odd.clone(),
                        best_match.clone(),
                        implied_profit,
                        self.calculate_confidence(poly_odd, &best_match),
                    ));
                }
            }
        }

        None
    }

    fn calculate_implied_profit(&self, poly_odd: &MarketOdds, btc_odd: &MarketOdds) -> Decimal {
        // If Polymarket offers lower odds than BTC market, buy YES on Poly and NO on BTC
        // Profit = (BTC odds) - (Poly odds)
        let diff = (btc_odd.odds - poly_odd.odds).abs();
        diff / poly_odd.odds
    }

    fn calculate_confidence(&self, poly_odd: &MarketOdds, btc_odd: &MarketOdds) -> f64 {
        // Confidence based on:
        // - Time difference between odds
        // - Volume/liquidity (if available)

        let time_diff = (poly_odd.timestamp - btc_odd.timestamp).num_seconds().abs() as f64;
        let time_factor = 1.0 / (1.0 + time_diff / 3600.0); // Decay over 1 hour

        // Higher confidence if both markets have recent data
        time_factor.min(1.0)
    }
}

impl Default for ArbitrageDetector {
    fn default() -> Self {
        Self::new()
    }
}

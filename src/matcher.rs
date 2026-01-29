use std::collections::HashSet;
use crate::models::MarketOdds;
use chrono::Utc;

pub struct OutcomeMatcher {
    similarity_threshold: f64,
}

impl OutcomeMatcher {
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.8,
        }
    }

    /// Check if two outcomes match using fuzzy matching
    pub fn outcomes_match(&self, a: &str, b: &str) -> bool {
        // Normalize both strings
        let a_norm = self.normalize(a);
        let b_norm = self.normalize(b);

        // Direct match after normalization
        if a_norm == b_norm {
            return true;
        }

        // Calculate similarity
        let similarity = self.calculate_similarity(&a_norm, &b_norm);
        similarity >= self.similarity_threshold
    }

    fn normalize(&self, s: &str) -> String {
        s.to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    fn calculate_similarity(&self, a: &str, b: &str) -> f64 {
        // Jaccard similarity on word sets
        let set_a: HashSet<&str> = a.split_whitespace().collect();
        let set_b: HashSet<&str> = b.split_whitespace().collect();

        if set_a.is_empty() && set_b.is_empty() {
            return 1.0;
        }

        let intersection: HashSet<_> = set_a.intersection(&set_b).cloned().collect();
        let union: HashSet<_> = set_a.union(&set_b).cloned().collect();

        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f64 / union.len() as f64
        }
    }

    /// Find the best matching outcome for a given odds entry
    pub fn find_best_match(&self, target: &MarketOdds, candidates: &[MarketOdds]) -> Option<MarketOdds> {
        let mut best_match = None;
        let mut best_score = 0.0;

        for candidate in candidates {
            if self.outcomes_match(&target.outcome, &candidate.outcome) {
                // Use similarity as tiebreaker
                let score = self.calculate_similarity(&target.outcome, &candidate.outcome);
                if score > best_score {
                    best_score = score;
                    best_match = Some(candidate.clone());
                }
            }
        }

        best_match
    }
}

impl Default for OutcomeMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    fn create_test_odds(outcome: &str, odds: Decimal) -> MarketOdds {
        MarketOdds {
            market_id: "test-market".to_string(),
            outcome: outcome.to_string(),
            odds,
            source: crate::models::MarketSource::Polymarket,
            timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_exact_match() {
        let matcher = OutcomeMatcher::new();
        assert!(matcher.outcomes_match("Yes", "yes"));
        assert!(matcher.outcomes_match("Trump wins", "trump wins"));
    }

    #[test]
    fn test_normalization() {
        let matcher = OutcomeMatcher::new();
        assert!(matcher.outcomes_match("YES - Trump", "yes trump"));
        assert!(matcher.outcomes_match("Biden: 2024", "biden 2024"));
    }

    #[test]
    fn test_jaccard_similarity() {
        let matcher = OutcomeMatcher::new();
        // Perfect matches (Jaccard = 1.0)
        assert!(matcher.outcomes_match("Trump wins", "Trump Wins"));
        assert!(matcher.outcomes_match("YES - Trump", "Trump - Yes"));
        assert!(matcher.outcomes_match("YES Trump wins", "Trump wins YES"));
        // High similarity (Jaccard > 0.8)
        assert!(matcher.outcomes_match("YES Trump", "Trump YES")); // 2/2 = 1.0
        // Low similarity - should not match
        assert!(!matcher.outcomes_match("Trump wins", "Biden wins")); // 0.0
        assert!(!matcher.outcomes_match("YES - Trump wins election", "Trump Wins - Yes")); // 3/4 = 0.75 < 0.8
    }

    #[test]
    fn test_find_best_match() {
        let matcher = OutcomeMatcher::new();
        let target = create_test_odds("YES - Trump wins", Decimal::from_str("0.6").unwrap());

        let candidates = vec![
            create_test_odds("Trump Wins - Yes", Decimal::from_str("0.55").unwrap()),
            create_test_odds("Biden wins - Yes", Decimal::from_str("0.4").unwrap()),
            create_test_odds("Donald Trump Victory", Decimal::from_str("0.62").unwrap()),
        ];

        let best = matcher.find_best_match(&target, &candidates);
        assert!(best.is_some());
        assert_eq!(best.unwrap().outcome, "Trump Wins - Yes");
    }

    #[test]
    fn test_empty_strings() {
        let matcher = OutcomeMatcher::new();
        assert!(matcher.outcomes_match("", ""));
        assert!(!matcher.outcomes_match("", "Trump wins"));
    }

    #[test]
    fn test_special_characters() {
        let matcher = OutcomeMatcher::new();
        assert!(matcher.outcomes_match("@Trump#2024$", "trump 2024"));
        assert!(matcher.outcomes_match("YES-Trump,Wins", "yes trump wins"));
    }
}

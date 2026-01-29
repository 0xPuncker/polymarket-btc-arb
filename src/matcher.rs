use std::collections::HashSet;
use crate::models::MarketOdds;

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

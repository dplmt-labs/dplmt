//! Weighted scoring across booth quotes. Lower score is better.

use crate::booth::{BoothQuote, SecurityRating};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortMode {
    Cheapest,
    Fastest,
    Safest,
    Balanced,
}

#[derive(Debug, Clone, Copy)]
pub struct Weights {
    pub amount: f64,
    pub duration: f64,
    pub security: f64,
    pub slippage: f64,
}

impl SortMode {
    pub fn weights(self) -> Weights {
        match self {
            SortMode::Cheapest => Weights {
                amount: 0.7,
                duration: 0.15,
                security: 0.1,
                slippage: 0.05,
            },
            SortMode::Fastest => Weights {
                amount: 0.1,
                duration: 0.7,
                security: 0.15,
                slippage: 0.05,
            },
            SortMode::Safest => Weights {
                amount: 0.1,
                duration: 0.1,
                security: 0.7,
                slippage: 0.1,
            },
            SortMode::Balanced => Weights {
                amount: 0.4,
                duration: 0.3,
                security: 0.2,
                slippage: 0.1,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredQuote {
    pub quote: BoothQuote,
    pub score: f64,
    pub rank: i32,
}

pub fn score_quotes(quotes: &[BoothQuote], mode: SortMode) -> Vec<ScoredQuote> {
    if quotes.is_empty() {
        return Vec::new();
    }
    let w = mode.weights();
    let max_out = quotes.iter().map(|q| q.amount_out).fold(0.0_f64, f64::max);
    let min_dur = quotes.iter().map(|q| q.duration_sec).min().unwrap_or(0) as f64;
    let max_dur = quotes.iter().map(|q| q.duration_sec).max().unwrap_or(0) as f64;
    let max_slip = quotes
        .iter()
        .map(|q| q.slippage_pct)
        .fold(0.0_f64, f64::max)
        .max(1e-4);
    let dur_spread = (max_dur - min_dur).max(1.0);

    let mut scored: Vec<ScoredQuote> = quotes
        .iter()
        .map(|q| {
            let amount_norm = if max_out > 0.0 {
                1.0 - q.amount_out / max_out
            } else {
                1.0
            };
            let dur_norm = (q.duration_sec as f64 - min_dur) / dur_spread;
            let sec_norm = 1.0 - q.security_rating.weight();
            let slip_norm = q.slippage_pct / max_slip;
            let score = w.amount * amount_norm
                + w.duration * dur_norm
                + w.security * sec_norm
                + w.slippage * slip_norm;
            ScoredQuote {
                quote: q.clone(),
                score,
                rank: -1,
            }
        })
        .collect();

    let mut sorted_idx: Vec<usize> = (0..scored.len()).collect();
    sorted_idx.sort_by(|a, b| {
        scored[*a]
            .score
            .partial_cmp(&scored[*b].score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for (rank, idx) in sorted_idx.iter().enumerate() {
        scored[*idx].rank = rank as i32 + 1;
    }

    scored
}

/// Convenience: best by each axis.
pub fn best_by(quotes: &[BoothQuote]) -> Option<BestBy> {
    if quotes.is_empty() {
        return None;
    }
    let cheapest = quotes
        .iter()
        .max_by(|a, b| {
            a.amount_out
                .partial_cmp(&b.amount_out)
                .unwrap_or(std::cmp::Ordering::Equal)
        })?
        .booth;
    let fastest = quotes.iter().min_by_key(|q| q.duration_sec)?.booth;
    let safest = quotes
        .iter()
        .max_by(|a, b| {
            a.security_rating
                .weight()
                .partial_cmp(&b.security_rating.weight())
                .unwrap_or(std::cmp::Ordering::Equal)
        })?
        .booth;
    let balanced = {
        let scored = score_quotes(quotes, SortMode::Balanced);
        scored
            .iter()
            .min_by(|a, b| {
                a.score
                    .partial_cmp(&b.score)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?
            .quote
            .booth
    };
    Some(BestBy {
        cheapest,
        fastest,
        safest,
        balanced,
    })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestBy {
    pub cheapest: crate::booth::BoothId,
    pub fastest: crate::booth::BoothId,
    pub safest: crate::booth::BoothId,
    pub balanced: crate::booth::BoothId,
}

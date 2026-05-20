//! Lightweight intent parser. Real production uses Claude API but a deterministic
//! regex layer keeps the engine self-contained for offline tooling.

use crate::chain::ChainId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParsedIntent {
    pub from: Option<ChainId>,
    pub to: Option<ChainId>,
    pub from_token: Option<String>,
    pub to_token: Option<String>,
    pub amount: Option<f64>,
    pub preference: Option<Preference>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Preference {
    Cheapest,
    Fastest,
    Safest,
    Balanced,
}

pub fn parse_intent_regex(text: &str) -> ParsedIntent {
    let lower = text.to_ascii_lowercase();
    let amount = find_amount(&lower);
    let chains = find_chains(&lower);
    let from = chains.first().copied();
    let to = chains.get(1).copied();
    let tokens = find_tokens(&lower);
    let from_token = tokens.first().cloned();
    let to_token = tokens.get(1).cloned().or_else(|| from_token.clone());
    let preference = if lower.contains("fast") {
        Some(Preference::Fastest)
    } else if lower.contains("safe") {
        Some(Preference::Safest)
    } else if lower.contains("cheap") || lower.contains("lowest") {
        Some(Preference::Cheapest)
    } else {
        Some(Preference::Balanced)
    };
    let confidence = match (from, to, &from_token, amount) {
        (Some(_), Some(_), Some(_), Some(_)) => 0.7,
        (Some(_), Some(_), _, Some(_)) => 0.55,
        _ => 0.2,
    };
    ParsedIntent {
        from,
        to,
        from_token,
        to_token,
        amount,
        preference,
        confidence,
    }
}

fn find_amount(s: &str) -> Option<f64> {
    let mut buf = String::new();
    let mut seen_dot = false;
    for ch in s.chars() {
        if ch.is_ascii_digit() {
            buf.push(ch);
        } else if ch == '.' && !seen_dot {
            buf.push(ch);
            seen_dot = true;
        } else if !buf.is_empty() {
            break;
        }
    }
    if buf.is_empty() {
        return None;
    }
    let mut value: f64 = buf.parse().ok()?;
    if s.contains(" k") || s.contains("k usd") || s.ends_with("k") {
        value *= 1000.0;
    } else if s.contains(" m") {
        value *= 1_000_000.0;
    }
    Some(value)
}

fn find_chains(s: &str) -> Vec<ChainId> {
    let aliases: &[(&str, ChainId)] = &[
        ("solana", ChainId::Solana),
        ("sol ", ChainId::Solana),
        ("ethereum", ChainId::Ethereum),
        (" eth", ChainId::Ethereum),
        ("arbitrum", ChainId::Arbitrum),
        ("arb", ChainId::Arbitrum),
        ("base", ChainId::Base),
        ("polygon", ChainId::Polygon),
        ("matic", ChainId::Polygon),
        ("bnb", ChainId::Bnb),
        ("bsc", ChainId::Bnb),
        ("avalanche", ChainId::Avalanche),
        ("avax", ChainId::Avalanche),
        ("sui", ChainId::Sui),
    ];
    let mut hits: Vec<(usize, ChainId)> = Vec::new();
    for (alias, chain) in aliases {
        let mut search_from = 0usize;
        while let Some(idx) = s[search_from..].find(alias) {
            let abs = search_from + idx;
            hits.push((abs, *chain));
            search_from = abs + alias.len();
        }
    }
    hits.sort_by_key(|h| h.0);
    let mut seen = Vec::with_capacity(2);
    for (_, chain) in hits {
        if !seen.contains(&chain) {
            seen.push(chain);
        }
        if seen.len() == 2 {
            break;
        }
    }
    seen
}

fn find_tokens(s: &str) -> Vec<String> {
    let cands = ["usdc", "usdt", "eth", "sol", "wbtc", "btc", "matic", "bnb", "avax", "sui", "dai"];
    let mut out: Vec<String> = Vec::new();
    for c in cands {
        if s.contains(c) && !out.contains(&c.to_uppercase()) {
            out.push(c.to_uppercase());
        }
        if out.len() == 2 {
            break;
        }
    }
    out
}

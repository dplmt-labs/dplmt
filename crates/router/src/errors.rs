//! Strongly-typed errors used across the routing engine.
//!
//! Every booth adapter, the scoring layer and the intent parser surface a
//! [`RouterError`]. Higher layers (HTTP handlers, CLI) match on the variant to
//! decide how to talk back to the caller -- a transient upstream timeout is
//! shown as a yellow seal, while [`SameChainRoute`] is a user-facing 400.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("source and destination chains must differ")]
    SameChainRoute,

    #[error("invalid amount: must be positive")]
    InvalidAmount,

    #[error("invalid token symbol: {0}")]
    InvalidToken(String),

    #[error("booth quote unavailable")]
    QuoteUnavailable,

    #[error("booth timeout after {0}ms")]
    Timeout(u64),

    #[error("rate limited by upstream: retry in {0}s")]
    RateLimited(u32),

    #[error("upstream error: {0}")]
    Upstream(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("anyhow: {0}")]
    Other(#[from] anyhow::Error),
}

impl RouterError {
    /// Returns a short machine-friendly tag suitable for analytics keys.
    pub fn tag(&self) -> &'static str {
        match self {
            RouterError::SameChainRoute => "same-chain",
            RouterError::InvalidAmount => "invalid-amount",
            RouterError::InvalidToken(_) => "invalid-token",
            RouterError::QuoteUnavailable => "quote-unavailable",
            RouterError::Timeout(_) => "timeout",
            RouterError::RateLimited(_) => "rate-limited",
            RouterError::Upstream(_) => "upstream",
            RouterError::Io(_) => "io",
            RouterError::Serde(_) => "serde",
            RouterError::Other(_) => "other",
        }
    }

    /// Whether the error is worth retrying with the same input.
    pub fn is_transient(&self) -> bool {
        matches!(
            self,
            RouterError::Timeout(_) | RouterError::RateLimited(_) | RouterError::Upstream(_)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_is_stable() {
        assert_eq!(RouterError::SameChainRoute.tag(), "same-chain");
        assert_eq!(RouterError::Timeout(9000).tag(), "timeout");
    }

    #[test]
    fn transient_classifier() {
        assert!(RouterError::Timeout(9000).is_transient());
        assert!(!RouterError::InvalidAmount.is_transient());
    }
}

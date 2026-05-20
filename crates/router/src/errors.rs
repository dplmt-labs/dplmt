//! Strongly-typed errors used across the routing engine.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("source and destination chains must differ")]
    SameChainRoute,
    #[error("invalid amount")]
    InvalidAmount,
    #[error("booth quote unavailable")]
    QuoteUnavailable,
    #[error("upstream error: {0}")]
    Upstream(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde error: {0}")]
    Serde(#[from] serde_json::Error),
}

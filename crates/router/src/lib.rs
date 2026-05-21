//! Multi-bridge routing engine for the DPLMT diplomatic round table.
//!
//! The engine fans out a single transfer request to N booths (bridge
//! adapters) and folds the responses into a single ranked answer. Each booth
//! is responsible for its own protocol; the engine itself is transport
//! agnostic.
//!
//! # Layout
//!
//! - [`chain`] -- chain metadata (eight nations seated at the round table)
//! - [`booth`] -- the [`Booth`] trait that every bridge adapter implements
//! - [`bridges`] -- concrete booth adapters for Wormhole, deBridge, Mayan,
//!   Stargate, Allbridge, Hyperlane, Across and Synapse
//! - [`scoring`] -- weighted ranking across amount-out, finality, security and
//!   slippage
//! - [`engine`] -- the [`Engine`] orchestrator
//! - [`intent`] -- regex-based natural-language quote parser (the dictation
//!   fallback for when the Claude API is unavailable)
//! - [`errors`] -- strongly-typed [`RouterError`]
//!
//! # Example
//!
//! ```no_run
//! use dplmt_router::{Engine, booth::QuoteRequest, chain::ChainId, scoring::SortMode};
//! use std::sync::Arc;
//!
//! # async fn _example() -> anyhow::Result<()> {
//! let booths = vec![Arc::new(dplmt_router::bridges::MayanBooth::new(None)) as Arc<dyn dplmt_router::booth::Booth>];
//! let engine = Engine::new(booths);
//! let req = QuoteRequest {
//!     from: ChainId::Solana,
//!     to: ChainId::Arbitrum,
//!     from_token: "USDC".into(),
//!     to_token: "USDC".into(),
//!     amount: 500.0,
//!     sender: None,
//!     recipient: None,
//! };
//! let cable = engine.open_channel(req, SortMode::Balanced).await?;
//! println!("partial failures: {}", cable.partial_failures.len());
//! # Ok(()) }
//! ```

pub mod chain;
pub mod booth;
pub mod bridges;
pub mod scoring;
pub mod engine;
pub mod intent;
pub mod errors;

pub use chain::{Chain, ChainId};
pub use booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
pub use scoring::{best_by, score_quotes, BestBy, ScoredQuote, SortMode};
pub use engine::{CableResponse, Engine};
pub use intent::{parse_intent_regex, ParsedIntent, Preference};
pub use errors::RouterError;

/// Crate version, embedded at build time.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Number of booths the diplomat polls in parallel for a single transfer.
pub const ROUND_TABLE_SEAT_COUNT: usize = 8;

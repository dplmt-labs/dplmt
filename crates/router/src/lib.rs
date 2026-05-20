//! Multi-bridge routing engine for the DPLMT diplomatic round table.
//!
//! The engine fans out a single transfer request to N booths (bridge adapters)
//! and folds the responses into a single ranked answer. Each booth is responsible
//! for its own protocol; the engine itself is transport-agnostic.

pub mod chain;
pub mod booth;
pub mod bridges;
pub mod scoring;
pub mod engine;
pub mod intent;
pub mod errors;

pub use chain::{Chain, ChainId};
pub use booth::{Booth, BoothId, BoothQuote};
pub use scoring::{score_quotes, SortMode};
pub use engine::Engine;
pub use errors::RouterError;

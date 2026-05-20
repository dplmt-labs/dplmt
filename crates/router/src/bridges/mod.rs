//! Concrete booth (bridge) adapters. Each implements `Booth` against a public
//! quote endpoint. The engine remains transport-agnostic.

pub mod debridge;
pub mod mayan;
pub mod stargate;
pub mod across;
pub mod allbridge;
pub mod hyperlane;
pub mod synapse;
pub mod wormhole;

pub use across::AcrossBooth;
pub use allbridge::AllbridgeBooth;
pub use debridge::DebridgeBooth;
pub use hyperlane::HyperlaneBooth;
pub use mayan::MayanBooth;
pub use stargate::StargateBooth;
pub use synapse::SynapseBooth;
pub use wormhole::WormholeBooth;

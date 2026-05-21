//! Concrete booth (bridge) adapters. Each module implements [`crate::booth::Booth`]
//! against a public quote endpoint. The engine remains transport-agnostic --
//! it sees only the trait object, never a specific HTTP client.
//!
//! Adapter inventory:
//!
//! | Booth | Module | Endpoint or SDK source |
//! |-------|--------|------------------------|
//! | Wormhole | [`wormhole`] | canonical NTT / Portal lanes (offline estimate) |
//! | deBridge DLN | [`debridge`] | `dln.debridge.finance/v1.0/dln/order/quote` |
//! | Mayan Swift | [`mayan`] | `price-api.mayan.finance/v3/quote` |
//! | Stargate | [`stargate`] | `stargate.finance/api/v1/quotes` |
//! | Allbridge Core | [`allbridge`] | `core.api.allbridgecoreapi.net/swap/calculator` |
//! | Hyperlane | [`hyperlane`] | warp route oracle pricing (offline estimate) |
//! | Across | [`across`] | `app.across.to/api/suggested-fees` |
//! | Synapse | [`synapse`] | `api.synapseprotocol.com/bridge` |

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

use crate::booth::Booth;
use std::sync::Arc;

/// Returns a default registry of every booth with no referral / no API keys.
/// Useful for getting a routing engine up in two lines.
pub fn default_registry() -> Vec<Arc<dyn Booth>> {
    vec![
        Arc::new(WormholeBooth::new()) as Arc<dyn Booth>,
        Arc::new(DebridgeBooth::new(None)),
        Arc::new(MayanBooth::new(None)),
        Arc::new(StargateBooth::new()),
        Arc::new(AllbridgeBooth::new()),
        Arc::new(HyperlaneBooth::new()),
        Arc::new(AcrossBooth::new()),
        Arc::new(SynapseBooth::new()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_has_eight_seats() {
        assert_eq!(default_registry().len(), 8);
    }
}

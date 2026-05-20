use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct WormholeBooth;

impl WormholeBooth {
    pub fn new() -> Self {
        WormholeBooth
    }
}

impl Default for WormholeBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for WormholeBooth {
    fn id(&self) -> BoothId {
        BoothId::Wormhole
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let supported = matches!(
            request.from,
            ChainId::Solana | ChainId::Ethereum | ChainId::Arbitrum | ChainId::Base | ChainId::Polygon | ChainId::Bnb | ChainId::Avalanche | ChainId::Sui
        ) && matches!(
            request.to,
            ChainId::Solana | ChainId::Ethereum | ChainId::Arbitrum | ChainId::Base | ChainId::Polygon | ChainId::Bnb | ChainId::Avalanche | ChainId::Sui
        );
        if !supported {
            return Err(RouterError::QuoteUnavailable);
        }
        // Canonical bridge: principal preserved, gas-only cost. ETH lanes
        // dominate the duration with VAA publication + finalisation.
        let finality_src = request.from.average_finality_sec().max(5);
        let finality_dst = request.to.average_finality_sec().max(5);
        let duration_sec = (finality_src.max(finality_dst) * 12) + 60;
        let fee_usd = if request.from == ChainId::Ethereum || request.to == ChainId::Ethereum {
            6.0
        } else {
            1.5
        };
        Ok(BoothQuote {
            booth: BoothId::Wormhole,
            amount_out: request.amount,
            duration_sec,
            fee_usd,
            slippage_pct: 0.0,
            security_rating: SecurityRating::A,
            meta: Some(serde_json::json!({"mode": "ntt/portal"})),
        })
    }
}

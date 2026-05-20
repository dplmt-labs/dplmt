use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct HyperlaneBooth;

impl HyperlaneBooth {
    pub fn new() -> Self {
        HyperlaneBooth
    }
}

impl Default for HyperlaneBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for HyperlaneBooth {
    fn id(&self) -> BoothId {
        BoothId::Hyperlane
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let supported = matches!(
            request.from,
            ChainId::Ethereum | ChainId::Arbitrum | ChainId::Base | ChainId::Polygon | ChainId::Bnb
        ) && matches!(
            request.to,
            ChainId::Ethereum | ChainId::Arbitrum | ChainId::Base | ChainId::Polygon | ChainId::Bnb
        );
        if !supported {
            return Err(RouterError::QuoteUnavailable);
        }
        // Warp route fees are oracle-priced. Without the on-chain hook we treat
        // the principal as preserved and surface a flat relayer fee.
        Ok(BoothQuote {
            booth: BoothId::Hyperlane,
            amount_out: request.amount,
            duration_sec: if request.from == ChainId::Ethereum || request.to == ChainId::Ethereum {
                240
            } else {
                60
            },
            fee_usd: 0.8,
            slippage_pct: 0.0,
            security_rating: SecurityRating::B,
            meta: Some(serde_json::json!({"mode": "warp-route"})),
        })
    }
}

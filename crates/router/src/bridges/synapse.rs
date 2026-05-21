use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct SynapseBooth {
    pub client: reqwest::Client,
}

impl SynapseBooth {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("dplmt-router/0.6")
                .build()
                .expect("client"),
        }
    }

    fn chain_id(c: ChainId) -> Option<u64> {
        Some(match c {
            ChainId::Ethereum => 1,
            ChainId::Arbitrum => 42161,
            ChainId::Base => 8453,
            ChainId::Polygon => 137,
            ChainId::Bnb => 56,
            ChainId::Avalanche => 43114,
            _ => return None,
        })
    }
}

impl Default for SynapseBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for SynapseBooth {
    fn id(&self) -> BoothId {
        BoothId::Synapse
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let from = Self::chain_id(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let to = Self::chain_id(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let amount_raw = (request.amount * 1e6) as u128;
        let res = self
            .client
            .get("https://api.synapseprotocol.com/bridge")
            .query(&[
                ("fromChain", from.to_string().as_str()),
                ("toChain", to.to_string().as_str()),
                ("amount", amount_raw.to_string().as_str()),
            ])
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("synapse {}", res.status())));
        }
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        let amount_out_raw = data["maxAmountOut"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let amount_out = amount_out_raw / 1e6;
        let fee_raw = data["bridgeFee"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0);
        let fee_usd = fee_raw / 1e6;

        Ok(BoothQuote {
            booth: BoothId::Synapse,
            amount_out,
            duration_sec: 600,
            fee_usd: if fee_usd > 0.0 { fee_usd } else { 0.5 },
            slippage_pct: 0.08,
            security_rating: SecurityRating::B,
            meta: Some(serde_json::json!({"module": data["bridgeModuleName"]})),
        })
    }
}

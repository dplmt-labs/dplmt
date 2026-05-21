use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct AcrossBooth {
    pub client: reqwest::Client,
}

impl AcrossBooth {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("dplmt-router/0.6")
                .build()
                .expect("client"),
        }
    }

    fn chain(c: ChainId) -> Option<u64> {
        Some(match c {
            ChainId::Ethereum => 1,
            ChainId::Arbitrum => 42161,
            ChainId::Base => 8453,
            ChainId::Polygon => 137,
            _ => return None,
        })
    }
}

impl Default for AcrossBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for AcrossBooth {
    fn id(&self) -> BoothId {
        BoothId::Across
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let from = Self::chain(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let to = Self::chain(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let amount_raw = (request.amount * 1e6) as u128;
        let res = self
            .client
            .get("https://app.across.to/api/suggested-fees")
            .query(&[
                ("token", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
                ("originChainId", from.to_string().as_str()),
                ("destinationChainId", to.to_string().as_str()),
                ("amount", amount_raw.to_string().as_str()),
            ])
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("across {}", res.status())));
        }
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        let relay = data["relayFeePct"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
            / 1e18;
        let lp = data["lpFeePct"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
            / 1e18;
        let total_fee = relay + lp;
        let amount_out = request.amount * (1.0 - total_fee);
        let fee_usd = request.amount * total_fee;

        Ok(BoothQuote {
            booth: BoothId::Across,
            amount_out,
            duration_sec: 90,
            fee_usd,
            slippage_pct: 0.01,
            security_rating: SecurityRating::A,
            meta: Some(serde_json::json!({"relayPct": relay, "lpPct": lp})),
        })
    }
}

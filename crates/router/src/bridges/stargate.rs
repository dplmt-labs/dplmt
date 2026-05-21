use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct StargateBooth {
    pub client: reqwest::Client,
}

impl StargateBooth {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("dplmt-router/0.6")
                .build()
                .expect("client"),
        }
    }

    fn chain(c: ChainId) -> Option<&'static str> {
        Some(match c {
            ChainId::Ethereum => "ethereum",
            ChainId::Arbitrum => "arbitrum",
            ChainId::Base => "base",
            ChainId::Polygon => "polygon",
            ChainId::Bnb => "bnb",
            ChainId::Avalanche => "avalanche",
            _ => return None,
        })
    }
}

impl Default for StargateBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for StargateBooth {
    fn id(&self) -> BoothId {
        BoothId::Stargate
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let from = Self::chain(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let to = Self::chain(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let amount_raw = (request.amount * 1e6) as u128;
        let res = self
            .client
            .get("https://stargate.finance/api/v1/quotes")
            .query(&[
                ("srcChainKey", from),
                ("dstChainKey", to),
                ("srcAddress", "0x0000000000000000000000000000000000000001"),
                ("dstAddress", "0x0000000000000000000000000000000000000001"),
                ("srcAmount", amount_raw.to_string().as_str()),
                ("dstAmountMin", "0"),
            ])
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("stargate {}", res.status())));
        }
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        let q = data["quotes"]
            .as_array()
            .and_then(|arr| {
                arr.iter()
                    .find(|q| q["route"] == "taxi")
                    .cloned()
                    .or_else(|| arr.first().cloned())
            })
            .ok_or(RouterError::QuoteUnavailable)?;
        let amount_out = q["dstAmount"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(0.0)
            / 1e6;
        let duration_sec = if q["route"] == "bus" { 720 } else { 120 };

        Ok(BoothQuote {
            booth: BoothId::Stargate,
            amount_out,
            duration_sec,
            fee_usd: 0.4,
            slippage_pct: 0.02,
            security_rating: SecurityRating::A,
            meta: Some(serde_json::json!({"route": q["route"]})),
        })
    }
}

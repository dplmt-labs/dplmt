use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct MayanBooth {
    pub referrer: Option<String>,
    pub client: reqwest::Client,
}

impl MayanBooth {
    pub fn new(referrer: Option<String>) -> Self {
        Self {
            referrer,
            client: reqwest::Client::builder().user_agent("dplmt-router/0.6").build().expect("client"),
        }
    }

    fn chain(c: ChainId) -> Option<&'static str> {
        Some(match c {
            ChainId::Solana => "solana",
            ChainId::Ethereum => "ethereum",
            ChainId::Arbitrum => "arbitrum",
            ChainId::Base => "base",
            ChainId::Polygon => "polygon",
            ChainId::Bnb => "bsc",
            ChainId::Avalanche => "avalanche",
            ChainId::Sui => "sui",
        })
    }
}

#[async_trait]
impl Booth for MayanBooth {
    fn id(&self) -> BoothId {
        BoothId::Mayan
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let from = Self::chain(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let to = Self::chain(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let amount = format!("{}", request.amount);
        let res = self
            .client
            .get("https://price-api.mayan.finance/v3/quote")
            .query(&[
                ("amountIn", amount.as_str()),
                ("fromChain", from),
                ("toChain", to),
                ("slippageBps", "50"),
                ("referrerBps", "0"),
            ])
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("mayan {}", res.status())));
        }
        let data: serde_json::Value = res.json().await.map_err(|e| RouterError::Upstream(e.to_string()))?;
        let list = data.as_array().cloned().unwrap_or_default();
        let q = list.into_iter().next().ok_or(RouterError::QuoteUnavailable)?;
        let amount_out = q["expectedAmountOut"].as_f64().unwrap_or(0.0);
        let duration_sec = q["eta"].as_u64().unwrap_or(12) as u32;
        let fee_usd = q["totalFeeUsd"].as_f64().unwrap_or(0.5);

        Ok(BoothQuote {
            booth: BoothId::Mayan,
            amount_out,
            duration_sec,
            fee_usd,
            slippage_pct: 0.1,
            security_rating: SecurityRating::A,
            meta: Some(serde_json::json!({"type": q["type"]})),
        })
    }
}

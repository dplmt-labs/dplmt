use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::chain::ChainId;
use crate::errors::RouterError;

pub struct AllbridgeBooth {
    pub client: reqwest::Client,
}

impl AllbridgeBooth {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder().user_agent("dplmt-router/0.6").build().expect("client"),
        }
    }

    fn chain(c: ChainId) -> Option<&'static str> {
        Some(match c {
            ChainId::Ethereum => "ETH",
            ChainId::Arbitrum => "ARB",
            ChainId::Polygon => "POL",
            ChainId::Bnb => "BSC",
            ChainId::Avalanche => "AVA",
            ChainId::Solana => "SOL",
            _ => return None,
        })
    }
}

impl Default for AllbridgeBooth {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Booth for AllbridgeBooth {
    fn id(&self) -> BoothId {
        BoothId::Allbridge
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let _src = Self::chain(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let _dst = Self::chain(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let amount_str = request.amount.to_string();
        let res = self
            .client
            .get("https://core.api.allbridgecoreapi.net/swap/calculator")
            .query(&[
                ("amount", amount_str.as_str()),
                ("messenger", "ALLBRIDGE"),
            ])
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("allbridge {}", res.status())));
        }
        let data: serde_json::Value = res.json().await.map_err(|e| RouterError::Upstream(e.to_string()))?;
        let amount_out = data["amountToBeReceived"].as_f64().unwrap_or(0.0);
        let fee_usd = data["fee"].as_f64().unwrap_or(0.0);

        Ok(BoothQuote {
            booth: BoothId::Allbridge,
            amount_out,
            duration_sec: 240,
            fee_usd,
            slippage_pct: 0.2,
            security_rating: SecurityRating::B,
            meta: Some(serde_json::json!({"messenger": "ALLBRIDGE"})),
        })
    }
}

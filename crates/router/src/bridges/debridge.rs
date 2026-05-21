use async_trait::async_trait;

use crate::booth::{Booth, BoothId, BoothQuote, QuoteRequest, SecurityRating};
use crate::errors::RouterError;

pub struct DebridgeBooth {
    pub referral_code: Option<String>,
    pub client: reqwest::Client,
}

impl DebridgeBooth {
    pub fn new(referral_code: Option<String>) -> Self {
        Self {
            referral_code,
            client: reqwest::Client::builder()
                .user_agent("dplmt-router/0.6")
                .build()
                .expect("client"),
        }
    }

    fn chain_id(c: crate::chain::ChainId) -> Option<u64> {
        use crate::chain::ChainId::*;
        Some(match c {
            Ethereum => 1,
            Arbitrum => 42161,
            Base => 8453,
            Polygon => 137,
            Bnb => 56,
            Avalanche => 43114,
            Solana => 7565164,
            Sui => return None,
        })
    }
}

#[async_trait]
impl Booth for DebridgeBooth {
    fn id(&self) -> BoothId {
        BoothId::Debridge
    }

    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError> {
        let src = Self::chain_id(request.from).ok_or(RouterError::QuoteUnavailable)?;
        let dst = Self::chain_id(request.to).ok_or(RouterError::QuoteUnavailable)?;
        let url = "https://dln.debridge.finance/v1.0/dln/order/quote".to_string();
        let mut req = self.client.get(url).query(&[
            ("srcChainId", src.to_string()),
            ("dstChainId", dst.to_string()),
            (
                "srcChainTokenIn",
                "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48".to_string(),
            ),
            (
                "dstChainTokenOut",
                "0xaf88d065e77c8cC2239327C5EDb3A432268e5831".to_string(),
            ),
            ("srcChainTokenInAmount", "100000000".to_string()),
            ("dstChainTokenOutAmount", "auto".to_string()),
            ("prependOperatingExpenses", "true".to_string()),
        ]);
        if let Some(rc) = &self.referral_code {
            req = req.query(&[("referralCode", rc.clone())]);
        }
        let res = req
            .send()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        if !res.status().is_success() {
            return Err(RouterError::Upstream(format!("debridge {}", res.status())));
        }
        let data: serde_json::Value = res
            .json()
            .await
            .map_err(|e| RouterError::Upstream(e.to_string()))?;
        let amount_out_raw = data["estimation"]["dstChainTokenOut"]["amount"]
            .as_str()
            .unwrap_or("0");
        let amount_out = amount_out_raw.parse::<f64>().unwrap_or(0.0) / 1e6;

        Ok(BoothQuote {
            booth: BoothId::Debridge,
            amount_out,
            duration_sec: 30,
            fee_usd: 0.6,
            slippage_pct: 0.05,
            security_rating: SecurityRating::A,
            meta: Some(serde_json::json!({"raw": data["orderId"]})),
        })
    }
}

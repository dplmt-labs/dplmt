//! A "booth" is one bridge adapter. Each booth implements `Booth` so the engine
//! can poll all of them uniformly.

use crate::chain::ChainId;
use crate::errors::RouterError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BoothId {
    Wormhole,
    Debridge,
    Mayan,
    Stargate,
    Allbridge,
    Hyperlane,
    Across,
    Synapse,
}

impl BoothId {
    pub fn name(self) -> &'static str {
        match self {
            BoothId::Wormhole => "Wormhole",
            BoothId::Debridge => "deBridge",
            BoothId::Mayan => "Mayan",
            BoothId::Stargate => "Stargate",
            BoothId::Allbridge => "Allbridge Core",
            BoothId::Hyperlane => "Hyperlane",
            BoothId::Across => "Across",
            BoothId::Synapse => "Synapse",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            BoothId::Wormhole => "Booth I -- Plenary Channel",
            BoothId::Debridge => "Booth II -- Solver Network",
            BoothId::Mayan => "Booth III -- Swift Auction House",
            BoothId::Stargate => "Booth IV -- LayerZero Direct",
            BoothId::Allbridge => "Booth V -- Stable Channel",
            BoothId::Hyperlane => "Booth VI -- Modular Mailroom",
            BoothId::Across => "Booth VII -- Intent Relay",
            BoothId::Synapse => "Booth VIII -- Veteran Embassy",
        }
    }

    pub fn all() -> &'static [BoothId] {
        &[
            BoothId::Wormhole,
            BoothId::Debridge,
            BoothId::Mayan,
            BoothId::Stargate,
            BoothId::Allbridge,
            BoothId::Hyperlane,
            BoothId::Across,
            BoothId::Synapse,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoothQuote {
    pub booth: BoothId,
    pub amount_out: f64,
    pub duration_sec: u32,
    pub fee_usd: f64,
    pub slippage_pct: f64,
    pub security_rating: SecurityRating,
    pub meta: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecurityRating {
    A,
    B,
    C,
}

impl SecurityRating {
    pub fn weight(self) -> f64 {
        match self {
            SecurityRating::A => 1.0,
            SecurityRating::B => 0.75,
            SecurityRating::C => 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteRequest {
    pub from: ChainId,
    pub to: ChainId,
    pub from_token: String,
    pub to_token: String,
    pub amount: f64,
    pub sender: Option<String>,
    pub recipient: Option<String>,
}

#[async_trait]
pub trait Booth: Send + Sync {
    fn id(&self) -> BoothId;
    async fn quote(&self, request: &QuoteRequest) -> Result<BoothQuote, RouterError>;
}

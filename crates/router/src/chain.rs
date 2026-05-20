//! Chain metadata used by the routing engine.

use serde::{Deserialize, Serialize};

/// Identifier for one of the chains seated at the diplomatic round table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChainId {
    Solana,
    Ethereum,
    Arbitrum,
    Base,
    Polygon,
    Bnb,
    Avalanche,
    Sui,
}

impl ChainId {
    pub fn display_name(self) -> &'static str {
        match self {
            ChainId::Solana => "Solana",
            ChainId::Ethereum => "Ethereum",
            ChainId::Arbitrum => "Arbitrum",
            ChainId::Base => "Base",
            ChainId::Polygon => "Polygon",
            ChainId::Bnb => "BNB Chain",
            ChainId::Avalanche => "Avalanche",
            ChainId::Sui => "Sui",
        }
    }

    pub fn plaque(self) -> &'static str {
        match self {
            ChainId::Solana => "Republic of Solana",
            ChainId::Ethereum => "Federation of Ethereum",
            ChainId::Arbitrum => "Province of Arbitrum",
            ChainId::Base => "City-state of Base",
            ChainId::Polygon => "Polygon Republic",
            ChainId::Bnb => "BNB Federation",
            ChainId::Avalanche => "Avalanche Alliance",
            ChainId::Sui => "Empire of Sui",
        }
    }

    pub fn is_evm(self) -> bool {
        !matches!(self, ChainId::Solana | ChainId::Sui)
    }

    pub fn average_finality_sec(self) -> u32 {
        match self {
            ChainId::Solana => 3,
            ChainId::Ethereum => 13,
            ChainId::Arbitrum => 1,
            ChainId::Base => 2,
            ChainId::Polygon => 2,
            ChainId::Bnb => 3,
            ChainId::Avalanche => 2,
            ChainId::Sui => 3,
        }
    }

    pub fn explorer(self) -> &'static str {
        match self {
            ChainId::Solana => "https://solscan.io",
            ChainId::Ethereum => "https://etherscan.io",
            ChainId::Arbitrum => "https://arbiscan.io",
            ChainId::Base => "https://basescan.org",
            ChainId::Polygon => "https://polygonscan.com",
            ChainId::Bnb => "https://bscscan.com",
            ChainId::Avalanche => "https://snowtrace.io",
            ChainId::Sui => "https://suiscan.xyz",
        }
    }

    pub fn evm_chain_id(self) -> Option<u64> {
        Some(match self {
            ChainId::Ethereum => 1,
            ChainId::Arbitrum => 42161,
            ChainId::Base => 8453,
            ChainId::Polygon => 137,
            ChainId::Bnb => 56,
            ChainId::Avalanche => 43114,
            ChainId::Solana | ChainId::Sui => return None,
        })
    }

    pub fn all() -> &'static [ChainId] {
        &[
            ChainId::Solana,
            ChainId::Ethereum,
            ChainId::Arbitrum,
            ChainId::Base,
            ChainId::Polygon,
            ChainId::Bnb,
            ChainId::Avalanche,
            ChainId::Sui,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chain {
    pub id: ChainId,
    pub display_name: String,
    pub native_symbol: String,
    pub plaque: String,
    pub explorer: String,
    pub average_finality_sec: u32,
}

impl From<ChainId> for Chain {
    fn from(id: ChainId) -> Self {
        let native = match id {
            ChainId::Solana => "SOL",
            ChainId::Ethereum => "ETH",
            ChainId::Arbitrum => "ETH",
            ChainId::Base => "ETH",
            ChainId::Polygon => "MATIC",
            ChainId::Bnb => "BNB",
            ChainId::Avalanche => "AVAX",
            ChainId::Sui => "SUI",
        };
        Chain {
            id,
            display_name: id.display_name().to_string(),
            native_symbol: native.to_string(),
            plaque: id.plaque().to_string(),
            explorer: id.explorer().to_string(),
            average_finality_sec: id.average_finality_sec(),
        }
    }
}

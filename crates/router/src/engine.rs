//! The diplomat's engine. Polls all booths concurrently and folds the responses.

use crate::booth::{Booth, BoothQuote, QuoteRequest};
use crate::errors::RouterError;
use crate::scoring::{best_by, score_quotes, BestBy, ScoredQuote, SortMode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{timeout, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct CableResponse {
    pub request: QuoteRequest,
    pub mode: SortMode,
    pub quotes: Vec<ScoredQuote>,
    pub best: Option<BestBy>,
    pub partial_failures: Vec<String>,
}

pub struct Engine {
    booths: Vec<Arc<dyn Booth>>,
    booth_timeout: Duration,
}

impl Engine {
    pub fn new(booths: Vec<Arc<dyn Booth>>) -> Self {
        Self {
            booths,
            booth_timeout: Duration::from_millis(9000),
        }
    }

    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.booth_timeout = Duration::from_millis(ms);
        self
    }

    pub fn booth_count(&self) -> usize {
        self.booths.len()
    }

    pub async fn open_channel(
        &self,
        request: QuoteRequest,
        mode: SortMode,
    ) -> Result<CableResponse, RouterError> {
        if request.from == request.to {
            return Err(RouterError::SameChainRoute);
        }
        if request.amount <= 0.0 {
            return Err(RouterError::InvalidAmount);
        }
        let handles = self.booths.iter().cloned().map(|booth| {
            let req = request.clone();
            let to = self.booth_timeout;
            tokio::spawn(async move {
                let booth_id = booth.id();
                match timeout(to, booth.quote(&req)).await {
                    Ok(Ok(q)) => Ok(q),
                    Ok(Err(err)) => Err((booth_id, err.to_string())),
                    Err(_) => Err((booth_id, "booth timeout".to_string())),
                }
            })
        });

        let mut quotes: Vec<BoothQuote> = Vec::new();
        let mut partial_failures: Vec<String> = Vec::new();

        for handle in handles {
            match handle.await {
                Ok(Ok(q)) => quotes.push(q),
                Ok(Err((bid, msg))) => partial_failures.push(format!("{:?}: {}", bid, msg)),
                Err(join_err) => partial_failures.push(format!("join: {}", join_err)),
            }
        }

        let scored = score_quotes(&quotes, mode);
        let best = best_by(&quotes);

        Ok(CableResponse {
            request,
            mode,
            quotes: scored,
            best,
            partial_failures,
        })
    }
}

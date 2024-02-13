use std::str::FromStr;

use serde::Serialize;
use sqlx::FromRow;

use crate::quote::{QuoteCreateRequest, QuoteUpdateRequest};

#[derive(Debug, Serialize, FromRow, Clone)]
pub struct AppQuote {
    pub id: uuid::Uuid,
    pub book: String,
    pub quote: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl AppQuote {
    fn new(book: String, quote: String) -> Self {
        let now_timestamptz = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            created_at: now_timestamptz,
            updated_at: now_timestamptz,
        }
    }
}

impl From<QuoteCreateRequest> for AppQuote {
    fn from(payload: QuoteCreateRequest) -> Self {
        Self::new(payload.book, payload.quote)
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct AppQuoteUpdateRequest {
    pub id: uuid::Uuid,
    pub book: String,
    pub quote: String,
}

impl TryFrom<QuoteUpdateRequest> for AppQuoteUpdateRequest {
    type Error = ();
    fn try_from(payload: QuoteUpdateRequest) -> Result<Self, Self::Error> {
        let Ok(id) = uuid::Uuid::from_str(&payload.id) else {
            return Err(());
        };
        Ok(Self {
            id,
            book: payload.book,
            quote: payload.quote,
        })
    }
}

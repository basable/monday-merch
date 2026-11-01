//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_order.quote_version`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuoteVersion {
    pub id: uuid::Uuid,
    pub quote_id: uuid::Uuid,
    pub version: i32,
    pub lines: serde_json::Value,
    pub shipping_minor: i64,
    pub handling_minor: i64,
    pub currency: String,
    pub note: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub published_by: Option<String>,
}

/// A row of `nano_order.quote_signature`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuoteSignature {
    pub id: uuid::Uuid,
    pub quote_id: uuid::Uuid,
    pub version: i32,
    pub signer_identity_id: uuid::Uuid,
    pub signer_name: String,
    pub signer_email: String,
    pub po_number: Option<String>,
    pub signed_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_order.quote_comment`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuoteComment {
    pub id: uuid::Uuid,
    pub quote_id: uuid::Uuid,
    pub author: String,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_order.production_event`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductionEvent {
    pub id: uuid::Uuid,
    pub run_id: uuid::Uuid,
    pub kind: String,
    pub detail: serde_json::Value,
    pub actor: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_order.lead`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Lead {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub company_name: Option<String>,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub erase_after: chrono::DateTime<chrono::Utc>,
}

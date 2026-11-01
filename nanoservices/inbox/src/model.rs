//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_inbox.inbound_seen`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InboundSeen {
    pub provider_message_id: String,
    pub message_object_id: uuid::Uuid,
    pub first_seen_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_inbox.review_decision`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ReviewDecision {
    pub id: uuid::Uuid,
    pub document_id: uuid::Uuid,
    pub decision: String,
    pub corrected_kind: Option<String>,
    pub corrections: Option<serde_json::Value>,
    pub actor: String,
    pub decided_at: chrono::DateTime<chrono::Utc>,
}

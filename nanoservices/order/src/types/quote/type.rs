//! The `quote` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 1;
/// The public-id prefix: `quo_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "quo";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("quote", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub contact: serde_json::Value,
    pub contact_email: String, // immutable
    pub company_id: Option<uuid::Uuid>,
    pub delivery: serde_json::Value,
    pub timeline: String,
    pub deadline_note: Option<String>,
    pub consultant_id: Option<uuid::Uuid>,
    pub published_version: i32,
    pub valid_until: Option<chrono::DateTime<chrono::Utc>>,
    pub withdrawn: bool,
    pub requested_at: chrono::DateTime<chrono::Utc>, // immutable
    pub sla_due_at: chrono::DateTime<chrono::Utc>, // immutable
    pub source_order_id: Option<uuid::Uuid>, // immutable
    pub source_document_id: Option<uuid::Uuid>, // immutable
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub sla_alerted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sent_version: i32,
    pub send_intent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub signed_version: Option<i32>,
    pub order_id: Option<uuid::Uuid>,
    pub expired_at: Option<chrono::DateTime<chrono::Utc>>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

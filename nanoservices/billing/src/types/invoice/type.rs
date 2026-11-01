//! The `invoice` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 5;
/// The public-id prefix: `inv_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "inv";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("invoice", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub name: String, // immutable
    pub kind: String, // immutable
    pub company_id: uuid::Uuid, // immutable
    pub company_snapshot: serde_json::Value, // immutable
    pub lines: serde_json::Value, // immutable
    pub currency: String, // immutable
    pub net_minor: i64, // immutable
    pub vat_minor: i64, // immutable
    pub gross_minor: i64, // immutable
    pub tax_point: chrono::DateTime<chrono::Utc>, // immutable
    pub terms_days: i32, // immutable
    pub corrects_invoice_id: Option<uuid::Uuid>, // immutable
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub number: Option<String>,
    pub pdf_key: Option<String>,
    pub ledger_version: i64,
    pub issued_at: Option<chrono::DateTime<chrono::Utc>>,
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
    pub send_intent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reminders_sent: i32,
    pub reminder_intent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

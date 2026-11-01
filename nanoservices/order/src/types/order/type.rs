//! The `order` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 2;
/// The public-id prefix: `ord_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "ord";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("order", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub quote_id: uuid::Uuid, // immutable
    pub quote_version: i32, // immutable
    pub company_id: uuid::Uuid, // immutable
    pub lines: serde_json::Value, // immutable
    pub currency: String, // immutable
    pub delivery: serde_json::Value, // immutable
    pub timeline: String, // immutable
    pub payment_gate: String, // immutable
    pub cancel_requested: bool,
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub invoice_id: Option<String>,
    pub paid_at: Option<chrono::DateTime<chrono::Utc>>,
    pub production_started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub attention: Option<String>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

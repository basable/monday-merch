//! The `production_run` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 3;
/// The public-id prefix: `prun_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "prun";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("production_run", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub order_id: uuid::Uuid, // immutable
    pub order_generation: i64,
    pub supplier_slug: String, // immutable
    pub lines: serde_json::Value, // immutable
    pub destination: serde_json::Value, // immutable
    pub due_on: chrono::DateTime<chrono::Utc>,
    pub cancel_requested: bool,
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub po_intent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub po_sent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub acknowledged_at: Option<chrono::DateTime<chrono::Utc>>,
    pub supplier_ref: Option<String>,
    pub expected_at: Option<chrono::DateTime<chrono::Utc>>,
    pub inbound_id: Option<String>,
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

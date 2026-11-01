//! The `inbound_document` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 10;
/// The public-id prefix: `idoc_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "idoc";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("inbound_document", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub message_id: uuid::Uuid, // immutable
    pub part_name: String, // immutable
    pub object_key: String, // immutable
    pub mime_type: String, // immutable
    pub sha256: String, // immutable
    pub sender_supplier_slug: Option<String>, // immutable
    pub sender_verified: bool, // immutable
    pub route_hint: Option<String>, // immutable
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub kind: Option<String>,
    pub extraction: Option<serde_json::Value>,
    pub model: Option<String>,
    pub checks: Option<serde_json::Value>,
    pub review_reason: Option<String>,
    pub routed_to: Option<String>,
    pub routed_ref: Option<String>,
    pub applied_at: Option<chrono::DateTime<chrono::Utc>>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

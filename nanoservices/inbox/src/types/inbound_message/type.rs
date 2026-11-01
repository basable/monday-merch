//! The `inbound_message` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 9;
/// The public-id prefix: `imsg_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "imsg";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("inbound_message", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub provider_message_id: String, // immutable
    pub to_address: String, // immutable
    pub from_address: String, // immutable
    pub subject: String, // immutable
    pub received_at: chrono::DateTime<chrono::Utc>, // immutable
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub raw_key: Option<String>,
    pub message_id_header: Option<String>,
    pub sender_supplier_slug: Option<String>,
    pub sender_verified: Option<bool>,
    pub route_hint: Option<String>,
    pub document_count: Option<i32>,
    pub rejection: Option<String>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

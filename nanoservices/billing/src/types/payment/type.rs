//! The `payment` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 6;
/// The public-id prefix: `pay_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "pay";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("payment", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub reference: String, // immutable
    pub invoice_id: Option<uuid::Uuid>, // immutable
    pub bill_id: Option<uuid::Uuid>, // immutable
    pub company_id: uuid::Uuid, // immutable
    pub method: String, // immutable
    pub direction: String, // immutable
    pub amount_minor: i64, // immutable
    pub currency: String, // immutable
    pub value_date: Option<chrono::DateTime<chrono::Utc>>,
    pub cancel_requested: bool,
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub intent_at: Option<chrono::DateTime<chrono::Utc>>,
    pub provider_id: Option<String>,
    pub checkout_url: Option<String>,
    pub ledger_version: i64,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub reversed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub failure: Option<String>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

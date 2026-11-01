//! The `supplier_bill` type declaration: spec (desired state, the identity
//! fields immutable) and status (observed state, written only under claim
//! authority). Mirror every column of the migration here AND in adapter.rs.

use basable_processingobject::ProcessingObjectType;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Adapter;

/// The registry key allocated by the scaffolder; never changes.
pub const TYPE_KEY: i16 = 8;
/// The public-id prefix: `sbill_<base62>`; a duplicate panics at boot.
pub const PUBLIC_ID_PREFIX: &str = "sbill";

/// The declaration the store binds: name, key, prefix and the adapter.
pub fn decl() -> ProcessingObjectType<Spec, Status, Adapter> {
    ProcessingObjectType::new("supplier_bill", TYPE_KEY, PUBLIC_ID_PREFIX, Adapter)
}

/// Desired state. Immutable fields are guarded by the migration's trigger
/// and omitted from `write_spec`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spec {
    pub name: String, // immutable
    pub supplier_slug: String, // immutable
    pub supplier_invoice_number: String, // immutable
    pub lines: serde_json::Value, // immutable
    pub currency: String, // immutable
    pub net_minor: i64, // immutable
    pub vat_minor: i64, // immutable
    pub gross_minor: i64, // immutable
    pub invoice_date: chrono::DateTime<chrono::Utc>, // immutable
    pub due_date: Option<chrono::DateTime<chrono::Utc>>, // immutable
    pub purchase_ref: Option<String>, // immutable
    pub bank_account: Option<String>, // immutable
    pub source_document_id: uuid::Uuid, // immutable
    pub approved_by: Option<String>,
    pub voided: bool,
}

/// Observed state, one whole row per write.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Status {
    pub phase: String,
    pub match_result: Option<serde_json::Value>,
    pub attention: Option<String>,
    pub ledger_version: i64,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub settled_at: Option<chrono::DateTime<chrono::Utc>>,
    // Declared-effect slot columns go here when an adapter uses
    // Strategy::Declared: `effect: Option<EffectSlot>`.
}

// `Uuid` is the id type of every reference field; the import stays even
// when this type declares none.
#[allow(dead_code)]
type IdType = Uuid;

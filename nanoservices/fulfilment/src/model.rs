//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_fulfilment.stock_item`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StockItem {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub name: String,
    pub product_id: Option<uuid::Uuid>,
    pub variant_id: Option<uuid::Uuid>,
    pub size: Option<String>,
    pub proof_id: Option<uuid::Uuid>,
    pub is_kit: bool,
}

/// A row of `nano_fulfilment.stock_level`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StockLevel {
    pub id: uuid::Uuid,
    pub stock_item_id: uuid::Uuid,
    pub warehouse_slug: String,
    pub on_hand: i32,
    pub reserved: i32,
}

/// A row of `nano_fulfilment.stock_movement`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StockMovement {
    pub id: uuid::Uuid,
    pub stock_item_id: uuid::Uuid,
    pub warehouse_slug: String,
    pub kind: String,
    pub on_hand_delta: i32,
    pub reserved_delta: i32,
    pub source: String,
    pub actor: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_fulfilment.kit_component`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct KitComponent {
    pub id: uuid::Uuid,
    pub kit_item_id: uuid::Uuid,
    pub component_item_id: uuid::Uuid,
    pub quantity: i32,
}

/// A row of `nano_fulfilment.assembly_order`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AssemblyOrder {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub kit_item_id: uuid::Uuid,
    pub warehouse_slug: String,
    pub quantity: i32,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub cancelled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_fulfilment.inbound`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Inbound {
    pub id: uuid::Uuid,
    pub name: String,
    pub source_generation: i64,
    pub company_id: uuid::Uuid,
    pub warehouse_slug: String,
    pub lines: serde_json::Value,
    pub received_lines: serde_json::Value,
    pub received_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_fulfilment.low_stock_rule`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LowStockRule {
    pub id: uuid::Uuid,
    pub stock_item_id: uuid::Uuid,
    pub threshold: i32,
    pub notify_email: String,
}

/// A row of `nano_fulfilment.shipment_batch`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentBatch {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub created_by: String,
    pub charges: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_fulfilment.shipment_address`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentAddress {
    pub shipment_id: uuid::Uuid,
    pub address: Option<serde_json::Value>,
    pub erase_after: Option<chrono::DateTime<chrono::Utc>>,
    pub erased_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_fulfilment.shipment_event`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ShipmentEvent {
    pub id: uuid::Uuid,
    pub shipment_id: uuid::Uuid,
    pub kind: String,
    pub detail: serde_json::Value,
    pub dedupe_key: String,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

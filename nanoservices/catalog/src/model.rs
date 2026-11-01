//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_catalog.product`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
    pub description: String,
    pub category_slug: String,
    pub brand_slug: Option<String>,
    pub collections: serde_json::Value,
    pub attributes: serde_json::Value,
    pub impact_note: Option<String>,
    pub sustainability_labels: serde_json::Value,
    pub min_qty: i32,
    pub production_days: i32,
    pub customisable: bool,
    pub best_selling: bool,
    pub is_new: bool,
    pub weight_grams: i32,
    pub hs_code: Option<String>,
    pub origin_country: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub search_text: String,
}

/// A row of `nano_catalog.variant`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Variant {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub colour_slug: String,
    pub sku: String,
    pub sizes: serde_json::Value,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_catalog.price_tier`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PriceTier {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub currency: String,
    pub min_qty: i32,
    pub unit_minor: i64,
}

/// A row of `nano_catalog.decoration_option`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DecorationOption {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub print_method_slug: String,
    pub areas: serde_json::Value,
    pub currency: String,
    pub setup_minor: i64,
    pub per_unit_minor: i64,
}

/// A row of `nano_catalog.product_image`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductImage {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub variant_id: Option<uuid::Uuid>,
    pub object_key: String,
    pub alt_text: String,
    pub position: i32,
    pub view: Option<String>,
}

/// A row of `nano_catalog.supplier_offer`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierOffer {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub supplier_slug: String,
    pub supplier_sku: String,
    pub currency: String,
    pub cost_minor: i64,
    pub lead_days: i32,
    pub preferred: bool,
}

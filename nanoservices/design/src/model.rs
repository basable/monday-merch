//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_design.brand_profile`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BrandProfile {
    pub company_id: uuid::Uuid,
    pub colours: serde_json::Value,
    pub fonts: serde_json::Value,
    pub guidelines: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_design.brand_asset`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BrandAsset {
    pub id: uuid::Uuid,
    pub company_id: Option<uuid::Uuid>,
    pub owner_identity_id: Option<uuid::Uuid>,
    pub kind: String,
    pub filename: String,
    pub object_key: String,
    pub mime_type: String,
    pub size_bytes: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub confirmed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_design.proof`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Proof {
    pub id: uuid::Uuid,
    pub subject_name: String,
    pub line_ref: String,
    pub version: i32,
    pub preview_key: String,
    pub artwork_key: String,
    pub print_spec: serde_json::Value,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub approved_at: Option<chrono::DateTime<chrono::Utc>>,
    pub approved_by: Option<String>,
    pub source_design_version_id: Option<uuid::Uuid>,
}

/// A row of `nano_design.design_document`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DesignDocument {
    pub id: uuid::Uuid,
    pub company_id: Option<uuid::Uuid>,
    pub owner_identity_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub variant_id: Option<uuid::Uuid>,
    pub name: String,
    pub draft: serde_json::Value,
    pub schema_version: i32,
    pub revision: i32,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_design.design_version`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DesignVersion {
    pub id: uuid::Uuid,
    pub design_id: uuid::Uuid,
    pub version: i32,
    pub document: serde_json::Value,
    pub schema_version: i32,
    pub preview_keys: serde_json::Value,
    pub created_by: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_design.design_template`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct DesignTemplate {
    pub id: uuid::Uuid,
    pub slug: String,
    pub title: String,
    pub kind: String,
    pub product_id: Option<uuid::Uuid>,
    pub document: serde_json::Value,
    pub schema_version: i32,
    pub preview_key: String,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    pub sort_order: i32,
}

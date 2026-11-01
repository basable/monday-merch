//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_account.company`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Company {
    pub id: uuid::Uuid,
    pub source_name: Option<String>,
    pub display_name: String,
    pub legal_name: Option<String>,
    pub vat_number: Option<String>,
    pub billing_address: Option<serde_json::Value>,
    pub country: String,
    pub currency: String,
    pub payment_gate: String,
    pub terms_days: Option<i32>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_account.department`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Department {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub name: String,
    pub cost_center: Option<String>,
}

/// A row of `nano_account.membership`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Membership {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub email: String,
    pub identity_id: Option<uuid::Uuid>,
    pub role: String,
    pub department_id: Option<uuid::Uuid>,
    pub invited_at: chrono::DateTime<chrono::Utc>,
    pub accepted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_account.staff_member`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StaffMember {
    pub id: uuid::Uuid,
    pub email: String,
    pub identity_id: Option<uuid::Uuid>,
    pub role: String,
    pub active: bool,
}

/// A row of `nano_account.address`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Address {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub label: String,
    pub contact_name: String,
    pub street: String,
    pub house_number: String,
    pub postcode: String,
    pub city: String,
    pub region: Option<String>,
    pub country: String,
    pub phone: Option<String>,
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_account.recipient`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Recipient {
    pub id: uuid::Uuid,
    pub company_id: uuid::Uuid,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub home_address: Option<serde_json::Value>,
    pub labels: serde_json::Value,
    pub source: String,
    pub erase_after: Option<chrono::DateTime<chrono::Utc>>,
    pub erased_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// A row of `nano_account.audit_event`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AuditEvent {
    pub id: uuid::Uuid,
    pub company_id: Option<uuid::Uuid>,
    pub actor: String,
    pub action: String,
    pub subject: String,
    pub detail: serde_json::Value,
    pub occurred_at: chrono::DateTime<chrono::Utc>,
}

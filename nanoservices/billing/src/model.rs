//! Row structs of this nanoservice's plain tables, one per table, mirroring
//! the columns in db/app/migrations exactly.

use serde::{Deserialize, Serialize};

/// A row of `nano_billing.invoice_counter`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InvoiceCounter {
    pub series: String,
    pub next_number: i64,
}

/// A row of `nano_billing.invoice_number`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InvoiceNumber {
    pub invoice_id: uuid::Uuid,
    pub series: String,
    pub number: String,
    pub assigned_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_billing.payment_webhook_event`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PaymentWebhookEvent {
    pub event_id: String,
    pub provider_id: String,
    pub kind: String,
    pub payload: serde_json::Value,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

/// A row of `nano_billing.supplier_bill_key`.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SupplierBillKey {
    pub dedupe_key: String,
    pub bill_id: uuid::Uuid,
}

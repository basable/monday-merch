//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_billing`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Billing;

impl Billing {
    // --- invoice_counter ---

    pub async fn insert_invoice_counter(&self, row: &model::InvoiceCounter) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO invoice_counter (series, next_number) VALUES ($1, $2)",
        )
        .bind(&row.series)
        .bind(row.next_number)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_invoice_counter(&self, id: Uuid) -> Result<Option<model::InvoiceCounter>, sqlx::Error> {
        sqlx::query_as::<_, model::InvoiceCounter>("SELECT series, next_number FROM invoice_counter WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- invoice_number ---

    pub async fn insert_invoice_number(&self, row: &model::InvoiceNumber) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO invoice_number (invoice_id, series, number, assigned_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(row.invoice_id)
        .bind(&row.series)
        .bind(&row.number)
        .bind(&row.assigned_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_invoice_number(&self, id: Uuid) -> Result<Option<model::InvoiceNumber>, sqlx::Error> {
        sqlx::query_as::<_, model::InvoiceNumber>("SELECT invoice_id, series, number, assigned_at FROM invoice_number WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- payment_webhook_event ---

    pub async fn insert_payment_webhook_event(&self, row: &model::PaymentWebhookEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO payment_webhook_event (event_id, provider_id, kind, payload, received_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(&row.event_id)
        .bind(&row.provider_id)
        .bind(&row.kind)
        .bind(&row.payload)
        .bind(&row.received_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_payment_webhook_event(&self, id: Uuid) -> Result<Option<model::PaymentWebhookEvent>, sqlx::Error> {
        sqlx::query_as::<_, model::PaymentWebhookEvent>("SELECT event_id, provider_id, kind, payload, received_at FROM payment_webhook_event WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- supplier_bill_key ---

    pub async fn insert_supplier_bill_key(&self, row: &model::SupplierBillKey) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO supplier_bill_key (dedupe_key, bill_id) VALUES ($1, $2)",
        )
        .bind(&row.dedupe_key)
        .bind(row.bill_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_supplier_bill_key(&self, id: Uuid) -> Result<Option<model::SupplierBillKey>, sqlx::Error> {
        sqlx::query_as::<_, model::SupplierBillKey>("SELECT dedupe_key, bill_id FROM supplier_bill_key WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}

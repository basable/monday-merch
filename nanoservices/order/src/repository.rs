//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_order`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Order;

impl Order {
    // --- quote_version ---

    pub async fn insert_quote_version(&self, row: &model::QuoteVersion) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO quote_version (id, quote_id, version, lines, shipping_minor, handling_minor, currency, note, published_at, published_by) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(row.id)
        .bind(row.quote_id)
        .bind(row.version)
        .bind(&row.lines)
        .bind(row.shipping_minor)
        .bind(row.handling_minor)
        .bind(&row.currency)
        .bind(&row.note)
        .bind(&row.published_at)
        .bind(&row.published_by)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_quote_version(&self, id: Uuid) -> Result<Option<model::QuoteVersion>, sqlx::Error> {
        sqlx::query_as::<_, model::QuoteVersion>("SELECT id, quote_id, version, lines, shipping_minor, handling_minor, currency, note, published_at, published_by FROM quote_version WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- quote_signature ---

    pub async fn insert_quote_signature(&self, row: &model::QuoteSignature) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO quote_signature (id, quote_id, version, signer_identity_id, signer_name, signer_email, po_number, signed_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(row.id)
        .bind(row.quote_id)
        .bind(row.version)
        .bind(row.signer_identity_id)
        .bind(&row.signer_name)
        .bind(&row.signer_email)
        .bind(&row.po_number)
        .bind(&row.signed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_quote_signature(&self, id: Uuid) -> Result<Option<model::QuoteSignature>, sqlx::Error> {
        sqlx::query_as::<_, model::QuoteSignature>("SELECT id, quote_id, version, signer_identity_id, signer_name, signer_email, po_number, signed_at FROM quote_signature WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- quote_comment ---

    pub async fn insert_quote_comment(&self, row: &model::QuoteComment) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO quote_comment (id, quote_id, author, body, created_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(row.quote_id)
        .bind(&row.author)
        .bind(&row.body)
        .bind(&row.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_quote_comment(&self, id: Uuid) -> Result<Option<model::QuoteComment>, sqlx::Error> {
        sqlx::query_as::<_, model::QuoteComment>("SELECT id, quote_id, author, body, created_at FROM quote_comment WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- production_event ---

    pub async fn insert_production_event(&self, row: &model::ProductionEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO production_event (id, run_id, kind, detail, actor, occurred_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id)
        .bind(row.run_id)
        .bind(&row.kind)
        .bind(&row.detail)
        .bind(&row.actor)
        .bind(&row.occurred_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_production_event(&self, id: Uuid) -> Result<Option<model::ProductionEvent>, sqlx::Error> {
        sqlx::query_as::<_, model::ProductionEvent>("SELECT id, run_id, kind, detail, actor, occurred_at FROM production_event WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- lead ---

    pub async fn insert_lead(&self, row: &model::Lead) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO lead (id, email, name, company_name, message, created_at, erase_after) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(&row.email)
        .bind(&row.name)
        .bind(&row.company_name)
        .bind(&row.message)
        .bind(&row.created_at)
        .bind(&row.erase_after)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_lead(&self, id: Uuid) -> Result<Option<model::Lead>, sqlx::Error> {
        sqlx::query_as::<_, model::Lead>("SELECT id, email, name, company_name, message, created_at, erase_after FROM lead WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}

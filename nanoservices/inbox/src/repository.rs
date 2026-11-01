//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_inbox`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Inbox;

impl Inbox {
    // --- inbound_seen ---

    pub async fn insert_inbound_seen(&self, row: &model::InboundSeen) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO inbound_seen (provider_message_id, message_object_id, first_seen_at) VALUES ($1, $2, $3)",
        )
        .bind(&row.provider_message_id)
        .bind(row.message_object_id)
        .bind(&row.first_seen_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_inbound_seen(&self, id: Uuid) -> Result<Option<model::InboundSeen>, sqlx::Error> {
        sqlx::query_as::<_, model::InboundSeen>("SELECT provider_message_id, message_object_id, first_seen_at FROM inbound_seen WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- review_decision ---

    pub async fn insert_review_decision(&self, row: &model::ReviewDecision) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO review_decision (id, document_id, decision, corrected_kind, corrections, actor, decided_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.document_id)
        .bind(&row.decision)
        .bind(&row.corrected_kind)
        .bind(&row.corrections)
        .bind(&row.actor)
        .bind(&row.decided_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_review_decision(&self, id: Uuid) -> Result<Option<model::ReviewDecision>, sqlx::Error> {
        sqlx::query_as::<_, model::ReviewDecision>("SELECT id, document_id, decision, corrected_kind, corrections, actor, decided_at FROM review_decision WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}

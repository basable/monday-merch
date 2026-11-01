//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `quote_spec` / `quote_status` tables. Five callbacks
//! plus the optional finalize, every one database-only over the restricted
//! `Tx`; every column present in BOTH `write_spec` and `read_rows`, so a
//! column can never be silently lost. Never touches envelope columns.
//! Errors are `sqlx::Error`: a column mapper fails no other way, and the
//! framework wraps them.

use basable_processingobject::{Adapter as AdapterTrait, Object, Ref, Row, Tx};
use uuid::Uuid;

use super::{Spec, Status};

pub struct Adapter;

impl AdapterTrait<Spec, Status> for Adapter {
    async fn insert_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO quote_spec (id, contact, contact_email, company_id, delivery, timeline, deadline_note, consultant_id, published_version, valid_until, withdrawn, requested_at, sla_due_at, source_order_id, source_document_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)")
            .bind(r.id)
            .bind(&spec.contact)
            .bind(&spec.contact_email)
            .bind(spec.company_id)
            .bind(&spec.delivery)
            .bind(&spec.timeline)
            .bind(&spec.deadline_note)
            .bind(spec.consultant_id)
            .bind(spec.published_version)
            .bind(&spec.valid_until)
            .bind(spec.withdrawn)
            .bind(&spec.requested_at)
            .bind(&spec.sla_due_at)
            .bind(spec.source_order_id)
            .bind(spec.source_document_id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO quote_status (id, phase, sla_alerted_at, sent_version, send_intent_at, signed_version, order_id, expired_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.sla_alerted_at)
            .bind(status.sent_version)
            .bind(&status.send_intent_at)
            .bind(status.signed_version)
            .bind(status.order_id)
            .bind(&status.expired_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.contact, s.contact_email, s.company_id, s.delivery, s.timeline, s.deadline_note, s.consultant_id, s.published_version, s.valid_until, s.withdrawn, s.requested_at, s.sla_due_at, s.source_order_id, s.source_document_id, t.phase, t.sla_alerted_at, t.sent_version, t.send_intent_at, t.signed_version, t.order_id, t.expired_at \
             FROM quote_spec s JOIN quote_status t ON t.id = s.id WHERE s.id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(tx)
        .await?;
        rows.into_iter()
            .map(|row| {
                use sqlx::Row as _;
                Ok(Row {
                    id: row.try_get("id")?,
                    spec: Spec {
                        contact: row.try_get("contact")?,
                        contact_email: row.try_get("contact_email")?,
                        company_id: row.try_get("company_id")?,
                        delivery: row.try_get("delivery")?,
                        timeline: row.try_get("timeline")?,
                        deadline_note: row.try_get("deadline_note")?,
                        consultant_id: row.try_get("consultant_id")?,
                        published_version: row.try_get("published_version")?,
                        valid_until: row.try_get("valid_until")?,
                        withdrawn: row.try_get("withdrawn")?,
                        requested_at: row.try_get("requested_at")?,
                        sla_due_at: row.try_get("sla_due_at")?,
                        source_order_id: row.try_get("source_order_id")?,
                        source_document_id: row.try_get("source_document_id")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        sla_alerted_at: row.try_get("sla_alerted_at")?,
                        sent_version: row.try_get("sent_version")?,
                        send_intent_at: row.try_get("send_intent_at")?,
                        signed_version: row.try_get("signed_version")?,
                        order_id: row.try_get("order_id")?,
                        expired_at: row.try_get("expired_at")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE quote_spec SET updated_at = clock_timestamp(), contact = $2, company_id = $3, delivery = $4, timeline = $5, deadline_note = $6, consultant_id = $7, published_version = $8, valid_until = $9, withdrawn = $10 WHERE id = $1")
            .bind(r.id)
            .bind(&spec.contact)
            .bind(spec.company_id)
            .bind(&spec.delivery)
            .bind(&spec.timeline)
            .bind(&spec.deadline_note)
            .bind(spec.consultant_id)
            .bind(spec.published_version)
            .bind(&spec.valid_until)
            .bind(spec.withdrawn)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE quote_status SET updated_at = clock_timestamp(), phase = $2, sla_alerted_at = $3, sent_version = $4, send_intent_at = $5, signed_version = $6, order_id = $7, expired_at = $8 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.sla_alerted_at)
            .bind(status.sent_version)
            .bind(&status.send_intent_at)
            .bind(status.signed_version)
            .bind(status.order_id)
            .bind(&status.expired_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

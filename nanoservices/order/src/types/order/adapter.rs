//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `order_spec` / `order_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO order_spec (id, quote_id, quote_version, company_id, lines, currency, delivery, timeline, payment_gate, cancel_requested) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(r.id)
            .bind(spec.quote_id)
            .bind(spec.quote_version)
            .bind(spec.company_id)
            .bind(&spec.lines)
            .bind(&spec.currency)
            .bind(&spec.delivery)
            .bind(&spec.timeline)
            .bind(&spec.payment_gate)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO order_status (id, phase, invoice_id, paid_at, production_started_at, received_at, completed_at, attention) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.invoice_id)
            .bind(&status.paid_at)
            .bind(&status.production_started_at)
            .bind(&status.received_at)
            .bind(&status.completed_at)
            .bind(&status.attention)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.quote_id, s.quote_version, s.company_id, s.lines, s.currency, s.delivery, s.timeline, s.payment_gate, s.cancel_requested, t.phase, t.invoice_id, t.paid_at, t.production_started_at, t.received_at, t.completed_at, t.attention \
             FROM order_spec s JOIN order_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        quote_id: row.try_get("quote_id")?,
                        quote_version: row.try_get("quote_version")?,
                        company_id: row.try_get("company_id")?,
                        lines: row.try_get("lines")?,
                        currency: row.try_get("currency")?,
                        delivery: row.try_get("delivery")?,
                        timeline: row.try_get("timeline")?,
                        payment_gate: row.try_get("payment_gate")?,
                        cancel_requested: row.try_get("cancel_requested")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        invoice_id: row.try_get("invoice_id")?,
                        paid_at: row.try_get("paid_at")?,
                        production_started_at: row.try_get("production_started_at")?,
                        received_at: row.try_get("received_at")?,
                        completed_at: row.try_get("completed_at")?,
                        attention: row.try_get("attention")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE order_spec SET updated_at = clock_timestamp(), cancel_requested = $2 WHERE id = $1")
            .bind(r.id)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE order_status SET updated_at = clock_timestamp(), phase = $2, invoice_id = $3, paid_at = $4, production_started_at = $5, received_at = $6, completed_at = $7, attention = $8 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.invoice_id)
            .bind(&status.paid_at)
            .bind(&status.production_started_at)
            .bind(&status.received_at)
            .bind(&status.completed_at)
            .bind(&status.attention)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

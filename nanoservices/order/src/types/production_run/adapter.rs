//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `production_run_spec` / `production_run_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO production_run_spec (id, order_id, order_generation, supplier_slug, lines, destination, due_on, cancel_requested) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(r.id)
            .bind(spec.order_id)
            .bind(spec.order_generation)
            .bind(&spec.supplier_slug)
            .bind(&spec.lines)
            .bind(&spec.destination)
            .bind(&spec.due_on)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO production_run_status (id, phase, po_intent_at, po_sent_at, acknowledged_at, supplier_ref, expected_at, inbound_id, received_at, carrier, tracking_number) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.po_intent_at)
            .bind(&status.po_sent_at)
            .bind(&status.acknowledged_at)
            .bind(&status.supplier_ref)
            .bind(&status.expected_at)
            .bind(&status.inbound_id)
            .bind(&status.received_at)
            .bind(&status.carrier)
            .bind(&status.tracking_number)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.order_id, s.order_generation, s.supplier_slug, s.lines, s.destination, s.due_on, s.cancel_requested, t.phase, t.po_intent_at, t.po_sent_at, t.acknowledged_at, t.supplier_ref, t.expected_at, t.inbound_id, t.received_at, t.carrier, t.tracking_number \
             FROM production_run_spec s JOIN production_run_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        order_id: row.try_get("order_id")?,
                        order_generation: row.try_get("order_generation")?,
                        supplier_slug: row.try_get("supplier_slug")?,
                        lines: row.try_get("lines")?,
                        destination: row.try_get("destination")?,
                        due_on: row.try_get("due_on")?,
                        cancel_requested: row.try_get("cancel_requested")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        po_intent_at: row.try_get("po_intent_at")?,
                        po_sent_at: row.try_get("po_sent_at")?,
                        acknowledged_at: row.try_get("acknowledged_at")?,
                        supplier_ref: row.try_get("supplier_ref")?,
                        expected_at: row.try_get("expected_at")?,
                        inbound_id: row.try_get("inbound_id")?,
                        received_at: row.try_get("received_at")?,
                        carrier: row.try_get("carrier")?,
                        tracking_number: row.try_get("tracking_number")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE production_run_spec SET updated_at = clock_timestamp(), order_generation = $2, due_on = $3, cancel_requested = $4 WHERE id = $1")
            .bind(r.id)
            .bind(spec.order_generation)
            .bind(&spec.due_on)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE production_run_status SET updated_at = clock_timestamp(), phase = $2, po_intent_at = $3, po_sent_at = $4, acknowledged_at = $5, supplier_ref = $6, expected_at = $7, inbound_id = $8, received_at = $9, carrier = $10, tracking_number = $11 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.po_intent_at)
            .bind(&status.po_sent_at)
            .bind(&status.acknowledged_at)
            .bind(&status.supplier_ref)
            .bind(&status.expected_at)
            .bind(&status.inbound_id)
            .bind(&status.received_at)
            .bind(&status.carrier)
            .bind(&status.tracking_number)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

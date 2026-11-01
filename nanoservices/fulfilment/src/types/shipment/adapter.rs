//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `shipment_spec` / `shipment_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO shipment_spec (id, name, source_generation, company_id, batch_id, warehouse_slug, lines, service_level, incoterm, cancel_requested) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(r.id)
            .bind(&spec.name)
            .bind(spec.source_generation)
            .bind(spec.company_id)
            .bind(spec.batch_id)
            .bind(&spec.warehouse_slug)
            .bind(&spec.lines)
            .bind(&spec.service_level)
            .bind(&spec.incoterm)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO shipment_status (id, phase, reserved_at, picked_at, label_slot, label_id, label_key, carrier, tracking_number, dispatched_at, delivered_at, exception) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.reserved_at)
            .bind(&status.picked_at)
            .bind(&status.label_slot)
            .bind(&status.label_id)
            .bind(&status.label_key)
            .bind(&status.carrier)
            .bind(&status.tracking_number)
            .bind(&status.dispatched_at)
            .bind(&status.delivered_at)
            .bind(&status.exception)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.name, s.source_generation, s.company_id, s.batch_id, s.warehouse_slug, s.lines, s.service_level, s.incoterm, s.cancel_requested, t.phase, t.reserved_at, t.picked_at, t.label_slot, t.label_id, t.label_key, t.carrier, t.tracking_number, t.dispatched_at, t.delivered_at, t.exception \
             FROM shipment_spec s JOIN shipment_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        name: row.try_get("name")?,
                        source_generation: row.try_get("source_generation")?,
                        company_id: row.try_get("company_id")?,
                        batch_id: row.try_get("batch_id")?,
                        warehouse_slug: row.try_get("warehouse_slug")?,
                        lines: row.try_get("lines")?,
                        service_level: row.try_get("service_level")?,
                        incoterm: row.try_get("incoterm")?,
                        cancel_requested: row.try_get("cancel_requested")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        reserved_at: row.try_get("reserved_at")?,
                        picked_at: row.try_get("picked_at")?,
                        label_slot: row.try_get("label_slot")?,
                        label_id: row.try_get("label_id")?,
                        label_key: row.try_get("label_key")?,
                        carrier: row.try_get("carrier")?,
                        tracking_number: row.try_get("tracking_number")?,
                        dispatched_at: row.try_get("dispatched_at")?,
                        delivered_at: row.try_get("delivered_at")?,
                        exception: row.try_get("exception")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE shipment_spec SET updated_at = clock_timestamp(), source_generation = $2, cancel_requested = $3 WHERE id = $1")
            .bind(r.id)
            .bind(spec.source_generation)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE shipment_status SET updated_at = clock_timestamp(), phase = $2, reserved_at = $3, picked_at = $4, label_slot = $5, label_id = $6, label_key = $7, carrier = $8, tracking_number = $9, dispatched_at = $10, delivered_at = $11, exception = $12 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.reserved_at)
            .bind(&status.picked_at)
            .bind(&status.label_slot)
            .bind(&status.label_id)
            .bind(&status.label_key)
            .bind(&status.carrier)
            .bind(&status.tracking_number)
            .bind(&status.dispatched_at)
            .bind(&status.delivered_at)
            .bind(&status.exception)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

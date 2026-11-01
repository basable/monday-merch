//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `inbound_message_spec` / `inbound_message_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO inbound_message_spec (id, provider_message_id, to_address, from_address, subject, received_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(r.id)
            .bind(&spec.provider_message_id)
            .bind(&spec.to_address)
            .bind(&spec.from_address)
            .bind(&spec.subject)
            .bind(&spec.received_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO inbound_message_status (id, phase, raw_key, message_id_header, sender_supplier_slug, sender_verified, route_hint, document_count, rejection) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.raw_key)
            .bind(&status.message_id_header)
            .bind(&status.sender_supplier_slug)
            .bind(status.sender_verified)
            .bind(&status.route_hint)
            .bind(status.document_count)
            .bind(&status.rejection)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.provider_message_id, s.to_address, s.from_address, s.subject, s.received_at, t.phase, t.raw_key, t.message_id_header, t.sender_supplier_slug, t.sender_verified, t.route_hint, t.document_count, t.rejection \
             FROM inbound_message_spec s JOIN inbound_message_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        provider_message_id: row.try_get("provider_message_id")?,
                        to_address: row.try_get("to_address")?,
                        from_address: row.try_get("from_address")?,
                        subject: row.try_get("subject")?,
                        received_at: row.try_get("received_at")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        raw_key: row.try_get("raw_key")?,
                        message_id_header: row.try_get("message_id_header")?,
                        sender_supplier_slug: row.try_get("sender_supplier_slug")?,
                        sender_verified: row.try_get("sender_verified")?,
                        route_hint: row.try_get("route_hint")?,
                        document_count: row.try_get("document_count")?,
                        rejection: row.try_get("rejection")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // No mutable spec columns yet: only the timestamp moves.
        sqlx::query("UPDATE inbound_message_spec SET updated_at = clock_timestamp() WHERE id = $1")
            .bind(r.id)
            .execute(tx)
            .await?;
        let _ = spec;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE inbound_message_status SET updated_at = clock_timestamp(), phase = $2, raw_key = $3, message_id_header = $4, sender_supplier_slug = $5, sender_verified = $6, route_hint = $7, document_count = $8, rejection = $9 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.raw_key)
            .bind(&status.message_id_header)
            .bind(&status.sender_supplier_slug)
            .bind(status.sender_verified)
            .bind(&status.route_hint)
            .bind(status.document_count)
            .bind(&status.rejection)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

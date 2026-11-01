//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `inbound_document_spec` / `inbound_document_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO inbound_document_spec (id, message_id, part_name, object_key, mime_type, sha256, sender_supplier_slug, sender_verified, route_hint) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(r.id)
            .bind(spec.message_id)
            .bind(&spec.part_name)
            .bind(&spec.object_key)
            .bind(&spec.mime_type)
            .bind(&spec.sha256)
            .bind(&spec.sender_supplier_slug)
            .bind(spec.sender_verified)
            .bind(&spec.route_hint)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO inbound_document_status (id, phase, kind, extraction, model, checks, review_reason, routed_to, routed_ref, applied_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.kind)
            .bind(&status.extraction)
            .bind(&status.model)
            .bind(&status.checks)
            .bind(&status.review_reason)
            .bind(&status.routed_to)
            .bind(&status.routed_ref)
            .bind(&status.applied_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.message_id, s.part_name, s.object_key, s.mime_type, s.sha256, s.sender_supplier_slug, s.sender_verified, s.route_hint, t.phase, t.kind, t.extraction, t.model, t.checks, t.review_reason, t.routed_to, t.routed_ref, t.applied_at \
             FROM inbound_document_spec s JOIN inbound_document_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        message_id: row.try_get("message_id")?,
                        part_name: row.try_get("part_name")?,
                        object_key: row.try_get("object_key")?,
                        mime_type: row.try_get("mime_type")?,
                        sha256: row.try_get("sha256")?,
                        sender_supplier_slug: row.try_get("sender_supplier_slug")?,
                        sender_verified: row.try_get("sender_verified")?,
                        route_hint: row.try_get("route_hint")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        kind: row.try_get("kind")?,
                        extraction: row.try_get("extraction")?,
                        model: row.try_get("model")?,
                        checks: row.try_get("checks")?,
                        review_reason: row.try_get("review_reason")?,
                        routed_to: row.try_get("routed_to")?,
                        routed_ref: row.try_get("routed_ref")?,
                        applied_at: row.try_get("applied_at")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // No mutable spec columns yet: only the timestamp moves.
        sqlx::query("UPDATE inbound_document_spec SET updated_at = clock_timestamp() WHERE id = $1")
            .bind(r.id)
            .execute(tx)
            .await?;
        let _ = spec;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE inbound_document_status SET updated_at = clock_timestamp(), phase = $2, kind = $3, extraction = $4, model = $5, checks = $6, review_reason = $7, routed_to = $8, routed_ref = $9, applied_at = $10 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.kind)
            .bind(&status.extraction)
            .bind(&status.model)
            .bind(&status.checks)
            .bind(&status.review_reason)
            .bind(&status.routed_to)
            .bind(&status.routed_ref)
            .bind(&status.applied_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

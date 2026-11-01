//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `vat_return_spec` / `vat_return_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO vat_return_spec (id, country, period, filed, filing_reference) VALUES ($1, $2, $3, $4, $5)")
            .bind(r.id)
            .bind(&spec.country)
            .bind(&spec.period)
            .bind(spec.filed)
            .bind(&spec.filing_reference)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO vat_return_status (id, phase, vat_due_minor, ledger_version, filed_at) VALUES ($1, $2, $3, $4, $5)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(status.vat_due_minor)
            .bind(status.ledger_version)
            .bind(&status.filed_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.country, s.period, s.filed, s.filing_reference, t.phase, t.vat_due_minor, t.ledger_version, t.filed_at \
             FROM vat_return_spec s JOIN vat_return_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        country: row.try_get("country")?,
                        period: row.try_get("period")?,
                        filed: row.try_get("filed")?,
                        filing_reference: row.try_get("filing_reference")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        vat_due_minor: row.try_get("vat_due_minor")?,
                        ledger_version: row.try_get("ledger_version")?,
                        filed_at: row.try_get("filed_at")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE vat_return_spec SET updated_at = clock_timestamp(), filed = $2, filing_reference = $3 WHERE id = $1")
            .bind(r.id)
            .bind(spec.filed)
            .bind(&spec.filing_reference)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE vat_return_status SET updated_at = clock_timestamp(), phase = $2, vat_due_minor = $3, ledger_version = $4, filed_at = $5 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(status.vat_due_minor)
            .bind(status.ledger_version)
            .bind(&status.filed_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

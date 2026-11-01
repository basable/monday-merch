//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `payment_spec` / `payment_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO payment_spec (id, reference, invoice_id, bill_id, company_id, method, direction, amount_minor, currency, value_date, cancel_requested) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
            .bind(r.id)
            .bind(&spec.reference)
            .bind(spec.invoice_id)
            .bind(spec.bill_id)
            .bind(spec.company_id)
            .bind(&spec.method)
            .bind(&spec.direction)
            .bind(spec.amount_minor)
            .bind(&spec.currency)
            .bind(&spec.value_date)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO payment_status (id, phase, intent_at, provider_id, checkout_url, ledger_version, settled_at, reversed_at, failure) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.intent_at)
            .bind(&status.provider_id)
            .bind(&status.checkout_url)
            .bind(status.ledger_version)
            .bind(&status.settled_at)
            .bind(&status.reversed_at)
            .bind(&status.failure)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.reference, s.invoice_id, s.bill_id, s.company_id, s.method, s.direction, s.amount_minor, s.currency, s.value_date, s.cancel_requested, t.phase, t.intent_at, t.provider_id, t.checkout_url, t.ledger_version, t.settled_at, t.reversed_at, t.failure \
             FROM payment_spec s JOIN payment_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        reference: row.try_get("reference")?,
                        invoice_id: row.try_get("invoice_id")?,
                        bill_id: row.try_get("bill_id")?,
                        company_id: row.try_get("company_id")?,
                        method: row.try_get("method")?,
                        direction: row.try_get("direction")?,
                        amount_minor: row.try_get("amount_minor")?,
                        currency: row.try_get("currency")?,
                        value_date: row.try_get("value_date")?,
                        cancel_requested: row.try_get("cancel_requested")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        intent_at: row.try_get("intent_at")?,
                        provider_id: row.try_get("provider_id")?,
                        checkout_url: row.try_get("checkout_url")?,
                        ledger_version: row.try_get("ledger_version")?,
                        settled_at: row.try_get("settled_at")?,
                        reversed_at: row.try_get("reversed_at")?,
                        failure: row.try_get("failure")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE payment_spec SET updated_at = clock_timestamp(), value_date = $2, cancel_requested = $3 WHERE id = $1")
            .bind(r.id)
            .bind(&spec.value_date)
            .bind(spec.cancel_requested)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE payment_status SET updated_at = clock_timestamp(), phase = $2, intent_at = $3, provider_id = $4, checkout_url = $5, ledger_version = $6, settled_at = $7, reversed_at = $8, failure = $9 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.intent_at)
            .bind(&status.provider_id)
            .bind(&status.checkout_url)
            .bind(status.ledger_version)
            .bind(&status.settled_at)
            .bind(&status.reversed_at)
            .bind(&status.failure)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

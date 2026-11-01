//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `supplier_bill_spec` / `supplier_bill_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO supplier_bill_spec (id, name, supplier_slug, supplier_invoice_number, lines, currency, net_minor, vat_minor, gross_minor, invoice_date, due_date, purchase_ref, bank_account, source_document_id, approved_by, voided) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)")
            .bind(r.id)
            .bind(&spec.name)
            .bind(&spec.supplier_slug)
            .bind(&spec.supplier_invoice_number)
            .bind(&spec.lines)
            .bind(&spec.currency)
            .bind(spec.net_minor)
            .bind(spec.vat_minor)
            .bind(spec.gross_minor)
            .bind(&spec.invoice_date)
            .bind(&spec.due_date)
            .bind(&spec.purchase_ref)
            .bind(&spec.bank_account)
            .bind(spec.source_document_id)
            .bind(&spec.approved_by)
            .bind(spec.voided)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO supplier_bill_status (id, phase, match_result, attention, ledger_version, approved_at, settled_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.match_result)
            .bind(&status.attention)
            .bind(status.ledger_version)
            .bind(&status.approved_at)
            .bind(&status.settled_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.name, s.supplier_slug, s.supplier_invoice_number, s.lines, s.currency, s.net_minor, s.vat_minor, s.gross_minor, s.invoice_date, s.due_date, s.purchase_ref, s.bank_account, s.source_document_id, s.approved_by, s.voided, t.phase, t.match_result, t.attention, t.ledger_version, t.approved_at, t.settled_at \
             FROM supplier_bill_spec s JOIN supplier_bill_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        supplier_slug: row.try_get("supplier_slug")?,
                        supplier_invoice_number: row.try_get("supplier_invoice_number")?,
                        lines: row.try_get("lines")?,
                        currency: row.try_get("currency")?,
                        net_minor: row.try_get("net_minor")?,
                        vat_minor: row.try_get("vat_minor")?,
                        gross_minor: row.try_get("gross_minor")?,
                        invoice_date: row.try_get("invoice_date")?,
                        due_date: row.try_get("due_date")?,
                        purchase_ref: row.try_get("purchase_ref")?,
                        bank_account: row.try_get("bank_account")?,
                        source_document_id: row.try_get("source_document_id")?,
                        approved_by: row.try_get("approved_by")?,
                        voided: row.try_get("voided")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        match_result: row.try_get("match_result")?,
                        attention: row.try_get("attention")?,
                        ledger_version: row.try_get("ledger_version")?,
                        approved_at: row.try_get("approved_at")?,
                        settled_at: row.try_get("settled_at")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // Immutable columns are omitted: the migration's trigger guards them.
        sqlx::query("UPDATE supplier_bill_spec SET updated_at = clock_timestamp(), approved_by = $2, voided = $3 WHERE id = $1")
            .bind(r.id)
            .bind(&spec.approved_by)
            .bind(spec.voided)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE supplier_bill_status SET updated_at = clock_timestamp(), phase = $2, match_result = $3, attention = $4, ledger_version = $5, approved_at = $6, settled_at = $7 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.match_result)
            .bind(&status.attention)
            .bind(status.ledger_version)
            .bind(&status.approved_at)
            .bind(&status.settled_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn finalize_delete(&self, _tx: &mut Tx<'_>, _obj: &Object<Spec, Status>) -> Result<(), sqlx::Error> {
        // Optional teardown evidence, run atomically with the envelope DELETE.
        Ok(())
    }
}

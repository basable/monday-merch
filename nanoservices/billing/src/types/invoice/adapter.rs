//! The adapter: a dumb column mapper between `Spec`/`Status` and the
//! `invoice_spec` / `invoice_status` tables. Five callbacks
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
        sqlx::query("INSERT INTO invoice_spec (id, name, kind, company_id, company_snapshot, lines, currency, net_minor, vat_minor, gross_minor, tax_point, terms_days, corrects_invoice_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)")
            .bind(r.id)
            .bind(&spec.name)
            .bind(&spec.kind)
            .bind(spec.company_id)
            .bind(&spec.company_snapshot)
            .bind(&spec.lines)
            .bind(&spec.currency)
            .bind(spec.net_minor)
            .bind(spec.vat_minor)
            .bind(spec.gross_minor)
            .bind(&spec.tax_point)
            .bind(spec.terms_days)
            .bind(spec.corrects_invoice_id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn insert_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO invoice_status (id, phase, number, pdf_key, ledger_version, issued_at, due_at, send_intent_at, sent_at, reminders_sent, reminder_intent_at, settled_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.number)
            .bind(&status.pdf_key)
            .bind(status.ledger_version)
            .bind(&status.issued_at)
            .bind(&status.due_at)
            .bind(&status.send_intent_at)
            .bind(&status.sent_at)
            .bind(status.reminders_sent)
            .bind(&status.reminder_intent_at)
            .bind(&status.settled_at)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read_rows(&self, tx: &mut Tx<'_>, ids: &[Uuid]) -> Result<Vec<Row<Spec, Status>>, sqlx::Error> {
        // Batched by contract: one query for the whole claim batch.
        let rows = sqlx::query(
            "SELECT s.id, s.name, s.kind, s.company_id, s.company_snapshot, s.lines, s.currency, s.net_minor, s.vat_minor, s.gross_minor, s.tax_point, s.terms_days, s.corrects_invoice_id, t.phase, t.number, t.pdf_key, t.ledger_version, t.issued_at, t.due_at, t.send_intent_at, t.sent_at, t.reminders_sent, t.reminder_intent_at, t.settled_at \
             FROM invoice_spec s JOIN invoice_status t ON t.id = s.id WHERE s.id = ANY($1)",
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
                        kind: row.try_get("kind")?,
                        company_id: row.try_get("company_id")?,
                        company_snapshot: row.try_get("company_snapshot")?,
                        lines: row.try_get("lines")?,
                        currency: row.try_get("currency")?,
                        net_minor: row.try_get("net_minor")?,
                        vat_minor: row.try_get("vat_minor")?,
                        gross_minor: row.try_get("gross_minor")?,
                        tax_point: row.try_get("tax_point")?,
                        terms_days: row.try_get("terms_days")?,
                        corrects_invoice_id: row.try_get("corrects_invoice_id")?,
                    },
                    status: Status {
                        phase: row.try_get("phase")?,
                        number: row.try_get("number")?,
                        pdf_key: row.try_get("pdf_key")?,
                        ledger_version: row.try_get("ledger_version")?,
                        issued_at: row.try_get("issued_at")?,
                        due_at: row.try_get("due_at")?,
                        send_intent_at: row.try_get("send_intent_at")?,
                        sent_at: row.try_get("sent_at")?,
                        reminders_sent: row.try_get("reminders_sent")?,
                        reminder_intent_at: row.try_get("reminder_intent_at")?,
                        settled_at: row.try_get("settled_at")?,
                    },
                })
            })
            .collect()
    }

    async fn write_spec(&self, tx: &mut Tx<'_>, r: &Ref, spec: &Spec) -> Result<(), sqlx::Error> {
        // No mutable spec columns yet: only the timestamp moves.
        sqlx::query("UPDATE invoice_spec SET updated_at = clock_timestamp() WHERE id = $1")
            .bind(r.id)
            .execute(tx)
            .await?;
        let _ = spec;
        Ok(())
    }

    async fn write_status(&self, tx: &mut Tx<'_>, r: &Ref, status: &Status) -> Result<(), sqlx::Error> {
        // The whole row, unconditionally: single provenance (invariant 3).
        sqlx::query("UPDATE invoice_status SET updated_at = clock_timestamp(), phase = $2, number = $3, pdf_key = $4, ledger_version = $5, issued_at = $6, due_at = $7, send_intent_at = $8, sent_at = $9, reminders_sent = $10, reminder_intent_at = $11, settled_at = $12 WHERE id = $1")
            .bind(r.id)
            .bind(&status.phase)
            .bind(&status.number)
            .bind(&status.pdf_key)
            .bind(status.ledger_version)
            .bind(&status.issued_at)
            .bind(&status.due_at)
            .bind(&status.send_intent_at)
            .bind(&status.sent_at)
            .bind(status.reminders_sent)
            .bind(&status.reminder_intent_at)
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

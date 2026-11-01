//! The ledger this nanoservice owns (the Directive §11): the engine over its
//! own schema with every object type registered. `accounts.rs` names the
//! batches; one module per object type holds its accounting. The snapshot
//! types are `messages::<Type>Snapshot`, shared with the producers.

pub mod accounts;
pub mod invoice;
pub mod credit_note;
pub mod payment;
pub mod vat_return;
pub mod supplier_bill;

use basable_ledger::Ledger;
use basable_ledger_postgres::PgStore;

/// The ledger over this nanoservice's pool.
pub type AppLedger = Ledger<PgStore>;

/// Builds the ledger and restates every process the version index lists
/// behind the running rules. A rules version changes only with a
/// deployment, so the restatement runs once, here, before the component
/// serves; a desired state that no longer balances under the new rules
/// stops the boot, which is the right time to find out.
pub async fn build(pool: &sqlx::PgPool) -> AppLedger {
    let ledger = Ledger::builder(PgStore::new(pool.clone()))
        .register::<messages::InvoiceSnapshot, invoice::InvoiceAccounting>()
        .register::<messages::CreditNoteSnapshot, credit_note::CreditNoteAccounting>()
        .register::<messages::PaymentSnapshot, payment::PaymentAccounting>()
        .register::<messages::VatReturnSnapshot, vat_return::VatReturnAccounting>()
        .register::<messages::SupplierBillSnapshot, supplier_bill::SupplierBillAccounting>()
        .build();
    let report = ledger
        .restate_behind()
        .await
        .expect("billing: the startup restatement of the ledger");
    if report.processes > 0 {
        tracing::info!(
            processes = report.processes,
            objects = report.objects,
            lines = report.lines,
            "ledger restated under the running rules"
        );
    }
    ledger
}

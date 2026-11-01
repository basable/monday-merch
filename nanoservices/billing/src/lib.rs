//! `billing`: Owns the ledger, invoices and credit notes, payments and VAT returns.
//!
//! Read `AGENTS.md` in this directory and `docs/DIRECTIVE.md` before editing.
//! What this nanoservice owns is what it is: 4 processing-object
//! type(s), 4 plain table(s), 4 external call(s), and
//! the project's ledger (5 object type(s)).

// A rendered skeleton has stubs nothing calls yet; the allow goes once the
// bodies are filled.
#![allow(dead_code)]

use basable_core::AppError;
use basable_db::{NanoPool, Nanoservice, Stateful};

pub mod handlers;
pub mod effects;
pub mod provider;
pub mod simulator;
pub mod model;
pub mod repository;
pub mod config;
pub mod types;
pub mod ledger;

/// The schema marker: this nanoservice owns `nano_billing` and gets a
/// pool bound to it (`SET ROLE nano_billing`). A cross-schema query
/// from that pool is a Postgres permission error.
pub struct Schema;
impl Nanoservice for Schema {
    const NAME: &'static str = "billing";
}
impl Stateful for Schema {}

/// The component. One value per process, shared by every handler and worker
/// (`&self` everywhere; in-memory state, if any, behind `std::sync::Mutex`
/// or atomics and never held across an `.await`).
pub struct Billing {
    pub(crate) pool: NanoPool<Schema>,
    pub(crate) ledger: ledger::AppLedger,
    pub(crate) calls: effects::Calls,
    /// The catalogs this nanoservice owns, read per request (`config.rs`).
    pub(crate) config: basable_config::Repository,
    pub(crate) invoice_store: types::invoice::Store,
    pub(crate) payment_store: types::payment::Store,
    pub(crate) vat_return_store: types::vat_return::Store,
    pub(crate) supplier_bill_store: types::supplier_bill::Store,
}

impl Billing {
    /// Builds the component. Called once from `app/src/main.rs`.
    /// Binding a type's store checks its migration ran; a missing one is a
    /// boot failure, which is the right time to find out.
    /// Building the ledger restates every process behind the running rules
    /// before anything serves (`ledger::build`).
    /// `config` is the repository main built over every registered config
    /// type, the seed applied.
    pub async fn new(_app: &basable_app::App, pool: NanoPool<Schema>, config: basable_config::Repository) -> Self {
        Self {
            pool: pool.clone(),
            ledger: ledger::build(pool.pool()).await,
            // Production wires the real provider; tests wire the simulator.
            calls: effects::Calls::from_env(),
            config,
            invoice_store: types::invoice::Store::bind(&pool, types::invoice::decl())
                .await
                .expect("billing: bind type invoice (the migration must have run)"),
            payment_store: types::payment::Store::bind(&pool, types::payment::decl())
                .await
                .expect("billing: bind type payment (the migration must have run)"),
            vat_return_store: types::vat_return::Store::bind(&pool, types::vat_return::decl())
                .await
                .expect("billing: bind type vat_return (the migration must have run)"),
            supplier_bill_store: types::supplier_bill::Store::bind(&pool, types::supplier_bill::decl())
                .await
                .expect("billing: bind type supplier_bill (the migration must have run)"),
        }
    }
}

impl<R> basable_app::Component<R> for Billing
where
    R: interfaces::BillingRoutes + Send + Sync + 'static,
{
    /// Every loop this nanoservice runs: a worker per processing-object
    /// type, then its schedules. The app starts every component's loops
    /// (`app/src/main.rs`: `.components(router)`) and joins them on
    /// shutdown, so a type or a schedule this nanoservice gains is added
    /// here and nowhere else.
    fn loops(&'static self, router: &'static R) -> basable_app::Loops {
        basable_app::Loops::new()
            .worker(types::invoice::worker(router, self))
            .worker(types::payment::worker(router, self))
            .worker(types::vat_return::worker(router, self))
            .worker(types::supplier_bill::worker(router, self))
    }
}

/// The error a step not filled in yet answers with, so the skeleton deploys
/// green instead of panicking. `regex_search unimplemented_step` lists what
/// is left.
pub(crate) fn unimplemented_step(step: &'static str) -> AppError {
    tracing::warn!(step, "unimplemented step reached");
    AppError::unimplemented(step)
}

//! `order`: Takes a request from quote to signature, then drives each order through invoicing, production and delivery.
//!
//! Read `AGENTS.md` in this directory and `docs/DIRECTIVE.md` before editing.
//! What this nanoservice owns is what it is: 3 processing-object
//! type(s), 5 plain table(s), 0 external call(s).

// A rendered skeleton has stubs nothing calls yet; the allow goes once the
// bodies are filled.
#![allow(dead_code)]

use basable_core::AppError;
use basable_db::{NanoPool, Nanoservice, Stateful};

pub mod handlers;
pub mod model;
pub mod repository;
pub mod worker;
pub mod config;
pub mod types;

/// The schema marker: this nanoservice owns `nano_order` and gets a
/// pool bound to it (`SET ROLE nano_order`). A cross-schema query
/// from that pool is a Postgres permission error.
pub struct Schema;
impl Nanoservice for Schema {
    const NAME: &'static str = "order";
}
impl Stateful for Schema {}

/// The component. One value per process, shared by every handler and worker
/// (`&self` everywhere; in-memory state, if any, behind `std::sync::Mutex`
/// or atomics and never held across an `.await`).
pub struct Order {
    pub(crate) pool: NanoPool<Schema>,
    /// The catalogs this nanoservice owns, read per request (`config.rs`).
    pub(crate) config: basable_config::Repository,
    pub(crate) quote_store: types::quote::Store,
    pub(crate) order_store: types::order::Store,
    pub(crate) production_run_store: types::production_run::Store,
}

impl Order {
    /// Builds the component. Called once from `app/src/main.rs`.
    /// Binding a type's store checks its migration ran; a missing one is a
    /// boot failure, which is the right time to find out.
    /// `config` is the repository main built over every registered config
    /// type, the seed applied.
    pub async fn new(_app: &basable_app::App, pool: NanoPool<Schema>, config: basable_config::Repository) -> Self {
        Self {
            pool: pool.clone(),
            config,
            quote_store: types::quote::Store::bind(&pool, types::quote::decl())
                .await
                .expect("order: bind type quote (the migration must have run)"),
            order_store: types::order::Store::bind(&pool, types::order::decl())
                .await
                .expect("order: bind type order (the migration must have run)"),
            production_run_store: types::production_run::Store::bind(&pool, types::production_run::decl())
                .await
                .expect("order: bind type production_run (the migration must have run)"),
        }
    }
}

impl<R> basable_app::Component<R> for Order
where
    R: interfaces::OrderRoutes + Send + Sync + 'static,
{
    /// Every loop this nanoservice runs: a worker per processing-object
    /// type, then its schedules. The app starts every component's loops
    /// (`app/src/main.rs`: `.components(router)`) and joins them on
    /// shutdown, so a type or a schedule this nanoservice gains is added
    /// here and nowhere else.
    fn loops(&'static self, router: &'static R) -> basable_app::Loops {
        let sender = interfaces::OrderSender::new(router);
        basable_app::Loops::new()
            .worker(types::quote::worker(router, self))
            .worker(types::order::worker(router, self))
            .worker(types::production_run::worker(router, self))
            .ticker(basable_app::Ticker::new("purge_leads", std::time::Duration::from_secs(86400), move |ctx| Box::pin(self.tick_purge_leads(ctx, sender))))
    }
}

/// The error a step not filled in yet answers with, so the skeleton deploys
/// green instead of panicking. `regex_search unimplemented_step` lists what
/// is left.
pub(crate) fn unimplemented_step(step: &'static str) -> AppError {
    tracing::warn!(step, "unimplemented step reached");
    AppError::unimplemented(step)
}

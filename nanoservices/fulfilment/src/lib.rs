//! `fulfilment`: Owns held stock, kits and inbound goods, and drives each shipment from reservation to delivery.
//!
//! Read `AGENTS.md` in this directory and `docs/DIRECTIVE.md` before editing.
//! What this nanoservice owns is what it is: 1 processing-object
//! type(s), 10 plain table(s), 2 external call(s).

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
pub mod worker;
pub mod config;
pub mod types;

/// The schema marker: this nanoservice owns `nano_fulfilment` and gets a
/// pool bound to it (`SET ROLE nano_fulfilment`). A cross-schema query
/// from that pool is a Postgres permission error.
pub struct Schema;
impl Nanoservice for Schema {
    const NAME: &'static str = "fulfilment";
}
impl Stateful for Schema {}

/// The component. One value per process, shared by every handler and worker
/// (`&self` everywhere; in-memory state, if any, behind `std::sync::Mutex`
/// or atomics and never held across an `.await`).
pub struct Fulfilment {
    pub(crate) pool: NanoPool<Schema>,
    pub(crate) calls: effects::Calls,
    /// The catalogs this nanoservice owns, read per request (`config.rs`).
    pub(crate) config: basable_config::Repository,
    pub(crate) shipment_store: types::shipment::Store,
}

impl Fulfilment {
    /// Builds the component. Called once from `app/src/main.rs`.
    /// Binding a type's store checks its migration ran; a missing one is a
    /// boot failure, which is the right time to find out.
    /// `config` is the repository main built over every registered config
    /// type, the seed applied.
    pub async fn new(_app: &basable_app::App, pool: NanoPool<Schema>, config: basable_config::Repository) -> Self {
        Self {
            pool: pool.clone(),
            // Production wires the real provider; tests wire the simulator.
            calls: effects::Calls::from_env(),
            config,
            shipment_store: types::shipment::Store::bind(&pool, types::shipment::decl())
                .await
                .expect("fulfilment: bind type shipment (the migration must have run)"),
        }
    }
}

impl<R> basable_app::Component<R> for Fulfilment
where
    R: interfaces::FulfilmentRoutes + Send + Sync + 'static,
{
    /// Every loop this nanoservice runs: a worker per processing-object
    /// type, then its schedules. The app starts every component's loops
    /// (`app/src/main.rs`: `.components(router)`) and joins them on
    /// shutdown, so a type or a schedule this nanoservice gains is added
    /// here and nowhere else.
    fn loops(&'static self, router: &'static R) -> basable_app::Loops {
        let sender = interfaces::FulfilmentSender::new(router);
        basable_app::Loops::new()
            .worker(types::shipment::worker(router, self))
            .ticker(basable_app::Ticker::new("check_low_stock", std::time::Duration::from_secs(3600), move |ctx| Box::pin(self.tick_check_low_stock(ctx, sender))))
            .ticker(basable_app::Ticker::new("purge_shipment_addresses", std::time::Duration::from_secs(86400), move |ctx| Box::pin(self.tick_purge_shipment_addresses(ctx, sender))))
    }
}

/// The error a step not filled in yet answers with, so the skeleton deploys
/// green instead of panicking. `regex_search unimplemented_step` lists what
/// is left.
pub(crate) fn unimplemented_step(step: &'static str) -> AppError {
    tracing::warn!(step, "unimplemented step reached");
    AppError::unimplemented(step)
}

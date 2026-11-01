//! Schedules are WORKERS this nanoservice owns: a ticker in its `loops()`
//! (lib.rs), which the app starts and joins on shutdown — immediate first
//! tick then one per interval, failures logged and retried next tick.
//! Never a CronJob. A tick that needs claim authority creates or nudges a
//! processing object; it never writes status itself.

use basable_core::{BoxError, Ctx};

use crate::Fulfilment;

impl Fulfilment {
    /// `check_low_stock`, every 1h. `_s` is this nanoservice's sender,
    /// for a tick that sends.
    pub async fn tick_check_low_stock<R>(&self, _ctx: &Ctx, _s: interfaces::FulfilmentSender<'static, R>) -> Result<(), BoxError>
    where
        R: interfaces::FulfilmentRoutes + Send + Sync + 'static,
    {
        Err(crate::unimplemented_step("fulfilment.tick_check_low_stock").into())
    }
    /// `purge_shipment_addresses`, every 24h. `_s` is this nanoservice's sender,
    /// for a tick that sends.
    pub async fn tick_purge_shipment_addresses<R>(&self, _ctx: &Ctx, _s: interfaces::FulfilmentSender<'static, R>) -> Result<(), BoxError>
    where
        R: interfaces::FulfilmentRoutes + Send + Sync + 'static,
    {
        Err(crate::unimplemented_step("fulfilment.tick_purge_shipment_addresses").into())
    }
}

//! Schedules are WORKERS this nanoservice owns: a ticker in its `loops()`
//! (lib.rs), which the app starts and joins on shutdown — immediate first
//! tick then one per interval, failures logged and retried next tick.
//! Never a CronJob. A tick that needs claim authority creates or nudges a
//! processing object; it never writes status itself.

use basable_core::{BoxError, Ctx};

use crate::Inbox;

impl Inbox {
    /// `purge_inbound`, every 24h. `_s` is this nanoservice's sender,
    /// for a tick that sends.
    pub async fn tick_purge_inbound<R>(&self, _ctx: &Ctx, _s: interfaces::InboxSender<'static, R>) -> Result<(), BoxError>
    where
        R: interfaces::InboxRoutes + Send + Sync + 'static,
    {
        Err(crate::unimplemented_step("inbox.tick_purge_inbound").into())
    }
}

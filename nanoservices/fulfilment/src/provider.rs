//! The provider seam: the narrow trait `effects.rs` sends through, the real
//! HTTP client, and the noop for environments without credentials. A
//! provider answers with a `BoxError` that carries its own verdict on its
//! own call: `definitive(..)` for a parsed 4xx — the receiver's structured
//! answer, and the proof of non-execution an irreversible adapter needs —
//! and `TransportError::wrap(..)` for everything whose outcome is
//! unknowable (a transport error, a 5xx, a timeout), which every classifier
//! holds ambiguous. A plain error is neither: the reversible classifier
//! (`Classifier::transport`) takes it as the receiver's answer, the
//! irreversible one (`Classifier::fail_closed_on_definitive`) holds it
//! ambiguous, so never return one for an outcome you cannot know.

use std::future::Future;

use basable_core::{BoxError, Ctx};
use basable_externaleffect::definitive;

use crate::effects::*;

/// Exactly the calls this nanoservice makes — never a wrapper around the
/// whole provider API. The futures are `Send` because a worker dispatches
/// them.
pub trait Provider: Send + Sync + 'static {
    fn create_label(&self, ctx: Ctx, args: CreateLabelArgs) -> impl Future<Output = Result<CreateLabelResult, BoxError>> + Send;
    /// Looks `create_label` up by its key: what the receiver holds, if anything.
    fn lookup_create_label(&self, ctx: Ctx, args: CreateLabelArgs) -> impl Future<Output = Result<Option<CreateLabelResult>, BoxError>> + Send;
    fn void_label(&self, ctx: Ctx, args: VoidLabelArgs) -> impl Future<Output = Result<VoidLabelResult, BoxError>> + Send;
}

/// The HTTP JSON client for carrier.
pub struct HttpProvider {
    pub base_url: String,
    pub client: reqwest::Client,
    pub token: String,
}

impl Provider for HttpProvider {
    async fn create_label(&self, _ctx: Ctx, _args: CreateLabelArgs) -> Result<CreateLabelResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("fulfilment.create_label: provider not implemented".into())
    }

    async fn lookup_create_label(&self, _ctx: Ctx, _args: CreateLabelArgs) -> Result<Option<CreateLabelResult>, BoxError> {
        Err("fulfilment.lookup_create_label: provider not implemented".into())
    }
    async fn void_label(&self, _ctx: Ctx, _args: VoidLabelArgs) -> Result<VoidLabelResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("fulfilment.void_label: provider not implemented".into())
    }
}

/// No credentials configured (local dev): every call is refused, so nothing
/// is sent and nothing is ambiguous.
pub struct NoopProvider;

impl Provider for NoopProvider {
    async fn create_label(&self, _ctx: Ctx, _args: CreateLabelArgs) -> Result<CreateLabelResult, BoxError> {
        Err(definitive("fulfilment.create_label: provider not configured"))
    }

    async fn lookup_create_label(&self, _ctx: Ctx, _args: CreateLabelArgs) -> Result<Option<CreateLabelResult>, BoxError> {
        Ok(None)
    }
    async fn void_label(&self, _ctx: Ctx, _args: VoidLabelArgs) -> Result<VoidLabelResult, BoxError> {
        Err(definitive("fulfilment.void_label: provider not configured"))
    }
}

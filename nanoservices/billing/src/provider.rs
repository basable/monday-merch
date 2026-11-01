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
    fn create_checkout_session(&self, ctx: Ctx, args: CreateCheckoutSessionArgs) -> impl Future<Output = Result<CreateCheckoutSessionResult, BoxError>> + Send;
    /// Reads `create_checkout_session` back by the provider id its answer carried.
    fn resolve_create_checkout_session(&self, ctx: Ctx, provider_id: String) -> impl Future<Output = Result<CreateCheckoutSessionResult, BoxError>> + Send;
    fn expire_checkout_session(&self, ctx: Ctx, args: ExpireCheckoutSessionArgs) -> impl Future<Output = Result<ExpireCheckoutSessionResult, BoxError>> + Send;
    fn create_refund(&self, ctx: Ctx, args: CreateRefundArgs) -> impl Future<Output = Result<CreateRefundResult, BoxError>> + Send;
    /// Reads `create_refund` back by the provider id its answer carried.
    fn resolve_create_refund(&self, ctx: Ctx, provider_id: String) -> impl Future<Output = Result<CreateRefundResult, BoxError>> + Send;
    fn store_document(&self, ctx: Ctx, args: StoreDocumentArgs) -> impl Future<Output = Result<StoreDocumentResult, BoxError>> + Send;
}

/// The HTTP JSON client for payments, object_storage.
pub struct HttpProvider {
    pub base_url: String,
    pub client: reqwest::Client,
    pub token: String,
}

impl Provider for HttpProvider {
    async fn create_checkout_session(&self, _ctx: Ctx, _args: CreateCheckoutSessionArgs) -> Result<CreateCheckoutSessionResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("billing.create_checkout_session: provider not implemented".into())
    }

    async fn resolve_create_checkout_session(&self, _ctx: Ctx, _provider_id: String) -> Result<CreateCheckoutSessionResult, BoxError> {
        Err("billing.resolve_create_checkout_session: provider not implemented".into())
    }
    async fn expire_checkout_session(&self, _ctx: Ctx, _args: ExpireCheckoutSessionArgs) -> Result<ExpireCheckoutSessionResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("billing.expire_checkout_session: provider not implemented".into())
    }
    async fn create_refund(&self, _ctx: Ctx, _args: CreateRefundArgs) -> Result<CreateRefundResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("billing.create_refund: provider not implemented".into())
    }

    async fn resolve_create_refund(&self, _ctx: Ctx, _provider_id: String) -> Result<CreateRefundResult, BoxError> {
        Err("billing.resolve_create_refund: provider not implemented".into())
    }
    async fn store_document(&self, _ctx: Ctx, _args: StoreDocumentArgs) -> Result<StoreDocumentResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("billing.store_document: provider not implemented".into())
    }
}

/// No credentials configured (local dev): every call is refused, so nothing
/// is sent and nothing is ambiguous.
pub struct NoopProvider;

impl Provider for NoopProvider {
    async fn create_checkout_session(&self, _ctx: Ctx, _args: CreateCheckoutSessionArgs) -> Result<CreateCheckoutSessionResult, BoxError> {
        Err(definitive("billing.create_checkout_session: provider not configured"))
    }

    async fn resolve_create_checkout_session(&self, _ctx: Ctx, _provider_id: String) -> Result<CreateCheckoutSessionResult, BoxError> {
        Err(definitive("billing.resolve_create_checkout_session: provider not configured"))
    }
    async fn expire_checkout_session(&self, _ctx: Ctx, _args: ExpireCheckoutSessionArgs) -> Result<ExpireCheckoutSessionResult, BoxError> {
        Err(definitive("billing.expire_checkout_session: provider not configured"))
    }
    async fn create_refund(&self, _ctx: Ctx, _args: CreateRefundArgs) -> Result<CreateRefundResult, BoxError> {
        Err(definitive("billing.create_refund: provider not configured"))
    }

    async fn resolve_create_refund(&self, _ctx: Ctx, _provider_id: String) -> Result<CreateRefundResult, BoxError> {
        Err(definitive("billing.resolve_create_refund: provider not configured"))
    }
    async fn store_document(&self, _ctx: Ctx, _args: StoreDocumentArgs) -> Result<StoreDocumentResult, BoxError> {
        Err(definitive("billing.store_document: provider not configured"))
    }
}

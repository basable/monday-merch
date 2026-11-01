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
    fn store_inbound_object(&self, ctx: Ctx, args: StoreInboundObjectArgs) -> impl Future<Output = Result<StoreInboundObjectResult, BoxError>> + Send;
    fn delete_inbound_object(&self, ctx: Ctx, args: DeleteInboundObjectArgs) -> impl Future<Output = Result<DeleteInboundObjectResult, BoxError>> + Send;
    fn extract_document(&self, ctx: Ctx, args: ExtractDocumentArgs) -> impl Future<Output = Result<ExtractDocumentResult, BoxError>> + Send;
}

/// The HTTP JSON client for object_storage, llm.
pub struct HttpProvider {
    pub base_url: String,
    pub client: reqwest::Client,
    pub token: String,
}

impl Provider for HttpProvider {
    async fn store_inbound_object(&self, _ctx: Ctx, _args: StoreInboundObjectArgs) -> Result<StoreInboundObjectResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("inbox.store_inbound_object: provider not implemented".into())
    }
    async fn delete_inbound_object(&self, _ctx: Ctx, _args: DeleteInboundObjectArgs) -> Result<DeleteInboundObjectResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("inbox.delete_inbound_object: provider not implemented".into())
    }
    async fn extract_document(&self, _ctx: Ctx, _args: ExtractDocumentArgs) -> Result<ExtractDocumentResult, BoxError> {
        // TODO: the request. A parsed 4xx is `Err(definitive(..))`; a
        // transport error, a 5xx or a timeout is
        // `Err(TransportError::wrap(..))`.
        Err("inbox.extract_document: provider not implemented".into())
    }
}

/// No credentials configured (local dev): every call is refused, so nothing
/// is sent and nothing is ambiguous.
pub struct NoopProvider;

impl Provider for NoopProvider {
    async fn store_inbound_object(&self, _ctx: Ctx, _args: StoreInboundObjectArgs) -> Result<StoreInboundObjectResult, BoxError> {
        Err(definitive("inbox.store_inbound_object: provider not configured"))
    }
    async fn delete_inbound_object(&self, _ctx: Ctx, _args: DeleteInboundObjectArgs) -> Result<DeleteInboundObjectResult, BoxError> {
        Err(definitive("inbox.delete_inbound_object: provider not configured"))
    }
    async fn extract_document(&self, _ctx: Ctx, _args: ExtractDocumentArgs) -> Result<ExtractDocumentResult, BoxError> {
        Err(definitive("inbox.extract_document: provider not configured"))
    }
}

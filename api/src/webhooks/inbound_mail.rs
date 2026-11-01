//! The inbound `inbound_mail` webhook, owned by `inbox`: verify the
//! delivery the way `inbound_mail` lets a receiver verify it, then ONE messenger
//! send to the owning nanoservice. No processing here; the nanoservice
//! decides what the event means (a side table plus a nudge, never a status
//! write). The route is a raw HTTP route beside the Connect services on
//! purpose: a provider posts its own body with its own headers, and the
//! raw bytes are what a signature covers.

use axum::{
    body::Bytes,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};

use crate::Api;

/// `POST /api/webhooks/inbound_mail`.
pub fn route<R>(_api: &'static Api, router: &'static R) -> Router
where
    R: interfaces::ApiRoutes + Send + Sync + 'static,
{
    let sender = interfaces::ApiSender::new(router);
    Router::new().route(
        "/api/webhooks/inbound_mail",
        post(move |headers: HeaderMap, body: Bytes| handle(sender, headers, body)),
    )
}

async fn handle<R>(_sender: interfaces::ApiSender<'static, R>, headers: HeaderMap, body: Bytes) -> StatusCode
where
    R: interfaces::ApiRoutes + Send + Sync + 'static,
{
    if !verify(&headers, &body) {
        return StatusCode::UNAUTHORIZED;
    }
    // TODO: parse the event and send it — `_sender.send_<message>(&ctx, m).await`.
    tracing::warn!(step = "api.webhooks.inbound_mail", "unimplemented step reached");
    StatusCode::ACCEPTED
}

/// TODO: `inbound_mail`'s verification. Implement the scheme the provider
/// documents for receivers — a signature over the raw body with a shared
/// secret, a token in a header, a certificate or public key it publishes,
/// a lookup against its API — and return true only for a delivery you have
/// verified. A secret the scheme needs is a user-supplied secret like any
/// other: read it once at mount from the environment, wired in
/// `k8s/app/deployment.yaml` with its own non-optional `secretKeyRef` on
/// `app-secrets` (docs/operations.md), never from the repository. Until
/// this is implemented every delivery is refused, on purpose: a receiver
/// that accepted unverified deliveries would let anyone on the internet
/// inject events into the messenger.
fn verify(_headers: &HeaderMap, _body: &[u8]) -> bool {
    false
}

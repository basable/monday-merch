//! Inbound webhooks: raw axum routes beside the Connect router. Each module
//! verifies its provider's signature, then makes ONE messenger send to the
//! owning nanoservice. The scaffolder adds `pub mod <provider>;` per declared
//! webhook between the markers.

use axum::Router;

use crate::Api;

// basable:webhooks-begin
pub mod carrier;
pub mod payments;
pub mod inbound_mail;
// basable:webhooks-end

/// Every webhook route, mounted under `/api/webhooks/<provider>`.
pub fn routes<R>(_api: &'static Api, _router: &'static R) -> Router
where
    R: interfaces::ApiRoutes + Send + Sync + 'static,
{
    let router = Router::new();
    // basable:webhook-routes-begin
    let router = router.merge(carrier::route(_api, _router));
    let router = router.merge(payments::route(_api, _router));
    let router = router.merge(inbound_mail::route(_api, _router));
    // basable:webhook-routes-end
    router
}

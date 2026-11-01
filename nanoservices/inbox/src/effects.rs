//! The external-effect adapters: one validated `Call` per external call
//! this nanoservice makes, with the strategy the plan declared (the
//! Directive §6). `send` is the ONE dispatch path; a raw provider call
//! anywhere else fails review. Audited one by one in tests/effects_audit.rs.

use std::sync::Arc;
use std::time::Duration;

#[allow(unused_imports)]
use basable_externaleffect::{
    Adapter, Call, Classifier, DeclaredResolution, LateCall, SlotIdentity, Strategy,
    intent_age_fn, key_fn, lookup_fn, provider_id_fn, resolve_by_lookup, resolve_keyed_fn,
    send_fn,
};

use crate::provider::{HttpProvider, NoopProvider, Provider};

/// The validated adapter set, built once at construction. Each call is
/// shared (`Arc`) so the audit can hold the very handle production
/// dispatches through.
pub struct Calls {
    pub store_inbound_object: Arc<Call<StoreInboundObjectArgs, StoreInboundObjectArgs, StoreInboundObjectResult>>,
    pub delete_inbound_object: Arc<Call<DeleteInboundObjectArgs, DeleteInboundObjectArgs, DeleteInboundObjectResult>>,
    pub extract_document: Arc<Call<ExtractDocumentArgs, ExtractDocumentArgs, ExtractDocumentResult>>,
}

/// Arguments of `store_inbound_object` (object_storage). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct StoreInboundObjectArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `store_inbound_object` returns.
#[derive(Debug, Clone)]
pub struct StoreInboundObjectResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `delete_inbound_object` (object_storage). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct DeleteInboundObjectArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `delete_inbound_object` returns.
#[derive(Debug, Clone)]
pub struct DeleteInboundObjectResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `extract_document` (llm). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct ExtractDocumentArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `extract_document` returns.
#[derive(Debug, Clone)]
pub struct ExtractDocumentResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

impl Calls {
    /// Picks the provider from the environment, once, at construction: the
    /// HTTP client when the credential is set, the noop otherwise.
    pub fn from_env() -> Self {
        match std::env::var("INBOX_PROVIDER_TOKEN") {
            Ok(token) if !token.is_empty() => Calls::new(Arc::new(HttpProvider {
                base_url: std::env::var("INBOX_PROVIDER_URL").unwrap_or_default(),
                client: reqwest::Client::new(),
                token,
            })),
            _ => Calls::new(Arc::new(NoopProvider)),
        }
    }

    /// Builds every adapter over one provider; the tests pass the simulator.
    pub fn new<P: Provider>(provider: Arc<P>) -> Self {
        Self {
            store_inbound_object: Arc::new(
                Call::new(Adapter {
                    operation: "store_inbound_object".into(),
                    key: Some(key_fn(|a: &StoreInboundObjectArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: StoreInboundObjectArgs| {
                            let p = Arc::clone(&p);
                            async move { p.store_inbound_object(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("inbox.store_inbound_object: invalid adapter"),
            ),
            delete_inbound_object: Arc::new(
                Call::new(Adapter {
                    operation: "delete_inbound_object".into(),
                    key: Some(key_fn(|a: &DeleteInboundObjectArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: DeleteInboundObjectArgs| {
                            let p = Arc::clone(&p);
                            async move { p.delete_inbound_object(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("inbox.delete_inbound_object: invalid adapter"),
            ),
            extract_document: Arc::new(
                Call::new(Adapter {
                    operation: "extract_document".into(),
                    key: Some(key_fn(|a: &ExtractDocumentArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: ExtractDocumentArgs| {
                            let p = Arc::clone(&p);
                            async move { p.extract_document(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("inbox.extract_document: invalid adapter"),
            ),
        }
    }
}

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
    pub create_checkout_session: Arc<Call<CreateCheckoutSessionArgs, CreateCheckoutSessionArgs, CreateCheckoutSessionResult>>,
    pub expire_checkout_session: Arc<Call<ExpireCheckoutSessionArgs, ExpireCheckoutSessionArgs, ExpireCheckoutSessionResult>>,
    pub create_refund: Arc<Call<CreateRefundArgs, CreateRefundArgs, CreateRefundResult>>,
    pub store_document: Arc<Call<StoreDocumentArgs, StoreDocumentArgs, StoreDocumentResult>>,
}

/// Arguments of `create_checkout_session` (payments). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    /// The database-clock time the intent was declared, for the replay
    /// window gate.
    pub intent_declared_at: chrono::DateTime<chrono::Utc>,
    // TODO: the payload.
}

impl CreateCheckoutSessionArgs {
    pub fn intent_age(&self) -> Duration {
        (chrono::Utc::now() - self.intent_declared_at).to_std().unwrap_or_default()
    }
}

/// What `create_checkout_session` returns.
#[derive(Debug, Clone)]
pub struct CreateCheckoutSessionResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `expire_checkout_session` (payments). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct ExpireCheckoutSessionArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `expire_checkout_session` returns.
#[derive(Debug, Clone)]
pub struct ExpireCheckoutSessionResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `create_refund` (payments). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct CreateRefundArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    /// The database-clock time the intent was declared, for the replay
    /// window gate.
    pub intent_declared_at: chrono::DateTime<chrono::Utc>,
    // TODO: the payload.
}

impl CreateRefundArgs {
    pub fn intent_age(&self) -> Duration {
        (chrono::Utc::now() - self.intent_declared_at).to_std().unwrap_or_default()
    }
}

/// What `create_refund` returns.
#[derive(Debug, Clone)]
pub struct CreateRefundResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `store_document` (object_storage). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct StoreDocumentArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `store_document` returns.
#[derive(Debug, Clone)]
pub struct StoreDocumentResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

impl Calls {
    /// Picks the provider from the environment, once, at construction: the
    /// HTTP client when the credential is set, the noop otherwise.
    pub fn from_env() -> Self {
        match std::env::var("BILLING_PROVIDER_TOKEN") {
            Ok(token) if !token.is_empty() => Calls::new(Arc::new(HttpProvider {
                base_url: std::env::var("BILLING_PROVIDER_URL").unwrap_or_default(),
                client: reqwest::Client::new(),
                token,
            })),
            _ => Calls::new(Arc::new(NoopProvider)),
        }
    }

    /// Builds every adapter over one provider; the tests pass the simulator.
    pub fn new<P: Provider>(provider: Arc<P>) -> Self {
        Self {
            create_checkout_session: Arc::new(
                Call::new(Adapter {
                    operation: "create_checkout_session".into(),
                    key: Some(key_fn(|a: &CreateCheckoutSessionArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: CreateCheckoutSessionArgs| {
                            let p = Arc::clone(&p);
                            async move { p.create_checkout_session(ctx, args).await }
                        })
                    },
                    // Irreversible: only the provider's definitive answer (a
                    // parsed 4xx) is a verdict; anything else is ambiguous
                    // and re-driven.
                    classify: Some(Classifier::fail_closed_on_definitive()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: true,
                    late_call: LateCall::KeyScoped,
                    strategy: Strategy::KeyedReplay {
                        // The provider's replay horizon, from the manifest
                        // (replayWindow): past it dispatch refuses to send.
                        window: std::time::Duration::from_secs(86400),
                        intent_age: intent_age_fn(|a: &CreateCheckoutSessionArgs| a.intent_age()),
                        provider_id: provider_id_fn(|r: &CreateCheckoutSessionResult| r.provider_id.clone()),
                        resolve: {
                            let p = Arc::clone(&provider);
                            resolve_keyed_fn(move |ctx, id: String| {
                                let p = Arc::clone(&p);
                                async move { p.resolve_create_checkout_session(ctx, id).await }
                            })
                        },
                    },
                })
                .expect("billing.create_checkout_session: invalid adapter"),
            ),
            expire_checkout_session: Arc::new(
                Call::new(Adapter {
                    operation: "expire_checkout_session".into(),
                    key: Some(key_fn(|a: &ExpireCheckoutSessionArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: ExpireCheckoutSessionArgs| {
                            let p = Arc::clone(&p);
                            async move { p.expire_checkout_session(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("billing.expire_checkout_session: invalid adapter"),
            ),
            create_refund: Arc::new(
                Call::new(Adapter {
                    operation: "create_refund".into(),
                    key: Some(key_fn(|a: &CreateRefundArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: CreateRefundArgs| {
                            let p = Arc::clone(&p);
                            async move { p.create_refund(ctx, args).await }
                        })
                    },
                    // Irreversible: only the provider's definitive answer (a
                    // parsed 4xx) is a verdict; anything else is ambiguous
                    // and re-driven.
                    classify: Some(Classifier::fail_closed_on_definitive()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: true,
                    late_call: LateCall::KeyScoped,
                    strategy: Strategy::KeyedReplay {
                        // The provider's replay horizon, from the manifest
                        // (replayWindow): past it dispatch refuses to send.
                        window: std::time::Duration::from_secs(86400),
                        intent_age: intent_age_fn(|a: &CreateRefundArgs| a.intent_age()),
                        provider_id: provider_id_fn(|r: &CreateRefundResult| r.provider_id.clone()),
                        resolve: {
                            let p = Arc::clone(&provider);
                            resolve_keyed_fn(move |ctx, id: String| {
                                let p = Arc::clone(&p);
                                async move { p.resolve_create_refund(ctx, id).await }
                            })
                        },
                    },
                })
                .expect("billing.create_refund: invalid adapter"),
            ),
            store_document: Arc::new(
                Call::new(Adapter {
                    operation: "store_document".into(),
                    key: Some(key_fn(|a: &StoreDocumentArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: StoreDocumentArgs| {
                            let p = Arc::clone(&p);
                            async move { p.store_document(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("billing.store_document: invalid adapter"),
            ),
        }
    }
}

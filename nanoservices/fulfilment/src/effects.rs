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
    pub create_label: Arc<Call<CreateLabelArgs, CreateLabelArgs, CreateLabelResult>>,
    pub void_label: Arc<Call<VoidLabelArgs, VoidLabelArgs, VoidLabelResult>>,
}

/// Arguments of `create_label` (carrier). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct CreateLabelArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `create_label` returns.
#[derive(Debug, Clone)]
pub struct CreateLabelResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

/// Arguments of `void_label` (carrier). Everything `key` and `send`
/// derive from must be here, durable before the send.
#[derive(Debug, Clone)]
pub struct VoidLabelArgs {
    /// The deterministic identity of this effect (never a retry count).
    pub key: String,
    // TODO: the payload.
}

/// What `void_label` returns.
#[derive(Debug, Clone)]
pub struct VoidLabelResult {
    /// The provider's id for what landed (its receipt, its object id).
    pub provider_id: String,
    // TODO: the rest of the provider's answer.
}

impl Calls {
    /// Picks the provider from the environment, once, at construction: the
    /// HTTP client when the credential is set, the noop otherwise.
    pub fn from_env() -> Self {
        match std::env::var("FULFILMENT_PROVIDER_TOKEN") {
            Ok(token) if !token.is_empty() => Calls::new(Arc::new(HttpProvider {
                base_url: std::env::var("FULFILMENT_PROVIDER_URL").unwrap_or_default(),
                client: reqwest::Client::new(),
                token,
            })),
            _ => Calls::new(Arc::new(NoopProvider)),
        }
    }

    /// Builds every adapter over one provider; the tests pass the simulator.
    pub fn new<P: Provider>(provider: Arc<P>) -> Self {
        Self {
            create_label: Arc::new(
                Call::new(Adapter {
                    operation: "create_label".into(),
                    key: Some(key_fn(|a: &CreateLabelArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: CreateLabelArgs| {
                            let p = Arc::clone(&p);
                            async move { p.create_label(ctx, args).await }
                        })
                    },
                    // Irreversible: only the provider's definitive answer (a
                    // parsed 4xx) is a verdict; anything else is ambiguous
                    // and re-driven.
                    classify: Some(Classifier::fail_closed_on_definitive()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: true,
                    late_call: LateCall::KeyScoped,
                    strategy: Strategy::Declared {
                        slot_identity: SlotIdentity::Keyed,
                        resolution: DeclaredResolution::Resolve(resolve_by_lookup({
                            let p = Arc::clone(&provider);
                            lookup_fn(move |ctx, args: CreateLabelArgs| {
                                let p = Arc::clone(&p);
                                async move { p.lookup_create_label(ctx, args).await }
                            })
                        })),
                    },
                })
                .expect("fulfilment.create_label: invalid adapter"),
            ),
            void_label: Arc::new(
                Call::new(Adapter {
                    operation: "void_label".into(),
                    key: Some(key_fn(|a: &VoidLabelArgs| a.key.clone())),
                    send: {
                        let p = Arc::clone(&provider);
                        send_fn(move |ctx, args: VoidLabelArgs| {
                            let p = Arc::clone(&p);
                            async move { p.void_label(ctx, args).await }
                        })
                    },
                    classify: Some(Classifier::transport()),
                    call_timeout: Duration::from_secs(30),
                    irreversible: false,
                    late_call: LateCall::Convergent,
                    strategy: Strategy::Idempotent,
                })
                .expect("fulfilment.void_label: invalid adapter"),
            ),
        }
    }
}

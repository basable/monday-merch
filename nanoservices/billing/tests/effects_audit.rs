//! The audit index: one `audit!` per adapter in effects.rs, against the
//! simulator. Green on day one; the first thing a real provider must keep
//! green. Ack-loss probes are mandatory for irreversible adapters.

use std::sync::Arc;

use basable_effecttest::{Harness, audit};
use billing::effects::*;
use billing::simulator::{Fault, Simulator};

audit!(create_checkout_session, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.create_checkout_session.clone(),
        || CreateCheckoutSessionArgs {
            key: "create_checkout_session-1".into(),
            intent_declared_at: chrono::Utc::now(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("create_checkout_session") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("create_checkout_session", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("create_checkout_session", Fault::AckLoss, 1) }
    })
    // A newer intent is a new key.
    .advance_intent(|a| CreateCheckoutSessionArgs {
        key: format!("{}-next", a.key),
        ..a
    })
    .past_window(|a| CreateCheckoutSessionArgs {
        intent_declared_at: a.intent_declared_at - chrono::Duration::days(2),
        ..a
    })
});

audit!(expire_checkout_session, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.expire_checkout_session.clone(),
        || ExpireCheckoutSessionArgs {
            key: "expire_checkout_session-1".into(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("expire_checkout_session") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("expire_checkout_session", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("expire_checkout_session", Fault::AckLoss, 1) }
    })
    // A convergent effect with no intent scope in its args: the only
    // possible late call is a duplicate.
    .advance_intent(|a| a)
});

audit!(create_refund, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.create_refund.clone(),
        || CreateRefundArgs {
            key: "create_refund-1".into(),
            intent_declared_at: chrono::Utc::now(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("create_refund") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("create_refund", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("create_refund", Fault::AckLoss, 1) }
    })
    // A newer intent is a new key.
    .advance_intent(|a| CreateRefundArgs {
        key: format!("{}-next", a.key),
        ..a
    })
    .past_window(|a| CreateRefundArgs {
        intent_declared_at: a.intent_declared_at - chrono::Duration::days(2),
        ..a
    })
});

audit!(store_document, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.store_document.clone(),
        || StoreDocumentArgs {
            key: "store_document-1".into(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("store_document") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("store_document", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("store_document", Fault::AckLoss, 1) }
    })
    // A convergent effect with no intent scope in its args: the only
    // possible late call is a duplicate.
    .advance_intent(|a| a)
});

//! The audit index: one `audit!` per adapter in effects.rs, against the
//! simulator. Green on day one; the first thing a real provider must keep
//! green. Ack-loss probes are mandatory for irreversible adapters.

use std::sync::Arc;

use basable_effecttest::{Harness, audit};
use fulfilment::effects::*;
use fulfilment::simulator::{Fault, Simulator};

audit!(create_label, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.create_label.clone(),
        || CreateLabelArgs {
            key: "create_label-1".into(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("create_label") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("create_label", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("create_label", Fault::AckLoss, 1) }
    })
    // A newer intent is a new key.
    .advance_intent(|a| CreateLabelArgs {
        key: format!("{}-next", a.key),
        ..a
    })
});

audit!(void_label, || async {
    let sim = Arc::new(Simulator::new());
    let calls = Calls::new(sim.clone());
    let landed = sim.clone();
    let definitive = sim.clone();
    let ambiguity = sim.clone();
    Harness::new(
        calls.void_label.clone(),
        || VoidLabelArgs {
            key: "void_label-1".into(),
        },
        move || {
            let sim = landed.clone();
            async move { sim.landed_count("void_label") }
        },
        move || {
            let sim = definitive.clone();
            async move { sim.inject("void_label", Fault::Refused, 1) }
        },
    )
    .inject_ambiguity(move || {
        let sim = ambiguity.clone();
        async move { sim.inject("void_label", Fault::AckLoss, 1) }
    })
    // A convergent effect with no intent scope in its args: the only
    // possible late call is a duplicate.
    .advance_intent(|a| a)
});

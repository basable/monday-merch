# billing flows

## `invoice` lifecycle

TODO: the state machine of this type — the phases in `status`, what moves
each transition, what the reconcile pass checks level by level, how
teardown confirms absence (the Directive §3 invariant 6).

```mermaid
stateDiagram-v2
    [*] --> pending
    pending --> converged: reconcile pass
    converged --> deleting: mark_deleted
    deleting --> [*]: absence confirmed → Delete
```

## `payment` lifecycle

TODO: the state machine of this type — the phases in `status`, what moves
each transition, what the reconcile pass checks level by level, how
teardown confirms absence (the Directive §3 invariant 6).

```mermaid
stateDiagram-v2
    [*] --> pending
    pending --> converged: reconcile pass
    converged --> deleting: mark_deleted
    deleting --> [*]: absence confirmed → Delete
```

## `vat_return` lifecycle

TODO: the state machine of this type — the phases in `status`, what moves
each transition, what the reconcile pass checks level by level, how
teardown confirms absence (the Directive §3 invariant 6).

```mermaid
stateDiagram-v2
    [*] --> pending
    pending --> converged: reconcile pass
    converged --> deleting: mark_deleted
    deleting --> [*]: absence confirmed → Delete
```

## `supplier_bill` lifecycle

TODO: the state machine of this type — the phases in `status`, what moves
each transition, what the reconcile pass checks level by level, how
teardown confirms absence (the Directive §3 invariant 6).

```mermaid
stateDiagram-v2
    [*] --> pending
    pending --> converged: reconcile pass
    converged --> deleting: mark_deleted
    deleting --> [*]: absence confirmed → Delete
```

## `invoice` in the ledger

The accounting states as `billing` reports them, and the changes the
ledger accepts (a state listing itself allows amount changes; a state with
no change out of it is terminal, its amounts frozen). Each state's desired
state is the positions in `src/ledger/invoice.rs`.

```mermaid
stateDiagram-v2
    [*] --> issued
```

## `credit_note` in the ledger

The accounting states as `billing` reports them, and the changes the
ledger accepts (a state listing itself allows amount changes; a state with
no change out of it is terminal, its amounts frozen). Each state's desired
state is the positions in `src/ledger/credit_note.rs`.

```mermaid
stateDiagram-v2
    [*] --> issued
```

## `payment` in the ledger

The accounting states as `billing` reports them, and the changes the
ledger accepts (a state listing itself allows amount changes; a state with
no change out of it is terminal, its amounts frozen). Each state's desired
state is the positions in `src/ledger/payment.rs`.

```mermaid
stateDiagram-v2
    [*] --> pending
    [*] --> settled
    pending --> settled
    pending --> reversed
    settled --> reversed
```

## `vat_return` in the ledger

The accounting states as `billing` reports them, and the changes the
ledger accepts (a state listing itself allows amount changes; a state with
no change out of it is terminal, its amounts frozen). Each state's desired
state is the positions in `src/ledger/vat_return.rs`.

```mermaid
stateDiagram-v2
    [*] --> filed
```

## `supplier_bill` in the ledger

The accounting states as `billing` reports them, and the changes the
ledger accepts (a state listing itself allows amount changes; a state with
no change out of it is terminal, its amounts frozen). Each state's desired
state is the positions in `src/ledger/supplier_bill.rs`.

```mermaid
stateDiagram-v2
    [*] --> approved
    approved --> voided
```

## Edge cases

| # | Trigger | Code path | Behaviour |
|---|---|---|---|
| 1 | TODO | | |

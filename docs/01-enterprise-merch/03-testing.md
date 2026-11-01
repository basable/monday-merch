# 03: Testing

Per-nanoservice test lists are in each `02-*.md`. This file covers what spans them.

## Unit tests

Pure logic with no database: line pricing, tier selection, VAT treatment, delivery date estimates, ledger `desired_state`, template rendering, and each reconcile pass's choice of outcome given a snapshot.

## External-effect audits

Exactly one `effecttest` audit per adapter, in the nanoservice's `tests/effects_audit.rs`, run through the real provider client against the nanoservice's simulator. Ack loss is mandatory for irreversible calls.

| Nanoservice | Call | Strategy | Irreversible |
| --- | --- | --- | --- |
| `design` | `delete_object` | `idempotent` | No |
| `fulfilment` | `create_label` | `declared` | Yes |
| `fulfilment` | `void_label` | `idempotent` | No |
| `billing` | `create_checkout_session` | `keyed_replay` 24h | Yes |
| `billing` | `expire_checkout_session` | `idempotent` | No |
| `billing` | `create_refund` | `keyed_replay` 24h | Yes |
| `billing` | `store_document` | `idempotent` | No |
| `notifier` | `send_email` | `keyed_replay` 24h | Yes |

## Integration suites

On `basable-testkit`, with Postgres in testcontainers and one database per test.

- **Order to delivery:** a quote through to a settled order in each delivery mode, with both payment gates, with simulators for every provider.
- **Tenant isolation:** company A's member calls every customer method with company B's ids and gets refused. Generate the method list from the manifest so a new method cannot be missed. This is the most important security test in a multi-tenant product.
- **Role checks:** every staff method refuses a customer, and every customer write refuses a `viewer`.
- **Cost privacy:** no public or customer response contains supplier cost.
- **Crash recovery:** kill an attempt between every effect and its status write in `order`, `production_run`, `shipment`, `invoice` and `payment`. The next pass converges with no duplicate effect.
- **Two replicas:** run two workers per type against one database, and each object is driven by one at a time.
- **Stock concurrency:** parallel reservations never oversell, and counts match movements.
- **Invoice sequence:** concurrent issuing gives a gap-free sequence per series.
- **Ledger:** `basable-ledger-testkit` scenario, property and conformance suites, plus a trial balance of zero per currency at the end of every order scenario.
- **Personal data:** after retention runs, no recipient or shipment address remains in any table.

## Browser checks

A small set of end-to-end runs against a preview environment: request a quote anonymously, sign in with an email code, sign a quote, pay in a Stripe sandbox, see the order's status. Also, `/shop` pages render without JavaScript, because search engines depend on that.

## What CI proves before a deploy

`.basable/verify.sh` (rendered by the scaffold) builds and runs all of the above except the browser checks: `bazel test //...` over the whole workspace, including the generated messenger and images, plus buf lint on the protos. A send nobody handles, an undeclared send and a response mismatch all fail at build time through `routing.yaml`. Plan commits touch only `docs/**` and start no build.

Performance-sensitive paths (PDF rendering, `/shop` pages) are measured in preview, because the production sandbox makes disk and network I/O cost more than a bare node.

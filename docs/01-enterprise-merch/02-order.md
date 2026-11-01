# 02: order

From a visitor's request to a delivered order. A quote is plain tables, because its waits are timestamps a ticker can check. An order and a production run are lifecycles, because each waits on people and other systems over days and must be re-driven until it matches. Directive §3, §5, §6 bind the processing-object types, §2 every table, §7 every message.

**Round 1 decided self-serve checkout `later`.** Every order comes from a signed quote.

## Quote: plain tables

Status runs: `requested` → `in_progress` → `sent` ⇄ `changes_requested` → `accepted`, or to `expired` or `withdrawn`.

**`quote`**: `number`, `company_id` (empty for an anonymous request until signature), `contact`, `contact_email`, `delivery` (one address, a saved address, or a mode from the list below), `timeline` (`standard` or `express`), `deadline_note`, `status`, `consultant_id`, `current_version`, `requested_at`, `sla_due_at`, `sla_alerted_at`, `valid_until`, `source_order_id` (for a reorder), `order_id`.

**`quote_version`** is what the customer sees: `quote_id`, `version`, `lines`, `shipping_minor`, `handling_minor`, `currency`, `note`, `published_at`, `published_by`. Quotes exclude VAT, which is added on the invoice.

**`quote_signature`** is append-only evidence: `quote_id`, `version`, `signer_identity_id`, `signer_name`, `signer_email`, `po_number`, `signed_at`.

**`quote_comment`** is the conversation: `quote_id`, `author`, `body`, `created_at`.

**`lead`** is a contact-form message: `email`, `name`, `company_name`, `message`, `created_at`, `erase_after`.

Quote invariants:

- A published `quote_version` never changes. A correction is a new version, and it resets the customer's review.
- Signing names an exact version and is a compare-and-set: `status = 'sent' AND current_version = $v AND valid_until > now()`. A second click with the same version returns the same order.
- Signing requires a Kratos session whose verified email is `contact_email` or a member of `company_id`. This proves mailbox control before a company is created.
- No changes after signature. `CancelOrder` is staff-only.

`RequestQuote` is public, so it is rate limited per IP hash with an in-memory `DashMap` (approximate across replicas, which is acceptable here), capped in size, and it re-prices every line through `PriceLinesRequest`. Spam protection beyond that is decided once real traffic shows the need.

### Signing, step by step

`AcceptQuote` runs: check signature rights → `EnsureCompanyRequest` when there is no company → `ApproveProofsRequest` → write the signature and `accepted` in one transaction → `TypedStore::create` the `order`, named by the quote → store `order_id`.

A crash between the last two steps leaves an `accepted` quote with no order. `sweep_quotes` finds these and finishes the job. The name makes that create idempotent.

## Processing object: `order`

**Spec**, immutable because a signed quote cannot change: `quote_id`, `quote_version`, `company_id`, `lines` (each with supplier, cost snapshot, proof, department), `currency`, `delivery`, `timeline`, `payment_gate` (derived at signature from the company, so the order does not re-derive a rule it does not own). Mutable: `cancel_requested`.

**Status**: `phase`, `invoice_id`, `paid_at`, `production_started_at`, `received_at`, `completed_at`, `attention` (a sentence for staff when something needs a person).

Delivery modes: `ship` (via the warehouse to one or more addresses), `store` (into held stock), `direct` (the supplier ships to the customer and staff record the tracking).

**Reconcile pass**, level-triggered, top to bottom, first unmet level wins:

| Level | Checks | Drives | Outcome |
| --- | --- | --- | --- |
| Deleting | `meta.deleting()` | No API calls `mark_deleted`. Confirm no run or invoice exists. | `Delete` when absent, otherwise `Retry` and stay loud |
| Cancelled | `cancel_requested` | `update_spec` on each run. A run already past `in_production` needs a person. | `Blocked` with `attention`, or `Settled` at phase `cancelled`. Any credit note is a staff decision. |
| Invoice | `invoice_id` empty | `EnsureInvoiceRequest` named `order:<id>`, with generation and lines | `Retry` on failure, otherwise continue |
| Payment gate | `payment_gate = prepay` and unpaid | `GetInvoiceRequest` | `Converged.after(15m)`. `InvoiceChanged` wakes it sooner. |
| Production | Runs missing | `TypedStore::create` one `production_run` per supplier, named `order:<id>:<supplier>` | Continue |
| Waiting on runs | Any run unfinished | Reads run status | `Converged.after(1h)`. A run nudges its order on change. |
| Delivery | Mode `ship` | `EnsureShipmentRequest` per address, named `order:<id>:<n>`, with generation | Continue |
| Waiting on delivery | Any shipment undelivered | `GetShipmentRequest` | `Converged.after(1h)`. `ShipmentChanged` wakes it sooner. |
| Done | All met | Sets `completed_at` | `Settled` |

Mode `store` is done once goods are received. Mode `direct` is done once every run reports delivered.

**Teardown:** orders are business records and no API deletes them. Teardown exists because invariant 6 requires a path, and it only confirms absence.

**Worker:** resync 1h, backoff 30s to 15m, `max_attempts` 0 (long waits are `Converged.after`, not failures, and a transport failure should keep trying), attempt timeout 60s.

The "invoice within 48 hours" promise is met in seconds on the normal path. A metric on orders in phase `invoicing` over one hour alerts staff.

## Processing object: `production_run`

One purchase order to one supplier.

**Spec**: `order_id`, `order_generation` (applied only when strictly higher, Directive §7), `supplier_slug`, `lines`, `destination` (warehouse or customer address), `due_on`, `cancel_requested`.

**Status**: `phase` (`drafted`, `po_sent`, `acknowledged`, `in_production`, `shipped`, `received`, `delivered`, `cancelled`), `po_intent_at`, `po_sent_at`, `acknowledged_at`, `supplier_ref`, `expected_at`, `inbound_id`, `received_at`, `carrier`, `tracking_number`.

Suppliers have no API at this stage, so staff record what they hear. That input has no claim, so it goes in the side table **`production_event`** (`run_id`, `kind`, `detail`, `actor`, `occurred_at`) followed by a `nudge` (Directive §3, invariant 3).

**Reconcile pass:**

| Level | Checks | Drives | Outcome |
| --- | --- | --- | --- |
| Deleting | `meta.deleting()` | Test path only | `Delete` once confirmed |
| Cancelled | `cancel_requested` | PO not sent: stop. Sent: email the supplier, key `po-cancel:<id>`, wait for a `cancel_confirmed` event. In production: needs a person. | `Settled`, `Converged.after(24h)` or `Blocked` |
| Purchase order | `po_sent_at` empty | `GetApprovedArtworkRequest`, `claim.write_status` with `po_intent_at`, then `SendEmailRequest` key `po:<id>` | `Retry` on failure. Past the 24h replay window it holds for staff rather than re-sending. |
| Events | New `production_event` rows | Folds them into status | Continue |
| Inbound | Destination is a warehouse | `EnsureInboundRequest` named by the run, with generation; `GetInboundRequest` | `Converged.after(6h)`. `InboundChanged` wakes it sooner. |
| Overdue | `expected_at` passed | Emails the consultant, key `overdue:<id>:<date>` | `Converged.after(24h)` |
| Done | Received or delivered | Nudges the order | `Settled` |

**Worker:** resync 6h, backoff 1m to 30m, `max_attempts` 0, attempt timeout 60s.

## Schedules

`sweep_quotes` runs every 15m: alerts the consultant at `sla_alert_hours` and marks a breach at `sla_hours`, stamping `sla_alerted_at` so each alert goes once; expires quotes past `valid_until`; finishes signed quotes that have no order yet; erases `lead` rows past `erase_after`.

## Config types

`quote_policy`. See `04-config.md`.

## Messages and API

`OrderService`, public: `RequestQuote`, `SubmitLead`. Customer: `ListQuotes`, `GetQuote`, `RequestQuoteChanges`, `AcceptQuote`, `Reorder`, `ListOrders`, `GetOrder`. Staff: `AssignQuote`, `PublishQuoteVersion`, `CancelOrder`, `ListProductionRuns`, `RecordProductionEvent`.

`Reorder` starts a new quote from an order's lines and approved proofs at today's prices. A restock request is the same call.

`InvoiceChanged`, `InboundChanged` and `ShipmentChanged` are wake-ups only. Each handler nudges the object concerned. The resync is what guarantees progress if an event is lost, so these are events and not cascades (Directive §7).

## Binding Directive sections

§1, §2 (tables), §3 to §5 (`order`, `production_run`), §7 (generations on `EnsureInvoiceRequest`, `EnsureInboundRequest`, `EnsureShipmentRequest`; names as seams), §11 (quote amounts are not owed, so the ledger starts at the invoice).

## Tests

- Golden path through every phase and each delivery mode, on the testkit.
- Signature: same version twice gives one order. A stale version, an expired quote and a wrong email are all refused.
- Crash between signature and `TypedStore::create`: `sweep_quotes` creates exactly one order.
- A prepay order starts no production until the invoice settles, and an on-terms order does not wait.
- A stale `order_generation` is dropped by `update_spec`, and an equal one passes.
- A fenced-out attempt writes no status.
- Published `quote_version` rows reject changes.
- Company A cannot see or sign company B's quotes.

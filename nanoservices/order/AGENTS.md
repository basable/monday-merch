# order

Takes a request from quote to signature, then drives each order through invoicing, production and delivery.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §3–§5
(the invariants, the author obligations, the adapter and outcome contract — it
owns processing-object types), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

| Processing-object type | Key | Prefix | Spec fields | Status fields |
|---|---|---|---|---|
| `quote` | 1 | `quo` | `contact`, `contact_email` (immutable), `company_id`, `delivery`, `timeline`, `deadline_note`, `consultant_id`, `published_version`, `valid_until`, `withdrawn`, `requested_at` (immutable), `sla_due_at` (immutable), `source_order_id` (immutable), `source_document_id` (immutable) | `phase`, `sla_alerted_at`, `sent_version`, `send_intent_at`, `signed_version`, `order_id`, `expired_at` |
| `order` | 2 | `ord` | `quote_id` (immutable), `quote_version` (immutable), `company_id` (immutable), `lines` (immutable), `currency` (immutable), `delivery` (immutable), `timeline` (immutable), `payment_gate` (immutable), `cancel_requested` | `phase`, `invoice_id`, `paid_at`, `production_started_at`, `received_at`, `completed_at`, `attention` |
| `production_run` | 3 | `prun` | `order_id` (immutable), `order_generation`, `supplier_slug` (immutable), `lines` (immutable), `destination` (immutable), `due_on`, `cancel_requested` | `phase`, `po_intent_at`, `po_sent_at`, `acknowledged_at`, `supplier_ref`, `expected_at`, `inbound_id`, `received_at`, `carrier`, `tracking_number` |

Plain tables (schema `nano_order`): `quote_version`, `quote_signature`, `quote_comment`, `production_event`, `lead`.

Config catalogs: `quote_policy` (`config/base/quote_policy.json`).

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Order` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl OrderHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/types/<type>/type.rs` | The `ProcessingObjectType` declaration: spec/status structs, `TYPE_KEY`, `PUBLIC_ID_PREFIX`. |
| `src/types/<type>/adapter.rs` | The dumb column mapper: every column in BOTH `write_spec` and `read_rows`. |
| `src/types/<type>/reconciler.rs` | The level-triggered pass and the worker policy (identical on every replica). |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/worker.rs` | The ticker workers (`purge_leads` every 24h), listed in this nanoservice's `loops()` (`src/lib.rs`), started by the app and joined on shutdown. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `RequestQuoteRequest` → `Quote`, `ListQuotesRequest` → `QuoteList`, `GetQuoteRequest` → `Quote`, `AssignQuoteRequest` → `Quote`, `PublishQuoteVersionRequest` → `Quote`, `RequestQuoteChangesRequest` → `Quote`, `AcceptQuoteRequest` → `Order`, `ReorderRequest` → `Quote`, `ListOrdersRequest` → `OrderList`, `GetOrderRequest` → `Order`, `CancelOrderRequest` → `Order`, `ListProductionRunsRequest` → `ProductionRunList`, `RecordProductionEventRequest` → `ProductionRun`, `SubmitLeadRequest` → `LeadAck`, `RecordSupplierUpdateRequest` → `ProductionRun`, `IngestCustomerEmailRequest` → `Quote`, `GetPurchaseMatchRequest` → `PurchaseMatch`, `InvoiceChanged`, `InboundChanged`, `ShipmentChanged`.
Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `EnsureCompanyRequest`, `PriceLinesRequest`, `GetSupplierOffersRequest`, `ListProofsRequest`, `ApproveProofsRequest`, `GetApprovedArtworkRequest`, `ResolveDesignVersionRequest`, `AdoptDesignsRequest`, `EnsureInvoiceRequest`, `GetInvoiceRequest`, `EnsureInboundRequest`, `GetInboundRequest`, `EnsureShipmentRequest`, `GetShipmentRequest`, `SendEmailRequest`, `AuditEvent`.

API: `OrderService` in `api/src/services/order.rs` (RequestQuote, ListQuotes, GetQuote, AssignQuote, PublishQuoteVersion, RequestQuoteChanges, AcceptQuote, Reorder, ListOrders, GetOrder, CancelOrder, ListProductionRuns, RecordProductionEvent, SubmitLead).

# inbox

Receives email, stores it, extracts structured data from each document, and routes the result to its owner or a review queue.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §3–§5
(the invariants, the author obligations, the adapter and outcome contract — it
owns processing-object types), §6 (external-effect admission — it owns adapters), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

| Processing-object type | Key | Prefix | Spec fields | Status fields |
|---|---|---|---|---|
| `inbound_message` | 9 | `imsg` | `provider_message_id` (immutable), `to_address` (immutable), `from_address` (immutable), `subject` (immutable), `received_at` (immutable) | `phase`, `raw_key`, `message_id_header`, `sender_supplier_slug`, `sender_verified`, `route_hint`, `document_count`, `rejection` |
| `inbound_document` | 10 | `idoc` | `message_id` (immutable), `part_name` (immutable), `object_key` (immutable), `mime_type` (immutable), `sha256` (immutable), `sender_supplier_slug` (immutable), `sender_verified` (immutable), `route_hint` (immutable) | `phase`, `kind`, `extraction`, `model`, `checks`, `review_reason`, `routed_to`, `routed_ref`, `applied_at` |

Plain tables (schema `nano_inbox`): `inbound_seen`, `review_decision`.

Config catalogs: `inbox_policy` (`config/base/inbox_policy.json`).

| External call | System | Strategy | Irreversible |
|---|---|---|---|
| `store_inbound_object` | object_storage | `idempotent` | no |
| `delete_inbound_object` | object_storage | `idempotent` | no |
| `extract_document` | llm | `idempotent` | no |

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Inbox` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl InboxHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/types/<type>/type.rs` | The `ProcessingObjectType` declaration: spec/status structs, `TYPE_KEY`, `PUBLIC_ID_PREFIX`. |
| `src/types/<type>/adapter.rs` | The dumb column mapper: every column in BOTH `write_spec` and `read_rows`. |
| `src/types/<type>/reconciler.rs` | The level-triggered pass and the worker policy (identical on every replica). |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/effects.rs` | One `Call` per external call with its strategy literal; `send` is the only dispatch path. |
| `src/provider.rs` / `src/simulator.rs` | The provider trait, the HTTP client (a parsed 4xx is `SendError::Refused`), the noop; the in-memory simulator the audit and the tests drive. |
| `tests/effects_audit.rs` | One `audit!` per adapter — the audit index. Green on day one; the first thing a real provider must keep green. |
| `src/worker.rs` | The ticker workers (`purge_inbound` every 24h), listed in this nanoservice's `loops()` (`src/lib.rs`), started by the app and joined on shutdown. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `ListInboundMessagesRequest` → `InboundMessageList`, `ListReviewQueueRequest` → `InboundDocumentList`, `GetInboundDocumentRequest` → `InboundDocument`, `GetInboundFileLinkRequest` → `DownloadLink`, `DecideInboundDocumentRequest` → `InboundDocument`, `InboundEmailEvent` → `WebhookAck`.
Sends: `ResolveMemberRequest`, `MatchSupplierRequest`, `EnsureSupplierBillRequest`, `RecordSupplierUpdateRequest`, `IngestCustomerEmailRequest`, `SendEmailRequest`, `AuditEvent`.

API: `InboxService` in `api/src/services/inbox.rs` (ListInboundMessages, ListReviewQueue, GetInboundDocument, GetInboundFileLink, DecideInboundDocument).

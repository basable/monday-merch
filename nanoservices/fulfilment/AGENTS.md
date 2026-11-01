# fulfilment

Owns held stock, kits and inbound goods, and drives each shipment from reservation to delivery.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §3–§5
(the invariants, the author obligations, the adapter and outcome contract — it
owns processing-object types), §6 (external-effect admission — it owns adapters), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

| Processing-object type | Key | Prefix | Spec fields | Status fields |
|---|---|---|---|---|
| `shipment` | 4 | `shp` | `name` (immutable), `source_generation`, `company_id` (immutable), `batch_id` (immutable), `warehouse_slug` (immutable), `lines` (immutable), `service_level` (immutable), `incoterm` (immutable), `cancel_requested` | `phase`, `reserved_at`, `picked_at`, `label_slot`, `label_id`, `label_key`, `carrier`, `tracking_number`, `dispatched_at`, `delivered_at`, `exception` |

Plain tables (schema `nano_fulfilment`): `stock_item`, `stock_level`, `stock_movement`, `kit_component`, `assembly_order`, `inbound`, `low_stock_rule`, `shipment_batch`, `shipment_address`, `shipment_event`.

Config catalogs: `warehouse` (`config/base/warehouse.json`), `shipping_zone` (`config/base/shipping_zone.json`), `fulfilment_fee` (`config/base/fulfilment_fee.json`).

| External call | System | Strategy | Irreversible |
|---|---|---|---|
| `create_label` | carrier | `declared` | yes |
| `void_label` | carrier | `idempotent` | no |

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Fulfilment` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl FulfilmentHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/types/<type>/type.rs` | The `ProcessingObjectType` declaration: spec/status structs, `TYPE_KEY`, `PUBLIC_ID_PREFIX`. |
| `src/types/<type>/adapter.rs` | The dumb column mapper: every column in BOTH `write_spec` and `read_rows`. |
| `src/types/<type>/reconciler.rs` | The level-triggered pass and the worker policy (identical on every replica). |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/effects.rs` | One `Call` per external call with its strategy literal; `send` is the only dispatch path. |
| `src/provider.rs` / `src/simulator.rs` | The provider trait, the HTTP client (a parsed 4xx is `SendError::Refused`), the noop; the in-memory simulator the audit and the tests drive. |
| `tests/effects_audit.rs` | One `audit!` per adapter — the audit index. Green on day one; the first thing a real provider must keep green. |
| `src/worker.rs` | The ticker workers (`check_low_stock` every 1h, `purge_shipment_addresses` every 24h), listed in this nanoservice's `loops()` (`src/lib.rs`), started by the app and joined on shutdown. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `EstimateDeliveryRequest` → `DeliveryEstimate`, `ListStockRequest` → `StockList`, `SetLowStockRuleRequest` → `LowStockRule`, `AdjustStockRequest` → `StockLevel`, `ListKitsRequest` → `KitList`, `UpsertKitRequest` → `Kit`, `CreateAssemblyOrderRequest` → `AssemblyOrder`, `RecordAssemblyRequest` → `AssemblyOrder`, `ListInboundRequest` → `InboundList`, `RecordInboundReceiptRequest` → `Inbound`, `CreateShipmentsRequest` → `ShipmentBatch`, `ListShipmentsRequest` → `ShipmentList`, `GetShipmentRequest` → `Shipment`, `RecordPickRequest` → `Shipment`, `CancelShipmentRequest` → `Shipment`, `EnsureInboundRequest` → `Inbound`, `GetInboundRequest` → `Inbound`, `EnsureShipmentRequest` → `Shipment`, `CarrierWebhookEvent` → `WebhookAck`.
Sends: `ResolveMemberRequest`, `ResolveRecipientsRequest`, `GetCustomsDataRequest`, `EnsureInvoiceRequest`, `SendEmailRequest`, `InboundChanged`, `ShipmentChanged`, `AuditEvent`.

API: `FulfilmentService` in `api/src/services/fulfilment.rs` (EstimateDelivery, ListStock, SetLowStockRule, AdjustStock, ListKits, UpsertKit, CreateAssemblyOrder, RecordAssembly, ListInbound, RecordInboundReceipt, CreateShipments, ListShipments, GetShipment, RecordPick, CancelShipment).

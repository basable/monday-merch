# 02: fulfilment

Goods in the building and goods on the road. A stock count is a quantity, not money, so it stays in plain columns (Directive §11). A shipment is a lifecycle: it waits on stock and pickers, drives a carrier, then follows tracking for days. Directive §3, §5, §6 bind `shipment`, §2 every table, §6 every call, §7 every message.

**Round 1 decided held stock, kits and shipping from stock `include`.** No warehouse or logistics partner is chosen yet, so picking is recorded by staff until round 3 settles who runs the warehouse.

## Stock: plain tables

**`stock_item`** is one thing a company holds: `company_id`, `name`, `product_id`, `variant_id`, `size`, `proof_id`, `is_kit`. It references other nanoservices' rows by id only, with no join and no foreign key across schemas.

**`stock_level`** is the current count per warehouse: `stock_item_id`, `warehouse_slug`, `on_hand`, `reserved`.

**`stock_movement`** is the append-only history: `stock_item_id`, `warehouse_slug`, `kind`, `on_hand_delta`, `reserved_delta`, `source`, `actor`, `occurred_at`.

**`kit_component`** defines a merch pack: `kit_item_id`, `component_item_id`, `quantity`.

**`assembly_order`** is a request to build packs: `company_id`, `kit_item_id`, `warehouse_slug`, `quantity`, `completed_at`, `cancelled_at`. It is plain because a person does the work and records it, and nothing needs driving.

**`inbound`** is goods expected from a supplier: `name` (the run's name, unique), `source_generation` (highest accepted, lower ones dropped), `company_id`, `warehouse_slug`, `lines`, `received_lines`, `received_at`. Whether goods are expected, partial or complete follows from those fields.

**`low_stock_rule`**: `stock_item_id`, `threshold`, `notify_email`.

Stock invariants:

- Every change locks the `stock_level` row with `FOR UPDATE`, updates it, and appends a `stock_movement` in the same transaction.
- `reserved` never exceeds `on_hand`, and neither goes negative. Add check constraints in a follow-up migration, along with a unique index on (`stock_item_id`, `warehouse_slug`), which a manifest index cannot express.
- `on_hand` always equals the sum of movements. A test checks this after every scenario.
- Recording an inbound receipt twice cannot count twice. Movements are keyed by `source`.

## Processing object: `shipment`

**Spec**: `name` (the caller's seam, such as `order:<id>:<n>`, or `batch:<id>:<n>`), `source_generation`, `company_id`, `batch_id`, `warehouse_slug`, `lines`, `service_level` (`standard`, `express`), `incoterm`. Mutable: `cancel_requested`.

**Status**: `phase` (`reserving`, `awaiting_stock`, `awaiting_pick`, `picked`, `label_created`, `dispatched`, `in_transit`, `delivered`, `exception`, `cancelled`), `reserved_at`, `picked_at`, `label_slot`, `label_id`, `label_key`, `carrier`, `tracking_number`, `dispatched_at`, `delivered_at`, `exception`.

Home addresses are not in the spec, because spec fields survive for the life of the object. They live in **`shipment_address`** (`shipment_id`, `address`, `erase_after`, `erased_at`) so they can be erased while the shipment record stays.

Inputs with no claim go in the side table **`shipment_event`** (`shipment_id`, `kind`, `detail`, `dedupe_key`, `occurred_at`) followed by a `nudge`: staff picks through `RecordPick` and tracking from the carrier webhook. `dedupe_key` is unique, so a webhook redelivery is harmless.

**`shipment_batch`** groups shipments from stock created together: `company_id`, `created_by`, `charges`, `created_at`. `CreateShipments` creates the batch with its charges, then one `shipment` per address.

**Reconcile pass:**

| Level | Checks | Drives | Outcome |
| --- | --- | --- | --- |
| Deleting | `meta.deleting()` | Release the reservation, `void_label` if a label exists | `Delete` on confirmed void, otherwise `Retry` and stay loud |
| Cancelled | `cancel_requested` | Before dispatch: as deleting, without removal. After dispatch: needs a person. | `Settled` at `cancelled`, or `Blocked` |
| Charges | Has `batch_id` | `EnsureInvoiceRequest` named `batch:<id>` with the batch's charges. Every shipment in the batch may send it, and `billing` dedupes on the name. | `Retry` on failure |
| Reserve | `reserved_at` empty | Own-table transaction on `stock_level` | Continue. If stock is short: `Converged.after(1h)` at `awaiting_stock`, with `exception` for staff. |
| Pick | `picked_at` empty | Reads `shipment_event` | `Converged.after(1h)` at `awaiting_pick`. `RecordPick` nudges it. |
| Label | `label_id` empty | Declare and send, below | `Retry` on ambiguity |
| Dispatch | No dispatch event | Reads `shipment_event`, polls by tracking number as a read | `Converged.after(6h)` |
| Tracking | Not delivered | Folds tracking events. Sends `ShipmentChanged` on a phase change. | `Converged.after(6h)` |
| Done | Delivered | Sets `erase_after` on the address | `Settled` |

**Worker:** resync 6h, backoff 1m to 30m, `max_attempts` 0, attempt timeout 60s.

## External calls

The provider is chosen in round 3. A multi-carrier aggregator is the likely shape, since their carriers are DHL, FedEx and PostNL.

**`create_label`** on `carrier`, strategy `declared`, `irreversible` because a bought label costs money. Before the send, `claim.write_status` records `label_slot`, a deterministic reference from the shipment id. On the next claim, the resolver looks the shipment up by that reference. Until a provider with such a lookup is chosen, the plan assumes `Hold`, meaning an ambiguous send stops for a person rather than buying a second label. If the chosen provider documents idempotency keys with a replay window, reshape this to `keyed_replay` (Directive §6 says to look for the cheaper strategy). The simulator models success, definitive rejection, ack loss and lookup.

**`void_label`** on `carrier`, strategy `idempotent`: voiding a voided label succeeds. If the provider differs, re-check at selection.

**Carrier webhook:** a raw route verifies the signature and makes one send, `CarrierWebhookEvent`. The handler inserts `shipment_event` and nudges. The pass works without it, because it polls on resync.

## Prices and estimates

`EstimateDelivery` adds `production_days` from the caller to the zone's transit days, skips weekends, and returns a date range. That is what the product page and quote form show. `shipping_zone` and `fulfilment_fee` give shipping and handling amounts. A shipment from stock is charged per batch. An order's shipping and handling are already lines on its quote.

Customs data (`hs_code`, origin, weight, value) comes from `GetCustomsDataRequest`. Non-EU labels need it.

## Schedules

`check_low_stock` runs every 1h: for each rule at or below its threshold, sends one email keyed `low-stock:<item>:<date>`.

`purge_shipment_addresses` runs every 24h: erases `shipment_address.address` past `erase_after`, 12 months after delivery by assumption. Legal review should confirm this.

## Config types

`warehouse`, `shipping_zone`, `fulfilment_fee`. See `04-config.md`.

## Messages and API

`FulfilmentService`, public: `EstimateDelivery`. Customer: `ListStock`, `SetLowStockRule`, `ListKits`, `UpsertKit`, `CreateAssemblyOrder`, `CreateShipments`, `ListShipments`, `GetShipment`. Staff: `AdjustStock`, `RecordAssembly`, `ListInbound`, `RecordInboundReceipt`, `RecordPick`, `CancelShipment`.

Internal: `EnsureInboundRequest`, `GetInboundRequest`, `EnsureShipmentRequest` and `GetShipmentRequest` from `order`. A generation lower than the stored one is dropped.

## Binding Directive sections

§1, §2 (tables), §3 to §5 (`shipment`), §6 (`create_label`, `void_label`), §7 (generations, names, events as wake-ups), §11 (stock is not money).

## Tests

- `effecttest` audits for `create_label` (ack loss included, mandatory because irreversible) and `void_label`.
- Parallel reservations against one `stock_level` never oversell. Run with 50 concurrent tasks.
- `on_hand` equals the sum of movements after every scenario.
- Assembly consumes components and adds packs in one transaction, and fails whole on a shortage.
- A redelivered carrier webhook adds one event. A lost webhook is caught by the polling pass.
- A crash after `label_slot` and before the response buys no second label.
- Cancelling before and after dispatch.
- Erased addresses leave the shipment readable.
- `EstimateDelivery` across a weekend and for express.

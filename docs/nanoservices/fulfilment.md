# fulfilment

Owns held stock, kits and inbound goods, and drives each shipment from reservation to delivery.

Design: `docs/01-enterprise-merch/02-fulfilment.md`. Code:
`nanoservices/fulfilment/` (read its `AGENTS.md` first).

## Owns

- Processing-object type `shipment` (key 4, `shp_…`): spec `name`, `source_generation`, `company_id`, `batch_id`, `warehouse_slug`, `lines`, `service_level`, `incoterm`, `cancel_requested`; status `phase`, `reserved_at`, `picked_at`, `label_slot`, `label_id`, `label_key`, `carrier`, `tracking_number`, `dispatched_at`, `delivered_at`, `exception`.
- Table `stock_item` (`id`, `company_id`, `name`, `product_id`, `variant_id`, `size`, `proof_id`, `is_kit`).
- Table `stock_level` (`id`, `stock_item_id`, `warehouse_slug`, `on_hand`, `reserved`).
- Table `stock_movement` (`id`, `stock_item_id`, `warehouse_slug`, `kind`, `on_hand_delta`, `reserved_delta`, `source`, `actor`, `occurred_at`).
- Table `kit_component` (`id`, `kit_item_id`, `component_item_id`, `quantity`).
- Table `assembly_order` (`id`, `company_id`, `kit_item_id`, `warehouse_slug`, `quantity`, `completed_at`, `cancelled_at`).
- Table `inbound` (`id`, `name`, `source_generation`, `company_id`, `warehouse_slug`, `lines`, `received_lines`, `received_at`).
- Table `low_stock_rule` (`id`, `stock_item_id`, `threshold`, `notify_email`).
- Table `shipment_batch` (`id`, `company_id`, `created_by`, `charges`, `created_at`).
- Table `shipment_address` (`shipment_id`, `address`, `erase_after`, `erased_at`).
- Table `shipment_event` (`id`, `shipment_id`, `kind`, `detail`, `dedupe_key`, `occurred_at`).
- Config type `warehouse`.
- Config type `shipping_zone`.
- Config type `fulfilment_fee`.
- External call `create_label` on carrier: `declared`, irreversible.
- External call `void_label` on carrier: `idempotent`.
- Schedule `check_low_stock` every 1h.
- Schedule `purge_shipment_addresses` every 24h.
- Webhook from `carrier`.

## Messages

- Handles: `EstimateDeliveryRequest`, `ListStockRequest`, `SetLowStockRuleRequest`, `AdjustStockRequest`, `ListKitsRequest`, `UpsertKitRequest`, `CreateAssemblyOrderRequest`, `RecordAssemblyRequest`, `ListInboundRequest`, `RecordInboundReceiptRequest`, `CreateShipmentsRequest`, `ListShipmentsRequest`, `GetShipmentRequest`, `RecordPickRequest`, `CancelShipmentRequest`, `EnsureInboundRequest`, `GetInboundRequest`, `EnsureShipmentRequest`, `CarrierWebhookEvent`.
- Sends: `ResolveMemberRequest`, `ResolveRecipientsRequest`, `GetCustomsDataRequest`, `EnsureInvoiceRequest`, `SendEmailRequest`, `InboundChanged`, `ShipmentChanged`, `AuditEvent`.

## Directive sections that bind

§2 always; §3, §4, §5 for each processing-object type; §6 for each adapter; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_estimate_delivery_request` in `src/handlers.rs`.
- [ ] `handle_list_stock_request` in `src/handlers.rs`.
- [ ] `handle_set_low_stock_rule_request` in `src/handlers.rs`.
- [ ] `handle_adjust_stock_request` in `src/handlers.rs`.
- [ ] `handle_list_kits_request` in `src/handlers.rs`.
- [ ] `handle_upsert_kit_request` in `src/handlers.rs`.
- [ ] `handle_create_assembly_order_request` in `src/handlers.rs`.
- [ ] `handle_record_assembly_request` in `src/handlers.rs`.
- [ ] `handle_list_inbound_request` in `src/handlers.rs`.
- [ ] `handle_record_inbound_receipt_request` in `src/handlers.rs`.
- [ ] `handle_create_shipments_request` in `src/handlers.rs`.
- [ ] `handle_list_shipments_request` in `src/handlers.rs`.
- [ ] `handle_get_shipment_request` in `src/handlers.rs`.
- [ ] `handle_record_pick_request` in `src/handlers.rs`.
- [ ] `handle_cancel_shipment_request` in `src/handlers.rs`.
- [ ] `handle_ensure_inbound_request` in `src/handlers.rs`.
- [ ] `handle_get_inbound_request` in `src/handlers.rs`.
- [ ] `handle_ensure_shipment_request` in `src/handlers.rs`.
- [ ] `handle_carrier_webhook_event` in `src/handlers.rs`.
- [ ] `shipment`: the reconcile pass in `types/shipment/reconciler.rs`; the worker policy.
- [ ] Table `stock_item`: the queries in `repository.rs`.
- [ ] Table `stock_level`: the queries in `repository.rs`.
- [ ] Table `stock_movement`: the queries in `repository.rs`.
- [ ] Table `kit_component`: the queries in `repository.rs`.
- [ ] Table `assembly_order`: the queries in `repository.rs`.
- [ ] Table `inbound`: the queries in `repository.rs`.
- [ ] Table `low_stock_rule`: the queries in `repository.rs`.
- [ ] Table `shipment_batch`: the queries in `repository.rs`.
- [ ] Table `shipment_address`: the queries in `repository.rs`.
- [ ] Table `shipment_event`: the queries in `repository.rs`.
- [ ] `send` for `create_label` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `void_label` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `tick_check_low_stock` in `worker.rs`.
- [ ] `tick_purge_shipment_addresses` in `worker.rs`.
- [ ] Signature verification for `carrier` in `api/src/webhooks/carrier.rs`.
- [ ] `FulfilmentService` field mapping in `api/src/services/fulfilment.rs` and the messages in `proto/fulfilment/v1/fulfilment.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.

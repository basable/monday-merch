# order

Takes a request from quote to signature, then drives each order through invoicing, production and delivery.

Design: `docs/01-enterprise-merch/02-order.md`. Code:
`nanoservices/order/` (read its `AGENTS.md` first).

## Owns

- Processing-object type `quote` (key 1, `quo_…`): spec `contact`, `contact_email`, `company_id`, `delivery`, `timeline`, `deadline_note`, `consultant_id`, `published_version`, `valid_until`, `withdrawn`, `requested_at`, `sla_due_at`, `source_order_id`, `source_document_id`; status `phase`, `sla_alerted_at`, `sent_version`, `send_intent_at`, `signed_version`, `order_id`, `expired_at`.
- Processing-object type `order` (key 2, `ord_…`): spec `quote_id`, `quote_version`, `company_id`, `lines`, `currency`, `delivery`, `timeline`, `payment_gate`, `cancel_requested`; status `phase`, `invoice_id`, `paid_at`, `production_started_at`, `received_at`, `completed_at`, `attention`.
- Processing-object type `production_run` (key 3, `prun_…`): spec `order_id`, `order_generation`, `supplier_slug`, `lines`, `destination`, `due_on`, `cancel_requested`; status `phase`, `po_intent_at`, `po_sent_at`, `acknowledged_at`, `supplier_ref`, `expected_at`, `inbound_id`, `received_at`, `carrier`, `tracking_number`.
- Table `quote_version` (`id`, `quote_id`, `version`, `lines`, `shipping_minor`, `handling_minor`, `currency`, `note`, `published_at`, `published_by`).
- Table `quote_signature` (`id`, `quote_id`, `version`, `signer_identity_id`, `signer_name`, `signer_email`, `po_number`, `signed_at`).
- Table `quote_comment` (`id`, `quote_id`, `author`, `body`, `created_at`).
- Table `production_event` (`id`, `run_id`, `kind`, `detail`, `actor`, `occurred_at`).
- Table `lead` (`id`, `email`, `name`, `company_name`, `message`, `created_at`, `erase_after`).
- Config type `quote_policy`.
- Schedule `purge_leads` every 24h.

## Messages

- Handles: `RequestQuoteRequest`, `ListQuotesRequest`, `GetQuoteRequest`, `AssignQuoteRequest`, `PublishQuoteVersionRequest`, `RequestQuoteChangesRequest`, `AcceptQuoteRequest`, `ReorderRequest`, `ListOrdersRequest`, `GetOrderRequest`, `CancelOrderRequest`, `ListProductionRunsRequest`, `RecordProductionEventRequest`, `SubmitLeadRequest`, `RecordSupplierUpdateRequest`, `IngestCustomerEmailRequest`, `GetPurchaseMatchRequest`, `InvoiceChanged`, `InboundChanged`, `ShipmentChanged`.
- Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `EnsureCompanyRequest`, `PriceLinesRequest`, `GetSupplierOffersRequest`, `ListProofsRequest`, `ApproveProofsRequest`, `GetApprovedArtworkRequest`, `ResolveDesignVersionRequest`, `AdoptDesignsRequest`, `EnsureInvoiceRequest`, `GetInvoiceRequest`, `EnsureInboundRequest`, `GetInboundRequest`, `EnsureShipmentRequest`, `GetShipmentRequest`, `SendEmailRequest`, `AuditEvent`.

## Directive sections that bind

§2 always; §3, §4, §5 for each processing-object type; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_request_quote_request` in `src/handlers.rs`.
- [ ] `handle_list_quotes_request` in `src/handlers.rs`.
- [ ] `handle_get_quote_request` in `src/handlers.rs`.
- [ ] `handle_assign_quote_request` in `src/handlers.rs`.
- [ ] `handle_publish_quote_version_request` in `src/handlers.rs`.
- [ ] `handle_request_quote_changes_request` in `src/handlers.rs`.
- [ ] `handle_accept_quote_request` in `src/handlers.rs`.
- [ ] `handle_reorder_request` in `src/handlers.rs`.
- [ ] `handle_list_orders_request` in `src/handlers.rs`.
- [ ] `handle_get_order_request` in `src/handlers.rs`.
- [ ] `handle_cancel_order_request` in `src/handlers.rs`.
- [ ] `handle_list_production_runs_request` in `src/handlers.rs`.
- [ ] `handle_record_production_event_request` in `src/handlers.rs`.
- [ ] `handle_submit_lead_request` in `src/handlers.rs`.
- [ ] `handle_record_supplier_update_request` in `src/handlers.rs`.
- [ ] `handle_ingest_customer_email_request` in `src/handlers.rs`.
- [ ] `handle_get_purchase_match_request` in `src/handlers.rs`.
- [ ] `handle_invoice_changed` in `src/handlers.rs`.
- [ ] `handle_inbound_changed` in `src/handlers.rs`.
- [ ] `handle_shipment_changed` in `src/handlers.rs`.
- [ ] `quote`: the reconcile pass in `types/quote/reconciler.rs`; the worker policy.
- [ ] `order`: the reconcile pass in `types/order/reconciler.rs`; the worker policy.
- [ ] `production_run`: the reconcile pass in `types/production_run/reconciler.rs`; the worker policy.
- [ ] Table `quote_version`: the queries in `repository.rs`.
- [ ] Table `quote_signature`: the queries in `repository.rs`.
- [ ] Table `quote_comment`: the queries in `repository.rs`.
- [ ] Table `production_event`: the queries in `repository.rs`.
- [ ] Table `lead`: the queries in `repository.rs`.
- [ ] `tick_purge_leads` in `worker.rs`.
- [ ] `OrderService` field mapping in `api/src/services/order.rs` and the messages in `proto/order/v1/order.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.

# inbox

Receives email, stores it, extracts structured data from each document, and routes the result to its owner or a review queue.

Design: `docs/01-enterprise-merch/02-inbox.md`. Code:
`nanoservices/inbox/` (read its `AGENTS.md` first).

## Owns

- Processing-object type `inbound_message` (key 9, `imsg_…`): spec `provider_message_id`, `to_address`, `from_address`, `subject`, `received_at`; status `phase`, `raw_key`, `message_id_header`, `sender_supplier_slug`, `sender_verified`, `route_hint`, `document_count`, `rejection`.
- Processing-object type `inbound_document` (key 10, `idoc_…`): spec `message_id`, `part_name`, `object_key`, `mime_type`, `sha256`, `sender_supplier_slug`, `sender_verified`, `route_hint`; status `phase`, `kind`, `extraction`, `model`, `checks`, `review_reason`, `routed_to`, `routed_ref`, `applied_at`.
- Table `inbound_seen` (`provider_message_id`, `message_object_id`, `first_seen_at`).
- Table `review_decision` (`id`, `document_id`, `decision`, `corrected_kind`, `corrections`, `actor`, `decided_at`).
- Config type `inbox_policy`.
- External call `store_inbound_object` on object_storage: `idempotent`.
- External call `delete_inbound_object` on object_storage: `idempotent`.
- External call `extract_document` on llm: `idempotent`.
- Schedule `purge_inbound` every 24h.
- Webhook from `inbound_mail`.

## Messages

- Handles: `ListInboundMessagesRequest`, `ListReviewQueueRequest`, `GetInboundDocumentRequest`, `GetInboundFileLinkRequest`, `DecideInboundDocumentRequest`, `InboundEmailEvent`.
- Sends: `ResolveMemberRequest`, `MatchSupplierRequest`, `EnsureSupplierBillRequest`, `RecordSupplierUpdateRequest`, `IngestCustomerEmailRequest`, `SendEmailRequest`, `AuditEvent`.

## Directive sections that bind

§2 always; §3, §4, §5 for each processing-object type; §6 for each adapter; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_list_inbound_messages_request` in `src/handlers.rs`.
- [ ] `handle_list_review_queue_request` in `src/handlers.rs`.
- [ ] `handle_get_inbound_document_request` in `src/handlers.rs`.
- [ ] `handle_get_inbound_file_link_request` in `src/handlers.rs`.
- [ ] `handle_decide_inbound_document_request` in `src/handlers.rs`.
- [ ] `handle_inbound_email_event` in `src/handlers.rs`.
- [ ] `inbound_message`: the reconcile pass in `types/inbound_message/reconciler.rs`; the worker policy.
- [ ] `inbound_document`: the reconcile pass in `types/inbound_document/reconciler.rs`; the worker policy.
- [ ] Table `inbound_seen`: the queries in `repository.rs`.
- [ ] Table `review_decision`: the queries in `repository.rs`.
- [ ] `send` for `store_inbound_object` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `delete_inbound_object` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `extract_document` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `tick_purge_inbound` in `worker.rs`.
- [ ] Signature verification for `inbound_mail` in `api/src/webhooks/inbound_mail.rs`.
- [ ] `InboxService` field mapping in `api/src/services/inbox.rs` and the messages in `proto/inbox/v1/inbox.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.

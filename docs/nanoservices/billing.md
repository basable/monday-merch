# billing

Owns the ledger, invoices and credit notes, payments and VAT returns.

Design: `docs/01-enterprise-merch/02-billing.md`. Code:
`nanoservices/billing/` (read its `AGENTS.md` first).

## Owns

- Processing-object type `invoice` (key 5, `inv_…`): spec `name`, `kind`, `company_id`, `company_snapshot`, `lines`, `currency`, `net_minor`, `vat_minor`, `gross_minor`, `tax_point`, `terms_days`, `corrects_invoice_id`; status `phase`, `number`, `pdf_key`, `ledger_version`, `issued_at`, `due_at`, `send_intent_at`, `sent_at`, `reminders_sent`, `reminder_intent_at`, `settled_at`.
- Processing-object type `payment` (key 6, `pay_…`): spec `reference`, `invoice_id`, `bill_id`, `company_id`, `method`, `direction`, `amount_minor`, `currency`, `value_date`, `cancel_requested`; status `phase`, `intent_at`, `provider_id`, `checkout_url`, `ledger_version`, `settled_at`, `reversed_at`, `failure`.
- Processing-object type `vat_return` (key 7, `vret_…`): spec `country`, `period`, `filed`, `filing_reference`; status `phase`, `vat_due_minor`, `ledger_version`, `filed_at`.
- Processing-object type `supplier_bill` (key 8, `sbill_…`): spec `name`, `supplier_slug`, `supplier_invoice_number`, `lines`, `currency`, `net_minor`, `vat_minor`, `gross_minor`, `invoice_date`, `due_date`, `purchase_ref`, `bank_account`, `source_document_id`, `approved_by`, `voided`; status `phase`, `match_result`, `attention`, `ledger_version`, `approved_at`, `settled_at`.
- Table `invoice_counter` (`series`, `next_number`).
- Table `invoice_number` (`invoice_id`, `series`, `number`, `assigned_at`).
- Table `payment_webhook_event` (`event_id`, `provider_id`, `kind`, `payload`, `received_at`).
- Table `supplier_bill_key` (`dedupe_key`, `bill_id`).
- Config type `vat_rule`.
- Config type `billing_policy`.
- The project's ledger: `invoice` (states `issued`; produced by `billing`), `credit_note` (states `issued`; produced by `billing`), `payment` (states `pending`, `settled`, `reversed`; produced by `billing`), `vat_return` (states `filed`; produced by `billing`), `supplier_bill` (states `approved`, `voided`; produced by `billing`).
- Produces ledger snapshots of `invoice` for `billing` (a `Book` from the reconcile pass).
- Produces ledger snapshots of `credit_note` for `billing` (a `Book` from the reconcile pass).
- Produces ledger snapshots of `payment` for `billing` (a `Book` from the reconcile pass).
- Produces ledger snapshots of `vat_return` for `billing` (a `Book` from the reconcile pass).
- Produces ledger snapshots of `supplier_bill` for `billing` (a `Book` from the reconcile pass).
- External call `create_checkout_session` on payments: `keyed_replay`, irreversible.
- External call `expire_checkout_session` on payments: `idempotent`.
- External call `create_refund` on payments: `keyed_replay`, irreversible.
- External call `store_document` on object_storage: `idempotent`.
- Webhook from `payments`.

## Messages

- Handles: `EnsureInvoiceRequest`, `ListInvoicesRequest`, `GetInvoiceRequest`, `GetInvoiceDownloadRequest`, `PayInvoiceByCardRequest`, `RecordBankPaymentRequest`, `IssueCreditNoteRequest`, `RecordRefundRequest`, `GetSpendReportRequest`, `DraftVatReturnRequest`, `MarkVatReturnFiledRequest`, `GetTrialBalanceRequest`, `EnsureSupplierBillRequest`, `ListSupplierBillsRequest`, `GetSupplierBillRequest`, `ApproveSupplierBillRequest`, `VoidSupplierBillRequest`, `RecordSupplierPaymentRequest`, `PaymentWebhookEvent`, `Book`.
- Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `GetPurchaseMatchRequest`, `SendEmailRequest`, `InvoiceChanged`, `AuditEvent`.

## Directive sections that bind

§2 always; §3, §4, §5 for each processing-object type; §6 for each adapter; §7 for each message sent; §10 for the docs; §11 for the ledger.

## TODO, in order

- [ ] `handle_ensure_invoice_request` in `src/handlers.rs`.
- [ ] `handle_list_invoices_request` in `src/handlers.rs`.
- [ ] `handle_get_invoice_request` in `src/handlers.rs`.
- [ ] `handle_get_invoice_download_request` in `src/handlers.rs`.
- [ ] `handle_pay_invoice_by_card_request` in `src/handlers.rs`.
- [ ] `handle_record_bank_payment_request` in `src/handlers.rs`.
- [ ] `handle_issue_credit_note_request` in `src/handlers.rs`.
- [ ] `handle_record_refund_request` in `src/handlers.rs`.
- [ ] `handle_get_spend_report_request` in `src/handlers.rs`.
- [ ] `handle_draft_vat_return_request` in `src/handlers.rs`.
- [ ] `handle_mark_vat_return_filed_request` in `src/handlers.rs`.
- [ ] `handle_get_trial_balance_request` in `src/handlers.rs`.
- [ ] `handle_ensure_supplier_bill_request` in `src/handlers.rs`.
- [ ] `handle_list_supplier_bills_request` in `src/handlers.rs`.
- [ ] `handle_get_supplier_bill_request` in `src/handlers.rs`.
- [ ] `handle_approve_supplier_bill_request` in `src/handlers.rs`.
- [ ] `handle_void_supplier_bill_request` in `src/handlers.rs`.
- [ ] `handle_record_supplier_payment_request` in `src/handlers.rs`.
- [ ] `handle_payment_webhook_event` in `src/handlers.rs`.
- [ ] `handle_book` in `src/handlers.rs`.
- [ ] `invoice`: the reconcile pass in `types/invoice/reconciler.rs`; the worker policy.
- [ ] `payment`: the reconcile pass in `types/payment/reconciler.rs`; the worker policy.
- [ ] `vat_return`: the reconcile pass in `types/vat_return/reconciler.rs`; the worker policy.
- [ ] `supplier_bill`: the reconcile pass in `types/supplier_bill/reconciler.rs`; the worker policy.
- [ ] Table `invoice_counter`: the queries in `repository.rs`.
- [ ] Table `invoice_number`: the queries in `repository.rs`.
- [ ] Table `payment_webhook_event`: the queries in `repository.rs`.
- [ ] Table `supplier_bill_key`: the queries in `repository.rs`.
- [ ] The snapshot fields in `crates/messages/src/ledger.rs`; the accounts in `src/ledger/accounts.rs`.
- [ ] `invoice`: `desired_state` per state in `src/ledger/invoice.rs`; its test in `tests/ledger.rs`.
- [ ] `credit_note`: `desired_state` per state in `src/ledger/credit_note.rs`; its test in `tests/ledger.rs`.
- [ ] `payment`: `desired_state` per state in `src/ledger/payment.rs`; its test in `tests/ledger.rs`.
- [ ] `vat_return`: `desired_state` per state in `src/ledger/vat_return.rs`; its test in `tests/ledger.rs`.
- [ ] `supplier_bill`: `desired_state` per state in `src/ledger/supplier_bill.rs`; its test in `tests/ledger.rs`.
- [ ] Send `Book` with the `invoice` snapshot from the reconcile pass; bump its version in status in the same write.
- [ ] Send `Book` with the `credit_note` snapshot from the reconcile pass; bump its version in status in the same write.
- [ ] Send `Book` with the `payment` snapshot from the reconcile pass; bump its version in status in the same write.
- [ ] Send `Book` with the `vat_return` snapshot from the reconcile pass; bump its version in status in the same write.
- [ ] Send `Book` with the `supplier_bill` snapshot from the reconcile pass; bump its version in status in the same write.
- [ ] `send` for `create_checkout_session` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `expire_checkout_session` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `create_refund` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `send` for `store_document` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] Signature verification for `payments` in `api/src/webhooks/payments.rs`.
- [ ] `BillingService` field mapping in `api/src/services/billing.rs` and the messages in `proto/billing/v1/billing.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.

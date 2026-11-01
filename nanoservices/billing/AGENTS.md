# billing

Owns the ledger, invoices and credit notes, payments and VAT returns.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §3–§5
(the invariants, the author obligations, the adapter and outcome contract — it
owns processing-object types), §6 (external-effect admission — it owns adapters), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true), §11 (money: it owns the ledger).

## What it owns

| Processing-object type | Key | Prefix | Spec fields | Status fields |
|---|---|---|---|---|
| `invoice` | 5 | `inv` | `name` (immutable), `kind` (immutable), `company_id` (immutable), `company_snapshot` (immutable), `lines` (immutable), `currency` (immutable), `net_minor` (immutable), `vat_minor` (immutable), `gross_minor` (immutable), `tax_point` (immutable), `terms_days` (immutable), `corrects_invoice_id` (immutable) | `phase`, `number`, `pdf_key`, `ledger_version`, `issued_at`, `due_at`, `send_intent_at`, `sent_at`, `reminders_sent`, `reminder_intent_at`, `settled_at` |
| `payment` | 6 | `pay` | `reference` (immutable), `invoice_id` (immutable), `bill_id` (immutable), `company_id` (immutable), `method` (immutable), `direction` (immutable), `amount_minor` (immutable), `currency` (immutable), `value_date`, `cancel_requested` | `phase`, `intent_at`, `provider_id`, `checkout_url`, `ledger_version`, `settled_at`, `reversed_at`, `failure` |
| `vat_return` | 7 | `vret` | `country` (immutable), `period` (immutable), `filed`, `filing_reference` | `phase`, `vat_due_minor`, `ledger_version`, `filed_at` |
| `supplier_bill` | 8 | `sbill` | `name` (immutable), `supplier_slug` (immutable), `supplier_invoice_number` (immutable), `lines` (immutable), `currency` (immutable), `net_minor` (immutable), `vat_minor` (immutable), `gross_minor` (immutable), `invoice_date` (immutable), `due_date` (immutable), `purchase_ref` (immutable), `bank_account` (immutable), `source_document_id` (immutable), `approved_by`, `voided` | `phase`, `match_result`, `attention`, `ledger_version`, `approved_at`, `settled_at` |

Plain tables (schema `nano_billing`): `invoice_counter`, `invoice_number`, `payment_webhook_event`, `supplier_bill_key`.

Config catalogs: `vat_rule` (`config/base/vat_rule.json`), `billing_policy` (`config/base/billing_policy.json`).

The project's ledger (`journal` and `process_ledger_object_version` in `nano_billing`), booking:

| Object type | Snapshot | States | Booked first as | Changes (a state listing itself allows amount changes; none listed: frozen) | Producer |
|---|---|---|---|---|---|
| `invoice` | `messages::InvoiceSnapshot` | `issued` | `issued` | `issued` → nothing | `billing` |
| `credit_note` | `messages::CreditNoteSnapshot` | `issued` | `issued` | `issued` → nothing | `billing` |
| `payment` | `messages::PaymentSnapshot` | `pending`, `settled`, `reversed` | `pending`, `settled` | `pending` → `settled`, `reversed`; `settled` → `reversed`; `reversed` → nothing | `billing` |
| `vat_return` | `messages::VatReturnSnapshot` | `filed` | `filed` | `filed` → nothing | `billing` |
| `supplier_bill` | `messages::SupplierBillSnapshot` | `approved`, `voided` | `approved` | `approved` → `voided`; `voided` → nothing | `billing` |

Produces ledger snapshots: `invoice` (`messages::InvoiceSnapshot`), `credit_note` (`messages::CreditNoteSnapshot`), `payment` (`messages::PaymentSnapshot`), `vat_return` (`messages::VatReturnSnapshot`), `supplier_bill` (`messages::SupplierBillSnapshot`), sent as a `Book` to `billing` from the reconcile pass that changes a state or an amount, with a version the status bumps in the same write.

| External call | System | Strategy | Irreversible |
|---|---|---|---|
| `create_checkout_session` | payments | `keyed_replay` | yes |
| `expire_checkout_session` | payments | `idempotent` | no |
| `create_refund` | payments | `keyed_replay` | yes |
| `store_document` | object_storage | `idempotent` | no |

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Billing` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl BillingHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/types/<type>/type.rs` | The `ProcessingObjectType` declaration: spec/status structs, `TYPE_KEY`, `PUBLIC_ID_PREFIX`. |
| `src/types/<type>/adapter.rs` | The dumb column mapper: every column in BOTH `write_spec` and `read_rows`. |
| `src/types/<type>/reconciler.rs` | The level-triggered pass and the worker policy (identical on every replica). |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/effects.rs` | One `Call` per external call with its strategy literal; `send` is the only dispatch path. |
| `src/provider.rs` / `src/simulator.rs` | The provider trait, the HTTP client (a parsed 4xx is `SendError::Refused`), the noop; the in-memory simulator the audit and the tests drive. |
| `tests/effects_audit.rs` | One `audit!` per adapter — the audit index. Green on day one; the first thing a real provider must keep green. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `src/ledger/mod.rs` | The ledger over this nanoservice's pool with every object type registered; `build` restates at boot. Generated-final: a new object type is added by `scaffold_nanoservice`. |
| `src/ledger/accounts.rs` | The `Account` enum: what names a batch (key, ledger, type). Yours. |
| `src/ledger/<type>.rs` | The `Accounting` of one object type: allowed changes (from the manifest), `desired_state` per state, `validate_balances`. Yours. |
| `../../crates/messages/src/ledger.rs` | The snapshot structs and state enums every producer builds and this ledger books; `Book`, `Booked`. The fields are yours. |
| `tests/ledger.rs` | The tables exist; one property test per object type on the in-memory store. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `EnsureInvoiceRequest` → `Invoice`, `ListInvoicesRequest` → `InvoiceList`, `GetInvoiceRequest` → `Invoice`, `GetInvoiceDownloadRequest` → `DownloadLink`, `PayInvoiceByCardRequest` → `CheckoutLink`, `RecordBankPaymentRequest` → `Payment`, `IssueCreditNoteRequest` → `Invoice`, `RecordRefundRequest` → `Payment`, `GetSpendReportRequest` → `SpendReport`, `DraftVatReturnRequest` → `VatReturn`, `MarkVatReturnFiledRequest` → `VatReturn`, `GetTrialBalanceRequest` → `TrialBalance`, `EnsureSupplierBillRequest` → `SupplierBill`, `ListSupplierBillsRequest` → `SupplierBillList`, `GetSupplierBillRequest` → `SupplierBill`, `ApproveSupplierBillRequest` → `SupplierBill`, `VoidSupplierBillRequest` → `SupplierBill`, `RecordSupplierPaymentRequest` → `Payment`, `PaymentWebhookEvent` → `WebhookAck`, `Book` → `Booked`.
Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `GetPurchaseMatchRequest`, `SendEmailRequest`, `InvoiceChanged`, `AuditEvent`.

API: `BillingService` in `api/src/services/billing.rs` (ListInvoices, GetInvoice, GetInvoiceDownload, PayInvoiceByCard, RecordBankPayment, IssueCreditNote, RecordRefund, GetSpendReport, DraftVatReturn, MarkVatReturnFiled, GetTrialBalance, ListSupplierBills, GetSupplierBill, ApproveSupplierBill, VoidSupplierBill, RecordSupplierPayment).

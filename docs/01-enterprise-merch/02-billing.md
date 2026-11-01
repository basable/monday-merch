# 02: billing

Everything owed, held or moved. This nanoservice owns the project's ledger (Directive §11). Directive §3, §5, §6 bind the processing-object types, §2 every table, §6 every call, §7 every message.

**Written for the recommended round 1 answer: card payment `include`.** If it is not included, the `payments` calls, the webhook, `PayInvoiceByCard` and `payment_webhook_event` go away. Bank transfer and the `payment` object stay.

## The ledger

A quote is not money owed, so nothing is booked until an invoice is issued.

| Object | States | Rule |
| --- | --- | --- |
| `invoice` | `issued` | Frozen once issued. A correction is a credit note. |
| `credit_note` | `issued` | Frozen once issued |
| `payment` | `pending`, `settled`, `reversed` | Starts `pending` for card, `settled` for a recorded bank transfer. `reversed` covers a chargeback. |
| `vat_return` | `filed` | Frozen. A correction is a new return. |

The producer of all four is `billing`. Each snapshot carries a version bumped in the same status write that changes it (`ledger_version`).

**Desired state per object.** Every date in a batch key comes from the snapshot, never the clock. Debit equals credit per ledger and currency, and currency is the unit.

| Object in state | Debit | Credit |
| --- | --- | --- |
| Invoice `issued` | `receivable:<invoice id>` gross | `revenue:<yyyy-mm of tax_point>` net, `vat:<country>:<yyyy-Qn>` VAT |
| Credit note `issued` | `revenue:<yyyy-mm>` net, `vat:<country>:<yyyy-Qn>` VAT | `receivable:<corrected invoice id>` gross |
| Payment `pending` | Nothing (a memo `in_flight:<payment id>`) | Nothing |
| Payment `settled`, incoming | `cash:<method>:<yyyy-mm>` | `receivable:<invoice id>` |
| Payment `settled`, outgoing (refund) | `receivable:<invoice id>` | `cash:<method>:<yyyy-mm>` |
| Payment `reversed` | Nothing, so the difference books the reversal | Nothing |
| VAT return `filed` | `vat:<country>:<period>` VAT due | `tax_authority:<country>` |

**Which batches close, and how:**

| Batch | Closed by |
| --- | --- |
| `receivable:<invoice id>` | Payments and credit notes. Zero means the invoice is settled. |
| `vat:<country>:<period>` | `vat_return` filed |
| `revenue:<yyyy-mm>`, `cash:<method>:<yyyy-mm>` | Not closed here. Reported for the accountant's books, reconciled there. |
| `tax_authority:<country>` | Not closed here until tax payments are recorded |

Positions are sums of booked lines, computed on read. Overpayment leaves a negative `receivable`, which the trial balance and staff list surface. Automatic credit handling is not in this round.

## Processing object: `invoice`

Invoices and credit notes are one lifecycle with `kind` (`invoice`, `credit_note`), because they differ only in sign and in the ledger object they book.

**Spec**, all immutable because an issued invoice is a legal document: `name` (the caller's seam, such as `order:<id>`), `kind`, `company_id`, `company_snapshot` (legal name, address and VAT number at issue), `lines` (with VAT per line and department, for reporting), `currency`, `net_minor`, `vat_minor`, `gross_minor`, `tax_point`, `terms_days`, `corrects_invoice_id`.

**Status**: `phase`, `number`, `pdf_key`, `ledger_version`, `issued_at`, `due_at`, `send_intent_at`, `sent_at`, `reminders_sent`, `reminder_intent_at`, `settled_at`.

`EnsureInvoiceRequest` works out VAT at creation, from `vat_rule`, the company and the destination: reverse charge for a valid EU VAT number in another member state, the destination's rate for an EU customer without one, and none for an export. A VIES VAT number check is a read, so it is not an adapter. The result is frozen in the spec. The request carries the caller's generation.

**Reconcile pass:**

| Level | Checks | Drives | Outcome |
| --- | --- | --- | --- |
| Deleting | `meta.deleting()` | No API marks an invoice deleted. Records must be kept. | `Retry`, loud by age |
| Number | `number` empty | Transaction: lock the `invoice_counter` row `FOR UPDATE`, insert `invoice_number`, advance the counter | Continue |
| PDF | `pdf_key` empty | Render, then `store_document` at a key derived from the invoice id | `Retry` on failure |
| Book | `ledger_version` is 0 | `Book` the snapshot. This comes before sending, so a customer never holds an invoice absent from the books. | `Retry` on failure |
| Send | `sent_at` empty | `claim.write_status` with `send_intent_at`, then `SendEmailRequest` key `invoice:<id>` | `Retry`. Past the replay window it holds for finance. |
| Settled? | Position of `receivable:<id>` | Zero or less: sets `settled_at`, sends `InvoiceChanged` | `Settled` |
| Reminders | Past due, per `reminder_days` | `claim.write_status` with `reminder_intent_at`, then email key `invoice:<id>:reminder:<n>` | `Converged.after` next reminder or due date |

**Numbering:** EU rules require a continuous sequence. **`invoice_counter`** (`series`, `next_number`) is the Directive §7 counter pattern, and **`invoice_number`** (`invoice_id`, `series`, `number`, `assigned_at`) makes the assignment idempotent. A rolled-back transaction consumes nothing, and a sequence is deliberately not used because sequences leave gaps. Credit notes have their own series.

**Worker:** resync 6h, backoff 30s to 15m, `max_attempts` 0, attempt timeout 120s because of PDF rendering.

## Processing object: `payment`

**Spec**: `reference` (unique: the bank reference or `card:<invoice id>:<n>`), `invoice_id`, `company_id`, `method` (`bank_transfer`, `card`), `direction` (`in`, `out`), `amount_minor`, `currency`, `value_date`, `cancel_requested`.

**Status**: `phase`, `intent_at`, `provider_id`, `checkout_url`, `ledger_version`, `settled_at`, `reversed_at`, `failure`.

**Reconcile pass:**

| Level | Checks | Drives | Outcome |
| --- | --- | --- | --- |
| Deleting or cancelled | `meta.deleting()`, `cancel_requested` | `expire_checkout_session` when one is open | `Delete` or `Settled` once confirmed |
| Bank transfer | `method = bank_transfer` | Books `settled`, nudges the invoice | `Settled` |
| Open session | Card, `provider_id` empty | `claim.write_status` with `intent_at`, then `create_checkout_session` | `Retry` on ambiguity. `dispatch` refuses past the window, so it holds. |
| Observe | Reads `payment_webhook_event`, then the session as a read | Paid: books `settled`, nudges the invoice. Expired: `failure`. | `Converged.after(1h)` while open, `Settled` when final |
| Refund | `direction = out`, card | `create_refund` | `Retry` on ambiguity |
| Reversal | Dispute or reversal event | Books `reversed`, nudges the invoice, emails finance | `Settled` |

**Worker:** resync 1h, backoff 30s to 15m, `max_attempts` 0, attempt timeout 60s.

A customer can pay one open invoice by card from the dashboard. Bank transfers are recorded by finance with `RecordBankPayment`, whose unique `reference` prevents recording one twice. Refunds follow a credit note: `RecordRefund` creates a `payment` with `direction: out`.

**Card fee:** their 3.5% fee is not copied. `card_fee_bps` defaults to 0. An issued invoice is frozen, so a fee would need its own invoice, and EU surcharge rules (PSD2) restrict card surcharges, so take advice before turning it on.

## Processing object: `vat_return`

**Spec**: `country`, `period` (such as `2026-Q4`), `filed`, `filing_reference`. **Status**: `phase`, `vat_due_minor`, `ledger_version`, `filed_at`.

Finance drafts it with `DraftVatReturn`, and the pass computes `vat_due_minor` from the position of `vat:<country>:<period>`. Actual filing happens at the tax authority, by a person. `MarkVatReturnFiled` writes `filed` and `filing_reference`, and the pass books `filed`, closing the batch. A later line landing in that batch stays visible as the next return's remainder. It is a lifecycle because it waits on finance and then books.

## Tables

**`payment_webhook_event`** is the inbox for webhooks, which arrive with no claim: `event_id` (primary key, so a redelivery is ignored), `provider_id`, `kind`, `payload`, `received_at`.

## External calls

**Payments provider: Stripe.** One-time payment through a Checkout Session. VAT is already on the invoice, so `automatic_tax` is not used. No `payment_method_types`, so payment methods are managed in the Stripe Dashboard. A restricted API key, not a full secret key. Separate sandboxes per environment. Stripe's published SDK list has no Rust SDK, so calls are plain HTTP behind the adapters, with a pinned `Stripe-Version` (latest at planning time: `2026-09-30.endive`).

**`create_checkout_session`**, strategy `keyed_replay`, `irreversible`, `replayWindow: 24h`. Stripe's documentation says keys may be removed once at least 24 hours old, and that a reused key after pruning creates a new request. The key is derived from the payment id. Include `integration_identifier`. The simulator models replay, ack loss, a replay past the window and a parameter mismatch.

**`expire_checkout_session`**, strategy `idempotent`: expiring an expired session has the same end state. Treat "already expired" and "already complete" as answers, and re-read before acting on a complete one.

**`create_refund`**, strategy `keyed_replay`, `irreversible`, `replayWindow: 24h`, same documentation. The key is derived from the payment id.

**`store_document`** on `object_storage`, strategy `idempotent`: a PUT to a fixed key converges. Invoices go in the private bucket under `billing/invoices/`.

**Payments webhook:** required, not optional. A raw route verifies the Stripe signature and makes one send, `PaymentWebhookEvent`. The handler inserts `payment_webhook_event` and nudges. Subscribed events: `checkout.session.completed`, `checkout.session.async_payment_succeeded`, `checkout.session.async_payment_failed`, `checkout.session.expired`, `charge.refunded`, and the dispute events. Settlement is gated on `payment_status`, never on the browser's return page.

## Config types

`vat_rule`, `billing_policy`. See `04-config.md`.

## Messages and API

`BillingService`, customer: `ListInvoices`, `GetInvoice`, `GetInvoiceDownload`, `PayInvoiceByCard`, `GetSpendReport`. Finance staff: `RecordBankPayment`, `IssueCreditNote`, `RecordRefund`, `DraftVatReturn`, `MarkVatReturnFiled`, `GetTrialBalance`.

`GetSpendReport` groups invoice lines by department, cost centre and order for a company. It reads spec lines and settled status, and no balance column exists.

Internal: `EnsureInvoiceRequest` from `order` and `fulfilment`, `GetInvoiceRequest` from `order`. `InvoiceChanged` is a wake-up for `order`.

## Binding Directive sections

§1, §2 (tables), §3 to §5 (`invoice`, `payment`, `vat_return`), §6 (all four calls), §7 (counter pattern, generations, names), §11 (the ledger).

## Tests

- `effecttest` audits for all four calls, with ack loss mandatory on `create_checkout_session` and `create_refund`.
- `basable-ledger-testkit` scenario, property and conformance suites for every `Accounting`.
- Debit equals credit per currency in every state, for reverse charge, national VAT and export.
- Concurrent issuing produces a gap-free, duplicate-free sequence per series, including a crash mid-transaction.
- A partial payment leaves the invoice open, and the final one settles it. A chargeback reopens it.
- A credit note against a paid invoice leaves a negative receivable until refunded.
- A redelivered Stripe event books once, and a lost one is caught by reading the session.
- The trial balance sums to zero per unit after every scenario.
- Changing a rule bumps the rules version and restates through appended journals only.

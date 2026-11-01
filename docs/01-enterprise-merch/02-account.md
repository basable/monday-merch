# 02: account

Who a customer is, and what they may do. No lifecycle: every row here is changed by a person's request and nothing waits on another system, so the answer to the Directive §1 question is no. This is a plain executor. Directive §2 binds every table, §7 every message.

## Tables

**`company`** is the customer. `source_name` is the seam for `EnsureCompanyRequest` (`quote:<id>`), unique so a retry returns the same company. `payment_gate` is `prepay` or `on_terms`, set by staff after a credit check, default `prepay`. `terms_days` is an agreed exception set by staff. Empty means the default in `billing_policy`, which only `billing` reads (Directive §2). `billing_address` is captured at signature. `display_name`, `legal_name`, `vat_number`, `country` and `currency` feed the invoice snapshot.

**`department`** holds `company_id`, `name` and an optional `cost_center`. Order lines and invoice lines carry the department id so spend can be grouped.

**`membership`** links a person to a company. `email` is the invited address, `identity_id` is filled at first sign-in once Kratos has verified that email, and `role` is `owner`, `admin`, `buyer`, `finance` or `viewer`. `department_id` is optional. `removed_at` revokes access without losing history. A partial unique index on (`company_id`, `email`) where `removed_at` is null goes in a follow-up migration, because a manifest index has no unique flag.

**`staff_member`** is the competitor's own staff: `role` is `consultant`, `designer`, `warehouse`, `finance` or `admin`. Staff are separate from memberships so a customer role can never grant internal access. `email` is unique, `identity_id` is filled at first sign-in, `active` switches access off.

**`address`** is a reusable delivery address of a company (office, event venue): `label`, `contact_name`, `street`, `house_number`, `postcode`, `city`, `region`, `country`, `phone`, `archived_at`.

**`recipient`** is a person a company ships to, typically an employee at home. This table holds the most sensitive data in the system: `email`, `first_name`, `last_name`, `phone`, `home_address`. `labels` groups recipients, `source` records where they came from (`csv`, `manual`). `erase_after` and `erased_at` implement retention.

**`audit_event`** is append-only: `company_id`, `actor`, `action`, `subject`, `detail`, `occurred_at`.

Queries: all company data is filtered by `company_id` taken from the resolved member, never from the request body.

## Invariants

- A public quote request never grants access to an existing company. `EnsureCompanyRequest` always creates a new company keyed on the quote. It never matches on company name or email domain. Staff merge duplicates by hand.
- A member's company comes from `ResolveMemberRequest`, whose result depends only on the verified Kratos identity. A signed-in person with no membership resolves to an identity with no company, which is enough to save a Studio design and nothing more.
- Erasure overwrites the personal fields of a `recipient` and sets `erased_at`. Rows remain so labels and counts stay consistent.

## Messages and API

`AccountService`: `GetCompany`, `UpdateCompany`, `ListMembers`, `InviteMember`, `UpdateMember`, `RemoveMember`, `ListDepartments`, `UpsertDepartment`, `ListAddresses`, `UpsertAddress`, `ListRecipients`, `UpsertRecipient`, `ImportRecipients`, `EraseRecipient`, `ListAuditEvents`.

Internal: `ResolveMemberRequest` (every other nanoservice, on every authenticated request), `EnsureCompanyRequest` (`order` at signature), `GetCompanyRequest` (`order`, `billing`), `ResolveRecipientsRequest` (`fulfilment`, returning the addresses for a shipment batch). `AuditEvent` is a void event from every nanoservice that changes something a customer's auditor would ask about.

`InviteMember` sends one `SendEmailRequest`, keyed `invite:<membership id>`.

`ImportRecipients` takes CSV, validates per row and reports row errors. A file is capped at 10,000 rows and runs on the request path. If real files prove slower than a request can hold, it becomes a processing object.

## Schedules

`purge_recipient_pii` runs every 24h and erases recipients past `erase_after`. The default retention is a business decision that needs legal review. The plan assumes 12 months after last use and makes it per-company configurable later.

## Binding Directive sections

§2 (tables), §7 (messages). `recipient` has its data protection obligations in `05-deployment.md`.

## Tests

- A member of company A receives `permission denied` on every read and write of company B, as one table-driven test over every `AccountService` method.
- `EnsureCompanyRequest` twice with one `source_name` returns one company.
- A quote request naming an existing company's name and email domain creates no membership in it.
- `EraseRecipient` leaves no personal data in the row. Check by reading every column.
- `ImportRecipients` with a bad row reports that row and imports the others.

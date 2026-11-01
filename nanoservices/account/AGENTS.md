# account

Owns companies, members, departments, addresses, recipients and the audit trail.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

Plain tables (schema `nano_account`): `company`, `department`, `membership`, `staff_member`, `address`, `recipient`, `audit_event`.

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Account` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl AccountHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/worker.rs` | The ticker workers (`purge_recipient_pii` every 24h), listed in this nanoservice's `loops()` (`src/lib.rs`), started by the app and joined on shutdown. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `GetCompanyRequest` → `Company`, `UpdateCompanyRequest` → `Company`, `EnsureCompanyRequest` → `Company`, `ResolveMemberRequest` → `Member`, `ListMembersRequest` → `MemberList`, `InviteMemberRequest` → `Member`, `UpdateMemberRequest` → `Member`, `RemoveMemberRequest` → `Member`, `ListDepartmentsRequest` → `DepartmentList`, `UpsertDepartmentRequest` → `Department`, `ListAddressesRequest` → `AddressList`, `UpsertAddressRequest` → `Address`, `ListRecipientsRequest` → `RecipientList`, `UpsertRecipientRequest` → `Recipient`, `ImportRecipientsRequest` → `RecipientImportResult`, `EraseRecipientRequest` → `Recipient`, `ResolveRecipientsRequest` → `RecipientList`, `ListAuditEventsRequest` → `AuditEventList`, `AuditEvent`.
Sends: `SendEmailRequest`.

API: `AccountService` in `api/src/services/account.rs` (GetCompany, UpdateCompany, ListMembers, InviteMember, UpdateMember, RemoveMember, ListDepartments, UpsertDepartment, ListAddresses, UpsertAddress, ListRecipients, UpsertRecipient, ImportRecipients, EraseRecipient, ListAuditEvents).

# account

Owns companies, members, departments, addresses, recipients and the audit trail.

Design: `docs/01-enterprise-merch/02-account.md`. Code:
`nanoservices/account/` (read its `AGENTS.md` first).

## Owns

- Table `company` (`id`, `source_name`, `display_name`, `legal_name`, `vat_number`, `billing_address`, `country`, `currency`, `payment_gate`, `terms_days`, `created_at`).
- Table `department` (`id`, `company_id`, `name`, `cost_center`).
- Table `membership` (`id`, `company_id`, `email`, `identity_id`, `role`, `department_id`, `invited_at`, `accepted_at`, `removed_at`).
- Table `staff_member` (`id`, `email`, `identity_id`, `role`, `active`).
- Table `address` (`id`, `company_id`, `label`, `contact_name`, `street`, `house_number`, `postcode`, `city`, `region`, `country`, `phone`, `archived_at`).
- Table `recipient` (`id`, `company_id`, `email`, `first_name`, `last_name`, `phone`, `home_address`, `labels`, `source`, `erase_after`, `erased_at`).
- Table `audit_event` (`id`, `company_id`, `actor`, `action`, `subject`, `detail`, `occurred_at`).
- Schedule `purge_recipient_pii` every 24h.

## Messages

- Handles: `GetCompanyRequest`, `UpdateCompanyRequest`, `EnsureCompanyRequest`, `ResolveMemberRequest`, `ListMembersRequest`, `InviteMemberRequest`, `UpdateMemberRequest`, `RemoveMemberRequest`, `ListDepartmentsRequest`, `UpsertDepartmentRequest`, `ListAddressesRequest`, `UpsertAddressRequest`, `ListRecipientsRequest`, `UpsertRecipientRequest`, `ImportRecipientsRequest`, `EraseRecipientRequest`, `ResolveRecipientsRequest`, `ListAuditEventsRequest`, `AuditEvent`.
- Sends: `SendEmailRequest`.

## Directive sections that bind

§2 always; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_get_company_request` in `src/handlers.rs`.
- [ ] `handle_update_company_request` in `src/handlers.rs`.
- [ ] `handle_ensure_company_request` in `src/handlers.rs`.
- [ ] `handle_resolve_member_request` in `src/handlers.rs`.
- [ ] `handle_list_members_request` in `src/handlers.rs`.
- [ ] `handle_invite_member_request` in `src/handlers.rs`.
- [ ] `handle_update_member_request` in `src/handlers.rs`.
- [ ] `handle_remove_member_request` in `src/handlers.rs`.
- [ ] `handle_list_departments_request` in `src/handlers.rs`.
- [ ] `handle_upsert_department_request` in `src/handlers.rs`.
- [ ] `handle_list_addresses_request` in `src/handlers.rs`.
- [ ] `handle_upsert_address_request` in `src/handlers.rs`.
- [ ] `handle_list_recipients_request` in `src/handlers.rs`.
- [ ] `handle_upsert_recipient_request` in `src/handlers.rs`.
- [ ] `handle_import_recipients_request` in `src/handlers.rs`.
- [ ] `handle_erase_recipient_request` in `src/handlers.rs`.
- [ ] `handle_resolve_recipients_request` in `src/handlers.rs`.
- [ ] `handle_list_audit_events_request` in `src/handlers.rs`.
- [ ] `handle_audit_event` in `src/handlers.rs`.
- [ ] Table `company`: the queries in `repository.rs`.
- [ ] Table `department`: the queries in `repository.rs`.
- [ ] Table `membership`: the queries in `repository.rs`.
- [ ] Table `staff_member`: the queries in `repository.rs`.
- [ ] Table `address`: the queries in `repository.rs`.
- [ ] Table `recipient`: the queries in `repository.rs`.
- [ ] Table `audit_event`: the queries in `repository.rs`.
- [ ] `tick_purge_recipient_pii` in `worker.rs`.
- [ ] `AccountService` field mapping in `api/src/services/account.rs` and the messages in `proto/account/v1/account.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.

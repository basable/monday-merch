# Agent tasks

The ordered list of what is left after the scaffold. Each nanoservice's
items are in `docs/nanoservices/<name>.md`; this file is the whole-project
order. Tick items off as they land.

1. Read `AGENTS.md`, `docs/DIRECTIVE.md`, `docs/architecture.md`.
2. Work through `docs/nanoservices/account.md` for `account`.
3. Work through `docs/nanoservices/catalog.md` for `catalog`.
4. Work through `docs/nanoservices/design.md` for `design`.
5. Work through `docs/nanoservices/order.md` for `order`.
6. Work through `docs/nanoservices/fulfilment.md` for `fulfilment`.
7. Work through `docs/nanoservices/billing.md` for `billing`.
8. Work through `docs/nanoservices/notifier.md` for `notifier`.
9. Work through `docs/nanoservices/inbox.md` for `inbox`.
10. Declare every message struct in `crates/messages/src/lib.rs` (the build names the missing ones).
11. Fill the proto messages under `proto/` and the field mapping in `api/src/services/`.
12. Un-ignore the `#[ignore = "implement first"]` tests as each piece lands; keep `tests/effects_audit.rs` green.
13. `regex_search unimplemented_step` — only the definitions left (one per crate) means the skeleton is filled.

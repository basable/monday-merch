-- migrate:up
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Plain table `supplier_bill_key` of nanoservice `billing`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE supplier_bill_key (
    dedupe_key TEXT NOT NULL PRIMARY KEY,
    bill_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS supplier_bill_key;
RESET ROLE;

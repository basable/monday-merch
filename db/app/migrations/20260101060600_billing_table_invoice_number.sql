-- migrate:up
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Plain table `invoice_number` of nanoservice `billing`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE invoice_number (
    invoice_id UUID NOT NULL PRIMARY KEY,
    series TEXT NOT NULL,
    number TEXT NOT NULL UNIQUE,
    assigned_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS invoice_number;
RESET ROLE;

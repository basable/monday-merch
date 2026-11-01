-- migrate:up
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;

-- Plain table `lead` of nanoservice `order`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE lead (
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL,
    name TEXT NOT NULL,
    company_name TEXT,
    message TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    erase_after TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_lead_erase_after ON lead (erase_after);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;
DROP TABLE IF EXISTS lead;
RESET ROLE;

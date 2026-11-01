-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `company` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE company (
    id UUID NOT NULL PRIMARY KEY,
    source_name TEXT UNIQUE,
    display_name TEXT NOT NULL,
    legal_name TEXT,
    vat_number TEXT,
    billing_address JSONB,
    country TEXT NOT NULL,
    currency TEXT NOT NULL,
    payment_gate TEXT NOT NULL,
    terms_days INTEGER,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS company;
RESET ROLE;

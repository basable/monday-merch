-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `address` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE address (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID NOT NULL,
    label TEXT NOT NULL,
    contact_name TEXT NOT NULL,
    street TEXT NOT NULL,
    house_number TEXT NOT NULL,
    postcode TEXT NOT NULL,
    city TEXT NOT NULL,
    region TEXT,
    country TEXT NOT NULL,
    phone TEXT,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_address_company_id ON address (company_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS address;
RESET ROLE;

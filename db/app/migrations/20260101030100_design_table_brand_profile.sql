-- migrate:up
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;

-- Plain table `brand_profile` of nanoservice `design`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE brand_profile (
    company_id UUID NOT NULL PRIMARY KEY,
    colours JSONB NOT NULL,
    fonts JSONB NOT NULL,
    guidelines TEXT,
    updated_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;
DROP TABLE IF EXISTS brand_profile;
RESET ROLE;

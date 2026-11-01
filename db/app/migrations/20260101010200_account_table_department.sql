-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `department` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE department (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID NOT NULL,
    name TEXT NOT NULL,
    cost_center TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_department_company_id ON department (company_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS department;
RESET ROLE;

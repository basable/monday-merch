-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `membership` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE membership (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID NOT NULL,
    email TEXT NOT NULL,
    identity_id UUID,
    role TEXT NOT NULL,
    department_id UUID,
    invited_at TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    removed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_membership_company_id ON membership (company_id);

CREATE INDEX idx_membership_email ON membership (email);

CREATE INDEX idx_membership_identity_id ON membership (identity_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS membership;
RESET ROLE;

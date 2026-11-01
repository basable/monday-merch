-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `audit_event` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE audit_event (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID,
    actor TEXT NOT NULL,
    action TEXT NOT NULL,
    subject TEXT NOT NULL,
    detail JSONB NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_audit_event_company_id ON audit_event (company_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS audit_event;
RESET ROLE;

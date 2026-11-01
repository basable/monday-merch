-- migrate:up
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;

-- Plain table `staff_member` of nanoservice `account`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE staff_member (
    id UUID NOT NULL PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    identity_id UUID,
    role TEXT NOT NULL,
    active BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_account;
SET LOCAL search_path = nano_account;
DROP TABLE IF EXISTS staff_member;
RESET ROLE;

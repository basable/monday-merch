-- migrate:up
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;

-- Plain table `inbound_seen` of nanoservice `inbox`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE inbound_seen (
    provider_message_id TEXT NOT NULL PRIMARY KEY,
    message_object_id UUID NOT NULL,
    first_seen_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;
DROP TABLE IF EXISTS inbound_seen;
RESET ROLE;

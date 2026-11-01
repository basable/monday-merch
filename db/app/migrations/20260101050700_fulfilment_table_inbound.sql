-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `inbound` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE inbound (
    id UUID NOT NULL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    source_generation BIGINT NOT NULL,
    company_id UUID NOT NULL,
    warehouse_slug TEXT NOT NULL,
    lines JSONB NOT NULL,
    received_lines JSONB NOT NULL,
    received_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_inbound_received_at ON inbound (received_at);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS inbound;
RESET ROLE;

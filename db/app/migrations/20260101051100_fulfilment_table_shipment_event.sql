-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `shipment_event` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE shipment_event (
    id UUID NOT NULL PRIMARY KEY,
    shipment_id UUID NOT NULL,
    kind TEXT NOT NULL,
    detail JSONB NOT NULL,
    dedupe_key TEXT NOT NULL UNIQUE,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_shipment_event_shipment_id ON shipment_event (shipment_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS shipment_event;
RESET ROLE;

-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `shipment_address` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE shipment_address (
    shipment_id UUID NOT NULL PRIMARY KEY,
    address JSONB,
    erase_after TIMESTAMPTZ,
    erased_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_shipment_address_erase_after ON shipment_address (erase_after);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS shipment_address;
RESET ROLE;

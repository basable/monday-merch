-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `kit_component` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE kit_component (
    id UUID NOT NULL PRIMARY KEY,
    kit_item_id UUID NOT NULL,
    component_item_id UUID NOT NULL,
    quantity INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_kit_component_kit_item_id ON kit_component (kit_item_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS kit_component;
RESET ROLE;

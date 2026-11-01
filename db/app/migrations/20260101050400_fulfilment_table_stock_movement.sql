-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `stock_movement` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE stock_movement (
    id UUID NOT NULL PRIMARY KEY,
    stock_item_id UUID NOT NULL,
    warehouse_slug TEXT NOT NULL,
    kind TEXT NOT NULL,
    on_hand_delta INTEGER NOT NULL,
    reserved_delta INTEGER NOT NULL,
    source TEXT NOT NULL,
    actor TEXT NOT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_stock_movement_stock_item_id ON stock_movement (stock_item_id);

CREATE INDEX idx_stock_movement_source ON stock_movement (source);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS stock_movement;
RESET ROLE;

-- migrate:up
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Plain table `assembly_order` of nanoservice `fulfilment`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE assembly_order (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID NOT NULL,
    kit_item_id UUID NOT NULL,
    warehouse_slug TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    completed_at TIMESTAMPTZ,
    cancelled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_assembly_order_company_id ON assembly_order (company_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS assembly_order;
RESET ROLE;

-- migrate:up
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;

-- Plain table `supplier_offer` of nanoservice `catalog`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE supplier_offer (
    id UUID NOT NULL PRIMARY KEY,
    product_id UUID NOT NULL,
    supplier_slug TEXT NOT NULL,
    supplier_sku TEXT NOT NULL,
    currency TEXT NOT NULL,
    cost_minor BIGINT NOT NULL,
    lead_days INTEGER NOT NULL,
    preferred BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_supplier_offer_product_id ON supplier_offer (product_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;
DROP TABLE IF EXISTS supplier_offer;
RESET ROLE;

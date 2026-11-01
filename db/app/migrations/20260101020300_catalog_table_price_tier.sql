-- migrate:up
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;

-- Plain table `price_tier` of nanoservice `catalog`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE price_tier (
    id UUID NOT NULL PRIMARY KEY,
    product_id UUID NOT NULL,
    currency TEXT NOT NULL,
    min_qty INTEGER NOT NULL,
    unit_minor BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_price_tier_product_id_currency ON price_tier (product_id, currency);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;
DROP TABLE IF EXISTS price_tier;
RESET ROLE;

-- migrate:up
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;

-- Plain table `product` of nanoservice `catalog`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE product (
    id UUID NOT NULL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category_slug TEXT NOT NULL,
    brand_slug TEXT,
    collections JSONB NOT NULL,
    attributes JSONB NOT NULL,
    impact_note TEXT,
    sustainability_labels JSONB NOT NULL,
    min_qty INTEGER NOT NULL,
    production_days INTEGER NOT NULL,
    customisable BOOLEAN NOT NULL,
    best_selling BOOLEAN NOT NULL,
    is_new BOOLEAN NOT NULL,
    weight_grams INTEGER NOT NULL,
    hs_code TEXT,
    origin_country TEXT,
    published_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    search_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_product_category_slug ON product (category_slug);

CREATE INDEX idx_product_archived_at ON product (archived_at);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;
DROP TABLE IF EXISTS product;
RESET ROLE;

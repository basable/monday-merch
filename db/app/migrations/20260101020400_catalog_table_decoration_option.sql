-- migrate:up
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;

-- Plain table `decoration_option` of nanoservice `catalog`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE decoration_option (
    id UUID NOT NULL PRIMARY KEY,
    product_id UUID NOT NULL,
    print_method_slug TEXT NOT NULL,
    areas JSONB NOT NULL,
    currency TEXT NOT NULL,
    setup_minor BIGINT NOT NULL,
    per_unit_minor BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_decoration_option_product_id ON decoration_option (product_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_catalog;
SET LOCAL search_path = nano_catalog;
DROP TABLE IF EXISTS decoration_option;
RESET ROLE;

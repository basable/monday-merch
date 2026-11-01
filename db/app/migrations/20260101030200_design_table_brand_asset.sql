-- migrate:up
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;

-- Plain table `brand_asset` of nanoservice `design`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE brand_asset (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID,
    owner_identity_id UUID,
    kind TEXT NOT NULL,
    filename TEXT NOT NULL,
    object_key TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    size_bytes BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    confirmed_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_brand_asset_company_id ON brand_asset (company_id);

CREATE INDEX idx_brand_asset_owner_identity_id ON brand_asset (owner_identity_id);

CREATE INDEX idx_brand_asset_confirmed_at ON brand_asset (confirmed_at);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;
DROP TABLE IF EXISTS brand_asset;
RESET ROLE;

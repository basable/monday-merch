-- migrate:up
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;

-- Plain table `design_version` of nanoservice `design`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE design_version (
    id UUID NOT NULL PRIMARY KEY,
    design_id UUID NOT NULL,
    version INTEGER NOT NULL,
    document JSONB NOT NULL,
    schema_version INTEGER NOT NULL,
    preview_keys JSONB NOT NULL,
    created_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_design_version_design_id ON design_version (design_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;
DROP TABLE IF EXISTS design_version;
RESET ROLE;

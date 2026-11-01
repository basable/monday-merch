-- migrate:up
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;

-- Plain table `proof` of nanoservice `design`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE proof (
    id UUID NOT NULL PRIMARY KEY,
    subject_name TEXT NOT NULL,
    line_ref TEXT NOT NULL,
    version INTEGER NOT NULL,
    preview_key TEXT NOT NULL,
    artwork_key TEXT NOT NULL,
    print_spec JSONB NOT NULL,
    created_by TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    approved_at TIMESTAMPTZ,
    approved_by TEXT,
    source_design_version_id UUID,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_proof_subject_name ON proof (subject_name);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;
DROP TABLE IF EXISTS proof;
RESET ROLE;

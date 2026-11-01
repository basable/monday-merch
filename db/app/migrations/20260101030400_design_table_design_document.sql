-- migrate:up
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;

-- Plain table `design_document` of nanoservice `design`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE design_document (
    id UUID NOT NULL PRIMARY KEY,
    company_id UUID,
    owner_identity_id UUID NOT NULL,
    product_id UUID NOT NULL,
    variant_id UUID,
    name TEXT NOT NULL,
    draft JSONB NOT NULL,
    schema_version INTEGER NOT NULL,
    revision INTEGER NOT NULL,
    archived_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

CREATE INDEX idx_design_document_company_id ON design_document (company_id);

CREATE INDEX idx_design_document_owner_identity_id ON design_document (owner_identity_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_design;
SET LOCAL search_path = nano_design;
DROP TABLE IF EXISTS design_document;
RESET ROLE;

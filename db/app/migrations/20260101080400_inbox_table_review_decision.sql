-- migrate:up
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;

-- Plain table `review_decision` of nanoservice `inbox`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE review_decision (
    id UUID NOT NULL PRIMARY KEY,
    document_id UUID NOT NULL,
    decision TEXT NOT NULL,
    corrected_kind TEXT,
    corrections JSONB,
    actor TEXT NOT NULL,
    decided_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_review_decision_document_id ON review_decision (document_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;
DROP TABLE IF EXISTS review_decision;
RESET ROLE;

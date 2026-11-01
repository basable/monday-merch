-- migrate:up
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;

-- Plain table `quote_comment` of nanoservice `order`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE quote_comment (
    id UUID NOT NULL PRIMARY KEY,
    quote_id UUID NOT NULL,
    author TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_quote_comment_quote_id ON quote_comment (quote_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;
DROP TABLE IF EXISTS quote_comment;
RESET ROLE;

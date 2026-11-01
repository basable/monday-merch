-- migrate:up
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Plain table `payment_webhook_event` of nanoservice `billing`. Only this
-- nanoservice's role can reach it (the Directive §2).
-- `created_at` and `updated_at` are added when the plan declares none of
-- its own; a declared one is the plan's, without the default.
CREATE TABLE payment_webhook_event (
    event_id TEXT NOT NULL PRIMARY KEY,
    provider_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    payload JSONB NOT NULL,
    received_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp()
);

CREATE INDEX idx_payment_webhook_event_provider_id ON payment_webhook_event (provider_id);

-- dbmate records this version in the same transaction, and the role may
-- not write the ledger.
RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS payment_webhook_event;
RESET ROLE;

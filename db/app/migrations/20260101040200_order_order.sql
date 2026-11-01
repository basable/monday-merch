-- migrate:up

-- Processing-object type `order` (key 2) of nanoservice
-- `order`: the registry row, the envelope partition INSIDE this
-- nanoservice's schema, and the typed spec/status tables hanging off the
-- partition by composite foreign key. The envelope owns identity,
-- generation, scheduling, deletion intent and claim authority; these tables
-- carry domain columns only (the Directive §3 invariant 1).
--
-- The registry row and the partition need the envelope parent's owner, the
-- `app` login this migration runs as, so they come first; the partition is
-- then handed to the nanoservice role, whose typed tables follow under
-- SET LOCAL ROLE. Every framework statement targets the partition, and the
-- typed tables reference the partition, so the role needs nothing on the
-- parent. RESET ROLE at the end: dbmate records this version in the same
-- transaction, and the role may not write the ledger.

INSERT INTO basable.processing_object_type (key, name) VALUES (2, 'order');

CREATE TABLE nano_order.processing_object_order
    PARTITION OF basable.processing_object FOR VALUES IN (2)
    WITH (fillfactor = 90);
ALTER TABLE nano_order.processing_object_order OWNER TO nano_order;

SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;

-- Desired state only.
CREATE TABLE order_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 2
        CHECK (processing_object_type_key = 2),
    quote_id UUID NOT NULL, -- immutable (trigger below)
    quote_version INTEGER NOT NULL, -- immutable (trigger below)
    company_id UUID NOT NULL, -- immutable (trigger below)
    lines JSONB NOT NULL, -- immutable (trigger below)
    currency TEXT NOT NULL, -- immutable (trigger below)
    delivery JSONB NOT NULL, -- immutable (trigger below)
    timeline TEXT NOT NULL, -- immutable (trigger below)
    payment_gate TEXT NOT NULL, -- immutable (trigger below)
    cancel_requested BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_order (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_order_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.quote_id IS DISTINCT FROM OLD.quote_id
       OR NEW.quote_version IS DISTINCT FROM OLD.quote_version
       OR NEW.company_id IS DISTINCT FROM OLD.company_id
       OR NEW.lines IS DISTINCT FROM OLD.lines
       OR NEW.currency IS DISTINCT FROM OLD.currency
       OR NEW.delivery IS DISTINCT FROM OLD.delivery
       OR NEW.timeline IS DISTINCT FROM OLD.timeline
       OR NEW.payment_gate IS DISTINCT FROM OLD.payment_gate THEN
        RAISE EXCEPTION 'order identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_order_spec_identity_immutable
    BEFORE UPDATE ON order_spec
    FOR EACH ROW EXECUTE FUNCTION reject_order_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE order_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 2
        CHECK (processing_object_type_key = 2),
    phase TEXT NOT NULL,
    invoice_id TEXT,
    paid_at TIMESTAMPTZ,
    production_started_at TIMESTAMPTZ,
    received_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    attention TEXT,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_order (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;
DROP TABLE IF EXISTS order_status;
DROP TABLE IF EXISTS order_spec;
DROP FUNCTION IF EXISTS reject_order_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_order.processing_object_order;
DELETE FROM basable.processing_object_type WHERE key = 2 AND name = 'order';

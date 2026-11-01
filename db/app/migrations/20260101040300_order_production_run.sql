-- migrate:up

-- Processing-object type `production_run` (key 3) of nanoservice
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

INSERT INTO basable.processing_object_type (key, name) VALUES (3, 'production_run');

CREATE TABLE nano_order.processing_object_production_run
    PARTITION OF basable.processing_object FOR VALUES IN (3)
    WITH (fillfactor = 90);
ALTER TABLE nano_order.processing_object_production_run OWNER TO nano_order;

SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;

-- Desired state only.
CREATE TABLE production_run_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 3
        CHECK (processing_object_type_key = 3),
    order_id UUID NOT NULL, -- immutable (trigger below)
    order_generation BIGINT NOT NULL,
    supplier_slug TEXT NOT NULL, -- immutable (trigger below)
    lines JSONB NOT NULL, -- immutable (trigger below)
    destination JSONB NOT NULL, -- immutable (trigger below)
    due_on TIMESTAMPTZ NOT NULL,
    cancel_requested BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_production_run (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_production_run_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.order_id IS DISTINCT FROM OLD.order_id
       OR NEW.supplier_slug IS DISTINCT FROM OLD.supplier_slug
       OR NEW.lines IS DISTINCT FROM OLD.lines
       OR NEW.destination IS DISTINCT FROM OLD.destination THEN
        RAISE EXCEPTION 'production_run identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_production_run_spec_identity_immutable
    BEFORE UPDATE ON production_run_spec
    FOR EACH ROW EXECUTE FUNCTION reject_production_run_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE production_run_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 3
        CHECK (processing_object_type_key = 3),
    phase TEXT NOT NULL,
    po_intent_at TIMESTAMPTZ,
    po_sent_at TIMESTAMPTZ,
    acknowledged_at TIMESTAMPTZ,
    supplier_ref TEXT,
    expected_at TIMESTAMPTZ,
    inbound_id TEXT,
    received_at TIMESTAMPTZ,
    carrier TEXT,
    tracking_number TEXT,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_production_run (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;
DROP TABLE IF EXISTS production_run_status;
DROP TABLE IF EXISTS production_run_spec;
DROP FUNCTION IF EXISTS reject_production_run_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_order.processing_object_production_run;
DELETE FROM basable.processing_object_type WHERE key = 3 AND name = 'production_run';

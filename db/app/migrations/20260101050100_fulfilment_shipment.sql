-- migrate:up

-- Processing-object type `shipment` (key 4) of nanoservice
-- `fulfilment`: the registry row, the envelope partition INSIDE this
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

INSERT INTO basable.processing_object_type (key, name) VALUES (4, 'shipment');

CREATE TABLE nano_fulfilment.processing_object_shipment
    PARTITION OF basable.processing_object FOR VALUES IN (4)
    WITH (fillfactor = 90);
ALTER TABLE nano_fulfilment.processing_object_shipment OWNER TO nano_fulfilment;

SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;

-- Desired state only.
CREATE TABLE shipment_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 4
        CHECK (processing_object_type_key = 4),
    name TEXT NOT NULL, -- immutable (trigger below)
    source_generation BIGINT NOT NULL,
    company_id UUID NOT NULL, -- immutable (trigger below)
    batch_id UUID, -- immutable (trigger below)
    warehouse_slug TEXT NOT NULL, -- immutable (trigger below)
    lines JSONB NOT NULL, -- immutable (trigger below)
    service_level TEXT NOT NULL, -- immutable (trigger below)
    incoterm TEXT NOT NULL, -- immutable (trigger below)
    cancel_requested BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_shipment (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_shipment_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.name IS DISTINCT FROM OLD.name
       OR NEW.company_id IS DISTINCT FROM OLD.company_id
       OR NEW.batch_id IS DISTINCT FROM OLD.batch_id
       OR NEW.warehouse_slug IS DISTINCT FROM OLD.warehouse_slug
       OR NEW.lines IS DISTINCT FROM OLD.lines
       OR NEW.service_level IS DISTINCT FROM OLD.service_level
       OR NEW.incoterm IS DISTINCT FROM OLD.incoterm THEN
        RAISE EXCEPTION 'shipment identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_shipment_spec_identity_immutable
    BEFORE UPDATE ON shipment_spec
    FOR EACH ROW EXECUTE FUNCTION reject_shipment_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE shipment_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 4
        CHECK (processing_object_type_key = 4),
    phase TEXT NOT NULL,
    reserved_at TIMESTAMPTZ,
    picked_at TIMESTAMPTZ,
    label_slot TEXT,
    label_id TEXT,
    label_key TEXT,
    carrier TEXT,
    tracking_number TEXT,
    dispatched_at TIMESTAMPTZ,
    delivered_at TIMESTAMPTZ,
    exception TEXT,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_shipment (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_fulfilment;
SET LOCAL search_path = nano_fulfilment;
DROP TABLE IF EXISTS shipment_status;
DROP TABLE IF EXISTS shipment_spec;
DROP FUNCTION IF EXISTS reject_shipment_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_fulfilment.processing_object_shipment;
DELETE FROM basable.processing_object_type WHERE key = 4 AND name = 'shipment';

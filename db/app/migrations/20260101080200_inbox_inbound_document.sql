-- migrate:up

-- Processing-object type `inbound_document` (key 10) of nanoservice
-- `inbox`: the registry row, the envelope partition INSIDE this
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

INSERT INTO basable.processing_object_type (key, name) VALUES (10, 'inbound_document');

CREATE TABLE nano_inbox.processing_object_inbound_document
    PARTITION OF basable.processing_object FOR VALUES IN (10)
    WITH (fillfactor = 90);
ALTER TABLE nano_inbox.processing_object_inbound_document OWNER TO nano_inbox;

SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;

-- Desired state only.
CREATE TABLE inbound_document_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 10
        CHECK (processing_object_type_key = 10),
    message_id UUID NOT NULL, -- immutable (trigger below)
    part_name TEXT NOT NULL, -- immutable (trigger below)
    object_key TEXT NOT NULL, -- immutable (trigger below)
    mime_type TEXT NOT NULL, -- immutable (trigger below)
    sha256 TEXT NOT NULL, -- immutable (trigger below)
    sender_supplier_slug TEXT, -- immutable (trigger below)
    sender_verified BOOLEAN NOT NULL, -- immutable (trigger below)
    route_hint TEXT, -- immutable (trigger below)
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_inbound_document (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_inbound_document_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.message_id IS DISTINCT FROM OLD.message_id
       OR NEW.part_name IS DISTINCT FROM OLD.part_name
       OR NEW.object_key IS DISTINCT FROM OLD.object_key
       OR NEW.mime_type IS DISTINCT FROM OLD.mime_type
       OR NEW.sha256 IS DISTINCT FROM OLD.sha256
       OR NEW.sender_supplier_slug IS DISTINCT FROM OLD.sender_supplier_slug
       OR NEW.sender_verified IS DISTINCT FROM OLD.sender_verified
       OR NEW.route_hint IS DISTINCT FROM OLD.route_hint THEN
        RAISE EXCEPTION 'inbound_document identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_inbound_document_spec_identity_immutable
    BEFORE UPDATE ON inbound_document_spec
    FOR EACH ROW EXECUTE FUNCTION reject_inbound_document_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE inbound_document_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 10
        CHECK (processing_object_type_key = 10),
    phase TEXT NOT NULL,
    kind TEXT,
    extraction JSONB,
    model TEXT,
    checks JSONB,
    review_reason TEXT,
    routed_to TEXT,
    routed_ref TEXT,
    applied_at TIMESTAMPTZ,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_inbound_document (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;
DROP TABLE IF EXISTS inbound_document_status;
DROP TABLE IF EXISTS inbound_document_spec;
DROP FUNCTION IF EXISTS reject_inbound_document_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_inbox.processing_object_inbound_document;
DELETE FROM basable.processing_object_type WHERE key = 10 AND name = 'inbound_document';

-- migrate:up

-- Processing-object type `inbound_message` (key 9) of nanoservice
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

INSERT INTO basable.processing_object_type (key, name) VALUES (9, 'inbound_message');

CREATE TABLE nano_inbox.processing_object_inbound_message
    PARTITION OF basable.processing_object FOR VALUES IN (9)
    WITH (fillfactor = 90);
ALTER TABLE nano_inbox.processing_object_inbound_message OWNER TO nano_inbox;

SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;

-- Desired state only.
CREATE TABLE inbound_message_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 9
        CHECK (processing_object_type_key = 9),
    provider_message_id TEXT NOT NULL, -- immutable (trigger below)
    to_address TEXT NOT NULL, -- immutable (trigger below)
    from_address TEXT NOT NULL, -- immutable (trigger below)
    subject TEXT NOT NULL, -- immutable (trigger below)
    received_at TIMESTAMPTZ NOT NULL, -- immutable (trigger below)
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_inbound_message (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_inbound_message_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.provider_message_id IS DISTINCT FROM OLD.provider_message_id
       OR NEW.to_address IS DISTINCT FROM OLD.to_address
       OR NEW.from_address IS DISTINCT FROM OLD.from_address
       OR NEW.subject IS DISTINCT FROM OLD.subject
       OR NEW.received_at IS DISTINCT FROM OLD.received_at THEN
        RAISE EXCEPTION 'inbound_message identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_inbound_message_spec_identity_immutable
    BEFORE UPDATE ON inbound_message_spec
    FOR EACH ROW EXECUTE FUNCTION reject_inbound_message_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE inbound_message_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 9
        CHECK (processing_object_type_key = 9),
    phase TEXT NOT NULL,
    raw_key TEXT,
    message_id_header TEXT,
    sender_supplier_slug TEXT,
    sender_verified BOOLEAN,
    route_hint TEXT,
    document_count INTEGER,
    rejection TEXT,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_inbound_message (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_inbox;
SET LOCAL search_path = nano_inbox;
DROP TABLE IF EXISTS inbound_message_status;
DROP TABLE IF EXISTS inbound_message_spec;
DROP FUNCTION IF EXISTS reject_inbound_message_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_inbox.processing_object_inbound_message;
DELETE FROM basable.processing_object_type WHERE key = 9 AND name = 'inbound_message';

-- migrate:up

-- Processing-object type `quote` (key 1) of nanoservice
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

INSERT INTO basable.processing_object_type (key, name) VALUES (1, 'quote');

CREATE TABLE nano_order.processing_object_quote
    PARTITION OF basable.processing_object FOR VALUES IN (1)
    WITH (fillfactor = 90);
ALTER TABLE nano_order.processing_object_quote OWNER TO nano_order;

SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;

-- Desired state only.
CREATE TABLE quote_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 1
        CHECK (processing_object_type_key = 1),
    contact JSONB NOT NULL,
    contact_email TEXT NOT NULL, -- immutable (trigger below)
    company_id UUID,
    delivery JSONB NOT NULL,
    timeline TEXT NOT NULL,
    deadline_note TEXT,
    consultant_id UUID,
    published_version INTEGER NOT NULL,
    valid_until TIMESTAMPTZ,
    withdrawn BOOLEAN NOT NULL,
    requested_at TIMESTAMPTZ NOT NULL, -- immutable (trigger below)
    sla_due_at TIMESTAMPTZ NOT NULL, -- immutable (trigger below)
    source_order_id UUID, -- immutable (trigger below)
    source_document_id UUID, -- immutable (trigger below)
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_quote (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_quote_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.contact_email IS DISTINCT FROM OLD.contact_email
       OR NEW.requested_at IS DISTINCT FROM OLD.requested_at
       OR NEW.sla_due_at IS DISTINCT FROM OLD.sla_due_at
       OR NEW.source_order_id IS DISTINCT FROM OLD.source_order_id
       OR NEW.source_document_id IS DISTINCT FROM OLD.source_document_id THEN
        RAISE EXCEPTION 'quote identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_quote_spec_identity_immutable
    BEFORE UPDATE ON quote_spec
    FOR EACH ROW EXECUTE FUNCTION reject_quote_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE quote_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 1
        CHECK (processing_object_type_key = 1),
    phase TEXT NOT NULL,
    sla_alerted_at TIMESTAMPTZ,
    sent_version INTEGER NOT NULL,
    send_intent_at TIMESTAMPTZ,
    signed_version INTEGER,
    order_id UUID,
    expired_at TIMESTAMPTZ,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_quote (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_order;
SET LOCAL search_path = nano_order;
DROP TABLE IF EXISTS quote_status;
DROP TABLE IF EXISTS quote_spec;
DROP FUNCTION IF EXISTS reject_quote_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_order.processing_object_quote;
DELETE FROM basable.processing_object_type WHERE key = 1 AND name = 'quote';

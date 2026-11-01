-- migrate:up

-- Processing-object type `vat_return` (key 7) of nanoservice
-- `billing`: the registry row, the envelope partition INSIDE this
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

INSERT INTO basable.processing_object_type (key, name) VALUES (7, 'vat_return');

CREATE TABLE nano_billing.processing_object_vat_return
    PARTITION OF basable.processing_object FOR VALUES IN (7)
    WITH (fillfactor = 90);
ALTER TABLE nano_billing.processing_object_vat_return OWNER TO nano_billing;

SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Desired state only.
CREATE TABLE vat_return_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 7
        CHECK (processing_object_type_key = 7),
    country TEXT NOT NULL, -- immutable (trigger below)
    period TEXT NOT NULL, -- immutable (trigger below)
    filed BOOLEAN NOT NULL,
    filing_reference TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_vat_return (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_vat_return_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.country IS DISTINCT FROM OLD.country
       OR NEW.period IS DISTINCT FROM OLD.period THEN
        RAISE EXCEPTION 'vat_return identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_vat_return_spec_identity_immutable
    BEFORE UPDATE ON vat_return_spec
    FOR EACH ROW EXECUTE FUNCTION reject_vat_return_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE vat_return_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 7
        CHECK (processing_object_type_key = 7),
    phase TEXT NOT NULL,
    vat_due_minor BIGINT,
    ledger_version BIGINT NOT NULL,
    filed_at TIMESTAMPTZ,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_vat_return (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS vat_return_status;
DROP TABLE IF EXISTS vat_return_spec;
DROP FUNCTION IF EXISTS reject_vat_return_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_billing.processing_object_vat_return;
DELETE FROM basable.processing_object_type WHERE key = 7 AND name = 'vat_return';

-- migrate:up

-- Processing-object type `invoice` (key 5) of nanoservice
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

INSERT INTO basable.processing_object_type (key, name) VALUES (5, 'invoice');

CREATE TABLE nano_billing.processing_object_invoice
    PARTITION OF basable.processing_object FOR VALUES IN (5)
    WITH (fillfactor = 90);
ALTER TABLE nano_billing.processing_object_invoice OWNER TO nano_billing;

SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Desired state only.
CREATE TABLE invoice_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 5
        CHECK (processing_object_type_key = 5),
    name TEXT NOT NULL, -- immutable (trigger below)
    kind TEXT NOT NULL, -- immutable (trigger below)
    company_id UUID NOT NULL, -- immutable (trigger below)
    company_snapshot JSONB NOT NULL, -- immutable (trigger below)
    lines JSONB NOT NULL, -- immutable (trigger below)
    currency TEXT NOT NULL, -- immutable (trigger below)
    net_minor BIGINT NOT NULL, -- immutable (trigger below)
    vat_minor BIGINT NOT NULL, -- immutable (trigger below)
    gross_minor BIGINT NOT NULL, -- immutable (trigger below)
    tax_point TIMESTAMPTZ NOT NULL, -- immutable (trigger below)
    terms_days INTEGER NOT NULL, -- immutable (trigger below)
    corrects_invoice_id UUID, -- immutable (trigger below)
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_invoice (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_invoice_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.name IS DISTINCT FROM OLD.name
       OR NEW.kind IS DISTINCT FROM OLD.kind
       OR NEW.company_id IS DISTINCT FROM OLD.company_id
       OR NEW.company_snapshot IS DISTINCT FROM OLD.company_snapshot
       OR NEW.lines IS DISTINCT FROM OLD.lines
       OR NEW.currency IS DISTINCT FROM OLD.currency
       OR NEW.net_minor IS DISTINCT FROM OLD.net_minor
       OR NEW.vat_minor IS DISTINCT FROM OLD.vat_minor
       OR NEW.gross_minor IS DISTINCT FROM OLD.gross_minor
       OR NEW.tax_point IS DISTINCT FROM OLD.tax_point
       OR NEW.terms_days IS DISTINCT FROM OLD.terms_days
       OR NEW.corrects_invoice_id IS DISTINCT FROM OLD.corrects_invoice_id THEN
        RAISE EXCEPTION 'invoice identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_invoice_spec_identity_immutable
    BEFORE UPDATE ON invoice_spec
    FOR EACH ROW EXECUTE FUNCTION reject_invoice_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE invoice_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 5
        CHECK (processing_object_type_key = 5),
    phase TEXT NOT NULL,
    number TEXT,
    pdf_key TEXT,
    ledger_version BIGINT NOT NULL,
    issued_at TIMESTAMPTZ,
    due_at TIMESTAMPTZ,
    send_intent_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    reminders_sent INTEGER NOT NULL,
    reminder_intent_at TIMESTAMPTZ,
    settled_at TIMESTAMPTZ,
    -- Declared-effect slot (uncomment when an adapter uses Strategy::Declared):
    -- effect_operation VARCHAR(64),
    -- effect_key TEXT,
    -- effect_declared_at TIMESTAMPTZ,
    -- effect_detail JSONB,
    -- CHECK ((effect_operation IS NULL) = (effect_key IS NULL) AND (effect_operation IS NULL) = (effect_declared_at IS NULL)),
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_invoice (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS invoice_status;
DROP TABLE IF EXISTS invoice_spec;
DROP FUNCTION IF EXISTS reject_invoice_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_billing.processing_object_invoice;
DELETE FROM basable.processing_object_type WHERE key = 5 AND name = 'invoice';

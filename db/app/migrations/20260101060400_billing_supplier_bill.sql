-- migrate:up

-- Processing-object type `supplier_bill` (key 8) of nanoservice
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

INSERT INTO basable.processing_object_type (key, name) VALUES (8, 'supplier_bill');

CREATE TABLE nano_billing.processing_object_supplier_bill
    PARTITION OF basable.processing_object FOR VALUES IN (8)
    WITH (fillfactor = 90);
ALTER TABLE nano_billing.processing_object_supplier_bill OWNER TO nano_billing;

SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;

-- Desired state only.
CREATE TABLE supplier_bill_spec (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 8
        CHECK (processing_object_type_key = 8),
    name TEXT NOT NULL, -- immutable (trigger below)
    supplier_slug TEXT NOT NULL, -- immutable (trigger below)
    supplier_invoice_number TEXT NOT NULL, -- immutable (trigger below)
    lines JSONB NOT NULL, -- immutable (trigger below)
    currency TEXT NOT NULL, -- immutable (trigger below)
    net_minor BIGINT NOT NULL, -- immutable (trigger below)
    vat_minor BIGINT NOT NULL, -- immutable (trigger below)
    gross_minor BIGINT NOT NULL, -- immutable (trigger below)
    invoice_date TIMESTAMPTZ NOT NULL, -- immutable (trigger below)
    due_date TIMESTAMPTZ, -- immutable (trigger below)
    purchase_ref TEXT, -- immutable (trigger below)
    bank_account TEXT, -- immutable (trigger below)
    source_document_id UUID NOT NULL, -- immutable (trigger below)
    approved_by TEXT,
    voided BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT clock_timestamp(),
    FOREIGN KEY (processing_object_type_key, id)
        REFERENCES processing_object_supplier_bill (processing_object_type_key, id)
        ON DELETE CASCADE
);

-- Identity columns can never be retargeted, whatever the query.
CREATE FUNCTION reject_supplier_bill_spec_identity_change()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
    IF NEW.name IS DISTINCT FROM OLD.name
       OR NEW.supplier_slug IS DISTINCT FROM OLD.supplier_slug
       OR NEW.supplier_invoice_number IS DISTINCT FROM OLD.supplier_invoice_number
       OR NEW.lines IS DISTINCT FROM OLD.lines
       OR NEW.currency IS DISTINCT FROM OLD.currency
       OR NEW.net_minor IS DISTINCT FROM OLD.net_minor
       OR NEW.vat_minor IS DISTINCT FROM OLD.vat_minor
       OR NEW.gross_minor IS DISTINCT FROM OLD.gross_minor
       OR NEW.invoice_date IS DISTINCT FROM OLD.invoice_date
       OR NEW.due_date IS DISTINCT FROM OLD.due_date
       OR NEW.purchase_ref IS DISTINCT FROM OLD.purchase_ref
       OR NEW.bank_account IS DISTINCT FROM OLD.bank_account
       OR NEW.source_document_id IS DISTINCT FROM OLD.source_document_id THEN
        RAISE EXCEPTION 'supplier_bill identity is immutable (attempted change on %)', OLD.id
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$;

CREATE TRIGGER trg_supplier_bill_spec_identity_immutable
    BEFORE UPDATE ON supplier_bill_spec
    FOR EACH ROW EXECUTE FUNCTION reject_supplier_bill_spec_identity_change();

-- Observed state only, written exclusively under exact claim authority
-- (fenced completions and Claim::write_status). No unfenced writer exists.
CREATE TABLE supplier_bill_status (
    id UUID PRIMARY KEY,
    processing_object_type_key SMALLINT NOT NULL DEFAULT 8
        CHECK (processing_object_type_key = 8),
    phase TEXT NOT NULL,
    match_result JSONB,
    attention TEXT,
    ledger_version BIGINT NOT NULL,
    approved_at TIMESTAMPTZ,
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
        REFERENCES processing_object_supplier_bill (processing_object_type_key, id)
        ON DELETE CASCADE
);

RESET ROLE;

-- migrate:down
SET LOCAL ROLE nano_billing;
SET LOCAL search_path = nano_billing;
DROP TABLE IF EXISTS supplier_bill_status;
DROP TABLE IF EXISTS supplier_bill_spec;
DROP FUNCTION IF EXISTS reject_supplier_bill_spec_identity_change();
RESET ROLE;
DROP TABLE IF EXISTS nano_billing.processing_object_supplier_bill;
DELETE FROM basable.processing_object_type WHERE key = 8 AND name = 'supplier_bill';

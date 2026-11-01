-- migrate:up

-- The schema and role of nanoservice `billing`. Every table it owns
-- lives in nano_billing and belongs to the nano_billing role;
-- only that role may touch them (the Directive §2, made mechanical). The
-- `app` login is a member of every nanoservice role WITH SET so each pool
-- can SET ROLE to its own; it owns the schema (it is the migrator, and the
-- owner of the envelope parent, so it is the one that creates a type's
-- partition in here) but nothing IN it, so an unswitched query against a
-- nanoservice's table is a permission error and only a deliberate SET ROLE
-- escapes.

-- The role is cluster-global and two databases may run this migration at
-- once (a test database per test). The existence check is not atomic, so
-- the concurrent loser's CREATE ROLE is caught and the check repeated: the
-- repeat is what makes the winner's committed role visible to the rest of
-- this transaction (its scan of pg_roles accepts the catalog invalidation
-- the winner's commit sent, which the failed CREATE ROLE left a stale
-- negative cache entry behind for).
DO $$
DECLARE
    attempts int := 0;
BEGIN
    LOOP
        IF EXISTS (SELECT 1 FROM pg_roles WHERE rolname = 'nano_billing') THEN
            EXIT;
        END IF;
        BEGIN
            CREATE ROLE nano_billing NOLOGIN NOINHERIT;
            EXIT;
        EXCEPTION WHEN duplicate_object OR unique_violation THEN
            attempts := attempts + 1;
            IF attempts > 50 THEN
                RAISE;
            END IF;
            PERFORM pg_sleep(0.05);
        END;
    END LOOP;
END $$;

GRANT nano_billing TO app WITH SET TRUE, INHERIT FALSE;

CREATE SCHEMA IF NOT EXISTS nano_billing;
GRANT USAGE, CREATE ON SCHEMA nano_billing TO nano_billing;
ALTER ROLE nano_billing SET search_path = nano_billing;

GRANT USAGE ON SCHEMA basable TO nano_billing;
GRANT SELECT ON basable.processing_object_type TO nano_billing;
GRANT USAGE ON SCHEMA basable_config TO nano_billing;
GRANT SELECT ON ALL TABLES IN SCHEMA basable_config TO nano_billing;
ALTER DEFAULT PRIVILEGES IN SCHEMA basable_config GRANT SELECT ON TABLES TO nano_billing;

-- migrate:down
DROP SCHEMA IF EXISTS nano_billing CASCADE;
REVOKE nano_billing FROM app;
DROP ROLE IF EXISTS nano_billing;
